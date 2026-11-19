use super::*;
use crawrad_doc::Doc;
use crawrad_ir::Node;

/// Form of 400x300 and whatever components the test wants.
fn doc(comps: &[(&str, Rect, &str)]) -> Doc {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    for (id, r, align) in comps {
        let mut n = Node::new(*id, "Button");
        n.set_rect(*r);
        n.props.insert("align".into(), Value::Enum((*align).into()));
        d.add_child("form_main", n).unwrap();
    }
    Doc::new(d)
}

fn rect(d: &Doc, id: &str) -> Rect {
    d.ir().node(id).unwrap().rect().unwrap()
}

#[test]
fn align_top_occupies_full_width_and_keeps_height() {
    let mut d = doc(&[("bar", Rect::new(37.0, 91.0, 50.0, 30.0), "Top")]);
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(rect(&d, "bar"), Rect::new(0.0, 0.0, 400.0, 30.0));
}

#[test]
fn align_bottom_sticks_to_bottom() {
    let mut d = doc(&[("status", Rect::new(0.0, 0.0, 50.0, 20.0), "Bottom")]);
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(rect(&d, "status"), Rect::new(0.0, 280.0, 400.0, 20.0));
}

#[test]
fn align_left_and_right_split_what_remains_in_z_order() {
    let mut d = doc(&[
        ("top", Rect::new(0.0, 0.0, 10.0, 40.0), "Top"),
        ("left", Rect::new(0.0, 0.0, 80.0, 10.0), "Left"),
        ("right", Rect::new(0.0, 0.0, 60.0, 10.0), "Right"),
    ]);
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();

    assert_eq!(rect(&d, "top"), Rect::new(0.0, 0.0, 400.0, 40.0));
    // the left side begins BELOW the top bar
    assert_eq!(rect(&d, "left"), Rect::new(0.0, 40.0, 80.0, 260.0));
    assert_eq!(rect(&d, "right"), Rect::new(340.0, 40.0, 60.0, 260.0));
}

#[test]
fn align_client_takes_what_remains_of_all() {
    let mut d = doc(&[
        ("top", Rect::new(0.0, 0.0, 10.0, 40.0), "Top"),
        ("bottom", Rect::new(0.0, 0.0, 10.0, 20.0), "Bottom"),
        ("left", Rect::new(0.0, 0.0, 80.0, 10.0), "Left"),
        ("middle", Rect::new(0.0, 0.0, 10.0, 10.0), "Client"),
    ]);
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(rect(&d, "middle"), Rect::new(80.0, 40.0, 320.0, 240.0));
}

#[test]
fn client_is_calculated_last_even_if_in_the_middle_of_order() {
    let mut d = doc(&[
        ("middle", Rect::new(0.0, 0.0, 10.0, 10.0), "Client"),
        ("top", Rect::new(0.0, 0.0, 10.0, 40.0), "Top"),
    ]);
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(
        rect(&d, "middle"),
        Rect::new(0.0, 40.0, 400.0, 260.0),
        "client must respect elements that aligned after it"
    );
}

#[test]
fn align_none_stays_where_it_is() {
    let d = doc(&[("free", Rect::new(37.0, 91.0, 50.0, 30.0), "None")]);
    assert!(apply(d.ir()).is_none(), "nothing to do, nothing in history");
}

#[test]
fn already_aligned_does_not_generate_transaction() {
    let mut d = doc(&[("bar", Rect::new(0.0, 0.0, 50.0, 30.0), "Top")]);
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();
    assert!(apply(d.ir()).is_none());
}

#[test]
fn resize_form_readjusts_all() {
    let mut d = doc(&[
        ("top", Rect::new(0.0, 0.0, 400.0, 40.0), "Top"),
        ("middle", Rect::new(0.0, 40.0, 400.0, 260.0), "Client"),
    ]);
    // the form shrinks
    let tx = crawrad_doc::Transaction::user("resize form").with(Command::SetRect {
        node: "form_main".into(),
        rect: Rect::new(0.0, 0.0, 200.0, 150.0),
    });
    d.apply(tx).unwrap();

    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(rect(&d, "top"), Rect::new(0.0, 0.0, 200.0, 40.0));
    assert_eq!(rect(&d, "middle"), Rect::new(0.0, 40.0, 200.0, 110.0));
}

