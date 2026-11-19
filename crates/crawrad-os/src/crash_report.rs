//! What the IDE says when it **stops**: the report written on a panic.
//!
//! # Why it exists
//!
//! The release build has no console (`windows_subsystem = "windows"` in
//! `main.rs`): whoever gets the finished program does not want a black
//! rectangle behind the window. The consequence is that, without a console, a
//! panic **vanishes without a word**: the default hook prints to a stderr that
//! does not exist, and the process exits. The person double-clicks and nothing
//! happens, and nobody has any record of what went wrong.
//!
//! With Makepad a failure to create the window is a panic too (`app_main!`
//! returns nothing), so the same path covers "it did not open" and "it stopped
//! halfway".
//!
//! This logic came from the egui shell's `falha_inicial.rs`, which paid for
//! the same lesson, and it keeps its tests.
//!
//! # The file first, the box after
//!
//! The order is not taste. The box **is the path that may not happen**: the
//! graphics session may be exactly what is broken. So the report goes to disk
//! **first**, and only then does the box open, saying where it is. If the box
//! does not show, the information is already written; if it shows, it points at
//! the same text.
//!
//! The box is Windows' own `MessageBoxW`, which is why this lives in the FFI
//! crate. Elsewhere there is no box: the report is the file plus stderr (the
//! egui shell used `rfd`, which on Linux depends on a portal or `zenity`; the
//! Makepad shell had no box at all until now).
//!
//! # Where the file lives
//!
//! In the user's configuration folder (`%APPDATA%\CrawRAD` on Windows,
//! `~/.config/crawrad` elsewhere), the one the IDE already uses for global
//! state. Next to the executable would be easier to find, but **that folder may
//! not accept writes**: an IDE installed in `C:\Program Files` writes zero bytes
//! there. The box shows the full path, which answers "hard to find". When not
//! even that folder exists, the last try is the temporary directory.
//!
//! The file is **rewritten** on each failure, not appended: whoever asks for
//! help is talking about the failure of now.

use crawrad_note::note;
use std::path::{Path, PathBuf};

#[note("crash-report-file-name")]
pub const FILE_NAME: &str = "crash-report.log";

#[note("crash-report-title")]
pub const TITLE: &str = "CrawRAD stopped";

#[note("crash-report-box-timeout")]
const BOX_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

#[note("crash-report-report")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// The text for the box and the file.
    pub message: String,
    /// Where the report went. `None` when not even writing was possible.
    pub file: Option<PathBuf>,
    /// The process exit code. Never zero: a script that launches the IDE
    /// needs to know it failed.
    pub code: u8,
}

#[note("crash-report-prepare")]
pub fn prepare(error: &str, details: &str, folder: Option<&Path>) -> Report {
    let file = folder.and_then(|p| write(p, error, details));
    Report {
        message: message(error, file.as_deref()),
        file,
        code: 1,
    }
}

#[note("crash-report-report-folder")]
pub fn report_folder(configuration: Option<PathBuf>) -> Option<PathBuf> {
    choose_folder(configuration)
}

#[note("crash-report-choose-folder")]
fn choose_folder(configuration: Option<PathBuf>) -> Option<PathBuf> {
    configuration.or_else(|| {
        let temp = std::env::temp_dir();
        temp.is_dir().then_some(temp)
    })
}

#[note("crash-report-write")]
fn write(folder: &Path, error: &str, details: &str) -> Option<PathBuf> {
    let _ = std::fs::create_dir_all(folder);
    let file = folder.join(FILE_NAME);
    std::fs::write(&file, file_contents(error, details))
        .ok()
        .map(|()| file)
}

#[note("crash-report-file-contents")]
fn file_contents(error: &str, details: &str) -> String {
    let when = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    let executable = std::env::current_exe()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    format!(
        "{TITLE}\n\
         \n\
         {EXPLANATION}\n\
         \n\
         Unix time: {when}\n\
         Version: {}\n\
         Target: {} {}\n\
         Executable: {executable}\n\
         \n\
         Technical details: {error}\n\
         \n\
         {details}\n",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH,
    )
}

#[note("crash-report-explanation")]
const EXPLANATION: &str = "CrawRAD hit an internal error and had to stop.\n\
     \n\
     If it happened while opening, the graphics session may not be able to \
     give the program a window: a remote session without hardware acceleration, \
     a machine with no usable graphics adapter, or an outdated display driver.\n\
     \n\
     What to do: open CrawRAD again. If it keeps happening, send the details \
     below to whoever supports this installation.";

#[note("crash-report-message")]
fn message(error: &str, file: Option<&Path>) -> String {
    let where_ = match file {
        Some(a) => format!("These details were also written to:\n{}", a.display()),
        None => "These details could not be written to a file: no writable \
                 folder was found. Copy this message before closing it."
            .to_string(),
    };
    format!("{EXPLANATION}\n\n{where_}\n\nTechnical details: {error}")
}

#[note("crash-report-report-2")]
pub fn report(error: &str, details: &str, configuration: Option<PathBuf>, show_box: bool) -> u8 {
    let report = prepare(error, details, report_folder(configuration).as_deref());
    // stderr stays: `windows_subsystem = "windows"` does not close it, it just
    // does not ALLOCATE a console. Whoever launches the IDE with stderr
    // redirected (a script, CI, a terminal) still gets this text.
    eprintln!("{}\n\n{details}", report.message);
    if show_box {
        show_message_box(&report.message);
    }
    report.code
}

#[note("crash-report-show-message-box")]
fn show_message_box(message: &str) {
    let (tx, rx) = std::sync::mpsc::channel();
    let text = message.to_string();
    std::thread::spawn(move || {
        native_box(&text);
        let _ = tx.send(());
    });
    let _ = rx.recv_timeout(BOX_TIMEOUT);
}

#[note("crash-report-native-box")]
#[cfg(target_os = "windows")]
fn native_box(text: &str) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW};
    let wide = |s: &str| {
        s.encode_utf16()
            .chain(std::iter::once(0))
            .collect::<Vec<u16>>()
    };
    let text = wide(text);
    let title = wide(TITLE);
    // SAFETY: both pointers come from `Vec<u16>` buffers that are NUL-terminated
    // and alive for the whole call; a null owner window is allowed.
    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            text.as_ptr(),
            title.as_ptr(),
            MB_OK | MB_ICONERROR,
        );
    }
}

#[note("crash-report-native-box-2")]
#[cfg(not(target_os = "windows"))]
fn native_box(_text: &str) {}

#[cfg(test)]
#[path = "test/crash_report/test.rs"]
mod tests;
