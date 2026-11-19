use super::*;
use crawrad_ir::{Font, ModalResult};

const THEME: Theme = Theme {
    text: Color::rgb(0xD9, 0xD9, 0xDB),
    surface: Color::rgb(0x2B, 0x2E, 0x31),
};

fn form() -> Document {
    let mut doc = Document::new("form1", "Form");
    doc.nodes
        .get_mut("form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 410.0));
    doc
}

fn add<'a>(doc: &'a mut Document, parent: &str, id: &str, class: &str, r: Rect) -> &'a mut Node {
    let mut n = Node::new(id, class);
    n.set_rect(r);
    doc.nodes.get_mut(parent).unwrap().children.push(id.into());
    doc.nodes.insert(id.into(), n);
    doc.nodes.get_mut(id).unwrap()
}

fn run(doc: &Document) -> Vec<Finding> {
    check(doc, &Registry::builtin(), THEME)
}

fn rules(doc: &Document) -> Vec<Rule> {
    run(doc).iter().map(|f| f.rule).collect()
}

fn wired(n: &mut Node) {
    n.events.insert("on_click".into(), "x_click".into());
}

#[test]
fn a_tidy_form_has_nothing_to_say() {
    let mut doc = form();
    wired(add(
        &mut doc,
        "form1",
        "ok",
        "Button",
        Rect::new(10.0, 10.0, 80.0, 24.0),
    ));
    add(
        &mut doc,
        "form1",
        "name",
        "Edit",
        Rect::new(100.0, 10.0, 120.0, 24.0),
    );
    assert_eq!(run(&doc), []);
}

#[test]
fn overlapping_siblings_are_an_error_with_no_fix() {
    let mut doc = form();
    wired(add(
        &mut doc,
        "form1",
        "save",
        "Button",
        Rect::new(400.0, 330.0, 110.0, 32.0),
    ));
    wired(add(
        &mut doc,
        "form1",
        "cancel",
        "Button",
        Rect::new(470.0, 342.0, 110.0, 32.0),
    ));
    let f = run(&doc);
    assert_eq!(f.len(), 1, "{f:?}");
    assert_eq!(f[0].rule, Rule::Overlap);
    assert!(f[0].rule.is_error());
    assert_eq!(f[0].text, "save and cancel overlap by 40 \u{00d7} 20 px");
    assert_eq!(f[0].nodes, ["cancel", "save"]);
    assert_eq!(f[0].fix, None);
}

#[test]
fn touching_or_aligned_or_hidden_controls_do_not_overlap() {
    let mut doc = form();
    wired(add(
        &mut doc,
        "form1",
        "a",
        "Button",
        Rect::new(0.0, 0.0, 80.0, 24.0),
    ));
    wired(add(
        &mut doc,
        "form1",
        "b",
        "Button",
        Rect::new(80.0, 0.0, 80.0, 24.0),
    ));
    let c = add(
        &mut doc,
        "form1",
        "c",
        "Panel",
        Rect::new(0.0, 0.0, 300.0, 40.0),
    );
    c.props.insert("align".into(), Value::Enum("Top".into()));
    let d = add(
        &mut doc,
        "form1",
        "d",
        "Edit",
        Rect::new(10.0, 5.0, 80.0, 24.0),
    );
    d.props.insert("visible".into(), Value::Bool(false));
    assert_eq!(rules(&doc), []);
}

#[test]
fn a_control_past_its_parent_is_fitted_back_inside() {
    let mut doc = form();
    add(
        &mut doc,
        "form1",
        "panel",
        "Panel",
        Rect::new(24.0, 104.0, 300.0, 120.0),
    );
    add(
        &mut doc,
        "panel",
        "edit2",
        "Edit",
        Rect::new(150.0, 44.0, 220.0, 28.0),
    );
    let f = run(&doc);
    assert_eq!(f.len(), 1, "{f:?}");
    assert_eq!(f[0].text, "edit2 ends 70 px past the right edge of panel");
    assert_eq!(f[0].nodes, ["edit2", "panel"]);
    let fix = f[0].fix.clone().unwrap();
    assert_eq!(fix.label(), "Fit inside");
    assert_eq!(
        fix,
        Fix::FitInside {
            node: "edit2".into(),
            rect: Rect::new(80.0, 44.0, 220.0, 28.0)
        }
    );
    let mut d = crawrad_doc::Doc::new(doc);
    d.apply(fix.transaction(d.ir()).unwrap()).unwrap();
    assert_eq!(run(d.ir()), []);
}

