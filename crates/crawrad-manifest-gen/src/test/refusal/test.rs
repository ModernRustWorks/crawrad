//! **What the macro refuses, and what it says while refusing.**
//!
//! A derive that guesses is worse than one that stops: the guess lands in the
//! Object Inspector as a property that looks declared and behaves wrong. Every
//! case here ends in a compile error the author can act on.

use crate::spec::ComponentSpec;
use syn::DeriveInput;

/// The error text, or a panic saying the macro accepted what it should not.
fn refusal(source: &str) -> String {
    let input: DeriveInput = syn::parse_str(source).expect("the test struct has to parse");
    match ComponentSpec::read(&input) {
        Ok(_) => panic!("the macro accepted a struct it should have refused"),
        Err(error) => error.to_string(),
    }
}

/// Without `palette` the component has nowhere to appear.
#[test]
fn a_component_without_a_palette_is_refused() {
    let said = refusal(
        r#"
        #[crawrad(prefix = "p", size = (10.0, 10.0))]
        struct Sample { }
        "#,
    );
    assert!(said.contains("palette"), "{said}");
}

/// A property with no `default` is refused, and the message names the field.
///
/// The macro cannot fall back to the type's zero: `Gauge::default()` starts
/// `text` at `"Gauge"` and `value` at `65`, and a silent `""`/`0` would put a
/// component on the canvas that looks broken the moment it is dropped.
#[test]
fn a_property_without_a_default_is_refused() {
    let said = refusal(
        r#"
        #[crawrad(palette = "P", prefix = "p", size = (10.0, 10.0))]
        struct Sample {
            #[prop(category = "behavior")]
            pub value: i64,
        }
        "#,
    );
    assert!(said.contains("default"), "{said}");
}

/// A type the macro cannot map is refused, and the message lists the kinds.
#[test]
fn an_unmappable_type_asks_for_the_kind() {
    let said = refusal(
        r#"
        #[crawrad(palette = "P", prefix = "p", size = (10.0, 10.0))]
        struct Sample {
            #[prop(category = "behavior", default = "")]
            pub odd: std::collections::HashMap<String, String>,
        }
        "#,
    );
    assert!(said.contains("kind"), "{said}");
    assert!(
        said.contains("collection"),
        "the message lists the kinds: {said}"
    );
}

/// A misspelled key is refused instead of quietly ignored.
///
/// Silently dropping `pallete` would ship a component whose palette group is
/// wrong, and the author would look at an attribute that reads correctly.
#[test]
fn a_misspelled_key_is_refused() {
    let said = refusal(
        r#"
        #[crawrad(pallete = "P", prefix = "p", size = (10.0, 10.0))]
        struct Sample { }
        "#,
    );
    assert!(said.contains("pallete"), "{said}");
}

/// A default the macro would have to evaluate is refused.
///
/// A manifest holding the text `SOME_CONST` instead of its value would be a lie
/// nobody reading the file could see.
#[test]
fn a_computed_default_is_refused() {
    let said = refusal(
        r#"
        #[crawrad(palette = "P", prefix = "p", size = (10.0, 10.0))]
        struct Sample {
            #[prop(category = "behavior", default = SOME_CONST)]
            pub value: i64,
        }
        "#,
    );
    assert!(said.contains("literal"), "{said}");
}

/// A tuple struct has no property names to publish.
#[test]
fn a_tuple_struct_is_refused() {
    let said = refusal(
        r#"
        #[crawrad(palette = "P", prefix = "p", size = (10.0, 10.0))]
        struct Sample(i64);
        "#,
    );
    assert!(said.contains("named fields"), "{said}");
}

/// `size` is a pair, and anything else is refused rather than padded.
#[test]
fn a_size_that_is_not_a_pair_is_refused() {
    let said = refusal(
        r#"
        #[crawrad(palette = "P", prefix = "p", size = (10.0, 10.0, 10.0))]
        struct Sample { }
        "#,
    );
    assert!(said.contains("pair"), "{said}");
}

/// An icon step with wrong arity is refused with a clear message.
#[test]
fn an_icon_step_with_wrong_arity_is_refused() {
    let said = refusal(
        r#"
        #[crawrad(palette = "P", prefix = "p", size = (10.0, 10.0))]
        #[icon(move_to(2.0))]
        struct Sample { }
        "#,
    );
    assert!(said.contains("move_to"), "{said}");
    assert!(said.contains("expects 2 arguments"), "{said}");
}

/// An unknown icon step is refused with a clear message.
#[test]
fn an_unknown_icon_step_is_refused() {
    let said = refusal(
        r#"
        #[crawrad(palette = "P", prefix = "p", size = (10.0, 10.0))]
        #[icon(polygon(2.0, 3.0))]
        struct Sample { }
        "#,
    );
    assert!(said.contains("polygon"), "{said}");
    assert!(said.contains("unknown icon step"), "{said}");
}

/// Multiple icon attributes on one component are refused.
#[test]
fn multiple_icon_attributes_are_refused() {
    let said = refusal(
        r#"
        #[crawrad(palette = "P", prefix = "p", size = (10.0, 10.0))]
        #[icon(fill)]
        #[icon(stroke)]
        struct Sample { }
        "#,
    );
    assert!(said.contains("only be declared once"), "{said}");
}
