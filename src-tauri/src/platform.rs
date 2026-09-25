// Platformverschillen die niet in één regel passen, op één plek (#199).
// Windows blijft precies zoals het was; de rest krijgt de gewone plekken van
// zijn eigen OS in plaats van Windows-variabelen die daar niet bestaan.

use std::path::PathBuf;

fn var(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|v| !v.trim().is_empty())
}

// De thuismap van de gebruiker. Windows: USERPROFILE (HOME als terugval, voor
// Git Bash). Elders: HOME. None als niets gezet is; de aanroeper kiest zelf wat
// dan redelijk is.
pub fn home_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    let v = var("USERPROFILE").or_else(|| var("HOME"));
    #[cfg(not(windows))]
    let v = var("HOME");
    v.map(PathBuf::from)
}

// Als string, leeg als er geen thuismap is. Voor plekken die een String bouwen.
pub fn home_string() -> String {
    home_dir().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default()
}

// Waar per-gebruiker-config van apps hoort, zonder "Taurus" erachter.
// Windows: %APPDATA%. macOS: ~/Library/Application Support. Andere Unix:
// $XDG_CONFIG_HOME, anders ~/.config.
//
// GEMETEN op macOS (#208): zonder dit viel config_dir terug op "./Taurus", en
// een .app start in "/" -- elke schrijfactie in de configmap faalde met os error 2.
pub fn config_base() -> Option<PathBuf> {
    #[cfg(windows)]
    return var("APPDATA").map(PathBuf::from);
    #[cfg(target_os = "macos")]
    return home_dir().map(|h| h.join("Library").join("Application Support"));
    #[cfg(all(unix, not(target_os = "macos")))]
    return var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| home_dir().map(|h| h.join(".config")));
}

// Beëindig een procesboom (#218). Windows doet dit met een Job Object
// (kill-on-close, zie `mod job`); elders stuurt portable-pty's child.kill() alleen
// een SIGHUP naar het agentproces zelf, en wat dat in een eigen procesgroep
// startte (een losgekoppelde achtergrondtaak, een server met `detached`) bleef
// draaien onder launchd.
//
// Eerst de hele boom verzamelen: zodra een ouder weg is, hangt zijn kind onder
// launchd en is het verband kwijt. Dan SIGTERM voor iedereen, en na twee seconden
// SIGKILL voor wie er dan nog is.
#[cfg(unix)]
pub fn kill_tree(root: u32) {
    let mut pids = descendants(root);
    pids.push(root);
    for &p in &pids {
        unsafe { libc::kill(p as libc::pid_t, libc::SIGTERM) };
    }
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(2));
        for &p in &pids {
            if unsafe { libc::kill(p as libc::pid_t, 0) } == 0 {
                unsafe { libc::kill(p as libc::pid_t, libc::SIGKILL) };
            }
        }
    });
}

// Alle afstammelingen van `root` (niet root zelf), uit één `ps`-momentopname.
#[cfg(unix)]
fn descendants(root: u32) -> Vec<u32> {
    let Ok(out) = std::process::Command::new("ps")
        .args(["-axo", "pid=,ppid="])
        .stderr(std::process::Stdio::null())
        .output()
    else {
        return Vec::new();
    };
    descendants_in(&String::from_utf8_lossy(&out.stdout), root)
}

#[cfg(unix)]
fn descendants_in(ps: &str, root: u32) -> Vec<u32> {
    let mut kids: std::collections::HashMap<u32, Vec<u32>> = Default::default();
    for line in ps.lines() {
        let mut it = line.split_whitespace().filter_map(|x| x.parse::<u32>().ok());
        if let (Some(pid), Some(ppid)) = (it.next(), it.next()) {
            kids.entry(ppid).or_default().push(pid);
        }
    }
    let mut out = Vec::new();
    let mut todo = vec![root];
    while let Some(p) = todo.pop() {
        for &c in kids.get(&p).map(|v| v.as_slice()).unwrap_or(&[]) {
            if c != root && !out.contains(&c) {
                out.push(c);
                todo.push(c);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_base_is_an_absolute_per_user_folder() {
        let b = config_base().expect("config_base");
        assert!(b.is_absolute(), "{:?}", b);
        #[cfg(target_os = "macos")]
        assert!(b.ends_with("Library/Application Support"), "{:?}", b);
    }

    #[test]
    fn home_dir_is_absolute() {
        assert!(home_dir().expect("home").is_absolute());
    }

    #[cfg(unix)]
    #[test]
    fn descendants_walks_the_whole_tree_and_nothing_else() {
        let ps = "  1     0\n 10     1\n 11    10\n 12    11\n 13    10\n 20     1\n 21    20\n";
        let mut d = descendants_in(ps, 10);
        d.sort();
        assert_eq!(d, vec![11, 12, 13]);
        assert!(descendants_in(ps, 99).is_empty());
    }

    // GEMETEN (#218): een kleinkind in een eigen procesgroep -- zoals een node-
    // proces met `detached` -- overleeft de SIGHUP die portable-pty's kill()
    // stuurt. kill_tree ruimt het wel op. Loopt een paar seconden.
    #[cfg(unix)]
    #[test]
    fn kill_tree_also_ends_a_detached_grandchild() {
        use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
        let alive = |p: u32| unsafe { libc::kill(p as libc::pid_t, 0) } == 0;
        // Elke start een eigen pty: na de hangup is een pty niet opnieuw te gebruiken.
        let spawn = || {
            let pair = NativePtySystem::default()
                .openpty(PtySize { rows: 24, cols: 80, pixel_width: 0, pixel_height: 0 })
                .unwrap();
            let mut c = CommandBuilder::new("/bin/sh");
            c.args(["-c", "perl -e 'setpgrp(0,0); sleep 60' & sleep 60"]);
            c.cwd(std::env::temp_dir());
            let child = pair.slave.spawn_command(c).unwrap();
            (pair, child)
        };
        let grandchild = |root: u32| -> u32 {
            for _ in 0..50 {
                if let Some(&g) = descendants(root).iter().find(|&&p| p != root) {
                    return g;
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            panic!("geen kleinkind");
        };

        // Zonder kill_tree: alleen de SIGHUP van portable-pty.
        let (_pty, mut child) = spawn();
        let root = child.process_id().unwrap();
        let orphans: Vec<u32> = {
            let _ = grandchild(root);
            std::thread::sleep(std::time::Duration::from_millis(200));
            let d = descendants(root);
            child.kill().unwrap();
            let _ = child.wait();
            std::thread::sleep(std::time::Duration::from_millis(300));
            d.into_iter().filter(|&p| alive(p)).collect()
        };
        assert!(!orphans.is_empty(), "verwacht: iets overleeft een kale kill()");
        for p in &orphans {
            unsafe { libc::kill(*p as libc::pid_t, libc::SIGKILL) };
        }

        // Met kill_tree: niets over.
        let (_pty2, mut child) = spawn();
        let root = child.process_id().unwrap();
        let _ = grandchild(root);
        std::thread::sleep(std::time::Duration::from_millis(200));
        let tree = descendants(root);
        kill_tree(root);
        let _ = child.wait();
        std::thread::sleep(std::time::Duration::from_millis(2500));
        let left: Vec<u32> = tree.into_iter().filter(|&p| alive(p)).collect();
        assert!(left.is_empty(), "nog in leven: {:?}", left);
    }
}
