//! The `.mcp.json` the IDE keeps in a project.

use serde_json::{Value, json};

#[test]
fn test_sync_workspace_mcp_config_creates_valid_mcp_json() {
    let temp_dir = std::env::temp_dir().join(format!("crawrad_mcp_test_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&temp_dir);

    super::sync(&temp_dir, "test-instance-123");

    let mcp_file = temp_dir.join(".mcp.json");
    assert!(
        mcp_file.is_file(),
        ".mcp.json must be created in workspace root"
    );

    let content = std::fs::read_to_string(&mcp_file).expect("read .mcp.json");
    let json_val: Value = serde_json::from_str(&content).expect("valid json");

    let server_entry = &json_val["mcpServers"]["crawrad"];
    assert!(server_entry.is_object());
    assert!(
        server_entry["command"]
            .as_str()
            .unwrap()
            .contains("crawrad-mcp-bridge")
    );
    assert_eq!(
        server_entry["args"],
        json!(["--instance", "test-instance-123"])
    );

    let _ = std::fs::remove_dir_all(&temp_dir);
}

/// **The Store build wrote a path nothing outside the package could run.**
///
/// An installed Store app lives under `C:\Program Files\WindowsApps\`, which
/// Windows locks down. The package reads its own folder, so the `is_file()` check
/// answered yes and the physical path went into `.mcp.json`; the agent's CLI, a
/// separate process, then got `fork/exec ... Access is denied` and started with no
/// MCP server at all. What the owner saw, three sessions in a row.
///
/// The real path is spelled with backslashes, which separate nothing on Unix - the
/// whole string would be ONE component there and the test would fail for that
/// reason and not for a defect. `WindowsApps` is the Store's own invention, so the
/// Windows spelling proves itself on Windows; the rule underneath is checked
/// everywhere, below.
#[cfg(windows)]
#[test]
fn a_path_inside_the_store_package_is_never_offered() {
    let store = std::path::Path::new(
        r"C:\Program Files\WindowsApps\GUARANA.CrawRAD_0.1.1.0_x64__2nygkg5m59wk2\crawrad-mcp-bridge.exe",
    );
    assert!(
        super::super::workspace_config::is_inside_package(store),
        "the Store's own install path was not recognised, so it would be written again"
    );
}

/// The rule itself, in a spelling every platform parses: a component named
/// `WindowsApps`, whatever its case, means the path belongs to the package.
///
/// Without this, nothing on Linux or macOS guards the rule at all, and a change
/// that broke it would only be caught by whoever happened to run the suite on
/// Windows.
#[test]
fn the_rule_holds_on_every_platform_in_a_portable_spelling() {
    use std::path::Path;
    let f = super::super::workspace_config::is_inside_package;

    assert!(f(Path::new("/var/lib/WindowsApps/crawrad-mcp-bridge")));
    assert!(
        f(Path::new("/var/lib/windowsapps/crawrad-mcp-bridge")),
        "case must not matter"
    );
    assert!(!f(Path::new("/usr/local/bin/crawrad-mcp-bridge")));
    assert!(
        !f(Path::new("/home/someone/MyWindowsApps/crawrad-mcp-bridge")),
        "a folder that merely CONTAINS the name is not the package folder"
    );
}

/// An ordinary install keeps the real path beside the IDE: the fix must not cost
/// everyone else the exact binary they were launching.
#[test]
fn an_ordinary_install_keeps_its_real_path() {
    for ok in [
        r"C:\Users\someone\CrawRAD\crawrad-mcp-bridge.exe",
        r"D:\dev\crawrad\target\release\crawrad-mcp-bridge.exe",
        "/usr/local/bin/crawrad-mcp-bridge",
    ] {
        assert!(
            !super::super::workspace_config::is_inside_package(std::path::Path::new(ok)),
            "`{ok}` was mistaken for a Store install and would lose its exact path"
        );
    }
}

/// The name written for a packaged install is the one the manifest declares its
/// execution alias under — if the two ever disagree, nothing resolves.
#[test]
fn the_bare_name_matches_the_alias_the_manifest_declares() {
    let cmd = super::super::workspace_config::bare_command();
    assert!(
        !cmd.contains('\\') && !cmd.contains('/'),
        "a path was offered where a bare name must be: `{cmd}`"
    );

    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../installer/windows/msix/AppxManifest.xml");
    let xml = std::fs::read_to_string(&manifest).expect("the MSIX manifest must be readable");
    assert!(
        xml.contains(r#"Alias="crawrad-mcp-bridge.exe""#),
        "the manifest declares no execution alias for the bridge, so the bare name \
         resolves to nothing on a Store install"
    );
    // THE TWO NAMES, ACTUALLY COMPARED. The assertion that used to stand here
    // searched the manifest for the same hand-written literal as the one above it,
    // so it compared the manifest with itself: `bare_command` could return any
    // name at all and this test stayed green (proved by mutation on 27/11/2026 -
    // `test_sync_workspace_mcp_config_creates_valid_mcp_json` caught it, this test
    // did not, although this is the test named for it). So the manifest's alias is
    // read out and compared with what `bare_command` actually returns.
    //
    // The extensions must come off first: `bare_command` returns `.exe` only on
    // Windows and the CI also runs Linux and macOS, so comparing the two verbatim
    // would fail there for a reason that has nothing to do with the bug.
    let declared = xml
        .split(r#"Alias=""#)
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .expect("the manifest declares no execution alias at all");
    let stem = |s: &str| s.trim_end_matches(".exe").to_string();
    assert_eq!(
        stem(declared),
        stem(&cmd),
        "the alias the manifest declares and the name written to .mcp.json disagree,          so the bare name resolves to nothing on a Store install"
    );

    // THE THIRD LINK. An alias pointing at a binary nobody ships resolves to
    // nothing, and the Store build loses its MCP server again by another door -
    // silently, because the manifest and the config would still agree with each
    // other. So the bundler's own list is checked here too: alias, written name
    // and shipped binary are one chain, and no link may be cut on its own.
    let bundler =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/bundle/bundle.py");
    let py = std::fs::read_to_string(&bundler).expect("the bundler must be readable");
    assert!(
        py.contains("\"crawrad-mcp-bridge\""),
        "the bundler no longer ships crawrad-mcp-bridge, so the manifest's          execution alias points at a binary that is not in the package"
    );
}
