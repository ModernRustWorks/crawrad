//! MCP tool `view.capture` implementation: screen region capture to PNG.

use std::path::PathBuf;

use base64::Engine;
use crawrad_note::note;
use serde_json::Value;

use crate::protocol::ToolCallResult;
use crate::run_tool::Answer;

#[note("capture-tool-captureask")]
#[derive(Clone, Debug, PartialEq)]
pub struct CaptureAsk {
    /// Logical `[x, y, width, height]`; `None` captures the whole window.
    pub region: Option<[f64; 4]>,
    /// Absolute path the PNG is also written to.
    pub path: Option<PathBuf>,
}

#[note("capture-tool-plan")]
pub fn plan(name: &str, args: &Value) -> Option<Result<CaptureAsk, Answer>> {
    (name == "view.capture").then(|| ask_of(args))
}

fn refuse(message: &str) -> Answer {
    (
        ToolCallResult::error_text("invalid-arguments", message),
        "error",
    )
}

fn ask_of(args: &Value) -> Result<CaptureAsk, Answer> {
    let given = |key: &str| args.get(key).filter(|v| !v.is_null());
    let keys = ["x", "y", "width", "height"];
    let present = keys.iter().filter(|k| given(k).is_some()).count();
    let region = match present {
        0 => None,
        4 => {
            let mut values = [0.0; 4];
            for (value, key) in values.iter_mut().zip(keys) {
                *value = given(key)
                    .and_then(Value::as_f64)
                    .ok_or_else(|| refuse(&format!("'{key}' must be a number.")))?;
            }
            if values[2] <= 0.0 || values[3] <= 0.0 {
                return Err(refuse("'width' and 'height' must be greater than zero."));
            }
            Some(values)
        }
        _ => {
            return Err(refuse(
                "Region requires all four coordinates: 'x', 'y', 'width', and 'height'.",
            ));
        }
    };
    let path = match given("path") {
        None => None,
        Some(entry) => {
            let destination = entry
                .as_str()
                .map(PathBuf::from)
                .ok_or_else(|| refuse("'path' must be a string."))?;
            if !destination.is_absolute() {
                return Err(refuse("'path' must be an absolute filesystem path."));
            }
            Some(destination)
        }
    };
    Ok(CaptureAsk { region, path })
}

#[note("capture-tool-answer")]
pub fn answer(args: &Value, dpi: f64) -> Answer {
    ask_of(args).map_or_else(
        |refused| refused,
        |ask| {
            let region = ask.region.map(|r| to_physical(r, dpi));
            done(&ask, crawrad_os::capture_main_window(region))
        },
    )
}

#[note("capture-tool-to-physical")]
pub fn to_physical(region: [f64; 4], dpi: f64) -> [i32; 4] {
    let [x, y, width, height] = region;
    [
        (x * dpi).floor() as i32,
        (y * dpi).floor() as i32,
        (width * dpi).ceil() as i32,
        (height * dpi).ceil() as i32,
    ]
}

#[note("capture-tool-encode-png")]
pub fn encode_png(capture: &crawrad_os::Capture) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut encoder = png::Encoder::new(&mut buffer, capture.width, capture.height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    if let Ok(mut writer) = encoder.write_header() {
        let _ = writer.write_image_data(&capture.rgba);
        let _ = writer.finish();
    }
    buffer
}

#[note("capture-tool-done")]
pub fn done(ask: &CaptureAsk, capture: Option<crawrad_os::Capture>) -> Answer {
    let Some(capture) = capture else {
        let message = "The IDE window could not be captured.";
        return (
            ToolCallResult::error_text("capture-unavailable", message),
            "error",
        );
    };

    let png_bytes = encode_png(&capture);

    if let Some(destination) = &ask.path
        && let Err(error) = std::fs::write(destination, &png_bytes)
    {
        return (
            ToolCallResult::error_text("write-failed", error.to_string()),
            "error",
        );
    }

    let encoded_base64 = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
    let path_value = ask.path.as_ref().map(|p| p.display().to_string());
    let data = serde_json::json!({
        "width": capture.width,
        "height": capture.height,
        "path": path_value,
    });
    let text = format!("Captured {}x{}.", capture.width, capture.height);
    (
        ToolCallResult::success_image(encoded_base64, data, text),
        "applied",
    )
}

#[cfg(test)]
#[path = "test/capture_tool/test.rs"]
mod tests;
