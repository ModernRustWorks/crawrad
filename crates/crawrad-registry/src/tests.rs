use super::*;

/// Whoever has an AREA for children has a screen.
///
/// The combination `Area` + non-visual does not exist: a component without a window
/// has no surface on which to place children. It was exactly the state of
/// `ActionList` (`container: true`, `visual: false`) and the price was a
/// silent defect — dropping a component onto its badge made the
/// component disappear.
///
/// Sweeps the entire catalog, not a list written here: a new component
/// repeating the combination falls into this test in the same commit it is born.
#[test]
fn whoever_has_area_for_children_has_screen() {
    for d in Registry::builtin().all() {
        if d.children_area() {
            assert!(
                d.visual,
                "`{}` claims to have an area for children and is not visual — what does not \
                 become a window has no surface on which to place children",
                d.class
            );
        }
    }
}

#[test]
fn macro_assumes_visual_not_container_and_no_text_line() {
    // Defaults exist so the common case doesn't have to state them. If
    // they change accidentally, an entire component changes nature in
    // silence — a Button would become a container, a non-visual would become a window.
    let d = component!(
        "Test",
        display: "Test",
        base_name: "test",
        palette: "Standard",
        prefix: "tst",
        size: (10.0, 10.0),
        targets: ["win32"],
        props: [],
    );
    assert!(d.visual, "component is visual by default");
    assert!(!d.accepts_children(), "and does not accept children");
    assert!(d.text_line.is_none(), "and has no text line");
    assert!(d.events.is_empty());
    assert_eq!(
        d.props.len(),
        window_props().len(),
        "without `base:`, inherits common properties"
    );
}

#[test]
fn component_can_reject_common_base() {
    let d = component!(
        "NoBase",
        display: "NoBase",
        base_name: "no_base",
        palette: "",
        prefix: "nb",
        size: (10.0, 10.0),
        targets: ["win32"],
        visual: false,
        children: Children::Collection { item: "Action" },
        base: Vec::new(),
        props: [PropDesc::new(
            "text",
            "Text",
            Value::Str(String::new()),
            Category::Appearance,
        )],
    );
    assert_eq!(d.props.len(), 1, "only the declared one");
    assert!(!d.visual);
    // The macro forwards the full value, and both questions come from it: what
    // has no screen can have children, and cannot have an AREA for them.
    assert!(d.accepts_children());
    assert!(!d.children_area());
}

#[test]
fn chained_constructors_say_the_same_thing_as_literal() {
    let chained = PropDesc::new(
        "images",
        "Images",
        Value::Ref(String::new()),
        Category::Appearance,
    )
    .references("ImageList")
    .read_only()
    .with_options(&["a", "b"]);
    assert_eq!(chained.ref_class, Some("ImageList"));
    assert!(chained.readonly);
    assert_eq!(chained.options, &["a", "b"]);
}

#[test]
fn non_visual_is_in_palette_but_does_not_draw() {
    let r = Registry::builtin();
    for class in ["ActionList", "ImageList"] {
        let d = r.get(class).unwrap();
        assert!(!d.visual, "{class} does not become a window");
        assert_eq!(d.palette, "Standard", "{class} must be draggable");
        assert_eq!(d.default_size, ICON_SIZE);
    }
    // Action is born inside the list, not loose on the form.
    assert_eq!(r.get("Action").unwrap().palette, "");
    assert!(!r.get("Action").unwrap().visual);
    assert!(r.get("Button").unwrap().visual);
}

#[test]
fn reference_declares_class_it_accepts() {
    // Prevents inspector from offering an ImageList where an Action belongs.
    let r = Registry::builtin();
    assert_eq!(
        r.get("Button").unwrap().prop("action").unwrap().ref_class,
        Some("Action")
    );
    assert_eq!(
        r.get("ActionList")
            .unwrap()
            .prop("images")
            .unwrap()
            .ref_class,
        Some("ImageList")
    );
}

#[test]
fn every_declared_property_passes_the_contract() {
    // ADR 0002 guard applied across the entire registry: new component
    // does not enter bringing properties outside the common core.
    let r = Registry::builtin();
    for d in r.all() {
        for p in &d.props {
            assert!(
                crawrad_ir::PropKey::parse(p.key).is_ok(),
                "{}.{} is outside the contract",
                d.class,
                p.key
            );
        }
    }
}

