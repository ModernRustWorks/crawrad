//! # The system's shell, for the generated application
//!
//! What a control hands to the operating system instead of doing itself: a
//! `LinkLabel`'s URL opens in the default browser.

/// The schemes a link may open. Anything else — a file path, a `javascript:`
/// — is refused: the system's URL handler would run it.
const SCHEMES: [&str; 3] = ["http://", "https://", "mailto:"];

/// Whether `url` is one a link may open.
pub fn openable(url: &str) -> bool {
    let lower = url.trim().to_ascii_lowercase();
    SCHEMES.iter().any(|s| lower.starts_with(s)) && !url.chars().any(char::is_control)
}

/// Opens `url` in the system's default browser (or mail client), without a
/// console window. Fire and forget; a URL [`openable`] refuses opens nothing
/// and answers `false`.
pub fn open_url(url: &str) -> bool {
    if !openable(url) {
        return false;
    }
    let url = url.trim();
    // the URL goes as ONE argument, never through `cmd`, where `&` and `|`
    // would be read as commands
    #[cfg(target_os = "windows")]
    let spawned = {
        use std::os::windows::process::CommandExt;
        std::process::Command::new("rundll32")
            .args(["url.dll,FileProtocolHandler", url])
            .creation_flags(0x0800_0000)
            .spawn()
    };
    #[cfg(target_os = "macos")]
    let spawned = std::process::Command::new("open").arg(url).spawn();
    #[cfg(all(unix, not(target_os = "macos")))]
    let spawned = std::process::Command::new("xdg-open").arg(url).spawn();
    #[cfg(not(any(unix, target_os = "windows")))]
    let spawned: std::io::Result<()> = Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        url.to_string(),
    ));
    spawned.is_ok()
}
