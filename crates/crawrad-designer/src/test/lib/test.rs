use super::*;
use crate::arrangement::{self, AlignMode, Equalize, Reference, ZOrder};
use crate::layout::{self, Anchoring};
use crawrad_doc::Doc;
use crawrad_ir::{Document, Node};

/// btn_ok at (24,96,96,32) and edt at (24,48,280,24)
fn doc() -> Document {
    let mut d = Document::new("form_main", "Form");
    let mut btn = Node::new("btn_ok", "Button");
    btn.set_rect(Rect::new(24.0, 96.0, 96.0, 32.0));
    d.add_child("form_main", btn).unwrap();
    let mut edt = Node::new("edt_nome", "Edit");
    edt.set_rect(Rect::new(24.0, 48.0, 280.0, 24.0));
    d.add_child("form_main", edt).unwrap();
    d
}

fn rect(d: &Document, id: &str) -> Rect {
    d.node(id).unwrap().rect().unwrap()
}

#[test]
fn empty_click_clears_selection_and_starts_marquee() {
    let mut des = Designer::default();
    let d = doc();
    let o = des.pointer_down(&d, &["btn_ok".into()], 500.0, 300.0, false);
    assert_eq!(o.select, Some(vec![]), "the click clears immediately");
    assert!(
        o.dragging,
        "and already begins the marquee: if the mouse drags, it turns into area selection"
    );
}

#[test]
fn click_selects_topmost_in_z_order() {
    let mut d = doc();
    // a panel on top, covering the button
    let mut top = Node::new("por_cima", "Label");
    top.set_rect(Rect::new(0.0, 0.0, 400.0, 400.0));
    d.add_child("form_main", top).unwrap();

    let mut des = Designer::default();
    let o = des.pointer_down(&d, &[], 40.0, 100.0, false);
    assert_eq!(o.select, Some(vec!["por_cima".to_string()]));
}

#[test]
fn ctrl_click_toggles_selection() {
    let mut des = Designer::default();
    let d = doc();
    let o = des.pointer_down(&d, &["btn_ok".into()], 40.0, 55.0, true);
    assert_eq!(
        o.select,
        Some(vec!["btn_ok".to_string(), "edt_nome".to_string()])
    );

    let o = des.pointer_down(&d, &["btn_ok".into(), "edt_nome".into()], 40.0, 55.0, true);
    assert_eq!(o.select, Some(vec!["btn_ok".to_string()]));
}

#[test]
fn drag_moves_with_grid_snap() {
    let mut des = Designer::default();
    let mut doc = Doc::new(doc());

    // in the core of the component: near the corner, the handle rules
    des.pointer_down(doc.ir(), &["btn_ok".into()], 60.0, 110.0, false);
    let o = des.pointer_move(doc.ir(), 77.0, 127.0); // +17,+17 -> snap to +16,+16
    doc.apply(o.tx.unwrap()).unwrap();

    let r = rect(doc.ir(), "btn_ok");
    assert_eq!((r.x, r.y), (40.0, 112.0));
    assert_eq!((r.w, r.h), (96.0, 32.0), "dragging does not change size");
}

#[test]
fn drag_moves_entire_selection_together() {
    let mut des = Designer::default();
    let mut doc = Doc::new(doc());
    let sel = vec!["btn_ok".to_string(), "edt_nome".to_string()];

    des.pointer_down(doc.ir(), &sel, 60.0, 110.0, false);
    let o = des.pointer_move(doc.ir(), 92.0, 110.0); // +32
    let tx = o.tx.unwrap();
    assert_eq!(tx.commands.len(), 2, "one command per component");
    doc.apply(tx).unwrap();

    assert_eq!(rect(doc.ir(), "btn_ok").x, 56.0);
    assert_eq!(rect(doc.ir(), "edt_nome").x, 56.0);
    // and undos with a single Ctrl+Z
    doc.undo().unwrap();
    assert_eq!(rect(doc.ir(), "btn_ok").x, 24.0);
    assert_eq!(rect(doc.ir(), "edt_nome").x, 24.0);
}

#[test]
fn click_without_move_produces_no_transaction() {
    let mut des = Designer::default();
    let d = doc();
    des.pointer_down(&d, &[], 60.0, 110.0, false);
    let o = des.pointer_move(&d, 60.4, 110.4);
    assert!(o.tx.is_none(), "mouse jitter must not become history");
}

#[test]
fn southeast_handle_stretches_down_and_right() {
    let mut des = Designer::default();
    let mut doc = Doc::new(doc());
    let sel = vec!["btn_ok".to_string()];

    // SE corner of btn_ok: (24+96, 96+32) = (120, 128)
    let h = des.hit_handle(doc.ir(), &sel, 120.0, 128.0);
    assert_eq!(h, Some(Handle::SE));

    des.pointer_down(doc.ir(), &sel, 120.0, 128.0, false);
    let o = des.pointer_move(doc.ir(), 160.0, 160.0);
    doc.apply(o.tx.unwrap()).unwrap();

    let r = rect(doc.ir(), "btn_ok");
    assert_eq!((r.x, r.y), (24.0, 96.0), "the opposite corner stays put");
    assert_eq!((r.w, r.h), (136.0, 64.0));
}

#[test]
fn northwest_handle_moves_origin_and_keeps_opposite_corner() {
    let mut des = Designer::default();
    let mut doc = Doc::new(doc());
    let sel = vec!["btn_ok".to_string()];

    des.pointer_down(doc.ir(), &sel, 24.0, 96.0, false); // NW corner
    let o = des.pointer_move(doc.ir(), 40.0, 112.0);
    doc.apply(o.tx.unwrap()).unwrap();

    let r = rect(doc.ir(), "btn_ok");
    assert_eq!((r.x, r.y), (40.0, 112.0));
    assert_eq!(
        (r.x + r.w, r.y + r.h),
        (120.0, 128.0),
        "the SE corner cannot have moved"
    );
}

#[test]
fn resize_respects_minimum_size() {
    let mut des = Designer::default();
    let mut doc = Doc::new(doc());
    let sel = vec!["btn_ok".to_string()];

    des.pointer_down(doc.ir(), &sel, 120.0, 128.0, false); // SE
    let o = des.pointer_move(doc.ir(), -500.0, -500.0);
    doc.apply(o.tx.unwrap()).unwrap();

    let r = rect(doc.ir(), "btn_ok");
    assert!(r.w >= des.min_size && r.h >= des.min_size);
}

#[test]
fn cancel_drag_restores_position() {
    let mut des = Designer::default();
    let mut doc = Doc::new(doc());
    let sel = vec!["btn_ok".to_string()];

    des.pointer_down(doc.ir(), &sel, 60.0, 110.0, false);
    let o = des.pointer_move(doc.ir(), 200.0, 300.0);
    doc.apply(o.tx.unwrap()).unwrap();
    assert_ne!(rect(doc.ir(), "btn_ok").x, 24.0);

    let o = des.cancel();
    doc.apply(o.tx.unwrap()).unwrap();
    assert_eq!(rect(doc.ir(), "btn_ok"), Rect::new(24.0, 96.0, 96.0, 32.0));
    assert!(!des.is_dragging());
}

#[test]
fn keyboard_arrow_moves_one_point() {
    let des = Designer::default();
    let mut doc = Doc::new(doc());
    let tx = des
        .nudge(doc.ir(), &["btn_ok".to_string()], 1.0, 0.0)
        .unwrap();
    doc.apply(tx).unwrap();
    assert_eq!(
        rect(doc.ir(), "btn_ok").x,
        25.0,
        "nudge does not go through snap"
    );
}

#[test]
fn without_selection_there_is_no_nudge() {
    let des = Designer::default();
    let d = doc();
    assert!(des.nudge(&d, &[], 1.0, 0.0).is_none());
}

#[test]
fn palette_drop_aligns_to_grid() {
    let des = Designer::default();
    let r = des.drop_rect(103.0, 47.0, 96.0, 32.0);
    assert_eq!((r.x, r.y), (104.0, 48.0));
}

#[test]
fn snap_off_respects_pixel() {
    let mut des = Designer {
        snap: false,
        ..Default::default()
    };
    let mut doc = Doc::new(doc());
    des.pointer_down(doc.ir(), &["btn_ok".into()], 60.0, 110.0, false);
    let o = des.pointer_move(doc.ir(), 77.0, 110.0);
    doc.apply(o.tx.unwrap()).unwrap();
    assert_eq!(rect(doc.ir(), "btn_ok").x, 41.0);
}

#[test]
fn a_snapped_move_says_how_far_the_pointer_is_from_the_landing() {
    let mut des = Designer::default();
    let mut doc = Doc::new(doc());
    des.pointer_down(doc.ir(), &["btn_ok".into()], 60.0, 110.0, false);
    // +17,+17: the pointer would put it at 41,113; it lands on 40,112
    let o = des.pointer_move(doc.ir(), 77.0, 127.0);
    let ghost = o
        .ghost
        .clone()
        .expect("a snapped move shows where it lands");
    assert_eq!(
        (ghost.node.as_str(), ghost.dx, ghost.dy),
        ("btn_ok", 1.0, 1.0)
    );
    doc.apply(o.tx.unwrap()).unwrap();
    assert_eq!(
        (rect(doc.ir(), "btn_ok").x, rect(doc.ir(), "btn_ok").y),
        (40.0, 112.0)
    );
}