#[test]
fn instantiating_any_class_produces_valid_document() {
    let r = Registry::builtin();
    let mut d = crawrad_ir::Document::new("form_main", "Form");
    for desc in r.all() {
        if desc.class == "Form" {
            continue;
        }
        let id = r.suggest_id(desc.class, |i| d.node(i).is_some());
        let n = r
            .instantiate(desc.class, id, Rect::new(0.0, 0.0, 10.0, 10.0))
            .unwrap();
        d.add_child("form_main", n).unwrap();
    }
    assert!(d.validate().is_ok(), "{:?}", d.validate());
}

#[test]
fn palette_brings_only_what_the_runtime_implements() {
    let r = Registry::builtin();
    let p = r.palette("egui");
    let classes: Vec<_> = p
        .iter()
        .flat_map(|(_, ds)| ds.iter().map(|d| d.class))
        .collect();
    assert!(classes.contains(&"Button"));
    assert!(!classes.contains(&"Form"), "Form is not a palette item");
}

#[test]
fn palette_brings_components_for_makepad_runtime() {
    let r = Registry::builtin();
    let p = r.palette("makepad");
    let classes: Vec<_> = p
        .iter()
        .flat_map(|(_, ds)| ds.iter().map(|d| d.class))
        .collect();
    assert!(classes.contains(&"Button"));
    assert!(classes.contains(&"Label"));
    assert!(classes.contains(&"Edit"));
    assert!(classes.contains(&"CheckBox"));
    assert!(classes.contains(&"Panel"));
    assert!(!classes.contains(&"Form"), "Form is not a palette item");
}

#[test]
fn instantiate_applies_descriptor_defaults() {
    let r = Registry::builtin();
    let n = r
        .instantiate("Button", "btn_1", Rect::new(10.0, 10.0, 96.0, 32.0))
        .unwrap();
    assert_eq!(n.class, "Button");
    assert_eq!(n.text(), Some("btn_1"), "is born with its own name written");
    assert_eq!(n.rect().unwrap().x, 10.0);
    assert_eq!(n.props.get("enabled"), Some(&Value::Bool(true)));
}

#[test]
fn created_node_respects_property_contract() {
    // every descriptor property must pass IR validation
    let r = Registry::builtin();
    let mut d = crawrad_ir::Document::new("form_main", "Form");
    for class in ["Button", "Label", "Edit"] {
        let id = r.suggest_id(class, |i| d.node(i).is_some());
        let n = r
            .instantiate(class, id, Rect::new(0.0, 0.0, 10.0, 10.0))
            .unwrap();
        d.add_child("form_main", n).unwrap();
    }
    assert!(d.validate().is_ok(), "{:?}", d.validate());
}

#[test]
fn automatic_name_follows_rust_standard() {
    let r = Registry::builtin();
    let existing = ["button1", "button2"];
    let id = r.suggest_id("Button", |i| existing.contains(&i));
    assert_eq!(id, "button3");
}

#[test]
fn props_for_separates_what_belongs_to_each_runtime() {
    let mut d = ComponentDesc {
        inherits: None,
        class: "X",
        display: "X",
        base_name: "x",
        palette: "",
        prefix: "x",
        default_size: (10.0, 10.0),
        props: window_props(),
        events: Vec::new(),
        targets: &["egui"],
        children: Children::NoChildren,
        derived_geometry: false,
        visual: true,
        emits_code: true,
        private_state: false,
        provides: &[],
        text_line: None,
        requires: Vec::new(),
        crate_module: "",
        desenho: Vec::new(),
        icon: Vec::new(),
    };
    d.props.push(PropDesc::new(
        "win32.Legacy",
        "Legacy",
        Value::Bool(false),
        Category::Advanced,
    ));
    let egui: Vec<_> = d.props_for("egui").map(|p| p.key).collect();
    let win: Vec<_> = d.props_for("win32").map(|p| p.key).collect();
    assert!(!egui.contains(&"win32.Legacy"), "{egui:?}");
    assert!(win.contains(&"win32.Legacy"), "{win:?}");
    assert!(
        egui.contains(&"rect") && win.contains(&"rect"),
        "common goes to both"
    );
}

