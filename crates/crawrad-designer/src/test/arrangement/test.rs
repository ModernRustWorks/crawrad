use super::*;
use crawrad_doc::Doc;
use crawrad_ir::Node;

/// a(10,10,100,20) · b(50,60,60,40) · c(200,150,80,30) in a 640x420 form
fn doc() -> Doc {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
    for (id, r) in [
        ("a", Rect::new(10.0, 10.0, 100.0, 20.0)),
        ("b", Rect::new(50.0, 60.0, 60.0, 40.0)),
        ("c", Rect::new(200.0, 150.0, 80.0, 30.0)),
    ] {
        let mut n = Node::new(id, "Button");
        n.set_rect(r);
        d.add_child("form_main", n).unwrap();
    }
    Doc::new(d)
}

fn sel() -> Vec<NodeId> {
    vec!["a".into(), "b".into(), "c".into()]
}

fn rect(d: &Doc, id: &str) -> Rect {
    d.ir().node(id).unwrap().rect().unwrap()
}

#[test]
fn align_left_uses_the_first_as_reference() {
    let mut d = doc();
    let tx = align(d.ir(), &sel(), AlignMode::Left, Reference::First).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(rect(&d, "a").x, 10.0, "the first one does not move");
    assert_eq!(rect(&d, "b").x, 10.0);
    assert_eq!(rect(&d, "c").x, 10.0);
    // and nothing else changed
    assert_eq!(rect(&d, "b").y, 60.0);
    assert_eq!(rect(&d, "c").w, 80.0);
}

#[test]
fn align_right_matches_right_edges() {
    let mut d = doc();
    let tx = align(d.ir(), &sel(), AlignMode::Right, Reference::First).unwrap();
    d.apply(tx).unwrap();
    let edge = 110.0; // a: 10 + 100
    for id in ["a", "b", "c"] {
        let r = rect(&d, id);
        assert_eq!(r.x + r.w, edge, "{id} did not touch");
    }
}

#[test]
fn align_center_vertical() {
    let mut d = doc();
    let tx = align(d.ir(), &sel(), AlignMode::CenterVertical, Reference::First).unwrap();
    d.apply(tx).unwrap();
    let center = 20.0; // a: 10 + 20/2
    for id in ["a", "b", "c"] {
        let r = rect(&d, id);
        assert_eq!(r.y + r.h / 2.0, center, "{id} out of center");
    }
}

#[test]
fn align_to_form_moves_all_including_the_first() {
    let mut d = doc();
    let tx = align(d.ir(), &sel(), AlignMode::Left, Reference::Form).unwrap();
    d.apply(tx).unwrap();
    for id in ["a", "b", "c"] {
        assert_eq!(rect(&d, id).x, 0.0, "{id} should touch form edge");
    }
}

#[test]
fn align_with_only_one_selected_does_nothing() {
    let d = doc();
    assert!(align(d.ir(), &["a".into()], AlignMode::Left, Reference::First).is_none());
}

#[test]
fn align_already_aligned_does_not_generate_transaction() {
    let mut d = doc();
    let tx = align(d.ir(), &sel(), AlignMode::Left, Reference::First).unwrap();
    d.apply(tx).unwrap();
    // second time: nothing to do, and nothing should enter the history
    assert!(align(d.ir(), &sel(), AlignMode::Left, Reference::First).is_none());
}

#[test]
fn distribute_makes_gaps_equal_and_endpoints_still() {
    let mut d = doc();
    let tx = distribute(d.ir(), &sel(), Distribute::Horizontal).unwrap();
    d.apply(tx).unwrap();

    let (a, b, c) = (rect(&d, "a"), rect(&d, "b"), rect(&d, "c"));
    assert_eq!(a.x, 10.0, "the left endpoint stays still");
    assert_eq!(c.x, 200.0, "the right endpoint stays still");
    let gap1 = b.x - (a.x + a.w);
    let gap2 = c.x - (b.x + b.w);
    assert!(
        (gap1 - gap2).abs() < 0.01,
        "different gaps: {gap1} and {gap2}"
    );
}

