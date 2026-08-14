// Machines vinden op het vertrouwde netwerk (#125).
//
// Het doel is de "machine toevoegen"-vorm kwijtraken: geen IP, geen gebruikersnaam,
// geen sleutelpad om op te zoeken. Een Taurus die bereikbaar staat kondigt zichzelf
// aan; een Taurus met het machinescherm open ziet dat.
//
// DRIE DINGEN DIE DE SPIKE MAT, en die hier alle drie in de code zitten:
//
// 1. `enable_addr_auto()` adverteert elk adres van elke interface -- op dit
//    werkstation ook de Hyper-V- en WSL-interne ranges. Dat vertelt de LAN hoe de
//    machine intern is opgedeeld en geen van die adressen is voor de andere kant
//    bruikbaar. Daarom: aankondigen op precies het adres van de VERTROUWDE
//    interface, en `addr_auto` blijft uit.
// 2. Dezelfde dienst loste negen keer in acht seconden op, telkens opnieuw vurend
//    terwijl er adressen bijkwamen. Daarom is de instantienaam de sleutel en wordt
//    een rij BIJGEWERKT, nooit toegevoegd.
// 3. Aankondigen is passief: dit levert nooit een melding, badge of popup op. Het
//    browsen loopt alleen terwijl het machinescherm open staat. Een interruptie
//    moet iets betekenen -- als elke Taurus op de gang knippert wanneer een collega
//    zijn host aanzet, verdrinkt de popup die wél een antwoord nodig heeft.
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use mdns_sd::{ResolvedService, ServiceDaemon, ServiceEvent, ServiceInfo};
use serde::{Deserialize, Serialize};

use crate::sshhost::netgate;

pub const SERVICE: &str = "_taurus._tcp.local.";

// Een openstaande hulpvraag zoals hij over de lijn gaat. Deserialize erbij omdat de
// frontend hem TERUGSTUURT om hem te beantwoorden.
//
// Dit is geen machine maar een VERZOEK. Een eerdere versie kondigde aanwezigheid
// aan -- elke Taurus die bereikbaar stond riep permanent dat hij bestond -- en dat
// beantwoordt een vraag die niemand stelt: wie er toevallig aan staat. Nu geldt
// hetzelfde als bij bluetooth: zichtbaar zolang je koppelt, de rest van de tijd
// bestaat het niet.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Found {
    // De instantienaam, en tegelijk de sleutel waarop wordt bijgewerkt.
    pub name: String,
    pub address: String,
    pub port: u16,
    // Wat de andere kant over zichzelf zegt. Een claim, geen bewijs: de identiteit
    // die telt is de fingerprint, en die wordt bij het verbinden gecontroleerd.
    pub user: String,
    pub fingerprint: String,
    // Genoeg om te kunnen verbinden ZONDER formulier -- dat is de hele belofte van
    // #125. Zonder OS weet de launch-kant niet welke shell hij aanspreekt, en
    // zonder werkmap moet er alsnog iets ingevuld worden. De machine die zich
    // aankondigt weet allebei van zichzelf, dus zegt hij het er meteen bij.
    pub os: String,
    pub home: String,
    // WAAR het verzoek over gaat. Een vraag wijst altijd naar één agent: juist dat
    // maakt het onmogelijk om per ongeluk "op een computer" uit te komen.
    pub agent_title: String,
    pub agent_cwd: String,
    // Eenmalig, en genoeg bewijs voor deze ene sessie. De uitnodiging ís de
    // toestemming, dus er hoort geen tweede popup bij de vrager.
    pub token: String,
}

#[derive(Default)]
pub struct Discovery {
    daemon: Mutex<Option<ServiceDaemon>>,
    seen: Mutex<HashMap<String, Found>>,
    browsing: AtomicBool,
    announced: Mutex<Option<String>>,
    // Waarom er niets te zien is, als er niets te zien is. Een lege lijst die
    // "geblokkeerd" betekent is de slechtste uitkomst: die leest als "er is
    // niemand". Eén eerlijke regel, geen workflow.
    problem: Mutex<String>,
}

impl Discovery {
    fn daemon(&self) -> Result<ServiceDaemon, String> {
        let mut slot = self.daemon.lock().unwrap();
        if let Some(d) = slot.as_ref() {
            return Ok(d.clone());
        }
        let d = ServiceDaemon::new().map_err(|e| format!("mDNS starten: {e}"))?;
        *slot = Some(d.clone());
        Ok(d)
    }

    pub fn problem(&self) -> String {
        self.problem.lock().unwrap().clone()
    }

    pub fn list(&self) -> Vec<Found> {
        let mut v: Vec<Found> = self.seen.lock().unwrap().values().cloned().collect();
        v.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        v
    }

