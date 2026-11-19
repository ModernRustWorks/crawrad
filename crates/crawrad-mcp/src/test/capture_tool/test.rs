//! Unit tests for capture_tool.

use std::path::PathBuf;

use base64::Engine;
use serde_json::json;

use crate::capture_tool::{CaptureAsk, done, plan, to_physical};
use crate::protocol::ToolContent;

#[test]
fn plan_returns_none_for_unrelated_names() {
    assert!(plan("unrelated.tool", &json!({})).is_none());
    assert!(plan("view.show_panel", &json!({})).is_none());
    assert!(plan("dialog.press", &json!({})).is_none());
}

#[test]
fn plan_accepts_empty_arguments_for_full_capture() {
    let planned = plan("view.capture", &json!({})).expect("view.capture should be handled");
    let ask = planned.expect("empty arguments should be valid");
    assert_eq!(
        ask,
        CaptureAsk {
            region: None,
            path: None
        }
    );
}

#[test]
fn plan_accepts_full_region_and_absolute_path() {
    let abs_path = if cfg!(windows) {
        "C:\\captures\\window.png"
    } else {
        "/tmp/window.png"
    };
    let args = json!({
        "x": 10.0,
        "y": 20.0,
        "width": 100.0,
        "height": 50.0,
        "path": abs_path,
    });
    let planned = plan("view.capture", &args).expect("view.capture should be handled");
    let ask = planned.expect("valid region and absolute path should be accepted");
    assert_eq!(
        ask,
        CaptureAsk {
            region: Some([10.0, 20.0, 100.0, 50.0]),
            path: Some(PathBuf::from(abs_path)),
        }
    );
}

#[test]
fn plan_refuses_partial_region() {
    let partial_cases = [
        json!({ "x": 10.0, "y": 20.0, "width": 100.0 }),
        json!({ "x": 10.0, "width": 100.0, "height": 50.0 }),
        json!({ "y": 20.0, "height": 50.0 }),
        json!({ "width": 100.0 }),
    ];
    for args in &partial_cases {
        let planned = plan("view.capture", args).expect("view.capture should be handled");
        let (refusal, outcome) = planned.expect_err("partial region should be refused");
        assert_eq!(outcome, "error");
        assert_eq!(refusal.data.unwrap()["code"], json!("invalid-arguments"));
    }
}

#[test]
fn plan_refuses_zero_or_negative_dimensions() {
    let bad_dimensions = [
        json!({ "x": 0.0, "y": 0.0, "width": 0.0, "height": 100.0 }),
        json!({ "x": 0.0, "y": 0.0, "width": 100.0, "height": 0.0 }),
        json!({ "x": 0.0, "y": 0.0, "width": -5.0, "height": 100.0 }),
        json!({ "x": 0.0, "y": 0.0, "width": 100.0, "height": -10.0 }),
    ];
    for args in &bad_dimensions {
        let planned = plan("view.capture", args).expect("view.capture should be handled");
        let (refusal, outcome) = planned.expect_err("zero or negative dimension should be refused");
        assert_eq!(outcome, "error");
        assert_eq!(refusal.data.unwrap()["code"], json!("invalid-arguments"));
    }
}

#[test]
fn plan_refuses_relative_path() {
    let args = json!({
        "path": "relative/file.png",
    });
    let planned = plan("view.capture", &args).expect("view.capture should be handled");
    let (refusal, outcome) = planned.expect_err("relative path should be refused");
    assert_eq!(outcome, "error");
    assert_eq!(refusal.data.unwrap()["code"], json!("invalid-arguments"));
}

#[test]
fn to_physical_at_fractional_dpi() {
    // x, y floor(v * dpi); width, height ceil(v * dpi)
    let logical = [10.2, 20.8, 100.1, 50.4];
    let physical = to_physical(logical, 1.5);
    // 10.2 * 1.5 = 15.3 -> floor = 15
    // 20.8 * 1.5 = 31.2 -> floor = 31
    // 100.1 * 1.5 = 150.15 -> ceil = 151
    // 50.4 * 1.5 = 75.6 -> ceil = 76
    assert_eq!(physical, [15, 31, 151, 76]);
}

#[test]
fn done_none_refuses_with_capture_unavailable() {
    let ask = CaptureAsk {
        region: None,
        path: None,
    };
    let (result, outcome) = done(&ask, None);
    assert_eq!(outcome, "error");
    assert!(result.is_error);
    assert_eq!(result.data.unwrap()["code"], json!("capture-unavailable"));
}

#[test]
fn done_some_returns_image_item_with_matching_data_and_png_header() {
    let ask = CaptureAsk {
        region: None,
        path: None,
    };
    let capture = crawrad_os::Capture {
        width: 2,
        height: 2,
        rgba: vec![
            255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 0, 255,
        ],
    };
    let (result, outcome) = done(&ask, Some(capture));
    assert_eq!(outcome, "applied");
    assert!(!result.is_error);

    let data = result.data.unwrap();
    assert_eq!(data["width"], json!(2));
    assert_eq!(data["height"], json!(2));
    assert!(data["path"].is_null());

    assert_eq!(result.content.len(), 2);
    let (base64_data, mime_type) = match &result.content[0] {
        ToolContent::Image { data, mime_type } => (data, mime_type),
        _ => panic!("expected Image content at index 0"),
    };
    assert_eq!(mime_type, "image/png");

    let decoded_bytes = base64::engine::general_purpose::STANDARD
        .decode(base64_data)
        .expect("base64 payload must decode successfully");
    assert!(decoded_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));

    match &result.content[1] {
        ToolContent::Text { text } => assert_eq!(text, "Captured 2x2."),
        _ => panic!("expected Text content at index 1"),
    }
}

#[test]
fn tool_content_image_serialization_matches_mcp_spec() {
    let item = ToolContent::Image {
        data: "aGVsbG8=".to_string(),
        mime_type: "image/png".to_string(),
    };
    let serialized = serde_json::to_string(&item).expect("serialization should succeed");
    assert_eq!(
        serialized,
        r#"{"type":"image","data":"aGVsbG8=","mimeType":"image/png"}"#
    );
}

#[test]
fn done_writes_png_to_specified_path_and_cleans_up() {
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("test_capture_tool_{}.png", std::process::id()));
    if temp_file.exists() {
        let _ = std::fs::remove_file(&temp_file);
    }

    let ask = CaptureAsk {
        region: None,
        path: Some(temp_file.clone()),
    };
    let capture = crawrad_os::Capture {
        width: 1,
        height: 1,
        rgba: vec![120, 130, 140, 255],
    };

    let (result, outcome) = done(&ask, Some(capture));
    assert_eq!(outcome, "applied");
    assert!(!result.is_error);
    assert!(
        temp_file.exists(),
        "PNG file must be written to destination path"
    );

    let file_bytes = std::fs::read(&temp_file).expect("file must be readable");
    assert!(file_bytes.starts_with(b"\x89PNG\r\n\x1a\n"));

    let _ = std::fs::remove_file(&temp_file);
}
