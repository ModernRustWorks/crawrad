//! The handler name: how it is given, and how it is recognized back.
//!
//! The two halves live together on purpose. The namer is
//! [`handler_padrao`]; the recognizer is [`e_nome_derivado`], and recognition
//! is what decides whether the `fn` in the user file is our stub — which pruning
//! can remove — or a user method, which is untouched. A rule changing only on
//! one side does not just produce an ugly name: it produces pruning that deletes
//! what is not its own, or never deletes anything again.
//!
//! # The rule changed on 16/09/2026, which is why there are two
//!
//! The name was `btn_okClick`, in the style of `Button1Click` from a classic RAD IDE.
//! It became `btn_ok_click`, which is the Rust standard — owner's decision: *"A classic RAD IDE is
//! only a reference for FEATURES, do not copy its syntax nomenclature."* The price
//! of the old name was an **inner** `#![allow(non_snake_case)]` at the top of the user's
//! file that silenced the entire file — including whatever the
//! user wrote in it. With the right name the warning disappears.
//!
//! Recognition accepts **both**: in a project generated before that date
//! the name CrawRAD gave was the old one, and the rule is *only touch what I
//! wrote, under the name I gave it*. Without the frozen rule, the empty stub of every
//! existing project would never be pruned again — and automatic handler
//! cleanup, which is an existing feature, would silently regress.

use super::*;

/// The text of a user file with **one** handler, just as the
/// generator writes it: marker on top, empty body, and end marker.
fn arquivo_com(marca_componente: &str, evento: &str, f: &str) -> String {
    format!(
        "use crate::form_main_designer::*;\n\
         \n\
         impl FormMain {{\n\
         \x20   /// `{marca_componente}.{evento}`\n\
         \x20   pub fn {f}(&mut self) {{\n\
         \x20       \n\
         \x20   }}\n\
         \n\
         \x20   // New event handlers go here.\n\
         }}\n"
    )
}

/// What pruning would take from this text.
fn podaveis(fonte: &str) -> Vec<String> {
    crate::rxp::podaveis(fonte, &[], &[])
        .into_iter()
        .map(|(_, nome)| nome)
        .collect()
}

#[test]
fn o_nome_e_o_componente_mais_o_evento_em_snake_case() {
    assert_eq!(handler_padrao("btn_ok", "on_click"), "btn_ok_click");
    assert_eq!(
        handler_padrao("edt_nome", "on_change"),
        "edt_nome_change",
        "single-word event must not lose its separator"
    );
    assert_eq!(
        handler_padrao("form_main", "on_close_query"),
        "form_main_close_query",
        "two-word event arrives in full"
    );
}

/// The component enters **as the user wrote it**, not as the IDE
/// would like it to have been written.
///
/// Until 16/09/2026 there was a case conversion here, and this test checked the
/// opposite: `Button1` + `on_key_down` had to yield `button1_key_down`. It was
/// removed by order of the owner — *"I don't want to help the classic RAD IDE developer come
/// write as if they were in a classic RAD IDE and let the IDE deal with it"*.
///
/// **This does not put `Button1_key_down` in any file**, and that is the half
/// that matters: what prevents it is the entry gate, in `Doc::apply`, which rejects
/// `Button1` saying `use \u{60}button1\u{60}`. Every component that reaches here
/// is already snake_case, because there is no other way for it to exist — the conversion was
/// a no-op for well-formed names, and only existed to save ill-formed ones.
#[test]
fn o_componente_entra_como_o_usuario_o_escreveu() {
    assert_eq!(handler_padrao("button1", "on_key_down"), "button1_key_down");
    assert_eq!(
        handler_padrao("tree_view1", "on_expanded"),
        "tree_view1_expanded"
    );
    // The pair that proves the translation was removed: if it returned, this line would
    // yield `button1_key_down` and the test would say why.
    assert_eq!(
        handler_padrao("Button1", "on_key_down"),
        "Button1_key_down",
        "generator does not fix name underneath — `Doc::apply` is what rejects it"
    );
}

/// And the entry gate indeed rejects it, writing out the correct form.
///
/// This is the other side of the test above. Without it, `Button1_key_down` there seems
/// like an isolated defect rather than a consequence of the refusal living elsewhere.
#[test]
fn o_nome_que_daria_handler_feio_nao_entra_no_documento() {
    assert!(
        !crawrad_ir::is_snake_case_name("Button1"),
        "if `Button1` were accepted, the ugly handler would have a way to arise"
    );
    assert_eq!(
        crawrad_ir::snake_case_hint("Button1").as_deref(),
        Some("button1"),
        "rejection must say what to write, not just say no"
    );
}

