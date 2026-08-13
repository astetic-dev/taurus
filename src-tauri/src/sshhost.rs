// Taurus als SSH-host (#121): dit werkstation bereikbaar maken voor een collega,
// zonder dat er een OpenSSH Server op hoeft. De toestemming zit hier in de GUI in
// plaats van in authorized_keys: een onbekende sleutel levert een pairing-popup,
// een sessie levert een tweede popup met deny/allow/join.
//
// Bewust een VOLLEDIGE shell, net als wat Taurus zelf op elke host krijgt. De
// echte knoppen zijn toestemming, zichtbaarheid en een audit-spoor -- niet een
// commandofilter, want dat is in een shell toch niet af te dwingen.
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex as StdMutex};

use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use russh::keys::ssh_key::LineEnding;
use russh::keys::{Algorithm, PrivateKey};
use russh::server::{Auth, Config, Handler, Msg, Server as _, Session};
use russh::{Channel, ChannelId, Preferred, SshId};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::config_dir;

pub mod netgate;
mod sftp;

// Vrij volgens het IANA-register (8283-8291 is Unassigned) en spelt TAUR op een
// telefoontoetsenbord. Boven 1024, dus luisteren vraagt geen verhoogde rechten.
pub const DEFAULT_PORT: u16 = 8287;

// Zo lang mag een popup open staan voordat hij zichzelf als "deny" beantwoordt.
// Ruim binnen wat een ssh-client uitzit, en kort genoeg dat een vergeten popup
// geen sleutel openzet.
const CONSENT_TIMEOUT_SECS: u64 = 45;

// --------------------------------------------------------------------------
// peers.json -- wie mag er aankloppen
// --------------------------------------------------------------------------

// Identiteit is de fingerprint, niet de naam: de gebruikersnaam in een
// ssh-verbinding is een claim die de client zelf verzint.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Peer {
    pub fingerprint: String,
    // Laatst geziene gebruikersnaam en adres: alleen om de lijst leesbaar te
    // maken, nooit om op te beslissen.
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub blocked: bool,
    // "Niet meer vragen" per peer. Nodig voor je eigen tweede machine, waar
    // niemand zit om op Allow te klikken.
    #[serde(default)]
    pub auto_allow: bool,
    #[serde(default)]
    pub added: String,
    #[serde(default)]
    pub last_seen: String,
}

fn peers_file() -> std::path::PathBuf {
    config_dir().join("peers.json")
}

pub fn read_peers() -> Vec<Peer> {
    std::fs::read_to_string(peers_file())
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_default()
}

pub fn write_peers(peers: &[Peer]) -> Result<(), String> {
    let dir = config_dir();
    std::fs::create_dir_all(&dir).map_err(|e| format!("map aanmaken: {e}"))?;
    let txt = serde_json::to_string_pretty(peers).map_err(|e| format!("serialiseren: {e}"))?;
    std::fs::write(peers_file(), txt).map_err(|e| format!("peers.json schrijven: {e}"))
}

fn upsert_peer(fingerprint: &str, f: impl FnOnce(&mut Peer)) {
    let mut peers = read_peers();
    match peers.iter_mut().find(|p| p.fingerprint == fingerprint) {
        Some(p) => f(p),
        None => {
            let mut p = Peer {
                fingerprint: fingerprint.to_string(),
                added: now_iso(),
                ..Default::default()
            };
            f(&mut p);
            peers.push(p);
        }
    }
    let _ = write_peers(&peers);
}

fn now_iso() -> String {
    // Geen chrono in de tree; seconden sinds epoch is genoeg om op te sorteren
    // en de frontend maakt er een leesbare datum van.
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_default()
}

// --------------------------------------------------------------------------
// Audit -- het spoor dat in de plaats komt van een commandofilter
// --------------------------------------------------------------------------

fn audit_dir() -> std::path::PathBuf {
    config_dir().join("audit")
}

// Eén regel per gebeurtenis, plus (bij een sessie) het volledige transcript
// ernaast. Best effort: een mislukte audit-schrijfactie mag een sessie niet
// tegenhouden, maar ze mag ook niet stilzwijgend verdwijnen -- vandaar het
// debug-log als terugvalpad.
pub fn audit(app: &AppHandle, kind: &str, peer: &str, detail: &str) {
    let dir = audit_dir();
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let line = format!("{}\t{}\t{}\t{}\n", now_iso(), kind, peer, detail.replace('\n', " "));
    let path = dir.join("events.log");
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        let _ = f.write_all(line.as_bytes());
    }
    let _ = app.emit("ssh-audit", (kind.to_string(), peer.to_string(), detail.to_string()));
}

