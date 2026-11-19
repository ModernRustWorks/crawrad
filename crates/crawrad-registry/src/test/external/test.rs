//! The gate over the descriptor that comes from outside.
//!
//! It is the first proof of [ADR 0033], and it is deliberately the one that
//! runs before any manifest format or ABI exists: **the outside shape has to be
//! able to carry everything a native component has.** If it loses a field here,
//! the manifest would lose it, and so would the library.
//!

use crate::external::{ExternalChildren, ExternalComponent, ExternalKind, ExternalProp};
use crate::{Category, PropKind, Registry};
use crawrad_ir::Value;

/// **Every component of the catalogue survives the round trip, whole.**
///
/// # Why the oracle is the catalogue itself
///
/// Writing an expected descriptor by hand here would be rule 8 of the study —
/// two hand-written lists compared against each other — and it would cover one
/// component instead of twenty. Derived from `Registry::builtin()`, the
/// twenty-first is charged on the day it is born, and a field added to
/// `ComponentDesc` without a home in `ExternalComponent` fails here at once.
#[test]
fn every_component_survives_the_round_trip() {
    let reg = Registry::builtin();
    let mut measured = 0;
    for native in reg.all() {
        let outside = ExternalComponent::from(native);
        let back = outside.into_catalogue();
        assert_eq!(
            &back, native,
            "`{}` did not come back whole from the outside shape",
            native.class
        );
        measured += 1;
    }
    assert!(
        measured >= 16,
        "the catalogue shrank to {measured} classes; this gate measures nothing"
    );
}

/// **The payload of the kind comes back too, and it is not rebuilt from the
/// neighbouring field.**
///
/// `PropDesc::with_options` writes the options into the field AND into
/// `PropKind::Enum`; `references` does the same with the class. Carrying only the
/// discriminant and rebuilding the payload from the neighbour would pass the
/// round trip above — because the catalogue keeps them in sync today — and be
/// wrong for the first component that sets one without the other.
#[test]
fn the_kind_keeps_its_own_payload() {
    let enumerated = ExternalProp {
        key: "align".to_string(),
        display: "Align".to_string(),
        default: Value::Enum("None".to_string()),
        category: Category::Layout,
        // The TYPE says `a`, and the FIELD says `b`. A converter reconstructing the
        // type from the field would erase the difference in silence.
        kind: ExternalKind::Enum(vec!["a".to_string()]),
        readonly: false,
        bindable: false,
        options: vec!["b".to_string()],
        ref_class: None,
        tree_collection: false,
        item_class: None,
        multiline: false,
        hidden_when: None,
    };
    let back = ExternalComponent {
        class: "X".to_string(),
        crate_module: String::new(),
        desenho: Vec::new(),
        icon: Vec::new(),
        display: "X".to_string(),
        base_name: "x".to_string(),
        palette: "Extras".to_string(),
        prefix: "x".to_string(),
        default_size: (10.0, 10.0),
        props: vec![enumerated],
        events: Vec::new(),
        targets: vec!["egui".to_string()],
        children: ExternalChildren::None,
        derived_geometry: false,
        visual: true,
        emits_code: true,
        private_state: true,
        provides: Vec::new(),
        text_line: None,
        requires: Vec::new(),
        inherits: None,
    }
    .into_catalogue();

    let p = &back.props[0];
    assert_eq!(p.options, &["b"], "the `options` field was rewritten");
    assert_eq!(
        p.kind,
        PropKind::Enum(&["a"]),
        "the type was reconstructed from neighbouring field, and difference vanished"
    );
}

/// **A reference keeps the class it accepts.**
///
/// It is what makes the Inspector list only the actions in `Button.Action` and
/// only the image lists in `ActionList.Images`. Losing it turns a filtered
/// picker into a picker over the whole form.
#[test]
fn a_reference_keeps_the_class_it_accepts() {
    let reg = Registry::builtin();
    let button = reg.get("Button").expect("`Button` is in the catalog");
    let native = button
        .prop("action")
        .expect("`Button.action` is in the catalog");
    assert_eq!(
        native.ref_class,
        Some("Action"),
        "the setup of this test changed shape"
    );

    let back = ExternalComponent::from(button).into_catalogue();
    let p = back.prop("action").expect("the property came back");
    assert_eq!(p.ref_class, Some("Action"));
    assert_eq!(p.kind, PropKind::Reference("Action"));
}

/// **The default value crosses with its TYPE**, not as text.
///
/// `Value` already owns what it carries, so it is the only field that does not
/// need conversion — and is precisely what a file format would most
/// easily flatten into `String`. This test locks that down before the format
/// exists.
#[test]
fn default_value_crosses_with_type() {
    let reg = Registry::builtin();
    let mut shapes: Vec<std::mem::Discriminant<Value>> = Vec::new();
    for d in reg.all() {
        let back = ExternalComponent::from(d).into_catalogue();
        for (before, after) in d.props.iter().zip(back.props.iter()) {
            assert_eq!(
                before.default, after.default,
                "`{}.{}` changed default value on round trip",
                d.class, before.key
            );
            let shape = std::mem::discriminant(&before.default);
            if !shapes.contains(&shape) {
                shapes.push(shape);
            }
        }
    }
    assert!(
        shapes.len() >= 5,
        "only {} shapes of `Value` were exercised; the catalog shrank or this \
         test stopped sweeping",
        shapes.len()
    );
}