/// The **entire** catalogue, not just an example: it is the same criterion as
/// `rustc`'s `non_snake_case`, written here because the user's file no
/// longer has any `allow` to silence it.
///
/// The component list lost `Button1` on 16/09/2026: it measured what the
/// conversion fixed, and the conversion was removed. What replaces it is
/// `o_nome_que_daria_handler_feio_nao_entra_no_documento` — the guarantee changed
/// location, from fixing to refusing, but did not cease to exist.
#[test]
fn todo_handler_do_catalogo_passaria_no_non_snake_case() {
    let r = crawrad_registry::Registry::builtin();
    let mut medidos = 0usize;
    for componente in ["btn_ok", "button1", "edt_nome", "cb_uf", "form_main"] {
        for desc in r.all() {
            for e in &desc.events {
                let h = handler_padrao(componente, e.name);
                assert!(
                    h.chars()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
                    "`{componente}` + `{}` produced `{h}`, which `rustc` would flag",
                    e.name
                );
                assert!(!h.contains("__"), "`{h}` has double underscore");
                assert!(!h.starts_with('_'), "`{h}` starts with underscore");
                medidos += 1;
            }
        }
    }
    assert!(
        medidos > 0,
        "registry shrank and this test measures nothing"
    );
}

/// The frozen rule returns the old name — and it never changes, because yesterday
/// does not change.
#[test]
fn a_regra_congelada_devolve_o_nome_de_antes() {
    assert_eq!(handler_padrao_legado("btn_ok", "on_click"), "btn_okClick");
    assert_eq!(
        handler_padrao_legado("form_main", "on_close_query"),
        "form_mainCloseQuery"
    );
}

#[test]
fn o_reconhecimento_aceita_as_duas_reguas_e_so_elas() {
    assert!(e_nome_derivado("btn_ok", "on_click", "btn_ok_click"));
    assert!(e_nome_derivado("btn_ok", "on_click", "btn_okClick"));
    assert!(
        !e_nome_derivado("btn_ok", "on_click", "salvar"),
        "a name manually assigned in the Inspector is not our stub"
    );
    assert!(
        !e_nome_derivado("btn_ok", "on_click", "btn_ok_clicked"),
        "almost identical is not identical"
    );
}

/// Pruning recognizes the stub named by **today's** rule.
///
/// Without this test the new rule lacked pruning coverage: the **entire** test
/// suite exercising pruning was born with old-named documents, and the
/// frozen rule would answer for it.
///
/// Measured by mutation, and both numbers say the same thing from opposite
/// sides: leaving only TODAY's rule in [`e_nome_derivado`] breaks **16**
/// tests (the magnitude of what would regress on the disk of existing users); leaving
/// only the FROZEN rule breaks **2** — this one and [`o_reconhecimento_aceita_as_duas_reguas_e_so_elas`],
/// which were zero before they existed.
#[test]
fn a_poda_reconhece_o_stub_da_regra_de_hoje() {
    let fonte = arquivo_com("btn_ok", "on_click", "btn_ok_click");
    assert_eq!(podaveis(&fonte), vec!["btn_ok_click".to_string()]);
}

/// Pruning recognizes the stub named by **yesterday's** rule — which is what
/// is on disk for every project generated before 16/09/2026.
#[test]
fn a_poda_reconhece_o_stub_da_regra_de_ontem() {
    let fonte = arquivo_com("btn_ok", "on_click", "btn_okClick");
    assert_eq!(podaveis(&fonte), vec!["btn_okClick".to_string()]);
}

/// And it does not recognize what neither rule would produce. This is the side that matters:
/// pruning that errs on the side of excess deletes user work.
#[test]
fn a_poda_nao_reconhece_nome_que_regra_nenhuma_daria() {
    let fonte = arquivo_com("btn_ok", "on_click", "salvar_o_cadastro");
    assert!(
        podaveis(&fonte).is_empty(),
        "pruning would remove a method it did not write:\n{fonte}"
    );
}

/// A handler with a written body is not removed, whatever name it has. The name
/// rule is the first gate, not the only one.
#[test]
fn o_nome_certo_com_corpo_escrito_continua_sendo_do_usuario() {
    let fonte = arquivo_com("btn_ok", "on_click", "btn_ok_click")
        .replace("        \n", "        self.salvar();\n");
    assert!(
        podaveis(&fonte).is_empty(),
        "pruning removed a handler with body:\n{fonte}"
    );
}
