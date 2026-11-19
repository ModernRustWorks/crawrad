use super::*;
use crawrad_designer::layout::Anchoring;
use crawrad_ir::{Document, Node, Rect, Value};

fn documento() -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").unwrap();
        raiz.set_rect(Rect::new(0.0, 0.0, 480.0, 320.0));
        raiz.set_name("form_main");
        raiz.set_text("Cadastro");
    }

    let mut lbl = Node::new("lbl1", "Label");
    lbl.set_name("lbl_nome");
    lbl.set_text("Customer name:");
    lbl.set_rect(Rect::new(24.0, 24.0, 120.0, 20.0));
    d.add_child("form_main", lbl).unwrap();

    let mut edt = Node::new("edt1", "Edit");
    edt.set_name("edt_nome");
    edt.set_text("");
    edt.set_rect(Rect::new(24.0, 48.0, 280.0, 24.0));
    edt.events
        .insert("on_change".into(), "edt_nomeChange".into());
    d.add_child("form_main", edt).unwrap();

    let mut btn = Node::new("btn1", "Button");
    btn.set_name("btn_ok");
    btn.set_text("&OK");
    btn.set_rect(Rect::new(24.0, 96.0, 96.0, 32.0));
    btn.events.insert("on_click".into(), "btn_okClick".into());
    d.add_child("form_main", btn).unwrap();

    // A non-visual, which must NOT appear in generated code.
    let mut acl = Node::new("acl1", "ActionList");
    acl.set_name("ActionList1");
    acl.set_rect(Rect::new(400.0, 8.0, 28.0, 28.0));
    d.add_child("form_main", acl).unwrap();

    d.validate().unwrap();
    d
}

fn gerar(d: &Document) -> Vec<Arquivo> {
    GeradorRxp
        .gerar(d, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("generation")
}

/// The body a stub is born with: the indented blank line where the cursor
/// lands, and nothing else.
///
/// It is the classic RAD IDE shape — `begin`, an inner line, `end` —, decided by the
/// owner on 08/09/2026. Until then there was a `// your code here` at this spot, which
/// is why so many tests here replaced THAT phrase: it was the only way to
/// "type in the body". Today the way is [`digitar`], which lands the cursor the way
/// the IDE lands it.
const CORPO_RECEM_NASCIDO: &str = "        \n";

/// Types `codigo` where the cursor would land after a double-click on that
/// handler — which is literally the user's gesture.
///
/// **Passes through [`crate::elo::ponto_de_digitacao`] on purpose.** A test that
/// assembled the position manually would be measuring itself; passing through here,
/// every test that "writes the handler body" also requires the landing to fall
/// inside the body, in the right column. If the landing changes place, dozens of
/// tests change their answer along with it — and that is what we want.
///
/// `codigo` goes **without** indentation and **without** newline: the cursor is already
/// after the eight spaces, and the newline is already in the file.
fn digitar(fonte: &str, handler: &str, codigo: &str) -> String {
    let p = crate::elo::ponto_de_digitacao(fonte, handler)
        .unwrap_or_else(|| panic!("handler `{handler}` is not in file:\n{fonte}"));
    let mut novo = fonte.to_string();
    novo.insert_str(p.byte, codigo);
    novo
}

fn conteudo<'a>(arqs: &'a [Arquivo], caminho: &str) -> &'a str {
    arqs.iter()
        .find(|a| a.caminho == std::path::Path::new(caminho))
        .unwrap_or_else(|| panic!("{caminho} was not generated"))
        .conteudo
        .as_str()
}

#[test]
fn o_plano_e_so_codigo_dentro_de_src() {
    let arqs = gerar(&documento());
    let caminhos: Vec<String> = arqs
        .iter()
        .map(|a| a.caminho.display().to_string().replace('\\', "/"))
        .collect();
    assert_eq!(
        caminhos,
        vec![
            "src/main.rs",
            "src/form_main.designer.rs",
            "src/form_main.rs"
        ]
    );
}

#[test]
fn o_gerador_nao_escreve_o_manifesto_nem_o_gitignore() {
    // This test replaces the one that proved `Cargo.toml` WAS CREATED RIGHT, and
    // proves the guarantee that replaced it: the manifest belongs to the user from
    // the first second, because `cargo init` creates it (ADR 0018). Our
    // template used to say `edition = "2021"` on the day `cargo init` on the same
    // machine said `"2024"`.
    //
    // The check is on DISK, not in the list: writing is what the user
    // experiences, and `escrever()` creates parent directories on its own.
    let raiz = std::env::temp_dir().join("crawrad_codegen_sem_manifesto");
    let _ = std::fs::remove_dir_all(&raiz);

    escrever(&raiz, &gerar(&documento())).unwrap();

    for nosso_nunca in ["Cargo.toml", "Cargo.lock", ".gitignore", ".vscode"] {
        assert!(
            !raiz.join(nosso_nunca).exists(),
            "generator wrote `{nosso_nunca}` — cargo should create this"
        );
    }
    // And what it wrote is all inside `src/`.
    let na_raiz: Vec<String> = std::fs::read_dir(&raiz)
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(na_raiz, vec!["src".to_string()], "{na_raiz:?}");

    let _ = std::fs::remove_dir_all(&raiz);
}

#[test]
fn o_manifesto_do_usuario_atravessa_a_geracao_sem_um_arranhao() {
    // The other side of the same guarantee, and what actually happens in practice:
    // the folder ALREADY has a `Cargo.toml`, created by `cargo init` and with dependencies
    // added by `cargo add`. Generating on top must not touch a single byte of it —
    // neither to "fix" nor to add what might be missing.
    let raiz = std::env::temp_dir().join("crawrad_codegen_manifesto_intacto");
    let _ = std::fs::remove_dir_all(&raiz);
    std::fs::create_dir_all(&raiz).unwrap();

    let manifesto = raiz.join("Cargo.toml");
    let do_cargo_init = "[package]\nname = \"meu_app\"\nedition = \"2024\"\n\n\
                         [dependencies]\neframe = \"0.36\"\n";
    std::fs::write(&manifesto, do_cargo_init).unwrap();

    let r = escrever(&raiz, &gerar(&documento())).unwrap();

    assert_eq!(
        std::fs::read_to_string(&manifesto).unwrap(),
        do_cargo_init,
        "the user's manifest changed"
    );
    // It does not even appear in the report: not preserved, not anything — the generator
    // does not know it exists.
    let citado = |v: &[std::path::PathBuf]| v.iter().any(|c| c.ends_with("Cargo.toml"));
    assert!(!citado(&r.criados) && !citado(&r.preservados) && !citado(&r.reescritos));

    let _ = std::fs::remove_dir_all(&raiz);
}

#[test]
fn o_projeto_gerado_nao_depende_de_nenhum_crate_nosso() {
    // Proof that GENERATION occurred, not a disguised interpreter: what comes
    // out of here knows `eframe` and nothing else.
    //
    // The proof changed location with ADR 0018: it used to be half-manifest ("no
    // `crawrad` in dependencies") and half-code. Now it is all in code, which is
    // where it counts — the manifest belongs to the user, who can put whatever they want there.
    //
    // ONE named exception: the startup-failure reporter. It interprets
    // nothing — it only tells the person why the window never opened — and
    // `crawrad-runtime` is already in every egui form project's manifest
    // (`dependencies.rs`, since the file dialogs). Anything else of
    // ours in the generated code still fails here.
    const STARTUP_REPORTER: &str = "crawrad_runtime::report_startup_failure(";
    let arqs = gerar(&documento());
    for a in &arqs {
        let code = a.conteudo.replace(STARTUP_REPORTER, "");
        assert!(
            !code.contains("crawrad_") && !code.contains("crawrad-"),
            "{} references our crate",
            a.caminho.display()
        );
    }
    assert!(
        conteudo(&arqs, "src/main.rs").contains("eframe::"),
        "the generated project is an eframe app"
    );
}

#[test]
fn o_evento_do_documento_vira_chamada_e_stub() {
    let arqs = gerar(&documento());
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(
        ui.contains("self.btn_okClick();"),
        "click must call handler:\n{ui}"
    );
    assert!(ui.contains("self.edt_nomeChange();"));

    let stub = conteudo(&arqs, "src/form_main.rs");
    assert!(stub.contains("pub fn btn_okClick(&mut self)"));
    assert!(stub.contains("pub fn edt_nomeChange(&mut self)"));
    // The body is born EMPTY, and truly empty — the `begin`/`end` shape of a
    // classic RAD IDE, decided by the owner on 08/09/2026. The assertion checks the entire
    // TEXT of the body, not "does not contain the note": this is what fails if
    // someone puts anything back in there.
    assert!(
        stub.contains("    pub fn btn_okClick(&mut self) {\n        \n    }\n"),
        "stub body must be an indented blank line and nothing else:\n{stub}"
    );
    assert!(
        !stub.contains("your code here"),
        "generator message returned inside the body:\n{stub}"
    );
}

#[test]
fn o_cursor_pousa_na_linha_de_dentro_do_stub_que_o_gerador_acabou_de_escrever() {
    // END TO END OF THE NEW GESTURE, without a screen: double-clicking generates, and the
    // cursor lands on the inner line of the body, already in the column where the user
    // will type. It is the sum of both halves requested by the owner on 08/09/2026 —
    // the empty body like `begin`/`end` of a classic RAD IDE, and indentation together.
    //
    // `elo` already measures landing on its own, with hand-built sources. What THIS
    // test adds is the pair: the column comes from what the GENERATOR actually wrote.
    // Changing stub indentation without updating the landing hits here, and only here.
    let arqs = gerar(&documento());
    let stub = conteudo(&arqs, "src/form_main.rs");
    let p = crate::elo::ponto_de_digitacao(stub, "btn_okClick").expect("the landing");

    assert_eq!(p.coluna, 9, "cursor did not land in body column:\n{stub}");
    assert!(
        stub[p.byte..].starts_with("\n    }"),
        "cursor must land at the END of the indentation with an empty line \
         ahead:\n{:?}",
        &stub[p.byte..p.byte + 12]
    );

    // And what lies to its LEFT is only whitespace — the line is blank. Without
    // this half, a stub born again with a message inside would pass
    // both assertions above.
    let inicio = stub[..p.byte].rfind('\n').expect("landing line") + 1;
    assert_eq!(
        &stub[inicio..p.byte],
        "        ",
        "cursor line is not blank"
    );
}

#[test]
fn o_acelerador_classico_nao_vai_para_o_codigo() {
    // `&OK` is `OK` with underlined O on Windows. In generated code `&` would
    // be a literal `&` on screen.
    let arqs = gerar(&documento());
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(
        ui.contains("text: \"OK\".to_string()"),
        "caption goes clean into state:\n{ui}"
    );
    assert!(!ui.contains("&OK"));
}

#[test]
fn o_nao_visual_vira_estado_mas_nao_desenha_no_aplicativo() {
    // An ActionList is something not visible. In the designer it is an icon; in
    // the generated app it becomes a state field, but draws no visual control.
    let arqs = gerar(&documento());
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(ui.contains("pub ActionList1: ActionList,"), "{ui}");
}

#[test]
fn a_posicao_do_designer_vira_a_posicao_no_aplicativo() {
    let arqs = gerar(&documento());
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(
        ui.contains("egui::vec2(24.0, 96.0)") && ui.contains("egui::vec2(96.0, 32.0)"),
        "button must be placed at x24 y96 96x32:\n{ui}"
    );
}

#[test]
fn o_edit_vira_campo_com_o_texto_do_documento() {
    let mut d = documento();
    d.node_mut("edt1").unwrap().set_text("Fulano");
    let arqs = gerar(&d);
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(ui.contains("pub edt_nome: Edit,"), "{ui}");
    assert!(
        ui.contains("text: \"Fulano\".to_string()"),
        "designer text is initial value:\n{ui}"
    );
    assert!(ui.contains("egui::TextEdit::singleline(&mut self.edt_nome.text)"));
}

#[test]
fn o_componente_invisivel_nasce_escondido_e_pode_aparecer_depois() {
    // Previously invisible items were simply not generated, so the application
    // had no way to ever show them. Now it exists, starts with `visible: false`, and
    // a user line brings it to the screen.
    let mut d = documento();
    d.node_mut("btn1")
        .unwrap()
        .props
        .insert("visible".into(), Value::Bool(false));
    let arqs = gerar(&d);
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(
        init_has(ui, "btn_ok: Button {", &["visible: false", "enabled: true"]),
        "{ui}"
    );
    assert!(ui.contains("if self.btn_ok.visible {"), "{ui}");
    // And the handler still exists: hiding is not deleting.
    assert!(ui.contains("self.btn_okClick();"), "{ui}");
}

#[test]
fn o_componente_desabilitado_sai_desabilitado() {
    let mut d = documento();
    d.node_mut("btn1")
        .unwrap()
        .props
        .insert("enabled".into(), Value::Bool(false));
    let arqs = gerar(&d);
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    // `false` goes into initial state, not into the middle of drawing: the
    // application can re-enable the button later.
    assert!(
        init_has(ui, "btn_ok: Button {", &["visible: true", "enabled: false"]),
        "{ui}"
    );
    assert!(ui.contains("let is_enabled = self.btn_ok.enabled;"), "{ui}");
}

#[test]
fn nome_que_nao_e_identificador_rust_e_recusado_com_motivo() {
    // `Name` becomes field and method. That is why it remains unique and
    // validated after ADR 0008 — the id is opaque, but what becomes code is the name.
    let mut d = documento();
    d.node_mut("btn1").unwrap().set_name("meu botão");
    let e = GeradorRxp
        .gerar(&d, "app", &crawrad_registry::Registry::builtin())
        .unwrap_err();
    assert!(matches!(e, Erro::NomeInvalido(n) if n == "meu botão"));

    let mut d = documento();
    d.node_mut("btn1").unwrap().set_name("type");
    assert!(matches!(
        GeradorRxp
            .gerar(&d, "app", &crawrad_registry::Registry::builtin())
            .unwrap_err(),
        Erro::NomeInvalido(_)
    ));
}

#[test]
fn a_arvore_vira_constante_no_projeto_gerado() {
    let mut d = documento();
    let mut tv = Node::new("tv1", "TreeView");
    tv.set_name("TreeView1");
    tv.set_rect(Rect::new(24.0, 140.0, 200.0, 120.0));
    tv.props.insert(
        "items".into(),
        crawrad_ir::Item::list_to(&[
            crawrad_ir::Item::with_children("Clientes", vec![crawrad_ir::Item::new("Ativos")]),
            crawrad_ir::Item::new("Produtos"),
        ]),
    );
    d.add_child("form_main", tv).unwrap();

    let arqs = gerar(&d);
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(!ui.contains("TREEVIEW1_ITEMS"), "{ui}");
    assert!(
        ui.contains("crawrad_runtime::tree::TreeNode::branch(\"Clientes\", vec![crawrad_runtime::tree::TreeNode::leaf(\"Ativos\")])"),
        "{ui}"
    );
    assert!(
        ui.contains("crawrad_runtime::tree::TreeNode::leaf(\"Produtos\")"),
        "{ui}"
    );
    assert!(
        !ui.contains("pub struct Item {"),
        "Item type is no longer emitted: runtime nodes are used"
    );
}

// -------------------------------------------------------------- real writing

#[test]
fn o_codigo_do_usuario_nunca_e_sobrescrito_e_o_gerado_sempre_e() {
    let raiz = std::env::temp_dir().join("crawrad_codegen_preserva");
    let _ = std::fs::remove_dir_all(&raiz);

    let d = documento();
    let arqs = gerar(&d);
    let r1 = escrever(&raiz, &arqs).unwrap();
    assert_eq!(r1.criados.len(), 3, "main, designer and stub");

    // The user writes their handler.
    let stub = raiz.join("src/form_main.rs");
    let meu = digitar(
        &std::fs::read_to_string(&stub).unwrap(),
        "edt_nomeChange",
        "self.edt_nome = \"salvo!\".into();",
    );
    std::fs::write(&stub, &meu).unwrap();

    // Generate again.
    let r2 = escrever(&raiz, &gerar(&d)).unwrap();

    assert_eq!(
        std::fs::read_to_string(&stub).unwrap(),
        meu,
        "user code must remain intact"
    );
    assert_eq!(r2.preservados.len(), 1, "the stub");
    assert_eq!(
        r2.reescritos.len(),
        2,
        "main.rs and ui.rs are always rebuilt"
    );

    let _ = std::fs::remove_dir_all(&raiz);
}

#[test]
fn o_rs_gerado_sai_sem_bom() {
    // This test was once the OPPOSITE of this, and the old reason was real: UTF-8 without
    // BOM is indistinguishable from Windows-1252 to guessers, and a modern editor
    // guessed — a tab displayed `formulÃ¡rio`. Removing accents from OUR
    // comments didn't help: the user's caption goes into code, and a
    // "Salvar Configuracoes" button puts accents in the literal.
    //
    // What flipped the decision on 01/09/2026 was measuring the other side. `rustc`
    // ignores the BOM, but rust-analyzer does not: it flags `expected an item` at
    // offset 0, and because of that the `#![allow(...)]` on the next line stops being
    // in an INNER attribute position and fails with `expected L_BRACK`. Without
    // `allow`, the `mod formPrincipal;` in `main.rs` flags `non_snake_case`, and
    // so do designer fields. A sample project opened in a modern editor with
    // ~30 false errors and 8 warnings because of three bytes; without BOM, ZERO.
    //
    // The test remained because the decision remained: without it, the BOM returns in the next
    // accent fix and generated code clutters the IDE again.
    let raiz = std::env::temp_dir().join("crawrad_codegen_bom");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&documento())).unwrap();

    for rs in [
        "src/main.rs",
        "src/form_main.designer.rs",
        "src/form_main.rs",
    ] {
        let b = std::fs::read(raiz.join(rs)).unwrap();
        assert_ne!(
            &b[..3],
            &[0xEF, 0xBB, 0xBF],
            "{rs} cannot be emitted with BOM"
        );
        // And remains valid UTF-8: removing the BOM is not changing encoding.
        String::from_utf8(b).unwrap_or_else(|_| panic!("{rs} must be UTF-8"));
    }
    let _ = std::fs::remove_dir_all(&raiz);
}

#[test]
fn o_acento_da_legenda_sai_em_utf8_de_verdade() {
    // The BOM was introduced because of accents, so removing the BOM requires proof
    // that accents remain correct. The user's caption goes ENTIRELY into the literal
    // of the generated code, and `ç`/`õ` must come out as their UTF-8 bytes — not
    // as `Ã§`, which is what you see when someone saves as Windows-1252.
    let mut d = documento();
    // The node id is `btn1`; `btn_ok` is the NAME, which becomes the field.
    d.node_mut("btn1").unwrap().set_text("Salvar Configurações");

    let raiz = std::env::temp_dir().join("crawrad_codegen_acento");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&d)).unwrap();

    let b = std::fs::read(raiz.join("src/form_main.designer.rs")).unwrap();
    let texto = String::from_utf8(b.clone()).expect("file must be UTF-8");
    assert!(
        texto.contains("Salvar Configurações"),
        "caption with accent must reach generated code"
    );
    // `ç` in UTF-8 is `C3 A7`. In Windows-1252 it would be the bare `E7` byte, which
    // is not even valid UTF-8 — and `from_utf8` above would have already failed.
    assert!(
        b.windows(2).any(|j| j == [0xC3, 0xA7]),
        "the `ç` must be in bytes as C3 A7"
    );
    let _ = std::fs::remove_dir_all(&raiz);
}

#[test]
fn evento_novo_faz_o_arquivo_do_usuario_crescer_sem_perder_nada() {
    let raiz = std::env::temp_dir().join("crawrad_codegen_cresce");
    let _ = std::fs::remove_dir_all(&raiz);

    let mut d = documento();
    escrever(&raiz, &gerar(&d)).unwrap();

    let stub = raiz.join("src/form_main.rs");
    let meu = digitar(
        &std::fs::read_to_string(&stub).unwrap(),
        "btn_okClick",
        "println!(\"meu código\");",
    );
    std::fs::write(&stub, &meu).unwrap();

    // The user wires a new event in the inspector.
    d.node_mut("lbl1")
        .unwrap()
        .events
        .insert("on_click".into(), "lbl_nomeClick".into());
    let r = escrever(&raiz, &gerar(&d)).unwrap();

    let depois = std::fs::read_to_string(&stub).unwrap();
    assert!(
        depois.contains("println!(\"meu código\");"),
        "what user wrote is still there"
    );
    assert!(
        depois.contains("pub fn lbl_nomeClick(&mut self)"),
        "and new handler appeared:\n{depois}"
    );
    assert_eq!(
        r.cresceram,
        vec![std::path::PathBuf::from("src/form_main.rs")]
    );

    // Generating again does not duplicate the handler.
    escrever(&raiz, &gerar(&d)).unwrap();
    let de_novo = std::fs::read_to_string(&stub).unwrap();
    assert_eq!(
        de_novo.matches("pub fn lbl_nomeClick").count(),
        1,
        "generating twice must not duplicate"
    );

    let _ = std::fs::remove_dir_all(&raiz);
}

#[test]
fn ligar_dois_eventos_deixa_o_arquivo_do_usuario_com_um_impl_so() {
    let raiz = std::env::temp_dir().join("crawrad_codegen_um_impl");
    let _ = std::fs::remove_dir_all(&raiz);

    // First generation: no events wired yet.
    let mut d = documento();
    d.node_mut("btn1").unwrap().events.clear();
    d.node_mut("edt1").unwrap().events.clear();
    escrever(&raiz, &gerar(&d)).unwrap();

    // The user wires click; then change.
    let mut d = documento();
    d.node_mut("edt1").unwrap().events.clear();
    escrever(&raiz, &gerar(&d)).unwrap();
    escrever(&raiz, &gerar(&documento())).unwrap();

    let txt = std::fs::read_to_string(raiz.join("src/form_main.rs")).unwrap();
    assert_eq!(
        txt.matches("impl FormMain").count(),
        1,
        "each new event opened its own `impl`:\n{txt}"
    );
    assert!(txt.contains("fn btn_okClick("), "{txt}");
    assert!(txt.contains("fn edt_nomeChange("), "{txt}");
    // The marker remains there, for the next event.
    assert!(txt.contains("New event handlers go here"), "{txt}");

    let _ = std::fs::remove_dir_all(&raiz);
}

#[test]
fn sem_a_marca_o_handler_ainda_entra_no_fim_sem_perder_nada() {
    let raiz = std::env::temp_dir().join("crawrad_codegen_sem_marca");
    let _ = std::fs::remove_dir_all(&raiz);

    let mut d = documento();
    d.node_mut("btn1").unwrap().events.clear();
    d.node_mut("edt1").unwrap().events.clear();
    escrever(&raiz, &gerar(&d)).unwrap();

    // The user deletes the marker — it's a comment, they can.
    let stub = raiz.join("src/form_main.rs");
    let meu = std::fs::read_to_string(&stub)
        .unwrap()
        .replace("    // Os handlers dos eventos novos entram aqui.\n", "");
    std::fs::write(&stub, &meu).unwrap();

    escrever(&raiz, &gerar(&documento())).unwrap();
    let txt = std::fs::read_to_string(&stub).unwrap();
    assert!(
        txt.contains("fn btn_okClick("),
        "handler must not be lost:\n{txt}"
    );

    let _ = std::fs::remove_dir_all(&raiz);
}

#[test]
fn align_vira_ancora_no_aplicativo_gerado() {
    // The designer already recorded the rectangle produced by `Align`; what
    // is needed in the application is the resizing behavior when the form changes size,
    // which is precisely a set of anchors.
    for (align, esperado) in [
        (
            "Top",
            "Anchors { left: true, top: true, right: true, bottom: false }",
        ),
        (
            "Bottom",
            "Anchors { left: true, top: false, right: true, bottom: true }",
        ),
        (
            "Left",
            "Anchors { left: true, top: true, right: false, bottom: true }",
        ),
        (
            "Right",
            "Anchors { left: false, top: true, right: true, bottom: true }",
        ),
        (
            "Client",
            "Anchors { left: true, top: true, right: true, bottom: true }",
        ),
    ] {
        let mut d = documento();
        d.node_mut("btn1")
            .unwrap()
            .props
            .insert("align".into(), Value::Enum(align.into()));
        let ui = {
            let arqs = gerar(&d);
            conteudo(&arqs, "src/form_main.designer.rs").to_string()
        };
        let linha = ui
            .lines()
            .find(|l| l.contains("96.0, 32.0"))
            .unwrap_or_else(|| panic!("where is the button?\n{ui}"));
        assert!(
            linha.contains(esperado),
            "align {align} should have become {esperado}:\n{linha}"
        );
    }
}

