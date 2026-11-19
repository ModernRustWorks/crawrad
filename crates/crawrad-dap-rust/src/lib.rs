//! The Rust [`ValuePresenter`] of CrawRAD (ADR 0048 §3.2).
//!
//! `lldb-dap` without Python shows a Rust value as its bare type name
//! (`alloc::string::String`) — but it still hands over the raw tree, `evaluate`
//! and `readMemory`. This crate rebuilds the value from those, in Rust, the way
//! `{:?}` prints it: `"Olá"`, `[10, 20, 30]`, `Some(7)`, `Circle { radius: 2.0 }`.
//!
//! **What it does not know, it does not guess**: [`ValuePresenter::present`]
//! returns `None` and the screen shows what the adapter sent. The shapes it
//! reads are the ones measured on 30/09/2026 (`-gnu`, DWARF); the `std`
//! internals it walks are listed in [`layout`] with their older spellings, so a
//! layout change in a new Rust shows raw instead of wrong.
#![forbid(unsafe_code)]

pub mod dwarf;
pub mod layout;
pub mod memory;
pub mod pdb;
mod render;
pub mod types;

use crawrad_dap::{ValuePresenter, ValueSource, Variable};
use crawrad_note::note;

#[note("max-depth")]
pub const MAX_DEPTH: usize = 3;

#[note("max-elements")]
pub const MAX_ELEMENTS: usize = 64;

#[note("max-string-bytes")]
pub const MAX_STRING_BYTES: usize = 4096;

#[note("load-layouts")]
pub fn load_layouts(program: &std::path::Path) -> Option<dwarf::TypeIndex> {
    dwarf::TypeIndex::load(program).or_else(|| pdb::load(&program.with_extension("pdb")))
}

#[note("rustpresenter")]
#[derive(Debug, Default, Clone)]
pub struct RustPresenter {
    index: Option<std::sync::Arc<dwarf::TypeIndex>>,
}

impl RustPresenter {
    #[note("rustpresenter-new")]
    pub fn new() -> Self {
        Self::default()
    }

    #[note("rustpresenter-for-program")]
    pub fn for_program(program: &std::path::Path) -> Self {
        Self {
            index: load_layouts(program).map(std::sync::Arc::new),
        }
    }

    pub fn has_layouts(&self) -> bool {
        self.index.is_some()
    }
}

impl ValuePresenter for RustPresenter {
    fn present(&self, variable: &Variable, source: &dyn ValueSource) -> Option<String> {
        let cx = render::Cx {
            src: source,
            index: self.index.as_deref(),
        };
        render::present(variable, &cx, MAX_DEPTH)
    }
}

#[cfg(test)]
#[path = "test/memory.rs"]
mod memory_tests;
#[cfg(test)]
#[path = "test/presenter.rs"]
mod presenter_tests;
