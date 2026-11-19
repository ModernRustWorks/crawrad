//! The IDE's link opener: [`crate::shell::open_url`] plus the status line it reports.
//! Kept out of `shell.rs`, which is copied verbatim into generated apps.

use crate::shell::open_url;
use crawrad_note::note;

#[note("link-open-and-say")]
pub fn open_and_say(url: &str) -> String {
    if open_url(url) {
        format!("Opening {url}")
    } else {
        format!("Not opened: {url} — only http, https and mailto links open")
    }
}