#[test]
fn as_margens_chegam_no_aplicativo_dentro_do_retangulo_e_nao_como_propriedade() {
    // The golden test of `Margins`/`AlignWithMargins`: **the generator did not change, and
    // this test exists to prove that it didn't need to.**
    //
    // The two properties do not describe an effect the toolkit must
    // produce. They are inputs to a calculation that ends in `Rect`, and that
    // calculation runs once, in `crawrad-designer`, which records the already-inset rectangle
    // in the document. What arrives here is the rectangle — and the target does not even learn
    // that margins exist. It is the same discipline `align` already followed: the generator
    // emits the anchor, never the cascade.
    //
    // What this test guards against is the tempting fix when someone thinks
    // "margins need to be emitted": emitting would apply the inset TWICE,
    // once in the document and once in the application, and the generated control would start
    // smaller than the one in the designer.
    let mut d = documento();
    {
        let n = d.node_mut("btn1").unwrap();
        // The rectangle that `layout::alinhar_filhos` would record for an `alTop`
        // with margin 3 on a form of width 400: inset on all four
        // sides, and it is already what rules.
        n.set_rect(Rect::new(3.0, 3.0, 394.0, 32.0));
        n.props.insert("align".into(), Value::Enum("Top".into()));
        n.props.insert(
            "margins".into(),
            Value::Edges(crawrad_ir::Edges::uniform(3.0)),
        );
        n.props
            .insert("align_with_margins".into(), Value::Bool(true));
    }
    // The document must remain valid: both keys are in the common
    // core, otherwise none of this loads.
    d.validate().expect("margins outside contract");

    let arqs = gerar(&d);
    let ui = conteudo(&arqs, "src/form_main.designer.rs");

    // 1. The emitted rectangle is the INSET one, as recorded by the designer.
    let linha = ui
        .lines()
        .find(|l| l.contains("394.0, 32.0"))
        .unwrap_or_else(|| panic!("inset rectangle did not reach generated code:\n{ui}"));
    assert!(
        linha.contains("3.0, 3.0"),
        "inset corner was lost along the way:\n{linha}"
    );

    // 2. And `Align` still becomes an anchor, without the margin interfering.
    assert!(
        linha.contains("Anchors { left: true, top: true, right: true, bottom: false }"),
        "alTop should still become anchor:\n{linha}"
    );

    // 3. Not a word about margin in the generated code — no property, no
    //    field, no inset. That is the point of the test.
    for proibida in ["margin", "Margin", "align_with_margins", "AlignWithMargins"] {
        assert!(
            !ui.contains(proibida),
            "`{proibida}` appeared in generated code: margin was emitted instead \
             of staying inside rect, and inset will be applied twice"
        );
    }
}

#[test]
fn align_none_deixa_as_ancoras_do_usuario_valerem() {
    let mut d = documento();
    {
        let n = d.node_mut("btn1").unwrap();
        n.props.insert("align".into(), Value::Enum("None".into()));
        n.props.insert(
            "anchors".into(),
            Value::Map(
                [
                    ("left".to_string(), Value::Bool(false)),
                    ("right".to_string(), Value::Bool(true)),
                ]
                .into_iter()
                .collect(),
            ),
        );
    }
    let arqs = gerar(&d);
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(
        ui.contains("Anchors { left: false, top: true, right: true, bottom: false }"),
        "{ui}"
    );
}

#[test]
fn cada_componente_leva_o_proprio_estado_para_o_aplicativo() {
    // The usual question: how does the application hide a button? Previously there was
    // no answer — the drawing emitted fixed designer values. Now
    // each component becomes a field with its state, and common fields come
    // flattened into each type: `TControl` without `.base.` in the way.
    let arqs = gerar(&documento());
    let ui = conteudo(&arqs, "src/form_main.designer.rs");

    for tipo in [
        "pub struct Label {",
        "pub struct Button {",
        "pub struct Edit {",
    ] {
        assert!(ui.contains(tipo), "missing {tipo}:\n{ui}");
    }
    // Only used types are included: the form has no tree.
    assert!(!ui.contains("pub struct TreeView {"), "{ui}");

    assert!(ui.contains("pub btn_ok: Button,"), "{ui}");
    assert!(ui.contains("pub lbl_nome: Label,"), "{ui}");
    // Common fields exist in all of them, flattened.
    assert_eq!(
        ui.matches("pub visible: bool,").count(),
        3,
        "one `visible` per used type:\n{ui}"
    );
}

#[test]
fn a_arvore_leva_a_selecao_como_estado() {
    let mut d = documento();
    let mut tv = Node::new("tv1", "TreeView");
    tv.set_name("tree");
    tv.set_rect(Rect::new(8.0, 140.0, 200.0, 120.0));
    d.add_child("form_main", tv).unwrap();
    let arqs = gerar(&d);
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(ui.contains("pub struct TreeView {"), "{ui}");
    assert!(ui.contains("pub selection: Vec<usize>,"), "{ui}");
    // the tree proposes the pick; the form writes it (OnChanging may veto)
    assert!(ui.contains("&self.tree.selection"), "{ui}");
    assert!(ui.contains("self.tree.selection = path;"), "{ui}");
}

#[test]
fn nome_so_de_sublinhado_nao_chega_a_tocar_o_arquivo_do_usuario() {
    // The defect, measured on 01/09/2026. `"___".trim()` is not empty, so it
    // passed the empty-name guard; and `___` is a valid Rust
    // identifier, so it passed `identificador`. But
    // `tipo_do_form("___")` is `""` — `split('_')` leaves no parts at all — and
    // then the rename migrator instructed replacing the word `FormMain` with *nothing*
    // inside the hand-written file:
    //
    //     fn _ajuda(_f: &FormMain) {}   ->   fn _ajuda(_f: &) {}
    //     // meu FormMain               ->   // meu
    //
    // Two words vanish and the user's project stops compiling. And the generated
    // code itself was already broken before that, with `impl  {`.
    //
    // This test is proof by mutation: remove `nome_tem_alfanumerico` from
    // `identificador` and it turns red displaying the mutilated file.
    let raiz = std::env::temp_dir().join("crawrad_codegen_sublinhado");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&documento())).unwrap();

    // The user writes by hand, citing the form type.
    let meu = raiz.join("src/form_main.rs");
    let antes = std::fs::read_to_string(&meu).unwrap()
        + "
fn _ajuda(_f: &FormMain) {}
// meu FormMain
";
    std::fs::write(&meu, &antes).unwrap();

    // And then renames the form to `___`, and runs the full path —
    // generate and write —, which is what the shell does.
    let mut d = documento();
    d.node_mut("form_main").unwrap().set_name("___");
    let r = GeradorRxp
        .gerar(&d, "app", &crawrad_registry::Registry::builtin())
        .and_then(|arqs| escrever(&raiz, &arqs));

    // Wherever the hand-written code ended up — under mutation the
    // migrator moves it to `src/___.rs` —, it must be byte-for-byte identical.
    // This assert comes **before** the error assert on purpose: it is the one that turns
    // red under mutation, and what it prints is the mutilation.
    let escrito_a_mao = std::fs::read_dir(raiz.join("src"))
        .unwrap()
        .flatten()
        .map(|e| std::fs::read_to_string(e.path()).unwrap_or_default())
        .find(|t| t.contains("_ajuda"))
        .unwrap_or_default();
    assert_eq!(
        escrito_a_mao, antes,
        "hand-written file should remain intact"
    );

    // And the error is about the name, inside `gerar()` — before `escrever` exists.
    let e = r.unwrap_err();
    assert!(
        matches!(&e, Erro::NomeInvalido(n) if n == "___"),
        "got {e:?}"
    );
    assert!(
        !raiz.join("src/___.rs").exists(),
        "the new file was not even created"
    );
}

#[test]
fn os_vizinhos_do_nome_so_de_sublinhado() {
    // Which ones `tipo_do_form` also breaks, and where each is caught. The difference
    // matters: what `identificador` rejects never reaches disk, because the
    // rejection occurs inside `gerar()`. `___` was the only one that slipped through.
    for n in ["_", "__", "___", "____"] {
        assert_eq!(tipo_do_form(n), "", "{n:?} derives to nothing");
        assert!(
            matches!(identificador(n), Err(Erro::NomeInvalido(_))),
            "{n:?} should have been rejected"
        );
    }

    // These `tipo_do_form` also spoils — `1abc` and `  ` derive into something
    // that is not a type, `fn` derives into `Fn` but the module `mod fn;` does not compile
    // —, and all were already rejected before this fix.
    for n in ["1abc", "fn", "  ", "meu botão", ""] {
        assert!(
            matches!(identificador(n), Err(Erro::NomeInvalido(_))),
            "{n:?} should have been rejected"
        );
    }

    // And the good case stays good.
    assert_eq!(tipo_do_form("form_main"), "FormMain");
    assert_eq!(identificador("form_main").unwrap(), "form_main");
    assert_eq!(identificador("_privado").unwrap(), "_privado");
}

#[test]
fn trocar_palavra_com_destino_vazio_nao_apaga() {
    // The last gate, and the one that caused the damage. Today it is unreachable —
    // `identificador` rejects beforehand —, but destructive code answers to its own
    // contract: an empty target is never anyone's intent.
    let texto = "fn _ajuda(_f: &FormMain) {}
// meu FormMain
";
    assert_eq!(trocar_palavra(texto, "FormMain", ""), texto);
    assert_eq!(trocar_palavra(texto, "", "Outro"), texto);
    // And the real replacement still replaces.
    assert_eq!(
        trocar_palavra(texto, "FormMain", "Cadastro"),
        "fn _ajuda(_f: &Cadastro) {}
// meu Cadastro
"
    );
}

#[test]
fn renomear_o_formulario_leva_o_codigo_do_usuario_junto() {
    // Without this, renaming in the inspector is a silent loss: the generator writes
    // an empty stub with the new name, `main.rs` points to it, and
    // hand-written handlers are orphaned on disk without any warning.
    let raiz = std::env::temp_dir().join("crawrad_codegen_rename");
    let _ = std::fs::remove_dir_all(&raiz);

    escrever(&raiz, &gerar(&documento())).unwrap();

    // The user writes the handler body.
    let stub = raiz.join("src/form_main.rs");
    let meu = digitar(
        &std::fs::read_to_string(&stub).unwrap(),
        "edt_nomeChange",
        "self.edt_nome.text = \"oi\".into();",
    );
    std::fs::write(&stub, &meu).unwrap();

    // And then renames the form.
    let mut d = documento();
    d.node_mut("form_main").unwrap().set_name("cadastro");
    let r = escrever(&raiz, &gerar(&d)).unwrap();

    assert_eq!(
        r.renomeados,
        vec![(
            std::path::PathBuf::from("src").join("form_main.rs"),
            std::path::PathBuf::from("src").join("cadastro.rs"),
        )]
    );
    assert!(!stub.exists(), "old-name file was moved, not copied");

    let agora = std::fs::read_to_string(raiz.join("src/cadastro.rs")).unwrap();
    assert!(
        agora.contains("self.edt_nome.text = \"oi\".into();"),
        "hand-written code should have survived:\n{agora}"
    );
    // The MODULE and type renamed together, otherwise none of this compiles.
    // `use` is a glob — the type is proven by `impl` right below it, and the
    // two assertions together cover the two replacements the migration performs.
    assert!(
        agora.contains("use crate::cadastro_designer::*;"),
        "{agora}"
    );
    assert!(agora.contains("impl Cadastro {"), "{agora}");
    assert!(!agora.contains("FormMain"), "{agora}");

    // The old designer disappears; the new one stays.
    assert!(raiz.join("src/cadastro.designer.rs").exists());
    assert!(!raiz.join("src/form_main.designer.rs").exists());
    assert_eq!(
        r.removidos,
        vec![std::path::PathBuf::from("src").join("form_main.designer.rs")]
    );

    let _ = std::fs::remove_dir_all(&raiz);
}

#[test]
fn com_dois_candidatos_o_rename_nao_adivinha() {
    // Two orphaned designers: impossible to know whose code it was. Not
    // touching is better than touching wrong — and the orphaned designer still disappears.
    let raiz = std::env::temp_dir().join("crawrad_codegen_ambiguo");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&documento())).unwrap();

    let src = raiz.join("src");
    std::fs::write(src.join("outro.designer.rs"), "// sobra\n").unwrap();
    std::fs::write(src.join("outro.rs"), "// meu\n").unwrap();

    let mut d = documento();
    d.node_mut("form_main").unwrap().set_name("cadastro");
    let r = escrever(&raiz, &gerar(&d)).unwrap();

    assert!(r.renomeados.is_empty(), "{:?}", r.renomeados);
    assert!(src.join("form_main.rs").exists());
    assert!(src.join("outro.rs").exists());
    assert_eq!(r.removidos.len(), 2, "both orphaned designers disappear");

    let _ = std::fs::remove_dir_all(&raiz);
}

#[test]
fn o_cabecalho_do_stub_so_promete_o_que_o_rename_cumpre() {
    // The user file header is the first thing they read, and for
    // two iterations it LIED: it said `never overwrites it`, then
    // `never changes a line you wrote` and `updating the `use` above`.
    // `migrar_rename` disproves all three — it calls `trocar_palavra` on the
    // ENTIRE text, so the type name changes inside user-written comments
    // as well.
    //
    // It slipped through twice because no test checked the text. This one guards
    // both sides: requires that the old promise doesn't return, and requires that the new
    // one is there — and then RUNS the rename, so the header describes
    // what actually happens rather than decoration.
    let arqs = gerar(&documento());
    let stub = conteudo(&arqs, "src/form_main.rs");
    // Flattened into a single line on purpose: in an early version of this test
    // the phrase from round 2 ESCAPED, because "updating the `use` above" was
    // broken across two lines of the header and `contains` couldn't see it. Promises
    // aren't measured line by line; they're measured by what a person reads.
    let cabecalho = stub
        .lines()
        .take_while(|l| l.starts_with("//!"))
        .map(|l| l.trim_start_matches("//!").trim())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(!cabecalho.is_empty(), "stub must have header");

    // 1. What `migrar_rename` disproves, and cannot return.
    for mentira in [
        "never overwrites it",
        "never touches",
        "never rewrites",
        "never changes a line you wrote",
        "nothing you wrote is touched",
        "updating the `use` above",
    ] {
        assert!(
            !cabecalho.contains(mentira),
            "header promises `{mentira}`, but rename contradicts it:\n{cabecalho}"
        );
    }

    // 2. What it MUST state, because it happens.
    for verdade in ["marker below", "type name", "comments"] {
        assert!(
            cabecalho.contains(verdade),
            "header does not mention `{verdade}`:\n{cabecalho}"
        );
    }

    // 3. And behavior: a user comment referencing the type emerges from the
    //    rename with the NEW name. This is the exact case that struck down the two
    //    previous phrases.
    let raiz = std::env::temp_dir().join("crawrad_codegen_cabecalho");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&documento())).unwrap();

    let dele = raiz.join("src/form_main.rs");
    let meu = std::fs::read_to_string(&dele).unwrap()
        + "\n// meu comentário: o FormMain é meu\nfn _ajuda(_f: &FormMain) {}\n";
    std::fs::write(&dele, &meu).unwrap();

    let mut d = documento();
    d.node_mut("form_main").unwrap().set_name("cadastro");
    escrever(&raiz, &gerar(&d)).unwrap();

    let depois = std::fs::read_to_string(raiz.join("src/cadastro.rs")).unwrap();
    assert!(
        depois.contains("// meu comentário: o Cadastro é meu"),
        "rename changes type INSIDE comment, and that is what header must state:\n{depois}"
    );
    assert!(
        depois.contains("fn _ajuda(_f: &Cadastro)"),
        "and in code as well:\n{depois}"
    );

    let _ = std::fs::remove_dir_all(&raiz);
}

#[test]
fn a_troca_de_nome_do_tipo_e_por_palavra_inteira() {
    // The user's `FormMainAntigo` must not become `CadastroAntigo`.
    let raiz = std::env::temp_dir().join("crawrad_codegen_palavra");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&documento())).unwrap();

    let stub = raiz.join("src/form_main.rs");
    let meu = std::fs::read_to_string(&stub).unwrap()
        + "\nstruct FormMainAntigo;\nfn _x(_: FormMainAntigo) {}\n";
    std::fs::write(&stub, &meu).unwrap();

    let mut d = documento();
    d.node_mut("form_main").unwrap().set_name("cadastro");
    escrever(&raiz, &gerar(&d)).unwrap();

    let agora = std::fs::read_to_string(raiz.join("src/cadastro.rs")).unwrap();
    assert!(agora.contains("struct FormMainAntigo;"), "{agora}");
    assert!(agora.contains("impl Cadastro {"), "{agora}");

    let _ = std::fs::remove_dir_all(&raiz);
}

// `set_readonly(false)` restores the directory to normal state for the
// `remove_dir_all` at the end. The lint warns that on Unix this leaves the file
// writable by everyone — here it is a file the test itself just created
// in a temporary directory, and without this the test would leave locked debris on disk.
#[test]
#[cfg(windows)]
fn falha_depois_do_temporario_escrito_nao_deixa_lixo_na_pasta_do_dono() {
    // Sibling of the failed-write test, and the case it did NOT cover. There
    // writing fails BEFORE creating anything, so there is no temporary
    // left over. Here the temporary is written successfully and `rename` is what
    // fails — and only on this branch does a real `.crawrad-tmp` exist on disk
    // for cleanup to remove.
    //
    // It is not corruption: `.crawrad-tmp` is not `.rs`, cargo ignores it and
    // `varrer_orfaos` does not touch it. But it is debris in the user's `src/` with the NAME
    // of their file, and whoever finds it thinks they lost code.
    //
    // The failure is forced by making the owner's file read-only: on Windows
    // `rename` over such a file returns `Access is denied`
    // AFTER the temporary is already written. This is what can be forced without an I/O
    // mock — a failure in the MIDDLE of `write` (full disk) cannot, which is why
    // both halves share a single cleanup routine instead of two similar ones.
    let raiz = std::env::temp_dir().join("crawrad_codegen_lixo");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&documento())).unwrap();

    let alvo = raiz.join("src/form_main.rs");
    let texto = std::fs::read(&alvo).unwrap();
    let mut com_bom = vec![0xEF, 0xBB, 0xBF];
    com_bom.extend_from_slice(&texto);
    std::fs::write(&alvo, &com_bom).unwrap();

    let mut permissoes = std::fs::metadata(&alvo).unwrap().permissions();
    permissoes.set_readonly(true);
    std::fs::set_permissions(&alvo, permissoes).unwrap();

    let r = escrever(&raiz, &gerar(&documento()));

    // Restore permissions BEFORE any assert, otherwise a failing test
    // leaves the directory unable to be removed.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissoes = std::fs::metadata(&alvo).unwrap().permissions();
        permissoes.set_mode(0o644);
        std::fs::set_permissions(&alvo, permissoes).unwrap();
    }
    #[cfg(windows)]
    {
        let mut permissoes = std::fs::metadata(&alvo).unwrap().permissions();
        let somente_leitura = false;
        permissoes.set_readonly(somente_leitura);
        std::fs::set_permissions(&alvo, permissoes).unwrap();
    }

    let temporario = raiz.join("src/form_main.rs.crawrad-tmp");
    let sobrou = temporario.exists();
    let conteudo = std::fs::read(&alvo).unwrap();
    let _ = std::fs::remove_dir_all(&raiz);

    assert!(r.is_err(), "rename should have failed: {r:?}");
    assert!(
        !sobrou,
        "a `.crawrad-tmp` was left in the user's folder: {}",
        temporario.display()
    );
    assert_eq!(
        conteudo, com_bom,
        "and owner's file remains intact, exactly as it was"
    );
}

/// The same promise on Unix, forced another way: there a read-only file does
/// NOT stop the `rename` (the folder's permission decides — measured in a
/// Linux container on 01/10/2026). A non-empty FOLDER in the owner's file's
/// place does: the temporary is written, the `rename` over it fails.
#[test]
#[cfg(unix)]
fn falha_no_rename_nao_deixa_lixo_tambem_no_unix() {
    let raiz = std::env::temp_dir().join("crawrad_codegen_lixo_unix");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&documento())).unwrap();
    let alvo = raiz.join("src/form_main.designer.rs");
    std::fs::remove_file(&alvo).unwrap();
    std::fs::create_dir_all(alvo.join("ocupado")).unwrap();

    let r = escrever(&raiz, &gerar(&documento()));
    let temporario = raiz.join("src/form_main.designer.rs.crawrad-tmp");
    let sobrou = temporario.exists();
    let _ = std::fs::remove_dir_all(&raiz);

    assert!(r.is_err(), "rename should have failed: {r:?}");
    assert!(
        !sobrou,
        "a `.crawrad-tmp` was left behind: {}",
        temporario.display()
    );
}

#[test]
fn temporario_esquecido_por_uma_execucao_morta_some_na_proxima() {
    // The case NO test can force: the process dies mid-`write`
    // and leaves a `.crawrad-tmp` half-done. The `gravar_bytes` cleanup does not run
    // because the process is gone — there is no possible fix for that.
    //
    // What can be guaranteed is that the debris does not stay FOREVER: the next
    // `Generate` writes over the same path and `rename` carries it away.
    // Applies to every file passing through `gravar_bytes`, which is all of them.
    let raiz = std::env::temp_dir().join("crawrad_codegen_lixo_velho");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&documento())).unwrap();

    // The corpse from the dead run: half a file, with their file name.
    let esquecido = raiz.join("src/main.rs.crawrad-tmp");
    std::fs::write(&esquecido, b"//! metade de um arquivo, de uma exec").unwrap();
    assert!(esquecido.exists(), "scenario must start with debris there");

    escrever(&raiz, &gerar(&documento())).unwrap();

    assert!(
        !esquecido.exists(),
        "forgotten `.crawrad-tmp` should be removed in next generation: {}",
        esquecido.display()
    );

    let _ = std::fs::remove_dir_all(&raiz);
}

#[test]
fn escrita_que_falha_nao_trunca_o_arquivo_do_dono() {
    // The `desbomar` exception opened the user's file for WRITING, and
    // `fs::write` opens with truncate: a failure midway would leave
    // `form_main.rs` — with hand-written handlers — truncated or empty.
    //
    // That is why `gravar_bytes` writes to a nearby temporary and executes `rename`.
    // This test forces failure the way it can be forced without an I/O mock:
    // occupies the temporary path with a FOLDER, and then `fs::write` to it
    // fails before anything in the user's file has been touched.
    //
    // That is what `rename` guarantees and `write` did not guarantee: anyone inspecting
    // the path sees the ENTIRE old file or the entire new file, never half.
    let raiz = std::env::temp_dir().join("crawrad_codegen_falha");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&documento())).unwrap();

    // The owner's file, with BOM (old project) and code inside.
    let alvo = raiz.join("src/form_main.rs");
    let miolo = concat!(
        "//! **This file is yours.**\n",
        "#![allow(non_snake_case)]\n",
        "\n",
        "use crate::form_main_designer::FormMain;\n",
        "\n",
        "impl FormMain {\n",
        "    pub fn btn_okClick(&mut self) {\n",
        "        // uma tarde inteira de trabalho do dono mora aqui\n",
        "    }\n",
        "\n",
        "    pub fn edt_nomeChange(&mut self) {}\n",
        "\n",
        "    // New event handlers go here.\n",
        "}\n",
    );
    let mut com_bom = vec![0xEF, 0xBB, 0xBF];
    com_bom.extend_from_slice(miolo.as_bytes());
    std::fs::write(&alvo, &com_bom).unwrap();

    // The temporary path, occupied by a folder: writing to it fails.
    let temporario = raiz.join("src/form_main.rs.crawrad-tmp");
    std::fs::create_dir_all(&temporario).unwrap();

    let r = escrever(&raiz, &gerar(&documento()));
    assert!(r.is_err(), "write should have failed: {r:?}");

    assert_eq!(
        std::fs::read(&alvo).unwrap(),
        com_bom,
        "owner's file must be INTACT, exactly as it was"
    );

    let _ = std::fs::remove_dir_all(&raiz);
}

#[test]
fn o_bom_sai_e_mais_nada_muda_no_arquivo_do_usuario() {
    // The rule says the user file only GROWS. Removing the BOM from a project
    // generated before 01/09/2026 is the ONLY exception, and this test is its
    // boundary: the three bytes leave, and the rest comes out BYTE FOR BYTE equal to what went in.
    //
    // It fails if someone one day takes advantage of this rewriting to reformat,
    // normalize line endings, reorder `use`, or fix indentation — and that is
    // why it exists: the second exception is always easier than the first.
    let raiz = std::env::temp_dir().join("crawrad_codegen_desbom");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&documento())).unwrap();

    // The user file as an OLD project would have it: BOM in front and
    // real code inside. Intentionally brings what a well-intentioned
    // "fix" would normalize: a CRLF line, a TAB instead of
    // spaces, crooked indentation, accents, and trailing spaces.
    let miolo = concat!(
        "//! **This file is yours.** CrawRAD never overwrites it.\r\n",
        "#![allow(non_snake_case)]\n",
        "\n",
        "use crate::form_main_designer::FormMain;\n",
        "\n",
        "impl FormMain {\n",
        "    /// Grava as configurações do usuário.   \n",
        "\tpub fn btn_okClick(&mut self) {\n",
        "self.edt_nome.text = \"Olá, coração!\".into();\n",
        "\t\t}\n",
        "\n",
        "    pub fn edt_nomeChange(&mut self) {}\n",
        "\n",
        "    // New event handlers go here.\n",
        "}\n",
    );
    let alvo = raiz.join("src/form_main.rs");
    let mut com_bom = vec![0xEF, 0xBB, 0xBF];
    com_bom.extend_from_slice(miolo.as_bytes());
    std::fs::write(&alvo, &com_bom).unwrap();

    // Regenerating the SAME document: both handlers are already there, nothing to
    // add — the file can only be PRESERVED.
    let r = escrever(&raiz, &gerar(&documento())).unwrap();
    assert!(
        r.preservados.contains(&PathBuf::from("src/form_main.rs")),
        "user's file remains theirs: {r:?}"
    );

    let depois = std::fs::read(&alvo).unwrap();
    assert_ne!(
        &depois[..3],
        &[0xEF, 0xBB, 0xBF],
        "BOM should have been removed"
    );
    assert_eq!(
        depois,
        com_bom[3..],
        "only the three bytes should leave — the rest is byte for byte the user's"
    );

    let _ = std::fs::remove_dir_all(&raiz);
}

