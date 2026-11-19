//! Pure model for project options (title, icon) and ICO/PNG processing.

use std::path::{Path, PathBuf};

pub use crawrad_actions::project_log::{LogFolder, LogLevel, LogOptions, NewFile};
pub use crawrad_actions::project_options_dialog::IconSource;

/// The built-in CrawRAD crab icon.
pub const DEFAULT_ICON: &[u8] = include_bytes!("../assets/crawrad.ico");

/// Choice of application icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconChoice {
    /// Built-in CrawRAD crab icon.
    #[default]
    Default,
    /// Custom icon stored in the project directory as `icon.ico`.
    Custom,
}

/// Project application options stored in `Cargo.toml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationOptions {
    /// Window and application title.
    pub title: String,
    /// Icon choice (default crab or custom project icon).
    pub icon: IconChoice,
}

/// Reads the application options from the project's `Cargo.toml`.
///
/// Looks under `[package.metadata.crawrad]` for `title` and `icon`.
/// If absent, `title` defaults to the package name and `icon` to `Default`.
pub fn read(project_dir: &Path) -> ApplicationOptions {
    let manifest = std::fs::read_to_string(project_dir.join(crate::workspace::MANIFEST));
    let parsed = manifest
        .ok()
        .and_then(|c| c.parse::<toml::Table>().ok())
        .unwrap_or_default();
    let package = parsed.get("package");
    let crawrad = package
        .and_then(|p| p.get("metadata"))
        .and_then(|m| m.get("crawrad"));
    let text = |v: Option<&toml::Value>, key: &str| {
        v.and_then(|t| t.get(key))
            .and_then(|s| s.as_str())
            .map(str::to_string)
    };
    let title = text(crawrad, "title")
        .or_else(|| text(package, "name"))
        .or_else(|| {
            project_dir
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| "app".to_string());
    let icon = match text(crawrad, "icon").as_deref() {
        Some("custom") => IconChoice::Custom,
        _ => IconChoice::Default,
    };
    ApplicationOptions { title, icon }
}

/// Rewrites only the `[package.metadata.crawrad]` table's `title` and `icon` keys
/// in the project's `Cargo.toml`, keeping every other line byte for byte.
///
/// # Errors
/// Returns an error if reading or writing `Cargo.toml` fails.
pub fn write(project_dir: &Path, opts: &ApplicationOptions) -> Result<(), String> {
    let icon = match opts.icon {
        IconChoice::Default => "default",
        IconChoice::Custom => "custom",
    };
    edit_manifest(
        project_dir,
        "[package.metadata.crawrad]",
        &[("title", quoted(&opts.title)), ("icon", quoted(icon))],
    )
}

/// A TOML basic string.
fn quoted(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Sets `key = value` lines in one section of the project's `Cargo.toml`, by
/// line, so the user's comments, order and line endings stay as they were. A
/// missing key goes at the section's end; a missing section at the file's end.
fn edit_manifest(project_dir: &Path, header: &str, keys: &[(&str, String)]) -> Result<(), String> {
    let path = project_dir.join(crate::workspace::MANIFEST);
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    std::fs::write(&path, set_section_keys(&content, header, keys))
        .map_err(|e| format!("cannot write {}: {e}", path.display()))
}

/// [`edit_manifest`] on text.
fn set_section_keys(content: &str, header: &str, keys: &[(&str, String)]) -> String {
    let mut lines: Vec<String> = content.lines().map(str::to_string).collect();
    let start = lines.iter().position(|l| l.trim() == header);
    let start = start.unwrap_or_else(|| {
        if lines.last().is_some_and(|l| !l.trim().is_empty()) {
            lines.push(String::new());
        }
        lines.push(header.to_string());
        lines.len() - 1
    });
    let mut end = (start + 1..lines.len())
        .find(|&i| lines[i].trim_start().starts_with('['))
        .unwrap_or(lines.len());
    while end > start + 1 && lines[end - 1].trim().is_empty() {
        end -= 1;
    }
    for (key, value) in keys {
        let line = format!("{key} = {value}");
        let found = (start + 1..end).find(|&i| {
            lines[i]
                .split_once('=')
                .is_some_and(|(k, _)| k.trim() == *key)
        });
        match found {
            Some(i) => lines[i] = line,
            None => {
                lines.insert(end, line);
                end += 1;
            }
        }
    }
    let eol = if content.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let mut out = lines.join(eol);
    if content.ends_with('\n') || content.is_empty() {
        out.push_str(eol);
    }
    out
}

/// The section the Log page keeps.
pub const LOG_SECTION: &str = "[package.metadata.crawrad.log]";

/// The project's log settings; the defaults for whatever the manifest lacks.
pub fn read_log(project_dir: &Path) -> LogOptions {
    let parsed = std::fs::read_to_string(project_dir.join(crate::workspace::MANIFEST))
        .ok()
        .and_then(|c| c.parse::<toml::Table>().ok())
        .unwrap_or_default();
    let log = ["package", "metadata", "crawrad", "log"]
        .iter()
        .try_fold(&parsed, |t, k| t.get(*k).and_then(toml::Value::as_table));
    let mut o = LogOptions::default();
    let Some(log) = log else {
        return o;
    };
    let text = |k: &str| log.get(k).and_then(toml::Value::as_str);
    if let Some(b) = log.get("enabled").and_then(toml::Value::as_bool) {
        o.enabled = b;
    }
    if let Some(n) = text("file_name") {
        o.file_name = n.to_string();
    }
    o.folder = match text("folder") {
        Some("exe") => LogFolder::Executable,
        Some("user") | None => LogFolder::UserData,
        Some(custom) => LogFolder::Custom(custom.to_string()),
    };
    if let Some(l) = text("debug_level").and_then(LogLevel::from_key) {
        o.debug_level = l;
    }
    if let Some(l) = text("release_level").and_then(LogLevel::from_key) {
        o.release_level = l;
    }
    if text("new_file") == Some(NewFile::EveryRun.key()) {
        o.new_file = NewFile::EveryRun;
    }
    if let Some(k) = log.get("keep").and_then(toml::Value::as_integer) {
        o.keep = u32::try_from(k).unwrap_or(o.keep);
    }
    o
}

/// Writes the log settings after [`crawrad_actions::project_log::check`] passes them.
pub fn write_log(project_dir: &Path, o: &LogOptions) -> Result<(), String> {
    crawrad_actions::project_log::check(o)?;
    let folder = match &o.folder {
        LogFolder::Executable => "exe",
        LogFolder::UserData => "user",
        LogFolder::Custom(f) => f.as_str(),
    };
    edit_manifest(
        project_dir,
        LOG_SECTION,
        &[
            ("enabled", o.enabled.to_string()),
            ("file_name", quoted(&o.file_name)),
            ("folder", quoted(folder)),
            ("debug_level", quoted(o.debug_level.key())),
            ("release_level", quoted(o.release_level.key())),
            ("new_file", quoted(o.new_file.key())),
            ("keep", o.keep.to_string()),
        ],
    )
}

/// Writes `<app>.log.toml` into the project folder, for the developer to ship
/// beside the executable; returns where it went.
pub fn write_sample_override(
    project_dir: &Path,
    app: &str,
    o: &LogOptions,
) -> Result<PathBuf, String> {
    let path = project_dir.join(crawrad_actions::project_log::override_file(app));
    std::fs::write(&path, crawrad_actions::project_log::sample_override(app, o))
        .map_err(|e| format!("cannot write {}: {e}", path.display()))?;
    Ok(path)
}

/// The section that stores custom size preview presets.
pub const SIZES_SECTION: &str = "[package.metadata.crawrad.sizes]";

fn parse_size_entry(s: &str) -> Option<(String, u32, u32)> {
    let (name, dims) = s.rsplit_once(':')?;
    let name = name.trim();
    if name.is_empty() {
        return None;
    }
    let (w_str, h_str) = dims.split_once(['x', 'X'])?;
    let w: u32 = w_str.parse().ok()?;
    let h: u32 = h_str.parse().ok()?;
    if w == 0 || h == 0 {
        return None;
    }
    Some((name.to_string(), w, h))
}

/// Reads custom sizes from the project's `Cargo.toml`.
pub fn read_sizes(project_dir: &Path) -> Vec<(String, u32, u32)> {
    let parsed = std::fs::read_to_string(project_dir.join(crate::workspace::MANIFEST))
        .ok()
        .and_then(|c| c.parse::<toml::Table>().ok())
        .unwrap_or_default();
    let sizes = ["package", "metadata", "crawrad", "sizes"]
        .iter()
        .try_fold(&parsed, |t, k| t.get(*k).and_then(toml::Value::as_table));
    let Some(sizes) = sizes else {
        return Vec::new();
    };
    let Some(arr) = sizes.get("custom").and_then(toml::Value::as_array) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for item in arr {
        let Some(s) = item.as_str() else { continue };
        if let Some(entry) = parse_size_entry(s) {
            out.push(entry);
        }
    }
    out
}

/// Writes custom sizes to the project's `Cargo.toml`.
pub fn write_sizes(project_dir: &Path, sizes: &[(String, u32, u32)]) -> Result<(), String> {
    let items: Vec<String> = sizes
        .iter()
        .map(|(name, w, h)| quoted(&format!("{name}:{w}x{h}")))
        .collect();
    let val = format!("[{}]", items.join(", "));
    edit_manifest(project_dir, SIZES_SECTION, &[("custom", val)])
}

/// Helper function to encode raw RGBA8 pixels into PNG bytes.
pub fn encode_png(width: u32, height: u32, rgba: &[u8]) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut out, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder
            .write_header()
            .map_err(|e| format!("PNG write header error: {e}"))?;
        writer
            .write_image_data(rgba)
            .map_err(|e| format!("PNG write image data error: {e}"))?;
    }
    Ok(out)
}

