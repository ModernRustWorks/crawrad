//! Selection by marquee — dragging over the form background.
//!
//! The behaviour is that of the classic IDE: selects by **intersection**, touching
//! suffices. Ctrl keeps what was already selected and adds to it.

use super::*;
use crawrad_ir::{Document, Node};

/// lbl (16,16,100,20) · edt (16,48,200,24) · btn (16,96,96,32)
fn doc() -> Document {
    let mut d = Document::new("form_main", "Form");
    let mut n = |id: &str, r: Rect| {
        let mut x = Node::new(id, "Button");
        x.set_rect(r);
        d.add_child("form_main", x).unwrap();
    };
    n("lbl", Rect::new(16.0, 16.0, 100.0, 20.0));
    n("edt", Rect::new(16.0, 48.0, 200.0, 24.0));
    n("btn", Rect::new(16.0, 96.0, 96.0, 32.0));
    d
}

fn marquee_select(
    des: &mut Designer,
    d: &Document,
    sel: &[NodeId],
    from: (f32, f32),
    to: (f32, f32),
    ctrl: bool,
) -> Vec<NodeId> {
    des.pointer_down(d, sel, from.0, from.1, ctrl);
    des.pointer_move(d, to.0, to.1);
    des.pointer_up(d).select.unwrap_or_default()
}

#[test]
fn marquee_selects_all_that_it_touches() {
    let mut des = Designer::default();
    let d = doc();
    // from top-left corner to middle of edt
    let sel = marquee_select(&mut des, &d, &[], (8.0, 8.0), (300.0, 60.0), false);
    assert_eq!(sel, vec!["lbl".to_string(), "edt".to_string()]);
}

#[test]
fn marquee_catches_by_intersection_and_not_only_what_is_fully_inside() {
    let mut des = Designer::default();
    let d = doc();
    // touches only left edge of edt
    let sel = marquee_select(&mut des, &d, &[], (10.0, 50.0), (30.0, 60.0), false);
    assert_eq!(sel, vec!["edt".to_string()], "touching already selects");
}

#[test]
fn marquee_in_any_direction() {
    let mut des = Designer::default();
    let d = doc();
    // dragging from bottom-right to top-left
    let sel = marquee_select(&mut des, &d, &[], (300.0, 140.0), (8.0, 90.0), false);
    assert_eq!(sel, vec!["btn".to_string()]);
}

#[test]
fn ctrl_plus_marquee_adds_to_preexisting_selection() {
    // exactly the complaint: Ctrl+click added, Ctrl+area did not
    let mut des = Designer::default();
    let d = doc();
    let sel = marquee_select(
        &mut des,
        &d,
        &["btn".to_string()],
        (8.0, 8.0),
        (300.0, 40.0),
        true,
    );
    assert_eq!(sel, vec!["btn".to_string(), "lbl".to_string()]);
}

#[test]
fn marquee_without_ctrl_replaces_selection() {
    let mut des = Designer::default();
    let d = doc();
    let sel = marquee_select(
        &mut des,
        &d,
        &["btn".to_string()],
        (8.0, 8.0),
        (300.0, 40.0),
        false,
    );
    assert_eq!(
        sel,
        vec!["lbl".to_string()],
        "without Ctrl, marquee starts from scratch"
    );
}

#[test]
fn marquee_in_empty_area_clears_selection() {
    let mut des = Designer::default();
    let d = doc();
    let sel = marquee_select(
        &mut des,
        &d,
        &["btn".to_string()],
        (400.0, 300.0),
        (500.0, 380.0),
        false,
    );
    assert!(sel.is_empty());
}

#[test]
fn marquee_is_drawn_while_dragging() {
    let mut des = Designer::default();
    let d = doc();
    des.pointer_down(&d, &[], 8.0, 8.0, false);
    let o = des.pointer_move(&d, 120.0, 60.0);
    let m = o.marquee.expect("runtime needs rectangle to draw");
    assert_eq!((m.x, m.y, m.w, m.h), (8.0, 8.0, 112.0, 52.0));
    assert!(o.tx.is_none(), "marquee does not touch document");

    // and vanishes upon release
    let o = des.pointer_up(&d);
    assert_eq!(o.marquee.map(|r| (r.w, r.h)), Some((0.0, 0.0)));
}