#[test]
fn alt_places_freely_and_shows_no_landing() {
    let mut des = Designer::default();
    let mut doc = Doc::new(doc());
    des.pointer_down(doc.ir(), &["btn_ok".into()], 60.0, 110.0, false);
    des.free = true;
    let o = des.pointer_move(doc.ir(), 77.0, 127.0);
    assert_eq!(
        o.ghost, None,
        "with Alt nothing snaps, so there is no landing to show"
    );
    doc.apply(o.tx.unwrap()).unwrap();
    assert_eq!(
        (rect(doc.ir(), "btn_ok").x, rect(doc.ir(), "btn_ok").y),
        (41.0, 113.0)
    );
}

#[test]
fn the_bar_steps_the_grid_through_its_sizes() {
    assert_eq!(adjacent_grid(8.0, true), 10.0);
    assert_eq!(adjacent_grid(8.0, false), 5.0);
    assert_eq!(
        adjacent_grid(2.0, false),
        2.0,
        "the first step is the floor"
    );
    assert_eq!(
        adjacent_grid(16.0, true),
        16.0,
        "the last step is the ceiling"
    );
    assert_eq!(
        adjacent_grid(7.0, true),
        8.0,
        "between two steps: the next one up"
    );
    assert_eq!(
        adjacent_grid(7.0, false),
        5.0,
        "between two steps: the next one down"
    );
    assert_eq!(
        adjacent_grid(40.0, false),
        16.0,
        "an Options size past the bar comes back to it"
    );
}

#[test]
fn handle_stays_outside_rectangle() {
    // Centered on the border — half inside, half outside, which is what the classic IDE does
    // — they EAT the content. In `PageControl` the top ones fell on the tabs,
    // and the middle one was between `Page1` and `Page2`. Worse than ugly: clicking the
    // tab is tested before selection, so the top handle switched pages instead of
    // resizing, and the control had no way to be stretched from the top.
    //
    // Facing outward, the inner edge of the little square touches the component's edge and
    // none of it is covered. And the small component gains as well: on a short `Label`
    // centered handles covered almost everything, leaving little to grab.
    let r = Rect::new(10.0, 20.0, 100.0, 50.0);
    let f = crawrad_runtime::HANDLE_OUTSET;
    assert_eq!(Handle::NW.anchor(r), (10.0 - f, 20.0 - f));
    assert_eq!(Handle::SE.anchor(r), (110.0 + f, 70.0 + f));
    // In the middle of a side, only the perpendicular axis extends; the other stays centered.
    assert_eq!(Handle::N.anchor(r), (60.0, 20.0 - f));
    assert_eq!(Handle::W.anchor(r), (10.0 - f, 45.0));
    assert!(f > 0.0, "outward means outward");
}

#[test]
fn handle_is_same_for_painter_and_clicker() {
    // What this test guards is not a number: it is the fact of ONE number existing.
    // The handle position was once written twice — `corners` in the adapter and
    // `Handle::anchor` here — agreeing by luck, with nothing requiring it. Moving
    // the handle on one side would put the drawing in one place and the click in another, and the
    // symptom would be "handle does not grab", which nobody links to that cause.
    //
    // The ORDER is also a contract: `Handle::ALL`, `crawrad_runtime::alcas` and the
    // adapter cursor table index by the same clockwise sequence.
    let r = Rect::new(10.0, 20.0, 100.0, 50.0);
    let from_boundary = crawrad_runtime::handles(r.x, r.y, r.w, r.h);
    for (i, h) in Handle::ALL.iter().enumerate() {
        assert_eq!(
            h.anchor(r),
            from_boundary[i],
            "`Handle::ALL` departed from clockwise order of `crawrad_runtime::alcas`"
        );
    }
}

#[test]
fn non_visual_has_no_resize_handle() {
    // The ActionList icon has the size it has — as in the classic IDE, it moves
    // but does not stretch.
    let mut d = doc();
    let mut acl = Node::new("ActionList1", "ActionList");
    acl.set_rect(Rect::new(300.0, 8.0, 28.0, 28.0));
    d.add_child("form_main", acl).unwrap();

    let des = Designer::with_registry(std::rc::Rc::new(crawrad_registry::Registry::builtin()));
    let sel = vec!["ActionList1".to_string()];
    // over the bottom-right corner, where a visual would have a handle
    assert!(des.hit_handle(&d, &sel, 328.0, 36.0).is_none());
    // and the same point on a button continues giving a handle
    assert!(
        des.hit_handle(&d, &["btn_ok".to_string()], 120.0, 128.0)
            .is_some()
    );
}

#[test]
fn without_registry_designer_assumes_all_draw() {
    // The designer never depended on the catalog to function; it continues that way.
    let d = doc();
    let des = Designer::default();
    assert!(des.visual(&d, &"btn_ok".to_string()));
    assert!(des.visual(&d, &"nao_existe".to_string()));
}

/// A form with a panel, and a panel INSIDE it.
fn with_panels() -> Document {
    let mut d = Document::new("form", "Form");
    {
        let root = d.node_mut("form").unwrap();
        root.set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
        root.set_name("form");
    }
    let mut outer = Node::new("pnl_fora", "Panel");
    outer.set_name("pnl_fora");
    outer.set_rect(Rect::new(20.0, 20.0, 300.0, 200.0));
    d.add_child("form", outer).unwrap();

    let mut inner = Node::new("pnl_dentro", "Panel");
    inner.set_name("pnl_dentro");
    // relative to pnl_fora: sits at (60, 60) on the form
    inner.set_rect(Rect::new(40.0, 40.0, 120.0, 100.0));
    d.add_child("pnl_fora", inner).unwrap();
    d.validate().unwrap();
    d
}

#[test]
fn dropping_outside_everything_goes_to_form() {
    let d = with_panels();
    let dz = Designer::with_registry(std::rc::Rc::new(Registry::builtin()));
    let (parent, x, y) = dz.parent_for_drop(&d, 350.0, 260.0);
    assert_eq!(parent, "form");
    assert_eq!(
        (x, y),
        (350.0, 260.0),
        "without parent, coordinate does not change"
    );
}

#[test]
fn dropping_on_panel_places_inside_and_position_becomes_relative() {
    let d = with_panels();
    let dz = Designer::with_registry(std::rc::Rc::new(Registry::builtin()));
    // (30, 30) is inside pnl_fora (20,20) and outside pnl_dentro
    let (parent, x, y) = dz.parent_for_drop(&d, 30.0, 30.0);
    assert_eq!(parent, "pnl_fora");
    assert_eq!((x, y), (10.0, 10.0));
}

#[test]
fn with_nested_panels_innermost_wins() {
    // What `csAcceptsControls` in the classic component library does: target is the innermost
    // container under the point, not the first to accept.
    let d = with_panels();
    let dz = Designer::with_registry(std::rc::Rc::new(Registry::builtin()));
    // (70, 70) is inside BOTH; the inner one begins at (60,60)
    let (parent, x, y) = dz.parent_for_drop(&d, 70.0, 70.0);
    assert_eq!(parent, "pnl_dentro");
    assert_eq!((x, y), (10.0, 10.0));
}

#[test]
fn whoever_does_not_accept_children_is_never_target() {
    // Dropping on top of an `Edit` puts nothing inside it. The registry
    // answers this (`container`), not a query to the user.
    let mut d = with_panels();
    let mut e = Node::new("edt", "Edit");
    e.set_name("edt_nome");
    e.set_rect(Rect::new(10.0, 10.0, 200.0, 24.0));
    d.add_child("pnl_fora", e).unwrap();
    let dz = Designer::with_registry(std::rc::Rc::new(Registry::builtin()));
    // over the Edit, which sits at (30,30) on the form
    let (parent, _, _) = dz.parent_for_drop(&d, 40.0, 35.0);
    assert_eq!(
        parent, "pnl_fora",
        "Edit does not accept children; the panel does"
    );
}

/// A form with panel, two children INSIDE and a sibling of the panel.
fn with_children() -> Document {
    let mut d = Document::new("form", "Form");
    d.node_mut("form")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    let mut p = Node::new("pnl", "Panel");
    p.set_rect(Rect::new(20.0, 20.0, 300.0, 200.0));
    d.add_child("form", p).unwrap();
    for (id, y) in [("a", 10.0), ("b", 50.0)] {
        let mut n = Node::new(id, "Button");
        n.set_rect(Rect::new(10.0, y, 96.0, 32.0));
        d.add_child("pnl", n).unwrap();
    }
    let mut uncle = Node::new("tio", "Button");
    uncle.set_rect(Rect::new(20.0, 240.0, 96.0, 32.0));
    d.add_child("form", uncle).unwrap();
    d.validate().unwrap();
    d
}

#[test]
fn bring_to_front_moves_among_siblings_and_never_passes_uncle() {
    // Read in the classic component library source: `SetZOrderPosition` modifies `FParent.FControls`,
    // never a global list. A panel child brought to front rises among
    // its siblings.
    let d = with_children();
    let tx = arrangement::z_order(&d, &["a".to_string()], ZOrder::Front).unwrap();
    for c in &tx.commands {
        match c {
            Command::Reorder { parent, .. } => {
                assert_eq!(parent, "pnl", "reordering is WITHIN the panel");
            }
            other => panic!("unexpected command: {other:?}"),
        }
    }
    // and the uncle remains where it was: nobody modified the form list
    assert!(
        !tx.commands
            .iter()
            .any(|c| matches!(c, Command::Reorder { parent, .. } if parent == "form")),
        "{tx:?}"
    );
}