#[test]
fn what_was_valid_in_both_targets_moved_to_common() {
    let r = Registry::builtin();
    let edit = r.get("Edit").unwrap();
    let keys: Vec<_> = edit.props.iter().map(|p| p.key).collect();
    assert!(keys.contains(&"password_char"), "{keys:?}");
    assert!(keys.contains(&"readonly"), "{keys:?}");
    assert!(
        !keys.iter().any(|k| k.starts_with("win32.")),
        "no descriptor may declare property of a removed target: {keys:?}"
    );
}

#[test]
fn edit_declares_text_input_props_and_events() {
    let r = Registry::builtin();
    let edit = r.get("Edit").unwrap();
    let prop_keys: Vec<_> = edit.props.iter().map(|p| p.key).collect();
    assert!(prop_keys.contains(&"text_hint"), "{prop_keys:?}");
    assert!(prop_keys.contains(&"numbers_only"), "{prop_keys:?}");
    assert!(prop_keys.contains(&"max_length"), "{prop_keys:?}");
    assert!(prop_keys.contains(&"alignment"), "{prop_keys:?}");
    assert!(prop_keys.contains(&"password_char"), "{prop_keys:?}");
    assert!(prop_keys.contains(&"readonly"), "{prop_keys:?}");

    let event_names: Vec<_> = edit.events.iter().map(|e| e.name).collect();
    assert!(event_names.contains(&"on_change"), "{event_names:?}");
    assert!(event_names.contains(&"on_enter"), "{event_names:?}");
    assert!(event_names.contains(&"on_exit"), "{event_names:?}");
    assert!(event_names.contains(&"on_key_press"), "{event_names:?}");
    assert!(event_names.contains(&"on_return"), "{event_names:?}");
}

#[test]
fn all_default_components_are_universal() {
    let r = Registry::builtin();
    for d in r.all() {
        assert!(d.supports("egui"), "{} must support egui", d.class);
        assert!(d.supports("makepad"), "{} must support makepad", d.class);
        assert!(
            d.supports("any_future"),
            "{} must be universal and support any future engine",
            d.class
        );
        assert!(
            !d.props.iter().any(|p| p.key.starts_with("win32.")),
            "{} still carries `win32.*` property",
            d.class
        );
    }
}

#[test]
fn only_tree_declares_tree_collection() {
    let r = Registry::builtin();
    let in_tree: Vec<&str> = r
        .all()
        .filter(|d| d.props.iter().any(|p| p.tree_collection))
        .map(|d| d.class)
        .collect();
    // ListView: a row's sub-items are its children, one level deep.
    assert_eq!(
        in_tree,
        vec!["ListView", "TreeView"],
        "only the tree and the list view have a tree collection"
    );
    for class in ["ComboBox", "ListBox"] {
        let items = r
            .get(class)
            .and_then(|d| d.prop("items"))
            .unwrap_or_else(|| panic!("{class} has `items`"));
        assert!(
            !items.tree_collection,
            "{class} is a FLAT list, and the editor cannot offer sub-items in it"
        );
    }
}

#[test]
fn only_memo_and_grid_declare_multiline_text() {
    let r = Registry::builtin();
    let mut multi: Vec<(&str, &str)> = r
        .all()
        .flat_map(|d| {
            d.props
                .iter()
                .filter(|p| p.multiline)
                .map(move |p| (d.class, p.key))
        })
        .collect();
    multi.sort();
    // the Grid's design-time cells: a line per row, a tab between columns;
    // a GridColumn's PickList: one value of its list editor per line
    assert_eq!(
        multi,
        vec![
            ("Grid", "cell_text"),
            ("GridColumn", "pick_list"),
            ("Memo", "text")
        ]
    );
    for d in r.all() {
        for p in d.props.iter().filter(|p| p.multiline) {
            assert!(
                matches!(p.default, Value::Str(_)),
                "{}.{} is multiline and not text: {}",
                d.class,
                p.key,
                p.default.kind()
            );
        }
    }
    assert!(!r.get("Button").unwrap().prop("text").unwrap().multiline);
}

#[test]
fn combo_is_in_standard_palette_of_both_targets() {
    let r = Registry::builtin();
    let d = r.get("ComboBox").expect("ComboBox in registry");
    assert_eq!(d.palette, "Standard");
    assert_eq!(d.prefix, "cbo");
    assert!(d.supports("egui"));
    assert!(d.supports("makepad"));
    assert!(d.props.iter().any(|p| p.key == "items"));
    assert!(d.props.iter().any(|p| p.key == "item_index"));
    assert_eq!(
        d.props
            .iter()
            .find(|p| p.key == "item_index")
            .map(|p| &p.default),
        Some(&Value::Int(-1))
    );
}