#[test]
fn aligned_component_is_fixed() {
    let d = doc(&[
        ("bar", Rect::new(0.0, 0.0, 50.0, 30.0), "Top"),
        ("free", Rect::new(0.0, 0.0, 50.0, 30.0), "None"),
    ]);
    assert!(is_fixed(d.ir(), "bar"));
    assert!(!is_fixed(d.ir(), "free"));
}

#[test]
fn all_fits_in_one_ctrl_z() {
    let mut d = doc(&[
        ("top", Rect::new(0.0, 0.0, 10.0, 40.0), "Top"),
        ("bottom", Rect::new(0.0, 0.0, 10.0, 20.0), "Bottom"),
    ]);
    let before = (rect(&d, "top"), rect(&d, "bottom"));
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();
    d.undo().unwrap();
    assert_eq!((rect(&d, "top"), rect(&d, "bottom")), before);
}

// ---------------------------------------------------------------- anchors

fn with_anchors(id: &str, r: Rect, a: Anchors) -> Doc {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    let mut n = Node::new(id, "Button");
    n.set_rect(r);
    n.props.insert("anchors".into(), a.write());
    d.add_child("form_main", n).unwrap();
    Doc::new(d)
}

#[test]
fn default_anchor_leaves_component_still() {
    let mut d = with_anchors("b", Rect::new(10.0, 10.0, 80.0, 24.0), Anchors::default());
    let tx = apply_anchors(d.ir(), 100.0, 50.0);
    assert!(tx.is_none(), "akLeft+akTop does not move when form grows");
    let _ = &mut d;
}

#[test]
fn anchored_to_right_follows_edge() {
    let a = Anchors {
        left: false,
        top: true,
        right: true,
        bottom: false,
    };
    let mut d = with_anchors("b", Rect::new(300.0, 10.0, 80.0, 24.0), a);
    let tx = apply_anchors(d.ir(), 100.0, 0.0).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(rect(&d, "b"), Rect::new(400.0, 10.0, 80.0, 24.0));
}

#[test]
fn anchored_on_both_sides_stretches() {
    let a = Anchors {
        left: true,
        top: true,
        right: true,
        bottom: false,
    };
    let mut d = with_anchors("b", Rect::new(10.0, 10.0, 380.0, 24.0), a);
    let tx = apply_anchors(d.ir(), 100.0, 0.0).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(rect(&d, "b").w, 480.0, "stretched with form");
    assert_eq!(rect(&d, "b").x, 10.0, "and left stayed where it was");
}

#[test]
fn anchored_at_bottom_goes_down_when_form_grows() {
    let a = Anchors {
        left: true,
        top: false,
        right: false,
        bottom: true,
    };
    let mut d = with_anchors("b", Rect::new(10.0, 260.0, 80.0, 24.0), a);
    let tx = apply_anchors(d.ir(), 0.0, 60.0).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(rect(&d, "b"), Rect::new(10.0, 320.0, 80.0, 24.0));
}

#[test]
fn component_with_align_ignores_anchors() {
    let mut d = doc(&[("bar", Rect::new(0.0, 0.0, 400.0, 40.0), "Top")]);
    assert!(
        apply_anchors(d.ir(), 100.0, 100.0).is_none(),
        "Align rules; anchors do not enter the conversation"
    );
    let _ = &mut d;
}

/// The four anchor combinations of an axis, constructed in both axes.
///
/// The pair is `(near, far)`: the origin edge and the opposite one. Constructing both
/// axes with the SAME pair is what makes an axis swap appear.
fn combinations() -> Vec<(&'static str, Anchors)> {
    let mut v = Vec::new();
    for (near, far) in [(true, true), (false, true), (true, false), (false, false)] {
        v.push((
            "horizontal",
            Anchors {
                left: near,
                right: far,
                top: true,
                bottom: false,
            },
        ));
        v.push((
            "vertical",
            Anchors {
                left: true,
                right: false,
                top: near,
                bottom: far,
            },
        ));
    }
    v
}

