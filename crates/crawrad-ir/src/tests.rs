use super::*;

mod ref_tests {
    use super::*;

    fn doc() -> Document {
        let mut d = Document::new("form_main", "Form");
        let mut acl = Node::new("ActionList1", "ActionList");
        acl.children.push("Action1".into());
        d.nodes
            .insert("Action1".into(), Node::new("Action1", "Action"));
        d.add_child("form_main", acl).unwrap();

        let mut b = Node::new("Button1", "Button");
        b.props
            .insert("action".into(), Value::Ref("Action1".into()));
        d.add_child("form_main", b).unwrap();
        d
    }

    #[test]
    fn reference_to_existing_node_is_valid() {
        assert!(doc().validate().is_ok());
    }

    #[test]
    fn dangling_reference_is_rejected() {
        let mut d = doc();
        d.node_mut("Button1")
            .unwrap()
            .props
            .insert("action".into(), Value::Ref("NaoExiste".into()));
        let e = d.validate().unwrap_err();
        assert!(
            matches!(e, IrError::DanglingRef { .. }),
            "dangling reference is silent garbage, it must hurt here: {e}"
        );
    }

    #[test]
    fn an_empty_reference_is_the_classic_nil() {
        let mut d = doc();
        d.node_mut("Button1")
            .unwrap()
            .props
            .insert("action".into(), Value::Ref(String::new()));
        assert!(d.validate().is_ok());
    }

    #[test]
    fn who_points_to_action_is_findable() {
        assert_eq!(doc().referrers("Action1"), vec!["Button1".to_string()]);
        assert!(doc().referrers("form_main").is_empty());
    }

    #[test]
    fn subtree_comes_from_parent_to_children() {
        let d = doc();
        let sub = d.subtree("ActionList1");
        assert_eq!(sub.len(), 2);
        assert_eq!(
            sub[0], "ActionList1",
            "parent comes before — removing is the reverse"
        );
        assert!(sub.contains(&"Action1".to_string()));
    }
}

fn form() -> Document {
    let mut d = Document::new("form_main", "Form");
    d.nodes
        .get_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));

    let mut btn = Node::new("btn_ok", "Button");
    btn.set_text("&OK");
    btn.set_rect(Rect::new(24.0, 96.0, 96.0, 32.0));
    btn.events.insert("on_click".into(), "btn_okClick".into());
    d.add_child("form_main", btn).unwrap();

    let mut edt = Node::new("edt_nome", "Edit");
    edt.set_rect(Rect::new(24.0, 48.0, 280.0, 24.0));
    // target-specific: Win32 has PasswordChar, egui does not
    edt.props
        .insert("win32.PasswordChar".into(), Value::Str("*".into()));
    d.add_child("form_main", edt).unwrap();

    d
}

#[test]
fn v1_file_opens_and_becomes_v2_without_losing_anything() {
    // ADR 0008 migration is straightforward because in v1 the id WAS the name: existing ids
    // become stable ids, and the name is born identical to the id the node
    // already had. Nothing moves, nothing is renumbered.
    let v1 = r#"{
      "format": 1,
      "root": "form_main",
      "nodes": {
        "form_main": { "id": "form_main", "class": "Form",
          "children": ["btn_ok", "Action1"] },
        "btn_ok": { "id": "btn_ok", "class": "Button",
          "props": { "text": { "t": "Str", "v": "&OK" },
                     "action": { "t": "Ref", "v": "Action1" } } },
        "Action1": { "id": "Action1", "class": "Action" }
      }
    }"#;

    let d = Document::from_json(v1).expect("v1 file must open");
    assert_eq!(d.format, FORMAT_VERSION, "and come out migrated");
    for id in ["form_main", "btn_ok", "Action1"] {
        let n = d.node(id).unwrap_or_else(|| panic!("{id} disappeared"));
        assert_eq!(
            n.name(),
            id,
            "the name is born identical to the id that already existed"
        );
    }
    assert_eq!(
        d.node("btn_ok").unwrap().props.get("action"),
        Some(&Value::Ref("Action1".into())),
        "the reference keeps pointing to the same place"
    );
    d.validate().expect("and the migrated document is valid");

    // Saving and reopening the migrated document is stable.
    let back = Document::from_json(&d.to_json().unwrap()).unwrap();
    assert_eq!(back, d);
}