#[test]
fn combo_box_and_list_box_have_sorted() {
    let r = Registry::builtin();
    for class in ["ListBox", "ComboBox"] {
        let d = r
            .get(class)
            .unwrap_or_else(|| panic!("{class} in registry"));
        let p = d
            .prop("sorted")
            .unwrap_or_else(|| panic!("{class} declares sorted"));
        assert_eq!(p.kind, PropKind::Bool);
        assert_eq!(p.default, Value::Bool(false));
    }
}

#[test]
fn combo_box_has_drop_down_count_defaulting_to_eight() {
    let r = Registry::builtin();
    let d = r.get("ComboBox").expect("ComboBox in registry");
    let p = d
        .prop("drop_down_count")
        .expect("ComboBox declares drop_down_count");
    assert_eq!(p.kind, PropKind::Int { min: 1, max: 100 });
    assert_eq!(p.default, Value::Int(8));
}

#[test]
fn combo_box_has_style_defaulting_to_drop_down_list() {
    let r = Registry::builtin();
    let d = r.get("ComboBox").expect("ComboBox in registry");
    let p = d.prop("style").expect("ComboBox declares style");
    assert_eq!(p.kind, PropKind::Enum(COMBO_BOX_STYLES));
    assert_eq!(p.default, Value::Enum("DropDownList".into()));
}

#[test]
fn combo_box_has_text_defaulting_to_empty() {
    let r = Registry::builtin();
    let d = r.get("ComboBox").expect("ComboBox in registry");
    let p = d.prop("text").expect("ComboBox declares text");
    assert_eq!(p.kind, PropKind::String);
    assert_eq!(p.default, Value::Str(String::new()));
}

#[test]
fn standard_palette_has_the_nine_of_the_day() {
    let r = Registry::builtin();
    for class in [
        "Label",
        "Button",
        "Edit",
        "CheckBox",
        "RadioButton",
        "ComboBox",
        "ListBox",
        "Memo",
        "TreeView",
    ] {
        let d = r
            .get(class)
            .unwrap_or_else(|| panic!("{class} is not in registry"));
        assert!(d.supports("egui"), "{class} must serve egui target");
        assert!(d.supports("makepad"), "{class} must serve makepad target");
        assert!(d.visual, "{class} is visual");
    }
}

#[test]
fn focus_recipients_have_tab_order_and_non_recipients_do_not() {
    let r = Registry::builtin();
    let has = |c: &str| r.get(c).unwrap().props.iter().any(|p| p.key == "tab_order");
    assert!(has("Button") && has("Edit") && has("CheckBox") && has("ListBox"));
    assert!(!has("Label"));
}

#[test]
fn props_catalog_provides_correct_typing() {
    let cap = Props::caption("Save");
    assert_eq!(cap.key, "text");
    assert_eq!(cap.display, "Caption");
    assert_eq!(cap.kind, PropKind::String);

    let en = Props::enabled(true);
    assert_eq!(en.kind, PropKind::Bool);

    let col = Props::color(crawrad_ir::Color::rgb(255, 0, 128));
    assert_eq!(col.kind, PropKind::Color);

    let marg = Props::margins(crawrad_ir::Edges::uniform(4.0));
    assert_eq!(marg.kind, PropKind::Edges);
}

