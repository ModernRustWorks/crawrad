//! The user file in **CRLF** remains the same file.
//!
//! # The defect, measured on screen
//!
//! 16/09/2026: double-clicking `OnMouseDown` on a button in the sample app
//! created the handler with the right signature — and placed it in a **second**
//! `impl FormMain`, below the first. The code compiles, which is why this
//! would go unnoticed for months.
//!
//! The cause is a single character: [`crate::rxp::MARCAS_ACEITAS`] is written with `\n`,
//! and the file came from a `git checkout` with `core.autocrlf=true`. A
//! raw `contains` cannot find `"\n    // ...\n"` inside `"\r\n    // ...\r\n"`,
//! and failing to find the anchor sends the handler to the end.
//!
//! **This is not an edge case.** It is what anyone gets when cloning the
//! repository on Windows, and it is what Notepad writes.
//!
//! # What these tests guard
//!
//! Two behaviors, and the second only exists because of the first: finding the
//! mark in CRLF, and writing the block with the SAME line ending as the file — a
//! `\n` snippet inside a `\r\n` file leaves the unit mixed, and the next
//! editor normalizes the entire file, turning a three-line handler into a
//! whole-file `git diff`.

use crawrad_ir::{Document, Node, Rect};

use crate::{Gerador, escrever, targets::AlvoGerador};

/// The minimal document for this test: a form with an `on_click` button
/// wired.
///
/// Minimal on purpose — what is under test is the ANCHOR, and each extra
/// component would be noise in the text that the assertion reads.
fn documento() -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").unwrap();
        raiz.set_rect(Rect::new(0.0, 0.0, 480.0, 320.0));
        raiz.set_name("form_main");
    }
    let mut btn = Node::new("btn1", "Button");
    btn.set_name("btn_ok");
    btn.set_rect(Rect::new(24.0, 96.0, 96.0, 32.0));
    btn.events.insert("on_click".into(), "btn_ok_click".into());
    d.add_child("form_main", btn).unwrap();
    d.validate().unwrap();
    d
}

fn gerar_em(raiz: &std::path::Path, d: &Document) {
    let arquivos = AlvoGerador::default()
        .create_generator()
        .gerar(d, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("generation");
    escrever(raiz, &arquivos).expect("writing");
}

fn unidade(raiz: &std::path::Path) -> String {
    std::fs::read_to_string(raiz.join("src/form_main.rs")).expect("user unit")
}

/// A folder with the project generated once and the user unit converted to
/// CRLF — the state the file arrives in from a `git clone` on Windows.
fn projeto_em_crlf(sufixo: &str) -> std::path::PathBuf {
    let raiz = std::env::temp_dir().join(format!("crawrad_codegen_crlf_{sufixo}"));
    let _ = std::fs::remove_dir_all(&raiz);
    gerar_em(&raiz, &documento());
    let caminho = raiz.join("src/form_main.rs");
    let texto = std::fs::read_to_string(&caminho).expect("unit");
    // Normalization first: without it, a `\r\n` that was already there would
    // become `\r\r\n` and the test would measure a file nobody has.
    let crlf = texto.replace("\r\n", "\n").replace('\n', "\r\n");
    std::fs::write(&caminho, crlf).expect("unit in CRLF");
    raiz
}

/// The document from [`documento`] with a second event wired — the gesture of
/// double-clicking a row in the `Events` tab.
fn com_o_mouse_down() -> Document {
    let mut d = documento();
    d.node_mut("btn1")
        .unwrap()
        .events
        .insert("on_mouse_down".into(), "btn_ok_mouse_down".into());
    d.validate().unwrap();
    d
}

#[test]
fn o_handler_novo_entra_na_marca_mesmo_com_o_arquivo_em_crlf() {
    let raiz = projeto_em_crlf("marca");
    let antes = unidade(&raiz);
    assert!(
        antes.contains("\r\n"),
        "setup failed: unit is not in CRLF:\n{antes}"
    );

    gerar_em(&raiz, &com_o_mouse_down());
    let depois = unidade(&raiz);

    assert!(
        depois.contains("pub fn btn_ok_mouse_down"),
        "handler was not written:\n{depois}"
    );
    // The `impl` DOES NOT SPLIT. This is the assertion that fails without the fix:
    // the handler was there, and in a second block.
    assert_eq!(
        depois.matches("impl FormMain").count(),
        1,
        "handler was spawned in a second `impl` instead of at the mark:\n{depois}"
    );
    // And it is placed ABOVE the mark, which is where the next one will also go.
    let handler = depois.find("pub fn btn_ok_mouse_down").expect("handler");
    let marca = depois
        .find("New event handlers go here")
        .expect("generator mark");
    assert!(
        handler < marca,
        "handler was written AFTER the mark:\n{depois}"
    );
}

#[test]
fn o_bloco_novo_nasce_no_fim_de_linha_do_arquivo() {
    let raiz = projeto_em_crlf("misto");
    gerar_em(&raiz, &com_o_mouse_down());
    let depois = unidade(&raiz);

    // No stray `\n`: every `\n` in the file has a `\r` before it. That is the
    // definition of "did not become mixed", and it is measured by counting.
    assert_eq!(
        depois.matches('\n').count(),
        depois.matches("\r\n").count(),
        "block was inserted with LF in a CRLF file, leaving the unit mixed:\n{depois:?}"
    );
}

#[test]
fn a_marca_e_achada_nos_dois_fins_de_linha() {
    // The minimal unit of the fix, and it answers BOTH things the inserter
    // needs to know: where, and with which line ending.
    let lf = "impl F {\n    // New event handlers go here.\n}\n";
    let crlf = lf.replace('\n', "\r\n");

    let (pos_lf, é_crlf) = crate::rxp::marca_em(lf).expect("mark in LF");
    assert!(!é_crlf);
    assert_eq!(&lf[pos_lf..pos_lf + 1], "\n");

    let (pos_crlf, é_crlf) = crate::rxp::marca_em(&crlf).expect("mark in CRLF");
    assert!(é_crlf, "mark was found, but CRLF was not reported");
    assert_eq!(&crlf[pos_crlf..pos_crlf + 2], "\r\n");

    // And a file with no mark at all still reports that it has none: that is what
    // sends the handler to the end, on purpose, when the user deleted the
    // mark.
    assert_eq!(crate::rxp::marca_em("impl F {\n}\n"), None);
}