    // Zoeken start bij het openen van het machinescherm en stopt bij het sluiten.
    pub fn browse_start(self: &Arc<Self>) -> Result<(), String> {
        if self.browsing.swap(true, Ordering::SeqCst) {
            return Ok(());
        }
        let d = match self.daemon() {
            Ok(d) => d,
            Err(e) => {
                self.browsing.store(false, Ordering::SeqCst);
                *self.problem.lock().unwrap() = e.clone();
                return Err(e);
            }
        };
        let rx = match d.browse(SERVICE) {
            Ok(rx) => rx,
            Err(e) => {
                self.browsing.store(false, Ordering::SeqCst);
                let msg = format!("zoeken op het netwerk lukt niet: {e}");
                *self.problem.lock().unwrap() = msg.clone();
                return Err(msg);
            }
        };
        *self.problem.lock().unwrap() = String::new();
        let me = self.clone();
        std::thread::spawn(move || {
            while me.browsing.load(Ordering::SeqCst) {
                // Met een timeout in plaats van blokkerend: anders blijft de thread
                // op een stil netwerk hangen tot er iets langskomt, en dan stopt
                // "sluit het scherm" pas bij de volgende aankondiging.
                match rx.recv_timeout(std::time::Duration::from_millis(400)) {
                    Ok(ServiceEvent::ServiceResolved(svc)) => me.absorb(&svc),
                    Ok(ServiceEvent::ServiceRemoved(_, full)) => {
                        me.seen.lock().unwrap().remove(&full);
                    }
                    Ok(_) => {}
                    Err(mdns_sd::RecvTimeoutError::Timeout) => {}
                    Err(_) => break,
                }
            }
        });
        Ok(())
    }

    pub fn browse_stop(&self) {
        self.browsing.store(false, Ordering::SeqCst);
        self.seen.lock().unwrap().clear();
    }

    fn absorb(&self, svc: &ResolvedService) {
        let full = svc.fullname.clone();
        // Onszelf niet in de lijst: je eigen machine "vinden" is ruis.
        if self.announced.lock().unwrap().as_deref() == Some(full.as_str()) {
            return;
        }
        let Some(address) = pick_ipv4(svc) else { return };
        let found = Found {
            name: label_from_fullname(&full),
            address,
            port: svc.port,
            user: prop(svc, "user"),
            fingerprint: prop(svc, "fp"),
            os: prop(svc, "os"),
            home: prop(svc, "home"),
            agent_title: prop(svc, "task"),
            agent_cwd: prop(svc, "cwd"),
            token: prop(svc, "tok"),
        };
        // Een aankondiging zonder token is geen VRAAG maar aanwezigheid, en daar
        // doen we niets meer mee. Zo kan een oudere Taurus op het netwerk ook geen
        // rijen opleveren waar niemand iets aan heeft.
        if found.token.trim().is_empty() {
            return;
        }
        // Sleutelen op de VOLLEDIGE naam en bijwerken: zie meting 2 hierboven.
        self.seen.lock().unwrap().insert(full, found);
    }

    // Een hulpvraag aankondigen. Zolang deze open staat is de machine zichtbaar;
    // trek je hem in, dan bestaat hij niet meer -- zoals bluetooth in koppelmodus.
    //
    // Aankondigen kan alleen als de deur ook open staat: de listener moet aan zijn
    // en het netwerk vertrouwd, want een vraag die niemand kan beantwoorden is geen
    // vraag. De aankondiging beschrijft daarmee precies het netwerk waarop de
    // listener luistert.
    #[allow(clippy::too_many_arguments)]
    pub fn announce(
        &self,
        port: u16,
        user: &str,
        fingerprint: &str,
        agent_title: &str,
        agent_cwd: &str,
        token: &str,
    ) -> Result<(), String> {
        self.unannounce();
        let Some(ip) = netgate::trusted_ipv4() else {
            return Err("Geen vertrouwd netwerk, dus geen adres om op aan te kondigen.".to_string());
        };
        let d = self.daemon()?;
        let name = instance_name();
        let props = [
            ("user".to_string(), user.to_string()),
            ("fp".to_string(), fingerprint.to_string()),
            ("os".to_string(), announced_os().to_string()),
            ("home".to_string(), home_dir()),
            ("task".to_string(), agent_title.to_string()),
            ("cwd".to_string(), agent_cwd.to_string()),
            ("tok".to_string(), token.to_string()),
        ];
        let info = ServiceInfo::new(
            SERVICE,
            &name,
            &format!("{name}.local."),
            std::net::IpAddr::V4(ip),
            port,
            &props[..],
        )
        .map_err(|e| format!("aankondiging opstellen: {e}"))?;
        let full = info.get_fullname().to_string();
        d.register(info).map_err(|e| format!("aankondigen: {e}"))?;
        *self.announced.lock().unwrap() = Some(full);
        Ok(())
    }

    pub fn unannounce(&self) {
        let full = self.announced.lock().unwrap().take();
        if let (Some(full), Ok(d)) = (full, self.daemon()) {
            let _ = d.unregister(&full);
        }
    }
}

fn prop(svc: &ResolvedService, key: &str) -> String {
    svc.txt_properties
        .get_property_val_str(key)
        .unwrap_or_default()
        .to_string()
}

