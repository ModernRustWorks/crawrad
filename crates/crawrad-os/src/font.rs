//! Native font family discovery and font picker dialog.
//!
//! # Why this module lives in `crawrad-os`
//!
//! Querying installed font families and opening the native font picker dialog requires
//! calling platform-specific operating system APIs (Win32 GDI / Dialogs on Windows,
//! fontconfig on Linux). As mandated by ADR 0009, unsafe system FFI calls are strictly
//! isolated to the `crawrad-os` adapter crate.
//!
//! # Why `installed_families()` is cached
//!
//! Enumerating system font families takes tens of milliseconds (querying GDI or fontconfig),
//! and the Object Inspector repaints on every frame. Caching the sorted, deduplicated family
//! list in a static [`std::sync::OnceLock`] ensures subsequent queries are instantaneous.
//!
//! # Non-Windows platforms
//!
//! On non-Windows platforms (Linux/macOS), no standard native font picker dialog is
//! integrated into this headless adapter; [`choose_font`] returns `None` while sub-rows in
//! the Object Inspector allow complete in-place editing of all font properties.

use crawrad_note::note;
use std::sync::OnceLock;

#[note("font-crawrad-font-dialog-answer")]
pub const CRAWRAD_FONT_DIALOG_ANSWER: &str = "CRAWRAD_FONT_DIALOG_ANSWER";

#[note("font-test-answer")]
pub fn test_answer() -> Option<String> {
    std::env::var(CRAWRAD_FONT_DIALOG_ANSWER).ok()
}

#[note("font-fontchoice")]
#[derive(Debug, Clone, PartialEq)]
pub struct FontChoice {
    pub name: String,
    pub size: f32,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike_out: bool,
    pub rgb: [u8; 3],
}

impl Default for FontChoice {
    fn default() -> Self {
        Self {
            name: "Segoe UI".to_string(),
            size: 9.0,
            bold: false,
            italic: false,
            underline: false,
            strike_out: false,
            rgb: [0, 0, 0],
        }
    }
}

static INSTALLED_FAMILIES: OnceLock<Vec<String>> = OnceLock::new();

#[note("font-installed-families")]
pub fn installed_families() -> &'static [String] {
    INSTALLED_FAMILIES.get_or_init(enumerate_installed_families)
}

#[cfg(target_os = "windows")]
fn enumerate_installed_families() -> Vec<String> {
    use windows_sys::Win32::Foundation::LPARAM;
    use windows_sys::Win32::Graphics::Gdi::{
        DEFAULT_CHARSET, EnumFontFamiliesExW, GetDC, LOGFONTW, ReleaseDC, TEXTMETRICW,
    };

    let mut families = Vec::new();

    unsafe extern "system" fn enum_font_fam_ex_proc(
        lpelf: *const LOGFONTW,
        _lpntm: *const TEXTMETRICW,
        _font_type: u32,
        lparam: LPARAM,
    ) -> i32 {
        if !lpelf.is_null() {
            let families = unsafe { &mut *(lparam as *mut Vec<String>) };
            let face = unsafe { &(*lpelf).lfFaceName };
            let len = face.iter().position(|&c| c == 0).unwrap_or(face.len());
            let name = String::from_utf16_lossy(&face[..len]);
            let trimmed = name.trim();
            // Skip vertical font families beginning with '@'
            if !trimmed.is_empty() && !trimmed.starts_with('@') {
                families.push(trimmed.to_string());
            }
        }
        1
    }

    unsafe {
        let hdc = GetDC(std::ptr::null_mut());
        if !hdc.is_null() {
            let mut lf: LOGFONTW = std::mem::zeroed();
            lf.lfCharSet = DEFAULT_CHARSET;
            EnumFontFamiliesExW(
                hdc,
                &lf,
                Some(enum_font_fam_ex_proc),
                &mut families as *mut _ as LPARAM,
                0,
            );
            ReleaseDC(std::ptr::null_mut(), hdc);
        }
    }

    families.sort();
    families.dedup();
    families
}

#[cfg(not(target_os = "windows"))]
fn enumerate_installed_families() -> Vec<String> {
    let output = crate::process::no_console("fc-list")
        .arg(":")
        .arg("family")
        .output();
    if let Ok(out) = output
        && out.status.success()
        && let Ok(s) = String::from_utf8(out.stdout)
    {
        let mut families: Vec<String> = s
            .lines()
            .flat_map(|line| line.split(','))
            .map(str::trim)
            .filter(|name| !name.is_empty() && !name.starts_with('@'))
            .map(str::to_string)
            .collect();
        families.sort();
        families.dedup();
        return families;
    }
    Vec::new()
}

#[note("font-choose-font")]
pub fn choose_font(initial: &FontChoice) -> Option<FontChoice> {
    #[cfg(target_os = "windows")]
    {
        choose_font_win32(initial)
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = initial;
        None
    }
}

#[cfg(target_os = "windows")]
fn choose_font_win32(initial: &FontChoice) -> Option<FontChoice> {
    use windows_sys::Win32::Graphics::Gdi::{DEFAULT_CHARSET, LOGFONTW};
    use windows_sys::Win32::UI::Controls::Dialogs::{
        CF_EFFECTS, CF_INITTOLOGFONTSTRUCT, CF_SCREENFONTS, CHOOSEFONTW, ChooseFontW,
    };

    unsafe {
        let mut lf: LOGFONTW = std::mem::zeroed();
        lf.lfHeight = -((initial.size * 96.0 / 72.0).round() as i32);
        lf.lfWeight = if initial.bold { 700 } else { 400 };
        lf.lfItalic = if initial.italic { 1 } else { 0 };
        lf.lfUnderline = if initial.underline { 1 } else { 0 };
        lf.lfStrikeOut = if initial.strike_out { 1 } else { 0 };
        lf.lfCharSet = DEFAULT_CHARSET;

        let name_utf16: Vec<u16> = initial.name.encode_utf16().collect();
        for (i, &ch) in name_utf16.iter().take(31).enumerate() {
            lf.lfFaceName[i] = ch;
        }

        let mut cf: CHOOSEFONTW = std::mem::zeroed();
        cf.lStructSize = std::mem::size_of::<CHOOSEFONTW>() as u32;
        cf.lpLogFont = &mut lf;
        cf.Flags = CF_SCREENFONTS | CF_EFFECTS | CF_INITTOLOGFONTSTRUCT;
        cf.rgbColors = (initial.rgb[0] as u32)
            | ((initial.rgb[1] as u32) << 8)
            | ((initial.rgb[2] as u32) << 16);

        if ChooseFontW(&mut cf) != 0 {
            let face_len = lf
                .lfFaceName
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(lf.lfFaceName.len());
            let name = String::from_utf16_lossy(&lf.lfFaceName[..face_len]);
            let size = if cf.iPointSize > 0 {
                cf.iPointSize as f32 / 10.0
            } else {
                initial.size
            };
            let bold = lf.lfWeight >= 700;
            let italic = lf.lfItalic != 0;
            let underline = lf.lfUnderline != 0;
            let strike_out = lf.lfStrikeOut != 0;
            let r = (cf.rgbColors & 0xFF) as u8;
            let g = ((cf.rgbColors >> 8) & 0xFF) as u8;
            let b = ((cf.rgbColors >> 16) & 0xFF) as u8;

            Some(FontChoice {
                name,
                size,
                bold,
                italic,
                underline,
                strike_out,
                rgb: [r, g, b],
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
#[path = "test/font/test.rs"]
mod tests;
