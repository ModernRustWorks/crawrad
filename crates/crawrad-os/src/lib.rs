//! **THE FFI ADAPTER** — the one crate in this workspace allowed to say
//! `unsafe`.
//!
//! # Why it exists
//!
//! [ADR 0009] wrote the rule in August 2026: *"every unsafe line of CrawRAD
//! lives in the FFI adapter, and the compiler guarantees the rest"*. Back then
//! the adapter was `crawrad-runtime-win32`, the Win32 control library. [ADR 0013]
//! retired it, and from that day the workspace had the rule and no house for
//! it — every crate declared `#![forbid(unsafe_code)]`, which is the same as
//! saying *"we no longer talk to the operating system"*.
//!
//! That held until the IDE had to ask a question no UI framework answers.
//!
//! # The question, and why nobody up the stack can answer it
//!
//! **Where does the taskbar end?** The Makepad shell maximizes its window and
//! the status bar disappears behind it. The cause is measured and is not ours:
//! the window is created `WS_POPUP`
//! (`makepad-platform-1.0.0/src/os/windows/win32_window.rs:320-326`), and
//! Windows maximizes a popup to the WHOLE MONITOR instead of the work area.
//! `ShowWindow(SW_MAXIMIZE)` on such a window covers the taskbar by design.
//!
//! Above this crate nobody can correct it:
//!
//! - `WindowGeom`, the only geometry Makepad publishes, carries
//!   `dpi_factor`, `position`, `inner_size` and `outer_size`
//!   (`makepad-platform-1.0.0/src/event/window.rs:11-20`) — **no work area,
//!   and not even the screen size**.
//! - `winit`, which `eframe` brings along, exposes a monitor's full size and
//!   never its usable part.
//! - The correct Win32 fix — answering `WM_GETMINMAXINFO` — is inside the
//!   framework's window procedure, which is not ours to write.
//!
//! So the answer has to be asked of the system directly, and asking the system
//! directly is `unsafe`. One crate carries that, and it is this one.
//!
//! # What this crate is NOT
//!
//! It is not a platform abstraction layer and it must not become one. It holds
//! the questions the frameworks above genuinely refuse to answer, each one
//! documented with the measurement that proves the refusal. A function that
//! could have been written safely one layer up does not belong here.
//!

use crawrad_note::note;
#[cfg(test)]
#[path = "test/capture/test.rs"]
mod capture_test;
#[cfg(test)]
#[path = "test/work_area/test.rs"]
mod work_area_test;

#[note("screenrect")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ScreenRect {
    /// Left edge.
    pub x: f64,
    /// Top edge.
    pub y: f64,
    /// Width, never negative.
    pub width: f64,
    /// Height, never negative.
    pub height: f64,
}

impl ScreenRect {
    #[note("screenrect-of-window")]
    pub fn of_window([x, y]: [f64; 2], [width, height]: [f64; 2], dpi: f64) -> Self {
        let dpi = if dpi > 0.0 { dpi } else { 1.0 };
        Self {
            x,
            y,
            width: width * dpi,
            height: height * dpi,
        }
    }

    #[note("screenrect-right")]
    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    #[note("screenrect-bottom")]
    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }

    #[note("screenrect-fits-in")]
    pub fn fits_in(&self, other: &ScreenRect) -> bool {
        self.x >= other.x
            && self.y >= other.y
            && self.right() <= other.right()
            && self.bottom() <= other.bottom()
    }
}

#[note("monitorarea")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MonitorArea {
    /// The whole monitor, taskbar included.
    pub full: ScreenRect,
    /// The part a maximized window may occupy — `rcWork` in Win32 terms.
    ///
    /// It excludes the taskbar and any other application desktop bar. It is
    /// always contained in [`MonitorArea::full`].
    pub work: ScreenRect,
}