// Transcript van één sessie. Het is precies wat er over de lijn ging, dus het
// kan geheimen bevatten die iemand in de sessie plakt -- staat zo in de docs.
struct Transcript(Option<std::fs::File>);

impl Transcript {
    fn open(id: &str) -> Self {
        let dir = audit_dir().join("transcripts");
        if std::fs::create_dir_all(&dir).is_err() {
            return Transcript(None);
        }
        Transcript(std::fs::File::create(dir.join(format!("{id}.log"))).ok())
    }
    fn write(&mut self, data: &[u8]) {
        if let Some(f) = self.0.as_mut() {
            let _ = f.write_all(data);
        }
    }
}

// --------------------------------------------------------------------------
// Toestemming -- de popups
// --------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Decision {
    Deny,
    Allow,
    // Allow + spiegel de sessie in een lokale tab.
    Join,
    Block,
    // Allow + onthoud dit, niet meer vragen.
    Always,
}

impl Decision {
    fn from_str(s: &str) -> Decision {
        match s {
            "allow" => Decision::Allow,
            "join" => Decision::Join,
            "block" => Decision::Block,
            "always" => Decision::Always,
            _ => Decision::Deny,
        }
    }
    fn permits(self) -> bool {
        matches!(self, Decision::Allow | Decision::Join | Decision::Always)
    }
}

#[derive(Default)]
pub struct Consents {
    waiting: StdMutex<HashMap<String, tokio::sync::oneshot::Sender<Decision>>>,
    seq: AtomicU64,
}

impl Consents {
    // Vraagt de GUI om een beslissing en wacht erop. Geen antwoord binnen de
    // time-out = deny: een popup die niemand ziet mag nooit toegang opleveren.
    async fn ask(&self, app: &AppHandle, kind: &str, payload: serde_json::Value) -> Decision {
        let id = format!("c{}", self.seq.fetch_add(1, Ordering::Relaxed));
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.waiting.lock().unwrap().insert(id.clone(), tx);

        let mut ev = payload;
        ev["id"] = serde_json::Value::String(id.clone());
        ev["kind"] = serde_json::Value::String(kind.to_string());
        if app.emit("ssh-consent", &ev).is_err() {
            self.waiting.lock().unwrap().remove(&id);
            return Decision::Deny;
        }

        let d = match tokio::time::timeout(
            std::time::Duration::from_secs(CONSENT_TIMEOUT_SECS),
            rx,
        )
        .await
        {
            Ok(Ok(d)) => d,
            // Time-out of een gesloten kanaal (venster weg): allebei deny.
            _ => Decision::Deny,
        };
        self.waiting.lock().unwrap().remove(&id);
        // De popup weghalen als hij vanzelf verliep.
        let _ = app.emit("ssh-consent-done", &id);
        d
    }

    pub fn reply(&self, id: &str, decision: &str) {
        if let Some(tx) = self.waiting.lock().unwrap().remove(id) {
            let _ = tx.send(Decision::from_str(decision));
        }
    }
}

// --------------------------------------------------------------------------
// Draaiende inkomende sessies
// --------------------------------------------------------------------------

// Wat de GUI van een draaiende inkomende sessie laat zien. Zichtbaarheid is een
// van de drie echte knoppen (naast toestemming en het audit-spoor).
#[derive(Serialize, Clone)]
pub struct InboundSession {
    pub id: String,
    pub peer: String,
    pub label: String,
    pub what: String,
    pub mirrored: bool,
}

#[derive(Default)]
pub struct HostState {
    pub consents: Consents,
    pub sessions: StdMutex<HashMap<String, InboundSession>>,
    // De terminals zelf, zodat een lokale JOIN-tab erin kan typen en de lokale
    // gebruiker een sessie kan afkappen.
    pub io: StdMutex<HashMap<String, Arc<SessionIo>>>,
    // Handle om de listener te stoppen; None = staat uit. Let op: dit is de
    // handle van de SERVER (RunningServerHandle), niet die van een sessie --
    // alleen de eerste kent shutdown().
    running: StdMutex<Option<russh::server::RunningServerHandle>>,
    pub port: StdMutex<u16>,
    // Wat de gebruiker WIL (het vinkje), los van of het nu kan. De listener
    // staat alleen open als allebei waar zijn: gewenst EN op een vertrouwd
    // netwerk. Zonder dat onderscheid zou een wisseling van netwerk het vinkje
    // stilzwijgend uitzetten, en dat is precies het soort verrassing dat je in
    // een beveiligingsinstelling niet wilt.
    desired: std::sync::atomic::AtomicBool,
    watching: std::sync::atomic::AtomicBool,
}

