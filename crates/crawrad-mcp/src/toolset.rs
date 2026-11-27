//! Toolsets: slices of the IDE's MCP surface, so an agent loads only the tools
//! its task needs. One IDE server; the bridge narrows it with `--groups`.
//!
//! A tool belongs to a group by its name's prefix (`designer.add` → `designer`).
//! A prefix may sit in several groups (`view` switches between design and
//! code, so both carry it).

use crawrad_note::note;
use serde_json::Value;

#[note("toolset-groups")]
pub const GROUPS: &[(&str, &[&str])] = &[
    (
        "design",
        &[
            "designer",
            "component",
            "collection",
            "align",
            "order",
            "layout",
            "inspector",
            "form",
            "strings",
            "page",
            "event",
            "binding",
            "action",
            "edit",
            "view",
            "ide",
            "dialog",
        ],
    ),
    (
        "code",
        &[
            "editor", "code", "lsp", "file", "build", "edit", "review", "view", "ide", "dialog",
        ],
    ),
    (
        "debug",
        &["debug", "run", "build", "editor", "view", "ide", "dialog"],
    ),
    (
        "project",
        &[
            "project", "explorer", "file", "build", "tools", "help", "view", "ide", "dialog",
        ],
    ),
    // The AI chat and its agents: one slice an agent loads alone to
    // operate the chat — its actions, the ACP agent catalog, and the reads.
    ("chat", &["chat", "acp", "ai", "view", "ide", "dialog"]),
    // What extensions contribute: its prefix is the extension's choice,
    // not an IDE area, so it is matched by name — see `Toolset::is_extension`.
    (EXTENSIONS, &["view"]),
];

#[note("toolset-extensions")]
pub const EXTENSIONS: &str = "extensions";

#[note("toolset-is-extension-tool")]
pub fn is_extension_tool(name: &str) -> bool {
    name.split_once(".tool.")
        .is_some_and(|(ext, tool)| !ext.is_empty() && !tool.is_empty())
}

#[note("toolset-toolset")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Toolset {
    groups: Vec<&'static str>,
    /// The names the IDE's last `tools/list` said extensions contributed.
    extension_tools: Vec<String>,
}

impl Toolset {
    #[note("toolset-toolset-all")]
    pub fn all() -> Self {
        Self::default()
    }

    #[note("toolset-toolset-parse")]
    pub fn parse(spec: &str) -> Result<Self, String> {
        let mut groups = Vec::new();
        for name in spec.split(',').map(str::trim).filter(|n| !n.is_empty()) {
            if name == "all" {
                return Ok(Self::all());
            }
            let Some((known, _)) = GROUPS.iter().find(|(g, _)| *g == name) else {
                let valid: Vec<&str> = GROUPS.iter().map(|(g, _)| *g).collect();
                return Err(format!(
                    "unknown tool group '{name}' (valid: {}, all)",
                    valid.join(", ")
                ));
            };
            if !groups.contains(known) {
                groups.push(*known);
            }
        }
        Ok(Self {
            groups,
            ..Self::default()
        })
    }

    #[note("toolset-toolset-describe")]
    pub fn describe(&self) -> String {
        if self.groups.is_empty() {
            "all".to_string()
        } else {
            self.groups.join(", ")
        }
    }

    #[note("toolset-toolset-allows")]
    pub fn allows(&self, name: &str) -> bool {
        if self.groups.is_empty() {
            return true;
        }
        if self.is_extension(name) {
            return self.groups.contains(&EXTENSIONS);
        }
        let prefix = name.split('.').next().unwrap_or(name);
        GROUPS
            .iter()
            .filter(|(g, _)| self.groups.contains(g))
            .any(|(_, prefixes)| prefixes.contains(&prefix))
    }

    #[note("toolset-toolset-is-extension")]
    fn is_extension(&self, name: &str) -> bool {
        let prefix = name.split('.').next().unwrap_or(name);
        let ide_prefix = GROUPS
            .iter()
            .filter(|(g, _)| *g != EXTENSIONS)
            .any(|(_, prefixes)| prefixes.contains(&prefix));
        self.extension_tools.iter().any(|t| t == name) || is_extension_tool(name) || !ide_prefix
    }

    #[note("toolset-toolset-filter-list-response")]
    pub fn filter_list_response(&mut self, response: &mut Value) {
        if let Some(names) = response
            .pointer("/result/_meta/extensionTools")
            .and_then(Value::as_array)
        {
            self.extension_tools = names
                .iter()
                .filter_map(|n| n.as_str().map(str::to_string))
                .collect();
        }
        if let Some(tools) = response
            .pointer_mut("/result/tools")
            .and_then(Value::as_array_mut)
        {
            tools.retain(|t| {
                t.get("name")
                    .and_then(Value::as_str)
                    .is_some_and(|n| self.allows(n))
            });
        }
    }

    #[note("toolset-toolset-refuse-call")]
    pub fn refuse_call(&self, request: &Value) -> Option<Value> {
        if request.get("method").and_then(Value::as_str) != Some("tools/call") {
            return None;
        }
        let name = request.pointer("/params/name").and_then(Value::as_str)?;
        if self.allows(name) {
            return None;
        }
        Some(serde_json::json!({
            "jsonrpc": "2.0",
            "id": request.get("id").cloned().unwrap_or(Value::Null),
            "error": {
                "code": -32602,
                "message": format!(
                    "tool '{name}' is not in this connection's groups ({}); use the server entry that carries it",
                    self.describe()
                )
            }
        }))
    }
}

#[cfg(test)]
#[path = "test/toolset/test.rs"]
mod tests;