/// **Shrinking well past zero and growing back returns the exact rectangle.**
///
/// This is the defect that the owner saw on screen with a `TreeView` anchored on all four:
/// *"the error is when it reaches H/W = 0 — then, when trying to go back, it gets lost"*.
/// The cause was `w.max(1.0)` in `Anchors::adjust`: application is
/// incremental, and whatever remained of the shrinking was thrown away.
///
/// Measured before the fix, with 380 x 280 anchored on all four: shrinking 500 and
/// growing 500 returned **501 x 501**, meaning 121 pt and 221 pt extra.
///
/// `N` is large on purpose — larger than the component AND the form,
/// so the rectangle genuinely crosses zero. A small `N` passes even
/// with the floor back, and would be a test that holds nothing.
#[test]
fn round_trip_through_floor_returns_exact_rectangle() {
    const N: f32 = 500.0;
    let base = Rect::new(10.0, 10.0, 380.0, 280.0);

    for (axis, a) in combinations() {
        let mut d = with_anchors("b", base, a);
        if let Some(tx) = apply_anchors(d.ir(), -N, -N) {
            d.apply(tx).unwrap();
        }
        let at_floor = rect(&d, "b");
        if let Some(tx) = apply_anchors(d.ir(), N, N) {
            d.apply(tx).unwrap();
        }
        let back = rect(&d, "b");

        assert_eq!(
            back,
            base,
            "\nTHE ANCHOR GOT LOST ON RETURN - axis {axis}, {a:?}\n\
             design        : {base:?}\n\
             after -{N} : {at_floor:?}\n\
             after +{N} : {back:?}\n\
             error         : dx={} dy={} dw={} dh={}\n\
             Shrinking the form past zero and growing back must return \
             the component to the exact place. Was measured in the classic \
             RAD IDE with a TListBox on all four anchors: anchored width on \
             both sides is a function of parent size, crosses zero and returns \
             along the same path. A floor that RECORDS (the old w.max(1.0)) \
             destroys the rest of the shrinking, and growth adds on top of \
             the floor instead of the design size.",
            back.x - base.x,
            back.y - base.y,
            back.w - base.w,
            back.h - base.h
        );
    }
}

/// The round trip in STEPS, which is how the user drags the edge.
///
/// A single step could pass by chance if the calculation were made from the original
/// design; here there are six irregular steps crossing zero and returning,
/// and the document is rewritten at each one.
#[test]
fn round_trip_in_irregular_steps_also_returns_exact() {
    let base = Rect::new(10.0, 10.0, 380.0, 280.0);
    // Sums to zero in both axes, and midway stays far from negative.
    let path = [
        (-100.0, -80.0),
        (-250.0, -180.0),
        (-120.0, -90.0),
        (200.0, 150.0),
        (150.0, 120.0),
        (120.0, 80.0),
    ];
    for (axis, a) in combinations() {
        let mut d = with_anchors("b", base, a);
        for (dw, dh) in path {
            if let Some(tx) = apply_anchors(d.ir(), dw, dh) {
                d.apply(tx).unwrap();
            }
        }
        assert_eq!(
            rect(&d, "b"),
            base,
            "axis {axis}, {a:?}: six steps that sum to zero must return the \
             design rectangle, and not an accumulated value."
        );
    }
}

