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
}