#[test]
fn a_control_bigger_than_its_parent_is_shrunk_to_fit() {
    let mut doc = form();
    add(
        &mut doc,
        "form1",
        "panel",
        "Panel",
        Rect::new(0.0, 0.0, 100.0, 50.0),
    );
    add(
        &mut doc,
        "panel",
        "memo",
        "Memo",
        Rect::new(-10.0, 5.0, 300.0, 30.0),
    );
    let Some(Fix::FitInside { rect, .. }) = run(&doc)[0].fix.clone() else {
        panic!()
    };
    assert_eq!(rect, Rect::new(0.0, 5.0, 100.0, 30.0));
}

fn label(doc: &mut Document, color: Color) {
    let n = add(
        doc,
        "form1",
        "label1",
        "Label",
        Rect::new(24.0, 58.0, 60.0, 20.0),
    );
    n.set_text("Name");
    let font = Font {
        color,
        ..Font::default()
    };
    n.props.insert("font".into(), Value::Font(font));
    n.props.insert("parent_font".into(), Value::Bool(false));
}

#[test]
fn faint_text_fails_wcag_aa_and_gets_a_colour_that_passes() {
    let mut doc = form();
    label(&mut doc, Color::rgb(0x5B, 0x5F, 0x63));
    let f = run(&doc);
    assert_eq!(f.len(), 1, "{f:?}");
    assert_eq!(f[0].rule, Rule::Contrast);
    assert!(!f[0].rule.is_error());
    assert!(
        f[0].text
            .starts_with("label1 text #5B5F63 on #2B2E31 is 2."),
        "{}",
        f[0].text
    );
    assert!(f[0].text.ends_with("WCAG AA needs 4.5:1"));
    let Some(Fix::FontColor { color, .. }) = f[0].fix.clone() else {
        panic!()
    };
    assert!(contrast(color, THEME.surface) >= 4.5);
    assert!(
        contrast(color, THEME.surface) < 4.8,
        "the nearest, not white"
    );
    let mut d = crawrad_doc::Doc::new(doc);
    let fix = f[0].fix.clone().unwrap();
    d.apply(fix.transaction(d.ir()).unwrap()).unwrap();
    assert_eq!(run(d.ir()), []);
}

#[test]
fn contrast_reads_the_parents_colour_and_leaves_the_themes_own_pair_alone() {
    let mut doc = form();
    label(&mut doc, Color::rgb(0xD9, 0xD9, 0xDB));
    assert_eq!(rules(&doc), [], "theme surface");
    doc.nodes
        .get_mut("form1")
        .unwrap()
        .props
        .insert("color".into(), Value::Color(Color::rgb(0xF0, 0xF0, 0xF0)));
    assert_eq!(rules(&doc), [Rule::Contrast], "the form's light colour");
    let mut plain = form();
    add(
        &mut plain,
        "form1",
        "l",
        "Label",
        Rect::new(0.0, 0.0, 50.0, 20.0),
    )
    .set_text("Hi");
    assert_eq!(rules(&plain), []);
}

#[test]
fn large_text_needs_only_three_to_one() {
    assert!((contrast(Color::rgb(0, 0, 0), Color::rgb(255, 255, 255)) - 21.0).abs() < 0.01);
    let mut doc = form();
    label(&mut doc, Color::rgb(0x80, 0x84, 0x88));
    assert_eq!(rules(&doc), [Rule::Contrast]);
    let n = doc.nodes.get_mut("label1").unwrap();
    let Some(Value::Font(f)) = n.props.get_mut("font") else {
        panic!()
    };
    f.size = 18.0;
    assert_eq!(rules(&doc), []);
}

#[test]
fn duplicate_and_unused_tab_stops_are_renumbered_in_their_order() {
    let mut doc = form();
    for (id, t, x) in [
        ("e1", 0, 0.0),
        ("e2", 2, 100.0),
        ("b", 2, 200.0),
        ("c", 4, 300.0),
    ] {
        let n = add(&mut doc, "form1", id, "Edit", Rect::new(x, 0.0, 80.0, 24.0));
        n.props.insert("tab_order".into(), Value::Int(t));
    }
    let f = run(&doc);
    assert_eq!(f.len(), 1, "{f:?}");
    assert_eq!(f[0].rule, Rule::TabOrder);
    assert_eq!(
        f[0].text,
        "e2 and b both have TabOrder 2; 1 and 3 are unused"
    );
    assert_eq!(
        f[0].nodes,
        ["b", "e2"],
        "the late one marked, the tie after"
    );
    let fix = f[0].fix.clone().unwrap();
    assert_eq!(
        fix,
        Fix::Renumber(vec!["e1".into(), "e2".into(), "b".into(), "c".into()])
    );
    let mut d = crawrad_doc::Doc::new(doc);
    d.apply(fix.transaction(d.ir()).unwrap()).unwrap();
    assert_eq!(run(d.ir()), []);
}

