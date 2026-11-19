use crate::coerce::coerce_to_property;
use crate::{Args, Clipboard, Context};
use crawrad_doc::{Command, Doc};
use crawrad_ir::{Color, Document, Font, Item, Node, Rect, Value};
use crawrad_registry::{Category, PropDesc, PropKind, Registry};

#[test]
fn enum_from_string_accepted_and_wrong_name_rejected_with_options() {
    let desc = PropDesc::new_typed(
        "kind",
        "Kind",
        Value::Enum("bkDefault".into()),
        Category::Appearance,
        PropKind::Enum(&["bkDefault", "bkPrimary", "bkInfo"]),
    );
    let ir = Document::new("form1", "Form");
    let reg = Registry::builtin();

    // Accepted string -> Value::Enum
    let res = coerce_to_property(&Value::Str("bkPrimary".into()), &desc, &ir, &reg);
    assert_eq!(res, Ok(Value::Enum("bkPrimary".into())));

    // Wrong name -> rejected with options in message
    let err = coerce_to_property(&Value::Str("bkUnknown".into()), &desc, &ir, &reg).unwrap_err();
    assert!(
        err.contains("one of [bkDefault, bkPrimary, bkInfo]"),
        "expected options in err: {err}"
    );
    assert!(
        err.contains("bkUnknown"),
        "expected bad value in err: {err}"
    );
}

#[test]
fn font_from_plain_object_and_from_tagged_form() {
    let desc = PropDesc::new_typed(
        "font",
        "Font",
        Value::Font(Font::default()),
        Category::Appearance,
        PropKind::Font,
    );
    let ir = Document::new("form1", "Form");
    let reg = Registry::builtin();

    // Plain object (as Value::Map from JSON {"name": "Arial", "size": 12.0, "bold": true})
    let mut plain_map = std::collections::BTreeMap::new();
    plain_map.insert("name".into(), Value::Str("Arial".into()));
    plain_map.insert("size".into(), Value::Float(12.0));
    plain_map.insert("bold".into(), Value::Bool(true));
    let res = coerce_to_property(&Value::Map(plain_map), &desc, &ir, &reg).unwrap();
    let expected = Font::new("Arial", 12.0).bold(true);
    assert_eq!(res, Value::Font(expected));

    // Tagged form: {"t": "Font", "v": {"name": "Arial", "size": 14.0}}
    let mut inner_map = std::collections::BTreeMap::new();
    inner_map.insert("name".into(), Value::Str("Arial".into()));
    inner_map.insert("size".into(), Value::Float(14.0));
    let mut tagged_map = std::collections::BTreeMap::new();
    tagged_map.insert("t".into(), Value::Str("Font".into()));
    tagged_map.insert("v".into(), Value::Map(inner_map));
    let res2 = coerce_to_property(&Value::Map(tagged_map), &desc, &ir, &reg).unwrap();
    let expected2 = Font::new("Arial", 14.0);
    assert_eq!(res2, Value::Font(expected2));
}

#[test]
fn color_from_hex_and_from_rgb_map() {
    let desc = PropDesc::new_typed(
        "color",
        "Color",
        Value::Color(Color::rgb(0, 0, 0)),
        Category::Appearance,
        PropKind::Color,
    );
    let ir = Document::new("form1", "Form");
    let reg = Registry::builtin();

    // Hex string "#ff0000"
    let res = coerce_to_property(&Value::Str("#ff0000".into()), &desc, &ir, &reg).unwrap();
    assert_eq!(res, Value::Color(Color::rgb(255, 0, 0)));

    // Map with {r, g, b} (a is filled to 255)
    let mut rgb_map = std::collections::BTreeMap::new();
    rgb_map.insert("r".into(), Value::Int(0));
    rgb_map.insert("g".into(), Value::Int(128));
    rgb_map.insert("b".into(), Value::Int(255));
    let res2 = coerce_to_property(&Value::Map(rgb_map), &desc, &ir, &reg).unwrap();
    assert_eq!(res2, Value::Color(Color::rgb(0, 128, 255)));
}

#[test]
fn rect_from_list_of_four_numbers() {
    let desc = PropDesc::new_typed(
        "rect",
        "Rect",
        Value::Rect(Rect::new(0.0, 0.0, 100.0, 50.0)),
        Category::Layout,
        PropKind::Rect,
    );
    let ir = Document::new("form1", "Form");
    let reg = Registry::builtin();

    let list = Value::List(vec![
        Value::Int(1),
        Value::Int(2),
        Value::Int(3),
        Value::Int(4),
    ]);
    let res = coerce_to_property(&list, &desc, &ir, &reg).unwrap();
    assert_eq!(res, Value::Rect(Rect::new(1.0, 2.0, 3.0, 4.0)));
}

