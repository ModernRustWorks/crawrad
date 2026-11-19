//! The gate over the door a third party comes in by.
//!
//! Two families, and the second is the one that matters:
//!
//! 1. **the door accepts what is already there** — every native component,
//!    taken apart and handed back in, is accepted;
//! 2. **the door refuses, one reason at a time** — and each refusal is a defect
//!    that would otherwise show up far from here, without the culprit's name.

use crate::external::ExternalComponent;
use crate::registration::RegistrationError;
use crate::{Category, Registry};
use crawrad_ir::Value;

/// A minimal component that the door accepts — the base every refusal below
/// mutates ONE field of.
///
/// Built here and not derived from the catalogue on purpose: a refusal test
/// needs a value that is correct except for the one thing being measured, and
/// a real component brings twenty other fields that could carry the failure.
fn acceptable() -> ExternalComponent {
    ExternalComponent {
        class: "Connection".to_string(),
        crate_module: String::new(),
        desenho: Vec::new(),
        icon: Vec::new(),
        display: "Data Engine".to_string(),
        base_name: "data_engine".to_string(),
        palette: "Data".to_string(),
        prefix: "dbe".to_string(),
        default_size: (28.0, 28.0),
        props: vec![crate::external::ExternalProp {
            key: "name".to_string(),
            display: "Name".to_string(),
            default: Value::Str(String::new()),
            category: Category::Identity,
            kind: crate::external::ExternalKind::Text,
            readonly: false,
            bindable: false,
            options: Vec::new(),
            ref_class: None,
            tree_collection: false,
            item_class: None,
            multiline: false,
            hidden_when: None,
        }],
        events: Vec::new(),
        targets: vec!["egui".to_string()],
        children: crate::external::ExternalChildren::None,
        derived_geometry: false,
        visual: false,
        emits_code: true,
        private_state: true,
        provides: Vec::new(),
        text_line: None,
        requires: Vec::new(),
        inherits: None,
    }
}

// ---------------------------------------------------------------------------
// 1 — THE DOOR ACCEPTS WHAT IS ALREADY THERE
// ---------------------------------------------------------------------------

/// **Every native component is accepted by the third-party door.**
///
/// # Why this is the strongest form of the test
///
/// It says the door is not narrower than the catalogue: whatever the twenty
/// built-ins are allowed to be, a third party is allowed to be. A rule written
/// here that the natives break would make the door a second, stricter
/// catalogue — and the day the natives migrate to this door (ADR 0033 §9),
/// they would not fit through it.
///
/// The oracle is `Registry::builtin()`, so the twenty-first is charged the day
/// it is born.
#[test]
fn every_native_component_fits_through_the_outside_door() {
    let source = Registry::builtin();
    let mut measured = 0;
    for native in source.all() {
        let outside = ExternalComponent::from(native);
        // An EMPTY catalogue, otherwise the class itself would already be taken.
        let mut empty = Registry::default();
        empty
            .register_external(outside)
            .unwrap_or_else(|e| panic!("`{}` was refused by the door: {e}", native.class));
        measured += 1;
    }
    assert!(
        measured >= 16,
        "the catalogue shrank to {measured} classes; this gate measures nothing"
    );
}

/// **And what entered through the door is the SAME descriptor.**
///
/// Accepting and storing something else would be worse than refusing.
#[test]
fn what_entered_through_the_door_is_the_same_descriptor() {
    let source = Registry::builtin();
    let native = source.get("Button").expect("`Button` is in the catalogue");
    let mut empty = Registry::default();
    empty
        .register_external(ExternalComponent::from(native))
        .expect("`Button` passes through the door");
    assert_eq!(
        empty.get("Button").expect("entered"),
        native,
        "the descriptor changed when crossing the door"
    );
}

// ---------------------------------------------------------------------------
// 2 — THE DOOR REFUSES, ONE REASON AT A TIME
// ---------------------------------------------------------------------------

