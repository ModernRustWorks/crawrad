//! **This file is yours.** CrawRAD does not regenerate it, and it
//! never removes code you wrote.
//!
//! It does write to the file, though — better you know than be
//! surprised:
//!
//! - wiring a new event in the Object Inspector inserts the
//!   handler at the marker below, leaving the rest alone;
//! - generating takes empty event handlers back out, and
//!   saving generates — so saving takes them out too. It does
//!   not matter whether the event is still wired: the wiring
//!   is dropped in the same step, so the form still compiles.
//!   This only ever reaches a handler that is still the empty
//!   one CrawRAD wrote, under the very name CrawRAD gave it.
//!   Type one character into the body, even a comment; rename
//!   the function; or write a line of your own right above it,
//!   such as a doc comment or an attribute — any of these and
//!   it is yours: it stays for good.
//!   Methods you wrote yourself are never looked at, whatever
//!   you name them and whatever you write above them;
//! - renaming the form moves this file along with it and
//!   replaces the form's type name throughout — in your code
//!   AND in your comments — because otherwise it would not
//!   compile.

// One glob, on purpose: it brings in the form's type, which
// the `impl` below is written on, and the event payload types
// (`KeyEvent`, `MouseEvent`) that a handler signature names
// with no path.
use crate::form_main_designer::*;

impl FormMain {
    // New event handlers go here.
}
