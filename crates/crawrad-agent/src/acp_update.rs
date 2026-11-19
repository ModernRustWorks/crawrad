//! Parser for ACP session update events conforming to Agent Client Protocol schema v1.
#![forbid(unsafe_code)]

use std::collections::HashMap;

use crate::client::AcpStreamEvent;
use crate::markdown::ToolCallStatus;
use crawrad_note::note;

#[note("acp-update-parse-update")]
pub fn parse_update(
    update: &serde_json::Value,
    titles: &mut HashMap<String, String>,
) -> Option<AcpStreamEvent> {
    let update_obj = update.get("update").unwrap_or(update);
    let session_update = update_obj
        .get("sessionUpdate")
        .and_then(|s| s.as_str())
        .unwrap_or("");

    match session_update {
        "agent_message_chunk" => {
            let content = update_obj.get("content")?;
            if content.get("type").and_then(|t| t.as_str()) == Some("text") {
                let text = content.get("text").and_then(|t| t.as_str())?;
                Some(AcpStreamEvent::TextDelta(text.to_string()))
            } else {
                None
            }
        }
        "agent_thought_chunk" => {
            let content = update_obj.get("content")?;
            if content.get("type").and_then(|t| t.as_str()) != Some("text") {
                return None;
            }
            let text = content.get("text").and_then(|t| t.as_str())?;
            Some(AcpStreamEvent::ThoughtDelta(text.to_string()))
        }
        "tool_call" => {
            let id = update_obj
                .get("toolCallId")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let title = update_obj
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            let input = update_obj
                .get("rawInput")
                .cloned()
                .unwrap_or(serde_json::json!({}));
            let status_str = update_obj
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("pending");

            titles.insert(id.clone(), title.clone());
            titles.insert(format!("status:{id}"), status_str.to_string());

            Some(AcpStreamEvent::ToolCall {
                id,
                name: title,
                input,
            })
        }
        "tool_call_update" => {
            let id = update_obj
                .get("toolCallId")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let new_title = update_obj.get("title").and_then(|v| v.as_str());
            let new_status = update_obj.get("status").and_then(|v| v.as_str());

            if new_title.is_none() && new_status.is_none() {
                return None;
            }

            if let Some(t) = new_title {
                titles.insert(id.clone(), t.to_string());
            }
            if let Some(s) = new_status {
                titles.insert(format!("status:{id}"), s.to_string());
            }

            let title = titles
                .get(&id)
                .cloned()
                .unwrap_or_else(|| "tool".to_string());

            let status_key = format!("status:{id}");
            let status_str = titles
                .get(&status_key)
                .map(|s| s.as_str())
                .unwrap_or("pending");

            let status = match status_str {
                "pending" => ToolCallStatus::Pending,
                "in_progress" | "running" => ToolCallStatus::Running,
                "completed" | "success" | "ok" => ToolCallStatus::Success,
                "failed" => ToolCallStatus::Failed,
                _ => ToolCallStatus::Failed,
            };

            Some(AcpStreamEvent::ToolCallUpdate {
                id,
                name: title,
                status,
                summary: String::new(),
            })
        }
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/acp_update/test.rs"]
mod tests;