#[test]
fn arquivo_reescrito_perde_o_bom_antigo() {
    // This test was also the opposite: it checked for ONE BOM, not two.
    // `read_to_string` returns the BOM as a character, and writing back wrote
    // another — rustc halted at `unknown start of token`. Affected form
    // renames, and would affect any file reread by the generator.
    //
    // Since 01/09/2026 the BOM was dropped, and this test became the MIGRATION test
    // via the rename path: projects generated before that day have the BOM on disk,
    // and the file must arrive on the other side without it.
    //
    // What strips it is `desbomar()`, not `gravar()`'s `strip_prefix` — that
    // is defense-in-depth and today is not exercised by any test (mutating
    // it leaves the suite green, measured). If one day `desbomar` leaves
    // `escrever`, here is where it shows: without this test the old file keeps
    // cluttering rust-analyzer forever.
    let raiz = std::env::temp_dir().join("crawrad_codegen_bom2");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&documento())).unwrap();

    // The OLD project on disk: the user file with BOM in front.
    let velho = raiz.join("src/form_main.rs");
    let texto = std::fs::read(&velho).unwrap();
    let mut com_bom = vec![0xEF, 0xBB, 0xBF];
    com_bom.extend_from_slice(&texto);
    std::fs::write(&velho, com_bom).unwrap();

    let mut d = documento();
    d.node_mut("form_main").unwrap().set_name("cadastro");
    escrever(&raiz, &gerar(&d)).unwrap();

    let b = std::fs::read(raiz.join("src/cadastro.rs")).unwrap();
    assert_ne!(
        &b[..3],
        &[0xEF, 0xBB, 0xBF],
        "rename must REMOVE BOM from old file, not carry it forward"
    );
    // And the user code survived the rename entirely.
    let texto = String::from_utf8(b).expect("UTF-8");
    assert!(texto.starts_with("//!"), "{texto}");
    assert!(texto.contains("impl Cadastro"), "{texto}");

    let _ = std::fs::remove_dir_all(&raiz);
}

/// The registry verifies the GENERATOR.
///
/// One step beyond the adapters test: a class that the designer
/// renders and the generator does not emit becomes a `TODO` in user code, which
/// they only discover when compiling the generated project.
///
/// The gauge is the catalogue's `emits_code` flag, not `visual`. While
/// it was `visual`, all non-visual components were skipped — and `Timer`, which does
/// not appear on screen and **needs** to be emitted, would have entered the registry without
/// anyone requiring its emission.
#[test]
fn toda_classe_que_gera_codigo_e_emitida() {
    let reg = crawrad_registry::Registry::builtin();
    for d in reg.all() {
        if !d.emits_code || d.class == "Form" || !d.supports("egui") {
            continue;
        }
        assert!(
            GeradorRxp.classes().contains(&d.class),
            "`{}` emits code and generator does not emit it",
            d.class
        );
    }
}

/// And the inverse: what the catalogue declares as data must not become a field.
#[test]
fn o_que_e_dado_no_catalogo_nao_vira_codigo() {
    let reg = crawrad_registry::Registry::builtin();
    for d in reg.all() {
        // This test checks the egui gerador (rxp); skip components that
        // don't support egui — they have their own target-specific checks.
        if !d.supports("egui") {
            continue;
        }
        assert_eq!(
            crate::rxp::emitido(d.class),
            d.emits_code,
            "`{}`: catalogue and generator disagree on code generation",
            d.class
        );
        // And the second question, which is different: what draws gets a rectangle.
        assert_eq!(
            crate::rxp::e_visual(d.class),
            d.visual,
            "`{}`: catalogue and generator disagree on drawing",
            d.class
        );
    }
}

#[test]
fn nao_emitimos_classe_que_o_registro_nao_conhece() {
    let reg = crawrad_registry::Registry::builtin();
    for c in GeradorRxp.classes() {
        assert!(reg.get(c).is_some(), "`{c}` is not in registry");
    }
}

// **The events that the Inspector offers and the generated application never triggers.**
//
// This list is a DEBT, not a decision: the user wires the `OnClick` of
// a `ListBox` in the Object Inspector, the generator writes the stub in their file,
// and nothing in `show()` calls that stub. The user's code stays there, correct,
// and never runs.
//
// **It once had seven lines.** The two that were removed were from `Form`, and were
// the worst case: the root is nobody's child, it escaped the traversal.

/// The designer generated for a form with ONE component of the class, with the
/// event wired to a handler of known name.
fn designer_com_evento(classe: &str, evento: &str) -> String {
    let arqs = gerado_com_evento(classe, evento);
    arqs.iter()
        .find(|a| a.caminho.to_string_lossy().contains(SUFIXO_GERADO))
        .expect("the designer")
        .conteudo
        .clone()
}

/// The USER file generated for the same form — where the stub is born.
fn stub_com_evento(classe: &str, evento: &str) -> String {
    let arqs = gerado_com_evento(classe, evento);
    arqs.iter()
        .find(|a| a.dono == Dono::Usuario)
        .expect("the user file")
        .conteudo
        .clone()
}

/// The entire project for a form with ONE component of the class, with the
/// event wired to a handler of known name. With `classe == "Form"`, the ROOT
/// receives the event.
fn gerado_com_evento(classe: &str, evento: &str) -> Vec<Arquivo> {
    let reg = crawrad_registry::Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").unwrap();
        raiz.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        raiz.set_name("form_main");
        raiz.set_text("Cadastro");
    }
    if classe == "Form" {
        let raiz = d.node_mut("form_main").unwrap();
        raiz.events.insert(evento.into(), "handler_do_teste".into());
    } else {
        let (w, h) = reg.get(classe).unwrap().default_size;
        let mut n = reg
            .instantiate(classe, "alvo1", Rect::new(8.0, 8.0, w, h))
            .unwrap();
        n.set_name("alvo1");
        n.events.insert(evento.into(), "handler_do_teste".into());
        d.add_child("form_main", n).unwrap();
    }
    d.validate().unwrap();
    GeradorRxp
        .gerar(&d, "app", &crawrad_registry::Registry::builtin())
        .unwrap()
}

/// **Every event in the catalogue generates a clean and valid stub.**
#[test]
fn todo_evento_do_catalogo_gera_stub_valido() {
    let reg = crawrad_registry::Registry::builtin();
    for desc in reg.all() {
        if !desc.emits_code && desc.class != "Form" {
            continue;
        }
        for e in &desc.events {
            let stub = stub_com_evento(desc.class, e.name);
            assert!(
                stub.contains(&format!(
                    "/// `{}.{}`",
                    if desc.class == "Form" {
                        "form_main"
                    } else {
                        "alvo1"
                    },
                    e.name
                )),
                "must generate event marker for {}.{}",
                desc.class,
                e.name
            );
            assert!(
                !stub.contains("TODO(CrawRAD)"),
                "stub of {}.{} must not contain TODO comments",
                desc.class,
                e.name
            );
        }
    }
}

// ------------------------------------------------- FORM events

#[test]
fn o_on_create_do_formulario_vira_stub_e_chamada() {
    // The defect: the root is nobody's child, it escaped the traversal, and
    // `OnCreate` wired in the Inspector became NOTHING — neither stub nor call.
    let ui = designer_com_evento("Form", "on_create");
    assert!(ui.contains("pub fn new() -> Self {"), "{ui}");
    assert!(ui.contains("form.handler_do_teste();"), "{ui}");

    let stub = stub_com_evento("Form", "on_create");
    assert!(
        stub.contains("pub fn handler_do_teste(&mut self)"),
        "{stub}"
    );
    assert!(stub.contains("/// `form_main.on_create`"), "{stub}");
}

#[test]
fn o_on_create_roda_depois_dos_valores_do_designer() {
    // **The order is the contract.** Inverted, the user writes
    // `self.edt.text = …` in `OnCreate` and sees the value overwritten by
    // `.crawform` — silently. Here this is measured in text; the real
    // effect is measured in `gate_fase3`, which constructs the form and checks the
    // field.
    let ui = designer_com_evento("Form", "on_create");
    let padrao = ui
        .find("let mut form = Self::default();")
        .expect("the Default");
    let chamada = ui.find("form.handler_do_teste();").expect("the OnCreate");
    assert!(
        padrao < chamada,
        "OnCreate must run AFTER designer values:
{ui}"
    );
}

#[test]
fn sem_on_create_o_new_nao_pede_mut() {
    // `let mut form` without anyone mutating it is `unused_mut`, and this
    // file cannot be fixed by the user.
    let arqs = gerar(&documento());
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(
        ui.contains(
            "pub fn new() -> Self {
        Self::default()
"
        ),
        "{ui}"
    );
    assert!(!ui.contains("let mut form ="), "{ui}");
}

#[test]
fn o_main_constroi_o_formulario_pelo_new() {
    // Without this `OnCreate` would exist and the application would not call it:
    // `main.rs` built using `default()`.
    let arqs = gerar(&documento());
    let main = conteudo(&arqs, "src/main.rs");
    assert!(main.contains("FormMain::new();"), "{main}");
    assert!(!main.contains("FormMain::default()"), "{main}");
}

#[test]
fn o_on_close_avisa_e_nao_pergunta() {
    // `OnClose` is a NOTIFICATION (ADR 0024): fires in the frame where closing
    // was requested, and nothing cancels it. If one day there is a veto, it comes
    // with a two-phase design — not with a `CancelClose` hidden here.
    let ui = designer_com_evento("Form", "on_close");
    assert!(
        ui.contains("if ui.input(|i| i.viewport().close_requested()) {"),
        "{ui}"
    );
    assert!(ui.contains("self.handler_do_teste();"), "{ui}");
    assert!(
        !ui.contains("CancelClose"),
        "veto is not supported in this version:
{ui}"
    );

    let stub = stub_com_evento("Form", "on_close");
    assert!(
        stub.contains("pub fn handler_do_teste(&mut self)"),
        "{stub}"
    );
}

#[test]
fn sem_on_close_nada_de_fechamento_entra_no_show() {
    let arqs = gerar(&documento());
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(!ui.contains("close_requested"), "{ui}");
}

#[test]
fn o_evento_que_o_alvo_dispara_nao_leva_marca_nenhuma() {
    // The other side: markings in every stub would be noise, and nobody reads noise.
    let arqs = gerar(&documento());
    let stub = conteudo(&arqs, "src/form_main.rs");
    assert!(stub.contains("pub fn btn_okClick(&mut self)"), "{stub}");
    assert!(!stub.contains("TODO(CrawRAD)"), "{stub}");
}

/// A form with the combo box, for its tests.
fn com_combo(indice: i64) -> Document {
    let mut d = documento();
    let mut c = Node::new("cbo1", "ComboBox");
    c.set_name("cbo_uf");
    c.set_rect(Rect::new(8.0, 200.0, 145.0, 24.0));
    c.props.insert(
        "items".into(),
        crawrad_ir::Item::list_to(&[crawrad_ir::Item::new("SP"), crawrad_ir::Item::new("RJ")]),
    );
    c.props.insert("item_index".into(), Value::Int(indice));
    c.events.insert("on_change".into(), "cbo_ufChange".into());
    d.add_child("form_main", c).unwrap();
    d.validate().unwrap();
    d
}

#[test]
fn o_combo_vira_lista_constante_e_estado() {
    let arqs = gerar(&com_combo(1));
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    // The document collection becomes a constant — flat, because it is flat.
    assert!(ui.contains("const CBO_UF_ITEMS: &[&str] = &["), "{ui}");
    assert!(ui.contains("\"SP\","), "{ui}");
    // And ItemIndex becomes the initial state.
    assert!(
        init_has(
            ui,
            "cbo_uf: ComboBox {",
            &[
                "visible: true",
                "enabled: true",
                "hint: \"\".to_string()",
                "show_hint: true",
                "popup_menu: \"\".to_string()",
                "cursor: Cursor::Default",
                "font: Font { name: \"\".to_string(), size: 0.0, bold: false, italic: false, underline: false, strike_out: false, color: [0, 0, 0, 0] }",
                "parent_font: true",
                "color: Color { r: 0, g: 0, b: 0, a: 0 }",
                "parent_color: true",
                "selected: Some(1)",
                "items: Strings::from_slice(CBO_UF_ITEMS)",
            ]
        ),
        "{ui}"
    );
    assert!(ui.contains("pub struct ComboBox {"), "{ui}");
    assert!(ui.contains("pub selected: Option<usize>,"), "{ui}");
    assert!(ui.contains("pub items: Strings,"), "{ui}");
}

#[test]
fn item_index_menos_um_e_nenhum_selecionado() {
    // `-1` is the *none* of a classic RAD IDE, not index -1.
    let arqs = gerar(&com_combo(-1));
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(ui.contains("selected: None"), "{ui}");
}

#[test]
fn mudar_a_escolha_do_combo_chama_o_handler() {
    let arqs = gerar(&com_combo(0));
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(
        ui.contains(
            "let changed = self.cbo_uf.selected != before || self.cbo_uf.text != text_before;"
        ),
        "change is compared with previous value:\n{ui}"
    );
    assert!(ui.contains("self.cbo_ufChange();"), "{ui}");

    // And the handler stub is born in the user's file.
    let stub = conteudo(&arqs, "src/form_main.rs");
    assert!(stub.contains("pub fn cbo_ufChange(&mut self)"), "{stub}");
}

#[test]
fn o_combo_gerado_fica_onde_o_designer_o_pos() {
    // Live discovery: egui's `ComboBox` does NOT accept a rectangle like
    // `ui.put` accepts — it places itself at the cursor of the receiving `Ui`. The emitted
    // code used `r` only for width, and the combo ended up wherever the panel
    // was, ignoring the designer position.
    let arqs = gerar(&com_combo(0));
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(
        ui.contains("egui::UiBuilder::new().max_rect(r)"),
        "combo needs a child Ui in designer rect:
{ui}"
    );
    assert!(
        ui.contains(".show_ui(&mut w,"),
        "and it draws inside this child:
{ui}"
    );
}

/// A form with one component from each class emitted by the target.
fn com_todos() -> Document {
    let reg = crawrad_registry::Registry::builtin();
    let mut d = documento();
    let mut y = 200.0;
    for (id, classe, nome) in [
        ("chk1", "CheckBox", "chk_ativo"),
        ("rb1", "RadioButton", "rb_fisica"),
        ("lst1", "ListBox", "lst_cidades"),
        ("mmo1", "Memo", "mmo_obs"),
        ("cbo1", "ComboBox", "cbo_uf"),
        ("tv1", "TreeView", "tv_areas"),
    ] {
        let mut n = Node::new(id, classe);
        n.set_name(nome);
        n.set_rect(Rect::new(8.0, y, 145.0, 24.0));
        // Only what the class declares — the generator now refuses a
        // property its class does not have, as the IDE always did.
        let desc = reg.get(classe).expect("a built-in class");
        for (key, value) in [
            ("checked", Value::Bool(true)),
            (
                "items",
                crawrad_ir::Item::list_to(&[
                    crawrad_ir::Item::new("A"),
                    crawrad_ir::Item::new("B"),
                ]),
            ),
            ("item_index", Value::Int(1)),
        ] {
            if desc.prop(key).is_some() {
                n.props.insert(key.into(), value);
            }
        }
        d.add_child("form_main", n).unwrap();
        y += 30.0;
    }
    d.validate().unwrap();
    d
}

#[test]
fn nenhum_marcador_do_gerador_vaza_para_o_codigo_gerado() {
    // Actually happened: the CheckBox widget was a literal with `{nome}`
    // passed as a VALUE to `format!`, so `{nome}` reached the user's file
    // as text and the project didn't compile. This catches the whole class
    // of error, and not just that one case.
    //
    // Scans the ENTIRE CATALOGUE, not the six-class list built by
    // `com_todos`: markers only leak through snippets of classes present on the
    // form, and the hand-written list was already five classes behind the
    // registry.
    let arqs = gerar(&documento_com_o_catalogo_inteiro());
    for a in &arqs {
        if a.caminho.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        for marcador in ["{nome}", "{NOME}", "{depois}", "{comum}", "{tipo_estado}"] {
            assert!(
                !a.conteudo.contains(marcador),
                "`{marcador}` leaked into {}:\n{}",
                a.caminho.display(),
                a.conteudo
            );
        }
    }
}

#[test]
fn as_nove_classes_saem_com_tipo_de_estado_proprio() {
    let arqs = gerar(&com_todos());
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    for tipo in [
        "pub struct Label {",
        "pub struct Button {",
        "pub struct Edit {",
        "pub struct CheckBox {",
        "pub struct RadioButton {",
        "pub struct ListBox {",
        "pub struct Memo {",
        "pub struct ComboBox {",
        "pub struct TreeView {",
    ] {
        assert!(ui.contains(tipo), "missing {tipo}:\n{ui}");
    }
}

#[test]
fn o_checked_do_documento_vira_o_estado_inicial() {
    let arqs = gerar(&com_todos());
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(
        init_has(
            ui,
            "chk_ativo: CheckBox {",
            &[
                "visible: true",
                "enabled: true",
                "hint: \"\".to_string()",
                "show_hint: true",
                "popup_menu: \"\".to_string()",
                "cursor: Cursor::Default",
                "font: Font { name: \"\".to_string(), size: 0.0, bold: false, italic: false, underline: false, strike_out: false, color: [0, 0, 0, 0] }",
                "parent_font: true",
                "text: \"\".to_string()",
                "checked: true",
                "state: \"cbChecked\".to_string()",
                "allow_grayed: false",
                "alignment: \"taRightJustify\".to_string()",
            ]
        ),
        "{ui}"
    );
    // And the widget reads state, not a literal.
    assert!(
        ui.contains("egui::Checkbox::new(&mut self.chk_ativo.checked"),
        "{ui}"
    );
    assert!(
        ui.contains("egui::RadioButton::new(self.rb_fisica.checked"),
        "{ui}"
    );
}

#[test]
fn o_memo_e_multilinha_e_a_lista_tem_a_colecao() {
    let arqs = gerar(&com_todos());
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(
        ui.contains("egui::TextEdit::multiline(&mut self.mmo_obs.text)"),
        "{ui}"
    );
    assert!(ui.contains("const LST_CIDADES_ITEMS: &[&str] = &["), "{ui}");
    assert!(
        init_has(
            ui,
            "lst_cidades: ListBox {",
            &[
                "visible: true",
                "enabled: true",
                "hint: \"\".to_string()",
                "show_hint: true",
                "popup_menu: \"\".to_string()",
                "cursor: Cursor::Default",
                "font: Font { name: \"\".to_string(), size: 0.0, bold: false, italic: false, underline: false, strike_out: false, color: [0, 0, 0, 0] }",
                "parent_font: true",
                "color: Color { r: 0, g: 0, b: 0, a: 0 }",
                "parent_color: true",
                "selected: Some(1)",
                "selection: vec![1]",
                "multi_select: false",
                "sorted: false",
                "items: Strings::from_slice(LST_CIDADES_ITEMS)",
            ]
        ),
        "{ui}"
    );
    assert!(ui.contains("pub items: Strings,"), "{ui}");
    assert!(ui.contains("pub lines: Strings,"), "{ui}");
}

#[test]
fn memo_e_listbox_permitem_push_em_lines_e_items() {
    let arqs = gerar(&com_todos());
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(ui.contains("pub struct Strings {"), "{ui}");
    assert!(ui.contains("impl std::ops::Deref for Strings"), "{ui}");
    assert!(ui.contains("impl std::ops::DerefMut for Strings"), "{ui}");
    assert!(
        ui.contains("self.lst_cidades.items.iter().enumerate()"),
        "{ui}"
    );
    assert!(ui.contains("self.cbo_uf.items.iter().enumerate()"), "{ui}");
    assert!(
        ui.contains("self.mmo_obs.lines.text() != self.mmo_obs.text"),
        "{ui}"
    );
    assert!(
        ui.contains("self.mmo_obs.lines.set_text(&self.mmo_obs.text);"),
        "{ui}"
    );
}

#[test]
fn escolher_um_radio_desmarca_os_irmaos_no_codigo_gerado() {
    // The question from the project owner that turned into a fix: the IDE already unchecked
    // siblings (RXP has the loop, and the classic component library of the time inherited from Windows), but the
    // GENERATED application did not — two completely unrelated `bool`s. The designer
    // showed one thing and the app did another.
    let mut d = documento();
    for (id, nome) in [
        ("rb1", "rb_fisica"),
        ("rb2", "rb_juridica"),
        ("rb3", "rb_outra"),
    ] {
        let mut n = Node::new(id, "RadioButton");
        n.set_name(nome);
        n.set_rect(Rect::new(8.0, 200.0, 110.0, 20.0));
        d.add_child("form_main", n).unwrap();
    }
    d.node_mut("rb1")
        .unwrap()
        .events
        .insert("on_click".into(), "rb_fisicaClick".into());
    d.validate().unwrap();

    let arqs = gerar(&d);
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(
        ui.contains("self.rb_fisica.checked = true;")
            && ui.contains("self.rb_juridica.checked = false;")
            && ui.contains("self.rb_outra.checked = false;"),
        "clicking rb_fisica should uncheck both siblings:
{ui}"
    );

    // And the handler runs AFTERWARDS: it already sees the final state, as in a classic RAD IDE.
    let i_marca = ui.find("self.rb_juridica.checked = false;").unwrap();
    let i_handler = ui.find("self.rb_fisicaClick();").unwrap();
    assert!(
        i_marca < i_handler,
        "handler cannot run before exclusion:
{ui}"
    );
}

#[test]
fn caixa_de_marcar_nao_desmarca_ninguem() {
    // What separates a CheckBox from a RadioButton is precisely this.
    let mut d = documento();
    for (id, nome) in [("chk1", "chk_a"), ("chk2", "chk_b")] {
        let mut n = Node::new(id, "CheckBox");
        n.set_name(nome);
        n.set_rect(Rect::new(8.0, 200.0, 110.0, 20.0));
        d.add_child("form_main", n).unwrap();
    }
    d.validate().unwrap();
    let arqs = gerar(&d);
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(!ui.contains("self.chk_b.checked = false;"), "{ui}");
}

#[test]
fn o_radio_gerado_para_na_borda_do_pai() {
    // The generated application must exclude like the IDE excludes: among siblings of the
    // same parent (`TurnSiblingsOff`). While the generator's radio list was
    // flat, clicking a radio inside a panel unchecked the ones outside — in the
    // app, not in the designer. That is the worst place for a divergence to appear,
    // because the drawing keeps telling the truth and only the program lies.
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").unwrap();
        raiz.set_rect(Rect::new(0.0, 0.0, 480.0, 320.0));
        raiz.set_name("form_main");
    }

    let mut pnl = Node::new("pnl1", "Panel");
    pnl.set_name("pnl_pagamento");
    pnl.set_rect(Rect::new(20.0, 20.0, 200.0, 120.0));
    d.add_child("form_main", pnl).unwrap();

    for (id, nome, y) in [("rb1", "rb_avista", 8.0), ("rb2", "rb_prazo", 32.0)] {
        let mut rb = Node::new(id, "RadioButton");
        rb.set_name(nome);
        rb.set_rect(Rect::new(8.0, y, 120.0, 20.0));
        d.add_child("pnl1", rb).unwrap();
    }

    let mut fora = Node::new("rb3", "RadioButton");
    fora.set_name("rb_fora");
    fora.set_rect(Rect::new(20.0, 200.0, 120.0, 20.0));
    d.add_child("form_main", fora).unwrap();
    d.validate().unwrap();

    let arqs = gerar(&d);
    let designer = conteudo(&arqs, "src/form_main.designer.rs");

    let corpo = fatia_do_clique(designer, "rb_avista");
    assert!(corpo.contains("self.rb_prazo.checked = false;"), "{corpo}");
    assert!(
        !corpo.contains("rb_fora"),
        "radio inside panel reached the outside one:\n{corpo}"
    );

    let corpo = fatia_do_clique(designer, "rb_fora");
    assert!(
        !corpo.contains("rb_avista") && !corpo.contains("rb_prazo"),
        "form radio reached inside panel:\n{corpo}"
    );
}

/// The `if clicked { ... }` block belonging to a component.
///
/// The generated code is a long function and each component has its own snippet;
/// searching for the bare string across the whole file would match the wrong radio.
fn fatia_do_clique(fonte: &str, nome: &str) -> String {
    let marca = format!("self.{nome}.checked = true;");
    let i = fonte.find(&marca).unwrap_or_else(|| {
        panic!("could not find snippet for `{nome}` in generated code:\n{fonte}")
    });
    let ini = fonte[..i].rfind("if clicked {").unwrap_or(i);
    // the whole block, braces matched: it holds nested `if`s
    let mut depth = 0usize;
    let mut fim = fonte.len();
    for (k, c) in fonte[ini..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    fim = ini + k;
                    break;
                }
            }
            _ => {}
        }
    }
    fonte[ini..fim].to_string()
}