impl HostState {
    pub fn is_running(&self) -> bool {
        self.running.lock().unwrap().is_some()
    }
    pub fn is_desired(&self) -> bool {
        self.desired.load(Ordering::Relaxed)
    }
}

// --------------------------------------------------------------------------
// De server
// --------------------------------------------------------------------------

fn host_key_path() -> std::path::PathBuf {
    config_dir().join("ssh_host_ed25519")
}

// Eén hostkey per machine, naast hosts.json. Blijft staan, want een veranderde
// hostkey laat elke client die ons al kent terecht alarm slaan.
fn load_or_create_host_key() -> Result<PrivateKey, String> {
    let p = host_key_path();
    if let Ok(txt) = std::fs::read_to_string(&p) {
        if let Ok(k) = PrivateKey::from_openssh(&txt) {
            return Ok(k);
        }
    }
    let k = PrivateKey::random(&mut rand::rng(), Algorithm::Ed25519)
        .map_err(|e| format!("hostkey genereren: {e}"))?;
    std::fs::create_dir_all(config_dir()).map_err(|e| format!("map aanmaken: {e}"))?;
    let pem = k.to_openssh(LineEnding::LF).map_err(|e| format!("hostkey coderen: {e}"))?;
    std::fs::write(&p, pem.as_bytes()).map_err(|e| format!("hostkey schrijven: {e}"))?;
    Ok(k)
}

pub fn host_key_fingerprint() -> String {
    load_or_create_host_key()
        .map(|k| k.public_key().fingerprint(Default::default()).to_string())
        .unwrap_or_else(|e| e)
}

#[derive(Clone)]
struct TaurusHost {
    app: AppHandle,
    state: Arc<HostState>,
}

impl russh::server::Server for TaurusHost {
    type Handler = HostSession;
    fn new_client(&mut self, addr: Option<SocketAddr>) -> HostSession {
        HostSession {
            app: self.app.clone(),
            state: self.state.clone(),
            address: addr.map(|a| a.ip().to_string()).unwrap_or_default(),
            user: String::new(),
            fingerprint: String::new(),
            auto_allow: false,
            ptys: Default::default(),
            chan_session: Default::default(),
            pty_req: Default::default(),
            channels: Default::default(),
            sftp_channels: Default::default(),
        }
    }
    fn handle_session_error(&mut self, e: russh::Error) {
        // Een client die ophangt geeft op Windows 10054; dat is einde sessie,
        // geen fout om te melden.
        if let russh::Error::IO(io) = &e {
            if io.kind() == std::io::ErrorKind::ConnectionReset {
                return;
            }
        }
        audit(&self.app, "error", "", &format!("sessie: {e:?}"));
    }
}

// Eén draaiende terminal, gedeeld door twee kanten: de SSH-client die hem
// startte en (bij JOIN) een lokale tab. Beide schrijven in dezelfde writer, dus
// twee toetsenborden op één agent -- dat IS de functie.
pub struct SessionIo {
    writer: StdMutex<Option<Box<dyn Write + Send>>>,
    master: StdMutex<Box<dyn portable_pty::MasterPty + Send>>,
    killer: StdMutex<Box<dyn portable_pty::ChildKiller + Send + Sync>>,
    // Twee vensters op één terminal hebben zelden dezelfde maat. De kleinste
    // wint, zoals tmux: dan valt er bij de ander hooguit ruimte weg in plaats
    // van tekst buiten beeld.
    remote: StdMutex<(u16, u16)>,
    local: StdMutex<Option<(u16, u16)>>,
}

impl SessionIo {
    pub fn write(&self, data: &[u8]) {
        if let Some(w) = self.writer.lock().unwrap().as_mut() {
            let _ = w.write_all(data);
            let _ = w.flush();
        }
    }

    fn apply_size(&self) {
        let (rc, rr) = *self.remote.lock().unwrap();
        let (c, r) = match *self.local.lock().unwrap() {
            Some((lc, lr)) => (rc.min(lc), rr.min(lr)),
            None => (rc, rr),
        };
        let _ = self.master.lock().unwrap().resize(PtySize {
            rows: r.max(1),
            cols: c.max(1),
            pixel_width: 0,
            pixel_height: 0,
        });
    }

