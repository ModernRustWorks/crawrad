use super::*;

#[test]
fn test_target_and_classes_consistency() {
    let rt = MakepadRuntime::new();
    assert_eq!(rt.target(), "makepad");
    for class_name in [
        "Button",
        "CheckBox",
        "RadioButton",
        "Edit",
        "GroupBox",
        "Label",
        "PageControl",
        "TabSheet",
        "Panel",
        "ComboBox",
        "ListBox",
        "Memo",
        "TreeView",
        "ScrollBar",
        "ScrollBox",
    ] {
        assert!(
            rt.classes().contains(&class_name),
            "{class_name} must be in Makepad classes()"
        );
    }
}

#[test]
fn test_lifecycle_open_update_close() {
    let mut doc = Document::new("MainForm", "Form");
    let mut btn_node = crawrad_ir::Node::new("btn_ok", "Button");
    btn_node.set_rect(Rect::new(10.0, 20.0, 100.0, 32.0));
    btn_node
        .props
        .insert("text".into(), Value::Str("Confirm".into()));
    doc.add_child("MainForm", btn_node).unwrap();

    let mut rt = MakepadRuntime::new();
    assert!(rt.open(&doc).is_ok());
    assert_eq!(rt.quantidade(), 1);

    let ctl = rt.ctl("btn_ok").expect("button present");
    assert_eq!(ctl.class, "Button");
    assert_eq!(ctl.text, "Confirm");
    assert_eq!(ctl.rect.w, 100.0);

    // Incremental update
    let mut touched = BTreeSet::new();
    doc.node_mut("btn_ok")
        .unwrap()
        .props
        .insert("text".into(), Value::Str("Save".into()));
    touched.insert("btn_ok".to_string());
    assert!(rt.update(&doc, &touched).is_ok());

    assert_eq!(rt.ctl("btn_ok").unwrap().text, "Save");

    rt.close();
    assert_eq!(rt.quantidade(), 0);
}

#[test]
fn test_common_properties_inheritance() {
    let mut doc = Document::new("MainForm", "Form");

    // Parent panel with coordinates
    let mut pnl = crawrad_ir::Node::new("pnl_footer", "Panel");
    pnl.set_rect(Rect::new(0.0, 300.0, 640.0, 60.0));
    pnl.props
        .insert("hint".into(), Value::Str("Action Bar".into()));
    doc.add_child("MainForm", pnl).unwrap();

    // Child button inside panel
    let mut btn = crawrad_ir::Node::new("btn_save", "Button");
    btn.set_rect(Rect::new(10.0, 10.0, 100.0, 32.0));
    btn.props.insert("text".into(), Value::Str("Save".into()));
    doc.add_child("pnl_footer", btn).unwrap();

    let mut rt = MakepadRuntime::new();
    assert!(rt.open(&doc).is_ok());
    assert_eq!(rt.quantidade(), 2);

    let ctl_pnl = rt.ctl("pnl_footer").expect("panel");
    assert_eq!(ctl_pnl.children, vec!["btn_save".to_string()]);
    assert_eq!(
        ctl_pnl.props.get("hint"),
        Some(&Value::Str("Action Bar".into()))
    );

    // Direct access via inheritance / Deref
    let ctl_btn = rt.ctl("btn_save").expect("child button");
    assert_eq!(ctl_btn.id, "btn_save");
    assert_eq!(ctl_btn.class, "Button");
    assert_eq!(ctl_btn.text, "Save");
    assert_eq!(ctl_btn.parent, "pnl_footer");
    assert!(ctl_btn.visible);
    assert!(ctl_btn.enabled);
}

#[test]
fn test_toggle_design_and_live_mode() {
    let mut rt = MakepadRuntime::new();
    assert_eq!(rt.modo, Mode::Design);
    rt.set_mode(Mode::Live);
    assert_eq!(rt.modo, Mode::Live);
    rt.set_mode(Mode::Design);
    assert_eq!(rt.modo, Mode::Design);
}

