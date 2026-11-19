use super::*;
use crawrad_ir::{Color, Node, Rect};

const THEME: Theme = Theme {
    text: Color::rgb(0xD9, 0xD9, 0xDB),
    surface: Color::rgb(0x2B, 0x2E, 0x31),
};

fn add(doc: &mut Document, parent: &str, id: &str, class: &str, r: Rect) {
    let mut n = Node::new(id, class);
    n.set_rect(r);
    doc.nodes.get_mut(parent).unwrap().children.push(id.into());
    doc.nodes.insert(id.into(), n);
}

/// An overlap (error), a button with no OnClick and a tied Tab stop.
fn form() -> Document {
    let mut doc = Document::new("form1", "Form");
    doc.nodes
        .get_mut("form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 410.0));
    add(
        &mut doc,
        "form1",
        "save",
        "Button",
        Rect::new(0.0, 0.0, 100.0, 30.0),
    );
    add(
        &mut doc,
        "form1",
        "cancel",
        "Button",
        Rect::new(50.0, 10.0, 100.0, 30.0),
    );
    for id in ["save", "cancel"] {
        let n = doc.nodes.get_mut(id).unwrap();
        n.props.insert("tab_order".into(), Value::Int(0));
    }
    doc.nodes
        .get_mut("save")
        .unwrap()
        .events
        .insert("on_click".into(), "save_click".into());
    doc
}

fn panel(doc: &Document) -> CheckPanel {
    let mut p = CheckPanel::default();
    assert!(p.refresh(doc, &Registry::builtin(), THEME, "form1", 1));
    p
}

#[test]
fn the_head_counts_and_offers_fix_all_for_what_needs_no_code() {
    let doc = form();
    let p = panel(&doc);
    assert_eq!(p.summary(), "1 error \u{00b7} 2 warnings");
    let v = p.view();
    assert_eq!(
        v.chips,
        [("Errors 1".into(), true), ("Warnings 2".into(), true)]
    );
    assert_eq!(v.note, "Checked on every change \u{00b7} 1 fixable");
    assert_eq!(v.fix_all.as_deref(), Some("Fix all 1"));
    let rules: Vec<_> = v.rows.iter().map(|r| (r.number, r.error, r.rule)).collect();
    assert_eq!(
        rules,
        [
            (1, true, "Overlap"),
            (2, false, "Tab order"),
            (3, false, "No OnClick")
        ]
    );
    assert_eq!(v.rows[0].fix, None);
    assert_eq!(v.rows[2].fix.as_deref(), Some("Add handler"));
}

#[test]
fn the_same_revision_is_not_checked_twice() {
    let doc = form();
    let mut p = panel(&doc);
    assert!(!p.refresh(&doc, &Registry::builtin(), THEME, "form1", 1));
    assert!(
        !p.refresh(&doc, &Registry::builtin(), THEME, "form1", 2),
        "same findings"
    );
    p.clear();
    assert_eq!(p.summary(), "No problems");
}

#[test]
fn a_chip_hides_its_kind_and_rows_keep_their_numbers() {
    let doc = form();
    let mut p = panel(&doc);
    assert!(matches!(
        p.input(CheckInput::Chip(0), &doc),
        CheckEffect::Paint
    ));
    let v = p.view();
    assert_eq!(v.chips[0], ("Errors 1".into(), false));
    let numbers: Vec<_> = v.rows.iter().map(|r| r.number).collect();
    assert_eq!(numbers, [2, 3]);
    let CheckEffect::Select(node) = p.input(CheckInput::Row(1), &doc) else {
        panic!()
    };
    assert_eq!(node, "cancel");
    assert!(p.view().rows[1].selected);
}

#[test]
fn a_fix_applies_or_binds_and_fix_all_is_one_step() {
    let doc = form();
    let mut p = panel(&doc);
    assert!(
        matches!(p.input(CheckInput::Fix(0), &doc), CheckEffect::Paint),
        "overlap: none"
    );
    let CheckEffect::Bind { node, event, .. } = p.input(CheckInput::Fix(2), &doc) else {
        panic!()
    };
    assert_eq!((node.as_str(), event.as_str()), ("cancel", "on_click"));
    let CheckEffect::Apply(tx) = p.input(CheckInput::FixAll, &doc) else {
        panic!()
    };
    let mut d = crawrad_doc::Doc::new(doc);
    d.apply(tx).unwrap();
    assert!(p.refresh(d.ir(), &Registry::builtin(), THEME, "form1", 2));
    assert_eq!(p.summary(), "1 error \u{00b7} 1 warning");
}

#[test]
fn pins_mark_each_finding_and_the_tied_stops() {
    let doc = form();
    let p = panel(&doc);
    let pins: Vec<_> = p
        .pins()
        .into_iter()
        .map(|p| (p.node, p.number, p.kind))
        .collect();
    assert_eq!(
        pins,
        [
            ("cancel".into(), 1, PinKind::Error),
            ("cancel".into(), 2, PinKind::Warning),
            ("cancel".into(), 2, PinKind::TabClash),
            ("save".into(), 2, PinKind::TabClash),
            ("cancel".into(), 3, PinKind::Warning),
        ]
    );
    assert_eq!(p.props_of("cancel"), [("rect", true), ("tab_order", false)]);
    assert_eq!(
        tab_badges(&doc),
        [("save".into(), "0".into()), ("cancel".into(), "0".into())]
    );
}

#[test]
fn mcp_reads_the_findings_by_control_name() {
    let mut doc = form();
    doc.nodes
        .get_mut("cancel")
        .unwrap()
        .set_name("ButtonCancel");
    let p = panel(&doc);
    let json = p.to_json(&doc);
    assert_eq!(json["summary"], "1 error \u{00b7} 2 warnings");
    assert_eq!(json["findings"][2]["controls"][0], "ButtonCancel");
    assert_eq!(json["findings"][2]["fix"], "Add handler");
    assert_eq!(json["findings"][0]["severity"], "error");
}

#[test]
fn tip_reads_finding_text_severity_and_fix_label() {
    let doc = form();
    let p = panel(&doc);
    assert_eq!(p.tip(0), None);
    assert_eq!(p.tip(4), None);
    let (text, error, fix) = p.tip(1).unwrap();
    assert!(error);
    assert!(!text.is_empty());
    assert_eq!(fix, None);

    let (_, error, fix) = p.tip(3).unwrap();
    assert!(!error);
    assert_eq!(fix.as_deref(), Some("Add handler"));
}