    pub fn set_remote_size(&self, cols: u16, rows: u16) {
        *self.remote.lock().unwrap() = (cols, rows);
        self.apply_size();
    }

    pub fn set_local_size(&self, cols: u16, rows: u16) {
        *self.local.lock().unwrap() = Some((cols, rows));
        self.apply_size();
    }

    // De lokale gebruiker kijkt niet meer mee; de sessie zelf loopt door.
    pub fn drop_local(&self) {
        *self.local.lock().unwrap() = None;
        self.apply_size();
    }

    pub fn kill(&self) {
        let _ = self.killer.lock().unwrap().kill();
    }
}

struct HostSession {
    app: AppHandle,
    state: Arc<HostState>,
    address: String,
    user: String,
    fingerprint: String,
    auto_allow: bool,
    ptys: Arc<StdMutex<HashMap<ChannelId, Arc<SessionIo>>>>,
    // Welk kanaal hoort bij welke sessie-id, zodat opruimen bij channel_close
    // ook de gedeelde kant (de JOIN-registratie) meeneemt.
    chan_session: Arc<StdMutex<HashMap<ChannelId, String>>>,
    pty_req: Arc<StdMutex<HashMap<ChannelId, (u16, u16, String)>>>,
    channels: Arc<tokio::sync::Mutex<HashMap<ChannelId, Channel<Msg>>>>,
    sftp_channels: Arc<StdMutex<HashSet<ChannelId>>>,
}

// GEMETEN: een client zonder eigen terminal vraagt een pty van 0x0 aan. Geef je
// dat door (ook geklemd op 1x1), dan loopt ConPTY vast en komt het kindproces
// nooit terug. Altijd een echte maat dus.
fn sane_size(v: u32, fallback: u16) -> u16 {
    match u16::try_from(v).unwrap_or(0) {
        0 => fallback,
        n => n,
    }
}

// Wat sshd op Windows doet: alles door cmd.exe. Zonder commando is het een shell.
fn shell_command(cmd: Option<&str>, cwd: Option<&str>) -> CommandBuilder {
    let mut c = CommandBuilder::new("cmd.exe");
    if let Some(line) = cmd {
        c.arg("/C");
        c.arg(line);
    }
    let home = std::env::var("USERPROFILE").unwrap_or_else(|_| ".".into());
    c.cwd(cwd.unwrap_or(&home));
    c
}

impl HostSession {
    fn peer_name(&self) -> String {
        format!("{}@{}", self.user, self.address)
    }

    // De sessie-popup. Bij "always" onthouden we het antwoord bij de peer.
    async fn ask_session(&mut self, what: &str) -> Decision {
        if self.auto_allow {
            return Decision::Allow;
        }
        let d = self
            .state
            .consents
            .ask(
                &self.app,
                "session",
                serde_json::json!({
                    "user": self.user,
                    "address": self.address,
                    "fingerprint": self.fingerprint,
                    "what": what,
                }),
            )
            .await;
        match d {
            Decision::Always => {
                let fp = self.fingerprint.clone();
                upsert_peer(&fp, |p| p.auto_allow = true);
                self.auto_allow = true;
            }
            Decision::Block => {
                let fp = self.fingerprint.clone();
                upsert_peer(&fp, |p| {
                    p.blocked = true;
                    p.auto_allow = false;
                });
            }
            _ => {}
        }
        audit(
            &self.app,
            if d.permits() { "session-allow" } else { "session-deny" },
            &self.peer_name(),
            what,
        );
        d
    }

    // Start een programma in een ConPTY, pompt beide kanten op, schrijft het
    // transcript en registreert de sessie zodat de GUI hem kan tonen en killen.
    fn start(
        &self,
        handle: russh::server::Handle,
        channel: ChannelId,
        cmd: Option<&str>,
        cols: u16,
        rows: u16,
        term: &str,
        mirror: bool,
    ) -> Result<(), String> {
        let pty = NativePtySystem::default();
        let pair = pty
            .openpty(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 })
            .map_err(|e| format!("openpty: {e}"))?;

        let mut builder = shell_command(cmd, None);
        builder.env("TERM", if term.is_empty() { "xterm-256color" } else { term });

        let mut child = pair.slave.spawn_command(builder).map_err(|e| format!("starten: {e}"))?;
        drop(pair.slave);

        let mut reader = pair.master.try_clone_reader().map_err(|e| format!("reader: {e}"))?;
        let writer = pair.master.take_writer().map_err(|e| format!("writer: {e}"))?;
        // Losse killer, want `child` verhuist zo naar de wacht-thread.
        let killer = child.clone_killer();

