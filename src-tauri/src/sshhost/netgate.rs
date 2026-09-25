// De netwerk-poort voor #121: het vinkje "bereikbaar" komt pas ergens op neer
// als je op een netwerk zit dat je vertrouwt. Op kantoor luistert Taurus, op de
// wifi van een cafe niet -- zonder dat je aan de instelling hoeft te denken.
//
// WEES EERLIJK over wat dit is: een poort tegen ONGELUKKEN, geen
// beveiligingsgrens. Een vijandig netwerk kan de naam van een vertrouwd netwerk
// nabootsen. Identiteit blijft de sleutel-fingerprint en de toestemmingspopups;
// dit voorkomt alleen dat je per ongeluk luistert waar je dat niet wilde.
use serde::{Deserialize, Serialize};

use crate::config_dir;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct NetInfo {
    // Het GUID van het netwerk: stabiel, ook als het netwerk hernoemd wordt.
    pub id: String,
    pub name: String,
    pub trusted: bool,
    // Wat Windows zelf van dit netwerk vindt: "public" / "private" / "domain".
    // Hoort in beeld VOORDAT je iets vertrouwt -- op een openbaar netwerk is
    // "laat collega's aankloppen" zelden wat je bedoelde. Gemeten: ursu stond
    // op public terwijl hetzelfde netwerk hier als private geldt, dus dit
    // verschilt echt per machine.
    #[serde(default)]
    pub category: String,
}

fn trusted_file() -> std::path::PathBuf {
    config_dir().join("trusted-networks.json")
}

pub fn read_trusted() -> Vec<String> {
    std::fs::read_to_string(trusted_file())
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_default()
}

pub fn set_trusted(id: &str, trusted: bool) -> Result<(), String> {
    let mut list = read_trusted();
    list.retain(|x| x != id);
    if trusted {
        list.push(id.to_string());
    }
    std::fs::create_dir_all(config_dir()).map_err(|e| format!("map aanmaken: {e}"))?;
    let txt = serde_json::to_string_pretty(&list).map_err(|e| format!("serialiseren: {e}"))?;
    std::fs::write(trusted_file(), txt).map_err(|e| format!("schrijven: {e}"))
}

// De netwerken waar deze machine NU op zit.
#[cfg(windows)]
pub fn current_networks() -> Vec<NetInfo> {
    use windows::Win32::Networking::NetworkListManager::{
        INetworkListManager, NetworkListManager, NLM_ENUM_NETWORK_CONNECTED,
    };
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED,
    };

    let trusted = read_trusted();
    let mut out = Vec::new();
    unsafe {
        // Al geinitialiseerd op deze thread is geen fout die ons iets kan schelen.
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let mgr: INetworkListManager =
            match CoCreateInstance(&NetworkListManager, None, CLSCTX_ALL) {
                Ok(m) => m,
                Err(_) => return out,
            };
        let Ok(nets) = mgr.GetNetworks(NLM_ENUM_NETWORK_CONNECTED) else {
            return out;
        };
        loop {
            let mut item = [const { None }; 1];
            let mut fetched: u32 = 0;
            if nets.Next(&mut item, Some(&mut fetched)).is_err() || fetched == 0 {
                break;
            }
            let Some(n) = item[0].take() else { break };
            let name = n.GetName().map(|b| b.to_string()).unwrap_or_default();
            let id = n
                .GetNetworkId()
                .map(|g| format!("{g:?}"))
                .unwrap_or_default();
            if id.is_empty() {
                continue;
            }
            let category = match n.GetCategory().map(|c| c.0) {
                Ok(0) => "public",
                Ok(1) => "private",
                Ok(2) => "domain",
                _ => "",
            }
            .to_string();
            let is_trusted = trusted.iter().any(|x| x == &id);
            out.push(NetInfo { id, name, trusted: is_trusted, category });
        }
    }
    out
}

// macOS: het netwerk is de gateway van de standaardroute plus de DHCP-domeinnaam
// (#220). Zwakker dan het GUID van Windows -- twee netwerken met hetzelfde
// routeradres en domein zijn hier hetzelfde -- maar dit is een poort tegen
// ongelukken, geen grens.
//
// GEMETEN: het MAC-adres van de gateway was de eerste keus, maar een proces dat
// een app start ziet de ARP-tabel leeg (`arp -an` geeft niets, `netstat -rn`
// toont link#16), terwijl dezelfde opdracht vanuit een shell hem wel toont. LAN-
// verbindingen werken wel. De Wi-Fi-naam (SSID) vraagt locatietoestemming.
#[cfg(target_os = "macos")]
pub fn current_networks() -> Vec<NetInfo> {
    let Some(r) = mac::default_route() else { return Vec::new() };
    let trusted = read_trusted();
    let id = mac::network_id(&r.gateway, r.domain.as_deref());
    let name = match &r.domain {
        Some(d) => format!("{} · {} ({})", r.port_name, r.gateway, d),
        None => format!("{} · {}", r.port_name, r.gateway),
    };
    let is_trusted = trusted.iter().any(|x| x == &id);
    vec![NetInfo { id, name, trusted: is_trusted, category: String::new() }]
}

