//! The kinds of row the Explorer draws — **one file each**.
//!
//! That is the whole rule of this folder, and it is the one the house asks
//! for: adding a kind of row costs *creating a file*, not editing a `match`
//! that lives somewhere else. Each file below owns everything about its own
//! row — the template, the indent, the drawing, the badge, the sentence behind
//! the badge, and what a click on it runs — and nothing outside it has to
//! learn the new kind exists.
//!
//! The one place that still has to hear about a new row is
//! `super::sample_rows`, and that is deliberate rather than forgotten: the DSL
//! gate covers **exactly** what the probe instantiates, so a row nobody
//! samples is a template nobody checks — and an unchecked template is one that
//! draws nothing at all, in silence.

// The `#[path]` attributes are for the same reason as in the file above: a module
// loaded via `#[path]` looks for child modules in the file's own directory, not in
// a directory with the module's name.
use crawrad_note::note;
#[path = "rows/caption.rs"]
pub mod caption;
#[path = "rows/file.rs"]
pub mod file;
#[path = "rows/form.rs"]
pub mod form;
#[path = "rows/group.rs"]
pub mod group;
#[path = "rows/legend.rs"]
pub mod legend;
#[path = "rows/member.rs"]
pub mod member;
#[path = "rows/note.rs"]
pub mod note;
#[path = "rows/section.rs"]
pub mod section;
#[path = "rows/summary.rs"]
pub mod summary;
#[path = "rows/timeline_entry.rs"]
pub mod timeline_entry;

#[note("explorer-rows-rows-child")]
pub const CHILD: usize = 1;