        let io = Arc::new(SessionIo {
            writer: StdMutex::new(Some(writer)),
            master: StdMutex::new(pair.master),
            killer: StdMutex::new(killer),
            remote: StdMutex::new((cols, rows)),
            local: StdMutex::new(None),
        });

        let session_id = format!("in-{}-{}", channel, now_iso());
        let mut transcript = Transcript::open(&session_id);
        let app = self.app.clone();
        let sid = session_id.clone();
        let rt = tokio::runtime::Handle::current();
        let reader_handle = handle.clone();

        std::thread::spawn(move || {
            use std::io::Read;
            let mut buf = [0u8; 8192];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        transcript.write(&buf[..n]);
                        // JOIN: dezelfde bytes ook naar een lokale tab. De
                        // frontend tekent ze in een tab die naast de sessie leeft.
                        if mirror {
                            let _ = app.emit(
                                "ssh-mirror-output",
                                (sid.clone(), crate::b64(&buf[..n])),
                            );
                        }
                        if rt.block_on(reader_handle.data(channel, buf[..n].to_vec())).is_err() {
                            break;
                        }
                    }
                }
            }
        });

        let rt2 = tokio::runtime::Handle::current();
        let app2 = self.app.clone();
        let state2 = self.state.clone();
        let sid2 = session_id.clone();
        std::thread::spawn(move || {
            let code = child.wait().map(|s| s.exit_code()).unwrap_or(1);
            state2.sessions.lock().unwrap().remove(&sid2);
            state2.io.lock().unwrap().remove(&sid2);
            let _ = app2.emit("ssh-sessions-changed", ());
            let _ = app2.emit("ssh-mirror-exit", sid2);
            rt2.block_on(async move {
                let _ = handle.exit_status_request(channel, code).await;
                let _ = handle.eof(channel).await;
                let _ = handle.close(channel).await;
            });
        });

        self.state.sessions.lock().unwrap().insert(
            session_id.clone(),
            InboundSession {
                id: session_id.clone(),
                peer: self.fingerprint.clone(),
                label: self.peer_name(),
                what: cmd.unwrap_or("shell").to_string(),
                mirrored: mirror,
            },
        );
        self.state.io.lock().unwrap().insert(session_id.clone(), io.clone());
        self.chan_session.lock().unwrap().insert(channel, session_id);
        let _ = self.app.emit("ssh-sessions-changed", ());

        self.ptys.lock().unwrap().insert(channel, io);
        Ok(())
    }
}

impl Handler for HostSession {
    type Error = russh::Error;

