use super::*;
use crate::layout::{Anchoring, Anchors};
use crawrad_ir::{Document, Node, Rect, Value};
use crawrad_registry::Registry;

fn form(w: f32, h: f32) -> Document {
    let mut doc = Document::new("form1", "Form");
    doc.nodes
        .get_mut("form1")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, w, h));
    doc
}

fn add<'a>(doc: &'a mut Document, parent: &str, id: &str, class: &str, r: Rect) -> &'a mut Node {
    let mut n = Node::new(id, class);
    n.set_rect(r);
    doc.nodes.get_mut(parent).unwrap().children.push(id.into());
    doc.nodes.insert(id.into(), n);
    doc.nodes.get_mut(id).unwrap()
}

#[test]
fn frame_at_design_size_changes_nothing() {
    let mut doc = form(640.0, 400.0);
    add(
        &mut doc,
        "form1",
        "btn",
        "Button",
        Rect::new(20.0, 30.0, 100.0, 40.0),
    );
    let anchored_both = add(
        &mut doc,
        "form1",
        "both",
        "Button",
        Rect::new(50.0, 50.0, 120.0, 30.0),
    );
    anchored_both.props.insert(
        "anchors".into(),
        Anchors {
            left: true,
            top: true,
            right: true,
            bottom: false,
        }
        .write(),
    );

    let framed = frame(&doc, 640.0, 400.0);
    assert_eq!(
        framed.node("form1").unwrap().rect().unwrap(),
        Rect::new(0.0, 0.0, 640.0, 400.0)
    );
    assert_eq!(
        framed.node("btn").unwrap().rect().unwrap(),
        Rect::new(20.0, 30.0, 100.0, 40.0)
    );
    assert_eq!(
        framed.node("both").unwrap().rect().unwrap(),
        Rect::new(50.0, 50.0, 120.0, 30.0)
    );
}

#[test]
fn frame_wider_resizes_anchored_and_keeps_left_anchored_width() {
    let mut doc = form(640.0, 400.0);
    let anchored_both = add(
        &mut doc,
        "form1",
        "both",
        "Button",
        Rect::new(20.0, 20.0, 100.0, 30.0),
    );
    anchored_both.props.insert(
        "anchors".into(),
        Anchors {
            left: true,
            top: true,
            right: true,
            bottom: false,
        }
        .write(),
    );

    let anchored_left = add(
        &mut doc,
        "form1",
        "left_only",
        "Button",
        Rect::new(150.0, 20.0, 100.0, 30.0),
    );
    anchored_left.props.insert(
        "anchors".into(),
        Anchors {
            left: true,
            top: true,
            right: false,
            bottom: false,
        }
        .write(),
    );

    let client = add(
        &mut doc,
        "form1",
        "fill",
        "Panel",
        Rect::new(0.0, 0.0, 10.0, 10.0),
    );
    client
        .props
        .insert("align".into(), Value::Enum("Client".into()));

    let framed = frame(&doc, 800.0, 400.0);

    // Difference dw = 160.0. "both" expands by 160 px.
    let r_both = framed.node("both").unwrap().rect().unwrap();
    assert_eq!(r_both.x, 20.0);
    assert_eq!(r_both.w, 260.0);

    // "left_only" keeps its original width.
    let r_left = framed.node("left_only").unwrap().rect().unwrap();
    assert_eq!(r_left.x, 150.0);
    assert_eq!(r_left.w, 100.0);

    // "fill" fills the framed root area (800x400).
    let r_client = framed.node("fill").unwrap().rect().unwrap();
    assert_eq!(r_client, Rect::new(0.0, 0.0, 800.0, 400.0));
}

#[test]
fn overflows_detection_for_right_bottom_hidden_and_non_visual() {
    let mut doc = form(390.0, 844.0);
    // 470 px wide control at x 24 -> runs past right edge by 104 px (24 + 470 - 390 = 104).
    add(
        &mut doc,
        "form1",
        "grid1",
        "Button",
        Rect::new(24.0, 10.0, 470.0, 32.0),
    );

    // Control running past the bottom edge by 56 px (800 + 100 - 844 = 56).
    add(
        &mut doc,
        "form1",
        "btn_bottom",
        "Button",
        Rect::new(10.0, 800.0, 100.0, 100.0),
    );

    // Hidden control past right edge -> must be skipped.
    let hidden = add(
        &mut doc,
        "form1",
        "hidden_ctl",
        "Button",
        Rect::new(24.0, 50.0, 470.0, 32.0),
    );
    hidden.props.insert("visible".into(), Value::Bool(false));

    // Non-visual component (Timer) past bottom-right -> must be skipped.
    add(
        &mut doc,
        "form1",
        "timer1",
        "Timer",
        Rect::new(400.0, 900.0, 28.0, 28.0),
    );

    let reg = Registry::builtin();
    let ovs = overflows(&doc, &reg);

    assert_eq!(ovs.len(), 2);
    assert_eq!(
        ovs[0],
        Overflow {
            node: "grid1".to_string(),
            edge: Edge::Right,
            px: 104,
        }
    );
    assert_eq!(
        ovs[1],
        Overflow {
            node: "btn_bottom".to_string(),
            edge: Edge::Bottom,
            px: 56,
        }
    );
}

