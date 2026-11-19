use super::*;
use crawrad_ir::Rect;

/// The dialog tests share ONE global `dialog_manager()`, and cargo runs them
/// in parallel: one test's `clear` or `resolve_dialog` landed in the middle
/// of another's (seen on the Linux CI, `test_dialog_show_confirm_callback`).
/// Each takes this lock first, so they run one at a time.
static DIALOGS: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// The lock, even after a test that held it panicked.
pub(crate) fn dialogs_alone() -> std::sync::MutexGuard<'static, ()> {
    DIALOGS.lock().unwrap_or_else(|e| e.into_inner())
}

#[test]
fn hierarchy_and_deref_work_in_a_chain() {
    let comp = ComponentBase::new("btn1", "Button1", "Button");
    let visual = VisualControlBase {
        base: comp,
        rect: Rect::new(10.0, 20.0, 80.0, 30.0),
        visible: true,
        enabled: true,
        text: "Save".into(),
        visual: true,
        derived_geometry: false,
    };
    let window = WindowControlBase {
        visual_base: visual,
        parent: "form1".into(),
        children: Vec::new(),
        clip: None,
        tab_order: 1,
    };

    // Direct access via Deref
    assert_eq!(window.id, "btn1");
    assert_eq!(window.name, "Button1");
    assert_eq!(window.class, "Button");
    assert_eq!(window.rect.w, 80.0);
    assert_eq!(window.text, "Save");
    assert_eq!(window.parent, "form1");
    assert_eq!(window.tab_order, 1);
}

#[test]
fn ladder_goes_up_and_down_without_skipping_steps() {
    // Climbing the entire ladder and descending entirely must return the bottom
    // end, passing through all steps in order. This is what catches a
    // `find` with the comparison flipped — which would always return the same
    // step and make the command appear dead.
    let mut z = ZOOMS[0];
    let mut ascending = vec![z];
    for _ in 0..ZOOMS.len() {
        z = adjacent_zoom(z, true);
        ascending.push(z);
    }
    assert_eq!(
        ascending,
        [
            ZOOMS[0], ZOOMS[1], ZOOMS[2], ZOOMS[3], ZOOMS[4], ZOOMS[5], ZOOMS[6], ZOOMS[7],
            // The eighth call is already at the extreme and stays on it.
            ZOOMS[7],
        ]
    );

    let mut z = ZOOMS[ZOOMS.len() - 1];
    for _ in 0..ZOOMS.len() {
        z = adjacent_zoom(z, false);
    }
    assert_eq!(
        z, ZOOMS[0],
        "descending all the way, stops at the bottom end"
    );
}

#[test]
fn ladder_does_not_wrap_around() {
    // At the extremes, the command remains inert. Wrapping around would save a click and
    // cost the thing most expected from zoom: that it does not jump from
    // 400% to 25% because a finger lingered on the button.
    assert_eq!(adjacent_zoom(ZOOMS[ZOOMS.len() - 1], true), ZOOMS[7]);
    assert_eq!(adjacent_zoom(ZOOMS[0], false), ZOOMS[0]);
}

#[test]
fn value_outside_ladder_moves_in_requested_direction() {
    // An agent via MCP, or an old session, might arrive with `0.9`.
    // Asking to *zoom in* must not zoom out — which is what a "closest
    // step" would do here, because 0.75 is closer to 0.9 than 1.0.
    assert_eq!(adjacent_zoom(0.9, true), 1.0);
    assert_eq!(adjacent_zoom(0.9, false), 0.75);
}

#[test]
fn one_hundred_percent_is_a_step() {
    // The only zoom where drawing measures what the application will measure must
    // be reachable by walking the ladder, not only via the reset
    // command — otherwise the ladder steps over it.
    assert!(ZOOMS.contains(&ZOOM_ONE));
}

#[test]
fn test_dialog_show_message_post_and_resolve() {
    let _lock = dialog_manager();
    let _alone = dialogs_alone();
    dialog_manager().lock().unwrap().clear();

    assert!(!is_dialog_open());
    show_message("Successfully calculated!");

    assert!(is_dialog_open());
    let snap = active_dialog().expect("there must be an active dialog");
    assert_eq!(snap.message, "Successfully calculated!");
    assert_eq!(snap.title, "Information");
    assert_eq!(snap.kind, DialogKind::Information);
    assert_eq!(snap.buttons, DialogButtons::Ok);

    let res = resolve_dialog(ModalResult::Ok);
    assert_eq!(res, Some(ModalResult::Ok));
    assert!(!is_dialog_open());
}

