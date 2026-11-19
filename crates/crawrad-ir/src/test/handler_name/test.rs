use super::*;

#[test]
fn default_and_legacy_handler_names() {
    assert_eq!(default_handler_name("btn_ok", "on_click"), "btn_ok_click");
    assert_eq!(legacy_handler_name("btn_ok", "on_click"), "btn_okClick");
    assert!(is_derived_handler_name(
        "btn_ok",
        "on_click",
        "btn_ok_click"
    ));
    assert!(is_derived_handler_name("btn_ok", "on_click", "btn_okClick"));
    assert!(!is_derived_handler_name(
        "btn_ok",
        "on_click",
        "custom_click"
    ));
}

#[test]
fn handler_identifier_validation() {
    assert!(is_handler_ident("btn_ok_click"));
    assert!(is_handler_ident("on_click"));
    assert!(is_handler_ident("_click"));
    assert!(is_handler_ident("onClick"));
    assert!(is_handler_ident("btn1"));
    assert!(is_handler_ident("a"));
    assert!(is_handler_ident("__"));

    assert!(!is_handler_ident(""));
    assert!(!is_handler_ident("_"));
    assert!(!is_handler_ident("1x"));
    assert!(!is_handler_ident("btn-ok"));
    assert!(!is_handler_ident("btn ok"));
}