/// Decodes PNG bytes to RGBA8 pixels.
pub fn decode_png_to_rgba(png_bytes: &[u8]) -> Result<(u32, u32, Vec<u8>), String> {
    let decoder = png::Decoder::new(std::io::Cursor::new(png_bytes));
    let mut reader = decoder
        .read_info()
        .map_err(|e| format!("PNG decode info error: {e}"))?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader
        .next_frame(&mut buf)
        .map_err(|e| format!("PNG decode frame error: {e}"))?;
    let (w, h) = (info.width, info.height);
    buf.truncate(info.buffer_size());

    let rgba = match info.color_type {
        png::ColorType::Rgba => buf,
        png::ColorType::Rgb => {
            let mut out = Vec::with_capacity((w * h * 4) as usize);
            for chunk in buf.chunks_exact(3) {
                out.extend_from_slice(&[chunk[0], chunk[1], chunk[2], 255]);
            }
            out
        }
        png::ColorType::Grayscale => {
            let mut out = Vec::with_capacity((w * h * 4) as usize);
            for &g in &buf {
                out.extend_from_slice(&[g, g, g, 255]);
            }
            out
        }
        png::ColorType::GrayscaleAlpha => {
            let mut out = Vec::with_capacity((w * h * 4) as usize);
            for chunk in buf.chunks_exact(2) {
                out.extend_from_slice(&[chunk[0], chunk[0], chunk[0], chunk[1]]);
            }
            out
        }
        png::ColorType::Indexed => {
            let palette = reader
                .info()
                .palette
                .as_deref()
                .ok_or("Indexed PNG without palette")?;
            let trns = reader.info().trns.as_deref();
            let mut out = Vec::with_capacity((w * h * 4) as usize);
            for &idx in &buf {
                let i = idx as usize;
                let r = palette.get(i * 3).copied().unwrap_or(0);
                let g = palette.get(i * 3 + 1).copied().unwrap_or(0);
                let b = palette.get(i * 3 + 2).copied().unwrap_or(0);
                let a = trns.and_then(|t| t.get(i).copied()).unwrap_or(255);
                out.extend_from_slice(&[r, g, b, a]);
            }
            out
        }
    };

    Ok((w, h, rgba))
}