#[test]
fn selection_across_two_parents_reorders_both_in_same_transaction() {
    // One Ctrl+Z undos everything — that is the golden rule.
    //
    // `pnl` is the first child of the form and `a` is the first of the panel:
    // both have room to rise, in different parents.
    let d = with_children();
    let tx =
        arrangement::z_order(&d, &["a".to_string(), "pnl".to_string()], ZOrder::Front).unwrap();
    let parents: std::collections::BTreeSet<String> = tx
        .commands
        .iter()
        .filter_map(|c| match c {
            Command::Reorder { parent, .. } => Some(parent.to_string()),
            _ => None,
        })
        .collect();
    assert!(
        parents.contains("pnl") && parents.contains("form"),
        "{parents:?}"
    );
}

#[test]
fn align_client_inside_panel_takes_panel_not_form() {
    // `AlignControls` runs on each container's `ClientRect`. An `alClient`
    // inside a 300x200 panel becomes 300x200, not 400x300.
    let mut d = with_children();
    d.node_mut("b")
        .unwrap()
        .props
        .insert("align".into(), crawrad_ir::Value::Enum("Client".into()));
    let tx = layout::apply(&d).expect("there had to be something to align");
    let new_rect = tx
        .commands
        .iter()
        .find_map(|c| match c {
            Command::SetRect { node, rect } if node == "b" => Some(rect),
            _ => None,
        })
        .expect("`b` had to be realigned");
    // `a` has no align, so client takes the entire area of the panel
    assert_eq!(*new_rect, Rect::new(0.0, 0.0, 300.0, 200.0));
}

#[test]
fn align_top_on_form_remains_valid_with_panels_in_tree() {
    // Root is the top container, and its cascade did not change.
    let mut d = with_children();
    d.node_mut("tio")
        .unwrap()
        .props
        .insert("align".into(), crawrad_ir::Value::Enum("Top".into()));
    let tx = layout::apply(&d).unwrap();
    let new_rect = tx
        .commands
        .iter()
        .find_map(|c| match c {
            Command::SetRect { node, rect } if node == "tio" => Some(rect),
            _ => None,
        })
        .unwrap();
    assert_eq!(*new_rect, Rect::new(0.0, 0.0, 400.0, 32.0));
}

#[test]
fn clicking_inside_panel_picks_child_not_panel() {
    // Found live: the button appeared inside the panel and could not be
    // selected. Hit-test only scanned root children, so the panel —
    // containing the point — always won, and the child was unreachable.
    let d = with_children();
    let dz = Designer::with_registry(std::rc::Rc::new(Registry::builtin()));
    // `a` is at (10,10) relative to panel (20,20) → (30,30) on form
    assert_eq!(dz.hit(&d, 40.0, 40.0).as_deref(), Some("a"));
    // and a point on the panel having NO child continues grabbing the panel
    assert_eq!(dz.hit(&d, 280.0, 190.0).as_deref(), Some("pnl"));
}

#[test]
fn child_handle_is_where_it_appears() {
    // The pointer arrives in form coordinates; the node `rect` is relative
    // to parent. Comparing both is comparing different spaces — and the handle of
    // a panel child was displaced by the panel's origin.
    let d = with_children();
    let dz = Designer::with_registry(std::rc::Rc::new(Registry::builtin()));
    // top-left corner of `a` on the form: (30, 30)
    assert_eq!(
        dz.hit_handle(&d, &["a".to_string()], 30.0, 30.0),
        Some(Handle::NW)
    );
    // and in the RELATIVE position there is no handle
    assert_eq!(dz.hit_handle(&d, &["a".to_string()], 10.0, 10.0), None);
}

#[test]
fn what_lives_inside_non_visual_is_not_on_screen() {
    // Found live, with two symptoms from a single cause: `Action` items are born
    // at (0,0) RELATIVE to `ActionList`, meaning over its icon.
    // When drawing descended the tree, all started being
    // painted there — labels came out garbled — and click picked the last
    // of them, so `ActionList` stopped opening the editor.
    let mut d = Document::new("form", "Form");
    d.node_mut("form")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    let mut acl = Node::new("acl", "ActionList");
    acl.set_name("ActionList1");
    acl.set_rect(Rect::new(100.0, 40.0, 28.0, 28.0));
    d.add_child("form", acl).unwrap();
    for id in ["EditCopy1", "EditUndo1"] {
        let mut a = Node::new(id, "Action");
        a.set_name(id);
        a.set_rect(Rect::new(0.0, 0.0, 28.0, 28.0));
        d.add_child("acl", a).unwrap();
    }
    d.validate().unwrap();

    let dz = Designer::with_registry(std::rc::Rc::new(Registry::builtin()));
    assert!(
        dz.on_canvas(&d, "acl"),
        "ActionList turns into an icon and appears"
    );
    assert!(
        !dz.on_canvas(&d, "EditUndo1"),
        "Action belongs to ActionList editor, not to the form"
    );
    // and clicking the icon must grab ActionList, not the last Action
    assert_eq!(dz.hit(&d, 110.0, 50.0).as_deref(), Some("acl"));
}

// ---------------------------------------------------------------------------
// The pages of a `PageControl`, on the designer side
//
// The reviewer measured that per-page visibility had ONE test, and it was for
// drawing. `hit`, `hit_handle`, `parent_for_drop`, `hit_tab` and
// `normalize_pages` were covered by nothing — and three of them were
// wrong.
// ---------------------------------------------------------------------------

/// A `PageControl` at (20,20,320,200) with two pages, the first open.
/// Each page has a button at (10,10,96,32) relative to it.
fn doc_with_pages() -> Document {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));

    let mut pc = Node::new("pgc", "PageControl");
    pc.set_name("pgc_cad");
    pc.set_rect(Rect::new(20.0, 20.0, 320.0, 200.0));
    pc.props
        .insert("active_page".into(), crawrad_ir::Value::Int(0));
    d.add_child("form_main", pc).unwrap();

    for (i, (id, btn)) in [("t0", "btn_p0"), ("t1", "btn_p1")].iter().enumerate() {
        let mut t = Node::new(*id, "TabSheet");
        t.set_name(format!("tab{i}"));
        t.set_text(format!("Page{}", i + 1));
        t.set_rect(Rect::new(0.0, 24.0, 320.0, 176.0));
        d.add_child("pgc", t).unwrap();

        let mut b = Node::new(*btn, "Button");
        b.set_name(*btn);
        b.set_rect(Rect::new(10.0, 10.0, 96.0, 32.0));
        d.add_child(id, b).unwrap();
    }
    d
}

/// Designer with catalog — without it nothing knows what is container or what has
/// derived geometry, and all these tests would pass by accident.
fn des_with_registry() -> Designer {
    Designer {
        registry: Some(std::rc::Rc::new(crawrad_registry::Registry::builtin())),
        ..Default::default()
    }
}

#[test]
fn click_does_not_reach_closed_page_components() {
    // Page 2 button is exactly under the same point as page 1
    // — the pages have the SAME rectangle, by derived geometry. Without the visibility
    // filter `hit` would return the top one, which is hidden.
    let des = des_with_registry();
    let d = doc_with_pages();
    // (20+0+10, 20+24+10) = inside the open page button
    assert_eq!(
        des.hit(&d, 40.0, 60.0),
        Some("btn_p0".to_string()),
        "the click must pick the button of the OPEN page"
    );
}

#[test]
fn handle_does_not_appear_for_closed_page_components() {
    // Handle without component underneath is interface lying: the resize
    // cursor appears and dragging does nothing.
    let des = des_with_registry();
    let d = doc_with_pages();
    let r = d.rect_in_form("btn_p1").unwrap();
    assert!(
        des.hit_handle(&d, &["btn_p1".into()], r.x, r.y).is_none(),
        "closed page component cannot have handle"
    );
    assert!(
        des.hit_handle(&d, &["btn_p0".into()], r.x, r.y).is_some(),
        "and open page one must have — otherwise this test proves nothing"
    );
}

#[test]
fn page_has_no_handle_and_does_not_drag() {
    // Derived geometry: `TabSheet` IS the client area of `PageControl`.
    // Dragging it generated motion that normalization undid on the next
    // turn — two undo entries and zero result.
    let des = des_with_registry();
    let d = doc_with_pages();
    let r = d.rect_in_form("t0").unwrap();
    assert!(
        des.hit_handle(&d, &["t0".into()], r.x, r.y).is_none(),
        "page does not resize"
    );
    assert!(
        des.nudge(&d, &["t0".into()], 8.0, 8.0).is_none(),
        "page does not move"
    );
}

#[test]
fn dropping_inside_pagecontrol_lands_on_active_page() {
    // This was wrong and the component VANISHED: both pages have the same
    // rectangle, and the tie-breaker "top one" gave the drop to the last — closed.
    let des = des_with_registry();
    let d = doc_with_pages();
    let (parent, _, _) = des.parent_for_drop(&d, 120.0, 120.0);
    assert_eq!(parent, "t0", "dropping must land on the active page");

    // And with the second page active, the reverse — otherwise the test would pass by
    // accident whenever the first won.
    let mut d2 = doc_with_pages();
    d2.node_mut("pgc")
        .unwrap()
        .props
        .insert("active_page".into(), crawrad_ir::Value::Int(1));
    let (parent, _, _) = des.parent_for_drop(&d2, 120.0, 120.0);
    assert_eq!(parent, "t1");
}