#[test]
fn two_nodes_with_same_name_are_rejected() {
    let mut d = form();
    let name = d.node("btn_ok").unwrap().name().to_string();
    d.node_mut("edt_nome").unwrap().set_name(name);
    assert!(matches!(
        d.validate().unwrap_err(),
        IrError::DuplicateName(_)
    ));
}

#[test]
fn name_with_only_underscores_is_rejected_on_validation() {
    // `"___".trim()` is not empty, so the empty-name guard used to let it
    // pass — and a `Name` without letters or digits derives to an
    // **empty** identifier in every target. The measured damage is in the doc of
    // `name_has_alphanumeric`: erased words inside the file handwritten by
    // the user.
    for n in ["_", "__", "___", "_ _"] {
        let mut d = form();
        d.node_mut("btn_ok").unwrap().set_name(n);
        let e = d.validate().unwrap_err();
        assert!(
            matches!(&e, IrError::NameWithoutAlnum(x) if x == n),
            "{n:?} came out {e:?}"
        );
    }
    // A single character of substance is enough — the rule is about derivation, not
    // about taste. What is a Rust rule (ASCII, reserved keyword) remains in
    // `crawrad-codegen`, which rejects before writing any file.
    for n in ["_a", "a_", "b1", "botão", "1abc", "fn"] {
        let mut d = form();
        d.node_mut("btn_ok").unwrap().set_name(n);
        d.validate().unwrap_or_else(|e| panic!("{n:?} gave {e:?}"));
    }
    assert!(!name_has_alphanumeric("___"));
    assert!(name_has_alphanumeric("_a"));
}

#[test]
fn name_changes_and_id_stays() {
    let mut d = form();
    d.node_mut("btn_ok").unwrap().set_name("btnGravar");
    assert_eq!(d.node("btn_ok").unwrap().name(), "btnGravar");
    assert!(d.node("btnGravar").is_none(), "id did not become the name");
    assert_eq!(
        d.node_by_name("btnGravar").map(|n| n.id.as_str()),
        Some("btn_ok")
    );
    d.validate().unwrap();
}

#[test]
fn round_trip_preserves_document() {
    let d = form();
    let json = d.to_json().unwrap();
    let back = Document::from_json(&json).unwrap();
    assert_eq!(d, back);
}

#[test]
fn collection_survives_the_file() {
    // Icons of an ImageList are a model collection — `Value::List` of
    // `Value::Map`, not JSON hidden inside a string. If the collection did not
    // make it across the file, the editor would save and the user would reopen empty.
    let mut d = form();
    let icons = Value::List(vec![
        Value::Map(BTreeMap::from([
            ("name".to_string(), Value::Str("FileSave".into())),
            ("symbol".to_string(), Value::Str("S".into())),
        ])),
        Value::Map(BTreeMap::from([
            ("name".to_string(), Value::Str("FileOpen".into())),
            ("symbol".to_string(), Value::Str("O".into())),
        ])),
    ]);
    d.node_mut("btn_ok")
        .unwrap()
        .props
        .insert("images_data".into(), icons.clone());

    d.validate()
        .expect("collection does not violate the contract");

    let back = Document::from_json(&d.to_json().unwrap()).unwrap();
    assert_eq!(
        back.node("btn_ok").unwrap().props.get("images_data"),
        Some(&icons),
        "the list must return from the file identical, in the same order"
    );
    back.validate().expect("and remain valid after reopening");
}

#[test]
fn round_trip_is_byte_for_byte_stable() {
    // deterministic order: the file does not change on its own between saves,
    // otherwise every save becomes noise in git diff.
    let a = form().to_json().unwrap();
    let b = Document::from_json(&a).unwrap().to_json().unwrap();
    assert_eq!(a, b);
}