#[test]
fn marquee_does_not_modify_document() {
    let mut des = Designer::default();
    let d = doc();
    let before = d.clone();
    marquee_select(&mut des, &d, &[], (8.0, 8.0), (300.0, 140.0), false);
    assert_eq!(d, before, "selecting is not editing");
}

#[test]
fn plain_click_on_background_does_not_count_as_marquee() {
    let mut des = Designer::default();
    let d = doc();
    des.pointer_down(&d, &["btn".to_string()], 400.0, 300.0, false);
    let o = des.pointer_up(&d);
    assert!(
        o.select.is_none(),
        "click already cleared selection on down; up decides nothing"
    );
}

#[test]
fn esc_cancels_marquee_without_selecting() {
    let mut des = Designer::default();
    let d = doc();
    des.pointer_down(&d, &[], 8.0, 8.0, false);
    des.pointer_move(&d, 300.0, 140.0);
    let o = des.cancel();
    assert!(o.select.is_none() && o.tx.is_none());
    assert!(!des.is_dragging());
}

#[test]
fn up_far_from_origin_without_drag_selects_nothing() {
    // Dangerous case: lost capture, button released outside window, a
    // `pointer_up` arriving with mouse far away. Without drag record, this
    // would become a selection of everything between both points.
    let mut des = Designer::default();
    let d = doc();
    des.pointer_down(&d, &[], 8.0, 8.0, false);
    // no pointer_move: designer never knew of drag
    let o = des.pointer_up(&d);
    assert!(o.select.is_none(), "without tracked drag, up cannot select");
}

#[test]
fn mouse_jitter_does_not_become_marquee() {
    let mut des = Designer::default();
    let d = doc();
    des.pointer_down(&d, &[], 20.0, 20.0, false);
    des.pointer_move(&d, 22.0, 21.0); // 2 units: below threshold
    let o = des.pointer_up(&d);
    assert!(o.select.is_none(), "click with jitter is still a click");
}

#[test]
fn marquee_counts_how_many_it_caught_for_status_bar() {
    let mut des = Designer::default();
    let d = doc();
    des.pointer_down(&d, &[], 8.0, 8.0, false);
    des.pointer_move(&d, 300.0, 60.0);
    let o = des.pointer_up(&d);
    let st = o.status.expect("gesture explains itself in status bar");
    assert!(st.contains("2 selected"), "status was: {st}");
}

// ---------------------------------------------------------- guides

#[test]
fn guide_appears_when_edge_touches_neighbor() {
    let mut des = Designer::default();
    let mut d = doc();
    // target with left at 16, equal to lbl
    d.node_mut("btn")
        .unwrap()
        .set_rect(Rect::new(16.0, 200.0, 96.0, 32.0));

    // drag edt close to left edge of others
    des.pointer_down(&d, &["edt".to_string()], 100.0, 60.0, false);
    let o = des.pointer_move(&d, 103.0, 60.0);
    assert!(
        !o.guides.is_empty(),
        "aligned with left edge of neighbors and did not notify"
    );
    assert!(o.guides.iter().any(|g| g.vertical && g.pos == 16.0));
}

#[test]
fn component_snaps_to_guide() {
    let mut des = Designer::default();
    let mut doc_ = crawrad_doc::Doc::new(doc());
    // target with left at 20; grid snap would take to 24
    doc_.ir().node("btn").unwrap();
    let tx = crawrad_doc::Transaction::user("prepare").with(crawrad_doc::Command::SetRect {
        node: "btn".into(),
        rect: Rect::new(20.0, 200.0, 96.0, 32.0),
    });
    doc_.apply(tx).unwrap();

    des.pointer_down(doc_.ir(), &["edt".to_string()], 100.0, 60.0, false);
    let o = des.pointer_move(doc_.ir(), 105.0, 60.0);
    doc_.apply(o.tx.unwrap()).unwrap();
    assert_eq!(
        doc_.ir().node("edt").unwrap().rect().unwrap().x,
        20.0,
        "guide overrides grid when within tolerance"
    );
}

