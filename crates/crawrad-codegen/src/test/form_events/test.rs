//! **A form event comes out as a CALL in the application**, and not only as a
//! signature in the user's file.
//!
//! # The defect these tests close
//!
//! Measured on 17/09/2026: of the 31 events some class in the
//! catalogue offers, the generated egui application fired **five**. The rest
//! went as far as the next-to-last step — the IDE wired it, the `.crawform`
//! stored it, the generator wrote the handler with the right signature — and
//! the application never called the function. The user wrote the body, ran it,
//! pressed the key, and nothing happened.
//!
//! Nothing along that path ever went red, and that is what makes the family
//! expensive: **everything you can see works.**
//!
//! # Why this sweeps the catalogue instead of listing the events here
//!
//! A list written here covers today's 21 and none of the ones to come. The
//! sweep asks the `Registry` which events the `Form` class declares and
//! charges each of them, so an event added to the catalogue tomorrow is born
//! already charged.

use crate::targets::egui::form_events::{FormEvents, form_events};
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
    format!("{}{}{}{}", e.items, e.new_body, e.before, e.after)
}

/// **The only one that is not dispatched, and it is written down here.**
///
/// `OnHide` cannot happen: the form IS the window of a generated application,
/// and it does not hide itself to come back. This constant exists so that the
/// exception stays ONE — the day another event stops coming out, the sweep
/// below fails instead of someone quietly adding a name here.
const NOT_DISPATCHED: &[&str] = &["on_hide"];

#[test]
fn every_form_event_becomes_a_call_in_the_application() {
    let r = Registry::builtin();
    let form = r.get("Form").expect("the `Form` class of the catalogue");
    let mut missing = Vec::new();
    for ev in &form.events {
        if NOT_DISPATCHED.contains(&ev.name) {
            continue;
        }
        // `.my_handler(` and not `self.my_handler(`: `OnCreate` runs inside
        // `new()`, where the form is still a local called `form` and there is
        // no `self` yet.
        if !emitted(ev.name).contains(".my_handler(") {
            missing.push(ev.name);
        }
    }
    assert!(
        missing.is_empty(),
        "these form events do NOT become a call in the generated application: \
         {missing:?}\n\n\
         The user wires the event in the Object Inspector, the generator \
         writes the handler with the right signature, they write the body — \
         and the application never calls the function. That is issue #141, \
         and it is back."
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
        "a form with no wired event got event code. The designer is rewritten \
         whole and the user cannot fix it: what nobody asked for does not go in"
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
            "`{event}` is called without BUILDING the `{tipo}`: either it \
             passes a value of some other type, or the call is not there"
        );
    }
    assert!(
        emitted("on_key_press").contains("self.my_handler(ch);"),
        "`on_key_press` receives the `char` that was typed, which is what the \
         catalogue signature declares"
    );
    assert!(
        emitted("on_close_query").contains("self.my_handler(&mut can_close);"),
        "`on_close_query` lost the veto: it takes the `&mut bool` precisely so \
         it can refuse the close, the way the classic RAD IDE does"
    );
}

#[test]
fn the_close_query_veto_reaches_the_framework() {
    assert!(
        emitted("on_close_query").contains("egui::ViewportCommand::CancelClose"),
        "`OnCloseQuery` asks and ignores the answer: without the `CancelClose` \
         the window closes anyway, and the user's `can_close = false` is worth \
         nothing"
    );
}

#[test]
fn the_key_map_covers_the_whole_vocabulary() {
    let out = emitted("on_key_down");
    let names = crate::events::key_variants();
    assert!(
        names.len() > 100,
        "the key vocabulary shrank to {} names — or `key_variants` stopped \
         reading the table",
        names.len()
    );
    let missing: Vec<&str> = names
        .iter()
        .copied()
        .filter(|v| *v != "Unknown")
        .filter(|v| !out.contains(&format!("Key::{v},")))
        .collect();
    assert!(
        missing.is_empty(),
        "the egui key map does not cover {missing:?}. A key left out comes \
         through as something else, or does not compile — and the map is \
         generated from the SAME list exactly so that this cannot happen"
    );
}

#[test]
fn the_frame_to_frame_state_is_born_only_for_who_needs_it() {
    for event in ["on_show", "on_resize", "on_activate", "on_mouse_enter"] {
        assert!(
            !wired(event).field.is_empty(),
            "`{event}` is an EDGE — it fires on the change, not on the \
             condition — and it was left with no previous frame to compare to"
        );
    }
    assert!(
        wired("on_click").field.is_empty(),
        "a form that only listens to `OnClick` got the frame-to-frame state. \
         That is one more field in the user's struct serving nobody"
    );
}
