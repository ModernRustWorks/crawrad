use super::*;

#[test]
fn params_need_key_lang_and_text() {
    let full = json!({ "key": "f.b.text", "lang": "de", "text": "Ja" });
    assert_eq!(params(&full), Some(("f.b.text", "de", "Ja")));
    assert_eq!(params(&json!({ "key": "f.b.text", "lang": "de" })), None);
}

#[test]
fn answer_maps_the_outcome() {
    assert_eq!(answer(Some(Ok("set".into()))).1, "applied");
    assert_eq!(answer(Some(Err("bad".into()))).1, "error");
    assert_eq!(answer(None).1, "error");
}