#[test]
fn guide_line_stays_in_gap_between_both() {
    // classic IDE does not cross entire form: line lives in space between them
    let mut des = Designer::default();
    let mut d = doc();
    d.node_mut("btn")
        .unwrap()
        .set_rect(Rect::new(16.0, 200.0, 96.0, 32.0));

    des.pointer_down(&d, &["lbl".to_string()], 60.0, 26.0, false);
    let o = des.pointer_move(&d, 60.0, 30.0);
    let g = o
        .guides
        .iter()
        .find(|g| g.vertical)
        .expect("expected vertical guide");
    // lbl goes to y=36+; btn begins at 200: stroke stays between them
    assert!(
        g.from >= 36.0 && g.to <= 200.0,
        "stroke from {} to {}",
        g.from,
        g.to
    );
}

#[test]
fn without_neighbor_nearby_there_is_no_guide() {
    let mut des = Designer::default();
    let d = doc();
    des.pointer_down(&d, &["btn".to_string()], 60.0, 110.0, false);
    let o = des.pointer_move(&d, 200.0, 300.0);
    assert!(o.guides.is_empty());
}

#[test]
fn ctrl_reveals_preexisting_alignments() {
    let mut d = doc();
    // lbl and edt already start with same left (16)
    let des = Designer::default();
    let guides = des.static_guides(&d, &["lbl".to_string()]);
    assert!(
        guides.iter().any(|g| g.vertical && g.pos == 16.0),
        "both left edges coincide and nobody notified"
    );

    // misalign ALL neighbors and notice vanishes
    d.node_mut("edt")
        .unwrap()
        .set_rect(Rect::new(23.0, 48.0, 200.0, 24.0));
    d.node_mut("btn")
        .unwrap()
        .set_rect(Rect::new(31.0, 96.0, 96.0, 32.0));
    let guides = des.static_guides(&d, &["lbl".to_string()]);
    assert!(
        !guides.iter().any(|g| g.vertical),
        "must not invent alignment that does not exist: {guides:?}"
    );
}

#[test]
fn without_selection_ctrl_shows_nothing() {
    let des = Designer::default();
    let d = doc();
    assert!(des.static_guides(&d, &[]).is_empty());
}

#[test]
fn text_alignment_beats_box_alignment() {
    // A Label of 20 next to an Edit of 24: aligning boxes leaves text
    // crooked. What commands is text line — pink guide of classic IDE.
    use std::rc::Rc;
    let reg = Rc::new(crawrad_registry::Registry::builtin());

    let mut d = Document::new("form_main", "Form");
    let mut lbl = Node::new("lbl", "Label");
    lbl.set_rect(Rect::new(16.0, 100.0, 100.0, 20.0));
    d.add_child("form_main", lbl).unwrap();
    let mut edt = Node::new("edt", "Edit");
    edt.set_rect(Rect::new(140.0, 300.0, 200.0, 24.0));
    d.add_child("form_main", edt).unwrap();

    let mut des = Designer::with_registry(reg);
    des.snap = false;

    // drag Edit close to height of Label
    des.pointer_down(&d, &["edt".to_string()], 200.0, 312.0, false);
    let o = des.pointer_move(&d, 200.0, 111.0);

    let g = o
        .guides
        .iter()
        .find(|g| !g.vertical)
        .expect("expected horizontal guide");
    assert_eq!(
        g.kind,
        GuideKind::Text,
        "guide had to be text, came {:?}",
        g.kind
    );

    // and Edit stops with text line exactly on Label's
    let cmd = o.tx.unwrap();
    let rect = match &cmd.commands[0] {
        crawrad_doc::Command::SetRect { rect, .. } => *rect,
        other => panic!("expected SetRect, came {other:?}"),
    };
    // Label: top 100 + indent 2 + ascent 11 = 113 (baseline)
    // Centred Edit: y + h/2 + (ascent-descent)/2 = y + 12 + 4
    assert_eq!(rect.y + 16.0, 113.0, "baseline misaligned");
}

