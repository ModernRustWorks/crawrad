//! A simple, robust Markdown parser for the CrawRAD AI Chat.
//!
//! Turns raw LLM agent answers into structured text blocks
//! and code blocks with a language and a copy button.

use crawrad_note::note;
use serde::{Deserialize, Serialize};

#[note("markdown-chatcontentblock")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatContentBlock {
    /// Plain text (prose, lists, paragraphs).
    Text(String),
    /// A code block delimited by ```.
    CodeBlock { language: String, code: String },
}

#[note("markdown-toolcallstatus")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolCallStatus {
    Pending,
    Running,
    Success,
    Failed,
}

impl ToolCallStatus {
    pub fn badge_label(&self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Running => "Running",
            Self::Success => "OK",
            Self::Failed => "Failed",
        }
    }
}

#[note("markdown-toolcallrecord")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolCallRecord {
    pub name: String,
    pub arguments: String,
    pub status: ToolCallStatus,
}

impl ToolCallRecord {
    pub fn new(name: impl Into<String>, arguments: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            arguments: arguments.into(),
            status: ToolCallStatus::Pending,
        }
    }

    pub fn with_status(mut self, status: ToolCallStatus) -> Self {
        self.status = status;
        self
    }
}

#[note("markdown-parse-chat-markdown")]
pub fn parse_chat_markdown(raw: &str) -> Vec<ChatContentBlock> {
    let mut blocks = Vec::new();
    let mut current_text = String::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut current_code = String::new();

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            if !in_code_block {
                // Start of a code block
                in_code_block = true;
                let lang = trimmed.trim_start_matches('`').trim();
                code_lang = if lang.is_empty() {
                    "code".to_string()
                } else {
                    lang.to_string()
                };
                if !current_text.trim().is_empty() {
                    blocks.push(ChatContentBlock::Text(current_text.trim().to_string()));
                    current_text.clear();
                }
            } else {
                // End of a code block
                in_code_block = false;
                blocks.push(ChatContentBlock::CodeBlock {
                    language: code_lang.clone(),
                    code: current_code.trim_end().to_string(),
                });
                code_lang.clear();
                current_code.clear();
            }
        } else if in_code_block {
            if !current_code.is_empty() {
                current_code.push('\n');
            }
            current_code.push_str(line);
        } else {
            if !current_text.is_empty() {
                current_text.push('\n');
            }
            current_text.push_str(&chat_heading(line));
        }
    }

    // If the code block was not closed (e.g. while streaming)
    if in_code_block && !current_code.is_empty() {
        blocks.push(ChatContentBlock::CodeBlock {
            language: if code_lang.is_empty() {
                "code".to_string()
            } else {
                code_lang
            },
            code: current_code.trim_end().to_string(),
        });
    } else if !current_text.trim().is_empty() {
        blocks.push(ChatContentBlock::Text(current_text.trim().to_string()));
    }

    blocks
}

/// A heading in a reply drawn near the text's size: the chat's Markdown widget
/// scales a heading by 4.5 / level, so `##` would be 2.25 times the text and
/// level 4 is 1.125 times. Every level becomes level 4.
fn chat_heading(line: &str) -> std::borrow::Cow<'_, str> {
    let hashes = line.len() - line.trim_start_matches('#').len();
    match (1..=6).contains(&hashes) && line[hashes..].starts_with(' ') {
        true => format!("####{}", &line[hashes..]).into(),
        false => line.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_plain_text() {
        let raw = "Hello, this is a plain assistant message.";
        let blocks = parse_chat_markdown(raw);
        assert_eq!(blocks.len(), 1);
        assert_eq!(
            blocks[0],
            ChatContentBlock::Text("Hello, this is a plain assistant message.".into())
        );
    }

    #[test]
    fn test_parse_code_block_with_language() {
        let raw = "Here is the implementation:\n\n```rust\nfn main() {\n    println!(\"CrawRAD\");\n}\n```\nDone!";
        let blocks = parse_chat_markdown(raw);
        assert_eq!(blocks.len(), 3);
        assert_eq!(
            blocks[0],
            ChatContentBlock::Text("Here is the implementation:".into())
        );
        match &blocks[1] {
            ChatContentBlock::CodeBlock { language, code } => {
                assert_eq!(language, "rust");
                assert_eq!(code, "fn main() {\n    println!(\"CrawRAD\");\n}");
            }
            _ => panic!("Expected code block"),
        }
        assert_eq!(blocks[2], ChatContentBlock::Text("Done!".into()));
    }

    #[test]
    fn a_heading_of_any_level_is_drawn_at_level_four() {
        let blocks = parse_chat_markdown(
            "# One
## Two
#hashtag
####### seven",
        );
        assert_eq!(
            blocks[0],
            ChatContentBlock::Text(
                "#### One
#### Two
#hashtag
####### seven"
                    .into()
            )
        );
    }

    #[test]
    fn test_parse_unclosed_code_block_streaming() {
        let raw = "Drafting:\n```rust\npub fn calculate() -> i32 {";
        let blocks = parse_chat_markdown(raw);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0], ChatContentBlock::Text("Drafting:".into()));
        match &blocks[1] {
            ChatContentBlock::CodeBlock { language, code } => {
                assert_eq!(language, "rust");
                assert_eq!(code, "pub fn calculate() -> i32 {");
            }
            _ => panic!("Expected code block"),
        }
    }
}