/// **The key that breaks the contract of ADR 0002 is refused HERE.**
///
/// # The defect this prevents, and it happens far away
///
/// Without this refusal, the component installs, appears on the palette, and the first
/// form that uses it is rejected by `Document::validate` **when reopening** —
/// with a message about a property key, without the name of the component that
/// declared it, and hours after installation.
#[test]
fn key_outside_contract_is_refused_at_the_door() {
    // `Foo` is not a known target.
    let mut c = acceptable();
    c.props[0].key = "Foo.bar".to_string();
    let e = Registry::default().register_external(c).unwrap_err();
    assert!(
        matches!(e, RegistrationError::BadPropKey { .. }),
        "expected key refusal, got {e:?}"
    );
    // THE SENTENCE CARRIES THE COMPONENT NAME, otherwise whoever installed it does not know what to
    // uninstall.
    assert!(
        e.to_string().contains("Connection"),
        "the sentence was: {e}"
    );

    // **AND A PREFIXLESS KEY PASSES, and this changed on 17/09/2026.**
    //
    // Here we used to require refusal of `InventedProperty` because it was not in the
    // `COMMON` list of forty names. Amendment 1 of ADR 0031 removed from this
    // list the gatekeeper role: it was a second copy of what the component
    // already declares, and it was exactly what prevented a third-party component
    // from bringing what is THEIRS — the `Connection.url` of the example.
    //
    // The refusal did not disappear; it changed places and became stronger. Who judges the
    // prefixless key is the CLASS, in `class_property::check`, and there
    // it catches what this list never caught: a `Button` with `interval`.
    let mut c = acceptable();
    c.props[0].key = "InventedProperty".to_string();
    Registry::default()
        .register_external(c)
        .expect("what the component declares is THEIRS, and the door no longer judges that");
}

/// **A class already taken is refused, and not overwritten.**
///
/// `Registry::register` inserts over existing entries, which is right for `builtin()`. A
/// third-party component that overwrote `Button` would swap the entire IDE palette
/// without a word on the screen.
#[test]
fn class_already_taken_does_not_overwrite() {
    let mut reg = Registry::builtin();
    let before = reg.get("Button").cloned().expect("`Button` is there");
    let mut c = acceptable();
    c.class = "Button".to_string();
    let e = reg.register_external(c).unwrap_err();
    assert_eq!(e, RegistrationError::ClassTaken("Button".to_string()));
    assert_eq!(
        reg.get("Button"),
        Some(&before),
        "the native `Button` was replaced by the outside one"
    );
}

/// **The class name becomes a TYPE in the generated project, and obeys the type rule.**
///
/// The refusal is written here and not left to `rustc` because the error must
/// arrive at install time, with the component name — and not as a compilation
/// error inside a project the user did not write.
#[test]
fn class_name_obeys_type_rule() {
    for bad in ["", "botao", "1Button", "My Button", "My_Button", "Botão"] {
        let mut c = acceptable();
        c.class = bad.to_string();
        let e = Registry::default().register_external(c).unwrap_err();
        assert!(
            matches!(e, RegistrationError::BadClass(_)),
            "`{bad}` should be refused as a class, got {e:?}"
        );
    }
}

/// **And the prefix becomes a FIELD, and obeys the field rule.**
#[test]
fn prefix_obeys_field_rule() {
    for bad in ["", "Btn", "1bt", "bt-n", "bt n"] {
        let mut c = acceptable();
        c.prefix = bad.to_string();
        let e = Registry::default().register_external(c).unwrap_err();
        assert!(
            matches!(e, RegistrationError::BadPrefix(_)),
            "`{bad}` should be refused as a prefix, got {e:?}"
        );
    }
    // And what the natives use passes — `acl`, `btn`, `form`, `edt`.
    for good in ["acl", "btn", "form", "edt", "tab_sheet1"] {
        let mut c = acceptable();
        c.prefix = good.to_string();
        assert!(
            Registry::default().register_external(c).is_ok(),
            "`{good}` is a native prefix and was refused"
        );
    }
}