#[test]
fn collection_from_list_of_strings_becomes_text_maps() {
    let desc = PropDesc::new_typed(
        "items",
        "Items",
        Value::List(vec![]),
        Category::Appearance,
        PropKind::Collection {
            hierarchical: false,
            item_class: None,
        },
    );
    let ir = Document::new("form1", "Form");
    let reg = Registry::builtin();

    let list = Value::List(vec![Value::Str("Rust".into()), Value::Str("egui".into())]);
    let res = coerce_to_property(&list, &desc, &ir, &reg).unwrap();
    assert_eq!(
        res,
        Value::List(vec![
            Item::new("Rust").to_value(),
            Item::new("egui").to_value(),
        ])
    );
}

#[test]
fn reference_resolves_name_to_id_rejects_unknown_and_handles_empty() {
    let desc = PropDesc::new_typed(
        "action",
        "Action",
        Value::Ref(String::new()),
        Category::Behavior,
        PropKind::Reference("Action"),
    );
    let mut ir = Document::new("form1", "Form");
    let mut act_node = Node::new("act_123", "Action");
    act_node.set_name("save_action");
    ir.add_child("form1", act_node).unwrap();
    let reg = Registry::builtin();

    // Resolve by component NAME "save_action" -> Value::Ref("act_123")
    let res = coerce_to_property(&Value::Str("save_action".into()), &desc, &ir, &reg).unwrap();
    assert_eq!(res, Value::Ref("act_123".into()));

    // Resolve by component ID "act_123" -> Value::Ref("act_123")
    let res_id = coerce_to_property(&Value::Str("act_123".into()), &desc, &ir, &reg).unwrap();
    assert_eq!(res_id, Value::Ref("act_123".into()));

    // Unknown name rejected
    let err = coerce_to_property(&Value::Str("unknown_act".into()), &desc, &ir, &reg).unwrap_err();
    assert!(err.contains("unknown_act"), "err: {err}");
    assert!(err.contains("Action"), "err: {err}");

    // Empty string -> Value::Ref("")
    let empty_res = coerce_to_property(&Value::Str("".into()), &desc, &ir, &reg).unwrap();
    assert_eq!(empty_res, Value::Ref(String::new()));
}

#[test]
fn int_out_of_range_rejected() {
    let desc = PropDesc::new_typed(
        "tab_order",
        "Tab Order",
        Value::Int(0),
        Category::Behavior,
        PropKind::Int { min: 0, max: 100 },
    );
    let ir = Document::new("form1", "Form");
    let reg = Registry::builtin();

    // In range
    let ok = coerce_to_property(&Value::Int(50), &desc, &ir, &reg).unwrap();
    assert_eq!(ok, Value::Int(50));

    // Out of range (high)
    let err = coerce_to_property(&Value::Int(150), &desc, &ir, &reg).unwrap_err();
    assert!(err.contains("an integer in 0..=100"), "err: {err}");
    assert!(err.contains("150"), "err: {err}");

    // Out of range (low)
    let err2 = coerce_to_property(&Value::Int(-1), &desc, &ir, &reg).unwrap_err();
    assert!(err2.contains("an integer in 0..=100"), "err: {err2}");
}

#[test]
fn already_correct_value_returned_unchanged() {
    let ir = Document::new("form1", "Form");
    let reg = Registry::builtin();

    // Color
    let desc_color = PropDesc::new_typed(
        "color",
        "Color",
        Value::Color(Color::rgb(0, 0, 0)),
        Category::Appearance,
        PropKind::Color,
    );
    let val_color = Value::Color(Color::rgb(10, 20, 30));
    assert_eq!(
        coerce_to_property(&val_color, &desc_color, &ir, &reg),
        Ok(val_color)
    );

    // Rect
    let desc_rect = PropDesc::new_typed(
        "rect",
        "Rect",
        Value::Rect(Rect::new(0.0, 0.0, 10.0, 10.0)),
        Category::Layout,
        PropKind::Rect,
    );
    let val_rect = Value::Rect(Rect::new(5.0, 15.0, 25.0, 35.0));
    assert_eq!(
        coerce_to_property(&val_rect, &desc_rect, &ir, &reg),
        Ok(val_rect)
    );

    // Bool
    let desc_bool = PropDesc::new_typed(
        "enabled",
        "Enabled",
        Value::Bool(true),
        Category::Behavior,
        PropKind::Bool,
    );
    assert_eq!(
        coerce_to_property(&Value::Bool(false), &desc_bool, &ir, &reg),
        Ok(Value::Bool(false))
    );
}