#[test]
fn all_components_use_props_with_valid_propkind() {
    let r = Registry::builtin();
    for d in r.all() {
        for p in &d.props {
            match p.kind {
                PropKind::String => assert!(matches!(p.default, Value::Str(_))),
                PropKind::Multiline => {
                    assert!(matches!(p.default, Value::Str(_)));
                    assert!(p.multiline);
                }
                PropKind::Bool => assert!(matches!(p.default, Value::Bool(_))),
                PropKind::Int { min, max } => {
                    assert!(matches!(p.default, Value::Int(_)));
                    assert!(min <= max);
                }
                PropKind::Float { min, max, step } => {
                    assert!(matches!(p.default, Value::Float(_)));
                    assert!(min <= max && step > 0.0);
                }
                PropKind::Color => assert!(matches!(p.default, Value::Color(_))),
                // **Only `Value::Edges`, and this TIGHTENED on 18/09/2026.**
                //
                // It used to accept `Value::Map(_)` too, because `anchors` — four
                // booleans — declared `Edges`, which are four NUMBERS. The
                // test had been relaxed to accommodate the lie, and with it the
                // piece label no longer decided the editor: the Inspector
                // disambiguated by the pair `(kind, value)`.
                //
                // With `PropKind::Set` (ADR 0037), each answers for itself.
                PropKind::Edges => assert!(matches!(p.default, Value::Edges(_))),
                PropKind::Set(flags) => {
                    assert!(!flags.is_empty(), "a `Set` without flags edits nothing");
                    let Value::Map(m) = &p.default else {
                        panic!("a `Set` default is a boolean map, and is {:?}", p.default)
                    };
                    for f in flags {
                        assert!(
                            matches!(m.get(*f), Some(Value::Bool(_))),
                            "the flag `{f}` is not in default, or is not boolean"
                        );
                    }
                }
                PropKind::Enum(opts) => {
                    assert!(!opts.is_empty());
                    let Value::Enum(default) = &p.default else {
                        panic!("enum default is not Value::Enum for {}", p.key);
                    };
                    assert!(
                        opts.contains(&default.as_str()),
                        "the default `{default}` is not in options {opts:?} for {}",
                        p.key
                    );
                }
                PropKind::Collection {
                    hierarchical,
                    item_class,
                } => {
                    assert!(matches!(p.default, Value::List(_)));
                    if hierarchical {
                        assert!(p.tree_collection);
                    }
                    if let Some(item_cls) = item_class {
                        assert!(!item_cls.is_empty());
                    }
                }
                PropKind::Reference(cls) => {
                    assert!(!cls.is_empty());
                    assert!(matches!(p.default, Value::Ref(_)));
                }
                // The sibling it names must exist and be a reference, or the
                // Inspector has no dataset to list the fields of.
                PropKind::FieldOf(source) => {
                    assert!(matches!(p.default, Value::Str(_)));
                    assert!(
                        d.props
                            .iter()
                            .any(|q| q.key == source && matches!(q.kind, PropKind::Reference(_))),
                        "`{}` names `{source}`, which is not a reference of `{}`",
                        p.key,
                        d.class
                    );
                }
                PropKind::Rect => assert!(matches!(p.default, Value::Rect(_))),
                PropKind::DateTime => assert!(matches!(p.default, Value::DateTime(_))),
                PropKind::FilePath => assert!(matches!(p.default, Value::FilePath(_))),
                PropKind::Cursor => assert!(matches!(p.default, Value::Cursor(_))),
                PropKind::ModalResult => assert!(matches!(p.default, Value::ModalResult(_))),
                PropKind::Shortcut => assert!(matches!(p.default, Value::Shortcut(_))),
                PropKind::Font => assert!(matches!(p.default, Value::Font(_))),
                PropKind::Picture => assert!(matches!(p.default, Value::Picture(_))),
            }
        }
    }
}

#[test]
fn tab_window_computes_visible_slice_and_arrows() {
    // Fits -> no arrows
    let w = tab_window(320.0, 3, 0, false);
    assert_eq!(
        w,
        TabWindow {
            first: 0,
            count: 3,
            arrows: false,
        }
    );

    // 320-wide with plus and 6 tabs -> arrows, count = floor((320-22-32)/92)=2
    let w0 = tab_window(320.0, 6, 0, true);
    assert_eq!(
        w0,
        TabWindow {
            first: 0,
            count: 2,
            arrows: true,
        }
    );

    let w5 = tab_window(320.0, 6, 5, true);
    assert_eq!(
        w5,
        TabWindow {
            first: 4,
            count: 2,
            arrows: true,
        }
    );

    let w3 = tab_window(320.0, 6, 3, true);
    assert_eq!(
        w3,
        TabWindow {
            first: 2,
            count: 2,
            arrows: true,
        }
    );

    // Tiny width -> count 1
    let wtiny = tab_window(10.0, 5, 2, true);
    assert_eq!(
        wtiny,
        TabWindow {
            first: 2,
            count: 1,
            arrows: true,
        }
    );

    // Shown 0 -> count 0, no arrows
    let wempty = tab_window(320.0, 0, 0, true);
    assert_eq!(
        wempty,
        TabWindow {
            first: 0,
            count: 0,
            arrows: false,
        }
    );

    assert_eq!(tab_scroll_left_x(320.0, true), 320.0 - 22.0 - 32.0);
    assert_eq!(tab_scroll_left_x(320.0, false), 320.0 - 32.0);
}