#[test]
fn drag_and_ctrl_use_same_rule_with_different_extents() {
    // Single implementation: what changes is stroke length. Two buttons in
    // equal columns, separated vertically, isolate guide extent.
    use std::rc::Rc;
    let reg = Rc::new(crawrad_registry::Registry::builtin());

    let mut d = Document::new("form_main", "Form");
    let mut a = Node::new("a", "Button");
    a.set_rect(Rect::new(100.0, 100.0, 96.0, 32.0));
    d.add_child("form_main", a).unwrap();
    let mut b = Node::new("b", "Button");
    b.set_rect(Rect::new(100.0, 300.0, 96.0, 32.0));
    d.add_child("form_main", b).unwrap();

    let mut des = Designer::with_registry(reg);
    des.snap = false;

    // Ctrl: vertical line crosses both (100 to 332)
    let static_guides = des.static_guides(&d, &["a".to_string()]);
    let v = static_guides
        .iter()
        .find(|g| g.vertical && g.pos == 100.0)
        .expect("left edges coincide");
    assert_eq!((v.from, v.to), (100.0, 332.0), "Ctrl crosses both");

    // Drag: same relationship, short stroke in gap (132 to 300)
    des.pointer_down(&d, &["a".to_string()], 140.0, 116.0, false);
    let o = des.pointer_move(&d, 141.0, 116.0);
    let v = o
        .guides
        .iter()
        .find(|g| g.vertical)
        .expect("expected vertical guide on drag");
    assert!(
        v.from >= 132.0 && v.to <= 300.0,
        "drag stroke stays in gap: {} to {}",
        v.from,
        v.to
    );
}

#[test]
fn stroke_chooses_closest_neighbor() {
    // Three on same left: stroke must connect dragged to NEAREST
    // neighbor. Connecting to farthest, it crosses middle one and
    // stays loose on form — which appeared on screen.
    use std::rc::Rc;
    let mut d = Document::new("form_main", "Form");
    for (id, y) in [("a", 10.0), ("b", 100.0), ("c", 200.0)] {
        let mut n = Node::new(id, "Button");
        n.set_rect(Rect::new(50.0, y, 96.0, 32.0));
        d.add_child("form_main", n).unwrap();
    }
    let mut des = Designer::with_registry(Rc::new(crawrad_registry::Registry::builtin()));
    des.snap = false;

    // misalign "a" and drag back
    d.node_mut("a")
        .unwrap()
        .set_rect(Rect::new(53.0, 10.0, 96.0, 32.0));
    des.pointer_down(&d, &["a".to_string()], 100.0, 26.0, false);
    let o = des.pointer_move(&d, 97.0, 26.0);

    let v = o
        .guides
        .iter()
        .find(|g| g.vertical)
        .expect("vertical guide");
    assert_eq!(v.pos, 50.0);
    assert!(
        v.to <= 100.0,
        "should connect to upper neighbor (b, at y=100), not distant one: {} to {}",
        v.from,
        v.to
    );
}

#[test]
fn all_valid_relationships_appear_together() {
    // Classic IDE shows top, bottom and center simultaneously when all three hold.
    // Showing only winner made top and bottom alignment vanish.
    use std::rc::Rc;
    let mut d = Document::new("form_main", "Form");
    let mut a = Node::new("a", "Button");
    a.set_rect(Rect::new(50.0, 100.0, 96.0, 32.0));
    d.add_child("form_main", a).unwrap();
    let mut b = Node::new("b", "Button");
    b.set_rect(Rect::new(300.0, 100.0, 96.0, 32.0));
    d.add_child("form_main", b).unwrap();

    let mut des = Designer::with_registry(Rc::new(crawrad_registry::Registry::builtin()));
    des.snap = false;

    // drag "a" maintaining same vertical strip of "b"
    des.pointer_down(&d, &["a".to_string()], 98.0, 116.0, false);
    let o = des.pointer_move(&d, 100.0, 116.0);

    let horizontals: Vec<_> = o.guides.iter().filter(|g| !g.vertical).collect();
    assert!(
        horizontals.len() >= 3,
        "top, bottom and center (and baseline) should appear together: {:?}",
        o.guides
    );
    assert!(horizontals.iter().any(|g| g.pos == 100.0), "missing top");
    assert!(horizontals.iter().any(|g| g.pos == 132.0), "missing bottom");
}

