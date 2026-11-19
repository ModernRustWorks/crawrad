//! **The empty handler goes away on save** — the case the owner photographed.
//!
//! On 17/09/2026 he double-clicked `OnClick` of a `CheckBox`, got the handler,
//! pressed `Ctrl+S`, and reported:
//!
//! > *"Ctrl+S cleared the link, but not the event block and it is empty."*
//!
//! The wiring went, the `fn` stayed. That is the worst half of the two: the
//! Object Inspector says nothing is wired and the file still carries a handler
//! nobody calls — and the next double-click on the same event writes a SECOND
//! one, because the name is taken.
//!
//! The text below is his file, transcribed from the screenshot, and it is the
//! whole point of this test: a hand-made sample would be the sample that
//! passes.

use crate::rxp::stubs::podaveis;

/// The owner's file, with the newly-created and empty handler at the end.
const ARQUIVO: &str = r#"//! **Este arquivo é seu.** O CrawRAD nunca o sobrescreve.
//!
//! Quando você liga um evento novo no Object Inspector, o handler aparece
//! na marca lá embaixo — e nada do que você escreveu é tocado.

use crate::form_main_designer::FormMain;

impl FormMain {
    /// `btn_ok.on_click`
    pub fn btn_ok_click(&mut self) {
        // seu código aqui
        self.edt_nome_.text = format!("Olá, {}!", self.edt_nome_.text);
        self.btn_ok.enabled = false;
        self.btn_cancel.text = "Desfazer".into();
    }

    /// `btn_cancel.on_click`
    pub fn btn_cancel_click(&mut self) {
        // seu código aqui
        self.edt_nome_.text.clear();
        self.btn_ok.enabled = true;
        self.btn_cancel.text = "Cancel".into();
    }

    /// `check_box1.on_click`
    pub fn check_box1_click(&mut self) {

    }

    // Os handlers dos eventos novos entram aqui.
}
"#;

/// **Empty handlers without bindings are prunable.**
///
/// `protegidos` comes from generator additions, and `Ctrl+S` already removed the binding —
/// so `check_box1_click` is not there, and the two written by the user
/// are.
#[test]
fn o_handler_vazio_que_perdeu_o_vinculo_sai_no_save() {
    let cortes = podaveis(
        ARQUIVO,
        &["btn_ok_click", "btn_cancel_click"],
        &[String::new()],
    );
    let nomes: Vec<&str> = cortes.iter().map(|(_, n)| n.as_str()).collect();
    assert_eq!(
        nomes,
        vec!["check_box1_click"],
        "empty handler that lost its binding was not pruned. The Inspector \
         says nothing is wired and the file still carries an uncalled `fn` \
         — and the next double-click on the same event writes a SECOND \
         one, because the name is taken."
    );
}

/// **And the two with bodies remain**, the other half of the contract.
#[test]
fn o_handler_com_corpo_nunca_sai() {
    let cortes = podaveis(ARQUIVO, &[], &[String::new()]);
    let nomes: Vec<&str> = cortes.iter().map(|(_, n)| n.as_str()).collect();
    assert!(
        !nomes.contains(&"btn_ok_click") && !nomes.contains(&"btn_cancel_click"),
        "pruning removed user code: {nomes:?}"
    );
}

/// The same file, but with the handler in the MIDDLE of the `impl` (between two methods).
const ARQUIVO_MEIO: &str = r#"//! **Este arquivo é seu.** O CrawRAD nunca o sobrescreve.

use crate::form_main_designer::FormMain;

impl FormMain {
    /// `btn_ok.on_click`
    pub fn btn_ok_click(&mut self) {
        self.edt_nome_.text = format!("Olá, {}!", self.edt_nome_.text);
        self.btn_ok.enabled = false;
    }

    /// `check_box1.on_click`
    pub fn check_box1_click(&mut self) {

    }

    /// `btn_cancel.on_click`
    pub fn btn_cancel_click(&mut self) {
        self.edt_nome_.text.clear();
        self.btn_cancel.text = "Cancel".into();
    }

    // Os handlers dos eventos novos entram aqui.
}
"#;

#[test]
fn an_empty_handler_in_the_middle_of_the_impl_is_pruned_on_save() {
    use crate::rxp::stubs::podar;

    let cortes = podaveis(
        ARQUIVO_MEIO,
        &["btn_ok_click", "btn_cancel_click"],
        &[String::new()],
    );
    let names: Vec<&str> = cortes.iter().map(|(_, n)| n.as_str()).collect();
    assert_eq!(
        names,
        vec!["check_box1_click"],
        "empty handler positioned in the middle of the impl must be pruned"
    );

    let (output, podados) = podar(
        ARQUIVO_MEIO,
        &["btn_ok_click", "btn_cancel_click"],
        &[String::new()],
    );
    assert_eq!(podados, vec!["check_box1_click"]);
    assert!(!output.contains("check_box1_click"));
    assert!(output.contains("pub fn btn_ok_click"));
    assert!(output.contains("pub fn btn_cancel_click"));
}

#[test]
fn a_handler_with_a_body_in_the_middle_of_the_impl_is_never_pruned() {
    let source = ARQUIVO_MEIO.replace(
        "pub fn check_box1_click(&mut self) {\n\n    }",
        "pub fn check_box1_click(&mut self) {\n        println!(\"keep me\");\n    }",
    );
    let cortes = podaveis(&source, &[], &[String::new()]);
    let names: Vec<&str> = cortes.iter().map(|(_, n)| n.as_str()).collect();
    assert!(
        !names.contains(&"check_box1_click"),
        "handler in the middle of the impl with a body must not be pruned: {names:?}"
    );
}