/// **Both anchor models produce the same rectangle, step by step.**
///
/// This equality is the reason the fix was cheap, and therefore it is
/// asserted and not assumed. The two models are:
///
/// - **summing without floor** — what `Anchors::adjust` does: receives delta and
///   lets the value cross zero;
/// - **recalculating from parent size** — what the classic component library does: stores the distance
///   to each anchored edge and the reference parent size, and recalculates
///   at each change.
///
/// If they diverged at some point, the cheap approach would not suffice and one would need
/// to store the rule in `Node` — which would touch `.crawform` and demand a
/// new `FORMAT_VERSION`. They do not diverge, and format remained untouched.
///
/// **Equality is algebraic, and this test is just a sample of it.** Both
/// models arrive at `w₀ + Σdw`; one adds term by term, the other all at once.
/// These six steps are integers and produce exact zero — but "zero" is a statement
/// about integers, not about `f32` in general: the reviewer scanned 128 000
/// steps with fractional deltas and found worst error of **0.0011 pt**,
/// floating point accumulation in pure translation branch, prior to this
/// fix. Grid and dragging produce integers, and there it is truly zero.
#[test]
fn both_anchor_models_produce_the_same_rectangle() {
    // The classic component library model: distance to each anchored edge, reference parent
    // size, and the calculation redone from scratch at each change.
    fn recalculate(r0: Rect, form0: Rect, form: Rect, a: Anchors) -> Rect {
        let slack_right = form0.w - (r0.x + r0.w);
        let slack_bottom = form0.h - (r0.y + r0.h);
        let (x, w) = match (a.left, a.right) {
            (true, true) => (r0.x, form.w - r0.x - slack_right),
            (false, true) => (form.w - slack_right - r0.w, r0.w),
            _ => (r0.x, r0.w),
        };
        let (y, h) = match (a.top, a.bottom) {
            (true, true) => (r0.y, form.h - r0.y - slack_bottom),
            (false, true) => (form.h - slack_bottom - r0.h, r0.h),
            _ => (r0.y, r0.h),
        };
        Rect::new(x, y, w, h)
    }

    let form0 = Rect::new(0.0, 0.0, 400.0, 300.0);
    let r0 = Rect::new(10.0, 10.0, 380.0, 280.0);
    let path = [
        (-100.0, -80.0),
        (-250.0, -180.0),
        (-120.0, -90.0),
        (200.0, 150.0),
        (150.0, 120.0),
        (120.0, 80.0),
    ];

    for (axis, a) in combinations() {
        let mut form = form0;
        let mut summed = r0;
        for (i, (dw, dh)) in path.into_iter().enumerate() {
            form = Rect::new(form.x, form.y, form.w + dw, form.h + dh);
            summed = a.adjust(summed, dw, dh);
            let recalculated = recalculate(r0, form0, form, a);
            assert_eq!(
                summed, recalculated,
                "\nTHE TWO ANCHOR MODELS DIVERGED - axis {axis}, {a:?}, \
                 step {i}\n\
                 form               : {} x {}\n\
                 summing delta      : {summed:?}\n\
                 recalculating (classic) : {recalculated:?}\n\
                 While both agree, summing delta is the classic component library written in \
                 another way and nothing needs to be saved in .crawform. If \
                 they diverge, this is the case forcing saving anchor rules \
                 in Node - and bumping FORMAT_VERSION.",
                form.w, form.h
            );
        }
    }
}

/// The case that CANNOT regress: without anchors, the component does not move.
///
/// Was fixed on 2026-09-08 and validated on screen by owner.
/// `default_anchor_leaves_component_still` covers `akLeft + akTop`; this
/// covers `Anchors = []`, and covers shrinking past zero — which is precisely
/// where a misplaced floor would begin moving what should stay still.
#[test]
fn without_any_anchor_component_does_not_move_even_crossing_zero() {
    let a = Anchors {
        left: false,
        top: false,
        right: false,
        bottom: false,
    };
    let base = Rect::new(150.0, 120.0, 80.0, 24.0);
    let mut d = with_anchors("b", base, a);
    for (dw, dh) in [(-500.0, -500.0), (-200.0, -200.0), (700.0, 700.0)] {
        if let Some(tx) = apply_anchors(d.ir(), dw, dh) {
            d.apply(tx).unwrap();
        }
        assert_eq!(
            rect(&d, "b"),
            base,
            "Anchors = [] stays still, and form changing size is not an \
             exception - not even when size crosses zero."
        );
    }
}

