use super::*;
use crawrad_doc::Doc;
use crawrad_ir::Value;

fn doc() -> Doc {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    let mut b = Node::new("btn_ok", "Button");
    b.set_rect(Rect::new(24.0, 96.0, 96.0, 32.0));
    b.set_text("&OK");
    b.events.insert("on_click".into(), "btn_okClick".into());
    d.add_child("form_main", b).unwrap();
    let mut e = Node::new("edt_nome", "Edit");
    e.set_rect(Rect::new(24.0, 48.0, 280.0, 24.0));
    d.add_child("form_main", e).unwrap();
    Doc::new(d)
}

/// Name generator in IDE style, but deterministic for the test.
fn namer(used: std::cell::RefCell<Vec<String>>) -> impl Fn(&str) -> String {
    move |class| {
        let mut u = used.borrow_mut();
        let id = format!("{}_{}", class.to_lowercase(), u.len() + 1);
        u.push(id.clone());
        id
    }
}

#[test]
fn paste_creates_new_component_with_properties_of_original() {
    let mut d = doc();
    let mut area = Clipboard::default();
    assert_eq!(area.copy(d.ir(), &["btn_ok".into()]), 1);

    let (tx, created) = area.paste(d.ir(), namer(Default::default())).unwrap();
    d.apply(tx).unwrap();

    let new_node = d.ir().node(&created[0]).unwrap();
    assert_eq!(new_node.class, "Button");
    assert_eq!(new_node.text(), Some("&OK"), "properties come along");
    assert_ne!(created[0], "btn_ok", "but name is different");
    assert!(d.ir().validate().is_ok());
}

#[test]
fn copy_is_born_offset_so_as_not_to_stay_hidden() {
    let mut d = doc();
    let mut area = Clipboard::default();
    area.copy(d.ir(), &["btn_ok".into()]);
    let (tx, created) = area.paste(d.ir(), namer(Default::default())).unwrap();
    d.apply(tx).unwrap();

    let r = d.ir().node(&created[0]).unwrap().rect().unwrap();
    assert_eq!((r.x, r.y), (32.0, 104.0));
    assert_eq!((r.w, r.h), (96.0, 32.0), "size is the same");
}

#[test]
fn event_handler_does_not_come_along() {
    // Two things calling the same function without user asking is a trap.
    let mut d = doc();
    let mut area = Clipboard::default();
    area.copy(d.ir(), &["btn_ok".into()]);
    let (tx, created) = area.paste(d.ir(), namer(Default::default())).unwrap();
    d.apply(tx).unwrap();

    assert!(d.ir().node(&created[0]).unwrap().events.is_empty());
    assert_eq!(
        d.ir().node("btn_ok").unwrap().events.len(),
        1,
        "and original keeps its own"
    );
}

#[test]
fn pasting_several_at_once_is_a_single_ctrl_z() {
    let mut d = doc();
    let mut area = Clipboard::default();
    area.copy(d.ir(), &["btn_ok".into(), "edt_nome".into()]);
    let (tx, created) = area.paste(d.ir(), namer(Default::default())).unwrap();
    d.apply(tx).unwrap();

    assert_eq!(created.len(), 2);
    assert_eq!(d.ir().nodes.len(), 5);
    d.undo().unwrap();
    assert_eq!(d.ir().nodes.len(), 3, "one undo removes both");
    assert!(d.ir().validate().is_ok());
}

#[test]
fn pasting_twice_generates_different_names() {
    let mut d = doc();
    let mut area = Clipboard::default();
    area.copy(d.ir(), &["btn_ok".into()]);

    let used: std::cell::RefCell<Vec<String>> = Default::default();
    for _ in 0..2 {
        let (tx, _) = area
            .paste(d.ir(), |c| {
                let mut u = used.borrow_mut();
                let id = format!("{}_{}", c.to_lowercase(), u.len() + 1);
                u.push(id.clone());
                id
            })
            .unwrap();
        d.apply(tx).unwrap();
    }
    assert_eq!(d.ir().nodes.len(), 5);
    assert!(d.ir().validate().is_ok());
}

