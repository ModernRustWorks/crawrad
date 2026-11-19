//! The gate for checking by CLASS.
//!
//! The test that decides is the first: it measures the hole that the global list never
//! caught, and that [ADR 0031] had already predicted in writing.
//!

use crate::Registry;
use crawrad_ir::{Document, Value};

use super::{ClassError, check};

/// A document with a form and a button inside, assembled by the catalog —
/// not by hand, so that it has exactly the properties that the palette gives.
fn form_with_button() -> Document {
    let reg = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    let root = d.root.clone();
    let button = reg
        .instantiate(
            "Button",
            "btn_ok",
            crawrad_ir::Rect::new(10.0, 10.0, 80.0, 24.0),
        )
        .expect("the `Button` is in the catalog");
    d.nodes.insert("btn_ok".into(), button);
    d.node_mut(&root)
        .expect("the root exists")
        .children
        .push("btn_ok".into());
    d
}

/// **The hole that the global list never caught: a `Button` with `Timer`'s
/// property.**
///
/// # The measurement, made before changing
///
/// With `validate()` from before — which consulted `contract::COMMON` —, this
/// **validated**:
///
/// ```text
/// a `Button` with the `interval` property -> Ok
/// ```
///
/// The list knew that `interval` was a legal key, but not whose. It is what
/// ADR 0031 predicted: *"any property guard needs the key to become
/// (class, key), or the loophole remains"*.
#[test]
fn the_button_with_the_timer_property_is_rejected() {
    let reg = Registry::builtin();
    let mut d = form_with_button();
    d.node_mut("btn_ok")
        .expect("the button is there")
        .props
        .insert("interval".into(), Value::Int(500));

    // THE `crawrad-ir` CONTRACT LETS IT PASS, and that is how it must be: it does
    // not know class, and the lower layer will not begin to know.
    assert!(
        d.validate().is_ok(),
        "the contract without catalog has no way to judge this"
    );

    let e = check(&d, &reg).expect_err("the class is who judges, and it rejects");
    match &e {
        ClassError::UndeclaredProperty { class, key, node } => {
            assert_eq!(class, "Button");
            assert_eq!(key, "interval");
            assert_eq!(node, "btn_ok");
        }
        other => panic!("expected property rejection, got {other:?}"),
    }
    // THE SENTENCE NAMES THE CLASS AND THE KEY. The previous one said *"not in the common
    // core"*, which sends whoever reads it looking in the wrong place.
    let sentence = e.to_string();
    assert!(
        sentence.contains("Button") && sentence.contains("interval"),
        "{sentence}"
    );
}

/// **What the palette creates passes** — the components, one by one.
///
/// Without this half, the check could be rejecting everything and the test above
/// would stay green by accident. The oracle is the catalog, so a newly born
/// component is charged on the day it is born.
#[test]
fn everything_the_palette_creates_passes_the_check() {
    let reg = Registry::builtin();
    let mut measured = 0;
    for desc in reg.all() {
        let mut d = Document::new("form_main", "Form");
        let root = d.root.clone();
        let node = reg
            .instantiate(
                desc.class,
                "target",
                crawrad_ir::Rect::new(0.0, 0.0, 50.0, 20.0),
            )
            .unwrap_or_else(|| panic!("`{}` does not instantiate", desc.class));
        d.nodes.insert("target".into(), node);
        d.node_mut(&root)
            .expect("root")
            .children
            .push("target".into());

        check(&d, &reg).unwrap_or_else(|e| {
            panic!(
                "`{}` was created by the palette and rejected here: {e}",
                desc.class
            )
        });
        measured += 1;
    }
    assert!(
        measured >= 16,
        "the catalog shrank to {measured} classes; this gate measures nothing"
    );
}

/// **A key with target prefix passes WITHOUT the class declaring it.**
///
/// # And this is what keeps the examples standing
///
/// The examples' `form_main.crawform` carries eight `win32.*` keys. The legacy
/// runtime died in ADR 0013 and **no class declares them** — rejecting them here would make
/// the new check reject the owner's own form.
///
/// And it is not indulgence: it is what the contract promises in writing — *"runtimes that
/// do not know it must preserve it in the file and ignore it at runtime"*.
#[test]
fn key_with_target_prefix_passes_without_class_declaring_it() {
    let reg = Registry::builtin();
    let mut d = form_with_button();
    d.node_mut("btn_ok")
        .expect("the button is there")
        .props
        .insert("win32.ShowRoot".into(), Value::Bool(true));
    check(&d, &reg).expect("target key is preserved, not judged by class");

    // And an UNKNOWN target continues to be caught — by `crawrad-ir`, which is what
    // can judge it without a catalog.
    let mut d = form_with_button();
    d.node_mut("btn_ok")
        .expect("the button is there")
        .props
        .insert("qt.Frame".into(), Value::Bool(true));
    assert!(
        d.validate().is_err(),
        "the target `qt` does not exist in this IDE"
    );
}