/// Pure Rust box filter image resizing for RGBA8 buffers.
pub fn resize_rgba(src_w: u32, src_h: u32, src: &[u8], dst_w: u32, dst_h: u32) -> Vec<u8> {
    if src_w == dst_w && src_h == dst_h {
        return src.to_vec();
    }
    let mut dst = vec![0u8; (dst_w * dst_h * 4) as usize];
    let x_scale = src_w as f64 / dst_w as f64;
    let y_scale = src_h as f64 / dst_h as f64;

    for dy in 0..dst_h {
        let y0 = dy as f64 * y_scale;
        let y1 = (dy + 1) as f64 * y_scale;
        let sy_start = y0.floor() as usize;
        let sy_end = (y1.ceil() as usize).min(src_h as usize);

        for dx in 0..dst_w {
            let x0 = dx as f64 * x_scale;
            let x1 = (dx + 1) as f64 * x_scale;
            let sx_start = x0.floor() as usize;
            let sx_end = (x1.ceil() as usize).min(src_w as usize);

            let mut r_sum = 0.0;
            let mut g_sum = 0.0;
            let mut b_sum = 0.0;
            let mut a_sum = 0.0;
            let mut w_sum = 0.0;

            for sy in sy_start..sy_end {
                let wy = ((sy + 1) as f64).min(y1) - (sy as f64).max(y0);
                if wy <= 0.0 {
                    continue;
                }
                let row_offset = sy * (src_w as usize) * 4;

                for sx in sx_start..sx_end {
                    let wx = ((sx + 1) as f64).min(x1) - (sx as f64).max(x0);
                    if wx <= 0.0 {
                        continue;
                    }
                    let weight = wx * wy;
                    let idx = row_offset + sx * 4;
                    r_sum += f64::from(src[idx]) * weight;
                    g_sum += f64::from(src[idx + 1]) * weight;
                    b_sum += f64::from(src[idx + 2]) * weight;
                    a_sum += f64::from(src[idx + 3]) * weight;
                    w_sum += weight;
                }
            }

            let dst_idx = (dy * dst_w + dx) as usize * 4;
            if w_sum > 0.0 {
                dst[dst_idx] = (r_sum / w_sum).round().clamp(0.0, 255.0) as u8;
                dst[dst_idx + 1] = (g_sum / w_sum).round().clamp(0.0, 255.0) as u8;
                dst[dst_idx + 2] = (b_sum / w_sum).round().clamp(0.0, 255.0) as u8;
                dst[dst_idx + 3] = (a_sum / w_sum).round().clamp(0.0, 255.0) as u8;
            }
        }
    }

    dst
}