// ------------------------------------------- gap comes from component
//
// `Designer` had a field `margin: 8.0`, chosen visually, and it
// contradicted the program itself: `Align` with `AlignWithMargins` spaces
// by sum of touching margins, and guide promised another number on
// same form. Field was removed; number now comes from component's `Margins`,
// with same calculation.

/// Two buttons side-by-side, in same vertical strip, with margins test
/// asks for. `a` is left of `b`.
fn two_side_by_side(margin: Option<f32>) -> Document {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 600.0, 300.0));
    for (id, x) in [("a", 50.0), ("b", 300.0)] {
        let mut n = Node::new(id, "Button");
        n.set_rect(Rect::new(x, 100.0, 96.0, 32.0));
        if let Some(m) = margin {
            n.props.insert(
                "margins".into(),
                crawrad_ir::Value::Edges(crawrad_ir::Edges::uniform(m)),
            );
        }
        d.add_child("form_main", n).unwrap();
    }
    d
}

/// Drags `a` until its right side is `raw_gap` before `b`'s left side,
/// and returns remaining gap after snapping.
fn gap_after_snap(d: &Document, raw_gap: f32) -> f32 {
    use std::rc::Rc;
    let mut des = Designer::with_registry(Rc::new(crawrad_registry::Registry::builtin()));
    // Without grid: guide must rule here, and grid of 8 would hide
    // number this test measures.
    des.snap = false;

    let a = d.node("a").unwrap().rect().unwrap();
    let b = d.node("b").unwrap().rect().unwrap();
    // where `a` would have to be for gap to be exactly `raw_gap`
    let target_x = b.x - raw_gap - a.w;
    let (px, py) = (a.x + 10.0, a.y + 10.0);
    des.pointer_down(d, &["a".to_string()], px, py, false);
    let o = des.pointer_move(d, px + (target_x - a.x), py);

    let tx = o.tx.expect("drag must produce transaction");
    let mut doc = crawrad_doc::Doc::new(d.clone());
    doc.apply(tx).unwrap();
    let new_node = doc.ir().node("a").unwrap().rect().unwrap();
    b.x - (new_node.x + new_node.w)
}

#[test]
fn spacing_guide_snaps_to_sum_of_margins_and_not_designer_number() {
    // With classic component library default (3 on each side), proper gap between two neighbors
    // is 6 — my right plus their left. Approaching, snaps to 6.
    //
    // Was 8, and 8 was nobody's margin: with `AlignWithMargins` on,
    // `Align` spaced by 6 and drag guide promised 8, on same
    // form and for same two components.
    let d = two_side_by_side(None);
    assert_eq!(
        gap_after_snap(&d, 8.0),
        6.0,
        "arriving with gap of 8, should snap to 6 of 3+3 default"
    );

    // And number COMES from component: with margin 10 on both, proper gap
    // becomes 20. If calculation were still a designer field, this case would produce
    // same result as above — and that is why both are here.
    let d = two_side_by_side(Some(10.0));
    assert_eq!(
        gap_after_snap(&d, 18.0),
        20.0,
        "with margin 10 on both sides proper spacing is 20"
    );
}

#[test]
fn hatched_strip_has_thickness_of_gap_it_promises() {
    // Spacing guide is drawn as a STRIP, and its thickness is the
    // gap. Promising 20 and hatching 8 is same family of defect as ADR 0019
    // seen from another angle: drawing saying one number and geometry
    // doing another.
    use std::rc::Rc;
    let d = two_side_by_side(Some(10.0));
    let mut des = Designer::with_registry(Rc::new(crawrad_registry::Registry::builtin()));
    des.snap = false;

    let a = d.node("a").unwrap().rect().unwrap();
    let (px, py) = (a.x + 10.0, a.y + 10.0);
    des.pointer_down(&d, &["a".to_string()], px, py, false);
    let target_x = 300.0 - 18.0 - a.w;
    let o = des.pointer_move(&d, px + (target_x - a.x), py);

    let strip = o
        .guides
        .iter()
        .find(|g| g.kind == crawrad_designer_spacing_guide())
        .expect("spacing strip had to appear");
    assert_eq!(strip.thickness, 20.0, "strip hatches gap that it promises");
}

