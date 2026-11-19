//! The gate of the three fictions the Makepad shell status bar used to tell.
//!
//! Each test here exists because of a line that **compiled** and lied:
//! the open file path hardcoded in the DSL, the node count hardcoded,
//! and the form loaded from `include_str!`. None of this broke the
//! build, and none showed up as an error — exactly the defect category
//! that can only be caught by locking the RULE in a test.
//!
//! What these tests cover is the PURE part: search order, failure
//! message, status bar summary, and tree rows. What they do not cover,
//! which is noted intentionally: whether the DSL `Label` actually receives this text —
//! that is screen, and screen is verified by looking.

use super::documento::{self, Linha};
use crawrad_doc::Doc;
use crawrad_ir::{Document, Node, Value};
use std::path::{Path, PathBuf};

fn bases() -> Vec<PathBuf> {
    vec![PathBuf::from("/base1"), PathBuf::from("/base2")]
}

#[test]
fn o_argumento_da_linha_de_comando_ganha_de_tudo() {
    let c = documento::candidatos(
        Some("meu.crawform".into()),
        Some("do_ambiente.crawform".into()),
        &bases(),
    );
    assert_eq!(
        c,
        vec![
            PathBuf::from("/base1/meu.crawform"),
            PathBuf::from("/base2/meu.crawform"),
        ],
        "with an argument, neither the variable nor repository candidates are included"
    );
}

#[test]
fn sem_argumento_a_variavel_de_ambiente_responde() {
    let c = documento::candidatos(None, Some("do_ambiente.crawform".into()), &bases());
    assert_eq!(c.len(), 2);
    assert!(c[0].ends_with("do_ambiente.crawform"));
}

#[test]
fn sem_nada_entram_os_candidatos_do_repositorio_na_ordem() {
    let c = documento::candidatos(None, None, &bases());
    assert_eq!(c.len(), documento::CANDIDATOS.len() * 2);
    assert!(c[0].ends_with("form_main.crawform"), "{c:?}");
}

/// The last project opens when nobody named a form — alone: the
/// repository's samples never open by themselves — and a form named on the
/// command line or in the variable is an order it never jumps in front of.
#[test]
fn the_last_project_goes_first_only_when_nothing_was_named() {
    let last = Some(PathBuf::from("/last/src/form.crawform"));
    let c = documento::start_list(None, None, last.clone(), &bases());
    assert_eq!(c, vec![PathBuf::from("/last/src/form.crawform")]);
    let c = documento::start_list(Some("meu.crawform".into()), None, last.clone(), &bases());
    assert!(
        !c.contains(&PathBuf::from("/last/src/form.crawform")),
        "{c:?}"
    );
    let c = documento::start_list(None, Some("env.crawform".into()), last, &bases());
    assert!(c[0].ends_with("env.crawform"), "{c:?}");
    // Nothing named and no last project: the Welcome page's turn.
    assert!(documento::start_list(None, None, None, &bases()).is_empty());
}

#[test]
fn caminho_absoluto_nao_ganha_base_nenhuma() {
    // Joining a base to an absolute path changes nothing, and would cause the
    // error message attempt list to repeat the same path N times.
    let absoluto = if cfg!(windows) {
        "C:\\tmp\\form.crawform"
    } else {
        "/tmp/form.crawform"
    };
    let c = documento::candidatos(Some(absoluto.into()), None, &bases());
    assert_eq!(c, vec![PathBuf::from(absoluto)]);
}

#[test]
fn quando_nada_carrega_a_falha_diz_tudo_que_foi_tentado() {
    let lista = vec![PathBuf::from("/a.crawform"), PathBuf::from("/b.crawform")];
    let erro = documento::carregar(&lista, |_| Err("no such file".into()))
        .expect_err("nothing should have loaded");
    assert!(erro.contains("a.crawform"), "{erro}");
    assert!(erro.contains("b.crawform"), "{erro}");
    assert!(erro.contains("no such file"), "{erro}");
}

#[test]
fn first_loaded_wins_and_returns_path() {
    let lista = vec![PathBuf::from("/a.crawform"), PathBuf::from("/b.crawform")];
    let json = Document::new("form", "Form").to_json().unwrap();
    let aberto = documento::carregar(&lista, |p: &Path| {
        if p.ends_with("b.crawform") {
            Ok(json.clone())
        } else {
            Err("no such file".into())
        }
    })
    .expect("the second one should have opened");
    assert_eq!(aberto.caminho, PathBuf::from("/b.crawform"));
    assert_eq!(aberto.doc.root, "form");
}

#[test]
fn texto_que_nao_e_formulario_nao_vira_documento_em_silencio() {
    let lista = vec![PathBuf::from("/a.crawform")];
    let erro = documento::carregar(&lista, |_| Ok("{ not a form }".into()))
        .expect_err("invalid json should not open");
    assert!(erro.contains("a.crawform"), "{erro}");
}

#[test]
fn sem_documento_os_numeros_da_barra_sao_travessao() {
    let c = documento::counters(None);
    assert_eq!(c.nodes, "—");
    assert_eq!(c.revision, "—");
    assert_eq!(c.selection, "no selection");
    // NO digits, and the check covers all three cells together: a `0` in one
    // of them is a plausible number for something that does not exist, and was
    // exactly the falsehood this bar once had to remove from the screen.
    for campo in [&c.selection, &c.nodes, &c.revision] {
        assert!(
            !campo.chars().any(|ch| ch.is_ascii_digit()),
            "without a document, not a single digit should remain on the bar: {campo}"
        );
    }
}

