//! What the registered signature becomes in the method declaration.
//!
//! # Why the example types carry a path (`std::io::Error`)
//!
//! Because **a generic with a simple comma does not fail the wrong splitter**,
//! and this was measured by mutation, not deduced. Changing
//! [`split_top_level`] back to `inner.split(',')` that was here, a
//! `fn(&mut self, e: Result<A, B>)` comes out split into `"e: Result<A"` and `"B>"` —
//! and the `join(", ")` at the end **reassembles the two exactly as they entered**,
//! because the loose piece has no `:` for the name prefixer to spoil. The
//! test passed green in both worlds.
//!
//! With a path inside the type, the loose piece HAS `:`, the prefixer reads it
//! as if it were a parameter (`_std: :io::Error>`), and the difference appears.
//! It is the house rule: the test that does not fail the old version was not
//! measuring anything.
//!
//! [`split_top_level`]: super::split_top_level

use super::*;

/// Builds an `EventDesc` only with signature, which is what is measured.
fn desc(signature: &'static str) -> EventDesc {
    EventDesc::new("on_test", "OnTest", signature)
}

#[test]
fn without_parameters_signature_becomes_receiver_alone() {
    assert_eq!(desc("fn(&mut self)").method_params(), "&mut self");
}

#[test]
fn declared_parameter_arrives_with_type_and_underscore() {
    assert_eq!(
        desc("fn(&mut self, e: KeyEvent)").method_params(),
        "&mut self, _e: KeyEvent"
    );
}

#[test]
fn underscore_is_not_doubled_when_already_in_signature() {
    assert_eq!(
        desc("fn(&mut self, _e: KeyEvent)").method_params(),
        "&mut self, _e: KeyEvent"
    );
}

/// The `&mut bool` of `OnCloseQuery` is a reference, and it must cross
/// intact — an output parameter arriving by value would compile and do
/// nothing.
#[test]
fn mutable_reference_crosses_intact() {
    assert_eq!(
        desc("fn(&mut self, can_close: &mut bool)").method_params(),
        "&mut self, _can_close: &mut bool"
    );
}

/// The latent defect: comma **inside** the type.
///
/// None of the events in the catalog has one today. The first that does would come out
/// split in half, and the error would appear in the user's project.
#[test]
fn comma_inside_generic_does_not_split_parameter() {
    assert_eq!(
        desc("fn(&mut self, e: Result<A, std::io::Error>)").method_params(),
        "&mut self, _e: Result<A, std::io::Error>"
    );
}

#[test]
fn comma_inside_tuple_does_not_split_parameter() {
    assert_eq!(
        desc("fn(&mut self, p: (f32, std::time::Duration))").method_params(),
        "&mut self, _p: (f32, std::time::Duration)"
    );
}

#[test]
fn nested_generic_closes_all_levels() {
    assert_eq!(
        desc("fn(&mut self, m: HashMap<String, Vec<std::path::PathBuf>>, n: u8)").method_params(),
        "&mut self, _m: HashMap<String, Vec<std::path::PathBuf>>, _n: u8"
    );
}

/// The arrow of a function type is the only `>` that closes nothing.
#[test]
fn function_type_arrow_is_not_confused_with_closing() {
    assert_eq!(
        desc("fn(&mut self, f: fn(u8, u8) -> std::cmp::Ordering, n: u8)").method_params(),
        "&mut self, _f: fn(u8, u8) -> std::cmp::Ordering, _n: u8"
    );
}

/// Signature that is no `fn(...)` at all: the catalog should not have one,
/// and if it does the handler is still born compilable.
#[test]
fn formless_signature_falls_back_to_receiver_alone() {
    assert_eq!(desc("this is not a signature").method_params(), "&mut self");
}

#[test]
fn bare_typed_parameter_signature_prepends_receiver() {
    assert_eq!(desc("value: i64").method_params(), "&mut self, _value: i64");
}

/// Sweep of the entire catalog: **every** registered signature must
/// produce a parameter list starting with the receiver.
///
/// Prevents a new `EventDesc` from being born with `fn(e: KeyEvent)` — without
/// receiver — and only being discovered in the user's project.
#[test]
fn every_signature_in_catalog_starts_with_receiver() {
    let reg = Registry::builtin();
    let mut seen: Vec<(&str, String)> = Vec::new();
    for desc in reg.all() {
        for ev in &desc.events {
            let params = ev.method_params();
            assert!(
                params.starts_with("&mut self") || params.starts_with("&self"),
                "`{}` of `{}` does not start with receiver: {params:?}",
                ev.name,
                desc.class
            );
            seen.push((ev.name, params));
        }
    }
    assert!(
        seen.iter().any(|(name, _)| *name == "on_key_down"),
        "the catalog lost `on_key_down` — this test stopped measuring anything"
    );
}

/// The same `name` cannot have two different signatures in the catalog.
///
/// The generator finds the event by class and, if not declared, sweeps the
/// catalog and takes the first. With two signatures for the same name, which
/// handler came out would depend on sweep order — and order is not a contract.
#[test]
fn same_event_has_same_signature_across_entire_catalog() {
    let reg = Registry::builtin();
    let mut by_name: std::collections::BTreeMap<&str, (&str, &str)> =
        std::collections::BTreeMap::new();
    for desc in reg.all() {
        for ev in &desc.events {
            if let Some((class, signature)) = by_name.insert(ev.name, (desc.class, ev.signature)) {
                assert_eq!(
                    signature, ev.signature,
                    "`{}` has two signatures: `{class}` says {signature:?} and `{}` says {:?}",
                    ev.name, desc.class, ev.signature
                );
            }
        }
    }
}
