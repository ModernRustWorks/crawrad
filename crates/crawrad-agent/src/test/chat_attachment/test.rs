//! Tests for chat attachments and prompt assembly integration.

use super::*;
use crate::prompt_assembler::{
    AssembleParams, EditorSelectionContext, ProjectContext, PromptAssembler, PromptMode,
};

#[test]
fn capture_copies_text_and_lines() {
    let sel = EditorSelectionContext {
        start_line: 10,
        end_line: 12,
        selected_text: "line 10\nline 11\nline 12".to_string(),
    };
    let file_text = "line 1\nline 2\nline 10\nline 11\nline 12\nline 13";
    let att = SelectionAttachment::capture("src/lib.rs", &sel, file_text, 1024);
    assert_eq!(att.file, "src/lib.rs");
    assert_eq!(att.start_line, 10);
    assert_eq!(att.end_line, 12);
    assert_eq!(att.text, "line 10\nline 11\nline 12");
    assert!(!att.truncated);
    assert_eq!(att.source_hash, text_hash(file_text));
}

#[test]
fn capture_truncation_cuts_at_char_boundary_with_multibyte() {
    // 3 ASCII chars followed by a 4-byte UTF-8 emoji (🦀: \u{1F980})
    let sel_text = "abc\u{1F980}def";
    let sel = EditorSelectionContext {
        start_line: 1,
        end_line: 1,
        selected_text: sel_text.to_string(),
    };
    // Budget of 5 bytes: "abc" is 3 bytes, crab is 4 bytes (ends at 7).
    // Budget 5 lands in the middle of crab, so cut must back up to 3 bytes ("abc").
    let att = SelectionAttachment::capture("src/main.rs", &sel, sel_text, 5);
    assert_eq!(att.text, "abc");
    assert!(att.truncated);
}

#[test]
fn chip_text_single_line_range_truncated_and_changed() {
    let sel = EditorSelectionContext {
        start_line: 42,
        end_line: 42,
        selected_text: "let x = 1;".to_string(),
    };
    let orig = "fn f() {\n    let x = 1;\n}";
    let att_single = SelectionAttachment::capture("crates/core/src/model.rs", &sel, orig, 1024);
    assert_eq!(att_single.chip_text(None), "model.rs:42");
    assert_eq!(att_single.chip_text(Some(orig)), "model.rs:42");

    let sel_range = EditorSelectionContext {
        start_line: 10,
        end_line: 25,
        selected_text: "content".to_string(),
    };
    let att_range = SelectionAttachment::capture("src/main.rs", &sel_range, orig, 1024);
    assert_eq!(att_range.chip_text(None), "main.rs:10-25");

    let mut att_trunc = att_range.clone();
    att_trunc.truncated = true;
    assert_eq!(att_trunc.chip_text(None), "main.rs:10-25 · truncated");

    // Changed text
    let modified = "fn f() {\n    let x = 2;\n}";
    assert_eq!(
        att_range.chip_text(Some(modified)),
        "main.rs:10-25 · changed since added"
    );
    assert_eq!(
        att_trunc.chip_text(Some(modified)),
        "main.rs:10-25 · truncated · changed since added"
    );
}

#[test]
fn assemble_prefers_attachment_over_live_selection() {
    // the marker lives only in the selection: the file's text goes into the prompt too
    let live_code = "fn run() {\n    work();\n}";
    let code_ctx = ProjectContext::CodeEditor {
        file_path: "src/main.rs".to_string(),
        language: "rust".to_string(),
        live_content: live_code.to_string(),
        selection: Some(EditorSelectionContext {
            start_line: 2,
            end_line: 2,
            selected_text: "LIVE_SELECTION".to_string(),
        }),
    };

    let attachment = SelectionAttachment {
        file: "src/other.rs".to_string(),
        start_line: 15,
        end_line: 20,
        text: "CAPTURED_SELECTION".to_string(),
        truncated: false,
        source_hash: 12345,
    };

    // With attachment: contains CAPTURED_SELECTION and does NOT contain LIVE_SELECTION
    let prompt_with_attachment = PromptAssembler::assemble(AssembleParams {
        mode: PromptMode::Chat,
        global_memory: None,
        project_context: &code_ctx,
        user_query: "Explain this snippet",
        session_history: &[],
        workspace_root: None,
        live_buffers: None,
        max_bytes_per_file: 4096,
        max_total_prompt_bytes: 16 * 1024,
        attachments: &[attachment],
    });

    assert!(prompt_with_attachment.contains("CAPTURED_SELECTION"));
    assert!(!prompt_with_attachment.contains("LIVE_SELECTION"));

    // With no attachment: still contains LIVE_SELECTION (today's behavior)
    let prompt_without_attachment = PromptAssembler::assemble(AssembleParams {
        mode: PromptMode::Chat,
        global_memory: None,
        project_context: &code_ctx,
        user_query: "Explain this snippet",
        session_history: &[],
        workspace_root: None,
        live_buffers: None,
        max_bytes_per_file: 4096,
        max_total_prompt_bytes: 16 * 1024,
        attachments: &[],
    });

    assert!(prompt_without_attachment.contains("LIVE_SELECTION"));
    assert!(!prompt_without_attachment.contains("CAPTURED_SELECTION"));
}