#[test]
fn test_makepad_reads_document_properties() {
    let mut doc = Document::new("Form1", "Form");

    // ListBox with items and selection
    let mut lst = crawrad_ir::Node::new("ListBox1", "ListBox");
    lst.set_rect(Rect::new(10.0, 10.0, 120.0, 80.0));
    lst.props.insert(
        "items".into(),
        crawrad_ir::Item::list_to(&[
            crawrad_ir::Item::new("Item1"),
            crawrad_ir::Item::new("Item2"),
            crawrad_ir::Item::new("Item3"),
        ]),
    );
    lst.props.insert("item_index".into(), Value::Int(1));
    doc.add_child("Form1", lst).unwrap();

    // CheckBox checked
    let mut chk = crawrad_ir::Node::new("CheckBox1", "CheckBox");
    chk.set_rect(Rect::new(10.0, 100.0, 100.0, 20.0));
    chk.props.insert("checked".into(), Value::Bool(true));
    doc.add_child("Form1", chk).unwrap();

    // PageControl with active_page
    let mut pgc = crawrad_ir::Node::new("PageControl1", "PageControl");
    pgc.set_rect(Rect::new(10.0, 130.0, 200.0, 150.0));
    pgc.props.insert("active_page".into(), Value::Int(2));
    doc.add_child("Form1", pgc).unwrap();

    let mut rt = MakepadRuntime::new();
    assert!(rt.open(&doc).is_ok());

    let ctl_lst = rt.ctl("ListBox1").expect("ListBox");
    assert_eq!(ctl_lst.itens.len(), 3);
    assert_eq!(ctl_lst.itens[0].text, "Item1");
    assert_eq!(ctl_lst.escolhido, Some(1));

    let ctl_chk = rt.ctl("CheckBox1").expect("CheckBox");
    assert!(ctl_chk.marcado);

    let ctl_pgc = rt.ctl("PageControl1").expect("PageControl");
    assert_eq!(ctl_pgc.pagina_ativa, 2);
}

#[test]
fn test_synchronize_non_visual_components() {
    let mut doc = Document::new("Form1", "Form");
    let acl = crawrad_ir::Node::new("ActionList1", "ActionList");
    doc.add_child("Form1", acl).unwrap();

    let iml = crawrad_ir::Node::new("ImageList1", "ImageList");
    doc.add_child("Form1", iml).unwrap();

    let mut rt = MakepadRuntime::new();
    assert!(rt.open(&doc).is_ok());

    let ctl_acl = rt.ctl("ActionList1").expect("ActionList present");
    assert!(!ctl_acl.visual);
    assert_eq!(ctl_acl.class, "ActionList");

    let ctl_iml = rt.ctl("ImageList1").expect("ImageList present");
    assert!(!ctl_iml.visual);
    assert_eq!(ctl_iml.class, "ImageList");
}

#[test]
fn test_makepad_supports_grid() {
    let mut rt = MakepadRuntime::new();
    rt.set_grid(2.0);
    assert_eq!(rt.grid, 2.0);

    rt.set_grid(0.0);
    assert_eq!(rt.grid, 0.0);

    rt.set_grid(8.0);
    assert_eq!(rt.grid, 8.0);
}

#[test]
fn test_listbox_and_memo_scroll_support() {
    let mut doc = Document::new("Form1", "Form");
    let mut lst = crawrad_ir::Node::new("ListBox1", "ListBox");
    lst.props.insert(
        "items".into(),
        crawrad_ir::Item::list_to(&[
            crawrad_ir::Item::new("Item1"),
            crawrad_ir::Item::new("Item2"),
            crawrad_ir::Item::new("Item3"),
        ]),
    );
    doc.add_child("Form1", lst).unwrap();

    let mut mmo = crawrad_ir::Node::new("Memo1", "Memo");
    mmo.props.insert(
        "scroll_bars".into(),
        crawrad_ir::Value::Enum("ssVertical".into()),
    );
    doc.add_child("Form1", mmo).unwrap();

    let mut rt = MakepadRuntime::new();
    rt.open(&doc).unwrap();

    let ctl_lst = rt.ctl("ListBox1").expect("ListBox");
    assert_eq!(ctl_lst.itens.len(), 3);
    assert_eq!(ctl_lst.scroll_y, 0.0);

    let ctl_mmo = rt.ctl("Memo1").expect("Memo");
    assert_eq!(
        ctl_mmo.props.get("scroll_bars"),
        Some(&crawrad_ir::Value::Enum("ssVertical".into()))
    );
}

