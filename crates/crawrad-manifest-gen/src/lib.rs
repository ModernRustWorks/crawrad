//! # `crawrad-manifest-gen`: **the `struct` becomes a manifest**
//!
//! Reads a Rust `struct` marked with `#[derive(Component)]` and writes the
//! object that `manifest.json` holds. It is the translator of the classic RAD
//! `published` section into Rust's idiom: see [ADR 0039].
//!
//! ## Why it is a separate crate
//!
//! Because **two things** do exactly this, from the same source:
//!
//! | who | when | what it adds |
//! |---|---|---|
//! | `#[derive(Component)]` | on the component author's `cargo build` | **proves** that `Default::default()` matches the declared defaults |
//! | `crawrad manifest` | on request, without compiling anything | **writes** the file, and it is what `New Project ▸ Package` will call |
//!
//! While this code lived inside the proc-macro crate, the second one was
//! **impossible**: Cargo does not let a proc-macro be an ordinary dependency of
//! a binary. The extraction is what unlocks the CLI, not a drawer tidy-up.
//!
//! ## `syn` reads text as well as it reads tokens
//!
//! That is what lets the CLI exist without compiling anything: [`from_source`]
//! takes the contents of a `.rs` and returns the components it declares. The
//! `derive` takes the same kind of tree, coming from the compiler instead of
//! from the disk.
//!

// No `unsafe`, and the compiler guarantees it, as in every crate of this house.
#![forbid(unsafe_code)]

mod canvas_ast;
mod render;
mod spec;

#[cfg(test)]
#[path = "test/render/test.rs"]
mod render_test;

#[cfg(test)]
#[path = "test/refusal/test.rs"]
mod refusal_test;

#[cfg(test)]
#[path = "test/from_source/test.rs"]
mod from_source_test;

pub use spec::{
    ComponentSpec, DrawSpec, EventSpec, IconSpec, ItemSpec, PropSpec, RequirementSpec, Size,
};

use std::collections::HashMap;
use syn::{DeriveInput, Item};

/// **The components a `.rs` file declares**, read from the text.
///
/// Finds every `struct` with `#[derive(Component)]` (including inside a nested
/// `mod`) and reads each one. The order is the file's, and it matters: it is
/// the order in which the components appear in the manifest and, in turn, in
/// the palette.
///
/// Returns `Ok(vec![])` when there is none, and that is **not an error**: a
/// package spreads its components over several files, and most of them have
/// none.
///
/// # Examples
///
/// ```rust
/// use crawrad_manifest_gen::from_source;
///
/// let components = from_source("struct Plain;").unwrap();
/// assert!(components.is_empty());
/// ```
///
/// # Errors
///
/// Returns a [`syn::Error`] if the source text fails to parse as valid Rust code or has invalid attributes.
pub fn from_source(source: &str) -> syn::Result<Vec<ComponentSpec>> {
    let file: syn::File = syn::parse_str(source)?;
    let mut found = Vec::new();
    let mut paints = HashMap::new();
    collect_components(&file.items, &mut found, &mut paints)?;
    for spec in &mut found {
        if spec.draws.is_empty()
            && let Some(draws) = paints.remove(&spec.class)
        {
            spec.draws = draws;
        }
    }
    Ok(found)
}

fn collect_components(
    items: &[Item],
    out: &mut Vec<ComponentSpec>,
    paints: &mut HashMap<String, Vec<DrawSpec>>,
) -> syn::Result<()> {
    for item in items {
        match item {
            Item::Struct(s) if derives_component(&s.attrs) => {
                let input = DeriveInput::from(s.clone());
                out.push(ComponentSpec::read(&input)?);
            }
            Item::Impl(item_impl) => {
                if let Some((target_class, draws)) = canvas_ast::parse_paint_impl(item_impl)? {
                    paints.insert(target_class, draws);
                }
            }
            // A component can live inside a `mod`, and a large package will
            // want that. Stopping at the root would make the CLI and the macro
            // disagree, and the macro does not stop, because the compiler calls
            // it wherever the `struct` is.
            Item::Mod(m) => {
                if let Some((_, inner)) = &m.content {
                    collect_components(inner, out, paints)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

/// Checks whether the struct asks for `Component` in its derive list.
///
/// Does the `struct` ask for `Component` in its `derive` list?
///
/// Looks at the **last segment** of the path, so
/// `#[derive(crawrad_component_derive::Component)]` counts too: that is how
/// `rustc` resolves it, and a CLI that answered differently from the compiler
/// would be a second truth.
fn derives_component(attrs: &[syn::Attribute]) -> bool {
    attrs
        .iter()
        .filter(|a| a.path().is_ident("derive"))
        .any(|a| {
            let mut found = false;
            let _ = a.parse_nested_meta(|meta| {
                if meta
                    .path
                    .segments
                    .last()
                    .is_some_and(|s| s.ident == "Component")
                {
                    found = true;
                }
                Ok(())
            });
            found
        })
}