#[test]
fn distribute_needs_three() {
    let d = doc();
    assert!(distribute(d.ir(), &["a".into(), "b".into()], Distribute::Horizontal).is_none());
}

#[test]
fn distribute_vertically_sorts_by_position_and_not_by_selection() {
    let mut d = doc();
    // out of order selection: c, a, b
    let out_of_order: Vec<NodeId> = vec!["c".into(), "a".into(), "b".into()];
    let tx = distribute(d.ir(), &out_of_order, Distribute::Vertical).unwrap();
    d.apply(tx).unwrap();

    let (a, b, c) = (rect(&d, "a"), rect(&d, "b"), rect(&d, "c"));
    assert_eq!(a.y, 10.0);
    assert_eq!(c.y, 150.0);
    let gap1 = b.y - (a.y + a.h);
    let gap2 = c.y - (b.y + b.h);
    assert!((gap1 - gap2).abs() < 0.01, "{gap1} and {gap2}");
}

#[test]
fn equalize_size_uses_the_first_as_model() {
    let mut d = doc();
    let tx = equalize(d.ir(), &sel(), Equalize::Both).unwrap();
    d.apply(tx).unwrap();
    for id in ["b", "c"] {
        let r = rect(&d, id);
        assert_eq!((r.w, r.h), (100.0, 20.0), "{id} did not copy size");
    }
    // position does not change
    assert_eq!(rect(&d, "c").x, 200.0);
}

#[test]
fn equalize_width_only_preserves_height() {
    let mut d = doc();
    let tx = equalize(d.ir(), &sel(), Equalize::Width).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(rect(&d, "b").w, 100.0);
    assert_eq!(rect(&d, "b").h, 40.0, "height was supposed to remain");
}

#[test]
fn center_in_form_moves_the_set_without_altering_spacing() {
    let mut d = doc();
    let before_b = rect(&d, "b");
    let before_c = rect(&d, "c");
    let dist_before = before_c.x - before_b.x;

    let tx = center_in_form(d.ir(), &sel(), AlignMode::CenterHorizontal).unwrap();
    d.apply(tx).unwrap();

    let b = rect(&d, "b");
    let c = rect(&d, "c");
    assert_eq!(c.x - b.x, dist_before, "internal spacing changed");

    // the set ended up centered: 640 - (280 - 10) = 370 / 2 = 185 slack
    let x0 = [rect(&d, "a").x, b.x, c.x]
        .into_iter()
        .fold(f32::MAX, f32::min);
    assert_eq!(x0, 185.0);
}

#[test]
fn everything_undoes_with_one_ctrl_z() {
    let mut d = doc();
    let before: Vec<Rect> = ["a", "b", "c"].iter().map(|i| rect(&d, i)).collect();

    let tx = align(d.ir(), &sel(), AlignMode::Top, Reference::First).unwrap();
    d.apply(tx).unwrap();
    d.undo().unwrap();

    let after: Vec<Rect> = ["a", "b", "c"].iter().map(|i| rect(&d, i)).collect();
    assert_eq!(before, after, "one batch, one undo");
    assert!(!d.can_undo());
}

#[test]
fn document_remains_valid_after_arranging() {
    let mut d = doc();
    for op in [
        align(d.ir(), &sel(), AlignMode::Top, Reference::First),
        equalize(d.ir(), &sel(), Equalize::Both),
    ]
    .into_iter()
    .flatten()
    {
        d.apply(op).unwrap();
    }
    if let Some(tx) = distribute(d.ir(), &sel(), Distribute::Horizontal) {
        d.apply(tx).unwrap();
    }
    assert!(d.ir().validate().is_ok());
}

// ------------------------------------------------------- Z-order and tab order

fn order(d: &Doc) -> Vec<String> {
    d.ir().node("form_main").unwrap().children.clone()
}

fn set_tab(d: &mut Doc, id: &str, t: i64) {
    d.apply(Transaction::user("t").with(Command::SetProp {
        node: id.into(),
        key: "tab_order".into(),
        value: Some(crawrad_ir::Value::Int(t)),
    }))
    .unwrap();
}