#[test]
fn o_groupbox_gerado_traz_a_moldura_so_quando_e_usado() {
    // Two things at once: `GroupBox` emits state and frame, and the frame
    // is NOT included when there is no group box. Dead functions in the user's
    // file trigger `cargo` warnings they cannot fix — the file is
    // generated, they do not control it.
    let sem = conteudo(&gerar(&documento()), "src/form_main.designer.rs").to_string();
    assert!(
        !sem.contains("fn group_box_frame"),
        "frame entered a form without `GroupBox`"
    );

    let mut d = documento();
    let mut gb = Node::new("gbx1", "GroupBox");
    gb.set_name("gbx_pagamento");
    gb.set_text("Payment");
    gb.set_rect(Rect::new(40.0, 150.0, 200.0, 100.0));
    d.add_child("form_main", gb).unwrap();
    d.validate().unwrap();

    let com = conteudo(&gerar(&d), "src/form_main.designer.rs").to_string();
    assert!(com.contains("fn group_box_frame"), "missing frame");
    assert!(com.contains("pub struct GroupBox"), "missing state");
    assert!(
        com.contains("pub gbx_pagamento: GroupBox,"),
        "missing component field"
    );
    assert!(
        com.contains(
            "group_box_frame(ui, r, self.gbx_pagamento.text.as_str(), self.gbx_pagamento.enabled);"
        ),
        "missing draw call"
    );
}

#[test]
fn o_filho_de_pagina_fechada_nao_desenha_no_app() {
    // Conditional visibility must hold in the APPLICATION, not just in the
    // designer. Without the guard, the page 2 button would be drawn on top of
    // page 1 content — and only someone running the program would find out.
    let mut d = documento();
    let mut pc = Node::new("pgc1", "PageControl");
    pc.set_name("pgc_cadastro");
    pc.set_rect(Rect::new(20.0, 150.0, 320.0, 200.0));
    pc.props.insert("active_page".into(), Value::Int(0));
    d.add_child("form_main", pc).unwrap();
    for (id, nome, legenda) in [("t1", "tab_dados", "Data"), ("t2", "tab_notas", "Notes")] {
        let mut t = Node::new(id, "TabSheet");
        t.set_name(nome);
        t.set_text(legenda);
        t.set_rect(Rect::new(0.0, 24.0, 320.0, 176.0));
        d.add_child("pgc1", t).unwrap();
    }
    let mut b = Node::new("bp2", "Button");
    b.set_name("btn_pagina_dois");
    b.set_text("Second");
    b.set_rect(Rect::new(10.0, 10.0, 96.0, 32.0));
    d.add_child("t2", b).unwrap();
    d.validate().unwrap();

    let g = conteudo(&gerar(&d), "src/form_main.designer.rs").to_string();
    assert!(g.contains("fn page_control("), "missing tab strip drawing");
    // Captions come from the STATE of each page, not from a constant with
    // frozen text: changing `text` at runtime must update the strip. And it was
    // that constant that left the form's `TabSheet` fields without a
    // single reader — `dead_code` silenced by a `#![allow]` that no longer exists.
    assert!(
        g.contains(
            "let captions: [(&str, &str); 2] = [(self.tab_dados.text.as_str(), if CRAWRAD_FORM_SHOW_HINT && self.tab_dados.show_hint { self.tab_dados.hint.as_str() } else { \"\" }), (self.tab_notas.text.as_str(), if CRAWRAD_FORM_SHOW_HINT && self.tab_notas.show_hint { self.tab_notas.hint.as_str() } else { \"\" })];"
        ),
        "strip captions did not come from page state"
    );
    assert!(
        !g.contains("PGC_CADASTRO_TABS"),
        "captions constant returned"
    );
    // The guard wraps the DRAWING SNIPPET, which starts at the comment the
    // generator emits per node. Searching for the bare name would match the struct field,
    // which comes before and is guarded by nothing.
    let trecho = |nome: &str| {
        let i = g
            .find(&format!("// {nome} — Button"))
            .unwrap_or_else(|| panic!("could not find drawing for `{nome}`"));
        g[..i].to_string()
    };
    assert!(
        trecho("btn_pagina_dois").ends_with(
            "if self.pgc_cadastro.visible && self.pgc_cadastro.active_page == 1 && self.tab_notas.visible {

        "
        ),
        "page 2 button was not placed under page 2 guard"
    );
    assert!(
        !trecho("btn_ok").contains("active_page =="),
        "component outside PageControl gained page guard"
    );
}

#[test]
fn a_guarda_de_pagina_compoe_com_a_dos_ancestrais() {
    // THE DEFECT: the guard was only for the CLOSEST page, with the comment
    // claiming that the rest came "from nested `if`s". It didn't —
    // `show()` is emitted flat; each node opens and CLOSES its own. With a
    // `PageControl` inside a `TabSheet`, the bottom button was placed under
    // `if self.pgc_interno.active_page == 0` and nothing else, and was drawn with
    // the outer tab closed.
    //
    // `gate_fase3` proves this through clicks, which is the test that matters; here
    // is the cheap version that runs on every `cargo test`.
    let mut d = documento();
    let mut ext = Node::new("pgc1", "PageControl");
    ext.set_name("pgc_externo");
    ext.set_rect(Rect::new(20.0, 150.0, 320.0, 200.0));
    d.add_child("form_main", ext).unwrap();
    for (id, nome) in [("te1", "tab_ext_um"), ("te2", "tab_ext_dois")] {
        let mut t = Node::new(id, "TabSheet");
        t.set_name(nome);
        t.set_text(nome);
        t.set_rect(Rect::new(0.0, 24.0, 320.0, 176.0));
        d.add_child("pgc1", t).unwrap();
    }
    // The inner page lives on the SECOND outer tab: this makes the two conditions
    // different (1 and 0) so one cannot be mistaken for the other.
    let mut int = Node::new("pgc2", "PageControl");
    int.set_name("pgc_interno");
    int.set_rect(Rect::new(8.0, 8.0, 280.0, 120.0));
    d.add_child("te2", int).unwrap();
    for (id, nome) in [("ti1", "tab_int_um"), ("ti2", "tab_int_dois")] {
        let mut t = Node::new(id, "TabSheet");
        t.set_name(nome);
        t.set_text(nome);
        t.set_rect(Rect::new(0.0, 24.0, 280.0, 96.0));
        d.add_child("pgc2", t).unwrap();
    }
    // And a `Panel` along the way, to prove that common containers do not
    // interrupt the ascent — and that its `visible` also
    // hides the button.
    let mut pn = Node::new("pnl1", "Panel");
    pn.set_name("pnl_fundo");
    pn.set_rect(Rect::new(4.0, 4.0, 200.0, 60.0));
    d.add_child("ti1", pn).unwrap();
    let mut b = Node::new("bf", "Button");
    b.set_name("btn_fundo");
    b.set_text("Deep");
    b.set_rect(Rect::new(4.0, 4.0, 96.0, 32.0));
    d.add_child("pnl1", b).unwrap();
    d.validate().unwrap();

    let g = conteudo(&gerar(&d), "src/form_main.designer.rs").to_string();
    let antes_do_desenho = |nome: &str, classe: &str| {
        let i = g
            .find(&format!("// {nome} — {classe}"))
            .unwrap_or_else(|| panic!("could not find drawing for `{nome}`"));
        g[..i].to_string()
    };
    assert!(
        antes_do_desenho("btn_fundo", "Button").ends_with(
            "if self.pgc_externo.visible && self.pgc_externo.active_page == 1 && self.tab_ext_dois.visible && self.pgc_interno.visible && self.pgc_interno.active_page == 0 && self.tab_int_um.visible && self.pnl_fundo.visible {

        "
        ),
        "bottom button guard did not include EXTERNAL page:\n{}",
        antes_do_desenho("btn_fundo", "Button")
            .lines()
            .next_back()
            .unwrap_or("")
    );
    // The `Panel` along the path is guarded by the same two conditions — it lives
    // exactly where the button lives.
    assert!(
        antes_do_desenho("pnl_fundo", "Panel").ends_with(
            "if self.pgc_externo.visible && self.pgc_externo.active_page == 1 && self.tab_ext_dois.visible && self.pgc_interno.visible && self.pgc_interno.active_page == 0 && self.tab_int_um.visible {

        "
        ),
        "panel inside nested page lost outer guard"
    );
    // And the inner `PageControl` is guarded only by the outer one: it is a child of the
    // outer tab, not of itself.
    assert!(
        antes_do_desenho("pgc_interno", "PageControl").ends_with(
            "if self.pgc_externo.visible && self.pgc_externo.active_page == 1 && self.tab_ext_dois.visible {

        "
        ),
        "inner PageControl gained its own guard"
    );
}

#[test]
fn as_medidas_da_aba_no_gerado_batem_com_o_registro() {
    // Tab dimensions used to be THREE copies: `crawrad-registry`,
    // `crawrad-runtime-egui`, and a string in `rxp.rs`. Today there are two — the registry
    // and `TAB_H`/`TAB_W` in `desenho_comum.rs` —, because Preview and the
    // application now read the same file.
    //
    // The doc for `ALTURA_DA_TIRA_DE_ABAS` stated that THREE places needed to
    // agree and that a test enforced it. It enforced two: the reviewer put 40/150
    // here and the entire workspace stayed green. Designer drawing would put
    // the tab in one place and the generated application in another.
    //
    // The remaining one stays here for dependency reasons:
    // `crawrad-runtime-egui` has `crawrad-registry` as a **dev-dependency** on
    // purpose (the adapter does not know the catalogue, ADR 0002/0013), and
    // `desenho_comum.rs` cannot depend on any of our crates — it is copied
    // into a project that only depends on `eframe`. Thus the bridge between the
    // catalogue and the drawing is this test, and it measures the TEXT that will be
    // emitted, not a constant from this crate.
    let secao = crate::rxp::trecho("abas");
    let esperado_h = format!(
        "const TAB_H: f32 = {:.1};",
        crawrad_registry::TAB_STRIP_HEIGHT
    );
    let esperado_w = format!("const TAB_W: f32 = {:.1};", crawrad_registry::TAB_WIDTH);
    assert!(
        secao.contains(&esperado_h),
        "tab height in generated code diverged from registry: expected `{esperado_h}`"
    );
    assert!(
        secao.contains(&esperado_w),
        "tab width in generated code diverged from registry: expected `{esperado_w}`"
    );
}

// =========================================================================
// THE SHARED DRAWING FILE
//
// `desenho_comum.rs` is compiled by `crawrad-runtime-egui` and emitted by
// this crate. The compiler guards the first reader; the tests below guard
// the second, which has no compiler until the user runs
// `cargo build` in their project.
// =========================================================================

/// The shared file, as this crate sees it: text.
fn desenho_comum() -> String {
    include_str!("../../../../crawrad-runtime-egui/src/desenho_comum.rs").replace("\r\n", "\n")
}

/// Everything the shared file actually EMITS, joined.
///
/// This is used to measure what reaches the user's project — the internal
/// crate header, which stays outside every section, does not arrive.
fn desenho_comum_emitido() -> String {
    secoes_do_desenho_comum()
        .iter()
        .map(|n| crate::rxp::trecho(n))
        .collect()
}

/// Section names, in the order they appear.
fn secoes_do_desenho_comum() -> Vec<String> {
    desenho_comum()
        .lines()
        .filter_map(|l| {
            l.trim()
                .strip_prefix("// [CRAWRAD:INICIO ")?
                .strip_suffix(']')
                .map(str::to_string)
        })
        .collect()
}

#[test]
fn o_desenho_comum_nao_tem_doc_de_modulo_nem_atributo_interno() {
    // **The rule measured by the study, and the only one that breaks compilation of
    // the user's project without breaking ours.** This text is inserted in the MIDDLE of
    // `designer.rs`, after `use eframe::egui;`, and `//!` or `#![...]`
    // after an item is `error[E0753]`. Here the file is the root of an
    // `include!` inside `lib.rs`, and the error would appear too — but the
    // first to break would be the user, during `cargo build` in their project.
    for (i, linha) in desenho_comum().lines().enumerate() {
        let t = linha.trim_start();
        assert!(
            !t.starts_with("//!") && !t.starts_with("#!["),
            "desenho_comum.rs:{}: `{}` — module doc and inner attribute do not \
             survive emission (E0753)",
            i + 1,
            linha.trim()
        );
    }
}

#[test]
fn o_desenho_comum_nao_tem_use_nenhum() {
    // egui paths are fully qualified because on one side `egui` is the crate
    // and on the other it is the `use eframe::egui;` written by the generator. A `use` here
    // would resolve on one side only — and the side that breaks is the user's.
    //
    // `trim_start` is not ornamental, and the sister rule for `//!` already had it: without
    // it, an INDENTED `use` — inside a `fn`, where it is most
    // natural to write — would pass green and break the user's `cargo build`.
    // Verified by the reviewer.
    for (i, linha) in desenho_comum().lines().enumerate() {
        assert!(
            !linha.trim_start().starts_with("use "),
            "desenho_comum.rs:{}: `{}` — shared file imports \
             nothing; prelude belongs to the compiling side",
            i + 1,
            linha.trim()
        );
    }
}

#[test]
fn o_desenho_comum_nao_congela_cor_nenhuma() {
    // **Owner constraint, stemming from the THEME decision:**
    // colors come from `ui.visuals()`, never from a literal. The day theme
    // becomes a form property, a file with hardcoded colors would need
    // to be completely rewritten — and it is precisely the file that must not
    // be rewritten, because it has two readers.
    // **The list of forbidden names was replaced with the entire `Color32::`**, and the
    // reason is that the list lied about its own completeness: it checked
    // `from_rgb`, `from_rgba`, and `BLACK`, while letting `Color32::WHITE`,
    // `Color32::RED`, and `Color32::from_gray(7)` slip through — all equally hardcoded.
    // A guard that enumerates forbidden patterns is always one constructor behind; one
    // that enumerates what is ALLOWED is not. Here no color is allowed: whoever needs
    // one queries `ui.visuals()`.
    let emitido = desenho_comum_emitido();
    for (i, linha) in emitido.lines().enumerate() {
        assert!(
            !linha.contains("Color32"),
            "desenho_comum.rs hardcoded a color on line {} of emitted part \
             (`{}`). All colors come from `ui.visuals()` — this file is the only \
             one that cannot be rewritten when THEME becomes a form \
             property (issue #76).",
            i + 1,
            linha.trim()
        );
    }
}

#[test]
fn a_linha_que_faz_o_fmt_enxergar_o_desenho_comum_continua_la() {
    // **The guard of the guard, and it is necessary because the guarded line LOOKS
    // redundant.** `desenho_comum.rs` enters the runtime via `include!`, and
    // `cargo fmt` follows `mod` but does not follow `include!` — verified: with the file
    // intentionally malformatted and without the line, `cargo fmt --all -- --check` exits
    // with 0. What makes rustfmt find the path is a
    // `#[cfg(any())] mod desenho_comum;`, which the compiler never sees.
    //
    // In other words: deleting it as "dead code" breaks no compilation, emits
    // no warning, and blinds `fmt` across the entire shared file. The gate
    // would stay green. This test is the only way for the removal to hurt
    // immediately, instead of months later in a broken file.
    //
    // Reads runtime `lib.rs` as TEXT, for the same reason that
    // `toda_secao_do_desenho_comum_e_emitida_por_alguem` reads `rxp.rs`: what
    // is guarded here is not a value, it is the existence of a line of code.
    let lib = include_str!("../../../../crawrad-runtime-egui/src/lib.rs").replace("\r\n", "\n");
    let mut viu_cfg = false;
    for (i, linha) in lib.lines().enumerate() {
        if linha.trim() != "#[cfg(any())]" {
            continue;
        }
        if lib.lines().nth(i + 1).map(str::trim) == Some("mod desenho_comum;") {
            viu_cfg = true;
        }
    }
    assert!(
        viu_cfg,
        "`#[cfg(any())] mod desenho_comum;` disappeared from \
         `crawrad-runtime-egui/src/lib.rs`. Without it `cargo fmt --all` stops \
         seeing the shared file — and still exits with 0, which makes \
         the loss invisible. It is not dead code: it is the path \
         rustfmt traverses and `include!` does not provide."
    );
    assert!(
        lib.contains("include!(\"desenho_comum.rs\");"),
        "`include!(\"desenho_comum.rs\")` disappeared from runtime. The shared \
         file would stop being COMPILED on this side and become what \
         it exists not to be: text only the user finds broken."
    );
}

#[test]
fn o_desenho_comum_nao_conhece_crate_nosso() {
    // On the other side none of our crates exist: the user project knows `eframe`
    // and nothing else, and that is the proof that real generation occurred rather
    // than disguised interpretation. A `crawrad_` here would only surface in the user's `cargo build`.
    assert!(
        !desenho_comum_emitido().contains("crawrad_"),
        "desenho_comum.rs referenced our crate; on the emitted side it does not exist"
    );
}

#[test]
fn todo_codigo_do_desenho_comum_mora_dentro_de_uma_secao() {
    // **This test allows the two above to measure only emitted content.** Code
    // outside a section is code that Preview compiles and the application never sees —
    // the divergence returns, now by omission rather than duplication. Outside
    // sections there can only be comments and blank lines.
    let mut dentro = false;
    for (i, linha) in desenho_comum().lines().enumerate() {
        let t = linha.trim();
        if t.starts_with("// [CRAWRAD:INICIO ") {
            dentro = true;
            continue;
        }
        if t.starts_with("// [CRAWRAD:FIM ") {
            dentro = false;
            continue;
        }
        assert!(
            dentro || t.is_empty() || t.starts_with("//"),
            "desenho_comum.rs:{}: `{t}` is outside all sections — Preview \
             compiles this and the application never receives it",
            i + 1
        );
    }
}

#[test]
fn todo_ui_id_que_e_emitido_carrega_o_nome_da_instancia() {
    // **Guard for the defect that cost a REJECT, and it checks text because the
    // compiler cannot catch it.**
    //
    // `Ui::id()` does NOT distinguish siblings: a child `Ui` is created with
    // `parent.id().with("child")` — a fixed word (`egui-0.36.1`,
    // `src/ui.rs:251`), and `Ui::id()` on line 344 returns that `stable_id`,
    // not the `unique_id` next to it. In the generated application it is worse: the
    // whole form draws into a single `Ui`, so `ui.id()` is literally the
    // same value for all components.
    //
    // The mistake was written in THREE places and the PR fixed one. This test
    // prevents the fourth: whoever writes `ui.id()` in code destined for
    // `designer.rs` without an instance discriminator on the same line fails
    // here, rather than on a user's form with two identical components.
    //
    // Measures the GENERATED FILE, not the `const`-strings: this also covers
    // anything emitted elsewhere, and the day call site and function live in
    // different files.
    let arquivos = gerar(&documento_com_o_catalogo_inteiro());
    let designer = arquivos
        .iter()
        .find(|a| a.caminho.to_string_lossy().contains(crate::SUFIXO_GERADO))
        .expect("the designer file");
    let mut achou = 0;
    for (i, linha) in designer.conteudo.lines().enumerate() {
        if linha.trim_start().starts_with("//") || !linha.contains("ui.id()") {
            continue;
        }
        achou += 1;
        // **Two ways to carry the discriminator, and both are valid.**
        //
        // `name` is used by the shared function (`page_control`), emitted ONCE
        // in the file and taking the instance parameter. String literals are used
        // by code emitting one line PER instance — third-party declared drawing
        // writes `ui.id().with(("crawrad_declared", "gau1"))`, with the
        // instance name embedded right there.
        //
        // The rule that matters is *"carries something distinguishing instances"*, and the
        // first version of this guard checked the SHAPE of one solution instead
        // of the property. The counterpart test for `evento_do_terceiro` checks the
        // property directly: two gauges on the same form must end up with
        // DIFFERENT `id`s.
        assert!(
            linha.contains("name") || linha.contains('"'),
            "{}:{}: `{}` — `ui.id()` without the instance name. Two components \
             of the same class on the same form would share the click target, \
             and clicking one would respond for the other.",
            designer.caminho.display(),
            i + 1,
            linha.trim()
        );
    }
    // And the guard cannot pass green by finding nothing: the day
    // `ui.id()` disappears from emitted output, this test must fail so someone can decide
    // if it is still needed — a blind probe is worse than no probe.
    assert!(
        achou >= 2,
        "expected at least two lines of code with `ui.id()` in the generated \
         designer (the tab and tree) and found {achou}; either emitted output changed, or \
         this guard stopped measuring something"
    );
}

#[test]
fn toda_secao_do_desenho_comum_fecha_e_e_unica() {
    let texto = desenho_comum();
    let secoes = secoes_do_desenho_comum();
    assert!(
        !secoes.is_empty(),
        "desenho_comum.rs has no sections — nothing would be emitted"
    );
    for nome in &secoes {
        let quantas = secoes.iter().filter(|o| *o == nome).count();
        assert_eq!(
            quantas, 1,
            "section `{nome}` opens {quantas} times; the name is the slice key"
        );
        assert!(
            texto.contains(&format!("// [CRAWRAD:FIM {nome}]")),
            "section `{nome}` opens and does not close — slicing would swallow the rest \
             of the file into the user's project"
        );
    }
    // And end marker without start marker: slicing would not find it, but it
    // would linger in the file asserting a nonexistent section exists.
    for linha in texto.lines() {
        if let Some(nome) = linha
            .trim()
            .strip_prefix("// [CRAWRAD:FIM ")
            .and_then(|r| r.strip_suffix(']'))
        {
            assert!(
                secoes.iter().any(|s| s == nome),
                "end marker `{nome}` has no start"
            );
        }
    }
}

#[test]
fn toda_secao_do_desenho_comum_e_emitida_por_alguem() {
    // A section nobody slices is code that Preview compiles and the application
    // never sees — exactly the divergence this file exists to prevent,
    // sneaking in through the back door.
    let gerador = include_str!("../../targets/egui/form.rs");
    for nome in secoes_do_desenho_comum() {
        let chamada = format!("trecho(\"{nome}\")");
        assert!(
            gerador.contains(&chamada),
            "section `{nome}` of desenho_comum.rs is not emitted by anything: \
             there is no `{chamada}` in the egui generator"
        );
    }
}

#[test]
fn o_recorte_devolve_a_secao_inteira_e_so_ela() {
    // The contract of [`crate::rxp::trecho`]: output starts on the line AFTER
    // the start marker and ends on the line BEFORE the end marker — markers are
    // internal and do not travel.
    let abas = crate::rxp::trecho("abas");
    assert!(
        !abas.contains("[CRAWRAD:"),
        "the slice carried the marker along into the user's project"
    );
    assert!(
        abas.starts_with('\n') && abas.ends_with("}\n"),
        "the slice does not join like the `const`-strings joined: starts with \
         `{:?}` and ends with `{:?}`",
        &abas[..abas.len().min(3)],
        &abas[abas.len().saturating_sub(3)..]
    );
    assert!(
        abas.contains("fn page_control("),
        "the slice of section `abas` did not include the function"
    );
    assert!(
        !abas.contains("fn timer_due("),
        "the slice of section `abas` went past the end and brought the timer along"
    );
    // And the inverse, so the marker is not useless: the timer
    // does not bring along the tab strip.
    let tempo = crate::rxp::trecho("temporizador");
    assert!(tempo.contains("fn timer_due(") && !tempo.contains("fn page_control("));
}

#[test]
fn o_recorte_normaliza_o_fim_de_linha_de_windows() {
    // **The test that cannot be empty.** `core.autocrlf` here is turned
    // on (`git status` warns verbatim: *"LF will be replaced
    // by CRLF the next time Git touches it"*), so `desenho_comum.rs`
    // arrives in CRLF on disk when cloned — while `const`-strings in
    // `rxp.rs` arrive in LF, because the Rust lexer normalizes string
    // literals while `include_str!` normalizes nothing.
    //
    // Measuring this against disk files would pass green by accident on any
    // machine that checked out in LF. Thus CRLF here is synthetic,
    // and that is what gets tested.
    let comum = desenho_comum();
    let em_crlf = comum.replace('\n', "\r\n");
    assert!(
        em_crlf.contains('\r'),
        "the synthetic CRLF was not constructed"
    );
    let recortado = crate::rxp::recorte(&em_crlf, "abas");
    assert!(
        !recortado.contains('\r'),
        "the slice returned CRLF; `designer.rs` would end up with mixed \
         line endings, depending on who generated it"
    );
    assert_eq!(
        recortado,
        crate::rxp::trecho("abas"),
        "the same text in CRLF and LF must yield the SAME slice"
    );
}

