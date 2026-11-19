//! A splash window: a borderless Win32 window, on a thread of its
//! own, that paints what a [`Painter`] hands it — a picture, filled
//! rectangles and lines of text — and closes when the painter says so.
//!
//! It knows nothing of what it shows; that is `crawrad-splash`. This crate only
//! carries the `unsafe` the window needs (ADR 0009). It tells the painter on
//! every tick whether another window of this process is on the screen, which
//! is how a splash knows the application it covers has shown up.
//!
//! Off Windows [`spawn`] does nothing.

use crawrad_note::note;
#[note("splash-window-area")]
pub type Area = [f32; 4];

#[note("splash-window-fill")]
#[derive(Debug, Clone, PartialEq)]
pub struct Fill {
    pub area: Area,
    pub color: u32,
}

#[note("splash-window-text")]
#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub text: String,
    pub area: Area,
    pub color: u32,
    /// Font height in 96-DPI pixels.
    pub size: f32,
    /// 400 regular, 600 semibold, 800 extra bold.
    pub weight: i32,
    pub align_right: bool,
}

#[note("splash-window-frame")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Frame {
    pub fills: Vec<Fill>,
    pub texts: Vec<Text>,
}

#[note("splash-window-painter")]
pub trait Painter: Send + 'static {
    #[note("splash-window-painter-size")]
    fn size(&self) -> (f32, f32);

    #[note("splash-window-painter-picture")]
    fn picture(&self, width: usize, height: usize) -> Vec<u32>;

    #[note("splash-window-painter-frame")]
    fn frame(&mut self, other_window_up: bool) -> Option<Frame>;
}

#[note("splash-window-spawn")]
pub fn spawn(painter: impl Painter) {
    let _ = std::thread::Builder::new()
        .name("splash".into())
        .spawn(move || window::run(Box::new(painter)));
}

#[cfg(not(target_os = "windows"))]
mod window {
    pub(super) fn run(_: Box<dyn super::Painter>) {}
}