    // Dit is het pairing-moment. De naam is een claim; de fingerprint is identiteit.
    async fn auth_publickey(
        &mut self,
        user: &str,
        key: &russh::keys::ssh_key::PublicKey,
    ) -> Result<Auth, Self::Error> {
        let fp = key.fingerprint(Default::default()).to_string();
        self.user = user.to_string();
        self.fingerprint = fp.clone();

        let known = read_peers().into_iter().find(|p| p.fingerprint == fp);

        if let Some(p) = &known {
            if p.blocked {
                // Geblokkeerd: geen popup, wel een auditregel. Zo blijft een
                // vervelende collega weg zonder dat hij je scherm kan bezetten.
                audit(&self.app, "auth-blocked", &self.peer_name(), &fp);
                return Ok(Auth::reject());
            }
            self.auto_allow = p.auto_allow;
            upsert_peer(&fp, |x| {
                x.last_seen = now_iso();
                x.label = user.to_string();
                x.address = self.address.clone();
            });
            audit(&self.app, "auth-known", &self.peer_name(), &fp);
            return Ok(Auth::Accept);
        }

        // Onbekende sleutel -> pairing-popup.
        audit(&self.app, "auth-unknown", &self.peer_name(), &fp);
        let d = self
            .state
            .consents
            .ask(
                &self.app,
                "pair",
                serde_json::json!({
                    "user": user,
                    "address": self.address,
                    "fingerprint": fp,
                }),
            )
            .await;

        match d {
            Decision::Block => {
                upsert_peer(&fp, |p| {
                    p.blocked = true;
                    p.label = user.to_string();
                    p.address = self.address.clone();
                    p.last_seen = now_iso();
                });
                audit(&self.app, "pair-block", &self.peer_name(), &fp);
                Ok(Auth::reject())
            }
            d if d.permits() => {
                upsert_peer(&fp, |p| {
                    p.label = user.to_string();
                    p.address = self.address.clone();
                    p.last_seen = now_iso();
                    p.auto_allow = d == Decision::Always;
                });
                self.auto_allow = d == Decision::Always;
                audit(&self.app, "pair-allow", &self.peer_name(), &fp);
                Ok(Auth::Accept)
            }
            _ => {
                audit(&self.app, "pair-deny", &self.peer_name(), &fp);
                Ok(Auth::reject())
            }
        }
    }

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        reply: russh::server::ChannelOpenHandle,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.channels.lock().await.insert(channel.id(), channel);
        reply.accept().await;
        Ok(())
    }

    async fn pty_request(
        &mut self,
        channel: ChannelId,
        term: &str,
        col_width: u32,
        row_height: u32,
        _pw: u32,
        _ph: u32,
        _modes: &[(russh::Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.pty_req.lock().unwrap().insert(
            channel,
            (sane_size(col_width, 80), sane_size(row_height, 24), term.to_string()),
        );
        session.channel_success(channel)?;
        Ok(())
    }

    async fn window_change_request(
        &mut self,
        channel: ChannelId,
        col_width: u32,
        row_height: u32,
        _pw: u32,
        _ph: u32,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let io = self.ptys.lock().unwrap().get(&channel).cloned();
        if let Some(io) = io {
            io.set_remote_size(sane_size(col_width, 80), sane_size(row_height, 24));
        }
        session.channel_success(channel)?;
        Ok(())
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        let io = self.ptys.lock().unwrap().get(&channel).cloned();
        if let Some(io) = io {
            io.write(data);
        }
        Ok(())
    }

    // GEMETEN: bij een pty-sessie mag client-EOF de terminal NIET sluiten -- dan
    // sterft cmd.exe meteen met 0xC000013A (STATUS_CONTROL_C_EXIT). Voor een
    // sftp-kanaal is dit juist het laatste moment om een exit-status te sturen:
    // zonder die status meldt scp een geslaagde overdracht als mislukt.
    async fn channel_eof(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        if self.sftp_channels.lock().unwrap().remove(&channel) {
            let handle = session.handle();
            tokio::spawn(async move {
                let _ = handle.exit_status_request(channel, 0).await;
                let _ = handle.eof(channel).await;
                let _ = handle.close(channel).await;
            });
        }
        Ok(())
    }

    async fn channel_close(
        &mut self,
        channel: ChannelId,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.ptys.lock().unwrap().remove(&channel);
        self.pty_req.lock().unwrap().remove(&channel);
        self.channels.lock().await.remove(&channel);
        // De sessie zelf blijft draaien tot het kindproces stopt -- Taurus is de
        // mux voor inkomende sessies, dus een verbroken verbinding kost niets.
        // Alleen de koppeling kanaal->sessie is nu betekenisloos.
        self.chan_session.lock().unwrap().remove(&channel);
        Ok(())
    }

    // De DROPZONE stuurt bestanden met scp.exe, en dat is sinds OpenSSH 9 een
    // SFTP-client.
    async fn subsystem_request(
        &mut self,
        channel: ChannelId,
        name: &str,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        if name != "sftp" {
            session.channel_failure(channel)?;
            return Ok(());
        }
        if !self.ask_session("bestandsoverdracht (SFTP)").await.permits() {
            session.channel_failure(channel)?;
            return Ok(());
        }
        let Some(ch) = self.channels.lock().await.remove(&channel) else {
            session.channel_failure(channel)?;
            return Ok(());
        };
        session.channel_success(channel)?;
        self.sftp_channels.lock().unwrap().insert(channel);
        audit(&self.app, "sftp-start", &self.peer_name(), "");
        russh_sftp::server::run(ch.into_stream(), sftp::SftpSession::default()).await;
        Ok(())
    }

    async fn shell_request(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let d = self.ask_session("een shell").await;
        if !d.permits() {
            session.channel_failure(channel)?;
            return Ok(());
        }
        let (cols, rows, term) = self
            .pty_req
            .lock()
            .unwrap()
            .get(&channel)
            .cloned()
            .unwrap_or((80, 24, "xterm-256color".into()));
        let handle = session.handle();
        match self.start(handle, channel, None, cols, rows, &term, d == Decision::Join) {
            Ok(()) => session.channel_success(channel)?,
            Err(e) => {
                audit(&self.app, "error", &self.peer_name(), &format!("shell: {e}"));
                session.channel_failure(channel)?;
            }
        }
        Ok(())
    }

    async fn exec_request(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let line = String::from_utf8_lossy(data).to_string();
        let want_pty = self.pty_req.lock().unwrap().get(&channel).cloned();

        match want_pty {
            // `ssh -t host "..."`: dit is het pad waarmee Taurus een tab opent.
            Some((cols, rows, term)) => {
                let d = self.ask_session(&format!("een sessie: {line}")).await;
                if !d.permits() {
                    session.channel_failure(channel)?;
                    return Ok(());
                }
                let handle = session.handle();
                match self.start(
                    handle,
                    channel,
                    Some(&line),
                    cols,
                    rows,
                    &term,
                    d == Decision::Join,
                ) {
                    Ok(()) => session.channel_success(channel)?,
                    Err(e) => {
                        audit(&self.app, "error", &self.peer_name(), &format!("exec: {e}"));
                        session.channel_failure(channel)?;
                    }
                }
            }
            // Zonder pty: de probe-route. Geen popup per commando -- een enkele
            // probe is een handvol ssh-rondes, en dan is de pairing de grens.
            None => {
                audit(&self.app, "exec", &self.peer_name(), &line);
                session.channel_success(channel)?;
                let handle = session.handle();
                tokio::task::spawn_blocking(move || {
                    let out = crate::quiet_command("cmd.exe").arg("/C").arg(&line).output();
                    let rt = tokio::runtime::Handle::current();
                    rt.block_on(async {
                        match out {
                            Ok(o) => {
                                if !o.stdout.is_empty() {
                                    let _ = handle.data(channel, o.stdout).await;
                                }
                                if !o.stderr.is_empty() {
                                    let _ = handle.extended_data(channel, 1, o.stderr).await;
                                }
                                let _ = handle
                                    .exit_status_request(
                                        channel,
                                        o.status.code().unwrap_or(0) as u32,
                                    )
                                    .await;
                            }
                            Err(e) => {
                                let _ =
                                    handle.data(channel, format!("{e}\n").into_bytes()).await;
                                let _ = handle.exit_status_request(channel, 1).await;
                            }
                        }
                        let _ = handle.eof(channel).await;
                        let _ = handle.close(channel).await;
                    });
                });
            }
        }
        Ok(())
    }
}

// --------------------------------------------------------------------------
// Starten en stoppen van de listener
// --------------------------------------------------------------------------

// Het vinkje. Zet de wens en laat de wachter beslissen of de deur ook echt open
// mag -- die kijkt naar het netwerk.
pub fn set_enabled(
    app: AppHandle,
    state: Arc<HostState>,
    enabled: bool,
    port: u16,
) -> Result<(), String> {
    state.desired.store(enabled, Ordering::Relaxed);
    *state.port.lock().unwrap() = port;
    if !enabled {
        stop(&app, &state);
        return Ok(());
    }
    if !netgate::on_trusted_network() {
        // Geen fout: de wens staat aan, de deur blijft dicht. De GUI legt uit
        // waarom, in plaats van het vinkje stiekem terug te zetten.
        let _ = app.emit("ssh-host-changed", ());
        watch_network(app, state);
        return Ok(());
    }
    let r = start(app.clone(), state.clone(), port);
    watch_network(app, state);
    r
}

// Netwerkwissels volgen. Bewust pollen in plaats van een COM-event-sink: een
// INetworkListManagerEvents-sink is veel meer code (en een extra apartment-
// thread) voor iets waar een halve minuut vertraging niet uitmaakt -- je loopt
// met een laptop van het ene netwerk naar het andere, niet per seconde.
fn watch_network(app: AppHandle, state: Arc<HostState>) {
    if state.watching.swap(true, Ordering::SeqCst) {
        return; // er loopt er al een
    }
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(15));
            if !state.is_desired() {
                state.watching.store(false, Ordering::SeqCst);
                return;
            }
            let ok = netgate::on_trusted_network();
            let running = state.is_running();
            if ok && !running {
                let port = *state.port.lock().unwrap();
                audit(&app, "network-trusted", "", "listener gaat open");
                let _ = start(app.clone(), state.clone(), port);
            } else if !ok && running {
                audit(&app, "network-untrusted", "", "listener gaat dicht");
                stop(&app, &state);
            }
        }
    });
}