#[test]
fn dropping_on_non_visual_badge_lands_on_form() {
    // `ActionList` **accepts children** — `Action` items live inside it — and
    // **has no area**: it is a 28-point badge. While that was a single flag
    // (`container: true`), `parent_for_drop` accepted the badge as parent, and the
    // component dropped over it ended up INSIDE ActionList — where it
    // disappears, because what lives inside a non-visual does not go to the
    // screen. The user clicked, the button vanished, and nothing said why.
    //
    // The node is born via `instantiate()` (rule 9): 28-pt rectangle and
    // non-visual nature come from the catalog, not from a literal here.
    let reg = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    let acl = reg
        .instantiate(
            "ActionList",
            "ActionList1",
            Rect::new(40.0, 40.0, 28.0, 28.0),
        )
        .expect("class in catalog");
    d.add_child("form_main", acl).unwrap();
    d.validate().unwrap();

    let des = des_with_registry();
    // Right in the middle of the badge, where the defect happened.
    let (parent, x, y) = des.parent_for_drop(&d, 50.0, 50.0);
    assert_eq!(parent, "form_main", "badge is not design surface");
    assert_eq!(
        (x, y),
        (50.0, 50.0),
        "and coordinate remains that of the form"
    );

    // Counter-proof: a `Panel` at the same place RECEIVES the component. Without it,
    // always returning root would pass this test.
    let mut d2 = Document::new("form_main", "Form");
    d2.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    let pnl = reg
        .instantiate("Panel", "Panel1", Rect::new(40.0, 40.0, 200.0, 100.0))
        .expect("class in catalog");
    d2.add_child("form_main", pnl).unwrap();
    let (parent, x, y) = des.parent_for_drop(&d2, 50.0, 50.0);
    assert_eq!(parent, "Panel1", "container with area receives");
    assert_eq!(
        (x, y),
        (10.0, 10.0),
        "and coordinate becomes relative to it"
    );
}

#[test]
fn tab_strip_switches_page_and_plus_creates() {
    let des = des_with_registry();
    let d = doc_with_pages();
    let y = 30.0; // inside strip: 20..44

    assert_eq!(
        des.hit_tab(&d, 25.0, y),
        Some(TabAction::Open {
            control: "pgc".into(),
            page: 0
        })
    );
    assert_eq!(
        des.hit_tab(&d, 20.0 + crawrad_registry::TAB_WIDTH + 5.0, y),
        Some(TabAction::Open {
            control: "pgc".into(),
            page: 1
        })
    );
    // The "+" on the right edge — and HERE is where the defect lived: with it after
    // the last tab, the fifth page was unreachable in a 320pt control.
    assert_eq!(
        des.hit_tab(&d, 20.0 + 320.0 - 5.0, y),
        Some(TabAction::Create {
            control: "pgc".into()
        })
    );
    // Below the strip is content, not tab.
    assert_eq!(des.hit_tab(&d, 25.0, 100.0), None);
}

#[test]
fn tab_strip_scroll_arrows_and_window_navigation() {
    let des = des_with_registry();
    let mut d = doc_with_pages();
    for i in 2..6 {
        let mut t = Node::new(format!("t{i}"), "TabSheet");
        t.set_name(format!("tab{i}"));
        t.set_text(format!("Page{}", i + 1));
        t.set_rect(Rect::new(0.0, 24.0, 320.0, 176.0));
        d.add_child("pgc", t).unwrap();
    }

    // Set active_page = 5
    d.node_mut("pgc")
        .unwrap()
        .props
        .insert("active_page".into(), crawrad_ir::Value::Int(5));

    let y = 30.0;
    // Click at first slot opens page 4
    assert_eq!(
        des.hit_tab(&d, 25.0, y),
        Some(TabAction::Open {
            control: "pgc".into(),
            page: 4,
        })
    );

    let lx = 20.0 + crawrad_registry::tab_scroll_left_x(320.0, true);
    let s = crawrad_registry::TAB_SCROLL_BUTTON_WIDTH;

    // Click on `<` opens 4
    assert_eq!(
        des.hit_tab(&d, lx + 2.0, y),
        Some(TabAction::Open {
            control: "pgc".into(),
            page: 4,
        })
    );

    // Click on `>` returns None (last page)
    assert_eq!(des.hit_tab(&d, lx + s + 2.0, y), None);

    // With active 0 click on `>` opens 1
    d.node_mut("pgc")
        .unwrap()
        .props
        .insert("active_page".into(), crawrad_ir::Value::Int(0));

    assert_eq!(
        des.hit_tab(&d, lx + s + 2.0, y),
        Some(TabAction::Open {
            control: "pgc".into(),
            page: 1,
        })
    );
    assert_eq!(des.hit_tab(&d, lx + 2.0, y), None);
}

#[test]
fn plus_remains_reachable_with_full_strip() {
    // Proof of fix: with enough tabs to overflow the width, the
    // "+" must remain clickable. Previously it went to `n * 92` and exited
    // the control starting from the fourth.
    let des = des_with_registry();
    let mut d = doc_with_pages();
    for i in 2..8 {
        let mut t = Node::new(format!("t{i}"), "TabSheet");
        t.set_name(format!("tab{i}"));
        t.set_text(format!("Page{}", i + 1));
        t.set_rect(Rect::new(0.0, 24.0, 320.0, 176.0));
        d.add_child("pgc", t).unwrap();
    }
    assert_eq!(
        des.hit_tab(&d, 20.0 + 320.0 - 5.0, 30.0),
        Some(TabAction::Create {
            control: "pgc".into()
        }),
        "with eight pages the + must remain reachable"
    );
}

#[test]
fn page_normalization_converges() {
    // `layout::apply` runs after EVERY transaction. If it never converges,
    // each user gesture pushes a spurious transaction to undo,
    // forever. That is what free `Align` on a page caused — measured across
    // six cycles oscillating between two rectangles.
    let mut d = doc_with_pages();
    // Out of position on purpose: normalization must correct it.
    d.node_mut("t0")
        .unwrap()
        .set_rect(Rect::new(50.0, 50.0, 10.0, 10.0));

    let mut doc = Doc::new(d);
    let tx = layout::apply(doc.ir()).expect("the page was out of place");
    doc.apply(tx).unwrap();
    assert_eq!(
        rect(doc.ir(), "t0"),
        Rect::new(0.0, crawrad_registry::TAB_STRIP_HEIGHT, 320.0, 176.0),
        "the page fills the control below the strip"
    );
    assert!(
        layout::apply(doc.ir()).is_none(),
        "already normalized document must not generate transaction — otherwise undo \
         fills with nothing at each gesture"
    );
}

#[test]
fn page_has_no_align_nor_anchors_in_inspector() {
    // Root cause of oscillation: free `align` on a page set the layout
    // pass fighting with normalization for the same rectangle. Fix is in the
    // CATALOG — remove the property —, not a special case in the pass.
    let reg = crawrad_registry::Registry::builtin();
    let d = reg.get("TabSheet").expect("TabSheet in catalog");
    // Margins enter the same list, for the same reason: they only take
    // effect by the hand of `Align`, and the only `Align` the page has is what
    // normalization imposes. Four numbers that the inspector accepts and nothing reads
    // are FALSE state on screen.
    for forbidden in ["align", "anchors", "margins", "align_with_margins"] {
        assert!(
            d.prop(forbidden).is_none(),
            "`TabSheet` cannot have `{forbidden}`: its geometry is derived"
        );
    }
    assert!(
        d.derived_geometry,
        "and flag must be on, otherwise handle and drag return"
    );
    // `Panel` continues with both — otherwise this test would pass by a
    // fix that broke everyone.
    let p = reg.get("Panel").unwrap();
    assert!(p.prop("align").is_some() && !p.derived_geometry);
    assert!(
        p.prop("margins").is_some() && p.prop("align_with_margins").is_some(),
        "`Panel` has margins: without this the above test would pass by a fix \
         that stripped the property from everyone"
    );
    // And the default is that of the classic component library: 3 on each side, with the switch OFF.
    // On by default, every existing form would move on its own upon
    // opening.
    assert_eq!(
        p.prop("margins").unwrap().default,
        crawrad_ir::Value::Edges(crawrad_ir::Edges::uniform(3.0))
    );
    assert_eq!(
        p.prop("align_with_margins").unwrap().default,
        crawrad_ir::Value::Bool(false)
    );
    // `Timer` has no surface and therefore has no margins — and the right way
    // to express that is not being on the `props_de_controle` step, not a
    // filter written separately.
    let t = reg.get("Timer").expect("Timer in catalog");
    assert!(
        t.prop("margins").is_none(),
        "margins on something without borders is a property that nothing reads"
    );
}

// ------------------------------------------------------- margins on screen
//
// The ADR 0019 guard applied to margins. The defect family that this
// repository has paid twice is the same: geometry is valid for DRAWING and
// invalid for HITTING. The symptom is always "handle does not grab".
//
// The wrong implementation has a name: inset margin during PAINTING, in the
// adapter, instead of recording the inset rectangle in the document. It looks right
// on screen and fails on everything clicked — which is why the two tests below
// measure click against where the handle would be IF it existed.