// Alleen IPv4: dat is wat de rest van Taurus als hostname wegschrijft, en een
// link-local IPv6 met een scope-id erachter is in hosts.json voor niemand leesbaar.
// Loopback overslaan -- een machine die zichzelf op 127.0.0.1 aankondigt is voor
// deze kant niets waard.
fn pick_ipv4(svc: &ResolvedService) -> Option<String> {
    svc.addresses
        .iter()
        .find(|a| a.is_ipv4() && !a.is_loopback())
        .map(|a| a.to_string())
}

// "ursu._taurus._tcp.local." -> "ursu". De volledige naam is de sleutel; dit is
// alleen wat de gebruiker leest.
fn label_from_fullname(full: &str) -> String {
    full.strip_suffix(&format!(".{SERVICE}"))
        .unwrap_or(full)
        .to_string()
}

// Het OS zoals de rest van Taurus het noemt (hosts.json kent "windows" en "linux").
// macOS draait dezelfde POSIX-kant op, dus die valt daar bewust onder.
fn announced_os() -> &'static str {
    if cfg!(windows) {
        "windows"
    } else {
        "linux"
    }
}

// Waar een sessie op deze machine zou beginnen als niemand iets kiest.
fn home_dir() -> String {
    for key in ["USERPROFILE", "HOME"] {
        if let Ok(v) = std::env::var(key) {
            if !v.trim().is_empty() {
                return v;
            }
        }
    }
    String::new()
}

// De naam waaronder deze machine zich aankondigt: zijn eigen naam, want dat is
// wat de collega verwacht te zien staan.
fn instance_name() -> String {
    for key in ["COMPUTERNAME", "HOSTNAME"] {
        if let Ok(v) = std::env::var(key) {
            let v = v.trim().to_string();
            if !v.is_empty() {
                return v;
            }
        }
    }
    "taurus".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_label_is_the_instance_name_without_the_service_suffix() {
        assert_eq!(label_from_fullname("ursu._taurus._tcp.local."), "ursu");
        // Een naam die het achtervoegsel niet heeft blijft heel: liever een lange
        // naam tonen dan er eentje half afknippen.
        assert_eq!(label_from_fullname("ursu"), "ursu");
    }

    // De machine kondigt zich onder zijn eigen naam aan; alleen als er echt niets
    // te vinden is valt hij terug, en dan nog op iets leesbaars.
    #[test]
    fn the_instance_name_falls_back_to_something_readable() {
        let n = instance_name();
        assert!(!n.is_empty());
        assert!(!n.contains('.'), "een punt zou de servicenaam breken: {n}");
    }

    // Echt aankondigen en echt zoeken, in één proces -- de meting uit de spike
    // nagespeeld: daar loste dezelfde dienst NEGEN keer in acht seconden op,
    // telkens opnieuw vurend terwijl er adressen bijkwamen. Als deze test één rij
    // oplevert klopt de sleutel; als hij er meer oplevert staat straks één machine
    // als een handvol regels op het scherm.
    //
    // #[ignore] omdat hij UDP 5353 bindt: dat vraagt op Windows een eigen
    // firewall-uitzondering en levert anders een Defender-prompt op. Draaien met
    // `cargo test --lib -- --ignored --nocapture announce_and_find`.
    #[test]
    #[ignore]
    fn announce_and_find_the_same_machine_once() {
        let d = ServiceDaemon::new().expect("mDNS-daemon");
        let props = [
            ("user".to_string(), "arjen".to_string()),
            ("fp".to_string(), "SHA256:test".to_string()),
            ("os".to_string(), "windows".to_string()),
            ("home".to_string(), r"C:\Users\arjen".to_string()),
        ];
        let info = ServiceInfo::new(
            SERVICE,
            "testmachine",
            "testmachine.local.",
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 2, 9)),
            8287,
            &props[..],
        )
        .expect("aankondiging opstellen");
        d.register(info).expect("aankondigen");

        let disco = Arc::new(Discovery::default());
        *disco.daemon.lock().unwrap() = Some(d.clone());
        disco.browse_start().expect("zoeken starten");

        // Ruim de tijd om meerdere resolves op te vangen: juist die herhaling is
        // wat hier getoetst wordt.
        std::thread::sleep(std::time::Duration::from_secs(8));
        let list = disco.list();
        disco.browse_stop();
        let _ = d.unregister(&format!("testmachine.{SERVICE}"));

        let ours: Vec<_> = list.iter().filter(|f| f.name == "testmachine").collect();
        assert_eq!(ours.len(), 1, "een machine hoort een rij te zijn: {list:?}");
        let f = ours[0];
        assert_eq!(f.address, "192.168.2.9");
        assert_eq!(f.port, 8287);
        assert_eq!(f.user, "arjen");
        assert_eq!(f.os, "windows");
        assert_eq!(f.home, r"C:\Users\arjen");
    }
}