#[test]
fn a_grid_can_point_at_a_data_engine_and_a_table() {
    let grid = components::grid();
    let source = grid.prop("data_source").expect("Grid declares data_source");
    assert_eq!(source.display, "DataSource");
    // a capability, not a class: a Connection and a Dataset both answer (ADR 0040)
    assert_eq!(source.ref_class, Some(props::ROW_SOURCE));
    assert_eq!(source.kind, PropKind::Reference("DbRowSource"));
    assert_eq!(source.default, Value::Ref(String::new()));
    assert_eq!(source.category, Category::Data);

    let table = grid.prop("table_name").expect("Grid declares table_name");
    assert_eq!(table.display, "TableName");
    assert_eq!(table.kind, PropKind::String);
    assert_eq!(table.default, Value::Str(String::new()));
    assert_eq!(table.category, Category::Data);
    assert_eq!(table.hidden_when, Some(("data_source", "DbDataset")));

    let rows = grid
        .prop("design_rows")
        .expect("Grid declares design_rows (#925)");
    assert_eq!(rows.category, Category::Data);
    assert_eq!(rows.default, Value::Enum("8".into()));
    assert_eq!(rows.options, ["Off", "8", "20", "50"]);
}

#[test]
fn a_grid_has_multiline_cell_text_property() {
    let grid = components::grid();
    let prop = grid.prop("cell_text").expect("Grid declares cell_text");
    assert_eq!(prop.display, "Cells");
    assert_eq!(prop.kind, PropKind::Multiline);
    assert_eq!(prop.default, Value::Str(String::new()));
    assert_eq!(prop.category, Category::Appearance);
    assert!(prop.multiline);
}

#[test]
fn a_grid_has_editing_property() {
    let grid = components::grid();
    let prop = grid.prop("editing").expect("Grid declares editing");
    assert_eq!(prop.display, "Editing");
    assert_eq!(prop.kind, PropKind::Bool);
    assert_eq!(prop.default, Value::Bool(false));
    assert_eq!(prop.category, Category::Behavior);
}

#[test]
fn a_grid_declares_on_select_cell() {
    let grid = components::grid();
    let ev = grid
        .events
        .iter()
        .find(|e| e.name == "on_select_cell")
        .expect("Grid declares on_select_cell");
    assert_eq!(ev.display, "OnSelectCell");
    assert_eq!(ev.signature, "fn(&mut self, col: usize, row: usize)");

    let click = grid
        .events
        .iter()
        .find(|e| e.name == "on_click")
        .expect("Grid declares on_click");
    assert_eq!(click.display, "OnClick");
    assert_eq!(click.signature, "fn(&mut self)");

    let dbl = grid
        .events
        .iter()
        .find(|e| e.name == "on_dbl_click")
        .expect("Grid declares on_dbl_click");
    assert_eq!(dbl.display, "OnDblClick");
    assert_eq!(dbl.signature, "fn(&mut self)");
}

/// A container that never takes the focus has no `TabStop`.
#[test]
fn containers_that_never_take_focus_have_no_tab_stop() {
    let r = Registry::builtin();
    for class in ["Panel", "GroupBox", "ScrollBox", "ToolBar"] {
        let d = r
            .get(class)
            .unwrap_or_else(|| panic!("{class} in registry"));
        assert!(
            d.prop("tab_stop").is_none(),
            "{class} must not declare `tab_stop` (#139)"
        );
        assert!(
            d.prop("tab_order").is_some(),
            "{class} must keep `tab_order`"
        );
    }
    let edit = r.get("Edit").unwrap();
    assert!(edit.prop("tab_stop").is_some());
    assert!(edit.prop("tab_order").is_some());
}

#[test]
fn form_declares_theme_property() {
    let r = Registry::builtin();
    let form = r.get("Form").expect("Form in registry");
    let theme = form.prop("theme").expect("Form declares theme");
    assert_eq!(theme.display, "Theme");
    assert_eq!(theme.category, Category::Appearance);
    assert_eq!(theme.options, &["thSystem", "thLight", "thDark"]);
    assert_eq!(theme.default, Value::Enum("thSystem".to_string()));
    assert_eq!(
        theme.kind,
        PropKind::Enum(&["thSystem", "thLight", "thDark"])
    );
}