#[test]
fn test_designer_cursor_mapping() {
    use crawrad_runtime::Axis;
    use makepad_widgets::MouseCursor;

    assert_eq!(
        designer::cursor_para_eixo(Axis::Nwse),
        MouseCursor::NwseResize
    );
    assert_eq!(
        designer::cursor_para_eixo(Axis::Nesw),
        MouseCursor::NeswResize
    );
    assert_eq!(designer::cursor_para_eixo(Axis::We), MouseCursor::EwResize);
    assert_eq!(designer::cursor_para_eixo(Axis::Ns), MouseCursor::NsResize);
}

#[test]
fn test_grid_dot_calculations() {
    let rect = Rect::new(0.0, 0.0, 100.0, 100.0);
    let points = designer::calculate_grid_points(rect, 20.0);
    assert!(!points.is_empty());
    assert_eq!(points[0], (20.0, 20.0));
}

/// **This target can namespace properties** — its name is in the contract.
///
/// # The bug this gate guards against, and it was costly
///
/// [ADR 0031] measured: while `makepad` was not in `contract::TARGETS`,
/// `PropKey::parse("makepad.x")` returned `UnknownTarget` and `validate()`
/// failed. **Every property specific to this adapter was pushed to the
/// common core or nowhere** — and three of them (`horz_scroll_bar`,
/// `vert_scroll_bar`, `border_width`) remain in `COMMON` to this day, read only by
/// files here, violating the law of ADR 0002 while the suite was green.
///
/// The test is DERIVED: it reads the constant published by this crate and the constant
/// published by the contract, and checks both. A new target that forgets to
/// register fails here, in its crate, rather than failing months later in
/// the form of a property in the wrong place.
///
#[test]
fn o_nome_deste_alvo_esta_no_contrato() {
    assert!(
        crawrad_ir::contract::TARGETS.contains(&crate::ALVO),
        "`{}` is not in `contract::TARGETS` ({:?}). Without it, every \
         property `{}.x` is rejected by `validate()` and there is nowhere to declare \
         what only this target honors",
        crate::ALVO,
        crawrad_ir::contract::TARGETS,
        crate::ALVO
    );
}

#[test]
fn an_inheriting_class_is_painted_as_its_base() {
    let mut doc = Document::new("MainForm", "Form");
    let mut price = crawrad_ir::Node::new("ped1", "PriceEdit");
    price.set_rect(Rect::new(10.0, 20.0, 121.0, 24.0));
    doc.add_child("MainForm", price).unwrap();
    let mut btn = crawrad_ir::Node::new("btn1", "Button");
    btn.set_rect(Rect::new(10.0, 60.0, 75.0, 25.0));
    doc.add_child("MainForm", btn).unwrap();

    let mut rt = MakepadRuntime::new();
    let mut looks = crawrad_ir::drawing::DeclaredLooks::default();
    looks.bases.insert("PriceEdit".into(), "SpinEdit".into());
    rt.com_desenhos(looks);
    assert!(rt.open(&doc).is_ok());

    assert_eq!(rt.ctl("ped1").expect("PriceEdit placed").class, "SpinEdit");
    assert_eq!(rt.ctl("btn1").expect("Button placed").class, "Button");
    // The document keeps the user's class: only the painting borrows the base.
    assert_eq!(doc.node("ped1").unwrap().class, "PriceEdit");
}