/// Converts a PNG image into a multi-frame Windows `.ico` container containing
/// 16, 32, 48, and 256 pixel PNG-encoded frames.
pub fn ico_from_png(png: &[u8]) -> Result<Vec<u8>, String> {
    let (src_w, src_h, rgba) = decode_png_to_rgba(png)?;

    let sizes = [16u32, 32, 48, 256];
    let mut encoded_frames = Vec::with_capacity(4);

    for &size in &sizes {
        let resized = resize_rgba(src_w, src_h, &rgba, size, size);
        let encoded = encode_png(size, size, &resized)?;
        encoded_frames.push((size, encoded));
    }

    // Build ICONDIR + ICONDIRENTRY container
    let count = encoded_frames.len() as u16;
    let mut ico = Vec::new();

    // ICONDIR: idReserved (0), idType (1 = ICON), idCount (4)
    ico.extend_from_slice(&0u16.to_le_bytes());
    ico.extend_from_slice(&1u16.to_le_bytes());
    ico.extend_from_slice(&count.to_le_bytes());

    let header_size = 6 + (count as usize) * 16;
    let mut current_offset = header_size as u32;

    for (size, data) in &encoded_frames {
        let w_byte = if *size >= 256 { 0 } else { *size as u8 };
        let h_byte = if *size >= 256 { 0 } else { *size as u8 };
        let bytes_in_res = data.len() as u32;

        ico.push(w_byte); // bWidth
        ico.push(h_byte); // bHeight
        ico.push(0); // bColorCount (0 = no palette)
        ico.push(0); // bReserved
        ico.extend_from_slice(&1u16.to_le_bytes()); // wPlanes
        ico.extend_from_slice(&32u16.to_le_bytes()); // wBitCount
        ico.extend_from_slice(&bytes_in_res.to_le_bytes());
        ico.extend_from_slice(&current_offset.to_le_bytes());

        current_offset += bytes_in_res;
    }

    for (_, data) in encoded_frames {
        ico.extend_from_slice(&data);
    }

    Ok(ico)
}

/// Lists all frames in an ICO container as `(size, payload)`.
pub fn ico_frames(ico: &[u8]) -> Vec<(u32, Vec<u8>)> {
    if ico.len() < 6 {
        return Vec::new();
    }
    if ico[0..2] != [0, 0] || ico[2..4] != [1, 0] {
        return Vec::new();
    }
    let count = u16::from_le_bytes([ico[4], ico[5]]) as usize;
    if ico.len() < 6 + count * 16 {
        return Vec::new();
    }

    let mut frames = Vec::with_capacity(count);
    for i in 0..count {
        let entry_pos = 6 + i * 16;
        let w_byte = ico[entry_pos];
        let w = if w_byte == 0 { 256 } else { u32::from(w_byte) };
        let bytes_in_res = u32::from_le_bytes([
            ico[entry_pos + 8],
            ico[entry_pos + 9],
            ico[entry_pos + 10],
            ico[entry_pos + 11],
        ]) as usize;
        let image_offset = u32::from_le_bytes([
            ico[entry_pos + 12],
            ico[entry_pos + 13],
            ico[entry_pos + 14],
            ico[entry_pos + 15],
        ]) as usize;

        if image_offset + bytes_in_res <= ico.len() {
            let payload = ico[image_offset..image_offset + bytes_in_res].to_vec();
            frames.push((w, payload));
        }
    }

    frames
}