#[note("monitor-at")]
#[cfg(target_os = "windows")]
pub fn monitor_at(point: (f64, f64)) -> Option<MonitorArea> {
    use windows_sys::Win32::Foundation::{POINT, RECT};
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromPoint,
    };

    fn to_rect(r: RECT) -> ScreenRect {
        ScreenRect {
            x: r.left as f64,
            y: r.top as f64,
            width: (r.right - r.left).max(0) as f64,
            height: (r.bottom - r.top).max(0) as f64,
        }
    }

    let pt = POINT {
        x: point.0 as i32,
        y: point.1 as i32,
    };
    let mut info = MONITORINFO {
        cbSize: core::mem::size_of::<MONITORINFO>() as u32,
        ..Default::default()
    };

    // SAFETY: `MonitorFromPoint` takes `POINT` by value and returns a handle
    // the system owns — there is nothing to free and nothing to keep. With
    // `MONITOR_DEFAULTTONEAREST` it cannot return null, so `GetMonitorInfoW`
    // always gets a live handle.
    //
    // `GetMonitorInfoW` writes into `info`, and the contract is `cbSize`:
    // it is filled above with the size of the very struct being passed, so the
    // system writes exactly the four fields declared and not one byte more.
    // Asking for `MONITORINFOEX` with this `cbSize` is what would corrupt the
    // stack, and that is why the plain `MONITORINFO` is the one used.
    let described = unsafe {
        let monitor = MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST);
        GetMonitorInfoW(monitor, &mut info) != 0
    };
    if !described {
        return None;
    }
    Some(MonitorArea {
        full: to_rect(info.rcMonitor),
        work: to_rect(info.rcWork),
    })
}

#[note("monitor-at-2")]
#[cfg(not(target_os = "windows"))]
pub fn monitor_at(_point: (f64, f64)) -> Option<MonitorArea> {
    None
}

#[note("place-main-window")]
#[cfg(feature = "full")]
#[cfg(target_os = "windows")]
pub fn place_main_window(rect: ScreenRect) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{MoveWindow, PostMessageW, WM_SIZE};

    let Some(hwnd) = main_window() else {
        return false;
    };
    // SAFETY: `hwnd` came from `EnumThreadWindows` on THIS thread moments ago
    // and is a window this process owns, so it cannot have been destroyed by
    // another thread in between. `MoveWindow` and `PostMessageW` only read it;
    // neither takes ownership and there is nothing to free.
    //
    // `bRepaint` is TRUE, and that is the second half of the defect: with
    // FALSE — which is what the framework passes (`win32_window.rs:944`) — the
    // strip the window has just gained is never invalidated, so it keeps
    // whatever was behind it until something else paints.
    unsafe {
        disable_window_rounding();
        if MoveWindow(
            hwnd,
            rect.x as i32,
            rect.y as i32,
            rect.width as i32,
            rect.height as i32,
            1,
        ) == 0
        {
            return false;
        }
        // The message the framework posts to itself for the same reason.
        PostMessageW(hwnd, WM_SIZE, 0, 0);
    }
    true
}

#[note("disable-window-rounding")]
#[cfg(feature = "full")]
#[cfg(target_os = "windows")]
pub fn disable_window_rounding() -> bool {
    use windows_sys::Win32::Graphics::Dwm::{
        DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_DONOTROUND, DwmSetWindowAttribute,
    };

    let Some(hwnd) = main_window() else {
        return false;
    };
    let corner_preference = DWMWCP_DONOTROUND;
    unsafe {
        // DWMWA_BORDER_COLOR = 34, DWMWA_COLOR_NONE = 0xFFFFFFFE.
        // Suppresses the 1px border the DWM draws on Windows 11 when square corners
        // are set on windows without the standard frame.
        let border_none: u32 = 0xFFFFFFFE;
        let _ = DwmSetWindowAttribute(
            hwnd,
            34,
            &border_none as *const _ as *const _,
            std::mem::size_of_val(&border_none) as u32,
        );

        DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE as u32,
            &corner_preference as *const _ as *const _,
            std::mem::size_of_val(&corner_preference) as u32,
        ) == 0
    }
}

