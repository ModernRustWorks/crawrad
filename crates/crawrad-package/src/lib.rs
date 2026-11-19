//! # crawrad-package — **where a component COMES FROM**
//!
//! Three crates answer three different questions about a component, and
//! none answers the neighbor's:
//!
//! | crate | the question |
//! |---|---|
//! | `crawrad-registry` | **what a component IS** — properties, events, palette |
//! | **`crawrad-package`** | **where it COMES FROM** — the manifest, validation, installation |
//! | `crawrad-component` | **what the AUTHOR writes** — the trait and the macro (does not yet exist) |
//!
//! That is why the file reader does not live in the catalogue: the job of
//! `crawrad-registry` is to say what a `Button` is, and it has no reason to know
//! what a disk is. And that is why the author SDK does not live here: whoever
//! writes a component does not need the manifest reader — they **write** the
//! file, the IDE is the one that **reads** it.
//!
//! # THE STANDARD, and it applies to every component from here on
//!
//! [ADR 0033] decides that a component is a crates.io crate and that the IDE
//! downloads the source, compiles and installs it. For the IDE to be able to
//! open and compile any component, all must have the same shape:
//!
//! ```text
//! my_component/
//! ├── Cargo.toml          crate-type = ["cdylib", "rlib"]
//! ├── manifest.json        ← THE MANIFEST: what the IDE reads BEFORE compiling
//! └── src/
//!     ├── lib.rs          the `register` — and only it
//!     ├── descriptor.rs   what it IS
//!     ├── design.rs       how it DRAWS itself in the designer
//!     ├── codegen.rs      what it WRITES into the generated app
//!     └── runtime.rs      what it DOES in the user's app
//! ```
//!
//! **One file per responsibility**, and `lib.rs` is just the door. It is the
//! same shape that this codebase has already proven twice on the inside — the
//! `DESENHOS` of the Makepad adapter and `desenho_comum.rs` of egui —, now on
//! the outside.
//!
//! # Why the manifest comes BEFORE compilation
//!
//! The IDE needs to know what it is going to install before spending a build:
//!
//! - **the name and version of the SDK**, to refuse early what it does not speak;
//! - **the classes that the package brings**, to refuse one that is already taken;
//! - **the properties**, to refuse a key that breaks the contract of
//!   ADR 0002 — which, if accepted, poisons every form that uses the component.
//!
//! Refusing after the build is making the user wait just to hear no.
//!

// This crate has no `unsafe`, and the compiler now guarantees this.
// Library loading — which requires it — lives in `crawrad-os`, the FFI
// adapter (ADR 0009 and ADR 0033 §5).
#![forbid(unsafe_code)]

pub mod manifest;

pub use manifest::{Manifest, ManifestError, PackageId};

#[cfg(test)]
#[path = "test/manifest/test.rs"]
mod test;