#[test]
fn a_gap_inside_a_container_is_fine_a_tie_is_not() {
    let mut doc = form();
    add(
        &mut doc,
        "form1",
        "p",
        "Panel",
        Rect::new(0.0, 0.0, 400.0, 200.0),
    )
    .props
    .insert("tab_order".into(), Value::Int(0));
    for (id, t, x) in [("a", 7, 0.0), ("b", 9, 100.0)] {
        let n = add(&mut doc, "p", id, "Edit", Rect::new(x, 0.0, 80.0, 24.0));
        n.props.insert("tab_order".into(), Value::Int(t));
    }
    assert_eq!(rules(&doc), []);
    doc.nodes
        .get_mut("b")
        .unwrap()
        .props
        .insert("tab_order".into(), Value::Int(7));
    assert_eq!(run(&doc)[0].text, "a and b both have TabOrder 7");
}

#[test]
fn a_button_with_no_click_action_or_modal_result_gets_a_handler() {
    let mut doc = form();
    add(
        &mut doc,
        "form1",
        "cancel",
        "Button",
        Rect::new(0.0, 0.0, 80.0, 24.0),
    )
    .set_name("ButtonCancel");
    let f = run(&doc);
    assert_eq!(f[0].text, "ButtonCancel does nothing when clicked");
    let fix = f[0].fix.clone().unwrap();
    assert_eq!(fix.label(), "Add handler");
    assert!(
        fix.transaction(&doc).is_none(),
        "it writes code: event.bind does it"
    );
    assert_eq!(
        fix,
        Fix::AddHandler {
            node: "cancel".into(),
            event: "on_click".into(),
            handler: crawrad_ir::default_handler_name("ButtonCancel", "on_click"),
        }
    );
    let n = doc.nodes.get_mut("cancel").unwrap();
    n.props.insert(
        "modal_result".into(),
        Value::ModalResult(ModalResult::Cancel),
    );
    assert_eq!(rules(&doc), []);
    let n = doc.nodes.get_mut("cancel").unwrap();
    n.props.remove("modal_result");
    n.props.insert("action".into(), Value::Ref("act1".into()));
    assert_eq!(rules(&doc), []);
}

#[test]
fn a_menu_leaf_without_on_click_is_named_by_its_path() {
    let mut doc = form();
    add(
        &mut doc,
        "form1",
        "menu",
        "MainMenu",
        Rect::new(0.0, 0.0, 28.0, 28.0),
    )
    .set_name("MainMenu");
    add(
        &mut doc,
        "menu",
        "file",
        "MenuItem",
        Rect::new(0.0, 0.0, 0.0, 0.0),
    )
    .set_text("File");
    add(
        &mut doc,
        "file",
        "export",
        "MenuItem",
        Rect::new(0.0, 0.0, 0.0, 0.0),
    )
    .set_text("Export");
    add(
        &mut doc,
        "file",
        "sep",
        "MenuItem",
        Rect::new(0.0, 0.0, 0.0, 0.0),
    )
    .set_text("-");
    let texts: Vec<String> = run(&doc).into_iter().map(|f| f.text).collect();
    assert_eq!(
        texts,
        ["MainMenu \u{203a} File \u{203a} Export has no OnClick"]
    );
}

#[test]
fn errors_come_first() {
    let mut doc = form();
    add(
        &mut doc,
        "form1",
        "lonely",
        "Button",
        Rect::new(0.0, 0.0, 80.0, 24.0),
    );
    add(
        &mut doc,
        "form1",
        "panel",
        "Panel",
        Rect::new(200.0, 0.0, 50.0, 50.0),
    );
    add(
        &mut doc,
        "panel",
        "e",
        "Edit",
        Rect::new(0.0, 0.0, 80.0, 24.0),
    );
    assert_eq!(rules(&doc), [Rule::OutsideParent, Rule::NoOnClick]);
}