/// **The `.crawform` stores and returns collapsed rectangle, without new format.**
///
/// This is the test backing the choice of the cheap model. Letting width
/// cross zero is only free if the format tolerates negative numbers without
/// a new field — and it does: `rect` is already four `f32` and `serde_json` does not
/// care about sign. `FORMAT_VERSION` stays where it is.
///
/// The expensive alternative was storing anchor rule in `Node` (distances to
/// each edge + reference parent size), which would require a new field,
/// a new `FORMAT_VERSION` and migration. An earlier study measured that the format
/// is a flat map; losing that is costly, and was not needed.
///
/// ## What this test covers, and what it does not cover
///
/// It covers two things: that the collapsed rectangle survives the round trip
/// through the file, and that `FORMAT_VERSION` remains **2**, against a literal.
///
/// It is **not** a guard of the entire format: does not compare schema, does not list
/// fields, and cannot say what changed. If someone bumps the format for a
/// legitimate reason unrelated to anchors, the right thing is to update the pin — this test
/// is a pin, and a pin serves for someone to trip over on purpose.
///
/// The durable place of the pin is `crawrad-ir`, where the constant lives; stayed here
/// because this is where the assertion it protects is, and `crawrad-ir` is not
/// touched without warning.
#[test]
fn format_stores_collapsed_rectangle_and_returns_it_equal() {
    let collapsed = Rect::new(300.0, 250.0, -120.0, -90.0);
    let mut d = Document::new("form_main", "Form");
    let mut n = Node::new("vanished", "Button");
    n.set_rect(collapsed);
    d.add_child("form_main", n).unwrap();

    let json = d.to_json().expect("serialize document");
    let back = Document::from_json(&json).expect("reopen document");

    assert_eq!(
        back.node("vanished").unwrap().rect(),
        Some(collapsed),
        "collapsed rectangle must return from file identically: it is what \
         returns component to the right size when form grows"
    );
    // The pin is the LITERAL 2, not `crawrad_ir::FORMAT_VERSION`.
    //
    // The constant used to be here, and thus the assert **could not fail**:
    // `Document::new` records `format: FORMAT_VERSION`, so both sides of
    // comparison moved together. Measured during review — with
    // `FORMAT_VERSION = 3` the whole workspace passed, 964 green, and nothing in
    // repository noticed. It was a test promising coverage it did not
    // have, whose message promised the next reader a failure that would never
    // happen.
    assert_eq!(
        back.format, 2,
        "FORMAT_VERSION changed from 2, and this fix relies on it NOT \
         having changed: the anchor that crosses zero was chosen for fitting in \
         already existing format (the rectangle is already four f32, and sign \
         costs no field).\n\
         If the number rose BECAUSE of anchor, someone substituted the expensive model \
         — rule stored in Node — without saying.\n\
         If it rose for another reason, update this pin and confirm that round \
         trip of anchor remains without needing a new field."
    );
}

#[test]
fn anchors_round_trip_through_value() {
    let a = Anchors {
        left: false,
        top: true,
        right: true,
        bottom: true,
    };
    assert_eq!(Anchors::read(Some(&a.write())), a);
    // missing value falls back to classic IDE default
    assert_eq!(Anchors::read(None), Anchors::default());
}

/// A panel `alClient` with two aligned children inside it.
fn nested_doc() -> Doc {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    let mut pnl = Node::new("pnl", "Panel");
    pnl.set_rect(Rect::new(0.0, 0.0, 50.0, 50.0));
    pnl.props
        .insert("align".into(), Value::Enum("Client".into()));
    d.add_child("form_main", pnl).unwrap();
    for (id, h, al) in [("bar", 20.0, "Top"), ("middle", 10.0, "Client")] {
        let mut n = Node::new(id, "Panel");
        n.set_rect(Rect::new(0.0, 0.0, 10.0, h));
        n.props.insert("align".into(), Value::Enum(al.into()));
        d.add_child("pnl", n).unwrap();
    }
    Doc::new(d)
}

#[test]
fn nested_alignment_converges_in_a_single_turn() {
    // Grandchild must see grandfather ALREADY at the new size. The pass read document
    // while the parent's new rectangle only existed inside the transaction, so
    // child was aligned against OLD parent area and only got it right on the next
    // turn — measured in two turns, always.
    //
    // Two turns is not slowness: it is a spurious transaction per gesture in
    // undo, and a whole frame on screen with grandchild at wrong size.
    let mut d = nested_doc();
    let tx = apply(d.ir()).expect("there is something to align");
    d.apply(tx).unwrap();

    assert_eq!(rect(&d, "pnl"), Rect::new(0.0, 0.0, 400.0, 300.0));
    assert_eq!(
        rect(&d, "bar"),
        Rect::new(0.0, 0.0, 400.0, 20.0),
        "grandchild must see panel already with 400 width, not with the 50 it \
         had before this same pass"
    );
    assert_eq!(rect(&d, "middle"), Rect::new(0.0, 20.0, 400.0, 280.0));
    assert!(
        apply(d.ir()).is_none(),
        "one turn must suffice — the second is a spurious transaction in undo"
    );
}

