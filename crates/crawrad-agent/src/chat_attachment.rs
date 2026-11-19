//! Chat attachment model and prompt rendering.

use crate::prompt_assembler::EditorSelectionContext;
use base64::Engine as _;
use crawrad_note::note;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};

#[note("chat-attachment-max-attachments")]
pub const MAX_ATTACHMENTS: usize = 4;

#[note("chat-attachment-selectionattachment")]
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SelectionAttachment {
    /// The path as shown to the user (relative to the workspace when possible).
    pub file: String,
    /// 1-based, as in [`EditorSelectionContext`].
    pub start_line: usize,
    pub end_line: usize,
    pub text: String,
    /// The text was cut to the byte budget: the chip and the prompt say so.
    pub truncated: bool,
    /// [`text_hash`] of the whole file when it was captured.
    pub source_hash: u64,
}

#[note("chat-attachment-text-hash")]
pub fn text_hash(text: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

impl SelectionAttachment {
    #[note("chat-attachment-selectionattachment-capture")]
    pub fn capture(
        file: &str,
        selection: &EditorSelectionContext,
        file_text: &str,
        max_bytes: usize,
    ) -> Self {
        let source_hash = text_hash(file_text);
        let (text, truncated) = if selection.selected_text.len() > max_bytes {
            let mut cut = max_bytes;
            while cut > 0 && !selection.selected_text.is_char_boundary(cut) {
                cut -= 1;
            }
            (selection.selected_text[..cut].to_string(), true)
        } else {
            (selection.selected_text.clone(), false)
        };
        Self {
            file: file.to_string(),
            start_line: selection.start_line,
            end_line: selection.end_line,
            text,
            truncated,
            source_hash,
        }
    }

    #[note("chat-attachment-selectionattachment-chip-text")]
    pub fn chip_text(&self, current_file_text: Option<&str>) -> String {
        let file_name = std::path::Path::new(&self.file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&self.file);
        let mut s = if self.start_line == self.end_line {
            format!("{file_name}:{}", self.start_line)
        } else {
            format!("{file_name}:{}-{}", self.start_line, self.end_line)
        };
        if self.truncated {
            s.push_str(" · truncated");
        }
        if current_file_text.is_some_and(|t| text_hash(t) != self.source_hash) {
            s.push_str(" · changed since added");
        }
        s
    }
}

#[note("chat-attachment-render")]
pub fn render(attachments: &[SelectionAttachment]) -> String {
    if attachments.is_empty() {
        return String::new();
    }
    let mut blocks = Vec::with_capacity(attachments.len());
    for att in attachments {
        let lang = match std::path::Path::new(&att.file)
            .extension()
            .and_then(|s| s.to_str())
        {
            Some("rs") => "rust",
            Some(ext) => ext,
            None => "",
        };
        let mut block = format!(
            "Attached selection from {}, lines {}-{}:\n```{}\n{}\n```",
            att.file, att.start_line, att.end_line, lang, att.text
        );
        if att.truncated {
            block.push_str(&format!("\n(truncated to {} bytes)", att.text.len()));
        }
        blocks.push(block);
    }
    blocks.join("\n\n")
}

#[note("chat-attachment-imageattachment")]
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ImageAttachment {
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
}

impl ImageAttachment {
    #[note("chat-attachment-imageattachment-save")]
    pub fn save(dir: &Path, png: &[u8]) -> Result<Self, String> {
        let decoder = png::Decoder::new(png);
        let reader = decoder
            .read_info()
            .map_err(|e| format!("Invalid PNG data: {e}"))?;
        let info = reader.info();
        let (width, height) = (info.width, info.height);

        std::fs::create_dir_all(dir)
            .map_err(|e| format!("Failed to create directory {}: {e}", dir.display()))?;

        let mut n = 1;
        let path = loop {
            let candidate = dir.join(format!("image-{n}.png"));
            if !candidate.exists() {
                break candidate;
            }
            n += 1;
        };

        std::fs::write(&path, png)
            .map_err(|e| format!("Failed to write {}: {e}", path.display()))?;

        Ok(Self {
            path,
            width,
            height,
        })
    }

    #[note("chat-attachment-imageattachment-chip-text")]
    pub fn chip_text(&self, index: usize) -> String {
        format!("Image {} · {}\u{d7}{}", index + 1, self.width, self.height)
    }

    #[note("chat-attachment-imageattachment-acp-block")]
    pub fn acp_block(&self) -> Result<serde_json::Value, String> {
        let bytes = std::fs::read(&self.path)
            .map_err(|e| format!("Failed to read {}: {e}", self.path.display()))?;
        let data = base64::engine::general_purpose::STANDARD.encode(&bytes);
        Ok(serde_json::json!({
            "type": "image",
            "mimeType": "image/png",
            "data": data,
        }))
    }

    #[note("chat-attachment-imageattachment-prompt-note")]
    pub fn prompt_note(&self) -> String {
        format!(
            "[Image attached: {}. Read this file to see it.]",
            self.path.display()
        )
    }
}

#[note("chat-attachment-images-dir")]
pub fn images_dir() -> PathBuf {
    std::env::temp_dir().join("crawrad-chat-images")
}

#[note("chat-attachment-paste-image")]
pub fn paste_image(taken: usize) -> Option<Result<ImageAttachment, String>> {
    if crawrad_os::clipboard::read_text().is_some() {
        return None;
    }
    let png = crawrad_os::clipboard::read_image_png()?;
    if taken >= MAX_ATTACHMENTS {
        return Some(Err("up to 4 attachments: remove one first".into()));
    }
    Some(ImageAttachment::save(&images_dir(), &png))
}

#[note("chat-attachment-remove-chip")]
pub fn remove_chip(
    selections: &mut Vec<SelectionAttachment>,
    images: &mut Vec<ImageAttachment>,
    i: usize,
) {
    if i < selections.len() {
        selections.remove(i);
    } else if i - selections.len() < images.len() {
        images.remove(i - selections.len());
    }
}

#[note("chat-attachment-prompt-blocks")]
pub fn prompt_blocks(
    prompt_text: &str,
    images: &[ImageAttachment],
    accepts_images: bool,
) -> Vec<serde_json::Value> {
    let text = |t: String| serde_json::json!({"type": "text", "text": t});
    let image = |img: &ImageAttachment| {
        let block = accepts_images.then(|| img.acp_block().ok()).flatten();
        block.unwrap_or_else(|| text(img.prompt_note()))
    };
    std::iter::once(text(prompt_text.to_string()))
        .chain(images.iter().map(image))
        .collect()
}

#[cfg(test)]
#[path = "test/chat_attachment/test.rs"]
mod tests;
