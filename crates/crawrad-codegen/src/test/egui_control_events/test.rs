//! The control event router in the egui target (`control_events`): who
//! enters the list, what stays with the widget itself, and when it is not even generated.

use crate::Gerador;
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;

/// A form with `(class, events)` controls, named `c0`, `c1`…
fn form_with(controls: &[(&str, &[&str])]) -> Document {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    let root = d.node_mut("form_main").expect("the root");
    root.set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
    root.set_name("form_main");
    for (i, (class, events)) in controls.iter().enumerate() {
        let name = format!("c{i}");
        let (w, h) = r.get(class).expect("a class").default_size;
        let mut n = r
            .instantiate(
                class,
                name.clone(),
                Rect::new(8.0, 8.0 + 40.0 * i as f32, w, h),
            )
            .expect("instantiates");
        n.set_name(&name);
        for e in *events {
            n.events.insert((*e).into(), format!("{name}_{e}"));
        }
        d.add_child("form_main", n).expect("into the form");
    }
    d.validate().expect("valid");
    d
}

/// The egui designer file of `d`.
fn designer(d: &Document) -> String {
    crate::GeradorRxp
        .gerar(d, "probe", &Registry::builtin())
        .expect("generates")
        .into_iter()
        .find(|a| {
            a.caminho
                .to_string_lossy()
                .ends_with(crate::SUFIXO_GERADO.trim_start_matches('.'))
        })
        .expect("the designer file")
        .conteudo
}

#[test]
fn a_button_keeps_its_own_click_and_the_router_takes_the_press() {
    let src = designer(&form_with(&[("Button", &["on_click", "on_mouse_down"])]));
    assert!(src.contains("struct CrawradControlRouter"), "{src}");
    assert!(
        src.contains("crawrad_rects.push((0, r.intersect(ui.clip_rect())))"),
        "{src}"
    );
    assert!(src.contains("(0, CrawradPointer::Down(button))"), "{src}");
    assert!(src.contains("self.c0_on_mouse_down(MouseEvent {"), "{src}");
    assert!(
        !src.contains("CrawradPointer::Click)"),
        "the button's widget reports its click; routing it too fires it twice:\n{src}"
    );
}

#[test]
fn a_form_with_no_routed_event_carries_no_router() {
    let src = designer(&form_with(&[
        ("Button", &["on_click"]),
        ("Edit", &["on_change"]),
    ]));
    assert!(!src.contains("CrawradControlRouter"), "{src}");
    assert!(!src.contains("crawrad_rects"), "{src}");
}

#[test]
fn with_someone_listening_to_the_focus_every_control_that_can_take_it_is_listed() {
    let src = designer(&form_with(&[
        ("Button", &["on_enter"]),
        ("Edit", &[]),
        ("Label", &[]),
    ]));
    assert!(
        src.contains("self.crawrad_router.route(ui, &crawrad_rects, &[0, 1])"),
        "the edit takes the focus away from the button, handler or not:\n{src}"
    );
    assert!(src.contains("(0, CrawradFocus::Enter)"), "{src}");
    assert!(
        !src.contains("crawrad_rects.push((2,"),
        "a label cannot take the focus:\n{src}"
    );
}

/// An Edit keeps its own keys, but its focus pair is the router's:
/// egui reports the widget's `lost_focus` on two passes running when a Tab
/// moves the focus, and the router fires once per change.
#[test]
fn an_edit_keeps_its_own_keys_and_the_router_has_its_focus() {
    let src = designer(&form_with(&[(
        "Edit",
        &["on_enter", "on_exit", "on_key_down", "on_key_press"],
    )]));
    assert!(
        src.contains("(0, CrawradFocus::Enter)") && src.contains("(0, CrawradFocus::Exit)"),
        "the router has the focus pair:\n{src}"
    );
    assert!(
        !src.contains("(0, CrawradFocus::KeyDown") && !src.contains("(0, CrawradFocus::Char"),
        "the edit's emitter calls its keys from egui's own:\n{src}"
    );
}