#[test]
fn growing_form_readjusts_grandchild_in_the_same_turn() {
    // The contrast of test above in path user walks: user
    // drags form edge, and `readjust` calls `apply` ONCE
    // per transaction. Whatever does not converge then stays wrong on screen.
    let mut d = nested_doc();
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();

    d.apply(
        crawrad_doc::Transaction::user("resize form").with(Command::SetRect {
            node: "form_main".into(),
            rect: Rect::new(0.0, 0.0, 600.0, 500.0),
        }),
    )
    .unwrap();

    let tx = apply(d.ir()).expect("form changed size");
    d.apply(tx).unwrap();
    assert_eq!(rect(&d, "bar"), Rect::new(0.0, 0.0, 600.0, 20.0));
    assert_eq!(rect(&d, "middle"), Rect::new(0.0, 20.0, 600.0, 480.0));
    assert!(apply(d.ir()).is_none());
}

// --------------------------------------------------------------- margins
//
// `Margins` and `AlignWithMargins`, the classic component library pair. The rule in one sentence:
// margin is measured against CONTAINER AREA, and this area shrinks with each
// aligned child — so slack between two neighbors is SUM of margins that
// touch, and not the maximum of them.

/// Form 400x300 with children that carry margins.
fn doc_with_margins(comps: &[(&str, Rect, &str, Edges, bool)]) -> Doc {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    for (id, r, align, m, on) in comps {
        let mut n = Node::new(*id, "Panel");
        n.set_rect(*r);
        n.props.insert("align".into(), Value::Enum((*align).into()));
        n.props.insert("margins".into(), Value::Edges(*m));
        n.props
            .insert("align_with_margins".into(), Value::Bool(*on));
        d.add_child("form_main", n).unwrap();
    }
    Doc::new(d)
}

#[test]
fn two_bars_aligned_to_top_are_separated_by_sum_of_margins() {
    // The test that catches both half-implementations, and neither
    // appears looking at only one control — was measured before writing code:
    //
    // - margin in rectangle and accumulator forgotten: `a` comes out IDENTICAL to
    //   correct and `b` rises to y=33, sticking two bars with margins
    //   drawn on top of each other;
    // - margin only in axis of `Align`: gap comes out correct, 6, and both bars
    //   get x=0 and width 400, without any lateral indent.
    //
    // Therefore assertions check BOTH controls in BOTH axes. Loosening
    // either returns one of the two defects without anyone noticing.
    let m = Edges::uniform(3.0);
    let mut d = doc_with_margins(&[
        ("a", Rect::new(0.0, 0.0, 10.0, 30.0), "Top", m, true),
        ("b", Rect::new(0.0, 0.0, 10.0, 40.0), "Top", m, true),
    ]);
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();

    let (a, b) = (rect(&d, "a"), rect(&d, "b"));
    assert_eq!(a, Rect::new(3.0, 3.0, 394.0, 30.0), "the top bar");
    assert_eq!(b, Rect::new(3.0, 39.0, 394.0, 40.0), "the bottom bar");
    assert_eq!(
        b.y - (a.y + a.h),
        6.0,
        "gap is sum of touching margins: base of `a` plus top of `b`"
    );
}

#[test]
fn margin_of_ten_separates_by_twenty_as_owner_described() {
    // Owner's phrasing, in numbers: "Margins=10 ... 10 is space separating what
    // is aligned alTop from its container". Each preserves 10 of area
    // allotted to it, and area already comes shrunk by preceding one — hence 20 between
    // the two, and 10 to form edge.
    let m = Edges::uniform(10.0);
    let mut d = doc_with_margins(&[
        ("a", Rect::new(0.0, 0.0, 10.0, 30.0), "Top", m, true),
        ("b", Rect::new(0.0, 0.0, 10.0, 30.0), "Top", m, true),
    ]);
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();

    let (a, b) = (rect(&d, "a"), rect(&d, "b"));
    assert_eq!(a, Rect::new(10.0, 10.0, 380.0, 30.0));
    assert_eq!(b, Rect::new(10.0, 60.0, 380.0, 30.0));
    assert_eq!(
        b.y - (a.y + a.h),
        20.0,
        "10 from one plus 10 from the other"
    );
}