/// `GuideKind::Spacing`, named here so test above reads as sentence.
fn crawrad_designer_spacing_guide() -> GuideKind {
    GuideKind::Spacing
}

/// A form with a single component already collapsed by anchor.
///
/// `(300, 250, -120 x -90)` is what remains of a `380 x 280` anchored on
/// four sides after form shrinks 500 — state owner saw in
/// the classic RAD IDE as a sliver.
fn collapsed_doc() -> Document {
    let mut d = Document::new("form_main", "Form");
    let mut collapsed = Node::new("vanished", "Button");
    collapsed.set_rect(Rect::new(300.0, 250.0, -120.0, -90.0));
    d.add_child("form_main", collapsed).unwrap();
    d
}

/// **Marquee passing over collapsed component catches it.**
///
/// Since 2026-09-09 anchor lets width cross zero on purpose —
/// that is what makes shrinking form and growing back return exact
/// component, as owner measured in the classic RAD IDE. Price is document
/// can contain negative `w`, and `Rect::intersects` reads inverted literally.
///
/// **Error is of omission, not excess.** Scanned on 2026-09-09 over 6912
/// marquees against `(300, 250, -120 x -90)`: no marquee catches because of
/// inversion, and **5472** cease to catch. Collapsed component becomes
/// unreachable by marquee, and would only return through Structure.
///
/// Marquee of this test — `(280, 230)` to `(340, 290)` — passes exactly over
/// `(300, 250)`, which is where component is.
#[test]
fn marquee_over_anchor_collapsed_still_catches_it() {
    let d = collapsed_doc();
    let mut des = Designer::default();
    let sel = marquee_select(&mut des, &d, &[], (280.0, 230.0), (340.0, 290.0), false);
    assert_eq!(
        sel,
        vec!["vanished".to_string()],
        "marquee over (300, 250) must catch collapsed component; \
         with negative w intersects requires marquee to cover entire [180, 300] \
         strip, and component disappears from marquee"
    );
}

/// Same along vertical axis, and with marquee opened bottom to top.
///
/// Both axes are clipped by same line of `non_inverted`, and clipping only
/// one would pass in this file unnoticed.
#[test]
fn collapsed_is_also_reachable_along_vertical_axis() {
    let d = collapsed_doc();
    let mut des = Designer::default();
    // Bottom to top and right to left: marquee normalizes.
    let sel = marquee_select(&mut des, &d, &[], (340.0, 290.0), (280.0, 230.0), false);
    assert_eq!(
        sel,
        vec!["vanished".to_string()],
        "marquee in any direction applies equally to collapsed component"
    );
}

/// Clipping of `non_inverted` must not INFLATE collapsed component.
///
/// Clipping at zero returns component exact point where it is — not
/// a bit more. A generous floor would give area to what has none, and component
/// would start being caught by marquee that only touches **beside** it.
///
/// Marquee of this test lives entirely in quadrant just below and right of
/// `(300, 250)`, starting at `x = 310`. **It limits floor from above, and does not
/// fix it at zero**: measured on 2026-09-09, fails from ~11 pt
/// (`max(20.0)` and `max(40.0)` turn red), and allows `max(1.0)`.
/// Between 0 and 1 there is no observable difference here, and test does not pretend there is.
#[test]
fn marquee_beside_collapse_point_catches_nothing() {
    let d = collapsed_doc();
    let mut des = Designer::default();
    let sel = marquee_select(&mut des, &d, &[], (310.0, 260.0), (330.0, 280.0), false);
    assert!(
        sel.is_empty(),
        "marquee (310,260)-(330,280) does not touch (300, 250), and returned \
         {sel:?}: non_inverted clipping gave area to collapsed component \
         instead of clipping it at zero"
    );
}

/// And marquee far away also does not catch.
#[test]
fn marquee_far_from_collapsed_continues_not_catching() {
    let d = collapsed_doc();
    let mut des = Designer::default();
    let sel = marquee_select(&mut des, &d, &[], (20.0, 20.0), (120.0, 120.0), false);
    assert!(
        sel.is_empty(),
        "marquee at (20,20)-(120,120) is far from (300, 250) and cannot catch \
         anything, yet returned {sel:?}"
    );
}