#[test]
fn bring_to_front_puts_at_end_of_order() {
    let mut d = doc();
    let tx = z_order(d.ir(), &["a".into()], ZOrder::Front).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(order(&d), vec!["b", "c", "a"]);
}

#[test]
fn send_to_back_puts_at_beginning() {
    let mut d = doc();
    let tx = z_order(d.ir(), &["c".into()], ZOrder::Back).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(order(&d), vec!["c", "a", "b"]);
}

#[test]
fn one_step_forward_swaps_with_neighbor() {
    let mut d = doc();
    let tx = z_order(d.ir(), &["a".into()], ZOrder::Forward).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(order(&d), vec!["b", "a", "c"]);
}

#[test]
fn already_at_front_does_not_generate_transaction() {
    let d = doc();
    assert!(z_order(d.ir(), &["c".into()], ZOrder::Front).is_none());
    assert!(z_order(d.ir(), &["a".into()], ZOrder::Back).is_none());
}

#[test]
fn multiple_selected_go_to_front_preserving_relative_order() {
    let mut d = doc();
    let tx = z_order(d.ir(), &["a".into(), "b".into()], ZOrder::Front).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(order(&d), vec!["c", "a", "b"]);
    assert!(d.ir().validate().is_ok());
}

#[test]
fn tab_order_follows_tab_order_prop_and_not_z_order() {
    let mut d = doc();
    set_tab(&mut d, "a", 2);
    set_tab(&mut d, "b", 0);
    set_tab(&mut d, "c", 1);
    let seq: Vec<String> = tab_order(d.ir()).into_iter().map(|(i, _)| i).collect();
    assert_eq!(seq, vec!["b", "c", "a"]);
}

#[test]
fn renumber_removes_gaps_and_ties() {
    let mut d = doc();
    set_tab(&mut d, "a", 7);
    set_tab(&mut d, "b", 7);
    set_tab(&mut d, "c", 90);

    let tx = normalize_tab_order(d.ir()).unwrap();
    d.apply(tx).unwrap();
    let numbers: Vec<i64> = tab_order(d.ir()).into_iter().map(|(_, t)| t).collect();
    assert_eq!(numbers, vec![0, 1, 2]);
    assert!(
        normalize_tab_order(d.ir()).is_none(),
        "renumbering again does nothing"
    );
}

#[test]
fn move_in_tab_order_swaps_with_neighbor() {
    let mut d = doc();
    // test nodes are born raw, without passing through registry: nodes without
    // TabOrder do not participate in tab order, like Label in the classic IDE
    set_tab(&mut d, "a", 0);
    set_tab(&mut d, "b", 1);
    set_tab(&mut d, "c", 2);
    let tx = move_tab_order(d.ir(), &"c".to_string(), true).unwrap();
    d.apply(tx).unwrap();
    let seq: Vec<String> = tab_order(d.ir()).into_iter().map(|(i, _)| i).collect();
    assert_eq!(seq, vec!["a", "c", "b"]);
}

#[test]
fn the_first_does_not_go_up_and_the_last_does_not_go_down() {
    let mut d = doc();
    set_tab(&mut d, "a", 0);
    set_tab(&mut d, "b", 1);
    set_tab(&mut d, "c", 2);
    assert!(move_tab_order(d.ir(), &"a".to_string(), true).is_none());
    assert!(move_tab_order(d.ir(), &"c".to_string(), false).is_none());
}

#[test]
fn new_component_enters_at_end_of_tab_sequence() {
    // In the classic IDE the newborn gets the next number; leaving everyone with
    // zero makes Tab walk by Z-order and not by what the user defined.
    let mut d = doc();
    assert_eq!(next_tab_order(d.ir()), 0, "no tabbable component yet");

    set_tab(&mut d, "a", 0);
    set_tab(&mut d, "b", 1);
    assert_eq!(next_tab_order(d.ir()), 2);

    // and respects a high number typed by hand in the inspector
    set_tab(&mut d, "c", 9);
    assert_eq!(next_tab_order(d.ir()), 10);
}
