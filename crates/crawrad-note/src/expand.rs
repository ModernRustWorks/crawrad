//! The expansion of `#[note("id")]`, on `proc_macro2` so it can be tested.
//!
//! Cargo only knows a file belongs to the build when the compiler reads it, so
//! the expansion always plants an `include_str!` of the note somewhere legal:
//! as the first statement of a function body, inside the expression of a
//! `const`/`static` (legal at module level AND inside an impl or trait), or as
//! a sibling `const _` for items that only ever live at module level. Items
//! with no body and no expression (a bodiless trait method, an associated or
//! module `type`) get the doc without tracking: their note is still read on
//! every build that recompiles the crate for another reason.
//!
//! The doc carries the id literal's span, so rustdoc lints (a broken
//! intra-doc link) point at the `#[note]` line instead of being silenced as
//! coming from an external macro.
//!
//! A note saved with the item's `signature` is checked against the item
//! when it changed, the tracker also uses a `#[deprecated]` const, so
//! the build WARNS at the attribute — stable Rust has no other way for a
//! macro to warn — and a `-D warnings` build fails. Items without a tracker
//! are not checked.

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Block, Expr, ImplItem, Item, LitStr, Stmt, TraitItem};

use crate::parse;

struct NoteArg {
    id: LitStr,
}

impl Parse for NoteArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let id: LitStr = input.parse()?;
        if !input.is_empty() {
            return Err(input.error("expected exactly one string literal argument"));
        }
        Ok(NoteArg { id })
    }
}

pub(crate) fn expand(attr: TokenStream, item: TokenStream, manifest_dir: &str) -> TokenStream {
    match try_expand(attr, item, manifest_dir) {
        Ok(tokens) => tokens,
        Err(err) => err.to_compile_error(),
    }
}

fn try_expand(
    attr: TokenStream,
    item: TokenStream,
    manifest_dir: &str,
) -> syn::Result<TokenStream> {
    let arg: NoteArg = syn::parse2(attr)?;
    let id = arg.id.value();
    let span = arg.id.span();
    let fail = |msg: String| syn::Error::new(span, msg);

    parse::validate_id(&id).map_err(fail)?;
    if manifest_dir.is_empty() {
        return Err(fail("CARGO_MANIFEST_DIR is not set".to_string()));
    }
    let path = parse::note_path(manifest_dir, &id);
    let text = std::fs::read_to_string(&path).map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            fail(parse::missing_file_error(&id, &path))
        } else {
            fail(format!(
                "failed to read note \"{id}\" at {}: {err}",
                path.display()
            ))
        }
    })?;
    let note = parse::parse_note(&id, &text).map_err(fail)?;
    let body = note.body;
    let stale = crawrad_note_format::stale_signature(
        &id,
        &note.signature,
        &crawrad_note_format::item_signature(&item),
    );
    let stale_warn = stale.map(|msg| {
        quote_spanned! {span=>
            #[deprecated(note = #msg)]
            const NOTE_SIGNATURE_CHANGED: () = ();
            let _ = NOTE_SIGNATURE_CHANGED;
        }
    });

    let doc_warn = if note.kind == crawrad_note_format::NoteKind::Doc {
        let facts = crawrad_note_format::rustdoc::item_facts(&item);
        let problems = crawrad_note_format::rustdoc::doc_problems(&body, &facts);
        if problems.is_empty() {
            None
        } else {
            let msg = problems.join("; ");
            Some(quote_spanned! {span=>
                #[deprecated(note = #msg)]
                const NOTE_NOT_RUSTDOC_STYLE: () = ();
                let _ = NOTE_NOT_RUSTDOC_STYLE;
            })
        }
    } else {
        None
    };

    let abs = std::fs::canonicalize(&path).unwrap_or(path);
    let abs = abs.to_string_lossy();
    let doc: Attribute = syn::parse_quote_spanned!(span=> #[doc = #body]);
    let track_stmt: Stmt = syn::parse_quote!({
        const _: &str = include_str!(#abs);
        #stale_warn
        #doc_warn
    });

    let sibling_track = track_stmt.clone();
    // syn reads a bodiless `fn f();` as a verbatim item; it is a trait item.
    let module_item = syn::parse2::<Item>(item.clone())
        .ok()
        .filter(|i| !matches!(i, Item::Verbatim(_)));
    if let Some(mut item) = module_item {
        let sibling = match &mut item {
            Item::Fn(f) => {
                f.attrs.insert(0, doc);
                f.block.stmts.insert(0, track_stmt);
                false
            }
            Item::Const(c) => {
                c.attrs.insert(0, doc);
                track_expr(&mut c.expr, track_stmt);
                false
            }
            Item::Static(s) => {
                s.attrs.insert(0, doc);
                track_expr(&mut s.expr, track_stmt);
                false
            }
            // `type` is legal in an impl too, where a sibling item is not.
            Item::Type(t) => {
                t.attrs.insert(0, doc);
                false
            }
            Item::Enum(i) => insert(&mut i.attrs, doc),
            Item::Struct(i) => insert(&mut i.attrs, doc),
            Item::Union(i) => insert(&mut i.attrs, doc),
            Item::Trait(i) => insert(&mut i.attrs, doc),
            Item::Mod(i) => insert(&mut i.attrs, doc),
            _ => {
                return Err(fail(format!(
                    "#[note(\"{id}\")] cannot document this kind of item"
                )));
            }
        };
        return Ok(if sibling {
            quote! {
                #item
                const _: () = #sibling_track;
            }
        } else {
            quote!(#item)
        });
    }

    // A bodiless trait method parses only as a trait item.
    if let Ok(mut item) = syn::parse2::<TraitItem>(item.clone())
        && let TraitItem::Fn(f) = &mut item
    {
        f.attrs.insert(0, doc);
        if let Some(block) = &mut f.default {
            block.stmts.insert(0, track_stmt);
        }
        return Ok(quote!(#item));
    }
    if let Ok(mut item) = syn::parse2::<ImplItem>(item.clone())
        && let ImplItem::Fn(f) = &mut item
    {
        f.attrs.insert(0, doc);
        f.block.stmts.insert(0, track_stmt);
        return Ok(quote!(#item));
    }
    Err(syn::Error::new(
        Span::call_site(),
        format!("#[note(\"{id}\")] cannot document this kind of item"),
    ))
}

fn insert(attrs: &mut Vec<Attribute>, doc: Attribute) -> bool {
    attrs.insert(0, doc);
    true
}

/// `expr` becomes `{ const _: &str = include_str!(..); expr }`.
fn track_expr(expr: &mut Box<Expr>, track: Stmt) {
    let inner = expr.as_ref().clone();
    let block: Block = syn::parse_quote!({ #track #inner });
    **expr = Expr::Block(syn::ExprBlock {
        attrs: Vec::new(),
        label: None,
        block,
    });
}

#[cfg(test)]
#[path = "test/expand/test.rs"]
mod test;