#[note("disable-window-rounding-2")]
#[cfg(feature = "full")]
#[cfg(not(target_os = "windows"))]
pub fn disable_window_rounding() -> bool {
    false
}

#[note("prepare-main-window")]
#[cfg(feature = "full")]
#[cfg(target_os = "windows")]
pub fn prepare_main_window() -> bool {
    use std::sync::atomic::{AtomicBool, Ordering};
    use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromWindow,
    };
    use windows_sys::Win32::UI::Shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass};
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        IsZoomed, MINMAXINFO, WM_GETMINMAXINFO, WM_NCCALCSIZE, WM_NCDESTROY,
    };

    const SUBCLASS_ID: usize = 0x4352_4157;
    static INSTALLED: AtomicBool = AtomicBool::new(false);

    // SAFETY: Registered as a window subclass procedure with `SetWindowSubclass`.
    // Windows invokes it with valid window messages on the UI thread.
    unsafe extern "system" fn maximize_in_work_area(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        _id: usize,
        _data: usize,
    ) -> LRESULT {
        if msg == WM_GETMINMAXINFO {
            // SAFETY: `DefSubclassProc` queries the next procedure in the subclass chain
            // to populate default minimum and maximum sizes.
            let r = unsafe { DefSubclassProc(hwnd, msg, wparam, lparam) };
            let mut info = MONITORINFO {
                cbSize: core::mem::size_of::<MONITORINFO>() as u32,
                ..Default::default()
            };
            // SAFETY: `MonitorFromWindow` with `MONITOR_DEFAULTTONEAREST` retrieves the
            // nearest display monitor for the window without returning null.
            let monitor = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) };
            if !monitor.is_null() && unsafe { GetMonitorInfoW(monitor, &mut info) } != 0 {
                // SAFETY: For `WM_GETMINMAXINFO`, `lparam` points to a valid `MINMAXINFO`
                // structure provided by the system.
                let mmi = unsafe { &mut *(lparam as *mut MINMAXINFO) };
                mmi.ptMaxPosition.x = info.rcWork.left - info.rcMonitor.left;
                mmi.ptMaxPosition.y = info.rcWork.top - info.rcMonitor.top;
                mmi.ptMaxSize.x = info.rcWork.right - info.rcWork.left;
                mmi.ptMaxSize.y = info.rcWork.bottom - info.rcWork.top;
            }
            return r;
        }
        // Maximized, the window already IS the work area (above), so its client is the
        // whole window: Makepad hands this message to the default procedure, which
        // takes the sizing border off every side and leaves a 7 px empty frame.
        // SAFETY: `IsZoomed` only reads the state of the valid `hwnd` Windows passed in.
        if msg == WM_NCCALCSIZE && wparam != 0 && unsafe { IsZoomed(hwnd) } != 0 {
            return 0;
        }
        if msg == WM_NCDESTROY {
            // SAFETY: Cleans up the subclass when the window's non-client area is destroyed.
            unsafe {
                RemoveWindowSubclass(hwnd, Some(maximize_in_work_area), SUBCLASS_ID);
            }
        }
        // SAFETY: DefSubclassProc forwards all remaining messages down the subclass chain.
        unsafe { DefSubclassProc(hwnd, msg, wparam, lparam) }
    }

    disable_window_rounding();

    let Some(hwnd) = main_window() else {
        return false;
    };

    if !INSTALLED.swap(true, Ordering::SeqCst) {
        // SAFETY: `hwnd` is a valid window handle returned by `main_window()`.
        // `SetWindowSubclass` associates the subclass procedure with `hwnd`.
        let ok = unsafe { SetWindowSubclass(hwnd, Some(maximize_in_work_area), SUBCLASS_ID, 0) };
        if ok == 0 {
            // Not installed: let the next call try again.
            INSTALLED.store(false, Ordering::SeqCst);
        }
    }

    true
}