#[test]
fn o_designer_gerado_nao_leva_fim_de_linha_de_windows() {
    // `core.autocrlf` here is turned on, so `desenho_comum.rs`
    // arrives in CRLF on disk when cloned — while `const`-strings in
    // `rxp.rs` arrive in LF, because Rust's lexer normalizes string literals
    // and does not normalize `include_str!`. Without `trecho` normalization,
    // `designer.rs` would end up with MIXED line endings, depending on who
    // ran generation. This test measures the entire file, not just the spliced part.
    //
    // The document uses the ENTIRE CATALOGUE on purpose: it is the only one guaranteeing
    // every section of the shared file got sliced in this run.
    let d = documento_com_o_catalogo_inteiro();
    let arquivos = gerar(&d);
    for a in &arquivos {
        assert!(
            !a.conteudo.contains('\r'),
            "file `{}` had `\\r`",
            a.caminho.display()
        );
    }
}

#[test]
fn comentario_citando_o_handler_nao_impede_a_criacao() {
    // Both directions of the defect `contains("fn nome(")` had, and both
    // broke the user's build without a trace.
    let raiz = std::env::temp_dir().join("crawrad_codegen_elo_dup");
    let _ = std::fs::remove_dir_all(&raiz);
    let raiz = raiz.as_path();

    let d = documento();
    d.validate().unwrap();
    escrever(raiz, &gerar(&d)).expect("first generation");

    let unidade = raiz.join("src/form_main.rs");
    let antes = std::fs::read_to_string(&unidade).expect("the user unit");

    // 1. FALSE POSITIVE: the user cites the signature in a comment. The handler
    //    `btn_okClick` does NOT exist yet in this mutilated file — if the generator
    //    thinks it exists, it won't create it and `.designer.rs` calls what isn't there.
    let mutilado = antes.replace(
        "pub fn btn_okClick(&mut self) {",
        "// TODO: fn btn_okClick( ainda vou escrever\n    pub fn OUTRO_NOME(&mut self) {",
    );
    assert!(mutilado.contains("// TODO: fn btn_okClick("));
    std::fs::write(&unidade, &mutilado).unwrap();
    escrever(raiz, &gerar(&d)).expect("second generation");
    let depois = std::fs::read_to_string(&unidade).unwrap();
    assert!(
        crate::elo::localizar_handler(&depois, "btn_okClick").is_some(),
        "the comment citing the signature prevented creating the handler:\n{depois}"
    );

    // 2. FALSE NEGATIVE: space before parenthesis. The handler EXISTS, and the
    //    generator must not duplicate it.
    let espacado = antes.replace("pub fn btn_okClick(", "pub fn btn_okClick (");
    std::fs::write(&unidade, &espacado).unwrap();
    escrever(raiz, &gerar(&d)).expect("third generation");
    let depois = std::fs::read_to_string(&unidade).unwrap();
    assert_eq!(
        crate::elo::handlers_do_arquivo(&depois)
            .iter()
            .filter(|(n, _)| n == "btn_okClick")
            .count(),
        1,
        "the generator duplicated the handler because of a space:\n{depois}"
    );
}

/// A form with **one instance of each catalog class**.
///
/// Used for scans needing to see all text the generator knows how to
/// emit — the Portuguese check and leaking generator markers. With a hand-written
/// form they measured only the header, and the next class was added without
/// anyone looking.
fn documento_com_o_catalogo_inteiro() -> Document {
    let r = crawrad_registry::Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").unwrap();
        raiz.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        raiz.set_name("form_main");
        raiz.set_text("Cadastro");
    }
    let mut y = 8.0;
    for desc in r.all() {
        if desc.class == "Form" {
            continue;
        }
        let id = r.suggest_id(desc.class, |i| d.node(i).is_some());
        let (w, h) = desc.default_size;
        let n = r
            .instantiate(desc.class, id, Rect::new(8.0, y, w, h))
            .unwrap();
        d.add_child("form_main", n).unwrap();
        y += h + 4.0;
    }
    d.validate().unwrap();
    d
}

#[test]
fn o_codigo_gerado_nao_tem_portugues() {
    // User decision: generated code belongs to the user, and is ENTIRELY in English.
    // The translation was once done halfway — it caught identifiers
    // and built-in functions, while leaving headers, insertion markers, and
    // stub bodies behind. In other words: Portuguese remained exactly in the file
    // the user opens to edit.
    //
    // This test scans what is OURS in generated output. The form's `Caption` is
    // excluded on purpose — component text is user data, and may
    // be in any language.
    //
    // The document uses the ENTIRE CATALOGUE, rather than three components:
    // text emitted by a class only appears when present on the form,
    // so scanning a small form would only measure the header. New classes
    // enter this scan automatically.
    let arqs = gerar(&documento_com_o_catalogo_inteiro());
    // Words that only appear if someone writes Portuguese in the boilerplate.
    const RASTROS: &[&str] = &[
        "arquivo",
        "código",
        "codigo",
        "formulário",
        "formulario",
        "usuário",
        "usuario",
        "gerado",
        "GERADO",
        "seu ",
        "você",
        "aqui",
        "entram",
        "reescreve",
        "nunca toca",
    ];
    for a in &arqs {
        if a.caminho.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        // The test form's `Caption` is "Cadastro" — user data,
        // not ours. We strip lines containing string literals before
        // scanning.
        let nosso: String = a
            .conteudo
            .lines()
            .filter(|l| !l.contains(".to_string()") && !l.contains("with_title"))
            .collect::<Vec<_>>()
            .join("\n");
        for r in RASTROS {
            assert!(
                !nosso.contains(r),
                "`{}` has Portuguese in our code: `{r}`\n{nosso}",
                a.caminho.display()
            );
        }
    }
}

#[test]
fn o_modulo_do_designer_leva_o_nome_do_formulario() {
    // It used to be named `ui`, which was a bug waiting for the second form:
    // two `mod ui` in the same `main.rs` collide immediately. And it was the name the
    // user saw in their file's `use`, without saying which form it belonged to.
    let arqs = gerar(&documento());
    let main = conteudo(&arqs, "src/main.rs");
    assert!(main.contains("pub mod form_main_designer;"), "{main}");
    assert!(!main.contains("mod ui;"), "{main}");
    let unidade = conteudo(&arqs, "src/form_main.rs");
    assert!(
        unidade.contains("use crate::form_main_designer::*;"),
        "{unidade}"
    );
    assert!(unidade.contains("impl FormMain {"), "{unidade}");
}

#[test]
fn o_app_gerado_nao_abre_console_no_windows() {
    // Without `windows_subsystem`, a Rust binary on Windows gets a console —
    // and the user's application would open with a black rectangle behind its
    // window. Nobody receiving a packaged application wants that.
    //
    // `not(debug_assertions)`: in development the console REMAINS, because that
    // is where `panic!` and `println!` go. It vanishes in release builds, which is
    // what gets delivered.
    let arqs = gerar(&documento());
    let main = conteudo(&arqs, "src/main.rs");
    assert!(
        main.contains(r#"#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]"#),
        "the generated app would open a console behind the window:\n{main}"
    );
}

#[test]
fn the_generated_app_reports_a_startup_failure_instead_of_vanishing() {
    // With no console in release, `fn main() -> eframe::Result` sent the
    // error to a stderr nobody reads and the program just did not open.
    let arqs = gerar(&documento());
    let main = conteudo(&arqs, "src/main.rs");
    assert!(
        main.contains("fn main() -> std::process::ExitCode"),
        "main still returns the toolkit's Result:
{main}"
    );
    assert!(
        main.contains("crawrad_runtime::report_startup_failure("),
        "the startup error is not shown to anyone:
{main}"
    );
}

// ------------------------------------------------------------------- Timer

/// Form with a `Timer` — non-visual, yet emitted.
fn com_timer(intervalo: i64, ligado: bool, com_handler: bool) -> Document {
    let mut d = documento();
    let mut t = Node::new("tmr1", "Timer");
    t.set_name("tmr_relogio");
    t.set_rect(Rect::new(440.0, 8.0, 28.0, 28.0));
    t.props.insert("interval".into(), Value::Int(intervalo));
    t.props.insert("enabled".into(), Value::Bool(ligado));
    if com_handler {
        t.events
            .insert("on_timer".into(), "tmr_relogioTimer".into());
    }
    d.add_child("form_main", t).unwrap();
    d.validate().unwrap();
    d
}

#[test]
fn o_timer_vira_estado_e_o_estado_nao_tem_visible() {
    let arqs = gerar(&com_timer(250, true, true));
    let ui = conteudo(&arqs, "src/form_main.designer.rs");

    assert!(ui.contains("    pub tmr_relogio: Timer,\n"), "{ui}");
    assert!(
        ui.contains("tmr_relogio: Timer { enabled: true, interval: 250, next_due: None },"),
        "{ui}"
    );
    // Flattening in ADR 0010 mirrors what the component HAS. A `Timer` has
    // no window, hence no `Visible` — and the emitted type cannot invent
    // a field that the inspector does not show.
    let tipo = ui
        .split("pub struct Timer {")
        .nth(1)
        .expect("type Timer should have been emitted");
    let corpo = tipo.split('}').next().unwrap();
    assert!(corpo.contains("pub enabled: bool"), "{corpo}");
    assert!(corpo.contains("pub interval: u64"), "{corpo}");
    assert!(corpo.contains("pub next_due: Option<f64>"), "{corpo}");
    assert!(
        !corpo.contains("visible"),
        "components without windows have nothing to hide:\n{corpo}"
    );
}

#[test]
fn o_timer_nao_ganha_retangulo_nem_ancora() {
    // It does not draw: emitting `anchor(...)` would produce an unused `let r` —
    // a `cargo` warning inside a file the user cannot fix — and
    // an `Anchors` meaningless for something without screen bounds.
    let arqs = gerar(&com_timer(250, true, true));
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    let trecho = ui
        .split("// tmr_relogio — Timer")
        .nth(1)
        .expect("the timer block");
    let bloco = trecho.split("\n        if self.").next().unwrap_or(trecho);
    assert!(
        !bloco.contains("anchor(origin"),
        "the timer received a rectangle:\n{bloco}"
    );
    assert!(
        !bloco.contains("Anchors {"),
        "the timer received anchors:\n{bloco}"
    );
}

#[test]
fn o_timer_pede_o_proximo_quadro_e_chama_o_handler() {
    // **The point separating egui from a classic RAD IDE.** Without `request_repaint_after`, an
    // idle application never repaints, and a timer that does not paint never
    // fires. In a classic RAD IDE, Windows wakes the program.
    let arqs = gerar(&com_timer(250, true, true));
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(
        ui.contains("request_repaint_after(timer_wait(now, self.tmr_relogio.next_due))"),
        "{ui}"
    );
    // And time comes from the CLOCK, not frame counts.
    assert!(ui.contains("let now = ui.input(|i| i.time);"), "{ui}");
    assert!(
        ui.contains("timer_due(now, self.tmr_relogio.interval, &mut self.tmr_relogio.next_due)"),
        "{ui}"
    );
    assert!(ui.contains("self.tmr_relogioTimer();"), "{ui}");

    // And the stub is generated in the user's file, like for any event.
    let unidade = conteudo(&arqs, "src/form_main.rs");
    assert!(
        unidade.contains("pub fn tmr_relogioTimer(&mut self)"),
        "{unidade}"
    );
}

#[test]
fn a_regra_do_acumulo_sai_escrita_no_codigo_gerado() {
    // The design choice — late ticks are DROPPED, not queued — belongs to the generator,
    // and whoever reads their own application code should find the rationale there, not in an ADR
    // they don't possess.
    let arqs = gerar(&com_timer(250, true, true));
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(
        ui.contains("Late ticks are dropped, never queued."),
        "the accumulation rule is not explained in generated code:\n{ui}"
    );
    // And its implementation: the deadline recalculated starting from NOW.
    assert!(ui.contains("*due = Some(now + interval);"), "{ui}");
}

#[test]
fn sem_timer_no_formulario_nada_de_temporizador_entra() {
    // Dead functions in generated files produce `cargo` warnings the user cannot
    // fix — the same rule governing `GroupBox` frames and tab strips.
    let arqs = gerar(&documento());
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(!ui.contains("fn timer_due"), "{ui}");
    assert!(!ui.contains("fn timer_wait"), "{ui}");
    assert!(!ui.contains("pub struct Timer"), "{ui}");
}

#[test]
fn timer_sem_handler_compila_sem_variavel_perdida() {
    let arqs = gerar(&com_timer(250, true, false));
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(ui.contains("let _ = fired;"), "{ui}");
    assert!(!ui.contains("Timer();"), "{ui}");
}

#[test]
fn intervalo_zero_ou_negativo_e_timer_parado() {
    // In a classic RAD IDE, `Interval = 0` is NOT a short interval: `UpdateTimer` only
    // calls `SetTimer` with `FInterval > 0`, and the timer remains stopped. Negative
    // does not even exist there; here the field is `u64`, and the honest answer is the same.
    for i in [0, -1] {
        let arqs = gerar(&com_timer(i, true, true));
        let ui = conteudo(&arqs, "src/form_main.designer.rs");
        assert!(
            ui.contains("interval: 0, next_due: None"),
            "interval {i} should have become 0:\n{ui}"
        );
        // And the emitted guard is what keeps it truly stopped.
        assert!(
            ui.contains("if self.tmr_relogio.enabled && self.tmr_relogio.interval > 0 {"),
            "{ui}"
        );
    }
}

#[test]
fn o_painel_leva_a_espessura_da_moldura_para_o_aplicativo() {
    // `border_width` was declared in the catalogue and nobody read it: neither RXP
    // nor generator — and the key was neither in the common core nor namespaced,
    // so every panel dropped from the palette failed `validate()`.
    // Now it is state, as required by ADR 0010: the running application can thicken the
    // frame.
    let mut d = documento();
    let mut pnl = Node::new("pnl1", "Panel");
    pnl.set_name("pnl_dados");
    pnl.set_rect(Rect::new(20.0, 160.0, 200.0, 120.0));
    pnl.props.insert("border_width".into(), Value::Int(3));
    d.add_child("form_main", pnl).unwrap();
    d.validate().unwrap();

    let arqs = gerar(&d);
    let ui = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(ui.contains("border_width: 3.0"), "{ui}");
    assert!(ui.contains("pub border_width: f32,"), "{ui}");
    assert!(
        ui.contains("self.pnl_dados.border_width,"),
        "the emitted frame ignored the document border width:\n{ui}"
    );
}

#[test]
fn nome_reservado_pelo_rust_para_o_futuro_tambem_e_recusado() {
    // The list had keywords IN USE rather than those reserved *for future
    // use*. They do nothing today yet cannot be identifiers — which
    // is why they are unguessable: anyone writing the list from memory
    // recalls `type` but forgets `box`.
    //
    // The hole was reachable via Object Inspector. A `Name` of `box` passed
    // validation, became a field in `.designer.rs`, and the generated project
    // failed to compile — with errors pointing at a file the user never wrote.
    //
    // This test takes the real execution path, not calling `identificador()`
    // directly: what matters is not the list's existence, but that `gerar()` STOPS. A
    // guard that nobody invokes guards nothing (rule 9 of the study).
    for palavra in [
        "box", "do", "macro", "priv", "typeof", "virtual", "yield", "gen", "become",
    ] {
        let mut d = Document::new("form_main", "Form");
        {
            let raiz = d.node_mut("form_main").unwrap();
            raiz.set_rect(Rect::new(0.0, 0.0, 480.0, 320.0));
            raiz.set_name("form_main");
        }
        let mut b = Node::new("b1", "Button");
        b.set_name(palavra);
        b.set_rect(Rect::new(8.0, 8.0, 96.0, 32.0));
        d.add_child("form_main", b).unwrap();

        assert!(
            GeradorRxp
                .gerar(&d, "meu_app", &crawrad_registry::Registry::builtin())
                .is_err(),
            "`{palavra}` is reserved by Rust and passed: the generated project would not \
             compile, and the error would point to a file the user never wrote"
        );
    }

    // And the contrast: common names keep passing. Without this, a `gerar()` that
    // rejected everything would pass this test.
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").unwrap();
        raiz.set_rect(Rect::new(0.0, 0.0, 480.0, 320.0));
        raiz.set_name("form_main");
    }
    let mut b = Node::new("b1", "Button");
    b.set_name("btn_gravar");
    b.set_rect(Rect::new(8.0, 8.0, 96.0, 32.0));
    d.add_child("form_main", b).unwrap();
    assert!(
        GeradorRxp
            .gerar(&d, "meu_app", &crawrad_registry::Registry::builtin())
            .is_ok()
    );
}

#[test]
fn generator_infrastructure_method_names_are_rejected() {
    for name in ["new", "show"] {
        assert!(
            identificador(name).is_err(),
            "`{name}` is reserved by generator and must be rejected"
        );

        let mut d = Document::new("form_main", "Form");
        {
            let root = d.node_mut("form_main").unwrap();
            root.set_rect(Rect::new(0.0, 0.0, 480.0, 320.0));
            root.set_name("form_main");
        }
        let mut b = Node::new("b1", "Button");
        b.set_name(name);
        b.set_rect(Rect::new(8.0, 8.0, 96.0, 32.0));
        d.add_child("form_main", b).unwrap();

        assert!(
            GeradorRxp
                .gerar(&d, "my_app", &crawrad_registry::Registry::builtin())
                .is_err(),
            "`{name}` is reserved by generator and should fail codegen"
        );
    }
}

// ---------------------------------------------------- empty handler pruning
//
// The other half of a classic RAD IDE's contract: bound events produce methods, and
// UNBOUND events remove the method again — but only as long as it is the method
// the IDE wrote and nobody filled in. The rule and the objection heard
// before adopting it are documented in `rxp::podar`.
//
// All tests below verify BEHAVIOR: they write to the file,
// generate again, and inspect disk contents.

/// A clean working directory with the project already generated once.
fn projeto_gerado(sufixo: &str) -> std::path::PathBuf {
    let raiz = std::env::temp_dir().join(format!("crawrad_codegen_poda_{sufixo}"));
    let _ = std::fs::remove_dir_all(&raiz);
    let d = documento();
    escrever(&raiz, &gerar(&d)).expect("first generation");
    raiz
}

/// The document from `documento()` with the button's `on_click` removed — the action
/// of clearing the `OnClick` field in the Object Inspector.
fn sem_o_clique() -> Document {
    let mut d = documento();
    d.node_mut("btn1").unwrap().events.remove("on_click");
    d.validate().unwrap();
    d
}

fn unidade(raiz: &std::path::Path) -> String {
    std::fs::read_to_string(raiz.join("src/form_main.rs")).expect("the user unit")
}

#[test]
fn handler_de_evento_vazio_some_quando_o_evento_e_desligado() {
    // PROOF 2, which matches the video: double click created the handler, the
    // user changed their mind, unbound the event — and the file reverts to what it was.
    let raiz = projeto_gerado("some");
    assert!(unidade(&raiz).contains("pub fn btn_okClick"));

    let r = escrever(&raiz, &gerar(&sem_o_clique())).expect("second generation");
    let depois = unidade(&raiz);

    assert!(
        !depois.contains("btn_okClick"),
        "the empty handler of the unbound event stayed on disk:\n{depois}"
    );
    assert_eq!(r.podados, vec!["btn_okClick"]);
    // The status bar REPORTS. Deleting silently is what frightens in a classic RAD IDE.
    assert_eq!(
        r.frase_da_poda().as_deref(),
        Some("1 empty handler removed")
    );
    // The file shrank, so it is NOT "preserved" — the word must
    // continue to mean intact content.
    assert!(
        !r.preservados
            .contains(&std::path::PathBuf::from("src/form_main.rs"))
    );

    // And its neighbor, still bound, remains where it was.
    assert!(
        depois.contains("pub fn edt_nomeChange"),
        "pruning removed the handler of the event that remains bound:\n{depois}"
    );

    // What remains is EXACTLY the file that would have been generated without that event:
    // no extra blank lines, no two `fn`s stuck together. This is what makes pruning
    // reversible — rebinding the event restores the previous file.
    let virgem = std::env::temp_dir().join("crawrad_codegen_poda_some_virgem");
    let _ = std::fs::remove_dir_all(&virgem);
    escrever(&virgem, &gerar(&sem_o_clique())).expect("generation from scratch");
    assert_eq!(
        depois,
        std::fs::read_to_string(virgem.join("src/form_main.rs")).unwrap(),
        "the pruned file differed from what would be generated without the event"
    );
}

#[test]
fn handler_com_codigo_do_usuario_nunca_some() {
    // PROOF 3, and the most important of the three: ATTACKING OUR OWN
    // IMPLEMENTATION. Each body here is an attempt to make pruning delete
    // human work, and each must survive unbinding the event.
    //
    // The first four are boundaries checked by a classic RAD IDE — and three of them
    // a classic RAD IDE GETS WRONG by deleting. Here the rule is "is the body empty?",
    // where empty means only whitespace plus text the generator ITSELF
    // wrote. Anything typed makes the body non-empty.
    // Each case is what the user TYPES on the line where the cursor landed —
    // without indentation and without line breaks, as both already exist in the file. Since
    // 08/09/2026 this line starts blank: there is no longer our message for
    // the test to overwrite, and the tested gesture became the actual gesture.
    let casos: &[(&str, &str)] = &[
        // The twenty-year idiom of a classic RAD IDE — there `{ }` exists precisely to
        // trick cleanup. Here nobody needs tricking: comments are
        // text someone typed, and typed text stays.
        ("user comment", "// deixar quieto por ora"),
        // The specialist did not know the classic RAD IDE's answer and asked to test it. Here
        // there is nothing to measure: it is code.
        ("unused let", "let _pendente = 1;"),
        // A real call.
        ("code", "self.edt_nome.text.clear();"),
        // ONE character, and not even a comment: `;` alone is an empty
        // statement that compiles. The body is now pure whitespace, so
        // this is the smallest gesture that stops it from being empty.
        ("single character", ";"),
        // What looks most like OUR text without being ours: a `TODO` on the same
        // line and in the same position as the warning written for unfired
        // events. `linha_do_gerador` compares full text, so
        // this is someone's typing.
        ("TODO resembling ours", "// TODO: ask the maintainer"),
    ];

    for (nome, corpo) in casos {
        let raiz = projeto_gerado(&format!("fica_{}", corpo.len()));
        let antes = unidade(&raiz);
        let escrito = digitar(&antes, "btn_okClick", corpo);
        assert_ne!(escrito, antes, "case {nome} was not written");
        std::fs::write(raiz.join("src/form_main.rs"), &escrito).unwrap();

        let r = escrever(&raiz, &gerar(&sem_o_clique())).expect("generation");
        let depois = unidade(&raiz);
        assert!(
            depois.contains("pub fn btn_okClick"),
            "pruning deleted the handler containing {nome}:\n{depois}"
        );
        assert!(
            depois.contains(&format!("        {corpo}")),
            "pruning altered the body with {nome}, or its indentation:\n{depois}"
        );
        assert!(r.podados.is_empty(), "the report lied in case {nome}");
        assert_eq!(r.frase_da_poda(), None);
    }
}

#[test]
fn metodo_comum_vazio_escrito_a_mao_fica_sempre() {
    // PROOF 1, and the condition that reduces blast radius to near zero:
    // a `fn` that was NEVER an event handler is invisible to pruning.
    //
    // In a classic RAD IDE, manually created empty methods are not cleaned up —
    // only empty EVENT methods are. The survey marked this point as UNCERTAIN;
    // long use of that IDE answered it.
    //
    // The three cases are the same empty `fn` in different disguises, and the
    // last is the worst, and got even worse after 08/09/2026: it is the stub
    // BYTE FOR BYTE — handler name and the same indented blank line written
    // by the generator — missing only the event marker on top. And it stays,
    // because the marker proves provenance, and it is not there.
    let raiz = projeto_gerado("comum");
    let antes = unidade(&raiz);
    let escrito = antes.replace(
        "    // New event handlers go here.\n",
        "    pub fn recalcular(&mut self) {}\n\
         \n    pub fn limpar(&mut self) {\n    }\n\
         \n    pub fn btn_faltaClick(&mut self) {\n        \n    }\n\
         \n    // New event handlers go here.\n",
    );
    assert_ne!(escrito, antes);
    std::fs::write(raiz.join("src/form_main.rs"), &escrito).unwrap();

    let r = escrever(&raiz, &gerar(&sem_o_clique())).expect("generation");
    let depois = unidade(&raiz);

    for n in ["recalcular", "limpar", "btn_faltaClick"] {
        assert!(
            depois.contains(&format!("pub fn {n}")),
            "pruning deleted `{n}`, which nobody created with a double click:\n{depois}"
        );
    }
    // And the genuine event handler does get removed — otherwise this test would
    // pass with pruning that does nothing.
    assert_eq!(r.podados, vec!["btn_okClick"]);
}

