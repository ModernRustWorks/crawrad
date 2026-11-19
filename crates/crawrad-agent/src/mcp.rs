use crawrad_note::note;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[note("mcp-mcptransport")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum McpTransport {
    /// Standard input/output process pipe.
    #[default]
    Stdio,
    /// Server-Sent Events / HTTP endpoint.
    Http,
}

#[note("mcp-mcpsource")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum McpSource {
    BuiltIn,
    Addon,
    Custom,
}

#[note("mcp-mcpserverconfig")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpServerConfig {
    /// Name identifier (e.g. `"github"`, `"filesystem"`, `"memory"`).
    pub name: String,
    /// Transport protocol.
    #[serde(default)]
    pub transport: McpTransport,
    /// Executable command line or URL (e.g. `"npx -y @modelcontextprotocol/server-github"`).
    pub command: String,
    /// Arguments for the command.
    #[serde(default)]
    pub args: Vec<String>,
    /// Environment variables to pass to the process.
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Origin of this server configuration.
    #[serde(default = "default_mcp_source")]
    pub source: McpSource,
    /// Whether this server is enabled for tool calls.
    #[serde(default = "default_true")]
    pub enabled: bool,
}

impl McpServerConfig {
    #[note("mcp-mcpserverconfig-probe")]
    pub fn probe(&self) -> bool {
        let cmd = self.command.trim();
        if cmd.is_empty() {
            return false;
        }
        if cmd.starts_with("http://") || cmd.starts_with("https://") {
            return false;
        }
        let binary_name = cmd.split_whitespace().next().unwrap_or(cmd);
        if let Ok(current_exe) = std::env::current_exe()
            && let Some(dir) = current_exe.parent()
        {
            for ext in &["", ".exe", ".cmd", ".bat"] {
                let candidate = dir.join(format!("{binary_name}{ext}"));
                if candidate.is_file() {
                    return true;
                }
            }
        }
        if let Ok(path_var) = std::env::var("PATH") {
            let extensions: &[&str] = if cfg!(windows) {
                &[".cmd", ".exe", ".bat", ""]
            } else {
                &[""]
            };
            for dir in std::env::split_paths(&path_var) {
                for ext in extensions {
                    let candidate = dir.join(format!("{binary_name}{ext}"));
                    if candidate.is_file() {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn default_mcp_source() -> McpSource {
    McpSource::Custom
}

fn default_true() -> bool {
    true
}

#[note("mcp-mcpregistry")]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpRegistry {
    pub servers: Vec<McpServerConfig>,
}

impl McpRegistry {
    pub fn new() -> Self {
        Self {
            servers: Self::default_servers(),
        }
    }

    #[note("mcp-mcpregistry-default-servers")]
    pub fn default_servers() -> Vec<McpServerConfig> {
        vec![
            McpServerConfig {
                name: "filesystem".into(),
                transport: McpTransport::Stdio,
                command: "crawrad-mcp-fs".into(),
                args: vec![],
                env: HashMap::new(),
                source: McpSource::BuiltIn,
                enabled: true,
            },
            McpServerConfig {
                name: "crawrad-ide".into(),
                transport: McpTransport::Stdio,
                command: "crawrad-mcp-bridge".into(),
                args: vec![],
                env: HashMap::new(),
                source: McpSource::BuiltIn,
                enabled: true,
            },
        ]
    }

    pub fn add_server(&mut self, server: McpServerConfig) {
        if let Some(existing) = self.servers.iter_mut().find(|s| s.name == server.name) {
            *existing = server;
        } else {
            self.servers.push(server);
        }
    }

    pub fn remove_server(&mut self, name: &str) {
        self.servers.retain(|s| s.name != name);
    }
}