#[note("prepare-main-window-2")]
#[cfg(feature = "full")]
#[cfg(not(target_os = "windows"))]
pub fn prepare_main_window() -> bool {
    false
}

#[note("nudge-resize")]
#[cfg(feature = "full")]
#[cfg(target_os = "windows")]
pub fn nudge_resize() -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{PostMessageW, WM_SIZE};

    let Some(hwnd) = main_window() else {
        return false;
    };
    // SAFETY: `hwnd` came from `EnumThreadWindows` on THIS thread moments ago
    // and is a window this process owns. `PostMessageW` only queues a message
    // against it; it takes no ownership and there is nothing to free.
    unsafe {
        PostMessageW(hwnd, WM_SIZE, 0, 0);
    }
    true
}

#[note("nudge-resize-2")]
#[cfg(feature = "full")]
#[cfg(not(target_os = "windows"))]
pub fn nudge_resize() -> bool {
    false
}

#[note("main-window")]
#[cfg(feature = "full")]
#[cfg(target_os = "windows")]
fn main_window() -> Option<windows_sys::Win32::Foundation::HWND> {
    use windows_sys::Win32::Foundation::{HWND, LPARAM};
    use windows_sys::Win32::System::Threading::GetCurrentThreadId;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        EnumThreadWindows, GW_OWNER, GetWindow, IsWindowVisible,
    };
    use windows_sys::core::BOOL;

    let mut found: Vec<HWND> = Vec::new();

    // SAFETY: the callback receives the `LPARAM` this call passes, and nothing
    // else touches it for the duration — `EnumThreadWindows` is synchronous.
    unsafe extern "system" fn collect_window(hwnd: HWND, lparam: LPARAM) -> BOOL {
        // SAFETY: `lparam` is the `&mut Vec<HWND>` handed to
        // `EnumThreadWindows` just below, alive for the whole enumeration.
        let out = unsafe { &mut *(lparam as *mut Vec<HWND>) };
        // Visible and unowned — the top-level window, not a tooltip, not a
        // modal dialog, not the invisible message window a toolkit may keep.
        let visible = unsafe { IsWindowVisible(hwnd) } != 0;
        let unowned = unsafe { GetWindow(hwnd, GW_OWNER) }.is_null();
        if visible && unowned {
            out.push(hwnd);
        }
        1
    }

    // SAFETY: the thread id is this thread's, and `found` outlives the call.
    unsafe {
        EnumThreadWindows(
            GetCurrentThreadId(),
            Some(collect_window),
            &mut found as *mut Vec<HWND> as LPARAM,
        );
    }
    match found.len() {
        1 => Some(found[0]),
        _ => None,
    }
}

#[note("capture")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capture {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// `width * height` RGBA pixels, rows top-down.
    pub rgba: Vec<u8>,
}

#[note("crop-bgra")]
pub fn crop_bgra(src: &[u8], src_width: u32, src_height: u32, region: [i32; 4]) -> Option<Capture> {
    let [rx, ry, rw, rh] = region;
    if rw <= 0 || rh <= 0 || src_width == 0 || src_height == 0 {
        return None;
    }
    let expected_len = (src_width as usize)
        .checked_mul(src_height as usize)?
        .checked_mul(4)?;
    if src.len() < expected_len {
        return None;
    }

    let left = rx.max(0);
    let top = ry.max(0);
    let right = rx.saturating_add(rw).min(src_width as i32);
    let bottom = ry.saturating_add(rh).min(src_height as i32);

    if left >= right || top >= bottom {
        return None;
    }

    let crop_width = (right - left) as u32;
    let crop_height = (bottom - top) as u32;
    let capacity = (crop_width as usize)
        .checked_mul(crop_height as usize)?
        .checked_mul(4)?;
    let mut rgba = Vec::with_capacity(capacity);

    for y in top..bottom {
        for x in left..right {
            let index = (y as usize * src_width as usize + x as usize) * 4;
            let b = src[index];
            let g = src[index + 1];
            let r = src[index + 2];
            rgba.push(r);
            rgba.push(g);
            rgba.push(b);
            rgba.push(255);
        }
    }

    Some(Capture {
        width: crop_width,
        height: crop_height,
        rgba,
    })
}

