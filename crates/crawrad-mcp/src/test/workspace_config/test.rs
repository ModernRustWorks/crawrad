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