#[test]
fn loading_drops_the_keys_the_form_no_longer_declares() {
    // A form saved with `align`/`tab_order`/… on its root opens without
    // them, and a control keeps its own.
    let mut d = Document::new("form_main", "Form");
    let mut b = Node::new("Button1", "Button");
    b.props.insert("tab_order".into(), Value::Int(3));
    d.add_child("form_main", b).unwrap();
    let root = d.node_mut("form_main").unwrap();
    for key in FORM_RETIRED_KEYS {
        root.props.insert(key.to_string(), Value::Bool(true));
    }
    root.set_text("Form1");

    let back = Document::from_json(&d.to_json().unwrap()).unwrap();
    let root = back.node("form_main").unwrap();
    for key in FORM_RETIRED_KEYS {
        assert!(!root.props.contains_key(*key), "{key} left on the form");
    }
    assert_eq!(root.text(), Some("Form1"));
    assert_eq!(
        back.node("Button1").unwrap().props.get("tab_order"),
        Some(&Value::Int(3))
    );
}

#[test]
fn common_property_is_accepted() {
    assert_eq!(
        PropKey::parse("text").unwrap(),
        PropKey::Common("text".into())
    );
}

/// **A key without prefix means *every target* — and nothing more.**
///
/// # What this test measured BEFORE, and why the yardstick changed
///
/// It was named `propriedade_de_alvo_sem_prefixo_e_recusada` and demanded that
/// `PasswordChar` be **rejected** here, for not being in the `COMMON` list of
/// forty names.
///
/// [ADR 0031 Amendment 1] stripped that list of its gatekeeping role: it was the
/// second copy of what the component already declares, and forgetting it already cost dearly —
/// `Panel.border_width` was born in the palette with a property that this `parse`
/// rejected, and **every form with a panel failed**.
///
/// **The rejection did not disappear; it changed places and grew stronger.** What says
/// `PasswordChar` exists is the CLASS, and the per-class check lives in
/// `crawrad-doc`, which sees the catalog. There it catches what this list never
/// caught: a `Button` with the property `interval`, which belongs to `Timer`.
///
#[test]
fn key_without_prefix_applies_to_every_target() {
    let k = PropKey::parse("PasswordChar")
        .expect("without prefix is common, and common is every target");
    assert_eq!(k, PropKey::Common("PasswordChar".into()));
    assert!(k.applies_to("egui"));
    assert!(k.applies_to("makepad"));
}

/// **What remains here is what needs no catalog at all.**
///
/// Malformed keys and unknown targets — the two questions that this crate
/// can answer on its own, being the bottom layer.
#[test]
fn what_remains_in_contract_is_what_needs_no_catalog() {
    // Target this IDE does not know.
    assert!(matches!(
        PropKey::parse("qt.Frame").unwrap_err(),
        ContractError::UnknownTarget { .. }
    ));
    // Key with two dots, and key with empty name after the dot.
    for malformed in ["a.b.c", "egui."] {
        assert!(
            matches!(
                PropKey::parse(malformed).unwrap_err(),
                ContractError::MalformedKey(_)
            ),
            "`{malformed}` should be malformed"
        );
    }
    // And EMPTY key, which without `COMMON` would have no one else rejecting it:
    // it was being barred by accident, because `""` was not in the list.
    assert!(matches!(
        PropKey::parse("").unwrap_err(),
        ContractError::MalformedKey(_)
    ));
}

#[test]
fn target_property_with_prefix_is_accepted() {
    let k = PropKey::parse("win32.PasswordChar").unwrap();
    assert!(k.applies_to("win32"));
    assert!(!k.applies_to("egui"));
}

#[test]
fn unknown_target_is_rejected() {
    assert!(matches!(
        PropKey::parse("qt.Frame").unwrap_err(),
        ContractError::UnknownTarget { .. }
    ));
}

