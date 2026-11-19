//! The core stays portable: it depends on nothing of CrawRAD and not on the engine.

#[test]
fn the_core_depends_on_no_crawrad_crate_nor_on_the_engine() {
    let manifest = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("the core Cargo.toml is readable");
    for line in manifest.lines() {
        let name = line.split('=').next().unwrap_or("").trim();
        assert!(
            !name.starts_with("crawrad") && name != "portadb",
            "portadb-core must not depend on `{name}`"
        );
    }
}