/// **Duplicate property is refused.**
///
/// Two with the same key means the Inspector drawing the same row twice, and
/// the generator emitting the same field twice — which does not compile, far from here.
#[test]
fn duplicate_property_is_refused() {
    let mut c = acceptable();
    let copy = c.props[0].clone();
    c.props.push(copy);
    let e = Registry::default().register_external(c).unwrap_err();
    assert!(
        matches!(e, RegistrationError::DuplicateProp { .. }),
        "{e:?}"
    );
}

/// **Duplicate event as well.**
#[test]
fn duplicate_event_is_refused() {
    let mut c = acceptable();
    let ev = crate::external::ExternalEvent {
        name: "on_connect".to_string(),
        display: "OnConnect".to_string(),
        signature: "fn(&mut self)".to_string(),
    };
    c.events.push(ev.clone());
    c.events.push(ev);
    let e = Registry::default().register_external(c).unwrap_err();
    assert!(
        matches!(e, RegistrationError::DuplicateEvent { .. }),
        "{e:?}"
    );
}

/// **The target this IDE does not know is refused, and the sentence SAYS which ones it knows.**
///
/// The list comes from `contract::TARGETS`, and not from a literal in the message: there
/// was already a sentence saying `(known: win32, egui)` that became stale the day
/// `makepad` arrived, sending whoever read it looking in the wrong place.
#[test]
fn unknown_target_is_refused_with_the_right_list() {
    let mut c = acceptable();
    c.targets = vec!["qt".to_string()];
    let e = Registry::default().register_external(c).unwrap_err();
    assert!(
        matches!(e, RegistrationError::UnknownTarget { .. }),
        "{e:?}"
    );
    for known in crawrad_ir::contract::TARGETS {
        assert!(
            e.to_string().contains(known),
            "the sentence does not mention `{known}`: {e}"
        );
    }

    // And the WILDCARD passes: a universal component names no target.
    let mut c = acceptable();
    c.targets = vec![crate::UNIVERSAL_TARGET.to_string()];
    assert!(Registry::default().register_external(c).is_ok());
}

/// **A component with no properties at all is refused.**
///
/// Not even `name`: without it the Inspector opens empty and the component has no way
/// to be referenced in the generated code. It is the minimum that makes a component exist.
#[test]
fn component_without_properties_is_refused() {
    let mut c = acceptable();
    c.props.clear();
    let e = Registry::default().register_external(c).unwrap_err();
    assert!(matches!(e, RegistrationError::NoProps(_)), "{e:?}");
}

/// **The refusal DOES NOT leave the component half-registered.**
///
/// The entire validation runs before conversion, so a refused class does not
/// appear in the catalogue — not even with the properties that passed before the one
/// that failed.
#[test]
fn refusal_leaves_nothing_behind() {
    let mut reg = Registry::default();
    let mut c = acceptable();
    c.props[0].key = "Foo.bar".to_string();
    let name = c.class.clone();
    assert!(reg.register_external(c).is_err());
    assert!(
        reg.get(&name).is_none(),
        "the refused class remained in the catalogue"
    );
    assert_eq!(reg.all().count(), 0);
}

fn price_edit() -> ExternalComponent {
    crate::sample_price_edit()
}

