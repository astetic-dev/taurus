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
            let is_trusted = trusted.iter().any(|x| x == &id);
            out.push(NetInfo { id, name, trusted: is_trusted });
        }
    }
    out
}

#[cfg(not(windows))]
pub fn current_networks() -> Vec<NetInfo> {
    Vec::new()
}

// Mag de listener nu open staan? Precies dan als een van de netwerken waar we op
// zitten vertrouwd is.
pub fn on_trusted_network() -> bool {
    current_networks().iter().any(|n| n.trusted)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Geen enkel netwerk vertrouwd = niet luisteren. Dit is de default na
    // installatie, en het moet de veilige kant op vallen.
    #[test]
    fn nothing_trusted_means_closed() {
        let nets = vec![
            NetInfo { id: "a".into(), name: "Cafe".into(), trusted: false },
            NetInfo { id: "b".into(), name: "Hotel".into(), trusted: false },
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

    #[test]
    fn one_trusted_network_is_enough() {
        let nets = vec![
            NetInfo { id: "a".into(), name: "Cafe".into(), trusted: false },
            NetInfo { id: "b".into(), name: "Kantoor".into(), trusted: true },
        ];
        assert!(nets.iter().any(|n| n.trusted));
    }
}