/// **The class that is not installed has its OWN sentence.**
///
/// It is the case born with a third-party component: someone uninstalls the package
/// and the forms that used it are left without an owner. Saying *"invalid property
/// key"* would send the person looking for the key, when what is missing is the
/// entire component.
#[test]
fn uninstalled_class_says_it_is_what_is_missing() {
    let reg = Registry::builtin();
    let mut d = form_with_button();
    d.node_mut("btn_ok").expect("the button is there").class = "Connection".to_string();

    let e = check(&d, &reg).expect_err("`Connection` is not installed");
    match &e {
        ClassError::UnknownClass { class, node } => {
            assert_eq!(class, "Connection");
            assert_eq!(node, "btn_ok");
        }
        other => panic!("expected class rejection, got {other:?}"),
    }
    assert!(e.to_string().contains("not installed"), "{e}");
}

/// **A THIRD-PARTY component passes, with the properties it declared.**
///
/// It is the other end of the story: the check is not our own list of names
/// in disguise. A component the IDE did not compile, registered through the door of
/// `register_external`, has its properties accepted exactly like those of the
/// native components.
#[test]
fn third_party_component_has_its_properties_accepted() {
    use crate::external::{ExternalChildren, ExternalComponent, ExternalKind, ExternalProp};

    let mut reg = Registry::builtin();
    let prop = |key: &str| ExternalProp {
        key: key.to_string(),
        display: key.to_string(),
        default: Value::Str(String::new()),
        category: crate::Category::Behavior,
        kind: ExternalKind::Text,
        readonly: false,
        bindable: false,
        options: Vec::new(),
        ref_class: None,
        tree_collection: false,
        item_class: None,
        multiline: false,
        hidden_when: None,
    };
    reg.register_external(ExternalComponent {
        inherits: None,
        class: "Gadget".to_string(),
        crate_module: String::new(),
        desenho: Vec::new(),
        icon: Vec::new(),
        display: "Gadget".to_string(),
        base_name: "gadget".to_string(),
        palette: "Extras".to_string(),
        prefix: "gdg".to_string(),
        default_size: (28.0, 28.0),
        props: vec![prop("name"), prop("invented_by_third_party")],
        events: Vec::new(),
        targets: vec!["*".to_string()],
        children: ExternalChildren::None,
        derived_geometry: false,
        visual: false,
        emits_code: true,
        private_state: true,
        provides: Vec::new(),
        text_line: None,
        requires: Vec::new(),
    })
    .expect("`Gadget` enters through the third-party door");

    let mut d = Document::new("form_main", "Form");
    let root = d.root.clone();
    let mut node = crawrad_ir::Node::new("gdg1", "Gadget");
    node.props.insert("name".into(), Value::Str("gdg1".into()));
    node.props
        .insert("invented_by_third_party".into(), Value::Str("x".into()));
    d.nodes.insert("gdg1".into(), node);
    d.node_mut(&root)
        .expect("root")
        .children
        .push("gdg1".into());

    check(&d, &reg).expect("what third-party declared belongs to it, and is valid");

    // And what it did NOT declare is rejected in the same way as in a native component.
    d.node_mut("gdg1")
        .expect("is there")
        .props
        .insert("undeclared".into(), Value::Str("x".into()));
    assert!(matches!(
        check(&d, &reg).unwrap_err(),
        ClassError::UndeclaredProperty { .. }
    ));
}

/// An event key the class does not declare is rejected with its name —
/// and one it declares passes. The pair is what makes the check measure.
#[test]
fn an_undeclared_event_is_rejected_and_a_declared_one_passes() {
    let reg = Registry::builtin();

    let mut d = form_with_button();
    d.node_mut("btn_ok")
        .expect("the button")
        .events
        .insert("on_click".into(), "btn_ok_click".into());
    assert_eq!(check(&d, &reg), Ok(()), "`on_click` is a Button event");

    d.node_mut("btn_ok")
        .expect("the button")
        .events
        .insert("on_click_xyz".into(), "btn_ok_xyz".into());
    match check(&d, &reg) {
        Err(ClassError::UndeclaredEvent { node, class, event }) => {
            assert_eq!(
                (node.as_str(), class.as_str(), event.as_str()),
                ("btn_ok", "Button", "on_click_xyz")
            );
        }
        other => panic!("an invented event must be refused by name, got {other:?}"),
    }
}