/// **PriceEdit inheriting SpinEdit registers and merges props/events.**
#[test]
fn price_edit_inheriting_spin_edit_registers() {
    let mut reg = Registry::builtin();
    let spin = reg.get("SpinEdit").expect("SpinEdit exists").clone();
    let pe = price_edit();
    reg.register_external(pe).expect("PriceEdit registers");

    let desc = reg.get("PriceEdit").expect("PriceEdit is registered");
    let spin_keys: Vec<&str> = spin.props.iter().map(|p| p.key).collect();
    let price_keys: Vec<&str> = desc.props.iter().map(|p| p.key).collect();

    assert_eq!(&price_keys[..spin_keys.len()], &spin_keys[..]);
    assert_eq!(&price_keys[spin_keys.len()..], &["currency", "decimals"]);

    for ev in &spin.events {
        assert!(desc.events.iter().any(|e| e.name == ev.name));
    }

    assert!(reg.is_inherited(desc, "value"));
    assert!(reg.is_inherited(desc, "name"));
    assert!(!reg.is_inherited(desc, "currency"));
    assert!(!reg.is_inherited(desc, "decimals"));

    assert_eq!(reg.effective_class("PriceEdit"), "SpinEdit");
    assert_eq!(reg.effective_class("SpinEdit"), "SpinEdit");
}

/// **Inheriting from an unknown base is refused.**
#[test]
fn inheriting_unknown_base_is_refused() {
    let mut reg = Registry::builtin();
    let mut pe = price_edit();
    pe.inherits = Some("NoSuchBase".to_string());
    let err = reg.register_external(pe).unwrap_err();
    assert_eq!(
        err,
        RegistrationError::UnknownBase {
            class: "PriceEdit".to_string(),
            base: "NoSuchBase".to_string(),
        }
    );
    assert!(err.to_string().contains("PriceEdit") && err.to_string().contains("NoSuchBase"));
}

/// **Inheriting from an external base is refused.**
#[test]
fn inheriting_external_base_is_refused() {
    let mut reg = Registry::builtin();
    let mut base = acceptable();
    base.class = "CustomBase".to_string();
    base.crate_module = "custom_module".to_string();
    reg.register_external(base)
        .expect("external base registers");

    let mut pe = price_edit();
    pe.inherits = Some("CustomBase".to_string());
    let err = reg.register_external(pe).unwrap_err();
    assert_eq!(
        err,
        RegistrationError::ExternalBase {
            class: "PriceEdit".to_string(),
            base: "CustomBase".to_string(),
        }
    );
    assert!(err.to_string().contains("PriceEdit") && err.to_string().contains("CustomBase"));

    // A manifest-only package has no crate either, and is still not native.
    let mut manifest_only = acceptable();
    manifest_only.class = "ManifestOnly".to_string();
    manifest_only.prefix = "mo".to_string();
    manifest_only.crate_module = String::new();
    reg.register_external(manifest_only)
        .expect("manifest-only base registers");
    let mut pe = price_edit();
    pe.inherits = Some("ManifestOnly".to_string());
    assert!(matches!(
        reg.register_external(pe),
        Err(RegistrationError::ExternalBase { .. })
    ));
}

/// **Clashing with a base prop key is refused.**
#[test]
fn inheriting_clashing_key_is_refused() {
    let mut reg = Registry::builtin();
    let mut pe = price_edit();
    pe.props.push(crate::external::ExternalProp {
        key: "value".to_string(),
        display: "Value".to_string(),
        default: Value::Int(0),
        category: Category::Behavior,
        kind: crate::external::ExternalKind::Int { min: 0, max: 100 },
        readonly: false,
        bindable: false,
        options: Vec::new(),
        ref_class: None,
        tree_collection: false,
        item_class: None,
        multiline: false,
        hidden_when: None,
    });
    let err = reg.register_external(pe).unwrap_err();
    assert_eq!(
        err,
        RegistrationError::ClashesWithBase {
            class: "PriceEdit".to_string(),
            key: "value".to_string(),
        }
    );
    assert!(err.to_string().contains("PriceEdit") && err.to_string().contains("value"));
}

/// **Every built-in in the catalogue still has `inherits == None`.**
#[test]
fn every_builtin_has_no_inherits() {
    let reg = Registry::builtin();
    for desc in reg.all() {
        assert_eq!(
            desc.inherits, None,
            "`{}` unexpectedly declared inherits",
            desc.class
        );
    }
}
