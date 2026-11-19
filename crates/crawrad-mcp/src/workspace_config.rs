//! The project's `.mcp.json`: the entry that points an agent's CLI at this
//! IDE instance through `crawrad-mcp-bridge`. Pure file work — moved out of the
//! shell (`ide_app/mcp_host.rs`), which only calls it.

use crawrad_note::note;
use serde_json::json;

#[note("workspace-config-sync")]
pub fn sync(workspace_root: &std::path::Path, instance_id: &str) {
    let bridge_cmd = match std::env::current_exe() {
        Ok(exe) => {
            let mut bridge = exe;
            bridge.set_file_name(if cfg!(windows) {
                "crawrad-mcp-bridge.exe"
            } else {
                "crawrad-mcp-bridge"
            });
            if bridge.is_file() {
                bridge.display().to_string()
            } else {
                "crawrad-mcp-bridge".to_string()
            }
        }
        Err(_) => "crawrad-mcp-bridge".to_string(),
    };

    let mcp_json_path = workspace_root.join(".mcp.json");
    let mut config_obj = if mcp_json_path.is_file() {
        std::fs::read_to_string(&mcp_json_path)
            .ok()
            .and_then(|txt| serde_json::from_str::<serde_json::Value>(&txt).ok())
            .unwrap_or_else(|| json!({}))
    } else {
        json!({})
    };

    if !config_obj.is_object() {
        config_obj = json!({});
    }

    let mcp_servers = config_obj
        .as_object_mut()
        .unwrap()
        .entry("mcpServers")
        .or_insert_with(|| json!({}));

    if let Some(servers) = mcp_servers.as_object_mut() {
        servers.insert(
            "crawrad".to_string(),
            json!({
                "command": bridge_cmd,
                "args": ["--instance", instance_id]
            }),
        );
    }

    if let Ok(content) = serde_json::to_string_pretty(&config_obj) {
        let _ = std::fs::write(mcp_json_path, content);
    }
}

#[cfg(test)]
#[path = "test/workspace_config/test.rs"]
mod tests;