/// Margin used by these tests, and why it is not the default 3.
///
/// Handle target has radius `ALCA_ALVO / 2` (7pt) and still sits `ALCA_FORA`
/// (3.5pt) outside the rectangle. A margin smaller than the sum of both leaves
/// the wrong corner INSIDE the grab radius of the right corner, and the test would pass
/// green with the wrong implementation. Measured: with margin 3, and then with 10,
/// `hit_handle` on form corner still answered `Some(NW)`.
const MARGIN_LARGER_THAN_HANDLE: f32 = 20.0;

/// And relation is enforced at COMPILE time, against the true
/// constants. If one day the handle grows, this stops compiling and whoever made it
/// grow reads why — instead of both tests below remaining green
/// measuring nothing, which is the worst way for a test to die.
const _: () = assert!(
    MARGIN_LARGER_THAN_HANDLE
        > crawrad_runtime::HANDLE_OUTSET + crawrad_runtime::HANDLE_TARGET / 2.0
);

/// A 400x300 form with an `alTop` bar of 30 inset by margins, and
/// alignment already applied.
fn doc_with_inset_bar() -> Doc {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    let mut n = Node::new("barra", "Panel");
    n.set_rect(Rect::new(0.0, 0.0, 10.0, 30.0));
    n.props
        .insert("align".into(), crawrad_ir::Value::Enum("Top".into()));
    n.props.insert(
        "margins".into(),
        crawrad_ir::Value::Edges(crawrad_ir::Edges::uniform(MARGIN_LARGER_THAN_HANDLE)),
    );
    n.props
        .insert("align_with_margins".into(), crawrad_ir::Value::Bool(true));
    d.add_child("form_main", n).unwrap();
    let mut d = Doc::new(d);
    let tx = layout::apply(d.ir()).expect("the bar must be inset");
    d.apply(tx).unwrap();
    assert_eq!(rect(d.ir(), "barra"), Rect::new(20.0, 20.0, 360.0, 30.0));
    d
}

/// Where the bar would be if margin had NOT entered the calculation — the
/// rectangle that the implementation-as-paint-inset would leave in the document.
const BAR_WITHOUT_INSET: Rect = Rect::new(0.0, 0.0, 400.0, 30.0);

#[test]
fn handle_stays_at_corner_inset_by_margin() {
    // Two assertions per handle, and the second is the decider: click at the place
    // where the handle would be without inset must yield NOTHING. Only the first, and a
    // generous grab radius would blur both positions — which is what
    // happened writing this test with margin 3.
    //
    // Defect that ADR 0019 relates in other words: "parent clip rectangle
    // held for child drawing and did not hold for its
    // handles". Today this passes because drawing, hit-test and handle read the SAME
    // `rect_in_form`; the test exists so it remains a single one.
    let des = des_with_registry();
    let d = doc_with_inset_bar();
    let sel = vec!["barra".to_string()];
    let inset_rect = rect(d.ir(), "barra");

    for h in [Handle::NW, Handle::NE, Handle::SE, Handle::SW] {
        let (cx, cy) = h.anchor(inset_rect);
        assert_eq!(
            des.hit_handle(d.ir(), &sel, cx, cy),
            Some(h),
            "handle {h:?} must answer at inset corner"
        );
        let (fx, fy) = h.anchor(BAR_WITHOUT_INSET);
        assert_eq!(
            des.hit_handle(d.ir(), &sel, fx, fy),
            None,
            "handle {h:?} answered at ({fx}, {fy}), which is where it would be if \
             margin were paint inset rather than rectangle in the document"
        );
    }
}

#[test]
fn margin_strip_is_not_clickable_control_area() {
    // The other side of the same coin: margin is space of the CONTAINER, not of the
    // control. Clicking it selects the form, as in the classic IDE — and if
    // hit-test ignored margin, twenty edge points would show the background
    // and click would grab the bar.
    let des = des_with_registry();
    let d = doc_with_inset_bar();

    assert_eq!(des.hit(d.ir(), 200.0, 10.0), None, "above inset top");
    assert_eq!(des.hit(d.ir(), 10.0, 30.0), None, "to the left of inset");
    assert_eq!(
        des.hit(d.ir(), 390.0, 30.0),
        None,
        "to the right: bar ends at 380, not 400"
    );
    assert_eq!(
        des.hit(d.ir(), 200.0, 30.0),
        Some("barra".to_string()),
        "and a point inside must grab the bar"
    );
    // Click on strip lands on the form, which is what the classic IDE selects.
    assert!(des.hit_form(d.ir(), 200.0, 10.0));
}

// ------------------------------------------------------------- Lock Controls
//
// The `Edit ▸ Lock Controls` of the classic IDE. What these tests guard is not "drag
// does not move": it is the boundary between MODIFYING and SELECTING, and its COVERAGE.
//
// The first version of this command only guarded drag `SetRect`, and
// `Align ▸ Left` — two items above `Lock controls` itself, in the same menu
// — moved the entire form with the strip saying "controls locked". That is why
// half of these tests measure the GATE (`refuses`) against REAL
// producers, not mouse gestures.

/// A form born as the user's form is born: components come from
/// catalog `instantiate()`, which is the only path palette uses.
///
/// `Node::new` does not carry descriptor properties (rule 9), and under lock
/// that truly matters: `align` and `anchors` decide what the layout
/// pass does, and that exact boundary is what these tests measure.
fn palette_form() -> Document {
    let reg = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    for (id, class, r) in [
        ("btn_ok", "Button", Rect::new(48.0, 96.0, 96.0, 32.0)),
        ("edt_nome", "Edit", Rect::new(24.0, 48.0, 280.0, 24.0)),
    ] {
        let n = reg.instantiate(class, id, r).expect("class in catalog");
        d.add_child("form_main", n).unwrap();
    }
    d.validate().unwrap();
    d
}

/// The locked designer, **with catalog** — without it `locked_for` would decide
/// correctly by accident, since without registry all is visual and nothing is derived.
fn locked_des() -> Designer {
    Designer {
        locked: true,
        ..des_with_registry()
    }
}

// --------------------------------------------------------------- the gesture

#[test]
fn locked_drag_does_not_move_and_bar_explains_why() {
    let mut des = locked_des();
    let doc = Doc::new(palette_form());
    let before = rect(doc.ir(), "btn_ok");
    let revision = doc.revision();

    // The same gesture as `drag_moves_with_grid_snap`, point by point.
    let o = des.pointer_down(doc.ir(), &["btn_ok".into()], 60.0, 110.0, false);
    assert_eq!(
        o.status.as_deref(),
        Some(LOCKED_WARNING),
        "refusing in silence is the defect, not the fix"
    );
    assert!(
        !o.dragging,
        "nothing is being dragged: whoever reads this field to draw guidelines or \
         switch cursor would paint the interface of a non-existent gesture"
    );
    let o = des.pointer_move(doc.ir(), 77.0, 127.0);
    assert!(o.tx.is_none(), "locked, drag produces no transaction");
    des.pointer_up(doc.ir());

    assert_eq!(rect(doc.ir(), "btn_ok"), before);
    assert_eq!(
        doc.revision(),
        revision,
        "lock belongs to designer: document must not even know a gesture occurred"
    );
}

#[test]
fn unlocked_same_drag_moves_and_dragging_field_follows() {
    // Contrast of test above. Without it, a broken `pointer_down` for
    // everyone would pass both assertions there.
    let mut des = des_with_registry();
    let mut doc = Doc::new(palette_form());
    let o = des.pointer_down(doc.ir(), &["btn_ok".into()], 60.0, 110.0, false);
    assert_eq!(o.status, None);
    assert!(o.dragging);
    let o = des.pointer_move(doc.ir(), 77.0, 127.0);
    doc.apply(o.tx.expect("unlocked, drag moves")).unwrap();
    assert_ne!(rect(doc.ir(), "btn_ok").x, 24.0);
}

#[test]
fn locked_handle_does_not_stretch_and_selection_survives_click() {
    let mut des = locked_des();
    let d = palette_form();
    let sel = vec!["btn_ok".to_string()];
    let r = rect(&d, "btn_ok");
    // Southeast corner, same one `southeast_handle_stretches_down_and_right`
    // uses to stretch.
    let (hx, hy) = Handle::SE.anchor(r);

    let o = des.pointer_down(&d, &sel, hx, hy, false);
    assert_eq!(o.status.as_deref(), Some(LOCKED_WARNING));
    assert!(
        o.select.is_none(),
        "handle is OUTSIDE the rectangle: if clicking it fell into the background, the \
         marquee would clear the selection the user had"
    );
    assert!(des.pointer_move(&d, hx + 40.0, hy + 40.0).tx.is_none());
}

#[test]
fn locked_click_continues_selecting() {
    // Reason command exists: layout stays locked and Object Inspector
    // continues editing everything, typed Left and Top included. Without selection
    // there is no Inspector.
    let mut des = locked_des();
    let d = palette_form();
    let o = des.pointer_down(&d, &[], 40.0, 55.0, false);
    assert_eq!(o.select, Some(vec!["edt_nome".to_string()]));
}

