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
    println!("cargo:rustc-env=TAURUS_BUILT_AT={}", secs);
    // Zonder dit hergebruikt cargo een eerdere build en blijft het moment staan.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=../src");
    tauri_build::build()
}
