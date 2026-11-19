use super::*;

#[test]
fn a_web_package_is_read_from_its_manifest_and_follows_an_edit() {
    let dir = std::env::temp_dir().join(format!("crawrad-test-page-look-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let manifest = dir.join("Cargo.toml");
    std::fs::write(&manifest, "[package]\nname = \"demo\"\n").unwrap();
    assert!(!is_web_package(&dir));
    std::thread::sleep(std::time::Duration::from_millis(20));
    std::fs::write(
        &manifest,
        "[package]\nname = \"demo\"\n\n[package.metadata.crawrad]\nplatform = \"web\"\n",
    )
    .unwrap();
    assert!(is_web_package(&dir));
    assert!(!is_web_package(&dir.join("missing")));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_theme_wins_and_system_follows_the_platform() {
    assert!(is_light(Some("thLight"), false));
    assert!(!is_light(Some("thDark"), true));
    assert!(is_light(Some("thSystem"), true));
    assert!(!is_light(Some("thSystem"), false));
    assert!(!is_light(None, false));
}
