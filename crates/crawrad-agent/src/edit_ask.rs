//! An agent's edit permission request shown as a diff: the proposed file, and the answer once decided.
#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crawrad_note::note;

use crate::permission::{PermissionDecision, PermissionOptionKind, PermissionRequest};

/// One text replacement an edit tool asks for.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Replace {
    old: String,
    new: String,
    all: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Change {
    Write(String),
    Replaces(Vec<Replace>),
}

#[note("edit-ask-proposededit")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProposedEdit {
    pub path: PathBuf,
    change: Change,
}

impl ProposedEdit {
    #[note("edit-ask-proposededit-from-tool-call")]
    pub fn from_tool_call(tool_call: &serde_json::Value) -> Option<Self> {
        let input = tool_call.get("rawInput")?;
        let path = PathBuf::from(input.get("file_path")?.as_str()?);
        if let Some(content) = input.get("content").and_then(|c| c.as_str()) {
            return Some(Self {
                path,
                change: Change::Write(content.to_string()),
            });
        }
        let replaces = match input.get("edits").and_then(|e| e.as_array()) {
            Some(edits) => edits.iter().map(replace_of).collect::<Option<Vec<_>>>()?,
            None => vec![replace_of(input)?],
        };
        Some(Self {
            path,
            change: Change::Replaces(replaces),
        })
    }

    #[note("edit-ask-proposededit-apply")]
    pub fn apply(&self, current: &str) -> Result<String, String> {
        let replaces = match &self.change {
            Change::Write(content) => return Ok(content.clone()),
            Change::Replaces(replaces) => replaces,
        };
        let mut text = current.to_string();
        for r in replaces {
            if r.old.is_empty() || !text.contains(&r.old) {
                return Err(format!(
                    "The text to replace is not in {}.",
                    self.path.display()
                ));
            }
            text = if r.all {
                text.replace(&r.old, &r.new)
            } else {
                text.replacen(&r.old, &r.new, 1)
            };
        }
        Ok(text)
    }
}

fn replace_of(v: &serde_json::Value) -> Option<Replace> {
    Some(Replace {
        old: v.get("old_string")?.as_str()?.to_string(),
        new: v.get("new_string")?.as_str()?.to_string(),
        all: v
            .get("replace_all")
            .and_then(|a| a.as_bool())
            .unwrap_or(false),
    })
}

#[note("edit-ask-option-of")]
pub fn option_of(req: &PermissionRequest, accepted: bool) -> String {
    let (kind, fallback) = if accepted {
        (PermissionOptionKind::AllowOnce, "allow_once")
    } else {
        (PermissionOptionKind::RejectOnce, "reject_once")
    };
    req.options
        .iter()
        .find(|o| o.kind == kind)
        .map(|o| o.option_id.clone())
        .unwrap_or_else(|| fallback.to_string())
}

#[note("edit-ask-agenteditasks")]
#[derive(Default)]
pub struct AgentEditAsks {
    held: HashMap<PathBuf, Arc<PermissionRequest>>,
}

impl AgentEditAsks {
    #[note("edit-ask-agenteditasks-hold")]
    pub fn hold(&mut self, path: PathBuf, req: Arc<PermissionRequest>) {
        if let Some(old) = self.held.insert(path, req) {
            old.respond(PermissionDecision::Selected(option_of(&old, false)));
        }
    }

    #[note("edit-ask-agenteditasks-allow")]
    pub fn allow(&self, req: &PermissionRequest) {
        req.respond(PermissionDecision::Selected(option_of(req, true)));
    }

    #[note("edit-ask-agenteditasks-decide")]
    pub fn decide(&mut self, path: &Path, accepted: bool) {
        if let Some(req) = self.held.remove(path) {
            req.respond(PermissionDecision::Selected(option_of(&req, accepted)));
        }
    }
}

#[cfg(test)]
#[path = "test/edit_ask/test.rs"]
mod tests;
