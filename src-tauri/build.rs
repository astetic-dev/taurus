fn main() {
    // Het buildmoment in de binary bakken. Sinds versienummers pas bij een release
    // wijzigen, is het nummer geen onderscheid meer tussen twee builds van dezelfde
    // avond -- en dan weet je niet of je je eigen wijziging voor je hebt. Nu zegt
    // het venster zelf welke build het is.
    //
    // Seconden sinds epoch: geen datumbibliotheek nodig, en de frontend heeft de
    // klok en de tijdzone van de gebruiker al.
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Een RELEASE draagt geen buildmoment en geen ui-markering: dat is
    // debug-gereedschap, en een datum naast het versienummer hoort niet in iets dat
    // je uitlevert. `TAURUS_RELEASE=1 cargo build --release` zet het uit; zonder die
    // variabele blijft het staan, want tijdens het werken is het juist nodig.
    let release = std::env::var("TAURUS_RELEASE")
        .map(|v| !v.is_empty() && v != "0")
        .unwrap_or(false);
    println!("cargo:rustc-env=TAURUS_BUILT_AT={}", if release { 0 } else { secs });
    println!("cargo:rerun-if-env-changed=TAURUS_RELEASE");
    // Zonder dit hergebruikt cargo een eerdere build en blijft het moment staan.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=../src");
    tauri_build::build()
}