pub fn start(app: AppHandle, state: Arc<HostState>, port: u16) -> Result<(), String> {
    if state.is_running() {
        return Ok(());
    }
    let key = load_or_create_host_key()?;
    let config = Arc::new(Config {
        server_id: SshId::Standard(format!("SSH-2.0-Taurus_{}", env!("CARGO_PKG_VERSION")).into()),
        inactivity_timeout: None,
        // Ruim: hier zit een mens met een popup tussen.
        auth_rejection_time: std::time::Duration::from_secs(2),
        keys: vec![key],
        preferred: Preferred::default(),
        ..Default::default()
    });

    let (tx, rx) = std::sync::mpsc::channel();
    let app2 = app.clone();
    let state2 = state.clone();
    tauri::async_runtime::spawn(async move {
        let socket = match tokio::net::TcpListener::bind(("0.0.0.0", port)).await {
            Ok(s) => s,
            Err(e) => {
                let _ = tx.send(Err(format!("kan niet luisteren op poort {port}: {e}")));
                return;
            }
        };
        let mut server = TaurusHost { app: app2.clone(), state: state2.clone() };
        let running = server.run_on_socket(config, &socket);
        let _ = tx.send(Ok(running.handle()));
        audit(&app2, "listener-start", "", &port.to_string());
        let _ = running.await;
        *state2.running.lock().unwrap() = None;
        audit(&app2, "listener-stop", "", "");
        let _ = app2.emit("ssh-host-changed", ());
    });

    match rx.recv_timeout(std::time::Duration::from_secs(10)) {
        Ok(Ok(handle)) => {
            *state.running.lock().unwrap() = Some(handle);
            *state.port.lock().unwrap() = port;
            let _ = app.emit("ssh-host-changed", ());
            Ok(())
        }
        Ok(Err(e)) => Err(e),
        Err(_) => Err("de server reageerde niet binnen 10 seconden".into()),
    }
}