/// **`validate` continues catching the key that this crate can judge on its own.**
///
/// # What it caught before, and who catches it now
///
/// It inserted `Cancel` on a button and demanded rejection — which came from the
/// `COMMON` list, because `Cancel` was not in it. With [ADR 0031 Amendment 1] that
/// question belongs to the CLASS, and the answer lives in `crawrad-doc`.
///
/// What continues to belong to this crate is the key that **no catalog is needed
/// to judge**: targets the IDE does not know, and malformed keys. This is what
/// this test measures now, across the whole document — which is the path that
/// `Doc::open` traverses.
///
#[test]
fn validate_catches_key_that_this_crate_judges_alone() {
    // Unknown target: no runtime of this IDE is named `qt`.
    let mut d = form();
    d.node_mut("btn_ok")
        .unwrap()
        .props
        .insert("qt.Frame".into(), Value::Bool(true));
    assert!(matches!(
        d.validate().unwrap_err(),
        IrError::Contract { .. }
    ));

    // Malformed.
    let mut d = form();
    d.node_mut("btn_ok")
        .unwrap()
        .props
        .insert("egui.".into(), Value::Bool(true));
    assert!(matches!(
        d.validate().unwrap_err(),
        IrError::Contract { .. }
    ));

    // And what it NO LONGER judges: `Cancel` without prefix passes here, and it is
    // `crawrad-doc` that rejects it saying that `Button` does not declare it.
    let mut d = form();
    d.node_mut("btn_ok")
        .unwrap()
        .props
        .insert("Cancel".into(), Value::Bool(true));
    assert!(
        d.validate().is_ok(),
        "the contract of this crate does not know classes, and cannot judge this"
    );
}

#[test]
fn props_for_filters_by_runtime() {
    let d = form();
    let edt = d.node("edt_nome").unwrap();
    let win: Vec<_> = edt.props_for("win32").map(|(k, _)| k.to_string()).collect();
    let egui: Vec<_> = edt.props_for("egui").map(|(k, _)| k.to_string()).collect();
    assert!(win.contains(&"win32.PasswordChar".to_string()));
    assert!(!egui.contains(&"win32.PasswordChar".to_string()));
    // common appears in both
    assert!(win.contains(&"rect".to_string()) && egui.contains(&"rect".to_string()));
}

#[test]
fn validate_catches_nonexistent_child() {
    let mut d = form();
    d.node_mut("form_main")
        .unwrap()
        .children
        .push("fantasma".into());
    assert!(matches!(d.validate().unwrap_err(), IrError::MissingNode(_)));
}

#[test]
fn validate_catches_orphan() {
    let mut d = form();
    d.nodes.insert("solto".into(), Node::new("solto", "Button"));
    assert!(matches!(d.validate().unwrap_err(), IrError::Orphan(_)));
}

#[test]
fn validate_catches_two_parents() {
    let mut d = form();
    let mut painel = Node::new("pnl", "Panel");
    painel.children.push("btn_ok".into()); // already child of form
    d.nodes.insert("pnl".into(), painel);
    d.node_mut("form_main").unwrap().children.push("pnl".into());
    assert!(matches!(
        d.validate().unwrap_err(),
        IrError::MultipleParents(_)
    ));
}

#[test]
fn remove_subtree_takes_descendants() {
    let mut d = form();
    let pnl = Node::new("pnl", "Panel");
    d.add_child("form_main", pnl).unwrap();
    d.add_child("pnl", Node::new("dentro", "Label")).unwrap();
    let out = d.remove_subtree("pnl");
    assert_eq!(out.len(), 2);
    assert!(d.node("dentro").is_none());
    assert!(d.validate().is_ok());
}

#[test]
fn root_cannot_be_removed() {
    let mut d = form();
    assert!(d.remove_subtree("form_main").is_empty());
    assert!(d.validate().is_ok());
}

#[test]
fn future_format_is_rejected_with_useful_message() {
    let json = r#"{"format":99,"root":"f","nodes":{"f":{"id":"f","class":"Form"}}}"#;
    let e = Document::from_json(json).unwrap_err();
    assert!(matches!(e, IrError::FutureFormat { found: 99, .. }));
}

#[test]
fn duplicate_id_is_rejected() {
    let mut d = form();
    assert!(matches!(
        d.add_child("form_main", Node::new("btn_ok", "Button")),
        Err(IrError::DuplicateId(_))
    ));
}

#[test]
fn color_hex_round_trip() {
    assert_eq!(Color::from_hex("#1B6E52").unwrap().to_hex(), "#1B6E52");
    assert_eq!(Color::from_hex("1B6E5280").unwrap().to_hex(), "#1B6E5280");
    assert!(Color::from_hex("#xyz").is_none());
}

#[test]
fn walk_visits_in_z_order() {
    let d = form();
    let ids: Vec<_> = d.walk().iter().map(|n| n.id.as_str()).collect();
    assert_eq!(ids, vec!["form_main", "btn_ok", "edt_nome"]);
}

