//! # `crawrad-component-derive` — **the manifest stops being written by hand**
//!
//! `#[derive(Component)]` reads a struct and emits the object that
//! `manifest.json` holds. It is the `published` section of a classic RAD language, said in Rust.
//!
//! ## Why it exists — the owner's correction, on 18/09/2026
//!
//! [ADR 0038] quoted him saying that the box a classic RAD programmer builds a
//! component on "is the source of the BPL". Hours later he took it back:
//!
//! > *"I lied: the BPL is the metadata, I cited it wrong."*
//!
//! That flips the parallel, and the flip is the whole reason for this crate:
//!
//! | in a classic RAD IDE | what it is | here |
//! |---|---|---|
//! | the runtime and component-library source you build on | the box: ready-made types | `crawrad-component` |
//! | the **design-time BPL** the IDE loads | **the metadata** the Object Inspector reads | `manifest.json` |
//!
//! And **nobody writes that metadata by hand in a classic RAD IDE.** There is no text file
//! beside the class describing what the class has: the programmer writes
//! `published`, and the *compiler* emits the RTTI. We were writing
//! `manifest.json` next to a `struct` that already said the same things.
//!
//! ## It had already gone wrong, in one day
//!
//! Measured on this crate's first day, on the very first component of the
//! suite: `Gauge` the struct had `visible` and `enabled`; `Gauge` the manifest
//! declared neither. The doc comment on the struct claimed both halves read
//! "the same properties" and listed those two by name. Two files, one truth,
//! and they disagreed within twenty-four hours of being written.
//!
//! Nothing caught it, and nothing could have: they were prose and TOML, and no
//! compiler reads either one against the other.
//!
//! ## What it does NOT do, and why that is deliberate
//!
//! It does not move the drawing verbs into code. They stay declarative, because
//! they are not metadata: they are what stands in for `Paint`, and the IDE
//! refuses to run a third party's paint routine ([ADR 0036]). The verbs are
//! attributes here for one reason only — so that one file is the source of the
//! whole manifest instead of two.
//!
//! ## The shape
//!
//! ```text
//! #[derive(Component, Debug, Clone, PartialEq)]
//! #[crawrad(palette = "Additional", prefix = "gau", size = (140.0, 48.0))]
//! #[event(name = "on_click")]
//! #[draw(verb = "box", fill = "surface", border = "border", radius = 4.0)]
//! pub struct Gauge {
//!     /// The caption, written over the bar.
//!     #[prop(display = "Caption", category = "appearance", default = "Gauge")]
//!     pub text: String,
//! }
//! ```
//!
//! `class`, `name` and `rect` are not written: the class is the struct's name,
//! and every component has the other two.
//!
//! The expansion is a `CRAWRAD_COMPONENT_JSON` constant and a test that asserts
//! `Default::default()` agrees with every declared default. A macro cannot
//! *read* `Default`, but it can *prove* the two match — and that closes the
//! last gap where the struct and the manifest could still drift apart.
//!
//! `crawrad_component::manifest!` turns those constants into the file on disk.
//!

// No `unsafe`, and the compiler guarantees it -- as in every crate of this house.
#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input};

// **Reading and writing live OUTSIDE**, in `crawrad-manifest-gen`.
//
// Not for elegance: Cargo does not let a proc-macro be an ordinary dependency
// of a binary, so while they lived here `crawrad manifest` was impossible.
// This crate kept only what a proc-macro alone can do: take the compiler's
// tree and give code back.
use crawrad_manifest_gen::{ComponentSpec, ItemSpec};

/// Emit a component's manifest block from its struct.
///
/// See the crate documentation for the shape and the reasoning.
///
/// # Examples
///
/// ```rust
/// use crawrad_component_derive::Component;
///
/// #[derive(Default, Component)]
/// #[crawrad(palette = "Standard", prefix = "btn", size = (80.0, 24.0))]
/// struct SampleButton {
///     #[prop(category = "appearance", default = "")]
///     caption: String,
/// }
///
/// assert!(SampleButton::CRAWRAD_COMPONENT_JSON.contains("\"caption\""));
/// ```
#[proc_macro_derive(Component, attributes(crawrad, prop, event, draw, icon, requires))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Emit a component item's manifest block from its struct.
///
/// # Examples
///
/// ```rust
/// use crawrad_component_derive::ComponentItem;
///
/// #[derive(Default, ComponentItem)]
/// struct SampleItem {
///     #[prop(category = "appearance", default = "")]
///     label: String,
/// }
///
/// assert!(SampleItem::CRAWRAD_ITEM_JSON.contains("\"label\""));
/// ```
#[proc_macro_derive(ComponentItem, attributes(prop))]
pub fn derive_component_item(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_item(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_item(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let spec = ItemSpec::read(input)?;
    let manifest = spec.to_json(4);
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let checks = spec.props.iter().filter_map(|prop| {
        let field = prop.field.as_ref()?;
        let default = prop.default_expr.as_ref()?;
        let key = prop.key.as_str();
        Some(quote! {
            ::core::assert_eq!(
                made.#field, #default,
                "the item manifest starts `{}` somewhere `Default` does not",
                #key,
            );
        })
    });

    let module = format_ident!(
        "__crawrad_manifest_of_item_{}",
        ident.to_string().to_lowercase(),
        span = ident.span()
    );

    Ok(quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            /// This item's descriptor JSON.
            pub const CRAWRAD_ITEM_JSON: &'static str = #manifest;
        }

        #[cfg(test)]
        mod #module {
            #[test]
            fn the_item_defaults_are_the_rust_defaults() {
                let made = <super::#ident as ::core::default::Default>::default();
                #(#checks)*
            }
        }
    })
}

fn expand(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let spec = ComponentSpec::read(input)?;
    // Indent 4: the block is written inside the file's `components` array,
    // which `crawrad_component::manifest!` opens at two levels.
    let manifest = spec.to_json(4);
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // **The defaults are declared twice, and this is what keeps them honest.**
    //
    // A proc-macro runs before anything exists, so it cannot ask
    // `Default::default()` what the field starts at -- it only sees the literal
    // the author wrote in the attribute. What it CAN do is emit the comparison
    // and let the test suite run it.
    let checks = spec.props.iter().filter_map(|prop| {
        let field = prop.field.as_ref()?;
        let default = prop.default_expr.as_ref()?;
        let key = prop.key.as_str();
        Some(quote! {
            ::core::assert_eq!(
                made.#field, #default,
                "the manifest starts `{}` somewhere `Default` does not",
                #key,
            );
        })
    });

    let module = format_ident!(
        "__crawrad_manifest_of_{}",
        ident.to_string().to_lowercase(),
        span = ident.span()
    );

    Ok(quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            /// This component's object, exactly as `manifest.json` holds it.
            ///
            /// Emitted by `#[derive(Component)]`. `crawrad_component::manifest!`
            /// assembles these into the file and proves the file matches.
            pub const CRAWRAD_COMPONENT_JSON: &'static str = #manifest;
        }

        #[cfg(test)]
        mod #module {
            /// Every default in the manifest is the default the type really has.
            ///
            /// Without this the two could drift the moment somebody edits
            /// `Default` and forgets the attribute -- which is the exact drift
            /// that made this macro necessary.
            #[test]
            fn the_manifest_defaults_are_the_rust_defaults() {
                let made = <super::#ident as ::core::default::Default>::default();
                #(#checks)*
            }
        }
    })
}
