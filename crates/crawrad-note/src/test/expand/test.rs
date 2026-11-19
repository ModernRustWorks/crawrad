use super::expand;
use quote::quote;

fn crate_dir() -> String {
    env!("CARGO_MANIFEST_DIR").to_string()
}

fn run(item: proc_macro2::TokenStream) -> String {
    expand(quote!("test-free-fn"), item, &crate_dir()).to_string()
}

#[test]
fn function_gets_the_doc_and_tracks_the_note_in_its_body() {
    let out = run(quote!(
        fn f() -> u8 {
            1
        }
    ));
    assert!(out.starts_with("# [doc = "), "{out}");
    assert!(out.contains("include_str !"), "{out}");
    assert!(
        !out.contains("const _ : () ="),
        "no sibling for a fn: {out}"
    );
}

#[test]
fn struct_gets_a_sibling_tracker() {
    let out = run(quote!(
        struct S;
    ));
    assert!(out.contains("# [doc = "), "{out}");
    assert!(out.contains("const _ : () ="), "{out}");
}

#[test]
fn const_tracks_inside_its_expression_so_it_also_works_in_an_impl() {
    let out = run(quote!(
        const X: u8 = 1;
    ));
    assert!(out.contains("include_str !"), "{out}");
    assert!(!out.contains("const _ : () ="), "{out}");
}

#[test]
fn bodiless_trait_method_keeps_its_doc() {
    let out = run(quote!(
        fn work(&self) -> u8;
    ));
    assert!(out.contains("# [doc = "), "{out}");
}

#[test]
fn method_with_self_tracks_in_its_body() {
    let out = run(quote!(
        pub fn label(self) -> u8 {
            1
        }
    ));
    assert!(out.contains("include_str !"), "{out}");
    assert!(!out.contains("const _ : () ="), "{out}");
}

#[test]
fn doc_carries_the_body_without_front_matter() {
    let out = run(quote!(
        struct S;
    ));
    assert!(!out.contains("kind: doc"), "{out}");
}

#[test]
fn missing_note_is_a_compile_error() {
    let out = expand(
        quote!("no-such-note"),
        quote!(
            struct S;
        ),
        &crate_dir(),
    )
    .to_string();
    assert!(out.contains("compile_error"), "{out}");
    assert!(out.contains("not found"), "{out}");
}

#[test]
fn bad_id_is_a_compile_error() {
    let out = expand(
        quote!("../x"),
        quote!(
            struct S;
        ),
        &crate_dir(),
    )
    .to_string();
    assert!(out.contains("compile_error"), "{out}");
}

#[test]
fn use_item_is_refused() {
    let out = run(quote!(
        use std::fmt;
    ));
    assert!(out.contains("compile_error"), "{out}");
}

fn signed(item: proc_macro2::TokenStream) -> String {
    expand(quote!("test-signed"), item, &crate_dir()).to_string()
}

/// **A changed signature warns, a reformatted one does not** — the
/// warning is a used `#[deprecated]` const, in the body of a fn and in the
/// sibling tracker of a struct.
#[test]
fn a_changed_signature_warns_through_a_deprecated_const() {
    let same = signed(quote!(
        #[inline]
        fn f(n: u8) -> u8 {
            n
        }
    ));
    assert!(!same.contains("deprecated"), "{same}");
    let changed = signed(quote!(
        fn f(n: u16) -> u8 {
            1
        }
    ));
    assert!(changed.contains("# [deprecated (note ="), "{changed}");
    assert!(
        changed.contains("was written for `fn f(n: u8) -> u8`"),
        "{changed}"
    );
    assert!(
        changed.contains("let _ = NOTE_SIGNATURE_CHANGED"),
        "{changed}"
    );
    let unsigned = run(quote!(
        fn other() {}
    ));
    assert!(!unsigned.contains("deprecated"), "{unsigned}");
    let sibling = signed(quote!(
        struct S;
    ));
    assert!(sibling.contains("const _ : () = {"), "{sibling}");
    assert!(sibling.contains("NOTE_SIGNATURE_CHANGED"), "{sibling}");
}

#[test]
fn rustdoc_problems_warn_through_a_deprecated_const() {
    // test-free-fn does not have # Errors, so a public fn returning Result warns
    let out = expand(
        quote!("test-free-fn"),
        quote!(
            pub fn f() -> Result<(), ()> {
                Ok(())
            }
        ),
        &crate_dir(),
    )
    .to_string();
    assert!(out.contains("NOTE_NOT_RUSTDOC_STYLE"), "{out}");
    assert!(out.contains("missing `# Errors`: returns Result"), "{out}");
}

#[test]
fn a_private_item_is_not_asked_for_sections() {
    // the same note on a private fn: sections are a public-API rule
    let out = expand(
        quote!("test-free-fn"),
        quote!(
            fn f() -> Result<(), ()> {
                Ok(())
            }
        ),
        &crate_dir(),
    )
    .to_string();
    assert!(!out.contains("NOTE_NOT_RUSTDOC_STYLE"), "{out}");
}