/// A form with a panel, and a button INSIDE it.
fn with_panel() -> Document {
    let mut d = Document::new("form", "Form");
    {
        let root = d.node_mut("form").unwrap();
        root.set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
        root.set_name("form");
    }
    let mut p = Node::new("pnl", "Panel");
    p.set_name("pnl_topo");
    p.set_rect(Rect::new(20.0, 30.0, 300.0, 120.0));
    d.add_child("form", p).unwrap();

    let mut b = Node::new("btn", "Button");
    b.set_name("btn_ok");
    // relative to the PANEL, not the form
    b.set_rect(Rect::new(10.0, 5.0, 96.0, 32.0));
    d.add_child("pnl", b).unwrap();

    let mut loose = Node::new("lbl", "Label");
    loose.set_name("lbl_solto");
    loose.set_rect(Rect::new(8.0, 200.0, 120.0, 20.0));
    d.add_child("form", loose).unwrap();
    d.validate().unwrap();
    d
}

#[test]
fn parent_of_each_node() {
    let d = with_panel();
    assert_eq!(d.parent_of("btn").map(|s| s.as_str()), Some("pnl"));
    assert_eq!(d.parent_of("pnl").map(|s| s.as_str()), Some("form"));
    assert_eq!(d.parent_of("form"), None, "the root has no parent");
    assert_eq!(d.parent_of("inexistente"), None);
}

#[test]
fn child_coordinate_is_relative_to_parent() {
    // It is the law of ADR 0012, and the same as the classic RAD IDE: the Left/Top of the button is
    // relative to the panel. Whoever sums is whoever paints.
    let d = with_panel();
    assert_eq!(d.parent_origin("btn"), (20.0, 30.0));
    assert_eq!(
        d.rect_in_form("btn"),
        Some(Rect::new(30.0, 35.0, 96.0, 32.0))
    );

    // Direct child of the form: the origin is zero, which is why the
    // rest of the system worked until now without knowing a distinction existed.
    assert_eq!(d.parent_origin("lbl"), (0.0, 0.0));
    assert_eq!(d.rect_in_form("lbl"), d.node("lbl").and_then(|n| n.rect()));
}

#[test]
fn paint_order_is_parent_before_children() {
    let d = with_panel();
    assert_eq!(
        d.in_paint_order(),
        vec!["pnl".to_string(), "btn".to_string(), "lbl".to_string()],
        "the panel comes before what is inside it, and the root is left out"
    );
}

#[test]
fn margins_cross_file_intact() {
    // `Value::Edges` was born in `value.rs` for "margin, padding and numerical
    // anchors" and spent its entire life without a client. Margins from classic RAD IDE
    // is the first, and what this test enforces is what a type without clients
    // never proved: that it survives `.crawform`.
    //
    // Saving and reopening is the path of EVERY form, and a margin that is
    // lost there does not disappear with a warning — it disappears by making the control touch
    // the border the next day, far from anything that looks like
    // the cause.
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    let mut n = Node::new("pnl", "Panel");
    n.set_rect(Rect::new(0.0, 0.0, 100.0, 40.0));
    n.props.insert(
        "margins".into(),
        Value::Edges(Edges {
            left: 1.0,
            top: 2.0,
            right: 3.0,
            bottom: 4.0,
        }),
    );
    n.props
        .insert("align_with_margins".into(), Value::Bool(true));
    d.add_child("form_main", n).unwrap();

    // Both keys are in the common core, otherwise the file does not even load.
    d.validate().expect("margins outside the contract");

    let json = d.to_json().unwrap();
    let back = Document::from_json(&json).expect("the file must reopen");
    let p = back.node("pnl").unwrap();
    assert_eq!(
        p.props.get("margins"),
        d.node("pnl").unwrap().props.get("margins"),
        "the four sides came back different from what they were"
    );
    assert_eq!(p.props.get("align_with_margins"), Some(&Value::Bool(true)));
    // And the four sides are four, and not one repeated: an asymmetric
    // margin saved as uniform would be invisible in an easy test.
    assert!(
        matches!(p.props.get("margins"), Some(Value::Edges(e)) if e.left == 1.0
            && e.top == 2.0
            && e.right == 3.0
            && e.bottom == 4.0),
        "came back {:?}",
        p.props.get("margins")
    );
}