#[test]
fn end_to_end_set_property_string_enum_lands_as_value_enum() {
    let reg = Registry::builtin();
    let mut doc = Document::new("form_main", "Form");
    let scroll_bar = reg
        .instantiate(
            "ScrollBar",
            "ScrollBar1",
            Rect::new(10.0, 10.0, 120.0, 16.0),
        )
        .expect("instantiate ScrollBar");
    doc.add_child("form_main", scroll_bar).unwrap();

    let d = Doc::new(doc);
    let action = crate::by_id("component.set_property").expect("component.set_property exists");

    // Pass string "sbVertical" for key "kind"
    let args = Args::texts("nodes", ["ScrollBar1"])
        .and("key", Value::Str("kind".into()))
        .and("value", Value::Str("sbVertical".into()));

    let ctx = Context {
        doc: Some(&d),
        registry: &reg,
        clipboard: &Clipboard::default(),
        args,
        project: crate::Project::default(),
        author: crawrad_doc::Author::User,
        editor_dirty: false,
        text: Default::default(),
    };

    let effect = (action.execute)(&ctx);
    let tx = effect.transaction.expect("produces transaction");
    let prop_cmd = tx.commands.iter().find(|c| match c {
        Command::SetProp { key, .. } => key == "kind",
        _ => false,
    });

    match prop_cmd {
        Some(Command::SetProp {
            key,
            value: Some(val),
            ..
        }) => {
            assert_eq!(key, "kind");
            assert_eq!(val, &Value::Enum("sbVertical".into()));
        }
        _ => panic!("Expected SetProp for 'kind', got {:?}", prop_cmd),
    }
}

/// A font or a colour of the control's own clears `ParentFont` /
/// `ParentColor` in the same step; any other key leaves them alone.
#[test]
fn own_font_or_colour_clears_the_parent_flag() {
    let reg = Registry::builtin();
    let mut doc = Document::new("form_main", "Form");
    let button = reg
        .instantiate("Button", "Button1", Rect::new(10.0, 10.0, 80.0, 24.0))
        .expect("instantiate Button");
    doc.add_child("form_main", button).unwrap();
    let d = Doc::new(doc);
    let action = crate::by_id("component.set_property").expect("component.set_property exists");
    let cleared = |key: &str, value: Value| {
        let ctx = Context {
            doc: Some(&d),
            registry: &reg,
            clipboard: &Clipboard::default(),
            args: Args::texts("nodes", ["Button1"])
                .and("key", Value::Str(key.into()))
                .and("value", value),
            project: crate::Project::default(),
            author: crawrad_doc::Author::User,
            editor_dirty: false,
            text: Default::default(),
        };
        let tx = (action.execute)(&ctx)
            .transaction
            .expect("produces transaction");
        tx.commands
            .iter()
            .filter_map(|c| match c {
                Command::SetProp {
                    key,
                    value: Some(Value::Bool(false)),
                    ..
                } if key.starts_with("parent_") => Some(key.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    let font = Value::Font(crawrad_ir::Font::default());
    assert_eq!(cleared("font", font), ["parent_font"]);
    let red = Value::Color(crawrad_ir::Color::from_hex("#FF0000").unwrap());
    assert_eq!(cleared("color", red), ["parent_color"]);
    assert!(cleared("text", Value::Str("OK".into())).is_empty());
}

/// A declared `makepad.*` key is typed like any other. It used to skip
/// the coercion for having a dot, so `makepad.look` landed as a string and the
/// codegen, which reads an enum, emitted the default slider.
#[test]
fn a_target_prefixed_enum_lands_as_value_enum() {
    let reg = Registry::builtin();
    let mut doc = Document::new("form_main", "Form");
    let track_bar = reg
        .instantiate("TrackBar", "TrackBar1", Rect::new(10.0, 10.0, 150.0, 32.0))
        .expect("instantiate TrackBar");
    doc.add_child("form_main", track_bar).unwrap();
    let d = Doc::new(doc);
    let action = crate::by_id("component.set_property").expect("component.set_property exists");
    let args = Args::texts("nodes", ["TrackBar1"])
        .and("key", Value::Str("makepad.look".into()))
        .and("value", Value::Str("Round".into()));
    let ctx = Context {
        doc: Some(&d),
        registry: &reg,
        clipboard: &Clipboard::default(),
        args,
        project: crate::Project::default(),
        author: crawrad_doc::Author::User,
        editor_dirty: false,
        text: Default::default(),
    };
    let tx = (action.execute)(&ctx)
        .transaction
        .expect("produces transaction");
    let value = tx.commands.iter().find_map(|c| match c {
        Command::SetProp { key, value, .. } if key == "makepad.look" => value.clone(),
        _ => None,
    });
    assert_eq!(value, Some(Value::Enum("Round".into())));
}
