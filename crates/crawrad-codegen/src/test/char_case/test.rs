//! **The Edit's `CharCase` reaches both targets**.
//!
//! The design-time text is turned when the app is generated, and what is
//! typed is turned as it changes: egui through the `char_case` section of
//! `desenho_comum.rs`, Makepad through `crawrad_runtime_makepad::edit::hold_case`.

use crate::Gerador;
use crawrad_ir::{Document, Rect, Value};

fn form(case: &str) -> Document {
    let r = crawrad_registry::Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    let mut n = r
        .instantiate("Edit", "edt", Rect::new(10.0, 10.0, 200.0, 26.0))
        .unwrap();
    n.set_name("edt");
    n.props.insert("text".into(), Value::Str("Hello".into()));
    n.props.insert("char_case".into(), Value::Enum(case.into()));
    d.add_child("form_main", n).unwrap();
    d
}

fn designer(g: &dyn Gerador, doc: &Document) -> String {
    g.gerar(doc, "app", &crawrad_registry::Registry::builtin())
        .expect("generating")
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().contains("designer"))
        .expect("the designer file")
        .conteudo
}

#[test]
fn egui_turns_the_text_it_opens_with_and_the_text_typed() {
    let code = designer(&crate::GeradorRxp, &form("ecUpperCase"));
    assert!(code.contains("text: \"HELLO\".to_string()"), "{code}");
    assert!(
        code.contains("char_case: \"ecUpperCase\".to_string()"),
        "{code}"
    );
    assert!(
        code.contains("self.edt.text = char_cased(&self.edt.text, &self.edt.char_case);"),
        "{code}"
    );
    assert!(
        code.contains("pub fn char_cased("),
        "the section rides along"
    );
}

#[test]
fn makepad_turns_the_text_it_opens_with_and_the_text_typed() {
    let code = designer(&crate::GeradorMakepad, &form("ecLowerCase"));
    assert!(code.contains("text: \"hello\".to_string()"), "{code}");
    assert!(
        code.matches("text: \"hello\"").count() >= 2,
        "the DSL text too:\n{code}"
    );
    assert!(
        code.contains("crawrad_runtime_makepad::edit::hold_case(cx, &self.ui.text_input(id!(edt)), &_val, &self.form.edt.char_case)"),
        "{code}"
    );
}

#[test]
fn ec_normal_leaves_the_text() {
    let code = designer(&crate::GeradorRxp, &form("ecNormal"));
    assert!(code.contains("text: \"Hello\".to_string()"), "{code}");
}