/// Decodes the ICO frame of `size` (or nearest larger) to RGBA8 pixels.
pub fn ico_frame_rgba(ico: &[u8], size: u32) -> Option<(u32, u32, Vec<u8>)> {
    let frames = ico_frames(ico);
    if frames.is_empty() {
        return None;
    }

    // Find frame matching exact size, or smallest frame >= size, or largest available
    let exact = frames.iter().find(|(s, _)| *s == size);
    let larger = frames
        .iter()
        .filter(|(s, _)| *s >= size)
        .min_by_key(|(s, _)| *s);
    let fallback = frames.iter().max_by_key(|(s, _)| *s);

    let chosen = exact.or(larger).or(fallback)?;
    let payload = &chosen.1;

    // Check if payload is PNG
    const PNG_MAGIC: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    if payload.starts_with(&PNG_MAGIC) {
        return decode_png_to_rgba(payload).ok();
    }

    // BMP DIB payload (BITMAPINFOHEADER is at least 40 bytes)
    if payload.len() >= 40 {
        let w = u32::from_le_bytes([payload[4], payload[5], payload[6], payload[7]]);
        let h_doubled = i32::from_le_bytes([payload[8], payload[9], payload[10], payload[11]]);
        let h = h_doubled.unsigned_abs() / 2;
        let bpp = u16::from_le_bytes([payload[14], payload[15]]);

        if bpp == 32 && payload.len() >= 40 + (w * h * 4) as usize {
            let xor_bytes = &payload[40..40 + (w * h * 4) as usize];
            let mut rgba = vec![0u8; (w * h * 4) as usize];
            let mut has_nonzero_alpha = false;

            // DIB rows are bottom-to-top
            for y in 0..h {
                let src_row = (h - 1 - y) * w * 4;
                let dst_row = y * w * 4;
                for x in 0..w {
                    let s = (src_row + x * 4) as usize;
                    let d = (dst_row + x * 4) as usize;
                    let b = xor_bytes[s];
                    let g = xor_bytes[s + 1];
                    let r = xor_bytes[s + 2];
                    let a = xor_bytes[s + 3];
                    if a > 0 {
                        has_nonzero_alpha = true;
                    }
                    rgba[d] = r;
                    rgba[d + 1] = g;
                    rgba[d + 2] = b;
                    rgba[d + 3] = a;
                }
            }

            // If all alpha values are zero, read transparency from 1-bit AND mask
            if !has_nonzero_alpha {
                let mask_offset = 40 + (w * h * 4) as usize;
                let mask_stride = (w.div_ceil(32) * 4) as usize;
                for y in 0..h {
                    let mask_row_idx = (h - 1 - y) as usize * mask_stride;
                    let dst_row = (y * w * 4) as usize;
                    for x in 0..w {
                        let byte_idx = mask_offset + mask_row_idx + (x as usize / 8);
                        let bit_idx = 7 - (x % 8);
                        let is_transparent =
                            byte_idx < payload.len() && ((payload[byte_idx] >> bit_idx) & 1) != 0;
                        let d = dst_row + (x as usize * 4);
                        rgba[d + 3] = if is_transparent { 0 } else { 255 };
                    }
                }
            }

            return Some((w, h, rgba));
        }
    }

    None
}

/// The project's icon for the `.exe` (resource 1, via `app.rc`).
pub const ICON_ICO: &str = "icon.ico";
/// Its 256 px frame as PNG: what the egui window shows in its title bar.
pub const ICON_PNG: &str = "icon.png";

/// The `.ico` of `source`: the crab, a `.ico` file as it is, or a `.png` turned
/// into a 16·32·48·256 `.ico`. What the dialog previews and what OK saves.
///
/// # Errors
/// Not a `.ico` / `.png`, unreadable, or not a valid image.
pub fn icon_bytes(source: &IconSource) -> Result<Vec<u8>, String> {
    let IconSource::File(path) = source else {
        return Ok(DEFAULT_ICON.to_vec());
    };
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if ext != "ico" && ext != "png" {
        return Err("only .ico or .png".to_string());
    }
    let bytes = std::fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let ico = if ext == "png" {
        ico_from_png(&bytes)?
    } else {
        bytes
    };
    ico_frame_rgba(&ico, 256).ok_or("not a valid .ico (no readable frame)")?;
    Ok(ico)
}