#[test]
fn locked_marquee_continues_selecting_multiple() {
    let mut des = locked_des();
    let d = palette_form();
    des.pointer_down(&d, &[], 350.0, 20.0, false);
    des.pointer_move(&d, 10.0, 200.0);
    let o = des.pointer_up(&d);
    let mut sel = o.select.expect("marquee must select");
    sel.sort();
    assert_eq!(sel, vec!["btn_ok".to_string(), "edt_nome".to_string()]);
}

#[test]
fn locked_form_still_changes_size() {
    // Command is called Lock *Controls*: it locks what is ON the form.
    // In the classic IDE the form window continues being resized via its
    // border even with lock on, and the form's three handles are our
    // border.
    let mut des = locked_des();
    let mut doc = Doc::new(palette_form());
    let root = doc.ir().root.clone();
    let r = doc.ir().node(&root).unwrap().rect().unwrap();
    let (hx, hy) = Handle::SE.anchor(r);

    des.pointer_down(doc.ir(), std::slice::from_ref(&root), hx, hy, false);
    let o = des.pointer_move(doc.ir(), hx + 40.0, hy + 40.0);
    let tx = o.tx.expect("the form is not a control on the form");
    assert!(
        !des.refuses(doc.ir(), &tx),
        "and applier gate must agree with gesture"
    );
    doc.apply(tx).unwrap();
    assert!(doc.ir().node(&root).unwrap().rect().unwrap().w > r.w);
}

// --------------------------------------------------------------- the gate
//
// From here down the mouse does not appear: what is measured is `refuses`, which is
// where rule truly lives, against transactions that REAL producers
// return.

#[test]
fn locked_alignment_palette_does_not_bypass_gate() {
    // Loophole that failed first version. `Align ▸ Left`, `Space equally`,
    // `Same width` and `Center in form` produce `SetRect` without ever passing through
    // designer — none of these functions receives `&Designer`, so guarding
    // drag guarded none of this.
    let d = palette_form();
    let sel = vec!["btn_ok".to_string(), "edt_nome".to_string()];
    let locked = locked_des();
    let free = des_with_registry();

    // Transactions come from real producers, not hand-crafted here.
    let productions: Vec<(&str, Transaction)> = vec![
        (
            "Align ▸ Left",
            arrangement::align(&d, &sel, AlignMode::Left, Reference::First)
                .expect("align produces transaction"),
        ),
        (
            "Size ▸ Same width",
            arrangement::equalize(&d, &sel, Equalize::Width)
                .expect("equalize produces transaction"),
        ),
        (
            "In form ▸ Center horizontally",
            arrangement::center_in_form(&d, &sel, AlignMode::CenterHorizontal)
                .expect("center produces transaction"),
        ),
    ];

    for (name, tx) in productions {
        assert!(
            locked.refuses(&d, &tx),
            "`{name}` passed through gate with lock on — loophole again"
        );
        assert!(
            !free.refuses(&d, &tx),
            "`{name}` cannot be refused with lock off"
        );
    }
}

#[test]
fn locked_keyboard_arrow_is_refused_at_same_gate_and_therefore_notifies() {
    // Arrow does not check lock: it produces `SetRect` like any producer and
    // the gate refuses. By design — while `nudge` returned
    // `None` alone, locked arrow was total silence, whereas locked
    // mouse explained what happened. Through gate, same sentence serves both.
    let des = locked_des();
    let d = palette_form();
    let tx = des
        .nudge(&d, &["btn_ok".to_string()], 1.0, 0.0)
        .expect("arrow continues producing transaction — gate is what refuses");
    assert!(des.refuses(&d, &tx));
    assert!(
        !des_with_registry().refuses(&d, &tx),
        "unlocked, same transaction passes"
    );
}

#[test]
fn locked_non_geometry_continues_passing() {
    // Counterbalance of gate, and reason `match` is exhaustive over
    // `Command`: it is Lock *Controls*, not Lock Properties. Object Inspector
    // writes `SetProp` — typed Left and Top included —, Delete writes
    // `RemoveNode`, palette writes `AddNode` and Z-order writes `Reorder`.
    // None of them is "modifying by gesture".
    let des = locked_des();
    let d = palette_form();
    let reg = Registry::builtin();

    let inspector_left = Transaction::user("left").with(Command::SetProp {
        node: "btn_ok".into(),
        key: "left".into(),
        value: Some(crawrad_ir::Value::Float(300.0)),
    });
    let rename = Transaction::user("rename").with(Command::Rename {
        node: "btn_ok".into(),
        name: "btn_gravar".into(),
        handlers: true,
    });
    let event = Transaction::user("evento").with(Command::SetEvent {
        node: "btn_ok".into(),
        event: "on_click".into(),
        handler: Some("btn_okClick".into()),
    });
    let drop_tx = Transaction::user("soltar").with(Command::AddNode {
        parent: "form_main".into(),
        node: reg
            .instantiate("Label", "lbl_1", Rect::new(8.0, 8.0, 64.0, 16.0))
            .unwrap(),
        index: 2,
    });
    let delete_tx = Transaction::user("delete").with(Command::RemoveNode {
        node: "btn_ok".into(),
    });
    let z_order_tx = arrangement::z_order(&d, &["btn_ok".to_string()], ZOrder::Front)
        .expect("Z order produces transaction");
    // `AlignWithMargins` MOVES the control, and still passes. Decision is
    // deliberate and this line exists so nobody "fixes" it without reading:
    //
    // blocking it would require `refuses` inspecting KEY of `SetProp`, i.e.,
    // maintaining list of "which properties are geometry" — `align`,
    // `anchors`, `margins`, `align_with_margins`, `left`, `top`, `visible`…
    // That list is the second copy of the rule that Amendment 1 of ADR 0005 exists
    // to kill (the design notes), and defect
    // it describes is exactly this: fifteen producers, five guarded,
    // ten free, and bar announcing a state that did not hold.
    //
    // And it is not a new case: switching `alNone` to `alTop` in inspector already moves
    // the control from edge to edge with lock on, and typed `Left` too —
    // written in `locked_for`. It is Lock *Controls*, not Lock Properties.
    let margins = Transaction::user("AlignWithMargins").with(Command::SetProp {
        node: "btn_ok".into(),
        key: "align_with_margins".into(),
        value: Some(crawrad_ir::Value::Bool(true)),
    });

    for (name, tx) in [
        ("Inspector writing Left", inspector_left),
        ("Inspector enabling AlignWithMargins", margins),
        ("Rename", rename),
        ("bind event", event),
        ("drop from palette", drop_tx),
        ("Delete", delete_tx),
        ("Bring to front", z_order_tx),
    ] {
        assert!(
            !des.refuses(&d, &tx),
            "`{name}` does not modify geometry via gesture — locking this would \
             turn the lock into a read-only mode that nobody requested"
        );
    }
}

#[test]
fn locked_reparent_is_refused_because_it_carries_rectangle() {
    // `Reparent` is drag dropping control inside a container:
    // parent and position in same command (ADR 0012). Letting it pass would let
    // movement pass through the back door.
    let des = locked_des();
    let d = palette_form();
    let tx = Transaction::user("Move into container").with(Command::Reparent {
        node: "btn_ok".into(),
        parent: "form_main".into(),
        index: 0,
        rect: Rect::new(0.0, 0.0, 96.0, 32.0),
    });
    assert!(des.refuses(&d, &tx));
}

#[test]
fn anchors_pass_would_be_refused_and_that_is_why_it_bypasses_the_gate() {
    // Pins O4 boundary: resizing form MOVES children, with
    // lock on, and this is faithful to classic IDE — there `alClient` follows
    // locked window. What makes this work is shell applying this pass
    // directly on `Doc`, without going through gate.
    //
    // Test measures gate, which is what can be measured here: if one day someone
    // sends `reacomodar` through `apply`, this assertion is what explains why
    // anchors stopped working.
    let mut d = palette_form();
    // Anchored to right: classic IDE makes this follow edge.
    d.node_mut("btn_ok").unwrap().props.insert(
        "anchors".into(),
        layout::Anchors {
            left: false,
            top: true,
            right: true,
            bottom: false,
        }
        .write(),
    );

    let tx = layout::apply_anchors(&d, 40.0, 0.0).expect("anchor reacts to new width");
    assert!(
        locked_des().refuses(&d, &tx),
        "anchors pass is `SetRect` on child: through gate it WOULD BE \
         refused, which is precisely why `reacomodar` does not pass there"
    );
}

// ------------------------------------------------- The three form handles
//
// The classic IDE resizes the form via its border, and does not ask for selection
// beforehand: you point to border and drag. Here the three handles — E, S and SE — play
// the role of that border, and adapter DRAWS them always, with resize
// arrow, whether form is selected or not.
//
// What these tests guard is ORDER of `pointer_down`, where
// fix could become a second defect: selected component handle
// must continue beating form handle when both land on same
// point — which happens with every control flush against right or
// bottom border.

/// Same 400x300 form from `palette_form`, with a button **flush**
/// against bottom-right corner: its SE handle lands exactly on form's SE handle.
fn form_with_button_flush_in_corner() -> Document {
    let reg = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    // 300+100 = 400 and 268+32 = 300: both edges coincide with form's.
    let n = reg
        .instantiate("Button", "btn_canto", Rect::new(300.0, 268.0, 100.0, 32.0))
        .expect("class in catalog");
    d.add_child("form_main", n).unwrap();
    d.validate().unwrap();
    d
}