#[cfg(target_os = "windows")]
mod window {
    use super::{Frame, Painter};
    use crawrad_note::note;
    use std::cell::RefCell;
    use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM};
    use windows_sys::Win32::Graphics::Gdi::{
        BI_RGB, BITMAPINFO, BITMAPINFOHEADER, BeginPaint, BitBlt, CreateCompatibleBitmap,
        CreateCompatibleDC, CreateFontW, CreateSolidBrush, DIB_RGB_COLORS, DT_LEFT, DT_NOPREFIX,
        DT_RIGHT, DT_SINGLELINE, DT_VCENTER, DeleteDC, DeleteObject, DrawTextW, EndPaint, FillRect,
        InvalidateRect, PAINTSTRUCT, SRCCOPY, SelectObject, SetBkMode, SetDIBitsToDevice,
        SetTextColor, TRANSPARENT,
    };
    use windows_sys::Win32::UI::HiDpi::{
        DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, GetDpiForSystem, SetThreadDpiAwarenessContext,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CS_HREDRAW, CS_VREDRAW, CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW,
        EnumWindows, GetMessageW, GetSystemMetrics, GetWindowThreadProcessId, IsWindowVisible,
        KillTimer, MSG, PostQuitMessage, RegisterClassW, SM_CXSCREEN, SM_CYSCREEN, SetTimer,
        TranslateMessage, WM_DESTROY, WM_PAINT, WM_TIMER, WNDCLASSW, WS_EX_TOOLWINDOW, WS_POPUP,
        WS_VISIBLE,
    };
    use windows_sys::core::BOOL;

    const TICK_MS: u32 = 50;

    struct Context {
        painter: Box<dyn Painter>,
        frame: Frame,
        picture: Vec<u32>,
        width: i32,
        height: i32,
        scale: f32,
    }

    thread_local! {
        static CONTEXT: RefCell<Option<Context>> = const { RefCell::new(None) };
    }

    fn wide(text: &str) -> Vec<u16> {
        text.encode_utf16().chain(std::iter::once(0)).collect()
    }

    #[note("splash-window-window-colorref")]
    fn colorref(c: u32) -> u32 {
        ((c & 0xff) << 16) | (c & 0xff00) | ((c >> 16) & 0xff)
    }

    #[note("splash-window-window-search")]
    struct Search {
        pid: u32,
        own: HWND,
        found: bool,
    }

    unsafe extern "system" fn visit(hwnd: HWND, lparam: LPARAM) -> BOOL {
        // SAFETY: `lparam` is the `Search` that `other_window_up` lends for
        // the duration of `EnumWindows`.
        unsafe {
            let search = &mut *(lparam as *mut Search);
            let mut pid = 0;
            GetWindowThreadProcessId(hwnd, &mut pid);
            if pid == search.pid && hwnd != search.own && IsWindowVisible(hwnd) != 0 {
                search.found = true;
                return 0;
            }
            1
        }
    }

    fn other_window_up(own: HWND) -> bool {
        let mut search = Search {
            pid: std::process::id(),
            own,
            found: false,
        };
        // SAFETY: `search` outlives the call, and `visit` only reads it as a `Search`.
        unsafe { EnumWindows(Some(visit), &mut search as *mut Search as LPARAM) };
        search.found
    }

    pub(super) fn run(painter: Box<dyn Painter>) {
        // SAFETY: plain Win32 calls on this thread's own window; every
        // pointer handed over lives for the duration of the call.
        unsafe {
            SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
            let scale = GetDpiForSystem() as f32 / 96.0;
            let (w, h) = painter.size();
            let (width, height) = ((w * scale) as i32, (h * scale) as i32);
            let class = wide("CrawradSplash");
            let wc = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(proc),
                lpszClassName: class.as_ptr(),
                ..std::mem::zeroed()
            };
            RegisterClassW(&wc);
            let picture = painter.picture(width as usize, height as usize);
            CONTEXT.with(|c| {
                *c.borrow_mut() = Some(Context {
                    painter,
                    frame: Frame::default(),
                    picture,
                    width,
                    height,
                    scale,
                })
            });
            let x = (GetSystemMetrics(SM_CXSCREEN) - width) / 2;
            let y = (GetSystemMetrics(SM_CYSCREEN) - height) / 2;
            let title = wide("Splash");
            let hwnd = CreateWindowExW(
                WS_EX_TOOLWINDOW,
                class.as_ptr(),
                title.as_ptr(),
                WS_POPUP | WS_VISIBLE,
                x,
                y,
                width,
                height,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null(),
            );
            if hwnd.is_null() {
                return;
            }
            SetTimer(hwnd, 1, TICK_MS, None);
            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
            CONTEXT.with(|c| c.borrow_mut().take());
        }
    }

    unsafe extern "system" fn proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        // SAFETY: called by Windows on the splash thread, with a valid `hwnd`.
        unsafe {
            match msg {
                WM_TIMER => {
                    let up = other_window_up(hwnd);
                    let keep = CONTEXT.with(|c| {
                        let mut c = c.borrow_mut();
                        let Some(c) = c.as_mut() else { return false };
                        match c.painter.frame(up) {
                            Some(frame) => {
                                c.frame = frame;
                                true
                            }
                            None => false,
                        }
                    });
                    if keep {
                        InvalidateRect(hwnd, std::ptr::null(), 0);
                    } else {
                        KillTimer(hwnd, 1);
                        DestroyWindow(hwnd);
                    }
                    0
                }
                WM_PAINT => {
                    CONTEXT.with(|c| {
                        if let Some(c) = c.borrow().as_ref() {
                            paint(hwnd, c);
                        }
                    });
                    0
                }
                WM_DESTROY => {
                    PostQuitMessage(0);
                    0
                }
                _ => DefWindowProcW(hwnd, msg, wparam, lparam),
            }
        }
    }

    unsafe fn paint(hwnd: HWND, c: &Context) {
        // SAFETY: GDI objects are created, selected and deleted here, inside
        // one BeginPaint/EndPaint pair; the picture holds width × height pixels.
        unsafe {
            let px = |v: f32| (v * c.scale) as i32;
            let rect = |a: [f32; 4]| RECT {
                left: px(a[0]),
                top: px(a[1]),
                right: px(a[2]),
                bottom: px(a[3]),
            };
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let screen = BeginPaint(hwnd, &mut ps);
            let dc = CreateCompatibleDC(screen);
            let bitmap = CreateCompatibleBitmap(screen, c.width, c.height);
            let old_bitmap = SelectObject(dc, bitmap);
            let mut info: BITMAPINFO = std::mem::zeroed();
            info.bmiHeader = BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: c.width,
                biHeight: -c.height,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB,
                ..std::mem::zeroed()
            };
            SetDIBitsToDevice(
                dc,
                0,
                0,
                c.width as u32,
                c.height as u32,
                0,
                0,
                0,
                c.height as u32,
                c.picture.as_ptr().cast(),
                &info,
                DIB_RGB_COLORS,
            );
            for fill in &c.frame.fills {
                let brush = CreateSolidBrush(colorref(fill.color));
                FillRect(dc, &rect(fill.area), brush);
                DeleteObject(brush);
            }
            SetBkMode(dc, TRANSPARENT as i32);
            let face = wide("Segoe UI");
            for text in &c.frame.texts {
                let font = CreateFontW(
                    -px(text.size),
                    0,
                    0,
                    0,
                    text.weight,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    5, // CLEARTYPE_QUALITY
                    0,
                    face.as_ptr(),
                );
                let old_font = SelectObject(dc, font);
                SetTextColor(dc, colorref(text.color));
                let w: Vec<u16> = text.text.encode_utf16().collect();
                let mut r = rect(text.area);
                let align = if text.align_right { DT_RIGHT } else { DT_LEFT };
                DrawTextW(
                    dc,
                    w.as_ptr(),
                    w.len() as i32,
                    &mut r,
                    align | DT_VCENTER | DT_SINGLELINE | DT_NOPREFIX,
                );
                SelectObject(dc, old_font);
                DeleteObject(font);
            }
            BitBlt(screen, 0, 0, c.width, c.height, dc, 0, 0, SRCCOPY);
            SelectObject(dc, old_bitmap);
            DeleteObject(bitmap);
            DeleteDC(dc);
            EndPaint(hwnd, &ps);
        }
    }
}