/// A box of the `Edit` family opens with its name as its text, but a masked
/// one opens empty: its name would be shaped into the mask's slots, and
/// `mask_edit1` used to open as `(1_) _____-____`.
#[test]
fn a_masked_box_is_born_empty_and_an_edit_with_its_name() {
    let r = Registry::builtin();
    let at = Rect::new(0.0, 0.0, 120.0, 24.0);
    let text = |class: &str, id: &str| {
        let n = r.instantiate(class, id, at).expect("a native class");
        n.props.get("text").cloned()
    };
    assert_eq!(text("Edit", "edit1"), Some(Value::Str("edit1".into())));
    assert_eq!(
        text("MaskEdit", "mask_edit1"),
        Some(Value::Str(String::new()))
    );
}

#[test]
fn a_grid_accepts_grid_columns() {
    let r = Registry::builtin();
    let grid = r.get("Grid").expect("Grid in registry");
    assert_eq!(grid.item_class(), Some("GridColumn"));
    assert!(grid.accepts_child("GridColumn"));
    assert!(!grid.accepts_child("Button"));

    let col = r.get("GridColumn").expect("GridColumn in registry");
    assert!(!col.visual);
    assert!(!col.emits_code);
    assert_eq!(col.palette, "");
    assert_eq!(col.prefix, "gdc");
    assert_eq!(col.base_name, "grid_column");
    assert!(col.prop("field").is_some());
    assert!(col.prop("display_format").is_some());
    assert!(col.prop("title").is_some());
    assert!(col.prop("width").is_some());
    assert!(col.prop("alignment").is_some());
    assert!(col.prop("visible").is_some());
}

/// THE OBJECT INSPECTOR MUST LIST DIFFERENT PROPERTIES PER FRAMEWORK.
///
/// The filtering already worked — the Inspector calls
/// `props_for(generator::target_key())` — but there was almost nothing to filter:
/// exactly ONE per-target property existed in the whole registry
/// (`makepad.look`). So the mechanism was real and the shelf was empty.
///
/// `slint.border-radius` is the first of Slint's own: a first-class property of
/// Slint's `Rectangle`, which Makepad and egui express differently or not at all.
#[test]
fn slints_own_rounding_is_offered_to_slint_and_withheld_from_the_others() {
    let r = crate::Registry::builtin();
    let carry_it = [
        "CheckListBox",
        "Chips",
        "ColorBox",
        "DateTimePicker",
        "Grid",
        "ListView",
        "MultiSelect",
        "Pagination",
        "ProgressBar",
        "RadioButton",
        "TreeView",
    ];
    for class in carry_it {
        let d = r
            .get(class)
            .unwrap_or_else(|| panic!("{class} must be in the catalogue"));
        let keys_for = |alvo: &str| -> Vec<&str> { d.props_for(alvo).map(|p| p.key).collect() };
        assert!(
            keys_for("slint").contains(&"slint.border-radius"),
            "{class}: the Inspector must offer it when Slint is generating"
        );
        for other in ["makepad", "egui"] {
            assert!(
                !keys_for(other).contains(&"slint.border-radius"),
                "{class}: {other} does not honour it, so offering it would be a lie"
            );
        }
        // AND THE LISTS MUST ACTUALLY DIFFER. Asserting one key in and one key out
        // would still pass if `props_for` returned everything for every target.
        assert_ne!(
            keys_for("slint").len(),
            keys_for("makepad").len(),
            "{class}: the two targets' property lists came out the same length"
        );
    }
}

/// Registering a property must not change what the IDE already produces.
///
/// Its default is the number the Slint target emits for that component today, so
/// a form that nobody touched generates byte for byte what it generated before.
#[test]
fn the_rounding_defaults_to_what_slint_already_emits() {
    let r = crate::Registry::builtin();
    for (class, expected) in [
        ("Chips", 12i64),
        ("ProgressBar", 3),
        ("RadioButton", 8),
        ("TreeView", 4),
    ] {
        let d = r.get(class).expect("in the catalogue");
        let p = d
            .props
            .iter()
            .find(|p| p.key == "slint.border-radius")
            .unwrap_or_else(|| panic!("{class} carries no slint.border-radius"));
        assert_eq!(
            p.default,
            crawrad_ir::Value::Int(expected),
            "{class}: the default must be what the generator hardcodes today"
        );
    }
}