#[test]
fn o_doc_comment_do_usuario_nao_e_a_nossa_marca() {
    // THE MISSING AXIS, and it cost a REJECT. The test above covers copied
    // BODY; this covers the DOC-COMMENT — the line above, where the
    // proof of origin lives.
    //
    // The first pruning version only checked the SHAPE of the marker: a backticked
    // token with exactly one dot. That is ordinary Rust doc-comment syntax, and the
    // product deleted the method of whoever wrote it. Tested on 07/09/2026 in an
    // ORDINARY generation — event bound, nothing unbound, the same `Generate` triggered
    // automatically by double-clicking.
    //
    // Each case is an empty `fn` with a doc-comment above it, and none of them
    // may be touched:
    let casos: &[(&str, &str, &str)] = &[
        // A file path. `Cargo.toml`, `app.rs`, `self.total`:
        // anything with a dot has the shape of the marker. This is the case
        // measured by the reviewer, which was being deleted.
        ("file path", "config.toml", "carregar_config"),
        // THE NAME DERIVES. `pedido` + `total` in CamelCase is `pedidoTotal`, which
        // is what this `fn` is called — the name check, by itself, would say yes.
        // What holds it back is the second latch: `total` is nobody's event.
        // This case verifies that the event list adds protection.
        (
            "the pair becomes the name itself",
            "pedido.total",
            "pedidoTotal",
        ),
        // The worst of all: the name is that of a REAL handler, and the event
        // is almost ours. The IR key is `on_click`, and `click` is not in the
        // list — so the marker could not have come from this generator.
        ("event without `on_`", "btn_ok.click", "btn_okClick2"),
    ];

    for (nome, marca, f) in casos {
        let raiz = projeto_gerado(&format!("doc_{}", marca.len()));
        let antes = unidade(&raiz);
        let meu = format!("    /// `{marca}`\n    pub fn {f}(&mut self) {{\n    }}\n\n");
        let escrito = antes.replace(
            "    // New event handlers go here.\n",
            &format!("{meu}    // New event handlers go here.\n"),
        );
        assert_ne!(escrito, antes, "case {nome} was not written");
        std::fs::write(raiz.join("src/form_main.rs"), &escrito).unwrap();

        // ORDINARY generation: the document is identical, with both events bound.
        let r = escrever(&raiz, &gerar(&documento())).expect("generation");
        let depois = unidade(&raiz);
        assert!(
            depois.contains(&format!("pub fn {f}")),
            "pruning deleted `{f}`, written by the user — their doc-comment \
             had the shape of our marker ({nome}):\n{depois}"
        );
        assert!(
            r.podados.is_empty(),
            "the report claimed to prune {:?} in case {nome}",
            r.podados
        );
    }
}

#[test]
fn o_handler_renomeado_a_mao_deixa_de_ser_nosso() {
    // The user renames the `fn` in the editor and the marker remains above it — the
    // link broke, and ADR 0015 says the resolution is genuine refactoring,
    // not guessing. What pruning MUST NOT do is mistake `salvar_pedido`
    // for its own stub just because our marker stayed there.
    //
    // The renamed `fn` does not even need to be unbound to be at risk: with
    // shape checking, the document pointed to `btn_okClick`, and the name
    // left in the file (`salvar_pedido`) was not among bound handlers. Orphaned,
    // marked, empty: it was being pruned during normal generation.
    let raiz = projeto_gerado("renomeado");
    let antes = unidade(&raiz);
    let escrito = antes.replace("pub fn btn_okClick", "pub fn salvar_pedido");
    assert_ne!(escrito, antes);
    std::fs::write(raiz.join("src/form_main.rs"), &escrito).unwrap();

    let r = escrever(&raiz, &gerar(&documento())).expect("generation");
    let depois = unidade(&raiz);
    assert!(
        depois.contains("pub fn salvar_pedido"),
        "pruning deleted the `fn` that the user renamed manually:\n{depois}"
    );
    assert!(r.podados.is_empty(), "pruned {:?}", r.podados);
}

/// **`event.rename` keeps the code**: the form rebinds
/// `btn_ok.on_click` to a new name, and the body the user wrote follows it;
/// the generated code calls the new name, and no empty stub is added.
#[test]
fn a_handler_renamed_by_the_form_keeps_its_body() {
    let root = projeto_gerado("evento_renomeado");
    let before = unidade(&root);
    let written = before.replace(
        "pub fn btn_okClick(&mut self) {\n",
        "pub fn btn_okClick(&mut self) {\n        let kept = 1;\n",
    );
    assert_ne!(written, before);
    std::fs::write(root.join("src/form_main.rs"), &written).unwrap();

    let mut d = documento();
    d.node_mut("btn1")
        .unwrap()
        .events
        .insert("on_click".into(), "save_order".into());
    escrever(&root, &gerar(&d)).expect("the generation");
    let after = unidade(&root);
    assert!(
        after.contains("pub fn save_order(&mut self) {\n        let kept = 1;"),
        "{after}"
    );
    assert!(!after.contains("btn_okClick"), "{after}");
    let generated = std::fs::read_to_string(root.join("src/form_main.designer.rs")).unwrap();
    assert!(generated.contains("save_order()") && !generated.contains("btn_okClick"));
}

/// Inserts `acima` on the line above `btn_ok.on_click` marker in the already
/// generated file — the gesture of someone documenting the handler before writing it.
fn escrever_acima_da_marca(raiz: &std::path::Path, acima: &str) -> String {
    let antes = unidade(raiz);
    let escrito = antes.replace(
        "    /// `btn_ok.on_click`\n",
        &format!("{acima}    /// `btn_ok.on_click`\n"),
    );
    assert_ne!(escrito, antes, "the text above was not written");
    std::fs::write(raiz.join("src/form_main.rs"), &escrito).unwrap();
    escrito
}

#[test]
fn o_que_o_usuario_escreve_acima_da_marca_segura_a_poda() {
    // THE MISSING AXIS, and it cost the third REJECT. Other tests
    // cover BODY and MARKER; this covers what lies ABOVE the marker.
    //
    // Slicing goes from marker to `}`, and the marker is its first line. What
    // the user wrote above it is not removed — and doc-comments and
    // attributes ATTACH to the item below. Removing the `fn` beneath them leaves
    // them orphaned and **the user's project stops compiling**. Measured on
    // 07/09/2026 with `rustc --edition 2021 --crate-type lib` on the file
    // left by pruning:
    //
    //     error[E0584]: found a documentation comment that doesn't document
    //                   anything
    //     error: expected item after attributes
    //
    // This is the only way this cleanup could break a build, and ADR 0025 dictates
    // that it never may. So it refuses: an empty `fn` remains, which is the cost
    // this decision already pays in three other places.
    let casos: &[(&str, &str)] = &[
        // Idiomatic rustdoc pattern: summary line first, marker
        // underneath, looking like metadata. That was how the defect surfaced.
        (
            "single line of rustdoc",
            "    /// Saves the order to the database.\n",
        ),
        // Same, sized like real documentation.
        (
            "a block of rustdoc",
            "    /// Saves the order to the database.\n\
             \x20   ///\n\
             \x20   /// Rolls back when the total does not match.\n",
        ),
        // Rust's other doc syntax. It also triggers E0584.
        ("block rustdoc", "    /** Saves the order. */\n"),
        // The same thing across three lines: the line above the marker is ` */`,
        // which neither opens nor closes anything on its own.
        (
            "three-line block rustdoc",
            "    /**\n\x20     * Saves the order.\n\x20     */\n",
        ),
        // An attribute is not a comment and breaks with another message — this case
        // was not in the review report, but emerged during testing.
        ("an attribute", "    #[allow(dead_code)]\n"),
        // Multi-line attribute: the line above is `)]`, which also needs
        // to stop pruning.
        (
            "a three-line attribute",
            "    #[allow(\n\x20       dead_code\n\x20   )]\n",
        ),
        // THE COST OF THIS DECISION PRODUCES THIS CASE, and it cost the fourth
        // REJECT. An empty `fn` remains and `cargo` warns; anyone's reaction
        // to that warning is writing `#[allow(dead_code)]` above the
        // handler — sometimes with a rationale comment on the same line.
        // The check used to read the ENTIRE LINE: the attribute ends in `]`, but the
        // line ends in `;`, pruning proceeded, and the user's file broke with
        // `error: expected item after attributes`.
        (
            "an attribute with attached comment",
            "    #[allow(dead_code)] // vira self.salvar();\n",
        ),
        // The other two tails of the same comment. If the check goes back to
        // looking at the entire line, each accepted suffix opens a backdoor.
        (
            "an attribute with comment opening a block",
            "    #[allow(dead_code)] // abre o bloco {\n",
        ),
        (
            "an attribute with comment closing a block",
            "    #[allow(dead_code)] // fecha }\n",
        ),
        // THE SAME trap one line further, and it doesn't start with `#`: in a
        // multi-line attribute, what sits above the marker is `)]`. Tested
        // with `rustc` on 07/09/2026 — rejecting via `starts_with('#')` lets this
        // pass, and the resulting file fails to compile.
        (
            "a three-line attribute with attached comment at closing",
            "    #[allow(\n\x20       dead_code\n\x20   )] // vira self.salvar();\n",
        ),
        // Block comment between attribute and line comment: neither is an item,
        // and the response must be identical.
        (
            "an attribute with block and line comments",
            "    #[allow(dead_code)] /* por ora */ // vira self.salvar();\n",
        ),
        // THIS CASE IS WHAT JUSTIFIES THE `#` TERM, ensuring the term is
        // not a shadowed guard: every attribute on its own line already ends in
        // `]` and would be rejected without it, and every attribute with an attached comment
        // already hits `//`. `#` only decides here — attribute and item on the SAME
        // line, with no `//` anywhere, where the item's `}` would close and pruning would proceed.
        // Pruning would be safe (the attribute belongs to the co-located item), but the
        // uniform response for user metadata lines is to reject, which is what is recorded.
        // Removing this term turns THIS case, and only this case, red.
        (
            "an attribute with method on the same line",
            "    #[inline] pub fn meu(&mut self) {}\n",
        ),
        // And this justifies the `/` term for the same reason, in sister format: a
        // BLOCK comment and item on the same line — and block comments
        // do not have `//`, so the first term does not catch them. Without `/`, the
        // method's `}` would close and pruning would proceed. Both user metadata
        // lines receive the same response, which makes the rule consistent.
        // Removing this term turns THIS case, and only this case, red.
        (
            "a block comment with method on the same line",
            "    /* nota */ pub fn meu(&mut self) {}\n",
        ),
        // A blank line DOES NOT save it: in Rust doc-comments span blank
        // lines and stay attached to the item below. Therefore the check
        // skips blank lines instead of only looking at the line immediately above.
        (
            "rustdoc with a blank line in between",
            "    /// Saves the order to the database.\n\n",
        ),
        // Ordinary comments COMPILE even if left orphaned — yet pruning
        // refuses anyway. It is text someone typed for that handler, and it is not
        // the generator's place to decide it now refers to the next handler.
        ("an ordinary comment", "    // check with the maintainer\n"),
        // The two below end like CODE ends, which is what makes them
        // interesting: whoever ports from a classic RAD IDE documents the equivalent
        // Pascal method, and in Pascal lines end in `;`. Relying only on suffixes would say
        // "this closes what came before", and pruning would erase the `fn` beneath
        // the user's text.
        (
            "rustdoc ending in semicolon",
            "    /// Espelha o Button1Click; da unit original.\n",
        ),
        (
            "ordinary comment ending in semicolon",
            "    // um dia isto vira self.salvar();\n",
        ),
        // THE FOUR BELOW ARE THE FIFTH REJECT, and all represent the same thing: a
        // `//` that **opens no comment at all**, because the line is already inside
        // a block comment or a string opened above. Knowing that would require
        // a lexer across all preceding code; our heuristic does not know, so
        // it rejects every line with `//`, whatever it may be. Tested on
        // 08/09/2026 with `rustc --edition 2021 --crate-type lib`.
        (
            "block rustdoc whose closing shares the line with a //",
            "    /** Salva o pedido.\n\x20       Um dia vira self.salvar(); // */\n",
        ),
        (
            "attribute with block comment closing after a //",
            "    #[allow(dead_code)] /* por ora\n\x20       vira self.salvar(); // */\n",
        ),
        // This contains no comment at all: `//` is inside a multi-line
        // STRING, and the line above the marker is `{// x"]`.
        (
            "attribute with multi-line string",
            "    #[doc = \"linha\n{// x\"]\n",
        ),
        // The fourth shape, reasoned out in review before measuring: the closing of a block
        // rustdoc with an attached comment. It starts with `*`, escapes both
        // `starts_with`, and ends with `;` — only `//` rejects it.
        (
            "block rustdoc closing with attached comment",
            "    /** Salva o pedido.\n\x20    */ // vira self.salvar();\n",
        ),
        // THE TWO BELOW WERE CONTRAST CASES, and were moved here in the fifth
        // REJECT. They genuinely close the preceding item and only carry a
        // comment along — pruning them would be safe. The check rejects them
        // anyway, because the only way to know that `//` really opens a
        // comment is a lexer across all prior text; without it, the four
        // cases above are indistinguishable from these two. Pruning backs off
        // when uncertain, and the cost is the usual one: an empty `fn` remains.
        (
            "the `}` of another method with attached comment",
            "    pub fn meu(&mut self) {\n        self.x();\n    } // fim\n",
        ),
        (
            "a constant with attached comment",
            "    const LIMITE: u8 = 10; // o teto\n",
        ),
        // THE FIFTH FORM, and it is the SIXTH REJECT. Here the line genuinely
        // closes the preceding item — the `const`'s `;` is there, and suffix checks
        // say yes under any reading — yet it still leaves a dangling item: the
        // `///` attached to the RIGHT. It is the same `/// Saves the order.` from earlier
        // lines, but positioned where no suffix inspection sees it.
        // Measured on 08/09/2026 with `examples/gerar` on a real `.crawform` and
        // `rustc --edition 2021 --crate-type lib`: the user's file
        // **compiled before** pruning and returned `error[E0584]` afterwards.
        //
        // And there is the silent version, which is worse: when the pruned handler is not
        // the last, the user's doc ends up documenting ANOTHER method and the file
        // compiles while telling a lie.
        //
        // THIS CASE IS ONE OF NINE JUSTIFYING THE `//` TERM, and is what it
        // exists to catch: removing `!t.contains("//")` turns red the
        // nine cases only it rejects — this one, its three tails, `)] // …;`,
        // `*/ // …;`, the two trailing `// …;`, and `"http://x"`. None of the
        // other three terms reach them.
        (
            "a constant with rustdoc attached on the right",
            "    const LIMITE: u8 = 10; /// vira self.salvar();\n",
        ),
        // The same form on the other suffix, showing the margins: the table already
        // recorded `} // fim` as a rejection, and this differs by ONE character —
        // the gap between text and code, and text won.
        (
            "the `}` of another method with rustdoc attached on the right",
            "    pub fn meu(&mut self) {\n        self.x();\n    } /// documenta o próximo;\n",
        ),
        // The two tails of the same right-attached rustdoc, for the same reason
        // as the tails of `// abre {` above: each accepted suffix opens a door.
        (
            "a constant with attached rustdoc closing a block",
            "    const LIMITE: u8 = 10; /// veja Foo {}\n",
        ),
        (
            "a constant with attached rustdoc opening a block",
            "    const LIMITE: u8 = 10; /// o teto de itens {\n",
        ),
        // THE TWO BELOW ARE THE GAP BETWEEN TEXT AND CODE, measured in the
        // review of the fifth REJECT: docs for this function and ADR 0025 stated that
        // `} // fim` and `const … ; // o teto` were rejected, and indeed they were — but
        // simply appending `;` to the comment caused both to PRUNE, because
        // reading stripped code matched a terminator. It was that gap that
        // permitted the fifth form. Now the rule matches what was documented.
        (
            "the `}` of another method with attached comment ending in `;`",
            "    pub fn meu(&mut self) {\n        self.x();\n    } // fim;\n",
        ),
        (
            "a constant with attached comment ending in `;`",
            "    const LIMITE: u8 = 10; // o teto;\n",
        ),
        // And the `//` that is no comment at all: it lives inside a single-line
        // string. The line genuinely terminates and pruning backs off — same
        // response, same reason, recorded in docs as a deliberate rejection.
        (
            "a constant whose value contains `//`",
            "    const URL: &str = \"http://x\";\n",
        ),
    ];

    // Directory suffix is the INDEX, not text length: with a table of
    // this size, two cases of the same length would share the same
    // root, and one would delete the other's project mid-run.
    for (i, (nome, acima)) in casos.iter().enumerate() {
        let raiz = projeto_gerado(&format!("acima_{i}"));
        let escrito = escrever_acima_da_marca(&raiz, acima);

        let r = escrever(&raiz, &gerar(&sem_o_clique())).expect("generation");
        let depois = unidade(&raiz);
        assert_eq!(
            depois, escrito,
            "pruning altered the file having {nome} above the marker"
        );
        assert!(
            r.podados.is_empty(),
            "pruning removed {:?} and left {nome} dangling — the user \
             file no longer compiles",
            r.podados
        );
        assert_eq!(r.frase_da_poda(), None);
    }
}

/// Document with the button renamed separately from the handler — which is the state
/// in which `Command::Rename` (`crates/crawrad-doc/src/command.rs`) leaves the node: it
/// changes `name` and **does not touch `node.events`**.
fn com_o_botao_renomeado() -> Document {
    let mut d = documento();
    d.node_mut("btn1").unwrap().set_name("btn_salvar");
    d.validate().unwrap();
    d
}

#[test]
fn renomear_o_componente_depois_de_gerar_nao_atrapalha_a_poda() {
    // THE GOOD HALF of the rename edge case, needing testing because code
    // inspection suggested otherwise: `stubs` selects markers via
    // `handler_padrao(nome_atual, evento)`, and with the button now called
    // `btn_salvar` it would be `btn_salvar.on_click` over `btn_okClick`
    // — a `fn` the generator would no longer recognize as its own.
    //
    // That is not what happens, due to the project invariant: user files
    // only GROW. The marker is already on disk from the first generation, and the
    // second rewrites nothing — it would only insert if the `fn`
    // were missing. Marker and name age together, and pruning keeps behaving
    // the same.
    let raiz = projeto_gerado("renomeou_depois");
    assert!(unidade(&raiz).contains("/// `btn_ok.on_click`"));

    let mut d = com_o_botao_renomeado();
    escrever(&raiz, &gerar(&d)).expect("generation after rename");
    let depois = unidade(&raiz);
    assert!(
        depois.contains("/// `btn_ok.on_click`"),
        "rename rewrote the marker in user file:\n{depois}"
    );

    // And the gesture that matters: unbinding the event still removes the handler.
    d.node_mut("btn1").unwrap().events.remove("on_click");
    d.validate().unwrap();
    let r = escrever(&raiz, &gerar(&d)).expect("generation without click");
    assert_eq!(
        r.podados,
        vec!["btn_okClick"],
        "renaming the component blinded pruning:\n{}",
        unidade(&raiz)
    );
}

#[test]
fn renomear_o_componente_antes_de_gerar_deixa_o_handler_para_sempre() {
    // THE LOST SLICE, and it is larger than ADR 0025 stated. The table
    // said "handler custom-named in Inspector"; this one was missing, where the
    // user **named nothing manually** — accepted the default name and later
    // renamed the component.
    //
    // Order of operations decides, which is why there are two tests. Binding an event in
    // Object Inspector DOES NOT generate (field `event.bind` only mutates the document;
    // generation happens via `Generate`). So between the two gestures fits:
    // bind `OnClick`, rename button, generate.
    //
    // The stub is born with the NEW name's marker over the OLD handler
    // name, and the marker does not derive the `fn` name. The generator wrote a `fn`
    // that it no longer recognizes as its own, and it is never removed.
    //
    // The cost is conservative — an empty `fn` and a `cargo` warning —, so
    // this is not a bug to fix here: what would have to migrate the handler is
    // `Command::Rename`, in another crate and another PR. It is in the table of ADR
    // 0025. The day it migrates, this test turns red, and it and the
    // table row exit in the same commit.
    let raiz = std::env::temp_dir().join("crawrad_codegen_poda_renomeou_antes");
    let _ = std::fs::remove_dir_all(&raiz);
    let mut d = com_o_botao_renomeado();
    escrever(&raiz, &gerar(&d)).expect("first generation with rename");

    let nascido = unidade(&raiz);
    assert!(
        nascido.contains("/// `btn_salvar.on_click`") && nascido.contains("pub fn btn_okClick"),
        "the stub had to start with the new name marker over the old \
         handler:\n{nascido}"
    );

    d.node_mut("btn1").unwrap().events.remove("on_click");
    d.validate().unwrap();
    let r = escrever(&raiz, &gerar(&d)).expect("generation without click");
    assert!(
        r.podados.is_empty(),
        "pruning removed a `fn` whose marker does not derive its name: {:?}",
        r.podados
    );
    assert!(
        unidade(&raiz).contains("pub fn btn_okClick"),
        "the empty `fn` remains, which is the safe side of error"
    );
}

#[test]
fn a_poda_segue_valendo_com_o_item_anterior_colado_na_marca() {
    // The contrast to `o_que_o_usuario_escreve_acima_da_marca_segura_a_poda`, and
    // it is mandatory: a check that ALWAYS rejected would pass there and
    // kill the entire cleanup. Here the line above is user code that
    // **terminates what came before** — nothing dangles when the block exits —, and
    // pruning must do its job.
    //
    // The three terminate differently on purpose: `}`, `{}` on a single
    // line, and `;`. Each accepted suffix has a case here.
    let casos: &[(&str, &str)] = &[
        (
            "the `}` of another method",
            "    pub fn meu(&mut self) {\n        self.x();\n    }\n",
        ),
        (
            "single-line empty method",
            "    pub fn vazio(&mut self) {}\n",
        ),
        ("a constant", "    const LIMITE: u8 = 10;\n"),
        // None of the three carries a comment, on purpose: in the fifth REJECT the
        // two that did (`} // fim` and `const … ; // o teto`) moved to the
        // rejection table. See the comment there for why. What remains
        // here is the axis this contrast exists to prove: a check
        // that ALWAYS rejected would pass the test above and kill the entire
        // cleanup. Each accepted suffix has a case, and none relies on stripping
        // comments.
    ];

    for (i, (nome, acima)) in casos.iter().enumerate() {
        let raiz = projeto_gerado(&format!("colado_{i}"));
        escrever_acima_da_marca(&raiz, acima);

        let r = escrever(&raiz, &gerar(&sem_o_clique())).expect("generation");
        let depois = unidade(&raiz);
        assert_eq!(
            r.podados,
            vec!["btn_okClick"],
            "with {nome} adjacent to the marker, pruning gave up on its \
             own block:\n{depois}"
        );
        assert!(
            depois.contains(acima.trim()),
            "pruning deleted {nome}, which belongs to the user:\n{depois}"
        );
    }
}

#[test]
fn o_bloco_do_stub_copiado_vira_codigo_do_usuario() {
    // Copying the entire stub — marker included — to start a helper is the
    // most natural gesture in the world in this file. The helper starts out
    // empty, with OUR marker on top, but it isn't ours.
    let raiz = projeto_gerado("copiado");
    let antes = unidade(&raiz);
    let escrito = antes.replace(
        "    // New event handlers go here.\n",
        "    /// `btn_ok.on_click`\n    pub fn minha_rotina(&mut self) {\n    }\n\n\
         \x20   // New event handlers go here.\n",
    );
    assert_ne!(escrito, antes);
    std::fs::write(raiz.join("src/form_main.rs"), &escrito).unwrap();

    // And here the event is REMOVED, for the test to measure pruning at work: the
    // real `btn_okClick` must be removed, while the helper copying its marker
    // must remain. Both in the same generation run.
    let r = escrever(&raiz, &gerar(&sem_o_clique())).expect("generation");
    let depois = unidade(&raiz);
    assert!(
        depois.contains("pub fn minha_rotina"),
        "pruning deleted the helper that copied the stub block:\n{depois}"
    );
    assert_eq!(
        r.podados,
        vec!["btn_okClick"],
        "the real stub was not removed, so this test measured no pruning"
    );
}

#[test]
fn a_poda_em_arquivo_crlf_nao_deixa_rastro() {
    // The file belongs to the user, and their editor on Windows saves CRLF. Slicing
    // captures the newline preceding the marker — and it comes as a
    // PAIR. Stopping at `\n`, the `\r` was left behind: nothing broke, `rustc`
    // accepted it, and the file accumulated an invisible character per prune.
    let raiz = projeto_gerado("crlf");
    let crlf = unidade(&raiz).replace('\n', "\r\n");
    std::fs::write(raiz.join("src/form_main.rs"), &crlf).unwrap();

    let r = escrever(&raiz, &gerar(&sem_o_clique())).expect("generation");
    let depois = unidade(&raiz);
    assert_eq!(r.podados, vec!["btn_okClick"], "CRLF prevented pruning");
    assert!(
        depois.contains("pub fn edt_nomeChange"),
        "pruning deleted the handler that remains bound:\n{depois}"
    );
    assert!(
        !depois.replace("\r\n", "").contains('\r'),
        "a stray CR remained in user file:\n{}",
        depois.replace('\r', "<CR>")
    );
    // Strong proof: the pruned file is the same file, byte for byte, with the
    // block removed — and nothing else changed shape.
    assert_eq!(
        depois,
        crlf.replace(
            "\r\n    /// `btn_ok.on_click`\r\n    pub fn btn_okClick(&mut self) \
             {\r\n        \r\n    }\r\n",
            ""
        ),
        "pruning touched more than the handler block"
    );
}