#[cfg(all(not(windows), not(target_os = "macos")))]
pub fn current_networks() -> Vec<NetInfo> {
    Vec::new()
}

// Mag de listener nu open staan? Precies dan als een van de netwerken waar we op
// zitten vertrouwd is.
pub fn on_trusted_network() -> bool {
    current_networks().iter().any(|n| n.trusted)
}

// Het IPv4-adres van de interface achter een vertrouwd netwerk (#125).
//
// GEMETEN in de spike: mdns-sd's `enable_addr_auto()` adverteert ELK adres van
// ELKE interface. Op dit werkstation waren dat naast 192.168.2.13 ook de
// Hyper-V- en WSL-interne ranges, loopback en link-local IPv6. Dat vertelt de
// hele LAN hoe de machine intern is opgedeeld, en geen van die adressen is voor
// de andere kant bruikbaar. Dus: aankondigen op precies de interface waarover
// het vertrouwde netwerk loopt -- hetzelfde netwerk waar de gate al over besloot.
#[cfg(windows)]
pub fn trusted_ipv4() -> Option<std::net::Ipv4Addr> {
    let ids = trusted_adapter_ids();
    if ids.is_empty() {
        return None;
    }
    adapter_ipv4s()
        .into_iter()
        .find(|(guid, _)| ids.iter().any(|id| same_guid(id, guid)))
        .map(|(_, ip)| ip)
}

// De twee bronnen schrijven hetzelfde GUID anders op, en dat is niet cosmetisch.
//
// GEMETEN: de Network List Manager gaf `20BBA5A7-53A0-4E38-BC79-96E968EC14E4` en
// `GetAdaptersAddresses` gaf `{20BBA5A7-53A0-4E38-BC79-96E968EC14E4}` -- mét
// accolades. Een kale string-vergelijking matchte dus NOOIT, waardoor
// `trusted_ipv4()` altijd None gaf en de aankondiging op elke machine stilzwijgend
// achterwege bleef. Geen foutmelding, want "geen vertrouwd netwerk" is een geldige
// toestand; precies het soort stilte waar je maanden overheen kijkt.
#[cfg_attr(not(windows), allow(dead_code))]
fn same_guid(a: &str, b: &str) -> bool {
    let norm = |s: &str| s.trim().trim_start_matches('{').trim_end_matches('}').to_ascii_lowercase();
    let (a, b) = (norm(a), norm(b));
    !a.is_empty() && a == b
}

// De adapter-GUIDs van de netwerken die we vertrouwen. Een netwerk kan meer dan
// een verbinding hebben (bekabeld en draadloos naar dezelfde LAN), dus dit is een
// lijst en niet een waarde.
#[cfg(windows)]
fn trusted_adapter_ids() -> Vec<String> {
    use windows::Win32::Networking::NetworkListManager::{
        INetworkListManager, NetworkListManager, NLM_ENUM_NETWORK_CONNECTED,
    };
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED,
    };

    let trusted = read_trusted();
    let mut out = Vec::new();
    if trusted.is_empty() {
        return out;
    }
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let Ok(mgr) = CoCreateInstance::<_, INetworkListManager>(&NetworkListManager, None, CLSCTX_ALL)
        else {
            return out;
        };
        let Ok(nets) = mgr.GetNetworks(NLM_ENUM_NETWORK_CONNECTED) else {
            return out;
        };
        loop {
            let mut item = [const { None }; 1];
            let mut fetched: u32 = 0;
            if nets.Next(&mut item, Some(&mut fetched)).is_err() || fetched == 0 {
                break;
            }
            let Some(n) = item[0].take() else { break };
            let id = n.GetNetworkId().map(|g| format!("{g:?}")).unwrap_or_default();
            if !trusted.iter().any(|x| x == &id) {
                continue;
            }
            let Ok(conns) = n.GetNetworkConnections() else { continue };
            loop {
                let mut c = [const { None }; 1];
                let mut got: u32 = 0;
                if conns.Next(&mut c, Some(&mut got)).is_err() || got == 0 {
                    break;
                }
                let Some(conn) = c[0].take() else { break };
                if let Ok(g) = conn.GetAdapterId() {
                    out.push(format!("{g:?}"));
                }
            }
        }
    }
    out
}

