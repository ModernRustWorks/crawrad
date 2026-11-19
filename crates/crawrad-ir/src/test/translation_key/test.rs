use super::*;

#[test]
fn text_key_formats_form_node_prop() {
    let node_id = "btn_1".to_string();
    assert_eq!(
        text_key("form_main", &node_id, "text"),
        "form_main.btn_1.text"
    );
}

#[test]
fn translatable_contains_expected_keys() {
    assert_eq!(TRANSLATABLE, &["text", "hint", "title", "text_hint"]);
}