#[test]
fn o_handler_chamado_de_outro_arquivo_do_projeto_fica() {
    // The same guard as `handler_vazio_chamado_a_mao_nao_e_removido`, but the
    // call site is in ANOTHER `.rs` — which is the path for `lib.rs::vizinhas`
    // that no test exercised: mutating it to always return empty didn't turn
    // a single test red in the entire suite. Deleting here breaks the build of a project
    // the user didn't even touch.
    let raiz = projeto_gerado("vizinho");
    std::fs::write(
        raiz.join("src/util.rs"),
        "use crate::form_main_designer::FormMain;\n\
         pub fn f(x: &mut FormMain) {\n    x.btn_okClick();\n}\n",
    )
    .unwrap();

    let r = escrever(&raiz, &gerar(&sem_o_clique())).expect("generation");
    let depois = unidade(&raiz);
    assert!(
        depois.contains("pub fn btn_okClick"),
        "pruning deleted a handler called by another project file:\n{depois}"
    );
    assert!(r.podados.is_empty(), "pruned {:?}", r.podados);
}

#[test]
fn o_homonimo_em_outro_impl_segura_a_poda() {
    // A `fn` with the same name across two `impl` blocks in the same file compiles, and creates
    // ambiguity: impossible to know which is the stub. `citado_fora` is what rejects it
    // — the second definition cites the name outside the first's
    // block. See docs in `podar`, which note that the count that seemed
    // active here never actually decided anything.
    let raiz = projeto_gerado("homonimo");
    let escrito = format!(
        "{}\nstruct Outro;\nimpl Outro {{\n    pub fn btn_okClick(&mut self) \
         {{\n        self.contar();\n    }}\n}}\n",
        unidade(&raiz)
    );
    std::fs::write(raiz.join("src/form_main.rs"), &escrito).unwrap();

    let r = escrever(&raiz, &gerar(&sem_o_clique())).expect("generation");
    let depois = unidade(&raiz);
    assert!(r.podados.is_empty(), "pruned {:?}", r.podados);
    assert_eq!(
        depois.matches("pub fn btn_okClick").count(),
        2,
        "pruning broke the homonym tie by guessing:\n{depois}"
    );
}

#[test]
fn dois_handlers_desligados_de_uma_vez_saem_os_dois() {
    // "I unbound everything" is a one-key gesture in Inspector, and is the ONLY
    // path where pruning performs multiple slices in a single pass. Without this
    // test, reversing the order of slices — which makes the second cut fall on
    // shifted text offsets — passed the entire test suite and crashed with
    // `range end index out of range` in the user's hands.
    //
    // And this is the only place where the plural status bar phrase is verified end
    // to end, with two real handlers removed from disk.
    let raiz = projeto_gerado("dois");
    let antes = unidade(&raiz);
    let escrito = antes.replace(
        "    // New event handlers go here.\n",
        "    pub fn meu_metodo(&mut self) {\n        self.x();\n    }\n\n\
         \x20   // New event handlers go here.\n",
    );
    assert_ne!(escrito, antes);
    std::fs::write(raiz.join("src/form_main.rs"), &escrito).unwrap();

    let mut d = sem_o_clique();
    d.node_mut("edt1").unwrap().events.remove("on_change");
    d.validate().unwrap();
    let r = escrever(&raiz, &gerar(&d)).expect("generation");
    let depois = unidade(&raiz);

    assert_eq!(r.podados, vec!["btn_okClick", "edt_nomeChange"]);
    assert_eq!(
        r.frase_da_poda().as_deref(),
        Some("2 empty handlers removed")
    );
    assert!(
        !depois.contains("btn_okClick") && !depois.contains("edt_nomeChange"),
        "handlers remained:\n{depois}"
    );
    assert!(
        depois.contains("self.x();"),
        "the second slice deleted the user method that was between the \
         two:\n{depois}"
    );
}

#[test]
fn o_registro_nao_tem_evento_que_a_poda_desconheca() {
    let reg = crawrad_registry::Registry::builtin();
    let mut do_registro: Vec<&str> = reg
        .all()
        .flat_map(|d| d.events.iter().map(|e| e.name))
        .collect();
    do_registro.sort_unstable();
    do_registro.dedup();

    let eventos = crate::rxp::todos_os_eventos();
    for e in &do_registro {
        assert!(
            eventos.contains(e),
            "the registry declares `{e}` and pruning does not know it: the handler of this \
             event would never leave the user file"
        );
        assert!(
            crate::rxp::eh_evento_conhecido(e),
            "event `{e}` must be recognized as a valid event by pruning"
        );
    }
}

#[test]
fn handler_vazio_chamado_a_mao_nao_e_removido() {
    // The boundary where a classic RAD IDE errs most: it does not analyze calls, deletes anyway, and the
    // unit stops compiling. Here we have the text and the check is cheap.
    //
    // Not a matter of policy — it is the guard preventing pruning from breaking the
    // build. See `elo::citado_fora`.
    let raiz = projeto_gerado("chamado");
    let antes = unidade(&raiz);
    let escrito = digitar(&antes, "edt_nomeChange", "self.btn_okClick();");
    assert_ne!(escrito, antes);
    std::fs::write(raiz.join("src/form_main.rs"), &escrito).unwrap();

    let r = escrever(&raiz, &gerar(&sem_o_clique())).expect("generation");
    let depois = unidade(&raiz);
    assert!(
        depois.contains("pub fn btn_okClick"),
        "pruning deleted a handler that other code calls — the user \
         project would fail to compile:\n{depois}"
    );
    assert!(r.podados.is_empty());
}

#[test]
fn a_citacao_em_comentario_nao_segura_o_handler() {
    // Contrast to the test above, and it is necessary: a guard inspecting
    // raw text would see `btn_okClick` in a comment and never prune
    // anything. Scanning uses MASKED source, for the usual reasons.
    let raiz = projeto_gerado("citado_em_comentario");
    let antes = unidade(&raiz);
    let escrito = digitar(
        &antes,
        "edt_nomeChange",
        "// um dia chamar btn_okClick daqui\n        let _ = \"btn_okClick\";",
    );
    assert_ne!(escrito, antes);
    std::fs::write(raiz.join("src/form_main.rs"), &escrito).unwrap();

    let r = escrever(&raiz, &gerar(&sem_o_clique())).expect("generation");
    assert_eq!(
        r.podados,
        vec!["btn_okClick"],
        "a comment citing the name stopped pruning:\n{}",
        unidade(&raiz)
    );
}

#[test]
fn o_handler_compartilhado_vazio_sai_e_leva_os_dois_vinculos() {
    // SWITCHED SIDES on 08/09/2026, and the change is in ADR 0025.
    //
    // Until `323a8ad` this test was named
    // `o_handler_compartilhado_fica_enquanto_um_dos_dois_estiver_ligado` and
    // proved the opposite: unbinding one left the handler on disk
    // for the other. That was a consequence of the old contract — "bound events
    // protect" —, and the old contract fell when the owner tested his classic RAD IDE:
    // "in a classic RAD IDE, regardless of whether the object link is bound or not, if it
    // finds it empty it cleans it up".
    //
    // What has **not** changed is why the old test existed: if a
    // binding outlived the handler, `.designer.rs` would call what does not
    // exist and the user's project would not compile. The answer now is not
    // preserving the handler — it is removing both bindings TOGETHER, which is what the
    // pre-pass does and what this test enforces.
    let mut d = documento();
    d.node_mut("edt1")
        .unwrap()
        .events
        .insert("on_change".into(), "btn_okClick".into());
    d.validate().unwrap();

    let raiz = std::env::temp_dir().join("crawrad_codegen_poda_compartilhado");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&d)).expect("first generation");

    // **ONE** definition, and this assert is what remains of a real defect:
    // `stubs` wrote one block per EVENT, so the shared
    // handler was generated twice in the same `impl` and the user's project
    // failed with `duplicate definitions` — in a file they didn't write.
    // Only surfaced because pruning was tested with shared handlers.
    assert_eq!(
        crate::elo::handlers_do_arquivo(&unidade(&raiz))
            .iter()
            .filter(|(n, _)| n == "btn_okClick")
            .count(),
        1,
        "the handler pointed to by two components was duplicated:\n{}",
        unidade(&raiz)
    );

    // The pre-pass inspects the shared handler and returns ONE unbind with
    // BOTH bindings. Just one would not suffice: the remainder would keep the call
    // in `.designer.rs`.
    let vencidos = crate::vinculos_vencidos(&raiz, &d);
    let compartilhado = vencidos
        .iter()
        .find(|x| x.handler == "btn_okClick")
        .expect("the empty shared handler must expire");
    assert_eq!(
        compartilhado.vinculos,
        vec![
            (crawrad_ir::NodeId::from("edt1"), "on_change".to_string()),
            (crawrad_ir::NodeId::from("btn1"), "on_click".to_string()),
        ],
        "the pre-pass left a binding behind — the designer would call what was removed"
    );

    // And the complete gesture: the IDE applies the unbind and generates. The handler exits, and
    // the designer no longer calls anything that does not exist.
    let arqs = gerar(&aplicar(&d, &vencidos));
    let r = escrever(&raiz, &arqs).expect("generation after unbinding");
    assert_eq!(r.podados, vec!["btn_okClick"]);
    assert!(
        !unidade(&raiz).contains("btn_okClick"),
        "the shared handler remained on disk:\n{}",
        unidade(&raiz)
    );
    assert!(
        !conteudo(&arqs, "src/form_main.designer.rs").contains("btn_okClick"),
        "the designer continued calling the handler that was just removed"
    );
}

// ------------------------------------------------------------- the NEW axis
//
// **Bound event, empty body, generate -> prune, and binding is removed from document.**
//
// Until 08/09/2026 the contract required the user to UNBIND the event first, and
// tests above verified that requirement. It fell following owner testing on
// his classic RAD IDE: binding state is irrelevant, the sole condition is an empty
// body. See ADR 0025.
//
// The `vinculos_vencidos` pre-pass decides, and it decides BEFORE the plan
// is constructed — because `.designer.rs` is derived from the document and emits
// calls as long as the binding exists.

/// What the shell does with pre-pass output: unbinds events.
///
/// This is the transaction's `Command::SetEvent { handler: None }`, written by
/// hand here because `crawrad-codegen` does not depend on `crawrad-doc` — nor should it.
fn aplicar(d: &Document, vencidos: &[crate::Desligamento]) -> Document {
    let mut novo = d.clone();
    for x in vencidos {
        for (no, evento) in &x.vinculos {
            novo.node_mut(no).unwrap().events.remove(evento);
        }
    }
    novo.validate().unwrap();
    novo
}

#[test]
fn o_handler_vazio_perde_o_vinculo_com_o_evento_ainda_ligado() {
    // THE OWNER'S CASE, 08/09/2026: accidental double-click, untouched body, and
    // he saves expecting it to vanish. Under the old contract it didn't —
    // the event remained bound and pruning refused. Refused correctly per
    // the contract, but the contract was wrong.
    let raiz = projeto_gerado("ligado");
    let d = documento();
    assert!(unidade(&raiz).contains("pub fn btn_okClick"));

    // Nothing was unbound: the document is the SAME that generated the file.
    let vencidos = crate::vinculos_vencidos(&raiz, &d);
    assert_eq!(
        vencidos
            .iter()
            .map(|x| x.handler.as_str())
            .collect::<Vec<_>>(),
        vec!["btn_okClick", "edt_nomeChange"],
        "the pre-pass did not see the two empty stubs of BOUND events"
    );
    assert_eq!(
        vencidos[0].vinculos,
        vec![(crawrad_ir::NodeId::from("btn1"), "on_click".to_string())]
    );
    assert_eq!(
        vencidos[1].vinculos,
        vec![(crawrad_ir::NodeId::from("edt1"), "on_change".to_string())]
    );

    // The IDE unbinds and generates. `escrever` didn't need to change: by the time it runs,
    // the binding is gone, and the legacy rule handles the rest.
    let arqs = gerar(&aplicar(&d, &vencidos));
    let r = escrever(&raiz, &arqs).expect("generation after unbinding");
    let depois = unidade(&raiz);
    assert_eq!(r.podados, vec!["btn_okClick", "edt_nomeChange"]);
    assert!(
        !depois.contains("btn_okClick") && !depois.contains("edt_nomeChange"),
        "the empty handler of bound event remained on disk:\n{depois}"
    );

    // ORDERING PROOF, preventing the `E0599` observed on 08/09/2026: the
    // designer in the SAME generation run no longer calls what was just removed.
    let designer = conteudo(&arqs, "src/form_main.designer.rs");
    assert!(
        !designer.contains("btn_okClick") && !designer.contains("edt_nomeChange"),
        "the designer calls a method that pruning removed — the project does not compile"
    );
}

/// The **form's** event counts like any other.
///
/// Root is not in `em_ordem_de_desenho` — it contains the drawing —,
/// so `rxp::vinculos` adds it manually with `once(raiz)`. Measured during
/// review: **removing this `once` left all 121 tests in the crate green**. The
/// failure mode is conservative (the form's `OnCreate` would never lose its
/// binding, and its empty stub would remain forever), and precisely because it is
/// silent it needs a gate: nobody notices a cleanup that never happens.
#[test]
fn o_evento_do_proprio_formulario_tambem_vence() {
    let mut d = documento();
    d.node_mut("form_main")
        .unwrap()
        .events
        .insert("on_create".into(), "form_mainCreate".into());
    d.validate().unwrap();

    let raiz = std::env::temp_dir().join("crawrad_codegen_poda_raiz");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&d)).expect("first generation");
    assert!(
        unidade(&raiz).contains("pub fn form_mainCreate"),
        "the root event stub was not even written — the test would measure something else"
    );

    let vencidos = crate::vinculos_vencidos(&raiz, &d);
    let da_raiz = vencidos
        .iter()
        .find(|x| x.handler == "form_mainCreate")
        .expect("the pre-pass did not see the empty handler of the form ITSELF");
    assert_eq!(
        da_raiz.vinculos,
        vec![(
            crawrad_ir::NodeId::from("form_main"),
            "on_create".to_string()
        )],
        "the returned binding is not the root node's"
    );
    let _ = std::fs::remove_dir_all(&raiz);
}

#[test]
fn o_comentario_no_corpo_segura_o_vinculo_do_evento_ligado() {
    // In a classic RAD IDE, adding `//` to an empty handler stops it from being
    // cleaned up. The twenty-year `{ }` idiom, and here it applies just the same.
    //
    // This test exists because of a NEW PATH: until 08/09/2026 a bound
    // event never even inspected the body, because the `ligados` list
    // guarded it earlier. Now `corpo_vazio` guards it alone — and
    // mutation testing proves it: forcing `corpo_vazio` to always return `true`, this test
    // turns red while the one above stays green.
    let raiz = projeto_gerado("comentario_ligado");
    let com_comentario = digitar(
        &unidade(&raiz),
        "btn_okClick",
        "// deixar vazio de propósito",
    );
    std::fs::write(raiz.join("src/form_main.rs"), &com_comentario).unwrap();

    let d = documento();
    let vencidos = crate::vinculos_vencidos(&raiz, &d);
    assert_eq!(
        vencidos
            .iter()
            .map(|x| x.handler.as_str())
            .collect::<Vec<_>>(),
        vec!["edt_nomeChange"],
        "the user's `//` did not protect the binding of their handler"
    );

    // And its binding remains in the document after the entire gesture.
    let depois_do_gesto = aplicar(&d, &vencidos);
    escrever(&raiz, &gerar(&depois_do_gesto)).expect("generation");
    assert!(
        unidade(&raiz).contains("deixar vazio de propósito"),
        "the handler the user wanted empty on purpose vanished:\n{}",
        unidade(&raiz)
    );
    assert_eq!(
        depois_do_gesto.node("btn1").unwrap().events.get("on_click"),
        Some(&"btn_okClick".to_string()),
        "the binding of the handler that remained was unbound anyway"
    );
}

#[test]
fn o_corpo_colapsado_pelo_cargo_fmt_continua_vazio_para_a_poda() {
    // MEASURED on 08/09/2026, with `cargo fmt` over a real generated project
    // (`cargo new` + `cargo add eframe@0.36` + `examples/gerar`):
    //
    //     -    pub fn btn_okClick(&mut self) {
    //     -
    //     -    }
    //     +    pub fn btn_okClick(&mut self) {}
    //
    // `rustfmt` collapses an empty body to `{}`, eliminating the ENTIRE line — not
    // just its spaces. That measurement guided the design: stub
    // indentation costs nothing compared to an empty line, because both
    // collapse under the same `cargo fmt`.
    //
    // What this test enforces is that formatting does not become IMMUNITY. A handler
    // that the user formatted and never filled in remains empty, continues to be pruned — and
    // the cursor still lands inside it.
    let raiz = projeto_gerado("formatado");
    let formatado = unidade(&raiz).replace(
        &format!("(&mut self) {{\n{CORPO_RECEM_NASCIDO}    }}"),
        "(&mut self) {}",
    );
    assert!(
        !formatado.contains(CORPO_RECEM_NASCIDO),
        "the test did not simulate `cargo fmt`:\n{formatado}"
    );
    std::fs::write(raiz.join("src/form_main.rs"), &formatado).unwrap();

    // The cursor still lands INSIDE the body: between the braces, which is the only
    // place left. A landing calculated from stub shape — "eight
    // spaces after the brace" — would point to the middle of the next line.
    let p = crate::elo::ponto_de_digitacao(&formatado, "btn_okClick").expect("the landing");
    assert!(
        formatado[p.byte..].starts_with("}\n"),
        "the cursor landed outside the collapsed body:\n{:?}",
        &formatado[p.byte..p.byte + 10]
    );

    let r = escrever(&raiz, &gerar(&sem_o_clique())).expect("generation");
    assert_eq!(
        r.podados,
        vec!["btn_okClick"],
        "running `cargo fmt` became immunity to pruning:\n{}",
        unidade(&raiz)
    );
    assert!(
        !unidade(&raiz).contains("btn_okClick"),
        "traces of the pruned handler remained:\n{}",
        unidade(&raiz)
    );
}

#[test]
fn o_stub_recem_criado_sobrevive_a_geracao_que_o_criou() {
    // NEWBORN IMMUNITY, which is not an explicit rule: it is a consequence of the
    // pre-pass reading DISK. Double-click triggers `Generate`, so creating
    // and cleaning up would happen in the same gesture — and the cursor would land nowhere.
    //
    // Here the handler is not yet in the file when the pre-pass runs, so
    // it is not a candidate, and `podar` in the same pass protects it via the
    // `ligados` list. Created in one gesture, cleaned up on the next save, which is what
    // a classic RAD IDE gets for free because there creating does not save.
    let raiz = std::env::temp_dir().join("crawrad_codegen_poda_recem");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&sem_o_clique())).expect("the project without the click");
    assert!(!unidade(&raiz).contains("btn_okClick"));

    // THE DOUBLE-CLICK: binds the event and generates in the same stroke.
    let d = documento();
    assert!(
        crate::vinculos_vencidos(&raiz, &d)
            .iter()
            .all(|x| x.handler != "btn_okClick"),
        "the pre-pass proposed unbinding a handler that does not even exist on disk"
    );
    escrever(&raiz, &gerar(&d)).expect("double-click generation");
    assert!(
        unidade(&raiz).contains("pub fn btn_okClick"),
        "the stub was born and died in the same gesture:\n{}",
        unidade(&raiz)
    );

    // AND ITS BODY IS EMPTY AT THIS INSTANT — immunity does not come from text.
    // Important to note after 08/09/2026, when `// your code here` was removed: the
    // question "is the body non-empty?" answers NO for this stub, both before
    // and after that change. What holds it are the two layers that do not
    // look at the body — the pre-pass, reading disk before it existed, and
    // the `ligados` list in `podar`, protecting it by NAME.
    //
    // The third layer is on the outside, and is the simplest: on FIRST generation the
    // user's file does not even exist, and `escrever` writes without passing through pruning.
    assert!(
        unidade(&raiz).contains(&format!(
            "    pub fn btn_okClick(&mut self) {{\n{CORPO_RECEM_NASCIDO}    }}\n"
        )),
        "the newly created stub failed to be born with an empty body:\n{}",
        unidade(&raiz)
    );

    // THE SUBSEQUENT SAVE, with no one having typed anything: now it expires.
    assert!(
        crate::vinculos_vencidos(&raiz, &d)
            .iter()
            .any(|x| x.handler == "btn_okClick"),
        "the untouched stub survived the subsequent save"
    );
}

#[test]
fn o_metodo_batizado_a_mao_nao_perde_o_vinculo() {
    // The two workarounds for a handler the owner WANTS empty, and this is the
    // first: methods without our marker are never inspected. Here the user
    // typed `salvar_pedido` into Inspector's `OnClick` field instead of
    // accepting the suggested `btn_okClick` — the name does not derive from the marker, and the
    // pre-pass has no way to prove the `fn` is its own.
    let raiz = projeto_gerado("batizado");
    let mut d = documento();
    d.node_mut("btn1")
        .unwrap()
        .events
        .insert("on_click".into(), "salvar_pedido".into());
    d.validate().unwrap();
    escrever(&raiz, &gerar(&d)).expect("generation with custom-named handler");
    assert!(unidade(&raiz).contains("pub fn salvar_pedido"));

    let vencidos = crate::vinculos_vencidos(&raiz, &d);
    assert!(
        !vencidos.iter().any(|x| x.handler == "salvar_pedido"),
        "the pre-pass proposed unbinding a handler it does not know it wrote"
    );
}

#[test]
fn o_handler_citado_por_outro_arquivo_mantem_o_vinculo() {
    // Build guard, now on the path of a BOUND event: if the binding
    // were dropped, the `fn` would remain on disk (`podar` also refuses) and the
    // document would lose the event to no one's benefit.
    //
    // Whether a classic RAD IDE checks references is UNVERIFIED — the specialist's
    // recollection is that it does not. The guard stays: it can only make
    // removal more conservative. See ADR 0025.
    let raiz = projeto_gerado("citado_ligado");
    std::fs::write(
        raiz.join("src/atalhos.rs"),
        "use crate::form_main_designer::FormMain;\n\
         pub fn f5(f: &mut FormMain) {\n    f.btn_okClick();\n}\n",
    )
    .unwrap();

    let vencidos = crate::vinculos_vencidos(&raiz, &documento());
    assert_eq!(
        vencidos
            .iter()
            .map(|x| x.handler.as_str())
            .collect::<Vec<_>>(),
        vec!["edt_nomeChange"],
        "the pre-pass unbound a handler that another file calls"
    );
}

#[test]
fn sem_arquivo_do_usuario_o_pre_passe_nao_propoe_nada() {
    // Every FIRST generation lands here, and is the same branch providing newborn
    // immunity: not knowing the text is reason to propose nothing.
    let raiz = std::env::temp_dir().join("crawrad_codegen_poda_sem_arquivo");
    let _ = std::fs::remove_dir_all(&raiz);
    std::fs::create_dir_all(&raiz).unwrap();
    assert!(crate::vinculos_vencidos(&raiz, &documento()).is_empty());
}

#[test]
fn o_handler_de_evento_nao_disparado_tambem_e_podado() {
    // When the event is unbound without the user having written code in the body,
    // the handler is pruned normally.
    let mut d = documento();
    let mut cbo = Node::new("cbo1", "ComboBox");
    cbo.set_name("cbo_uf");
    cbo.set_rect(Rect::new(24.0, 140.0, 120.0, 24.0));
    cbo.events.insert("on_click".into(), "cbo_ufClick".into());
    d.add_child("form_main", cbo).unwrap();
    d.validate().unwrap();

    let raiz = std::env::temp_dir().join("crawrad_codegen_poda_nao_disparado");
    let _ = std::fs::remove_dir_all(&raiz);
    escrever(&raiz, &gerar(&d)).expect("first generation");
    assert!(unidade(&raiz).contains("pub fn cbo_ufClick"));

    let mut sem = d.clone();
    sem.node_mut("cbo1").unwrap().events.remove("on_click");
    sem.validate().unwrap();
    let r = escrever(&raiz, &gerar(&sem)).expect("second generation");
    let depois = unidade(&raiz);
    assert_eq!(r.podados, vec!["cbo_ufClick"]);
    assert!(
        !depois.contains("pub fn cbo_ufClick"),
        "the handler remained:\n{depois}"
    );
}

#[test]
fn ligar_e_desligar_o_mesmo_evento_nao_deixa_rastro() {
    // The complete round trip, which is pruning's stability test: five
    // bind/unbind cycles must leave the file byte-for-byte identical to
    // what it was. A cut eating one extra blank line or leaving one
    // behind shows up here on the third pass.
    let raiz = projeto_gerado("ida_e_volta");
    let com = documento();
    let sem = sem_o_clique();
    let referencia_com = unidade(&raiz);

    escrever(&raiz, &gerar(&sem)).unwrap();
    let referencia_sem = unidade(&raiz);

    for volta in 0..5 {
        escrever(&raiz, &gerar(&com)).unwrap();
        assert_eq!(unidade(&raiz), referencia_com, "cycle {volta}, rebound");
        escrever(&raiz, &gerar(&sem)).unwrap();
        assert_eq!(unidade(&raiz), referencia_sem, "cycle {volta}, unbound");
    }
}

#[test]
fn a_frase_da_poda_esta_em_ingles_e_concorda_no_numero() {
    // It goes to the status bar, and the status bar is read by the user.
    let mut r = Relatorio::default();
    assert_eq!(r.frase_da_poda(), None);
    r.podados.push("aClick".into());
    assert_eq!(
        r.frase_da_poda().as_deref(),
        Some("1 empty handler removed")
    );
    r.podados.push("bClick".into());
    assert_eq!(
        r.frase_da_poda().as_deref(),
        Some("2 empty handlers removed")
    );
}