#[test]
fn queued_message_attachments_serde_round_trip() {
    use crate::session::QueuedMessage;

    let json_no_att = r#"{"id":"q1","text":"test query","timestamp":"now","has_image":false}"#;
    let msg: QueuedMessage =
        serde_json::from_str(json_no_att).expect("deserialize without attachments");
    assert!(msg.attachments.is_empty());

    let att = SelectionAttachment {
        file: "src/main.rs".to_string(),
        start_line: 1,
        end_line: 5,
        text: "fn main() {}".to_string(),
        truncated: false,
        source_hash: 42,
    };
    let original = QueuedMessage::new("q2", "review code").with_attachments(vec![att.clone()]);
    let serialized = serde_json::to_string(&original).expect("serialize queued message");
    let round_trip: QueuedMessage =
        serde_json::from_str(&serialized).expect("deserialize round trip");
    assert_eq!(round_trip, original);
    assert_eq!(round_trip.attachments, vec![att]);
}

fn make_test_png(width: u32, height: u32) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut encoder = png::Encoder::new(&mut buf, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().expect("write header");
    let data = vec![255u8; (width * height * 4) as usize];
    writer.write_image_data(&data).expect("write image data");
    drop(writer);
    buf
}

#[test]
fn image_attachment_save_and_properties() {
    use base64::Engine as _;

    let temp_dir = std::env::temp_dir().join(format!("crawrad-test-att-{}", uuid::Uuid::new_v4()));
    let png = make_test_png(2, 1);

    let att1 = ImageAttachment::save(&temp_dir, &png).expect("save image 1");
    assert_eq!(att1.width, 2);
    assert_eq!(att1.height, 1);
    assert!(att1.path.exists());
    assert_eq!(att1.path.file_name().unwrap(), "image-1.png");

    let att2 = ImageAttachment::save(&temp_dir, &png).expect("save image 2");
    assert_eq!(att2.path.file_name().unwrap(), "image-2.png");

    assert_eq!(att1.chip_text(0), "Image 1 · 2\u{d7}1");
    assert_eq!(att2.chip_text(1), "Image 2 · 2\u{d7}1");

    let note = att1.prompt_note();
    assert!(note.contains(&att1.path.display().to_string()));
    assert!(note.starts_with("[Image attached: "));

    let acp = att1.acp_block().expect("acp block");
    assert_eq!(acp["type"], "image");
    assert_eq!(acp["mimeType"], "image/png");
    let b64 = acp["data"].as_str().expect("base64 string");
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(b64)
        .expect("decode base64");
    assert_eq!(decoded, png);

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn prompt_blocks_handles_accepts_images_flag() {
    let temp_dir = std::env::temp_dir().join(format!("crawrad-test-pb-{}", uuid::Uuid::new_v4()));
    let png = make_test_png(1, 1);
    let att = ImageAttachment::save(&temp_dir, &png).expect("save image");

    // With accepts_images = true
    let blocks_accepted = prompt_blocks("hello agent", std::slice::from_ref(&att), true);
    assert_eq!(blocks_accepted.len(), 2);
    assert_eq!(blocks_accepted[0]["type"], "text");
    assert_eq!(blocks_accepted[0]["text"], "hello agent");
    assert_eq!(blocks_accepted[1]["type"], "image");
    assert_eq!(blocks_accepted[1]["mimeType"], "image/png");

    // With accepts_images = false
    let blocks_declined = prompt_blocks("hello agent", std::slice::from_ref(&att), false);
    assert_eq!(blocks_declined.len(), 2);
    assert_eq!(blocks_declined[0]["type"], "text");
    assert_eq!(blocks_declined[1]["type"], "text");
    assert_eq!(
        blocks_declined[1]["text"].as_str().unwrap(),
        att.prompt_note()
    );

    // With missing file and accepts_images = true -> falls back to prompt_note
    let missing_att = ImageAttachment {
        path: temp_dir.join("nonexistent.png"),
        width: 10,
        height: 10,
    };
    let blocks_missing = prompt_blocks("prompt", std::slice::from_ref(&missing_att), true);
    assert_eq!(blocks_missing.len(), 2);
    assert_eq!(blocks_missing[1]["type"], "text");
    assert_eq!(
        blocks_missing[1]["text"].as_str().unwrap(),
        missing_att.prompt_note()
    );

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn a_chip_index_counts_the_selections_first_then_the_images() {
    let img = |n: &str| ImageAttachment {
        path: n.into(),
        width: 1,
        height: 1,
    };
    let sel = SelectionAttachment {
        file: "a.rs".into(),
        start_line: 1,
        end_line: 1,
        text: "x".into(),
        truncated: false,
        source_hash: 0,
    };
    let (mut sels, mut imgs) = (vec![sel], vec![img("1.png"), img("2.png")]);
    remove_chip(&mut sels, &mut imgs, 2);
    assert_eq!(imgs, vec![img("1.png")], "chip 2 is the second image");
    remove_chip(&mut sels, &mut imgs, 0);
    assert!(
        sels.is_empty() && imgs.len() == 1,
        "chip 0 is the selection"
    );
    remove_chip(&mut sels, &mut imgs, 5);
    assert_eq!(imgs.len(), 1, "past the end removes nothing");
}
