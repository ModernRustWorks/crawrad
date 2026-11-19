//! **Enter in the generated app's text boxes goes through the classic RAD rule**
//! the form's `handle_event` hands each key to
//! `crawrad_runtime_makepad::keys::text_box_enter` with its memos and its edits.

use super::text_box_enter_call;
use crawrad_ir::Node;

fn node(id: &str, class: &str, name: &str) -> Node {
    let mut n = Node::new(id, class);
    n.set_name(name);
    n
}

#[test]
fn a_form_with_a_memo_and_an_edit_hands_them_both_to_the_rule() {
    let memo = node("mmo1", "Memo", "mmo_obs");
    let edit = node("edt1", "Edit", "edt_nome");
    let button = node("btn1", "Button", "btn_ok");
    let call = text_box_enter_call(&[&memo, &edit, &button]).unwrap();
    assert!(
        call.contains(
            "crawrad_runtime_makepad::keys::text_box_enter(cx, &[self.ui.widget(id!(mmo_obs))], \
             &[self.ui.widget(id!(edt_nome))], event)"
        ),
        "the memo goes in the first list and the edit in the second:\n{call}"
    );
    assert!(
        call.contains("let event = text_box_key.as_ref().unwrap_or(event);"),
        "the widgets must get the rewritten key, not the original:\n{call}"
    );
    assert!(
        !call.contains("btn_ok"),
        "a button is not a text box:\n{call}"
    );
}

/// The generated designer of a form with a Memo, whole.
fn designer_with_a_memo() -> String {
    let r = crawrad_registry::Registry::builtin();
    let mut d = crawrad_ir::Document::new("form_main", "Form");
    d.node_mut("form_main")
        .expect("the root")
        .set_rect(crawrad_ir::Rect::new(0.0, 0.0, 640.0, 480.0));
    let mut memo = r
        .instantiate("Memo", "mmo1", crawrad_ir::Rect::new(8.0, 8.0, 200.0, 80.0))
        .expect("the catalogue's Memo");
    memo.set_name("mmo_obs");
    d.add_child("form_main", memo).unwrap();
    d.validate().unwrap();
    let root = d.node("form_main").unwrap();
    let memo = d.node("mmo1").unwrap();
    super::form_designer_rs(&d, &r, root, "FormMain", &[memo]).unwrap()
}

#[test]
fn the_generated_handle_event_rewrites_the_key_before_any_widget_sees_it() {
    let code = designer_with_a_memo();
    let handle = &code[code
        .find("fn handle_event(")
        .expect("the app's handle_event")..];
    let rule = handle
        .find("keys::text_box_enter(")
        .expect("the handle_event must hand the key to the classic RAD rule");
    for widgets_see_it in [
        "self.match_event(cx, event)",
        "self.ui.handle_event(cx, event",
    ] {
        let at = handle.find(widgets_see_it).expect(widgets_see_it);
        assert!(
            rule < at,
            "`{widgets_see_it}` runs before the key is rewritten — the Memo \
             would get the original Enter:\n{handle}"
        );
    }
}

#[test]
fn a_form_without_text_boxes_writes_nothing() {
    let button = node("btn1", "Button", "btn_ok");
    assert_eq!(text_box_enter_call(&[&button]).unwrap(), "");
}
