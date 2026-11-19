//! What `designer.set_component_properties` answers, key by key.
//!
//! Each key answers for itself: applied, or refused with the reason. Before,
//! the tool counted the keys it was SENT and said `ok`, so a key the class does
//! not have (`caption` on a `ListColumn`, whose key is `text`) passed silently.
//! Pure of the window, so it lives here; the shell only runs the loop.

use crawrad_note::note;
use serde_json::{Value, json};

#[note("properties-unknown-key")]
pub fn unknown_key(class: &str, keys: &[&str], key: &str) -> Option<String> {
    if keys.is_empty() || key.contains('.') || keys.contains(&key) {
        return None;
    }
    Some(format!(
        "{class} has no property `{key}`; its keys: {}",
        keys.join(", ")
    ))
}

#[note("properties-outcome")]
#[derive(Debug, Default)]
pub struct Outcome {
    applied: Vec<String>,
    refused: Vec<(String, String)>,
}

impl Outcome {
    pub fn apply(&mut self, key: &str) {
        self.applied.push(key.to_string());
    }

    pub fn refuse(&mut self, key: &str, reason: impl Into<String>) {
        self.refused.push((key.to_string(), reason.into()));
    }

    #[note("properties-outcome-failed")]
    pub fn failed(&self) -> bool {
        self.applied.is_empty() && !self.refused.is_empty()
    }

    #[note("properties-outcome-answer")]
    pub fn answer(&self, node_id: &str, sent: usize, revision: u64) -> (Value, String) {
        let refused: Vec<Value> = self
            .refused
            .iter()
            .map(|(k, r)| json!({ "key": k, "reason": r }))
            .collect();
        let data = json!({
            "id": node_id,
            "properties_updated": self.applied.len(),
            "applied": self.applied,
            "refused": refused,
            "revision": revision,
            "status": if self.refused.is_empty() { "ok" } else { "partial" }
        });
        let why: String = self
            .refused
            .iter()
            .map(|(k, r)| format!(" Refused `{k}`: {r}"))
            .collect();
        let said = format!(
            "Updated {} of {sent} properties on component '{node_id}' (rev {revision}).{why}",
            self.applied.len()
        );
        (data, said)
    }
}

#[cfg(test)]
#[path = "test/properties/test.rs"]
mod tests;