#[test]
fn frame_note_messages_and_warning_flag() {
    let reg = Registry::builtin();

    // Clean form: fits, no warning.
    let doc_clean = form(390.0, 844.0);
    assert_eq!(frame_note(&doc_clean, &reg), ("Fits".to_string(), false));

    // One overflow: single line, warning.
    let mut doc_one = form(390.0, 844.0);
    add(
        &mut doc_one,
        "form1",
        "grid1",
        "Button",
        Rect::new(24.0, 10.0, 470.0, 32.0),
    );
    assert_eq!(
        frame_note(&doc_one, &reg),
        (
            "grid1 runs 104 px past the right edge — anchor it right (akRight) or set Align"
                .to_string(),
            true
        )
    );

    // Two overflows: first line + "(and 1 more)", warning.
    let mut doc_two = doc_one.clone();
    add(
        &mut doc_two,
        "form1",
        "btn_bottom",
        "Button",
        Rect::new(10.0, 800.0, 100.0, 100.0),
    );
    assert_eq!(
        frame_note(&doc_two, &reg),
        (
            "grid1 runs 104 px past the right edge — anchor it right (akRight) or set Align (and 1 more)"
                .to_string(),
            true
        )
    );
}

#[test]
fn presets_order_per_platform_and_shown_flags() {
    // Desktop: Desktop 1280x800, Tablet 820x1180, Phone 390x844, Laptop 1366x768, Full HD 1920x1080.
    let desktop = presets(SizePlatform::Desktop);
    assert_eq!(
        desktop,
        vec![
            SizePreset {
                name: "Desktop".to_string(),
                width: 1280,
                height: 800,
            },
            SizePreset {
                name: "Tablet".to_string(),
                width: 820,
                height: 1180,
            },
            SizePreset {
                name: "Phone".to_string(),
                width: 390,
                height: 844,
            },
            SizePreset {
                name: "Laptop".to_string(),
                width: 1366,
                height: 768,
            },
            SizePreset {
                name: "Full HD".to_string(),
                width: 1920,
                height: 1080,
            },
        ]
    );
    assert_eq!(
        presets_shown(SizePlatform::Desktop),
        vec![true, true, true, false, false]
    );

    // Mobile: Phone 390x844, Tablet 820x1180, Desktop 1280x800, Large phone 430x932, Tablet landscape 1180x820.
    let mobile = presets(SizePlatform::Mobile);
    assert_eq!(
        mobile,
        vec![
            SizePreset {
                name: "Phone".to_string(),
                width: 390,
                height: 844,
            },
            SizePreset {
                name: "Tablet".to_string(),
                width: 820,
                height: 1180,
            },
            SizePreset {
                name: "Desktop".to_string(),
                width: 1280,
                height: 800,
            },
            SizePreset {
                name: "Large phone".to_string(),
                width: 430,
                height: 932,
            },
            SizePreset {
                name: "Tablet landscape".to_string(),
                width: 1180,
                height: 820,
            },
        ]
    );
    assert_eq!(
        presets_shown(SizePlatform::Mobile),
        vec![true, true, false, false, false]
    );

    // Web: "1440" 1440x900, "1024" 1024x768, "390" 390x844, "768" 768x1024.
    let web = presets(SizePlatform::Web);
    assert_eq!(
        web,
        vec![
            SizePreset {
                name: "1440".to_string(),
                width: 1440,
                height: 900,
            },
            SizePreset {
                name: "1024".to_string(),
                width: 1024,
                height: 768,
            },
            SizePreset {
                name: "390".to_string(),
                width: 390,
                height: 844,
            },
            SizePreset {
                name: "768".to_string(),
                width: 768,
                height: 1024,
            },
        ]
    );
    assert_eq!(
        presets_shown(SizePlatform::Web),
        vec![true, true, true, false]
    );
}

