//! Where the `std` keeps a buffer's pointer and length.
//!
//! These are internals of the standard library, not a stable interface, and
//! they have moved before (`RawVecInner` arrived to erase the element type).
//! Each list goes from the current spelling to the older ones; the presenter
//! tries them in order and, if none resolves, shows the value raw — never a
//! number read from the wrong place.

use crawrad_note::note;
#[note("layout-vec-len")]
pub const VEC_LEN: &[&str] = &["{}.len"];

#[note("layout-vec-ptr")]
pub const VEC_PTR: &[&str] = &[
    // measured on 30/09/2026 (Rust 1.95 / 1.98, -gnu)
    "{}.buf.inner.ptr.pointer.pointer",
    // before `RawVecInner`
    "{}.buf.ptr.pointer.pointer",
    "{}.buf.ptr.pointer",
];

#[note("layout-string-vec")]
pub const STRING_VEC: &str = "{}.vec";

#[note("layout-at")]
pub fn at(path: &str, base: &str) -> String {
    path.replacen("{}", base, 1)
}

#[note("layout-hash-len")]
pub const HASH_LEN: &[&str] = &[
    // measured on 30/09/2026 (Rust 1.95 / 1.98, -gnu)
    "{}.base.table.table.items",
    // a HashSet wraps a HashMap
    "{}.base.map.table.table.items",
];

#[note("layout-hash-ctrl")]
pub const HASH_CTRL: &[&str] = &[
    // measured on 30/09/2026 (Rust 1.95, -gnu)
    "{}.base.table.table.ctrl.pointer",
    "{}.base.map.table.table.ctrl.pointer",
];

#[note("layout-hash-bucket-mask")]
pub const HASH_BUCKET_MASK: &[&str] = &[
    // measured on 30/09/2026 (Rust 1.95, -gnu)
    "{}.base.table.table.bucket_mask",
    "{}.base.map.table.table.bucket_mask",
];