#[test]
fn a_label_with_an_edit_to_its_right_suggests_a_link_and_applying_it_clears_the_finding() {
    let mut doc = form();
    add(
        &mut doc,
        "form1",
        "lbl1",
        "Label",
        Rect::new(10.0, 10.0, 50.0, 20.0),
    );
    let edt = add(
        &mut doc,
        "form1",
        "edt1",
        "Edit",
        Rect::new(68.0, 8.0, 100.0, 24.0),
    );
    edt.set_name("EditName");
    let f = run(&doc);
    assert_eq!(f.len(), 1, "{f:?}");
    assert_eq!(f[0].rule, Rule::LabelLink);
    assert!(!f[0].rule.is_error());
    assert_eq!(f[0].nodes, ["lbl1", "edt1"]);
    assert_eq!(f[0].prop, Some("focus_control"));
    assert_eq!(
        f[0].text,
        "lbl1 labels EditName but its FocusControl is empty"
    );
    let fix = f[0].fix.clone().unwrap();
    assert_eq!(fix.label(), "Link to EditName");
    let tx = fix.transaction(&doc).unwrap();
    let mut d = crawrad_doc::Doc::new(doc);
    d.apply(tx).unwrap();
    assert_eq!(
        d.ir().node("lbl1").unwrap().props.get("focus_control"),
        Some(&Value::Ref("edt1".into()))
    );
    assert_eq!(run(d.ir()), []);
}

#[test]
fn a_label_with_an_edit_directly_below_it_is_found() {
    let mut doc = form();
    add(
        &mut doc,
        "form1",
        "lbl1",
        "Label",
        Rect::new(10.0, 10.0, 80.0, 20.0),
    );
    add(
        &mut doc,
        "form1",
        "edt1",
        "Edit",
        Rect::new(10.0, 34.0, 100.0, 24.0),
    );
    let f = run(&doc);
    assert_eq!(f.len(), 1, "{f:?}");
    assert_eq!(f[0].rule, Rule::LabelLink);
    assert_eq!(f[0].nodes, ["lbl1", "edt1"]);
}

#[test]
fn a_label_already_linked_has_no_finding() {
    let mut doc = form();
    let lbl = add(
        &mut doc,
        "form1",
        "lbl1",
        "Label",
        Rect::new(10.0, 10.0, 50.0, 20.0),
    );
    lbl.props
        .insert("focus_control".into(), Value::Ref("edt1".into()));
    add(
        &mut doc,
        "form1",
        "edt1",
        "Edit",
        Rect::new(68.0, 8.0, 100.0, 24.0),
    );
    assert_eq!(rules(&doc), []);
}

#[test]
fn a_label_with_an_edit_far_away_has_no_finding() {
    let mut doc = form();
    add(
        &mut doc,
        "form1",
        "lbl1",
        "Label",
        Rect::new(10.0, 10.0, 50.0, 20.0),
    );
    add(
        &mut doc,
        "form1",
        "edt1",
        "Edit",
        Rect::new(160.0, 8.0, 100.0, 24.0),
    );
    assert_eq!(rules(&doc), []);
}

#[test]
fn a_label_beside_a_checkbox_only_has_no_finding() {
    let mut doc = form();
    add(
        &mut doc,
        "form1",
        "lbl1",
        "Label",
        Rect::new(10.0, 10.0, 50.0, 20.0),
    );
    add(
        &mut doc,
        "form1",
        "chk1",
        "CheckBox",
        Rect::new(68.0, 8.0, 100.0, 24.0),
    );
    assert_eq!(rules(&doc), []);
}

#[test]
fn a_label_beside_two_controls_picks_the_nearest_target() {
    let mut doc = form();
    add(
        &mut doc,
        "form1",
        "lbl1",
        "Label",
        Rect::new(10.0, 10.0, 50.0, 20.0),
    );
    add(
        &mut doc,
        "form1",
        "edt1",
        "Edit",
        Rect::new(80.0, 8.0, 100.0, 24.0),
    );
    add(
        &mut doc,
        "form1",
        "cbo1",
        "ComboBox",
        Rect::new(10.0, 34.0, 100.0, 24.0),
    );
    let f = run(&doc);
    assert_eq!(f.len(), 1, "{f:?}");
    assert_eq!(f[0].rule, Rule::LabelLink);
    assert_eq!(f[0].nodes, ["lbl1", "cbo1"]);
    let fix = f[0].fix.clone().unwrap();
    assert_eq!(fix.label(), "Link to cbo1");
}
