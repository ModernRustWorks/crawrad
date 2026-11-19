//! The system clipboard's text, read — on Windows, macOS and Linux alike.
//!
//! Makepad writes the clipboard (`Cx::copy_to_clipboard`) and hands a paste
//! made by key to the focused widget as an event; nothing READS it on
//! request. A paste from the editor's context menu has to ask, and
//! this is where it asks.

use crawrad_note::note;
#[note("clipboard-read-text")]
pub fn read_text() -> Option<String> {
    let text = arboard::Clipboard::new().ok()?.get_text().ok()?;
    (!text.is_empty()).then_some(text)
}

#[note("clipboard-write-text")]
pub fn write_text(text: &str) -> bool {
    arboard::Clipboard::new()
        .and_then(|mut c| c.set_text(text.to_string()))
        .is_ok()
}

#[note("clipboard-rgba-to-png")]
pub fn rgba_to_png(width: usize, height: usize, rgba: &[u8]) -> Option<Vec<u8>> {
    if width == 0 || height == 0 {
        return None;
    }
    let expected_len = width.checked_mul(height)?.checked_mul(4)?;
    if rgba.len() != expected_len {
        return None;
    }
    let mut buf = Vec::new();
    let mut encoder = png::Encoder::new(&mut buf, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().ok()?;
    writer.write_image_data(rgba).ok()?;
    drop(writer);
    Some(buf)
}

#[note("clipboard-read-image-png")]
pub fn read_image_png() -> Option<Vec<u8>> {
    let image = arboard::Clipboard::new().ok()?.get_image().ok()?;
    rgba_to_png(image.width, image.height, &image.bytes)
}

#[cfg(test)]
#[path = "test/clipboard/test.rs"]
mod tests;