// (adapter-GUID, IPv4) voor elke interface die er een heeft. GetAdaptersAddresses
// geeft de GUID terug als AdapterName, in precies de vorm die de Network List
// Manager ook gebruikt -- dat is wat de twee bronnen aan elkaar knoopt.
#[cfg(windows)]
fn adapter_ipv4s() -> Vec<(String, std::net::Ipv4Addr)> {
    use windows::Win32::NetworkManagement::IpHelper::{
        GetAdaptersAddresses, GAA_FLAG_SKIP_ANYCAST, GAA_FLAG_SKIP_DNS_SERVER,
        GAA_FLAG_SKIP_MULTICAST, IP_ADAPTER_ADDRESSES_LH,
    };
    use windows::Win32::Networking::WinSock::{AF_INET, SOCKADDR_IN};

    let mut out = Vec::new();
    let flags = GAA_FLAG_SKIP_ANYCAST | GAA_FLAG_SKIP_MULTICAST | GAA_FLAG_SKIP_DNS_SERVER;
    unsafe {
        // Twee rondes: eerst vragen hoe groot de buffer moet zijn, dan vullen.
        // De API kan tussendoor groeien, vandaar de ruime marge en een harde stop.
        let mut size: u32 = 16 * 1024;
        let mut buf: Vec<u8> = Vec::new();
        for _ in 0..3 {
            buf.resize(size as usize, 0);
            let r = GetAdaptersAddresses(
                AF_INET.0 as u32,
                flags,
                None,
                Some(buf.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH),
                &mut size,
            );
            // 111 = ERROR_BUFFER_OVERFLOW: nog een keer met de gevraagde maat.
            if r == 111 {
                continue;
            }
            if r != 0 {
                return out;
            }
            let mut p = buf.as_ptr() as *const IP_ADAPTER_ADDRESSES_LH;
            while !p.is_null() {
                let a = &*p;
                let guid = if a.AdapterName.is_null() {
                    String::new()
                } else {
                    a.AdapterName.to_string().unwrap_or_default()
                };
                let mut ua = a.FirstUnicastAddress;
                while !ua.is_null() {
                    let sa = (*ua).Address.lpSockaddr;
                    if !sa.is_null() && (*sa).sa_family == AF_INET {
                        let v4 = &*(sa as *const SOCKADDR_IN);
                        let o = v4.sin_addr.S_un.S_addr.to_ne_bytes();
                        let ip = std::net::Ipv4Addr::new(o[0], o[1], o[2], o[3]);
                        if !ip.is_loopback() && !ip.is_unspecified() && !guid.is_empty() {
                            out.push((guid.clone(), ip));
                        }
                    }
                    ua = (*ua).Next;
                }
                p = a.Next;
            }
            return out;
        }
    }
    out
}

#[cfg(target_os = "macos")]
pub fn trusted_ipv4() -> Option<std::net::Ipv4Addr> {
    if !on_trusted_network() {
        return None;
    }
    let r = mac::default_route()?;
    mac::run("ipconfig", &["getifaddr", &r.interface])?.trim().parse().ok()
}

#[cfg(all(not(windows), not(target_os = "macos")))]
pub fn trusted_ipv4() -> Option<std::net::Ipv4Addr> {
    None
}

#[cfg(target_os = "macos")]
mod mac {
    pub struct Route {
        pub gateway: String,
        pub interface: String,
        pub domain: Option<String>,
        pub port_name: String,
    }

    pub fn network_id(gateway: &str, domain: Option<&str>) -> String {
        match domain {
            Some(d) => format!("gw:{}/{}", gateway, d.to_ascii_lowercase()),
            None => format!("gw:{}", gateway),
        }
    }