#[test]
fn tree_with_cycle_does_not_hang_ide() {
    // `validate` catches this, but a tampered file could arrive before
    // it — and hanging is worse than ignoring.
    let mut d = with_panel();
    d.node_mut("btn").unwrap().children.push("pnl".into());
    let _ = d.ancestors("btn");
    let _ = d.parent_origin("btn");
}

#[test]
fn datetime_survives_the_file() {
    let dt = DateTime::new(2026, 9, 19, 14, 55, 30);
    let val = Value::DateTime(dt);
    assert_eq!(val.kind(), "datetime");
    assert_eq!(val.as_datetime(), Some(dt));

    let json = serde_json::to_string(&val).unwrap();
    let back: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(back, val);
}

#[test]
fn filepath_survives_the_file() {
    let fp = FilePath::new("data/schema.sql");
    let val = Value::FilePath(fp.clone());
    assert_eq!(val.kind(), "file_path");
    assert_eq!(val.as_file_path(), Some("data/schema.sql"));

    let json = serde_json::to_string(&val).unwrap();
    let back: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(back, val);
}

#[test]
fn cursor_survives_the_file() {
    let cur = Cursor::Hand;
    let val = Value::Cursor(cur);
    assert_eq!(val.kind(), "cursor");
    assert_eq!(val.as_cursor(), Some(Cursor::Hand));

    let json = serde_json::to_string(&val).unwrap();
    let back: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(back, val);
}

#[test]
fn modal_result_survives_the_file() {
    let mr = ModalResult::Ok;
    let val = Value::ModalResult(mr);
    assert_eq!(val.kind(), "modal_result");
    assert_eq!(val.as_modal_result(), Some(ModalResult::Ok));

    let json = serde_json::to_string(&val).unwrap();
    let back: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(back, val);
}