#[test]
fn asymmetric_margins_only_sum_those_that_touch() {
    // Four different sides on each. If calculation sums `top` with `top`, or
    // uses the larger of the two, the gap comes out different from 10 — and with equal margins
    // these errors are invisible, which is why this test exists in addition to one
    // above.
    let ma = Edges {
        left: 1.0,
        top: 2.0,
        right: 3.0,
        bottom: 4.0,
    };
    let mb = Edges {
        left: 5.0,
        top: 6.0,
        right: 7.0,
        bottom: 8.0,
    };
    let mut d = doc_with_margins(&[
        ("a", Rect::new(0.0, 0.0, 10.0, 30.0), "Top", ma, true),
        ("b", Rect::new(0.0, 0.0, 10.0, 40.0), "Top", mb, true),
    ]);
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();

    let (a, b) = (rect(&d, "a"), rect(&d, "b"));
    assert_eq!(a, Rect::new(1.0, 2.0, 396.0, 30.0));
    assert_eq!(b, Rect::new(5.0, 42.0, 388.0, 40.0));
    assert_eq!(
        b.y - (a.y + a.h),
        10.0,
        "base of `a` (4) plus top of `b` (6) — and not 2+6 nor max(4,6)"
    );
}

#[test]
fn four_sides_of_align_respect_own_margins() {
    // Entire cascade with margin: top, sides, and client. `alClient`
    // receives what remained of all ALREADY discounted of their margins, and further
    // discounts its own.
    let m = Edges::uniform(3.0);
    let mut d = doc_with_margins(&[
        ("top", Rect::new(0.0, 0.0, 10.0, 30.0), "Top", m, true),
        ("left", Rect::new(0.0, 0.0, 80.0, 10.0), "Left", m, true),
        ("right", Rect::new(0.0, 0.0, 60.0, 10.0), "Right", m, true),
        ("middle", Rect::new(0.0, 0.0, 10.0, 10.0), "Client", m, true),
    ]);
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();

    assert_eq!(rect(&d, "top"), Rect::new(3.0, 3.0, 394.0, 30.0));
    assert_eq!(rect(&d, "left"), Rect::new(3.0, 39.0, 80.0, 258.0));
    assert_eq!(rect(&d, "right"), Rect::new(337.0, 39.0, 60.0, 258.0));
    assert_eq!(rect(&d, "middle"), Rect::new(89.0, 39.0, 242.0, 258.0));
}

#[test]
fn false_align_with_margins_aligns_exactly_as_before() {
    // Migration test, authorizing change to enter without touching
    // any existing `.crawform`: with margins RECORDED and switch
    // off — which is the classic component library and catalog default —, result is
    // byte for byte that of `align_top_occupies_full_width_and_keeps_height`.
    let m = Edges::uniform(3.0);
    let mut d = doc_with_margins(&[
        ("a", Rect::new(37.0, 91.0, 50.0, 30.0), "Top", m, false),
        ("b", Rect::new(0.0, 0.0, 10.0, 40.0), "Top", m, false),
        (
            "middle",
            Rect::new(0.0, 0.0, 10.0, 10.0),
            "Client",
            m,
            false,
        ),
    ]);
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();

    assert_eq!(rect(&d, "a"), Rect::new(0.0, 0.0, 400.0, 30.0));
    assert_eq!(rect(&d, "b"), Rect::new(0.0, 30.0, 400.0, 40.0));
    assert_eq!(rect(&d, "middle"), Rect::new(0.0, 70.0, 400.0, 230.0));
}

#[test]
fn toggling_align_with_margins_returns_to_original_rectangle() {
    // Switch is reversible. Without this, "trying out" property would be
    // a one-way street: turning off would leave control where margin put it,
    // and person would have to reposition by hand what they only wanted to preview.
    let m = Edges::uniform(3.0);
    let mut d = doc_with_margins(&[("a", Rect::new(0.0, 0.0, 10.0, 30.0), "Top", m, false)]);
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();
    let before = rect(&d, "a");
    assert_eq!(before, Rect::new(0.0, 0.0, 400.0, 30.0));

    let toggle = |v: bool| {
        crawrad_doc::Transaction::user("AlignWithMargins").with(Command::SetProp {
            node: "a".into(),
            key: "align_with_margins".into(),
            value: Some(Value::Bool(v)),
        })
    };

    d.apply(toggle(true)).unwrap();
    let tx = apply(d.ir()).expect("turning on switch must move control");
    d.apply(tx).unwrap();
    assert_eq!(rect(&d, "a"), Rect::new(3.0, 3.0, 394.0, 30.0));

    d.apply(toggle(false)).unwrap();
    let tx = apply(d.ir()).expect("turning off must return");
    d.apply(tx).unwrap();
    assert_eq!(
        rect(&d, "a"),
        before,
        "turning off did not return to what it was"
    );
}