#[note("capture-main-window")]
#[cfg(feature = "full")]
#[cfg(target_os = "windows")]
pub fn capture_main_window(region: Option<[i32; 4]>) -> Option<Capture> {
    use windows_sys::Win32::Foundation::{HWND, RECT};
    use windows_sys::Win32::Graphics::Gdi::{
        BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleBitmap, CreateCompatibleDC,
        DIB_RGB_COLORS, DeleteDC, DeleteObject, GetDC, GetDIBits, HDC, HGDIOBJ, ReleaseDC,
        SelectObject,
    };
    use windows_sys::Win32::Storage::Xps::PrintWindow;
    use windows_sys::Win32::UI::WindowsAndMessaging::GetClientRect;

    const PW_CLIENTONLY: u32 = 1;
    const PW_RENDERFULLCONTENT: u32 = 2;

    struct GdiCaptureGuard {
        hwnd: HWND,
        dc: HDC,
        mem_dc: HDC,
        bitmap: HGDIOBJ,
        old_bitmap: HGDIOBJ,
    }

    impl Drop for GdiCaptureGuard {
        fn drop(&mut self) {
            // SAFETY: Selected handles are restored and GDI resources are destroyed in reverse order.
            unsafe {
                if !self.old_bitmap.is_null() {
                    SelectObject(self.mem_dc, self.old_bitmap);
                }
                if !self.bitmap.is_null() {
                    DeleteObject(self.bitmap);
                }
                if !self.mem_dc.is_null() {
                    DeleteDC(self.mem_dc);
                }
                if !self.dc.is_null() {
                    ReleaseDC(self.hwnd, self.dc);
                }
            }
        }
    }

    let hwnd = main_window()?;

    let mut client_rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    // SAFETY: `hwnd` is a valid top-level window owned by this thread.
    let rect_ok = unsafe { GetClientRect(hwnd, &mut client_rect) };
    if rect_ok == 0 {
        return None;
    }

    let cw = client_rect.right - client_rect.left;
    let ch = client_rect.bottom - client_rect.top;
    if cw <= 0 || ch <= 0 {
        return None;
    }

    let mut guard = GdiCaptureGuard {
        hwnd,
        dc: std::ptr::null_mut(),
        mem_dc: std::ptr::null_mut(),
        bitmap: std::ptr::null_mut(),
        old_bitmap: std::ptr::null_mut(),
    };

    // SAFETY: `hwnd` is valid and DC is released by `guard` upon drop.
    guard.dc = unsafe { GetDC(hwnd) };
    if guard.dc.is_null() {
        return None;
    }

    // SAFETY: Creating memory device context compatible with window DC.
    guard.mem_dc = unsafe { CreateCompatibleDC(guard.dc) };
    if guard.mem_dc.is_null() {
        return None;
    }

    // SAFETY: Creating DIB/bitmap compatible with window DC of client area dimensions.
    guard.bitmap = unsafe { CreateCompatibleBitmap(guard.dc, cw, ch) as HGDIOBJ };
    if guard.bitmap.is_null() {
        return None;
    }

    // SAFETY: Selecting compatible bitmap into memory DC; previous GDI object is stored for restore.
    guard.old_bitmap = unsafe { SelectObject(guard.mem_dc, guard.bitmap) };
    if guard.old_bitmap.is_null() {
        return None;
    }

    // SAFETY: Window content drawn by Direct3D requires `PW_RENDERFULLCONTENT` to capture visual backing store.
    let printed = unsafe { PrintWindow(hwnd, guard.mem_dc, PW_CLIENTONLY | PW_RENDERFULLCONTENT) };
    if printed == 0 {
        return None;
    }

    // `GetDIBits` must not read a bitmap that is still selected into a DC.
    // SAFETY: both handles were made above and are still alive in `guard`.
    unsafe { SelectObject(guard.mem_dc, guard.old_bitmap) };
    guard.old_bitmap = std::ptr::null_mut();

    // SAFETY: `BITMAPINFO` is plain data; all-zero is a valid value of it.
    let mut bmi: BITMAPINFO = unsafe { std::mem::zeroed() };
    bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    bmi.bmiHeader.biWidth = cw;
    bmi.bmiHeader.biHeight = -ch;
    bmi.bmiHeader.biPlanes = 1;
    bmi.bmiHeader.biBitCount = 32;
    bmi.bmiHeader.biCompression = BI_RGB;

    let byte_count = (cw as usize).checked_mul(ch as usize)?.checked_mul(4)?;
    let mut bgra_bytes = vec![0u8; byte_count];

    // SAFETY: `guard.mem_dc` and `guard.bitmap` are valid; buffer size matches 32 bpp top-down dimensions.
    let lines_copied = unsafe {
        GetDIBits(
            guard.mem_dc,
            guard.bitmap as _,
            0,
            ch as u32,
            bgra_bytes.as_mut_ptr() as *mut _,
            &mut bmi as *mut _,
            DIB_RGB_COLORS,
        )
    };
    if lines_copied == 0 {
        return None;
    }

    let target_region = region.unwrap_or([0, 0, cw, ch]);
    crop_bgra(&bgra_bytes, cw as u32, ch as u32, target_region)
}

