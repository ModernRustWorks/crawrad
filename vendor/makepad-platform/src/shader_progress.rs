//! CrawRAD patch: how far the shader preparation of the first frames
//! has gone, for a splash window on another thread to show.
//!
//! The Windows backend counts here as it goes (`hlsl_compile_shaders`); the
//! other backends leave it at zero, which reads as "nothing to wait for".

use std::sync::atomic::{AtomicUsize, Ordering};

static TOTAL: AtomicUsize = AtomicUsize::new(0);
static DONE: AtomicUsize = AtomicUsize::new(0);
static COMPILED: AtomicUsize = AtomicUsize::new(0);

/// Shaders ready, of how many asked for so far, and how many of the ready
/// ones had to be compiled rather than read from the disk cache.
pub fn progress() -> (usize, usize, usize) {
    (DONE.load(Ordering::Relaxed), TOTAL.load(Ordering::Relaxed), COMPILED.load(Ordering::Relaxed))
}

pub(crate) fn asked(count: usize) {
    TOTAL.fetch_add(count, Ordering::Relaxed);
}

pub(crate) fn ready() {
    DONE.fetch_add(1, Ordering::Relaxed);
}

pub(crate) fn compiled() {
    COMPILED.fetch_add(1, Ordering::Relaxed);
}