/// Form SE handle and flush button handle are the SAME point — if
/// they stop being, both order tests below start measuring nothing.
#[test]
fn flush_button_handle_falls_on_top_of_form_handle() {
    let d = form_with_button_flush_in_corner();
    let form_rect = d.node("form_main").unwrap().rect().unwrap();
    let btn_rect = rect(&d, "btn_canto");
    assert_eq!(
        Handle::SE.anchor(form_rect),
        Handle::SE.anchor(btn_rect),
        "without coincidence there is no dispute for order to resolve"
    );
}

#[test]
fn form_handle_grabs_without_form_being_selected() {
    // Owner complaint, point by point: aims handle to resize
    // form WITHOUT selecting it first, and instead of stretching a
    // marquee appears — outside form area, because handle lives 3.5
    // units outside the border.
    let mut des = des_with_registry();
    let mut doc = Doc::new(palette_form());
    let root = doc.ir().root.clone();
    let r = doc.ir().node(&root).unwrap().rect().unwrap();
    let (hx, hy) = Handle::SE.anchor(r);

    // Nothing selected, how form arrives when IDE opens.
    let o = des.pointer_down(doc.ir(), &[], hx, hy, false);
    assert!(o.dragging, "handle must start a gesture");

    let o = des.pointer_move(doc.ir(), hx + 40.0, hy + 44.0);
    assert!(
        o.marquee.is_none(),
        "this is the defect: handle became selection marquee"
    );
    let tx = o.tx.expect("form handle must stretch the form");
    doc.apply(tx).unwrap();

    let new_rect = doc.ir().node(&root).unwrap().rect().unwrap();
    assert_eq!(
        (new_rect.w, new_rect.h),
        (440.0, 344.0),
        "form had to have grown"
    );
    assert_eq!(
        (new_rect.x, new_rect.y),
        (r.x, r.y),
        "and form origin is fixed"
    );
}

#[test]
fn grabbing_form_handle_selects_form() {
    // In classic IDE, modifying form size hands Object Inspector to
    // it. And there is internal reason: handles hand is holding are the
    // form's — leaving selection on a component would mean screen showing
    // handles of one object and gesture modifying another.
    let mut des = des_with_registry();
    let doc = Doc::new(palette_form());
    let root = doc.ir().root.clone();
    let r = doc.ir().node(&root).unwrap().rect().unwrap();
    let (hx, hy) = Handle::E.anchor(r);

    let o = des.pointer_down(doc.ir(), &["btn_ok".into()], hx, hy, false);
    assert_eq!(
        o.select,
        Some(vec![root.clone()]),
        "whoever grabs form handle is editing the form"
    );
    des.cancel();

    // And with it ALREADY selected no selection is emitted: `Some` here would be
    // shell republishing same selection at each grab — Object Inspector
    // reconstructed mid-gesture, without anything having changed.
    let o = des.pointer_down(doc.ir(), std::slice::from_ref(&root), hx, hy, false);
    assert_eq!(o.select, None, "nothing changed: nothing to publish");
    assert!(o.dragging, "and gesture continues being a resize");
}

#[test]
fn component_handle_beats_form_handle_at_same_point() {
    // Test that prevents fix from becoming defect. Control flush with
    // border has handle at same pixel as form handle; if form
    // came first, user would lose resizing of every
    // component touching right or bottom margin.
    let mut des = des_with_registry();
    let mut doc = Doc::new(form_with_button_flush_in_corner());
    let form_rect = doc.ir().node("form_main").unwrap().rect().unwrap();
    let (hx, hy) = Handle::SE.anchor(rect(doc.ir(), "btn_canto"));

    des.pointer_down(doc.ir(), &["btn_canto".into()], hx, hy, false);
    let o = des.pointer_move(doc.ir(), hx + 40.0, hy + 44.0);
    let tx = o.tx.expect("button handle must stretch the button");
    let targets: Vec<String> = tx
        .commands
        .iter()
        .map(|c| match c {
            Command::SetRect { node, .. } => node.clone(),
            other => format!("{other:?}"),
        })
        .collect();
    assert_eq!(
        targets,
        vec!["btn_canto".to_string()],
        "gesture grabbed form instead of selected component"
    );
    doc.apply(tx).unwrap();
    assert_eq!(
        doc.ir().node("form_main").unwrap().rect().unwrap(),
        form_rect,
        "and form could not have moved along"
    );
}

#[test]
fn form_has_three_handles_not_eight() {
    // Other side of same doorway, and defect of same family: with form
    // SELECTED, step 1 gave it the EIGHT handles of
    // `hit_handle`, although adapter draws three. The five invisible ones grabbed
    // — clicking 14 units from top-left corner of form started
    // resize nobody asked for, in a corner where screen shows
    // nothing, and `pointer_move` still pins form origin: you pull
    // top corner and bottom edge is what moves.
    //
    // In classic IDE form stretches via right edge, bottom edge and
    // corner between the two. Other five do not exist there and cannot
    // exist here.
    let mut des = des_with_registry();
    let d = palette_form();
    let root = d.root.clone();
    let form_rect = d.node(&root).unwrap().rect().unwrap();
    let selected = std::slice::from_ref(&root);

    for h in [Handle::NW, Handle::N, Handle::NE, Handle::W, Handle::SW] {
        let (hx, hy) = h.anchor(form_rect);
        assert_eq!(
            des.hit_handle(&d, selected, hx, hy),
            None,
            "handle {h:?} of form is not drawn, and cannot grab"
        );
        des.pointer_down(&d, selected, hx, hy, false);
        let o = des.pointer_move(&d, hx + 40.0, hy + 44.0);
        assert!(o.tx.is_none(), "handle {h:?} of form changed its size");
        des.cancel();
    }

    // And the three that exist continue existing.
    for h in [Handle::E, Handle::S, Handle::SE] {
        let (hx, hy) = h.anchor(form_rect);
        assert_eq!(des.hit_form_handle(&d, hx, hy), Some(h));
    }
}

#[test]
fn marquee_continues_originating_outside_form_on_purpose() {
    // Screenshot of complaint shows marquee starting OUTSIDE form, and the
    // question is fair: is starting marquee there a legitimate gesture?
    //
    // Yes, and decision is deliberate. What was wrong in screenshot was not
    // place, it was step: point landed inside target of SE handle, and handle
    // had to win — test above guards that. Outside target
    // of handle, gray area around form is canvas table (same
    // canvas having ruler and zoom, ADR 0019), and starting marquee there is the ONLY
    // comfortable way to lasso control touching border: starting
    // inside, first click would fall over it and turn into drag.
    //
    // Classic IDE does not have this table — there, outside form is another window —,
    // so visual contract does not answer this, and canvas that
    // already exists answers.
    let mut des = des_with_registry();
    let d = palette_form();
    let form_rect = d.node("form_main").unwrap().rect().unwrap();
    // 60 units beyond corner: outside form AND outside target of SE handle,
    // which ends at (403.5 + 7; 303.5 + 7).
    let (px, py) = (form_rect.w + 60.0, form_rect.h + 60.0);
    assert_eq!(
        des.hit_form_handle(&d, px, py),
        None,
        "starting point must be outside handle target, otherwise this \
         test measures opposite of what it states"
    );

    des.pointer_down(&d, &[], px, py, false);
    des.pointer_move(&d, 20.0, 40.0);
    let mut sel = des.pointer_up(&d).select.expect("marquee must select");
    sel.sort();
    assert_eq!(sel, vec!["btn_ok".to_string(), "edt_nome".to_string()]);
}

#[test]
fn without_component_selected_same_point_grabs_form_handle() {
    // Other side: handle only exists for what is selected, so at same
    // point, with button unselected, what answers is form.
    let mut des = des_with_registry();
    let mut doc = Doc::new(form_with_button_flush_in_corner());
    let btn_rect = rect(doc.ir(), "btn_canto");
    let (hx, hy) = Handle::SE.anchor(btn_rect);

    des.pointer_down(doc.ir(), &[], hx, hy, false);
    let o = des.pointer_move(doc.ir(), hx + 40.0, hy + 44.0);
    let tx =
        o.tx.expect("without selection, what answers is form handle");
    doc.apply(tx).unwrap();

    let new_rect = doc.ir().node("form_main").unwrap().rect().unwrap();
    assert_eq!((new_rect.w, new_rect.h), (440.0, 344.0));
    assert_eq!(
        rect(doc.ir(), "btn_canto"),
        btn_rect,
        "and button, which was not selected, stays where it was"
    );
}

// ---------------------------------------------------------------------
// THE FOUR KEYBOARD ARROWS — the map dictated by owner on 2026-09-17
// ---------------------------------------------------------------------