    pub fn run(program: &str, args: &[&str]) -> Option<String> {
        let o = std::process::Command::new(program)
            .args(args)
            .stdin(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .output()
            .ok()?;
        o.status.success().then(|| String::from_utf8_lossy(&o.stdout).into_owned())
    }

    pub fn default_route() -> Option<Route> {
        let route = run("route", &["-n", "get", "default"])?;
        let field = |k: &str| {
            route.lines().find_map(|l| l.trim().strip_prefix(k).map(|v| v.trim().to_string()))
        };
        let gateway = field("gateway:")?;
        let interface = field("interface:")?;
        // Geen DHCP (vast adres) is geen fout: dan telt alleen de gateway.
        let domain = run("ipconfig", &["getpacket", &interface]).and_then(|t| dhcp_domain(&t));
        let port_name = run("networksetup", &["-listallhardwareports"])
            .and_then(|t| port_name_for(&t, &interface))
            .unwrap_or_else(|| interface.clone());
        Some(Route { gateway, interface, domain, port_name })
    }

    // "domain_name (string): home" uit `ipconfig getpacket`.
    pub fn dhcp_domain(packet: &str) -> Option<String> {
        packet
            .lines()
            .find_map(|l| l.trim().strip_prefix("domain_name (string):"))
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
    }

    // "Hardware Port: Wi-Fi\nDevice: en1" -> "Wi-Fi".
    pub fn port_name_for(listing: &str, device: &str) -> Option<String> {
        let mut port = None;
        for l in listing.lines() {
            if let Some(p) = l.strip_prefix("Hardware Port:") {
                port = Some(p.trim().to_string());
            } else if l.strip_prefix("Device:").map(str::trim) == Some(device) {
                return port;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "macos")]
    #[test]
    fn the_network_id_is_gateway_and_dhcp_domain() {
        let packet = "siaddr = 192.168.2.254\nserver_identifier (ip): 192.168.2.254\ndomain_name (string): Home\nrouter (ip_mult): {192.168.2.254}\n";
        let d = mac::dhcp_domain(packet);
        assert_eq!(d.as_deref(), Some("Home"));
        assert_eq!(mac::network_id("192.168.2.254", d.as_deref()), "gw:192.168.2.254/home");
        // Vast adres, geen DHCP: alleen de gateway.
        assert_eq!(mac::dhcp_domain("siaddr = 0.0.0.0\n"), None);
        assert_eq!(mac::network_id("10.0.0.1", None), "gw:10.0.0.1");
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn the_hardware_port_is_found_by_device() {
        let l = "Hardware Port: Ethernet\nDevice: en0\nEthernet Address: a\n\nHardware Port: Wi-Fi\nDevice: en1\n";
        assert_eq!(mac::port_name_for(l, "en1").as_deref(), Some("Wi-Fi"));
        assert_eq!(mac::port_name_for(l, "en9"), None);
    }

    // Geen enkel netwerk vertrouwd = niet luisteren. Dit is de default na
    // installatie, en het moet de veilige kant op vallen.
    #[test]
    fn nothing_trusted_means_closed() {
        let nets = vec![
            NetInfo { id: "a".into(), name: "Cafe".into(), trusted: false, category: "public".into() },
            NetInfo { id: "b".into(), name: "Hotel".into(), trusted: false, category: "public".into() },
        ];
        assert!(!nets.iter().any(|n| n.trusted));
    }

    // Handmatig: `cargo test -- --ignored --nocapture netwerken` laat zien wat
    // de Network List Manager op deze machine teruggeeft. Genegeerd in de
    // gewone run, want de uitkomst hangt af van waar de machine op dat moment
    // op zit.
    #[test]
    #[ignore]
    fn toon_netwerken() {
        for n in current_networks() {
            println!("{}  {}  vertrouwd={}", n.id, n.name, n.trusted);
        }
    }

    // Handmatig, net als hierboven: `cargo test -- --ignored --nocapture trusted_ip`
    // laat zien welk adres de aankondiging zou dragen. Moet het LAN-adres zijn en
    // nadrukkelijk NIET een Hyper-V- of WSL-intern adres (#125).
    #[test]
    #[ignore]
    #[cfg(windows)]
    fn toon_trusted_ip() {
        println!("alle adapters met een IPv4:");
        for (guid, ip) in adapter_ipv4s() {
            println!("  {ip}  {guid}");
        }
        println!("vertrouwde adapters: {:?}", trusted_adapter_ids());
        println!("aankondigen op: {:?}", trusted_ipv4());
    }

    // GEMETEN op dit werkstation, en de reden dat deze functie bestaat: dezelfde
    // adapter kwam als `20BBA5A7-...` uit de Network List Manager en als
    // `{20BBA5A7-...}` uit GetAdaptersAddresses. Zonder normaliseren matchte dat
    // nooit, en dan kondigt Taurus zich op geen enkele machine aan -- zonder fout,
    // want "geen vertrouwd netwerk" is een geldig antwoord.
    #[test]
    fn a_guid_with_braces_is_the_same_adapter() {
        assert!(same_guid(
            "20BBA5A7-53A0-4E38-BC79-96E968EC14E4",
            "{20BBA5A7-53A0-4E38-BC79-96E968EC14E4}"
        ));
        assert!(same_guid(
            "{20bba5a7-53a0-4e38-bc79-96e968ec14e4}",
            "20BBA5A7-53A0-4E38-BC79-96E968EC14E4"
        ));
        // Twee echt verschillende adapters blijven verschillend, en leeg matcht
        // nooit -- anders zou een adapter zonder naam op alles passen.
        assert!(!same_guid("{a}", "{b}"));
        assert!(!same_guid("", "{}"));
    }

    #[test]
    fn one_trusted_network_is_enough() {
        let nets = vec![
            NetInfo { id: "a".into(), name: "Cafe".into(), trusted: false, category: "public".into() },
            NetInfo { id: "b".into(), name: "Kantoor".into(), trusted: true, category: "domain".into() },
        ];
        assert!(nets.iter().any(|n| n.trusted));
    }
}