#[test]
fn test_dialog_show_warning_and_error() {
    let _alone = dialogs_alone();
    dialog_manager().lock().unwrap().clear();

    show_warning("Check the fields");
    let snap = active_dialog().unwrap();
    assert_eq!(snap.kind, DialogKind::Warning);
    assert_eq!(snap.title, "Warning");
    assert_eq!(snap.kind.icon_symbol(), "⚠");
    resolve_dialog(ModalResult::Ok);

    show_error("Failed to save");
    let snap2 = active_dialog().unwrap();
    assert_eq!(snap2.kind, DialogKind::Error);
    assert_eq!(snap2.title, "Error");
    assert_eq!(snap2.kind.icon_symbol(), "✖");
    resolve_dialog(ModalResult::Ok);
}

#[test]
fn test_dialog_show_confirm_callback() {
    let _alone = dialogs_alone();
    dialog_manager().lock().unwrap().clear();

    use std::sync::atomic::{AtomicBool, Ordering};
    static CALLED: AtomicBool = AtomicBool::new(false);

    show_confirm("Are you sure you want to delete?", |res| {
        if res == ModalResult::Yes {
            CALLED.store(true, Ordering::SeqCst);
        }
    });

    let snap = active_dialog().unwrap();
    assert_eq!(snap.kind, DialogKind::Confirmation);
    assert_eq!(snap.buttons, DialogButtons::YesNo);

    resolve_dialog(ModalResult::Yes);
    assert!(CALLED.load(Ordering::SeqCst));
    assert!(!is_dialog_open());
}

#[test]
fn test_dialog_crawrad_alias() {
    let _alone = dialogs_alone();
    dialog_manager().lock().unwrap().clear();

    crate::crawrad::show_message("Message via alias crawrad::");
    let snap = active_dialog().unwrap();
    assert_eq!(snap.message, "Message via alias crawrad::");
    resolve_dialog(ModalResult::Ok);
}

/// #500: a link opens only the web and mail; a path or a script is refused.
#[test]
fn a_link_opens_only_web_and_mail() {
    assert!(crate::shell::openable("https://example.com/?a=1&b=2"));
    assert!(crate::shell::openable(" HTTP://example.com "));
    assert!(crate::shell::openable("mailto:someone@example.com"));
    for refused in [
        "",
        "C:/Windows/System32/calc.exe",
        "file:///C:/x.exe",
        "javascript:alert(1)",
        "example.com",
        "https://example.com/\nnext",
    ] {
        assert!(!crate::shell::openable(refused), "{refused:?}");
        assert!(!crate::shell::open_url(refused), "{refused:?}");
    }
}

#[test]
fn open_and_say_reports_refused_sentence() {
    assert!(crate::shell::openable("https://example.com/search?a=1&b=2"));
    assert_eq!(
        crate::link::open_and_say("javascript:alert(1)"),
        "Not opened: javascript:alert(1) — only http, https and mailto links open"
    );
    assert_eq!(
        crate::link::open_and_say("C:/Windows/System32/calc.exe"),
        "Not opened: C:/Windows/System32/calc.exe — only http, https and mailto links open"
    );
}

#[test]
fn cyclic_document_does_not_hang_in_hidden_page() {
    let mut doc = Document::new("root", "Form");
    let mut n1 = crawrad_ir::Node::new("n1", "Panel");
    let mut n2 = crawrad_ir::Node::new("n2", "Panel");
    n1.children.push("n2".into());
    n2.children.push("n1".into());
    doc.nodes.insert("n1".into(), n1);
    doc.nodes.insert("n2".into(), n2);

    assert!(!in_hidden_page(&doc, "n1"));
}

#[test]
fn spin_edit_hooks_defaults() {
    struct PlainExt;
    impl SpinEditHooks for PlainExt {}

    let mut ext = PlainExt;
    assert_eq!(ext.display(5), "5");
    assert_eq!(ext.parse(" 7 "), Some(7));
    assert_eq!(ext.parse("x"), None);
    ext.changed(5);
}