/// **Step of each chord comes from GRID**, not from number written in shell.
///
/// Reason [`Designer::arrow_step`] exists. With step resolved in
/// each IDE, changing `Grid size` in options would change arrow of one and not
/// the other — and discrepancy would appear as *"here it moves 8, there it moves 1"*, far
/// from where it was decided.
#[test]
fn arrow_step_follows_grid() {
    let mut des = Designer {
        snap: true,
        grid: 8.0,
        ..Default::default()
    };
    assert_eq!(
        des.arrow_step(ArrowNudge::Move),
        8.0,
        "plain arrow moves one grid step"
    );
    assert_eq!(
        des.arrow_step(ArrowNudge::Resize),
        8.0,
        "`Shift+arrow` grows one grid step, as plain arrow moves one"
    );
    assert_eq!(
        des.arrow_step(ArrowNudge::MoveFast),
        24.0,
        "`Ctrl+Shift+arrow` moves three steps — and owner stated the number"
    );
    assert_eq!(
        des.arrow_step(ArrowNudge::MoveFine),
        1.0,
        "`Ctrl+arrow` is FINE adjustment: without it, plain arrow moving from grid \
         to grid leaves keyboard without any way to land off it"
    );

    des.grid = 16.0;
    assert_eq!(
        des.arrow_step(ArrowNudge::Move),
        16.0,
        "changing `Grid size` in options must change arrow along with it"
    );
}

/// **With snap off grid commands nothing**, and arrow moves one by one.
///
/// Same pair of states mouse respects: without `Snap to grid`,
/// positioning is free. Arrow that continued jumping eight by eight would be
/// unchecked `Snap to grid` applying to mouse and not to keyboard.
#[test]
fn without_snap_arrow_moves_one_by_one() {
    let des = Designer {
        snap: false,
        grid: 8.0,
        ..Default::default()
    };
    assert_eq!(des.arrow_step(ArrowNudge::Move), 1.0);
    assert_eq!(des.arrow_step(ArrowNudge::Resize), 1.0);
    assert_eq!(des.arrow_step(ArrowNudge::MoveFast), 3.0);
}

/// `Shift+arrow` changes SIZE and leaves origin put — corner of classic IDE.
#[test]
fn resize_arrow_grows_via_bottom_right_corner() {
    let des = Designer::default();
    let mut doc = Doc::new(doc());
    let before = rect(doc.ir(), "btn_ok");
    let tx = des
        .nudge_size(doc.ir(), &["btn_ok".to_string()], 8.0, 0.0)
        .unwrap();
    doc.apply(tx).unwrap();
    let after = rect(doc.ir(), "btn_ok");
    assert_eq!(
        (after.x, after.y),
        (before.x, before.y),
        "origin moved: `Shift+arrow` grows via bottom-right corner, and \
         component cannot move out of place while changing size"
    );
    assert_eq!((after.w, after.h), (before.w + 8.0, before.h));
}

/// **Shrinking does not invert rectangle**, and at floor arrow simply does nothing.
///
/// Inverted rectangle is component that vanishes from canvas without saying anything. And a
/// transaction that changes nothing is an undo entry that undoes nothing —
/// worse than not answering, because it seems it did.
#[test]
fn resize_arrow_does_not_invert_rectangle() {
    let des = Designer::default();
    let mut doc = Doc::new(doc());
    let before = rect(doc.ir(), "btn_ok");
    let tx = des
        .nudge_size(doc.ir(), &["btn_ok".to_string()], -(before.w + 50.0), 0.0)
        .unwrap();
    doc.apply(tx).unwrap();
    assert_eq!(
        rect(doc.ir(), "btn_ok").w,
        0.0,
        "shrunk to the floor, and stopped"
    );

    assert!(
        des.nudge_size(doc.ir(), &["btn_ok".to_string()], -8.0, 0.0)
            .is_none(),
        "at floor, arrow still produced transaction — an undo \
         entry that undoes nothing"
    );
}

#[test]
fn without_selection_there_is_no_resize() {
    let des = Designer::default();
    let d = doc();
    assert!(des.nudge_size(&d, &[], 8.0, 0.0).is_none());
}

/// A `StatusPanel` inside its `StatusBar` is drawn by the bar, so it has
/// no place of its own on the canvas, while a `Timer` dropped on a `Panel`
/// keeps its badge: the rule is "the owner collects this class", not "a
/// non-visual under a visual".
#[test]
fn a_collection_item_on_a_visual_owner_is_drawn_by_it() {
    let mut d = doc();
    let mut sb = Node::new("sb", "StatusBar");
    sb.set_rect(Rect::new(0.0, 300.0, 320.0, 20.0));
    d.add_child("form_main", sb).unwrap();
    let mut stp = Node::new("stp", "StatusPanel");
    stp.set_rect(Rect::new(0.0, 0.0, 28.0, 28.0));
    d.add_child("sb", stp).unwrap();
    let mut pnl = Node::new("pnl", "Panel");
    pnl.set_rect(Rect::new(400.0, 8.0, 120.0, 80.0));
    d.add_child("form_main", pnl).unwrap();
    let mut tmr = Node::new("tmr", "Timer");
    tmr.set_rect(Rect::new(8.0, 8.0, 28.0, 28.0));
    d.add_child("pnl", tmr).unwrap();

    let des = Designer::with_registry(std::rc::Rc::new(crawrad_registry::Registry::builtin()));
    assert!(!des.on_canvas(&d, "stp"), "the bar draws its panel");
    assert!(des.on_canvas(&d, "sb"));
    assert!(
        des.on_canvas(&d, "tmr"),
        "a Timer on a Panel keeps its badge"
    );
}

/// Guarding test: form resize handles span entire border within 6px margin.
#[test]
fn form_resize_handles_span_entire_border() {
    let mut d = Document::new("form_main", "Form");
    let root = d.root.clone();
    if let Some(form) = d.node_mut(&root) {
        form.set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    }
    let des = Designer::default();

    assert_eq!(des.hit_form_handle(&d, 400.0, 300.0), Some(Handle::SE));

    assert_eq!(
        des.hit_form_handle(&d, 402.0, 20.0),
        Some(Handle::E),
        "right edge should respond along the entire height within 6px"
    );
    assert_eq!(
        des.hit_form_handle(&d, 398.0, 280.0),
        Some(Handle::E),
        "right edge should respond near bottom within 6px"
    );

    assert_eq!(
        des.hit_form_handle(&d, 20.0, 302.0),
        Some(Handle::S),
        "bottom edge should respond along the entire width within 6px"
    );
    assert_eq!(
        des.hit_form_handle(&d, 380.0, 298.0),
        Some(Handle::S),
        "bottom edge should respond near right corner within 6px"
    );

    assert_eq!(des.hit_form_handle(&d, 410.0, 20.0), None);
    assert_eq!(des.hit_form_handle(&d, 20.0, 310.0), None);
}

#[test]
fn a_non_visual_under_a_panel_is_still_what_the_click_grabs() {
    // Found live: an ActionList dropped over a Panel shows its badge in
    // front, and the click moved the panel instead.
    let mut d = Document::new("form", "Form");
    d.node_mut("form")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    let mut acl = Node::new("acl", "ActionList");
    acl.set_name("ActionList1");
    acl.set_rect(Rect::new(100.0, 40.0, 28.0, 28.0));
    d.add_child("form", acl).unwrap();
    let mut pnl = Node::new("pnl", "Panel");
    pnl.set_name("Panel1");
    pnl.set_rect(Rect::new(50.0, 20.0, 200.0, 150.0));
    d.add_child("form", pnl).unwrap();
    d.validate().unwrap();

    let dz = Designer::with_registry(std::rc::Rc::new(Registry::builtin()));
    assert_eq!(dz.hit(&d, 110.0, 50.0).as_deref(), Some("acl"));
    assert_eq!(dz.hit(&d, 200.0, 150.0).as_deref(), Some("pnl"));
}

#[test]
fn default_size_of_newly_dropped_inheriting_component_matches_base() {
    let mut reg = Registry::builtin();
    reg.register_external(crawrad_registry::sample_price_edit())
        .unwrap();

    let spin_edit = reg.get("SpinEdit").unwrap();
    let price_edit = reg.get("PriceEdit").unwrap();
    let button = reg.get("Button").unwrap();

    // Default size of newly dropped PriceEdit == that of a SpinEdit
    assert_eq!(price_edit.default_size, spin_edit.default_size);

    // When dropped and instantiated via Registry
    let node_spin = reg
        .instantiate(
            "SpinEdit",
            "spn1",
            Rect::new(
                10.0,
                10.0,
                spin_edit.default_size.0,
                spin_edit.default_size.1,
            ),
        )
        .unwrap();
    let node_price = reg
        .instantiate(
            "PriceEdit",
            "ped1",
            Rect::new(
                10.0,
                10.0,
                price_edit.default_size.0,
                price_edit.default_size.1,
            ),
        )
        .unwrap();
    let node_btn = reg
        .instantiate(
            "Button",
            "btn1",
            Rect::new(10.0, 10.0, button.default_size.0, button.default_size.1),
        )
        .unwrap();

    let (rp, rs) = (node_price.rect().unwrap(), node_spin.rect().unwrap());
    assert_eq!((rp.w, rp.h), (rs.w, rs.h));

    // Designer visual check treats inheriting component as visual like base
    let mut doc = Document::new("form", "Form");
    doc.node_mut("form")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    doc.add_child("form", node_price).unwrap();
    doc.add_child("form", node_btn).unwrap();

    let designer = Designer::with_registry(std::rc::Rc::new(reg));
    assert!(designer.visual(&doc, &crawrad_ir::NodeId::from("ped1")));
    assert!(designer.on_canvas(&doc, "ped1"));
    assert!(designer.visual(&doc, &crawrad_ir::NodeId::from("btn1")));
}
