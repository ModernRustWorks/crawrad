//! **The form event comes out as a CALL in the Makepad application too** — the
//! twin of `test/form_events/test.rs`, and the same sweep.
//!
//! # Why the same sweep twice, and not one shared test
//!
//! Because the answer is allowed to differ and MUST be measured on each side.
//! Until 17/09/2026 the two targets disagreed about which events they fired —
//! egui did `on_close` and `on_timer`, Makepad did `on_scroll` and neither did
//! the eight with a parameter — and nobody knew, because no test asked the
//! question of both. A shared test asked once would have hidden exactly that.

use crate::targets::makepad::form_events::{FormEvents, form_events};
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;

/// A form with `event` wired to the handler `my_handler`, already through the
/// generator.
fn wired(event: &str) -> FormEvents {
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").expect("the freshly created root");
        root.set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
        root.events.insert(event.into(), "my_handler".into());
    }
    form_events(d.node("form_main").expect("the root")).expect("the form has a wired event")
}

/// Everything that form writes, in one string.
fn emitted(event: &str) -> String {
    let e = wired(event);
    format!("{}{}{}", e.items, e.on_create, e.handle)
}

/// The same single exception as the other target, and for the same reason: the
/// form IS the window, and it does not hide itself to come back.
const NOT_DISPATCHED: &[&str] = &["on_hide"];

#[test]
fn every_form_event_becomes_a_call_in_the_makepad_application() {
    let r = Registry::builtin();
    let form = r.get("Form").expect("the `Form` class of the catalogue");
    let mut missing = Vec::new();
    for ev in &form.events {
        if NOT_DISPATCHED.contains(&ev.name) {
            continue;
        }
        if !emitted(ev.name).contains(".my_handler(") {
            missing.push(ev.name);
        }
    }
    assert!(
        missing.is_empty(),
        "these form events do NOT become a call in the generated MAKEPAD \
         application: {missing:?}\n\n\
         The egui target may well dispatch them — that is precisely the shape \
         of the defect the two targets had before: each fired a different \
         handful, and the same form behaved differently depending on which \
         one generated it."
    );
}

#[test]
fn the_two_targets_dispatch_the_same_events() {
    let r = Registry::builtin();
    let form = r.get("Form").expect("the `Form` class of the catalogue");
    let mut only_one = Vec::new();
    for ev in &form.events {
        let makepad = emitted(ev.name).contains(".my_handler(");
        let egui = {
            let mut d = Document::new("form_main", "Form");
            {
                let root = d.node_mut("form_main").expect("the root");
                root.set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
                root.events.insert(ev.name.into(), "my_handler".into());
            }
            let e = crate::targets::egui::form_events::form_events(
                d.node("form_main").expect("the root"),
            )
            .expect("wired");
            format!("{}{}{}{}", e.items, e.new_body, e.before, e.after).contains(".my_handler(")
        };
        if makepad != egui {
            only_one.push((ev.name, if egui { "only egui" } else { "only makepad" }));
        }
    }
    assert!(
        only_one.is_empty(),
        "these events fire in ONE target and not in the other: {only_one:?}\n\n\
         The same form, generated twice, would behave in two ways — and the \
         user has no way of knowing which half they are on. Whatever one \
         target cannot do must be written down as not done on BOTH, not left \
         to be discovered."
    );
}

#[test]
fn the_payload_of_the_eight_is_built_with_the_catalogue_type() {
    for (event, tipo) in [
        ("on_key_down", "KeyEvent {"),
        ("on_key_up", "KeyEvent {"),
        ("on_mouse_down", "MouseEvent {"),
        ("on_mouse_up", "MouseEvent {"),
        ("on_mouse_move", "MouseMoveEvent {"),
        ("on_mouse_activate", "MouseActivateEvent {"),
    ] {
        assert!(
            emitted(event).contains(tipo),
            "`{event}` is called without BUILDING the `{tipo}` in the Makepad \
             target"
        );
    }
    assert!(
        emitted("on_key_press").contains("self.form.my_handler(ch);"),
        "`on_key_press` receives the `char` that was typed, and in Makepad \
         that is `Event::TextInput`"
    );
    assert!(
        emitted("on_close_query").contains("self.form.my_handler(&mut can_close);"),
        "`on_close_query` lost the veto in the Makepad target"
    );
}

#[test]
fn the_close_query_veto_reaches_makepad() {
    assert!(
        emitted("on_close_query").contains("e.accept_close.set(false);"),
        "`OnCloseQuery` asks and ignores the answer: without setting \
         `accept_close` the window closes anyway"
    );
}

#[test]
fn the_key_table_only_names_keys_the_contract_has() {
    let known: Vec<&str> = crate::events::key_variants();
    let out = emitted("on_key_down");
    let unknown: Vec<&str> = out
        .lines()
        .filter_map(|l| l.trim().strip_prefix("KeyCode::"))
        .filter_map(|l| l.split("Key::").nth(1))
        .map(|t| t.trim_end_matches(','))
        .filter(|t| !known.contains(t))
        .collect();
    assert!(
        unknown.is_empty(),
        "the Makepad key table names {unknown:?}, which the `Key` of the \
         contract does not have. That does not fail here — it fails in the \
         USER's project, with a name they never typed"
    );
}

#[test]
fn the_click_uses_makepads_own_thresholds() {
    let out = emitted("on_dbl_click");
    assert!(
        out.contains("TAP_COUNT_TIME") && out.contains("TAP_COUNT_DISTANCE"),
        "the double click grew numbers of its own. Makepad publishes the two \
         it uses, and a click on the form has to mean what a click on a widget \
         means — otherwise the same gesture counts as two different things on \
         the same screen"
    );
}

#[test]
fn a_form_with_no_event_generates_not_one_line() {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .expect("the root")
        .set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
    let e = form_events(d.node("form_main").expect("the root")).expect("no event wired");
    assert!(
        e.empty() && e.items.is_empty() && e.field.is_empty(),
        "a form with no wired event got event code in the Makepad target"
    );
}

#[test]
fn on_create_targets_self_form_for_initial_sync() {
    let e = wired("on_create");
    assert_eq!(e.on_create, "            self.form.my_handler();\n");
}