pub fn stop(app: &AppHandle, state: &HostState) {
    let handle = state.running.lock().unwrap().take();
    if let Some(h) = handle {
        h.shutdown("Taurus is niet langer bereikbaar".into());
    }
    let _ = app.emit("ssh-host-changed", ());
}

#[cfg(test)]
mod tests {
    use super::*;

    // GEMETEN tegen OpenSSH_for_Windows_9.5p2: een client zonder eigen terminal
    // vraagt 0x0 aan, en dan loopt ConPTY vast -- het kindproces komt nooit
    // terug en de tab blijft leeg. Deze klem is dus geen cosmetica.
    #[test]
    fn pty_size_zero_becomes_usable() {
        assert_eq!(sane_size(0, 80), 80);
        assert_eq!(sane_size(0, 24), 24);
        assert_eq!(sane_size(120, 80), 120);
        // Groter dan u16 is onzin van de client; val terug in plaats van af te kappen.
        assert_eq!(sane_size(100_000, 80), 80);
    }

    #[test]
    fn only_permitting_decisions_open_a_session() {
        assert!(Decision::Allow.permits());
        assert!(Decision::Join.permits());
        assert!(Decision::Always.permits());
        assert!(!Decision::Deny.permits());
        assert!(!Decision::Block.permits());
    }

    // Alles wat geen expliciet ja is, is nee. Een onbekend antwoord (oude
    // frontend, typefout) mag nooit toegang opleveren.
    #[test]
    fn unknown_answer_is_deny() {
        assert_eq!(Decision::from_str("allow"), Decision::Allow);
        assert_eq!(Decision::from_str("join"), Decision::Join);
        assert_eq!(Decision::from_str("always"), Decision::Always);
        assert_eq!(Decision::from_str("block"), Decision::Block);
        assert_eq!(Decision::from_str("deny"), Decision::Deny);
        assert_eq!(Decision::from_str(""), Decision::Deny);
        assert_eq!(Decision::from_str("ALLOW"), Decision::Deny);
        assert_eq!(Decision::from_str("ja graag"), Decision::Deny);
    }

    // Een shell zonder commando, en een commando via cmd.exe /C -- zoals sshd
    // op Windows het doet. De probe van Taurus stuurt hier PowerShell-regels
    // met -EncodedCommand doorheen.
    #[test]
    fn exec_runs_through_cmd() {
        let shell = format!("{:?}", shell_command(None, None));
        assert!(shell.contains("cmd.exe"), "{shell}");
        assert!(!shell.contains("/C"), "shell mag geen commando meekrijgen: {shell}");

        let exec = format!("{:?}", shell_command(Some("powershell -NoProfile -Enc AAA"), None));
        assert!(exec.contains("/C"), "{exec}");
        assert!(exec.contains("powershell"), "{exec}");
    }
}