#[test]
fn shortcut_survives_the_file() {
    let sc = Shortcut::ctrl(Key::S);
    let val = Value::Shortcut(sc);
    assert_eq!(val.kind(), "shortcut");
    assert_eq!(val.as_shortcut(), Some(Shortcut::ctrl(Key::S)));

    let json = serde_json::to_string(&val).unwrap();
    assert_eq!(json, r#"{"t":"Shortcut","v":"Ctrl+S"}"#);
    let back: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(back, val);

    // Also test parsing from string and empty/none
    let sc_none = Shortcut::NONE;
    let val_none = Value::Shortcut(sc_none);
    let json_none = serde_json::to_string(&val_none).unwrap();
    assert_eq!(json_none, r#"{"t":"Shortcut","v":""}"#);
    let back_none: Value = serde_json::from_str(&json_none).unwrap();
    assert_eq!(back_none, val_none);
}

#[test]
fn font_survives_the_file() {
    let font = Font::new("Arial", 11.0).bold(true).italic(true);
    let val = Value::Font(font.clone());
    assert_eq!(val.kind(), "font");
    assert_eq!(val.as_font(), Some(&font));

    let json = serde_json::to_string(&val).unwrap();
    let back: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(back, val);

    // Test backwards compatibility with "family" key in JSON
    let legacy_json = r#"{"t":"Font","v":{"family":"Tahoma","size":10.0,"bold":false,"italic":false,"underline":false}}"#;
    let legacy_val: Value = serde_json::from_str(legacy_json).unwrap();
    let parsed_font = legacy_val.as_font().unwrap();
    assert_eq!(parsed_font.name, "Tahoma");
    assert_eq!(parsed_font.size, 10.0);
}

/// The text colour travels in the file, in the Inspector text and back,
/// and a file written before it existed reads as the theme's colour.
#[test]
fn font_color_survives_the_file_and_the_inspector_text() {
    let font = Font::new("Arial", 11.0).color(Color::rgb(0x3A, 0x5A, 0x8A));
    let val = Value::Font(font.clone());
    let back: Value = serde_json::from_str(&serde_json::to_string(&val).unwrap()).unwrap();
    assert_eq!(back, val);

    let text = font.to_string_formatted();
    assert_eq!(text, "Arial, 11.0pt, #3A5A8A");
    assert_eq!(Font::from_str_formatted(&text), Some(font));

    let legacy = r#"{"t":"Font","v":{"name":"Tahoma","size":10.0}}"#;
    let legacy: Value = serde_json::from_str(legacy).unwrap();
    assert_eq!(
        legacy.as_font().unwrap().color.a,
        0,
        "no colour = the theme's"
    );
    assert_eq!(Font::from_str_formatted("Arial, 9pt, #12"), None);
}

#[test]
fn picture_survives_the_file() {
    let png_header = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 1, 2, 3];
    let pic = Picture::from_bytes(png_header.clone());
    let val = Value::Picture(pic.clone());
    assert_eq!(val.kind(), "picture");
    assert_eq!(val.as_picture(), Some(&pic));

    let json = serde_json::to_string(&val).unwrap();
    let back: Value = serde_json::from_str(&json).unwrap();
    assert_eq!(back, val);

    // Empty picture roundtrip
    let empty_pic = Picture::empty();
    let val_empty = Value::Picture(empty_pic.clone());
    let json_empty = serde_json::to_string(&val_empty).unwrap();
    let back_empty: Value = serde_json::from_str(&json_empty).unwrap();
    assert_eq!(back_empty, val_empty);

    // Picture with external source
    let src_pic = Picture::empty().with_source("assets/banner.png");
    let val_src = Value::Picture(src_pic.clone());
    let json_src = serde_json::to_string(&val_src).unwrap();
    let back_src: Value = serde_json::from_str(&json_src).unwrap();
    assert_eq!(back_src, val_src);
}

#[test]
fn visual_inheritance_serializes_and_deserializes() {
    let doc = Document::new_inherited("form_cliente", "Form", "form_base");
    assert_eq!(doc.inherits(), Some("form_base"));

    let json = doc.to_json().unwrap();
    assert!(json.contains("\"inherits\": \"form_base\""));

    let back = Document::from_json(&json).unwrap();
    assert_eq!(back.inherits(), Some("form_base"));
    assert_eq!(back.root, "form_cliente");
}

#[test]
fn visual_inheritance_resolves_ancestor_and_new_nodes() {
    let mut base = Document::new("form_base", "Form");
    let mut btn_salvar = Node::new("btn_salvar", "Button");
    btn_salvar.set_text("Save");
    btn_salvar.set_rect(Rect::new(10.0, 10.0, 80.0, 30.0));
    base.add_child("form_base", btn_salvar).unwrap();

    let mut derived = Document::new_inherited("form_cliente", "Form", "form_base");
    // New node added in derived form
    let mut edt_nome = Node::new("edt_nome", "Edit");
    edt_nome.set_text("Default Customer");
    edt_nome.set_rect(Rect::new(10.0, 50.0, 200.0, 24.0));
    derived.add_child("form_cliente", edt_nome).unwrap();

    // Property override on inherited node
    let mut btn_override = Node::new("btn_salvar", "Button");
    btn_override.set_text("Save Customer");
    derived.nodes.insert("btn_salvar".into(), btn_override);

    let resolved = derived.resolve_inheritance(&base).unwrap();
    assert_eq!(resolved.root, "form_cliente");
    assert_eq!(resolved.inherits(), Some("form_base"));

    // Inherited node exists and has overridden text
    let btn = resolved.node("btn_salvar").unwrap();
    assert_eq!(btn.text(), Some("Save Customer"));
    assert_eq!(btn.rect(), Some(Rect::new(10.0, 10.0, 80.0, 30.0))); // preserved from base

    // New node exists in resolved document
    let edt = resolved.node("edt_nome").unwrap();
    assert_eq!(edt.text(), Some("Default Customer"));

    // Both are children of the root
    let root = resolved.node("form_cliente").unwrap();
    assert!(root.children.contains(&"btn_salvar".to_string()));
    assert!(root.children.contains(&"edt_nome".to_string()));

    // Verification of inherited node
    assert!(resolved.is_inherited("btn_salvar", &base));
    assert!(!resolved.is_inherited("edt_nome", &base));
}

#[test]
fn visual_inheritance_rejects_self_inheritance_and_cycle() {
    let derived = Document::new_inherited("form_base", "Form", "form_base");
    let base = Document::new("form_base", "Form");
    let err = derived.resolve_inheritance(&base).unwrap_err();
    assert!(matches!(err, IrError::SelfInheritance(_)));

    let a = Document::new_inherited("form_a", "Form", "form_b");
    let b = Document::new_inherited("form_b", "Form", "form_a");
    let err = a.resolve_inheritance(&b).unwrap_err();
    assert!(matches!(err, IrError::InheritanceCycle { .. }));
}

fn panel_base() -> Document {
    let mut base = Document::new("form_a", "Form");
    base.add_child("form_a", Node::new("pnl_top", "Panel"))
        .unwrap();
    let mut save = Node::new("btn_save", "Button");
    save.set_text("Save");
    save.events
        .insert("on_click".into(), "btn_save_click".into());
    base.add_child("pnl_top", save).unwrap();
    base.add_child("form_a", Node::new("btn_close", "Button"))
        .unwrap();
    base
}

#[test]
fn the_own_part_keeps_only_what_the_derived_form_adds_and_round_trips() {
    let base = panel_base();
    let derived = Document::new_inherited("form_b", "Form", "form_a");
    let mut resolved = derived.resolve_inheritance(&base).unwrap();
    // an own button inside the inherited panel, and an override of an inherited event
    resolved
        .add_child("pnl_top", Node::new("btn_new", "Button"))
        .unwrap();
    resolved
        .nodes
        .get_mut("btn_save")
        .unwrap()
        .events
        .insert("on_click".into(), "btn_save_click_override".into());

    let own = resolved.own_part(&base);
    let text = own.to_json().unwrap();
    let loaded = Document::from_json(&text).expect("the own part loads on its own");
    assert!(
        loaded.node("btn_close").is_none(),
        "an untouched inherited node stays out"
    );
    let panel = &loaded.node("pnl_top").unwrap().children;
    assert_eq!(panel, &["btn_save".to_string(), "btn_new".to_string()]);
    let save = loaded.node("btn_save").unwrap();
    assert!(save.props.is_empty(), "the inherited text is the base's");
    assert_eq!(save.events["on_click"], "btn_save_click_override");
    assert_eq!(loaded.resolve_inheritance(&base).unwrap(), resolved);
}

#[test]
fn an_event_set_back_to_the_base_handler_leaves_the_own_part() {
    let base = panel_base();
    let derived = Document::new_inherited("form_b", "Form", "form_a");
    let resolved = derived.resolve_inheritance(&base).unwrap();
    let own = resolved.own_part(&base);
    assert_eq!(own.nodes.len(), 1, "only the root: {:?}", own.nodes.keys());
    assert_eq!(own.resolve_inheritance(&base).unwrap(), resolved);
}

#[test]
fn an_own_node_pointing_at_an_inherited_one_brings_it_into_the_file() {
    let mut base = panel_base();
    base.add_child("form_a", Node::new("act_save", "Action"))
        .unwrap();
    let derived = Document::new_inherited("form_b", "Form", "form_a");
    let mut resolved = derived.resolve_inheritance(&base).unwrap();
    let mut item = Node::new("btn_again", "Button");
    item.props
        .insert("action".into(), Value::Ref("act_save".into()));
    resolved.add_child("form_b", item).unwrap();
    let own = resolved.own_part(&base);
    let loaded = Document::from_json(&own.to_json().unwrap()).expect("no dangling ref");
    assert!(loaded.node("act_save").is_some());
    assert_eq!(loaded.resolve_inheritance(&base).unwrap(), resolved);
}

#[test]
fn a_chain_folds_from_the_outermost_base_down() {
    let a = panel_base();
    let mut b = Document::new_inherited("form_b", "Form", "form_a");
    b.add_child("form_b", Node::new("btn_b", "Button")).unwrap();
    let chain = Document::resolve_chain(&[b, a]).unwrap();
    assert_eq!(chain.root, "form_b");
    assert!(chain.node("btn_save").is_some() && chain.node("btn_b").is_some());
    assert!(Document::resolve_chain(&[]).is_none());
}