#[test]
fn margins_do_not_generate_transaction_when_already_converged() {
    // `apply` runs after EVERY transaction. A cascade of margins that does not
    // converge pushes a spurious entry into undo on each gesture, forever —
    // was defect that free `Align` on a page caused.
    let m = Edges::uniform(3.0);
    let mut d = doc_with_margins(&[
        ("a", Rect::new(0.0, 0.0, 10.0, 30.0), "Top", m, true),
        ("b", Rect::new(0.0, 0.0, 10.0, 20.0), "Bottom", m, true),
        ("middle", Rect::new(0.0, 0.0, 10.0, 10.0), "Client", m, true),
    ]);
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();
    assert!(
        apply(d.ir()).is_none(),
        "one turn suffices, and second must come empty"
    );
}

#[test]
fn margin_larger_than_area_does_not_produce_negative_rectangle() {
    // Negative width is not "small": `Rect::intersects` of marquee
    // selection reads it backwards. Before margins this required an absurd form;
    // with them, four numbers fitting in inspector suffice.
    //
    // **The effect is INVERSE of what this line said.** Here it was written
    // that component "starts being selected by any marquee anywhere on form".
    // Measured on 2026-09-09, scanning 6912 marquees against `(300, 250, -120 x -90)`:
    // NO marquee starts hitting because of inversion, and 5472 cease to hit.
    // Inverted condition is more restrictive — marquee would have to cover entire
    // inverted strip —, so component DISAPPEARS from marquee instead of appearing too much.
    // `max(0.0)` in `align_children` remains right; what was wrong was reason.
    //
    // What depends on this measurement today is `without_inversion` of `lib.rs`, which
    // does same cut in marquee because anchor started producing negative on purpose.
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 20.0, 20.0));
    for (id, al, h) in [
        ("top", "Top", 4.0),
        ("side", "Left", 4.0),
        ("middle", "Client", 4.0),
    ] {
        let mut n = Node::new(id, "Panel");
        n.set_rect(Rect::new(0.0, 0.0, 4.0, h));
        n.props.insert("align".into(), Value::Enum(al.into()));
        n.props
            .insert("margins".into(), Value::Edges(Edges::uniform(50.0)));
        n.props
            .insert("align_with_margins".into(), Value::Bool(true));
        d.add_child("form_main", n).unwrap();
    }
    let mut d = Doc::new(d);
    if let Some(tx) = apply(d.ir()) {
        d.apply(tx).unwrap();
    }
    for id in ["top", "side", "middle"] {
        let r = rect(&d, id);
        assert!(
            r.w >= 0.0 && r.h >= 0.0,
            "`{id}` came out with negative side: {r:?}"
        );
    }
}

/// A ToolBar lays out its controls left to right, centred on its
/// height; a button takes ButtonWidth x ButtonHeight, an Edit keeps its size.
#[test]
fn the_tool_bar_lays_out_its_controls() {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    let mut bar = Node::new("tbr", "ToolBar");
    bar.set_rect(Rect::new(0.0, 0.0, 400.0, 32.0));
    bar.props.insert("button_width".into(), Value::Int(30));
    bar.props.insert("button_height".into(), Value::Int(26));
    d.add_child("form_main", bar).unwrap();
    for (id, class, r) in [
        ("b1", "SpeedButton", Rect::new(200.0, 3.0, 50.0, 20.0)),
        ("b2", "Button", Rect::new(10.0, 1.0, 80.0, 24.0)),
        ("e1", "Edit", Rect::new(5.0, 5.0, 100.0, 22.0)),
    ] {
        let mut n = Node::new(id, class);
        n.set_rect(r);
        d.add_child("tbr", n).unwrap();
    }
    let mut d = Doc::new(d);
    let tx = apply(d.ir()).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(rect(&d, "b1"), Rect::new(0.0, 3.0, 30.0, 26.0));
    assert_eq!(rect(&d, "b2"), Rect::new(30.0, 3.0, 30.0, 26.0));
    assert_eq!(rect(&d, "e1"), Rect::new(60.0, 5.0, 100.0, 22.0));
    // already laid out: nothing more to do
    assert!(apply(d.ir()).is_none());
}