#[test]
fn add_custom_validation_limits_and_duplicate_names() {
    let mut sp = SizePreview::new(SizePlatform::Desktop);

    // Valid addition: adding a custom size shows it.
    assert!(sp.add_custom("My Device", 1024, 600).is_ok());
    assert_eq!(sp.custom.len(), 1);
    assert_eq!(sp.custom[0].0.name, "My Device");
    assert!(sp.custom[0].1);

    // Empty or whitespace name.
    assert!(sp.add_custom("", 800, 600).is_err());
    assert!(sp.add_custom("   ", 800, 600).is_err());

    // Duplicate custom name.
    assert!(sp.add_custom("My Device", 1200, 700).is_err());

    // Duplicate preset name.
    assert!(sp.add_custom("Desktop", 1200, 700).is_err());
    assert!(sp.add_custom("Phone", 1200, 700).is_err());
    assert!(sp.add_custom("Laptop", 1200, 700).is_err());

    // Width limits: 120..=7680.
    assert!(sp.add_custom("Narrow", 119, 600).is_err());
    assert!(sp.add_custom("Wide", 7681, 600).is_err());

    // Height limits: 120..=7680.
    assert!(sp.add_custom("Short", 600, 119).is_err());
    assert!(sp.add_custom("Tall", 600, 7681).is_err());

    // Boundaries are accepted.
    assert!(sp.add_custom("Min Size", 120, 120).is_ok());
    assert!(sp.add_custom("Max Size", 7680, 7680).is_ok());

    // shown() includes shown presets, followed by custom ones.
    let shown_names: Vec<&str> = sp.shown().into_iter().map(|p| p.name.as_str()).collect();
    assert_eq!(
        shown_names,
        vec![
            "Desktop",
            "Tablet",
            "Phone",
            "My Device",
            "Min Size",
            "Max Size"
        ]
    );

    // Toggling a custom size hides it from shown().
    // Combined index 5 corresponds to "My Device" (after 5 desktop presets 0..=4).
    sp.toggle(5);
    let shown_after_toggle_custom: Vec<&str> =
        sp.shown().into_iter().map(|p| p.name.as_str()).collect();
    assert_eq!(
        shown_after_toggle_custom,
        vec!["Desktop", "Tablet", "Phone", "Min Size", "Max Size"]
    );

    // Toggling preset off removes it from shown.
    sp.toggle(0); // turns Desktop off
    let shown_after_toggle: Vec<&str> = sp.shown().into_iter().map(|p| p.name.as_str()).collect();
    assert_eq!(
        shown_after_toggle,
        vec!["Tablet", "Phone", "Min Size", "Max Size"]
    );

    // Toggling custom size again shows it.
    sp.toggle(5);
    let shown_after_custom_on: Vec<&str> =
        sp.shown().into_iter().map(|p| p.name.as_str()).collect();
    assert_eq!(
        shown_after_custom_on,
        vec!["Tablet", "Phone", "My Device", "Min Size", "Max Size"]
    );

    // remove_custom removes custom size.
    sp.remove_custom(0); // removes My Device
    let shown_after_remove: Vec<&str> = sp.shown().into_iter().map(|p| p.name.as_str()).collect();
    assert_eq!(
        shown_after_remove,
        vec!["Tablet", "Phone", "Min Size", "Max Size"]
    );
}

#[test]
fn a_fraction_of_a_pixel_past_the_edge_counts_as_one_px_and_flush_is_no_overflow() {
    let mut doc = form(390.0, 844.0);
    add(
        &mut doc,
        "form1",
        "edt1",
        "Button",
        Rect::new(24.0, 10.0, 366.5, 32.0),
    );
    add(
        &mut doc,
        "form1",
        "flush",
        "Button",
        Rect::new(24.0, 60.0, 366.0, 32.0),
    );
    let ovs = overflows(&doc, &Registry::builtin());
    assert_eq!(
        ovs,
        vec![Overflow {
            node: "edt1".to_string(),
            edge: Edge::Right,
            px: 1,
        }]
    );
}

#[test]
fn a_control_whose_class_is_not_installed_is_no_overflow() {
    let mut doc = form(390.0, 844.0);
    add(
        &mut doc,
        "form1",
        "conn1",
        "Connection",
        Rect::new(500.0, 10.0, 100.0, 32.0),
    );
    assert!(overflows(&doc, &Registry::builtin()).is_empty());
}