#[test]
fn o_cabecalho_do_stub_avisa_que_a_poda_existe() {
    // The header promised in English that the generator "never removes code you
    // wrote" — and the promise remains TRUE, because what pruning removes is
    // nobody's code. But the file now shrinks, and a header failing
    // to explain that lets the product surprise those who trusted it.
    let arqs = gerar(&documento());
    let texto = conteudo(&arqs, "src/form_main.rs");
    assert!(
        texto.contains("never removes code you wrote"),
        "the promise keeping trust in the file vanished from header"
    );
    assert!(
        texto.contains("generating takes empty event handlers back out"),
        "the header does not mention that the file also shrinks:\n{texto}"
    );
    // THE TRIGGER, which is what this file most easily misdocuments.
    // Until the previous version the header said UNBINDING the event
    // removed the handler — and ADR 0025 reversed that: generating removes it,
    // and saving generates. The old phrase was not imprecise, it contradicted
    // the product, leading readers to believe bound handlers were safe.
    //
    // The cost of an error here is the highest in the generator: `src/<form>.rs` is
    // `Dono::Usuario`, written ONCE and never rewritten. Any project
    // created with the wrong phrase carries it forever — no future generation
    // corrects it. Therefore this test checks both sides: the real trigger
    // must be present, and the old one must NOT return.
    assert!(
        texto.contains("saving generates"),
        "the header does not say WHEN pruning happens (generating, and saving generates):\n{texto}"
    );
    assert!(
        !texto.contains("unwiring an event takes its handler back out"),
        "the phrase stating that unbinding the event is the trigger returned — \
         it is the opposite of ADR 0025, and this file is never rewritten:\n{texto}"
    );
    assert!(
        texto.contains("does\n//!   not matter whether the event is still wired"),
        "the header does not say that a bound event DOES NOT protect:\n{texto}"
    );
    // And the reason the project continues to compile, which is the half the
    // user does not see: the binding exits in the SAME pass, otherwise `.designer.rs`
    // would call a `fn` that no longer exists.
    assert!(
        texto.contains("the wiring\n//!   is dropped in the same step"),
        "the header does not mention that the binding is removed together:\n{texto}"
    );
    assert!(
        texto.contains("Methods you wrote yourself are never"),
        "the header does not say that ordinary methods are never checked:\n{texto}"
    );
    // The missing caveat, and the costliest one: writing above the
    // handler is what a careful user does, and it prevents pruning. The
    // header previously listed only touched bodies and renamed `fn`s.
    assert!(
        texto.contains("write a line") && texto.contains("right above it"),
        "the header does not mention that writing above the handler makes it yours:\n{texto}"
    );
    // THE ESCAPE HATCH, which is the gesture used by the owner himself: a `//` in the body
    // to hold the spot and return later. Without this line the header describes
    // deletion without explaining how to prevent it.
    assert!(
        texto.contains("Type one character into the body, even a comment"),
        "the header does not say how the user prevents pruning:\n{texto}"
    );
}

/// **Anchor gate: emitted rule matches the `crawrad-designer` rule.**
///
/// Not a text-against-text test. It RUNS the designer rule —
/// `crawrad_designer::layout::Anchors::ajustar`, the same function the canvas
/// calls when the user drags the form edge — across the four
/// anchor combinations, inspects what it did to the rect, and requires the
/// corresponding arm of the emitted `fn anchor` to say the exact same thing.
///
/// ## Why it exists
///
/// The two branches diverged, and each had a comment justifying its side.
/// With no anchors, the IDE left the component in place while the application added
/// half the growth: `btn_livre` at `(500, 400)` on a +100 × +60 form
/// stayed at `(500, 400)` in designer and reached `(550, 430)` on screen — **50 pt and
/// 30 pt**. That was divergence D6 of the study, and the worst among them, because pure
/// geometry cannot excuse itself with themes.
///
/// ## WHAT IT MEASURES CHANGED ON 18/09/2026, which needs to be documented
///
/// The two branches became **one**: the calculation lives in
/// `crawrad_component::anchored` — standalone, without toolkit — and both sides
/// call it. The designer via `Anchors::ajustar`, the application via wrapper
/// `fn anchor`, which only adapts egui types at the boundaries and floors values.
///
/// Comparing two texts that are **the same file** would be a tautology, so what
/// remains here is what stays invariant:
///
/// 1. emitted text is the standalone piece, not a copy pasted by someone;
/// 2. all **four arms** remain explicit — a `|` or `_` would hide
///    `(false, false)`, which was how D6 entered;
/// 3. and what the piece DOES remains what its text says.
///
/// ## Why this is not a text comparison, and why it matters
///
/// Expectations derive from the MEASURED behavior of the designer, not from a table
/// hardcoded here. Changing `layout.rs` forces `rxp.rs` to change
/// alongside it, and changing `rxp.rs` forces `layout.rs` — bidirectionally, with
/// a single test. A designer rule this test cannot translate into
/// emitted text triggers the `panic!` below, which is the honest way of saying *"the
/// rule changed and nobody told me"* instead of passing by omission.
#[test]
fn a_regra_da_ancora_emitida_e_a_do_designer() {
    use crawrad_designer::layout::Anchors;

    // A component needing anchors, so `ANCORAGEM` gets emitted.
    let ui = {
        let arqs = gerar(&documento());
        conteudo(&arqs, "src/form_main.designer.rs").to_string()
    };

    // The growth simulated by the test: asymmetric on purpose, so swapped
    // axes (`grown.x` instead of `grown.y`) cannot slip through.
    const DW: f32 = 100.0;
    const DH: f32 = 60.0;
    let base = crawrad_ir::Rect::new(500.0, 400.0, 80.0, 24.0);

    // **THE WRAPPER DELEGATES.** Without this, someone could rewrite the formula
    // inside `fn anchor` and the four arms below would measure a `match` that
    // nobody calls anymore.
    assert!(
        ui.contains(
            "let (x, y, w, h) = anchored(pos.x, pos.y, size.x, size.y, a, grown.x, grown.y);"
        ),
        "the emitted `fn anchor` stopped calling the crate function:\n{}",
        ui.lines()
            .filter(|l| l.contains("anchor"))
            .collect::<Vec<_>>()
            .join("\n")
    );

    for (cabeca, eixo) in [
        ("match (a.left, a.right) {", Eixo::Horizontal),
        ("match (a.top, a.bottom) {", Eixo::Vertical),
    ] {
        let bracos = bracos_do_match(&ui, cabeca);
        assert_eq!(
            bracos.len(),
            4,
            "the `{cabeca}` of the generated application has {} arms, and anchors are \
             four combinations — a `_ =>` would hide precisely the case that \
             nobody remembers to inspect:\n{bracos:#?}",
            bracos.len()
        );

        for (perto, longe) in [(true, true), (false, true), (true, false), (false, false)] {
            let a = match eixo {
                Eixo::Horizontal => Anchors {
                    left: perto,
                    right: longe,
                    top: true,
                    bottom: false,
                },
                Eixo::Vertical => Anchors {
                    left: true,
                    right: false,
                    top: perto,
                    bottom: longe,
                },
            };
            let novo = a.adjust(base, DW, DH);
            let esperado = eixo.traduzir(base, novo, DW, DH);
            let padrao = format!("({perto}, {longe})");
            let escrito = bracos
                .iter()
                .find(|(p, _)| *p == padrao)
                .map(|(_, c)| c.as_str())
                .unwrap_or_else(|| {
                    panic!(
                        "the `{cabeca}` of the generated application does not have arm {padrao}:\n{bracos:#?}"
                    )
                });
            assert_eq!(
                escrito, esperado,
                "\nTHE ANCHOR RULE DIVERGED, and the drawing and application will \
                 show different things.\n\
                 axis             : {eixo:?}\n\
                 anchors          : {padrao}\n\
                 the IDE  (`Anchors::ajustar`, {base:?} + {DW}x{DH}) → {novo:?}\n\
                 the IDE, translated : `{esperado}`\n\
                 APPLICATION       : `{escrito}`\n\
                 The component is the same and property is the same: either both \
                 arms say this, or the user designs one screen and runs another."
            );
        }
    }
}

/// The axis being measured by [`a_regra_da_ancora_emitida_e_a_do_designer`].
#[derive(Debug, Clone, Copy)]
enum Eixo {
    Horizontal,
    Vertical,
}

impl Eixo {
    /// What the designer rule DID, expressed in the text emitted by the generator.
    ///
    /// The ending `panic!` is the critical part: any new designer rule — aspect
    /// ratio preservation, shrinking, etc. — has no translation written here, and
    /// approving by omission would be the exact hole through which D6 entered.
    fn traduzir(
        self,
        base: crawrad_ir::Rect,
        novo: crawrad_ir::Rect,
        dw: f32,
        dh: f32,
    ) -> &'static str {
        let (p0, t0, p1, t1, d, campo) = match self {
            Eixo::Horizontal => (base.x, base.w, novo.x, novo.w, dw, "dw"),
            Eixo::Vertical => (base.y, base.h, novo.y, novo.h, dh, "dh"),
        };
        let (moveu, esticou) = (p1 - p0, t1 - t0);
        let quase = |a: f32, b: f32| (a - b).abs() < 0.001;
        match self {
            Eixo::Horizontal if quase(moveu, 0.0) && quase(esticou, d) => "w += dw",
            Eixo::Horizontal if quase(moveu, d) && quase(esticou, 0.0) => "x += dw",
            Eixo::Horizontal if quase(moveu, 0.0) && quase(esticou, 0.0) => "{}",
            Eixo::Vertical if quase(moveu, 0.0) && quase(esticou, d) => "h += dh",
            Eixo::Vertical if quase(moveu, d) && quase(esticou, 0.0) => "y += dh",
            Eixo::Vertical if quase(moveu, 0.0) && quase(esticou, 0.0) => "{}",
            _ => panic!(
                "`crawrad-designer` did something this test does not know how \
                 to write in `{campo}`: moved {moveu} and stretched {esticou} for \
                 a growth of {d}. Translate the new case here — which now lives \
                 in the CRATE, in `crawrad-component/src/pieces/anchors.rs`."
            ),
        }
    }
}

/// Arms of a `match` in generated text, as `(pattern, body)` pairs.
///
/// Slices from header to the line closing the `match` at four-space
/// indentation — emitted `fn anchor` is shallow, and deeper `}` do not exist
/// inside it. Comment lines do not become match arms.
fn bracos_do_match(fonte: &str, cabeca: &str) -> Vec<(String, String)> {
    let i = fonte
        .find(cabeca)
        .unwrap_or_else(|| panic!("the generated application has no `{cabeca}`:\n{fonte}"));
    let resto = &fonte[i + cabeca.len()..];
    let fim = resto
        .find("\n    }")
        .unwrap_or_else(|| panic!("the `{cabeca}` of the generated application does not close"));
    resto[..fim]
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with("//"))
        .map(|l| {
            let (p, c) = l
                .split_once("=>")
                .unwrap_or_else(|| panic!("`{l}` is not a `match` arm"));
            (
                p.trim().to_string(),
                c.trim().trim_end_matches(',').trim().to_string(),
            )
        })
        .collect()
}

/// On both targets a click on an `AllowGrayed` CheckBox moves `state`
/// one step (Unchecked -> Checked -> Grayed), `checked` follows it, and the
/// click is followed by `OnChange`; egui draws the grayed state as
/// indeterminate.
#[test]
fn the_three_state_check_box_cycles_and_fires_on_change() {
    let mut d = Document::new("form_main", "Form");
    {
        let f = d.node_mut("form_main").unwrap();
        f.set_name("form_main");
        f.set_rect(Rect::new(0.0, 0.0, 320.0, 200.0));
    }
    let mut chk = crawrad_registry::Registry::builtin()
        .instantiate("CheckBox", "chk_tri", Rect::new(8.0, 8.0, 120.0, 20.0))
        .unwrap();
    chk.set_name("chk_tri");
    chk.props.insert("allow_grayed".into(), Value::Bool(true));
    chk.events
        .insert("on_change".into(), "chk_tri_change".into());
    d.add_child("form_main", chk).unwrap();

    let egui = gerar(&d);
    let egui = conteudo(&egui, "src/form_main.designer.rs");
    let makepad = crate::GeradorMakepad
        .gerar(&d, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("generation");
    let makepad = conteudo(&makepad, "src/form_main.designer.rs");
    for (target, code) in [("egui", egui), ("makepad", makepad)] {
        assert!(
            code.contains("\"cbChecked\" => \"cbGrayed\""),
            "{target}: the cycle reaches grayed"
        );
        assert!(
            code.contains("chk_tri_change()"),
            "{target}: OnChange is called"
        );
    }
    assert!(
        egui.contains(".indeterminate(self.chk_tri.state == \"cbGrayed\")"),
        "{egui}"
    );
    assert!(
        makepad.contains("crawrad_runtime_makepad::check::grayed("),
        "{makepad}"
    );
}

/// A click on a radio calls the OnChange of the radio that gains the
/// mark and of the sibling that loses it, each only when its `checked`
/// really moved; on both targets.
#[test]
fn the_radio_calls_on_change_on_whoever_changed() {
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").unwrap();
        root.set_rect(Rect::new(0.0, 0.0, 480.0, 320.0));
        root.set_name("form_main");
    }
    for (id, name, y) in [("rb1", "rb_cash", 8.0), ("rb2", "rb_card", 32.0)] {
        let mut rb = Node::new(id, "RadioButton");
        rb.set_name(name);
        rb.set_rect(Rect::new(8.0, y, 120.0, 20.0));
        rb.events
            .insert("on_change".into(), format!("{name}_change"));
        d.add_child("form_main", rb).unwrap();
    }
    d.validate().unwrap();

    let egui = gerar(&d);
    let egui = conteudo(&egui, "src/form_main.designer.rs").to_string();
    let makepad = crate::GeradorMakepad
        .gerar(&d, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("generation");
    let makepad = conteudo(&makepad, "src/form_main.designer.rs").to_string();
    for (target, code) in [("egui", &egui), ("makepad", &makepad)] {
        let own = "if !self.form.rb_cash.checked {".replace(
            "self.form.",
            if target == "egui" {
                "self."
            } else {
                "self.form."
            },
        );
        let lost = "if self.form.rb_card.checked {".replace(
            "self.form.",
            if target == "egui" {
                "self."
            } else {
                "self.form."
            },
        );
        assert!(
            code.contains(&own),
            "{target}: the marked radio checks it was not marked"
        );
        assert!(
            code.contains(&lost),
            "{target}: the sibling checks it was marked"
        );
        assert!(
            code.contains("rb_cash_change()") && code.contains("rb_card_change()"),
            "{target}"
        );
    }
}

/// Off SimplePanel the StatusBar draws its panels, and both targets
/// read each panel's text, width and alignment.
#[test]
fn the_status_bar_draws_its_panels_on_both_targets() {
    let r = crawrad_registry::Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").unwrap();
        root.set_name("form_main");
        root.set_rect(Rect::new(0.0, 0.0, 320.0, 200.0));
    }
    let mut sb = r
        .instantiate("StatusBar", "stb_main", Rect::new(0.0, 180.0, 320.0, 20.0))
        .unwrap();
    sb.set_name("stb_main");
    sb.props.insert("simple_panel".into(), Value::Bool(false));
    d.add_child("form_main", sb).unwrap();
    let mut p = r
        .instantiate("StatusPanel", "stp_mode", Rect::new(0.0, 0.0, 28.0, 28.0))
        .unwrap();
    p.set_name("stp_mode");
    p.set_text("INS");
    d.add_child("stb_main", p).unwrap();

    let egui = gerar(&d);
    let egui = conteudo(&egui, "src/form_main.designer.rs").to_string();
    let makepad = crate::GeradorMakepad
        .gerar(&d, "meu_app", &r)
        .expect("generation");
    let makepad = conteudo(&makepad, "src/form_main.designer.rs").to_string();
    assert!(egui.contains("self.stp_mode.text.as_str()"), "{egui}");
    assert!(
        egui.contains("self.stp_mode.width") && egui.contains("self.stp_mode.alignment"),
        "{egui}"
    );
    assert!(
        makepad.contains("crawrad_runtime_makepad::status::panel(")
            && makepad.contains("&self.form.stp_mode.text"),
        "{makepad}"
    );
    assert!(makepad.contains("p_0 = <View>"), "{makepad}");
}

/// On both targets the Shape draws its kind with its own brush and
/// pen; Makepad reaches the shader through `shape::apply` and imports the
/// shader library its `Sdf2d` lives in.
#[test]
fn the_shape_draws_its_kind_with_its_brush_and_pen() {
    let r = crawrad_registry::Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").unwrap();
        root.set_name("form_main");
        root.set_rect(Rect::new(0.0, 0.0, 320.0, 200.0));
    }
    let mut shp = r
        .instantiate("Shape", "shp_dot", Rect::new(8.0, 8.0, 40.0, 40.0))
        .unwrap();
    shp.set_name("shp_dot");
    shp.props
        .insert("shape".into(), Value::Enum("Ellipse".into()));
    d.add_child("form_main", shp).unwrap();

    let egui = gerar(&d);
    let egui = conteudo(&egui, "src/form_main.designer.rs").to_string();
    let makepad = crate::GeradorMakepad
        .gerar(&d, "meu_app", &r)
        .expect("generation");
    let makepad = conteudo(&makepad, "src/form_main.designer.rs").to_string();
    assert!(
        egui.contains("egui::Shape::ellipse_filled") && egui.contains("self.shp_dot.brush_color"),
        "{egui}"
    );
    assert!(
        makepad.contains("crawrad_runtime_makepad::shape::apply(")
            && makepad.contains("brush_color.to_f32()"),
        "{makepad}"
    );
    assert!(
        makepad.contains("use makepad_draw::shader::std::*;"),
        "the Sdf2d is in scope"
    );
    assert!(makepad.contains("instance pen_color"), "{makepad}");
}

/// OnPaint hands a `Canvas` — on a PaintBox and on the form — and both
/// targets draw what it recorded: egui paints the commands in the box (the
/// form's under its controls), Makepad hands them to a `CrawradPaintBox`.
#[test]
fn on_paint_draws_on_a_canvas_on_both_targets() {
    let r = crawrad_registry::Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").unwrap();
        root.set_name("form_main");
        root.set_rect(Rect::new(0.0, 0.0, 320.0, 200.0));
        root.events
            .insert("on_paint".into(), "form_main_paint".into());
    }
    let mut pbx = r
        .instantiate("PaintBox", "pbx_chart", Rect::new(8.0, 8.0, 120.0, 80.0))
        .unwrap();
    pbx.set_name("pbx_chart");
    pbx.events
        .insert("on_paint".into(), "pbx_chart_paint".into());
    d.add_child("form_main", pbx).unwrap();

    let egui = gerar(&d);
    let user = conteudo(&egui, "src/form_main.rs").to_string();
    let egui = conteudo(&egui, "src/form_main.designer.rs").to_string();
    let makepad = crate::GeradorMakepad
        .gerar(&d, "meu_app", &r)
        .expect("generation");
    let makepad = conteudo(&makepad, "src/form_main.designer.rs").to_string();

    assert!(
        user.contains("pub fn pbx_chart_paint(&mut self, _canvas: &mut Canvas)"),
        "{user}"
    );
    for (target, code) in [("egui", &egui), ("makepad", &makepad)] {
        assert!(
            code.contains("pub struct Canvas"),
            "{target}: the Canvas is emitted"
        );
        assert!(
            code.contains("pbx_chart_paint(&mut canvas)"),
            "{target}: the PaintBox's OnPaint runs"
        );
        assert!(
            code.contains("form_main_paint(&mut canvas)"),
            "{target}: the form's OnPaint runs"
        );
    }
    assert!(
        egui.contains("PaintCommand::Line"),
        "egui paints the commands"
    );
    assert!(
        makepad.contains("pbx_chart = <CrawradPaintBox>"),
        "{makepad}"
    );
    assert!(
        makepad.contains("crawrad_form_paint = <CrawradPaintBox>"),
        "the form has its paint box"
    );
}

/// On both targets a SpeedButton in a group (GroupIndex above 0) goes
/// down on a click and lifts the others of its group; with AllowAllUp a
/// click on the one that is down lifts it.
#[test]
fn the_speed_buttons_of_a_group_go_down_one_at_a_time() {
    let r = crawrad_registry::Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").unwrap();
        root.set_name("form_main");
        root.set_rect(Rect::new(0.0, 0.0, 320.0, 200.0));
    }
    for (i, id) in ["sbt_a", "sbt_b"].into_iter().enumerate() {
        let mut b = r
            .instantiate(
                "SpeedButton",
                id,
                Rect::new(8.0 + i as f32 * 30.0, 8.0, 28.0, 24.0),
            )
            .unwrap();
        b.set_name(id);
        b.props.insert("group_index".into(), Value::Int(1));
        d.add_child("form_main", b).unwrap();
    }
    let egui = gerar(&d);
    let egui = conteudo(&egui, "src/form_main.designer.rs").to_string();
    let makepad = crate::GeradorMakepad
        .gerar(&d, "meu_app", &r)
        .expect("generation");
    let makepad = conteudo(&makepad, "src/form_main.designer.rs").to_string();
    for (target, code, p) in [
        ("egui", &egui, "self."),
        ("makepad", &makepad, "self.form."),
    ] {
        assert!(
            code.contains(&format!("{p}sbt_a.down = true;")),
            "{target}: the clicked one goes down"
        );
        assert!(
            code.contains(&format!(
                "if {p}sbt_b.group_index == {p}sbt_a.group_index {{"
            )),
            "{target}: the others of its group are lifted"
        );
        assert!(
            code.contains(&format!("if {p}sbt_a.allow_all_up {{")),
            "{target}: AllowAllUp lifts it"
        );
    }
    assert!(
        egui.contains(".selected(self.sbt_a.down).frame_when_inactive(!self.sbt_a.flat)"),
        "{egui}"
    );
    assert!(
        makepad.contains("crawrad_runtime_makepad::speed::look("),
        "{makepad}"
    );
}

/// The Explorer preview judges the text on screen, like `Generate`.
///
/// On disk `btn_okClick` is empty and expires. In the editor the user has
/// typed into it, unsaved: judged by that text it is the user's, and it
/// stays.
#[test]
fn the_pre_pass_judges_the_text_being_edited() {
    let root = projeto_gerado("editing");
    let on_disk = unidade(&root);
    let at = on_disk
        .find("pub fn btn_okClick")
        .and_then(|i| on_disk[i..].find('{').map(|j| i + j + 1))
        .expect("the stub is on disk");
    let typed = format!(
        "{}\n        // typed, not saved{}",
        &on_disk[..at],
        &on_disk[at..]
    );
    let unit = root.join("src/form_main.rs");
    let editing = |p: &std::path::Path| (p == unit).then(|| typed.clone());

    let by_disk: Vec<String> = crate::vinculos_vencidos(&root, &documento())
        .into_iter()
        .map(|x| x.handler)
        .collect();
    assert!(by_disk.contains(&"btn_okClick".to_string()), "{by_disk:?}");

    let by_screen: Vec<String> = crate::stale_bindings_editing(&root, &documento(), &editing)
        .into_iter()
        .map(|x| x.handler)
        .collect();
    assert!(
        !by_screen.contains(&"btn_okClick".to_string()),
        "the preview expired a handler the user is typing into: {by_screen:?}"
    );
    assert_eq!(unidade(&root), on_disk, "judging the edit wrote to disk");
}

/// The `field: value` pairs of one initialiser, in any order (the
/// fields follow the catalogue's order).
fn init_has(ui: &str, head: &str, pairs: &[&str]) -> bool {
    let Some(i) = ui.find(head) else {
        return false;
    };
    let line = ui[i..].lines().next().unwrap_or_default();
    pairs.iter().all(|p| line.contains(p))
}

/// **A hidden Panel takes its children with it** in the egui
/// target, as nesting does in Makepad. The Button inside is drawn only while
/// the Panel is visible; a Button on the form is not guarded by it.
#[test]
fn a_hidden_panel_hides_its_children_in_egui() {
    let mut d = documento();
    let mut pn = Node::new("pnl1", "Panel");
    pn.set_name("pnl_box");
    pn.set_rect(Rect::new(20.0, 150.0, 200.0, 100.0));
    d.add_child("form_main", pn).unwrap();
    let mut inner = Node::new("bi", "Button");
    inner.set_name("btn_inner");
    inner.set_rect(Rect::new(8.0, 8.0, 96.0, 32.0));
    d.add_child("pnl1", inner).unwrap();
    d.validate().unwrap();

    let g = conteudo(&gerar(&d), "src/form_main.designer.rs").to_string();
    let before = |name: &str, class: &str| {
        let i = g
            .find(&format!("// {name} — {class}"))
            .unwrap_or_else(|| panic!("no drawing of `{name}`"));
        g[..i]
            .lines()
            .rfind(|l| !l.trim().is_empty())
            .unwrap_or("")
            .trim()
            .to_string()
    };
    assert_eq!(before("btn_inner", "Button"), "if self.pnl_box.visible {");
    assert_ne!(before("pnl_box", "Panel"), "if self.pnl_box.visible {");
}