#[test]
fn what_is_copied_survives_deletion_of_original() {
    let mut d = doc();
    let mut area = Clipboard::default();
    area.copy(d.ir(), &["btn_ok".into()]);

    d.apply(
        crawrad_doc::Transaction::user("remove").with(Command::RemoveNode {
            node: "btn_ok".into(),
        }),
    )
    .unwrap();

    let (tx, created) = area.paste(d.ir(), namer(Default::default())).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(
        d.ir().node(&created[0]).unwrap().text(),
        Some("&OK"),
        "clipboard stores data, not a reference"
    );
}

#[test]
fn copying_the_form_makes_no_sense() {
    let d = doc();
    let mut area = Clipboard::default();
    assert_eq!(area.copy(d.ir(), &["form_main".into()]), 0);
    assert!(area.is_empty());
}

#[test]
fn pasting_with_empty_clipboard_does_nothing() {
    let d = doc();
    let area = Clipboard::default();
    assert!(area.paste(d.ir(), namer(Default::default())).is_none());
}

#[test]
fn duplicate_does_not_touch_clipboard() {
    let mut d = doc();
    let mut area = Clipboard::default();
    area.copy(d.ir(), &["edt_nome".into()]);

    let (tx, created) = duplicate(d.ir(), &["btn_ok".into()], namer(Default::default())).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(d.ir().node(&created[0]).unwrap().class, "Button");

    // Edit continues copied
    let (tx, created) = area.paste(d.ir(), |c| format!("{c}_pasted")).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(d.ir().node(&created[0]).unwrap().class, "Edit");
}

#[test]
fn duplicate_carries_target_specific_properties() {
    let mut d = doc();
    d.apply(
        crawrad_doc::Transaction::user("password").with(Command::SetProp {
            node: "edt_nome".into(),
            key: "win32.PasswordChar".into(),
            value: Some(Value::Str("*".into())),
        }),
    )
    .unwrap();

    let (tx, created) = duplicate(d.ir(), &["edt_nome".into()], |c| format!("{c}_2")).unwrap();
    d.apply(tx).unwrap();
    assert_eq!(
        d.ir()
            .node(&created[0])
            .unwrap()
            .props
            .get("win32.PasswordChar"),
        Some(&Value::Str("*".into()))
    );
    assert!(d.ir().validate().is_ok());
}

#[test]
fn content_travels_as_text_to_system_clipboard() {
    // This is what makes Ctrl+V work (egui only delivers paste event
    // when there is text in clipboard) and allows pasting between two windows.
    let d = doc();
    let mut area = Clipboard::default();
    area.copy(d.ir(), &["btn_ok".into(), "edt_nome".into()]);

    let txt = area.to_text();
    let back = Clipboard::from_text(&txt).expect("should recognize own format");
    assert_eq!(back.len(), 2);
    assert_eq!(back.nodes[0].text(), Some("&OK"));
}

#[test]
fn text_from_outside_does_not_become_component() {
    assert!(Clipboard::from_text("some text").is_none());
    assert!(Clipboard::from_text("{\"kind\":\"other.thing\",\"nodes\":[]}").is_none());
    assert!(Clipboard::from_text("{\"kind\":\"crawrad.components\",\"nodes\":[]}").is_none());
}

#[test]
fn copy_enters_at_end_of_tab_sequence() {
    // Inheriting original's number would leave two components competing for the same
    // Tab position — and the second would be decided by Z-order, without user
    // having asked.
    let mut d = doc();
    for (id, t) in [("btn_ok", 0), ("edt_nome", 1)] {
        d.apply(crawrad_doc::Transaction::user("t").with(Command::SetProp {
            node: id.into(),
            key: "tab_order".into(),
            value: Some(Value::Int(t)),
        }))
        .unwrap();
    }

    let mut area = Clipboard::default();
    area.copy(d.ir(), &["btn_ok".into(), "edt_nome".into()]);
    let (tx, created) = area.paste(d.ir(), namer(Default::default())).unwrap();
    d.apply(tx).unwrap();

    let tab = |id: &str| d.ir().node(id).unwrap().props.get("tab_order").cloned();
    assert_eq!(tab(&created[0]), Some(Value::Int(2)));
    assert_eq!(tab(&created[1]), Some(Value::Int(3)));
    assert_eq!(tab("btn_ok"), Some(Value::Int(0)), "original does not move");
}