/// The icon the project has now: its own [`ICON_ICO`] when `Custom`, else the crab.
pub fn current_icon(project_dir: &Path, choice: IconChoice) -> Vec<u8> {
    let own =
        (choice == IconChoice::Custom).then(|| std::fs::read(project_dir.join(ICON_ICO)).ok());
    own.flatten().unwrap_or_else(|| DEFAULT_ICON.to_vec())
}

/// The frame of `size` (or the nearest larger) as PNG, for a preview.
pub fn preview_png(ico: &[u8], size: u32) -> Option<Vec<u8>> {
    let (w, h, rgba) = ico_frame_rgba(ico, size)?;
    encode_png(w, h, &rgba).ok()
}

/// The Windows resource script beside `build.rs`: icon 1, and the title as the
/// `.exe`'s description and product name (what Task Manager and Explorer show).
pub const APP_RC: &str = "app.rc";

/// [`APP_RC`]'s text for `title` (UTF-8, quotes doubled as `rc` wants them).
pub fn app_rc(title: &str) -> String {
    let t = title.replace('"', "\"\"");
    format!(
        "#pragma code_page(65001)\n1 ICON \"{ICON_ICO}\"\n1 VERSIONINFO\nBEGIN\n  BLOCK \"StringFileInfo\"\n  BEGIN\n    BLOCK \"040904b0\"\n    BEGIN\n      VALUE \"FileDescription\", \"{t}\"\n      VALUE \"ProductName\", \"{t}\"\n    END\n  END\n  BLOCK \"VarFileInfo\"\n  BEGIN\n    VALUE \"Translation\", 0x409, 1200\n  END\nEND\n"
    )
}

/// Writes the chosen icon into the project folder as [`ICON_ICO`] + [`ICON_PNG`].
///
/// # Errors
/// What [`icon_bytes`] refuses, or a file that cannot be written.
pub fn apply_icon(project_dir: &Path, choice: IconSource) -> Result<IconChoice, String> {
    let ico = icon_bytes(&choice)?;
    let png = preview_png(&ico, 256).ok_or("not a valid .ico (no readable frame)")?;
    for (name, bytes) in [(ICON_ICO, &ico), (ICON_PNG, &png)] {
        std::fs::write(project_dir.join(name), bytes)
            .map_err(|e| format!("cannot write {name}: {e}"))?;
    }
    Ok(match choice {
        IconSource::Default => IconChoice::Default,
        IconSource::File(_) => IconChoice::Custom,
    })
}

/// The Application page's OK: the title and, when one was picked, the icon. Returns the
/// sentence for the status bar.
///
/// # Errors
/// `Cargo.toml` or the icon could not be written; nothing after the failure runs.
pub fn save(project_dir: &Path, title: &str, icon: Option<IconSource>) -> Result<String, String> {
    let current = read(project_dir).icon;
    let icon = match icon {
        Some(source) => apply_icon(project_dir, source)?,
        None => current,
    };
    // A project that embeds its icon (it has `app.rc`) takes the new title into the .exe too.
    let rc = project_dir.join(APP_RC);
    if rc.is_file() {
        std::fs::write(&rc, app_rc(title)).map_err(|e| format!("cannot write {APP_RC}: {e}"))?;
    }
    write(
        project_dir,
        &ApplicationOptions {
            title: title.to_string(),
            icon,
        },
    )?;
    Ok(format!(
        "Project options saved: title \"{title}\", icon {}",
        match icon {
            IconChoice::Default => "CrawRAD crab",
            IconChoice::Custom => ICON_ICO,
        }
    ))
}

/// The dialog's OK: the log settings are checked first, so a refused page writes
/// nothing; then the Application page, then the Log page.
pub fn save_dialog(
    project_dir: &Path,
    st: &crawrad_actions::project_options_dialog::ProjectOptionsState,
) -> Result<String, String> {
    crawrad_actions::project_log::check(&st.log)?;
    let said = save(project_dir, &st.title, st.pending_icon.clone())?;
    write_log(project_dir, &st.log)?;
    Ok(said)
}

#[cfg(test)]
#[path = "test/project_options/test.rs"]
mod project_options_test;