fn doc_de_teste() -> Doc {
    let mut d = Document::new("form_main", "Form");
    let mut botao = Node::new("btn_ok", "Button");
    // The name is intentionally DIFFERENT from the id: this is what ADR 0008 separated,
    // and the UI displays the name.
    botao
        .props
        .insert("name".into(), Value::Str("OkButton".into()));
    d.add_child("form_main", botao).unwrap();
    Doc::new(d)
}

#[test]
fn o_resumo_conta_os_nos_do_documento_aberto() {
    let d = doc_de_teste();
    let c = documento::counters(Some(&d));
    // The numbers are READ from the document: two nodes (the root counts, as in the
    // egui shell — `main.rs:6334`) and revision zero for a newly opened document.
    assert_eq!(c.selection, "no selection");
    assert_eq!(c.nodes, "2");
    assert_eq!(c.revision, "0");
}

#[test]
fn o_resumo_diz_o_nome_do_selecionado_e_nao_o_id() {
    let mut d = doc_de_teste();
    d.select_one("btn_ok");
    let c = documento::counters(Some(&d));
    assert_eq!(c.selection, "OkButton selected");
    assert!(
        !c.selection.contains("btn_ok"),
        "the id does not appear in the interface: {}",
        c.selection
    );
}

/// **The three cells arrive SEPARATED, which is the point.**
///
/// Until 2026-09-17 this was a single string — `no selection    2 nodes
/// rev 0` — and the rich bar requested by the owner renders each fact in its
/// own cell. Splitting them after the fact would require searching for spaces in
/// text where a component's NAME can contain anything: a button named
/// `2 nodes` would break the parsing.
#[test]
fn um_nome_com_espaco_nao_confunde_as_celulas() {
    let mut d = Document::new("form_main", "Form");
    let mut botao = Node::new("btn_ok", "Button");
    botao
        .props
        .insert("name".into(), Value::Str("2 nodes    rev 9".into()));
    d.add_child("form_main", botao).unwrap();
    let mut d = Doc::new(d);
    d.select_one("btn_ok");

    let c = documento::counters(Some(&d));
    assert_eq!(c.selection, "2 nodes    rev 9 selected");
    assert_eq!(
        c.nodes, "2",
        "the count did not come from the component name"
    );
    assert_eq!(
        c.revision, "0",
        "the revision did not come from the component name"
    );
}

#[test]
fn file_label_shows_open_path_or_failure() {
    let p = PathBuf::from("/x/form.crawform");
    assert_eq!(
        documento::file_label(Some(&p), None),
        "",
        "the breadcrumb and the title already name the open file"
    );
    assert_eq!(
        documento::file_label(None, Some("no form opened — tried x")),
        "no form opened — tried x"
    );
    assert_eq!(documento::file_label(None, None), "no form open");
}

#[test]
fn a_arvore_comeca_na_raiz_e_recua_um_nivel_a_cada_descida() {
    let mut d = Document::new("form_main", "Form");
    d.add_child("form_main", Node::new("pnl", "Panel")).unwrap();
    d.add_child("pnl", Node::new("btn", "Button")).unwrap();

    let linhas = documento::linhas_da_arvore(&d);
    let niveis: Vec<(String, usize)> = linhas
        .iter()
        .map(|l: &Linha| (l.id.clone(), l.nivel))
        .collect();
    assert_eq!(
        niveis,
        vec![
            ("form_main".to_string(), 0),
            ("pnl".to_string(), 1),
            ("btn".to_string(), 2),
        ]
    );
}

#[test]
fn a_linha_da_arvore_traz_nome_e_classe() {
    let d = doc_de_teste();
    let linhas = documento::linhas_da_arvore(d.ir());
    let botao = linhas.iter().find(|l| l.id == "btn_ok").unwrap();
    // Both come SEPARATED, which allows rendering them in different colors:
    // name in text color and class in tertiary, with separator `·` in
    // between — as in the owner's screenshot.
    assert_eq!(botao.nome, "OkButton");
    assert_eq!(botao.classe, "Button");
    assert_eq!(
        botao.sufixo_de_classe(),
        format!("{}Button", documento::SEPARADOR),
        "the suffix drawn on screen after the name is the separator with the class"
    );
}

/// A node without a class does not get a dangling separator.
///
/// There are none in the catalog today; this row exists because the root of a
/// corrupted document would land here, and a stray `·` at the end of the name is the
/// kind of clutter nobody wants.
#[test]
fn no_sem_classe_nao_ganha_separador() {
    let l = Linha {
        id: "x".to_string(),
        nivel: 0,
        nome: "x".to_string(),
        classe: String::new(),
    };
    assert_eq!(l.sufixo_de_classe(), "");
}

#[test]
fn os_formularios_candidatos_existem_no_repositorio() {
    // The candidate list is the promise the shell makes when no path is
    // provided: if one vanishes from the repository, the IDE falls back to the second
    // unnoticed. Here that becomes a test failure.
    let raiz = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("the shell lives in crates/crawrad-ide")
        .to_path_buf();
    for c in documento::CANDIDATOS {
        let p = raiz.join(c);
        assert!(p.is_file(), "candidate {} no longer exists", p.display());
        Document::load(&p).unwrap_or_else(|e| panic!("{} fails to open: {e}", p.display()));
    }
}