#[note("capture-main-window-2")]
#[cfg(feature = "full")]
#[cfg(not(target_os = "windows"))]
pub fn capture_main_window(_region: Option<[i32; 4]>) -> Option<Capture> {
    None
}

#[note("place-main-window-2")]
#[cfg(feature = "full")]
#[cfg(not(target_os = "windows"))]
pub fn place_main_window(_rect: ScreenRect) -> bool {
    false
}

#[cfg(feature = "full")]
#[note("set-env-for-children")]
pub fn set_env_for_children(key: &str, value: &str) {
    // SAFETY: see above: one call, before any reader is started.
    unsafe { std::env::set_var(key, value) };
}

#[cfg(feature = "full")]
pub mod clipboard;
#[cfg(feature = "full")]
pub mod clock;
#[cfg(feature = "full")]
pub mod crash_report;
#[cfg(feature = "full")]
pub mod font;
#[cfg(feature = "full")]
pub mod ipc;
#[cfg(feature = "full")]
pub mod process;
#[cfg(feature = "full")]
pub mod splash_window;
#[cfg(feature = "full")]
pub mod tasks;
#[cfg(feature = "full")]
pub mod tree;
#[cfg(feature = "full")]
pub mod watch;

/// **A PATH INSIDE `WindowsApps` IS UNUSABLE TO EVERYONE BUT THE PACKAGE.**
///
/// An installed Store app lives under `C:\Program Files\WindowsApps\`, a folder
/// Windows locks down: the package itself reads it, so an `is_file()` check
/// answers yes, and every other process is refused. Two places need to know this
/// and for different reasons - where components may be installed, and which
/// command may be written into a project's `.mcp.json` - so the fact lives here,
/// with the rest of what this house knows about the operating system, instead of
/// being stated twice and drifting.
///
/// Writing such a path into `.mcp.json` sent the agent's CLI, a separate process,
/// to `fork/exec ... Access is denied`, and it started with no MCP server at all.
#[note("is-inside-store-package")]
pub fn is_inside_store_package(path: &std::path::Path) -> bool {
    path.components().any(|c| {
        c.as_os_str()
            .to_str()
            .is_some_and(|s| s.eq_ignore_ascii_case("WindowsApps"))
    })
}
