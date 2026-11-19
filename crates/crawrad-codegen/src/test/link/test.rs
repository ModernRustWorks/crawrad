use super::*;

/// The assertion that all of them use: the returned byte must land **on the definition
/// name**, and what follows from it is the start of the actual
/// signature. Comparing text instead of counting lines is what makes the test
/// fail for the right reason when it fails.
fn aponta_para(fonte: &str, nome: &str, assinatura: &str) -> Local {
    let l = localizar_handler(fonte, nome)
        .unwrap_or_else(|| panic!("did not find `{nome}` in:\n{fonte}"));
    assert!(
        fonte[l.byte..].starts_with(assinatura),
        "landed in the wrong place (line {}, column {}): {:?}",
        l.linha,
        l.coluna,
        &fonte[l.byte..(l.byte + 40).min(fonte.len())]
    );
    l
}

#[test]
fn acha_a_definicao_e_diz_a_linha() {
    let fonte = "//! **Este arquivo é seu.**\n\
                 use crate::ui::FormMain;\n\
                 \n\
                 impl FormMain {\n\
                     pub fn btn_okClick(&mut self) {\n\
                     self.edt_nome.text.clear();\n\
                 }\n\
                 }\n";
    let l = aponta_para(fonte, "btn_okClick", "btn_okClick(&mut self)");
    assert_eq!(l.linha, 5);
    // `pub fn ` is 7 characters before the name, and the column is 1-based.
    assert_eq!(l.coluna, 8);
}

#[test]
fn o_bom_do_arquivo_nao_ocupa_coluna() {
    // CrawRAD no longer writes a BOM, but OLD project files have it, and
    // `read_to_string` returns it as a character. Counting it would shift the
    // entire first line one column to the right — and the byte must
    // remain that of the text received by the editor, BOM included.
    let fonte = "\u{feff}impl FormMain { pub fn btn_okClick(&mut self) {} }\n";
    let l = aponta_para(fonte, "btn_okClick", "btn_okClick(&mut self)");
    assert_eq!((l.linha, l.coluna), (1, 24));
    assert_eq!(l.byte, fonte.find("btn_okClick").unwrap());
}

#[test]
fn o_comentario_de_linha_nao_e_definicao() {
    let fonte = "impl FormMain {\n\
                 // TODO: chamar fn btn_okClick( daqui quando o Cancel entrar\n\
                 /// Ver `fn btn_okClick(` acima.\n\
                 pub fn btn_okClick(&mut self) {}\n\
                 }\n";
    let l = aponta_para(fonte, "btn_okClick", "btn_okClick(&mut self)");
    assert_eq!(l.linha, 4);
}

#[test]
fn o_comentario_de_bloco_aninha() {
    // Rust's `/* */` nests. Closing at the first `*/` would turn the middle of
    // the old draft back into code.
    let fonte = "/*\n\
                 Rascunho:\n\
                 /* fn btn_okClick(&mut self) {} */\n\
                 e este também: fn btn_okClick(&mut self) {}\n\
                 */\n\
                 impl FormMain {\n\
                 pub fn btn_okClick(&mut self) { /* pronto */ }\n\
                 }\n";
    let l = aponta_para(
        fonte,
        "btn_okClick",
        "btn_okClick(&mut self) { /* pronto */ }",
    );
    assert_eq!(l.linha, 7);
    assert_eq!(handlers_do_arquivo(fonte).len(), 1);
}

#[test]
fn a_string_nao_e_definicao() {
    let fonte = "impl FormMain {\n\
                 pub fn ajuda(&self) -> &str {\n\
                 \"write fn btn_okClick(&mut self) in your file\"\n\
                 }\n\
                 pub fn com_aspas(&self) -> &str {\n\
                 \"he said \\\"fn btn_okClick(&mut self)\\\" and left\"\n\
                 }\n\
                 pub fn btn_okClick(&mut self) {}\n\
                 }\n";
    let l = aponta_para(fonte, "btn_okClick", "btn_okClick(&mut self) {}");
    assert_eq!(l.linha, 8);
    assert_eq!(
        nomes(fonte),
        vec!["ajuda", "com_aspas", "btn_okClick"],
        "only the three real definitions"
    );
}

#[test]
fn a_string_crua_nao_e_definicao() {
    // In a raw string there is no escape, so the inner quote does not close
    // anything — `"#` closes it.
    let fonte = "impl FormMain {\n\
                 pub fn sql(&self) -> &str {\n\
                 r#\"SELECT \"fn btn_okClick(\" FROM ajuda\"#\n\
                 }\n\
                 pub fn btn_okClick(&mut self) {}\n\
                 }\n";
    let l = aponta_para(fonte, "btn_okClick", "btn_okClick(&mut self) {}");
    assert_eq!(l.linha, 5);
}

#[test]
fn a_chamada_nao_e_definicao() {
    // The designer emits `self.btn_okClick();`. If that counted as a
    // definition, double-clicking would land in the generated file.
    let fonte = "impl FormMain {\n\
                 pub fn btn_cancelClick(&mut self) {\n\
                 self.btn_okClick();\n\
                 (self.btn_okClick)();\n\
                 }\n\
                 }\n";
    assert_eq!(
        localizar_handler(fonte, "btn_okClick"),
        None,
        "call is not definition: here the handler still needs to be created"
    );
}

#[test]
fn o_prefixo_de_outro_nome_nao_serve() {
    let fonte = "impl FormMain {\n\
                 pub fn btn_okClick(&mut self) {}\n\
                 pub fn btn_ok(&mut self) {}\n\
                 pub fn btn_okClickDuplo(&mut self) {}\n\
                 }\n";
    assert_eq!(aponta_para(fonte, "btn_ok", "btn_ok(&mut self)").linha, 3);
    assert_eq!(
        aponta_para(fonte, "btn_okClick", "btn_okClick(&mut self)").linha,
        2
    );
    assert_eq!(
        aponta_para(fonte, "btn_okClickDuplo", "btn_okClickDuplo(&mut self)").linha,
        4
    );
}

#[test]
fn todas_as_formas_de_declarar_valem() {
    let fonte = "impl FormMain {\n\
                 fn a1(&mut self) {}\n\
                 pub fn a2(&mut self) {}\n\
                 pub(crate) fn a3(&mut self) {}\n\
                 pub async fn a4(&mut self) {}\n\
                 const fn a5() {}\n\
                 pub fn   a6 (&mut self) {}\n\
                 fn\n\
                 a7(&mut self) {}\n\
                 }\n";
    assert_eq!(nomes(fonte), vec!["a1", "a2", "a3", "a4", "a5", "a6", "a7"]);
    assert_eq!(aponta_para(fonte, "a6", "a6 (&mut self)").linha, 7);
    assert_eq!(aponta_para(fonte, "a7", "a7(&mut self)").linha, 9);
}

#[test]
fn o_lifetime_nao_e_char_e_o_char_nao_e_lifetime() {
    // Both pitfalls live in the same source, and both **before** the
    // handler — which is where they truly hurt: the new handler enters at the
    // mark, at the end of the `impl`, so everything the user wrote by hand
    // comes above it.
    //
    // Reading `&'a str` as a literal makes the scan run until the next
    // single quote and swallow the `fn` along the way — and the lifetimes here are
    // odd in count, which is the common case and what makes the last one run to
    // the end of the file. Reading `'"'` as a lifetime leaves a stray double
    // quote and the rest of the file becomes a string. Opposite errors, one test.
    let fonte = "use crate::ui::FormMain;\n\
                 \n\
                 pub struct Ajuda<'a> {\n\
                 pub texto: &'a str,\n\
                 }\n\
                 \n\
                 impl<'a> Ajuda<'a> {\n\
                 pub fn rotulo(&self) -> &'a str {\n\
                 self.texto\n\
                 }\n\
                 }\n\
                 \n\
                 impl FormMain {\n\
                 pub fn separador(&self) -> char {\n\
                 '\"'\n\
                 }\n\
                 \n\
                 pub fn escapes(&self) -> [char; 2] {\n\
                 ['\\'', '\\\\']\n\
                 }\n\
                 \n\
                 pub fn btn_okClick(&mut self) {\n\
                 'volta: loop { break 'volta; }\n\
                 }\n\
                 }\n";
    let l = aponta_para(fonte, "btn_okClick", "btn_okClick(&mut self)");
    assert_eq!(l.linha, 22);
    assert_eq!(
        nomes(fonte),
        vec!["rotulo", "separador", "escapes", "btn_okClick"]
    );
}

#[test]
fn impl_de_outro_tipo_e_ambiguidade_visivel() {
    // Two `impl`s in the same file can have the same `fn`. Choosing the
    // first is a choice; hiding that there was a tie would be a lie.
    let fonte = "impl Auxiliar {\n\
                 pub fn btn_okClick(&mut self) { /* do auxiliar */ }\n\
                 }\n\
                 \n\
                 impl FormMain {\n\
                 pub fn btn_okClick(&mut self) { /* do formulário */ }\n\
                 fn interna(&self) {\n\
                 fn aninhada() {}\n\
                 aninhada();\n\
                 }\n\
                 }\n";
    assert_eq!(
        aponta_para(
            fonte,
            "btn_okClick",
            "btn_okClick(&mut self) { /* do auxiliar */ }"
        )
        .linha,
        2
    );
    let todos = handlers_do_arquivo(fonte);
    let repetidos: Vec<&(String, Local)> =
        todos.iter().filter(|(n, _)| n == "btn_okClick").collect();
    assert_eq!(repetidos.len(), 2, "the tie must appear in the list");
    assert_eq!(repetidos[1].1.linha, 6);
    assert!(
        nomes(fonte).contains(&"aninhada".to_string()),
        "the inner `fn` inside a body also exists"
    );
}

#[test]
fn o_inventario_serve_para_achar_orfao() {
    let fonte = "impl FormMain {\n\
                 pub fn btn_okClick(&mut self) {}\n\
                 pub fn btn_velhoClick(&mut self) {}\n\
                 }\n";
    let ligados = ["btn_okClick"];
    let orfaos: Vec<String> = handlers_do_arquivo(fonte)
        .into_iter()
        .map(|(n, _)| n)
        .filter(|n| !ligados.contains(&n.as_str()))
        .collect();
    assert_eq!(orfaos, vec!["btn_velhoClick"]);
}

#[test]
fn ida_e_volta_com_o_gerador() {
    // Proof that the two halves speak the same language: what `stubs()`
    // writes, `localizar_handler` finds. If either changes format on its
    // own, this is where it shows up.
    use crate::{Gerador, GeradorRxp};
    use crawrad_ir::{Document, Node, Rect};

    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 480.0, 320.0));

    let mut edt = Node::new("edt1", "Edit");
    edt.set_name("edt_nome");
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
    d.validate().unwrap();

    let arqs = GeradorRxp
        .gerar(&d, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("generation");
    let fonte = &arqs
        .iter()
        .find(|a| a.caminho == std::path::Path::new("src/form_main.rs"))
        .expect("the user file")
        .conteudo;

    let mut achados = nomes(fonte);
    achados.sort();
    assert_eq!(achados, vec!["btn_okClick", "edt_nomeChange"]);

    for nome in ["btn_okClick", "edt_nomeChange"] {
        let l = localizar_handler(fonte, nome).unwrap_or_else(|| {
            panic!("the generator wrote `{nome}` and the link did not find it:\n{fonte}")
        });
        assert!(
            fonte[l.byte..].starts_with(&format!("{nome}(&mut self)")),
            "the byte must land on the definition name, not on `fn` or doc-comment"
        );
        // The line the IDE will display is the signature line, not the
        // `/// \u{60}btn_ok.on_click\u{60}` right above it.
        let linha = fonte.lines().nth(l.linha - 1).unwrap_or_default();
        assert!(linha.contains(&format!("fn {nome}(")), "linha: {linha:?}");
    }
}

fn nomes(fonte: &str) -> Vec<String> {
    handlers_do_arquivo(fonte)
        .into_iter()
        .map(|(n, _)| n)
        .collect()
}

// ------------------------------------------------------- measure and slice

fn extensao_de(fonte: &str, nome: &str) -> Extensao {
    let l = localizar_handler(fonte, nome).expect("find");
    extensao_do_handler(fonte, l.byte).expect("measure")
}

#[test]
fn a_chave_dentro_de_texto_nao_fecha_o_corpo() {
    // The whole pitfall in a single file: `{}` from `format!`, brace in a
    // raw string, brace in a comment, and a real nested block.
    // Counting braces in RAW source would stop the slice in the middle of the body —
    // and pruning would cut half of an `fn`.
    let fonte = "impl FormMain {\n\
                 pub fn btn_okClick(&mut self) {\n\
                 println!(\"{} }} {{\", 1);\n\
                 let _ = r#\"} } }\"#;\n\
                 // } isto não fecha nada\n\
                 if true { let _ = 1; }\n\
                 }\n\
                 pub fn depois(&mut self) {}\n\
                 }\n";
    let e = extensao_de(fonte, "btn_okClick");
    assert!(
        fonte[e.corpo.clone()].contains("if true"),
        "the body stopped early: {:?}",
        &fonte[e.corpo.clone()]
    );
    assert!(fonte[e.item.clone()].starts_with("pub fn btn_okClick"));
    assert!(
        fonte[e.item.clone()].ends_with("}"),
        "the item must end at the closing brace of the body"
    );
    assert!(
        !fonte[e.item.clone()].contains("pub fn depois"),
        "the slice swallowed the next `fn`"
    );
}

#[test]
fn o_item_comeca_na_linha_e_nao_no_fn() {
    // Indentation is part of what gets removed. Starting at `pub` would leave
    // four lone spaces on a line after slicing.
    let fonte = "impl FormMain {\n    pub fn btn_okClick(&mut self) {}\n}\n";
    let e = extensao_de(fonte, "btn_okClick");
    assert_eq!(&fonte[e.item], "    pub fn btn_okClick(&mut self) {}");
}

#[test]
fn corpo_que_nao_fecha_nao_e_medido() {
    // Half-written file while someone is typing. Measuring incorrectly is worse
    // than not measuring: the caller bails out, and nothing is deleted.
    let fonte = "impl FormMain {\n    pub fn btn_okClick(&mut self) {\n";
    let l = localizar_handler(fonte, "btn_okClick").unwrap();
    assert_eq!(extensao_do_handler(fonte, l.byte), None);
}

#[test]
fn o_cursor_pousa_dentro_do_corpo_e_ja_indentado() {
    // THE NEW AXIS, measured without a screen: after a double-click the cursor does
    // not land on the method NAME — it lands on the inner line, already at the column
    // where the user will type. It is the `begin`/`end` of a classic RAD IDE, and the owner
    // asked for both halves on 08/09/2026: the empty body AND indentation.
    //
    // The `concat!` is intentional: Rust's `\n\` continuation strips the
    // indentation of the next line, and indentation is precisely what this
    // test measures.
    let fonte = concat!(
        "impl FormMain {\n",
        "    /// `btn_ok.on_click`\n",
        "    pub fn btn_okClick(&mut self) {\n",
        "        \n",
        "    }\n",
        "}\n",
    );

    let p = ponto_de_digitacao(fonte, "btn_okClick").expect("the landing");
    assert_eq!(
        (p.linha, p.coluna),
        (4, 9),
        "the cursor had to land on the inner line, at the body column"
    );
    assert!(
        fonte[p.byte..].starts_with("\n    }"),
        "the cursor landed BEFORE indentation: the first keystroke would write \
         at the wrong column"
    );

    // The TWO questions of the link, and they answer different things about the
    // same handler. This is why `ponto_de_digitacao` is a new function:
    // the generator's `crescido` still needs the name to know whether the
    // handler already exists.
    let nome = localizar_handler(fonte, "btn_okClick").expect("the name");
    assert!(fonte[nome.byte..].starts_with("btn_okClick"));
    assert_ne!(
        nome.byte, p.byte,
        "the two questions became the same — and then double-click would land \
         inside the identifier again"
    );
}

#[test]
fn o_pouso_le_o_arquivo_como_ele_esta() {
    // The file belongs to the user, and three things alter the line the
    // generator wrote. All measured on 08/09/2026, and the contract is single:
    // **the cursor never lands outside the body**. The worst case is column 1,
    // which is still inside.
    //
    // That is why the landing comes from the TEXT and not from the stub format:
    // recalculating "eight spaces after the brace" would point to the middle of
    // nowhere on the first `cargo fmt`.
    let casos: &[(&str, &str, &str)] = &[
        // Measured: `rustfmt --edition 2021` collapses empty body into `{}` —
        // with indentation, without it, either way. Cursor lands between the braces.
        (
            "cargo fmt collapsed the body",
            concat!("impl F {\n", "    pub fn h(&mut self) {}\n", "}\n"),
            "}\n}\n",
        ),
        // An editor that trims trailing whitespace. Column 1, the worst case.
        (
            "someone trimmed the end of the line",
            concat!(
                "impl F {\n",
                "    pub fn h(&mut self) {\n",
                "\n",
                "    }\n",
                "}\n"
            ),
            "\n    }\n",
        ),
        // The user typed: landing is at the start of their code, which is
        // exactly where one wants to return on the second double-click.
        (
            "the user already wrote the body",
            concat!(
                "impl F {\n",
                "    pub fn h(&mut self) {\n",
                "        self.a();\n",
                "    }\n",
                "}\n"
            ),
            "self.a();",
        ),
        // Single-line body: there is no inner line, so landing is
        // after the space separating from the brace.
        (
            "the body fits on the signature line",
            concat!(
                "impl F {\n",
                "    pub fn h(&mut self) { /* pronto */ }\n",
                "}\n"
            ),
            "/* pronto */ }",
        ),
    ];

    for (nome, fonte, esperado) in casos {
        let p = ponto_de_digitacao(fonte, "h")
            .unwrap_or_else(|| panic!("{nome}: there was no landing"));
        assert!(
            fonte[p.byte..].starts_with(esperado),
            "{nome}: the cursor landed at {:?}, and not at {esperado:?}",
            &fonte[p.byte..]
        );
        let corpo = extensao_do_handler(fonte, localizar_handler(fonte, "h").unwrap().byte)
            .unwrap()
            .corpo;
        assert!(
            corpo.contains(&p.byte) || p.byte == corpo.end,
            "{nome}: the cursor landed OUTSIDE the body — the first keystroke \
             would break the user's file"
        );
    }
}

#[test]
fn sem_a_fn_e_sem_o_corpo_fechado_nao_ha_pouso() {
    // The two bailouts, inherited entirely from what it calls. Without this,
    // double-clicking a handler renamed by the user would land the cursor
    // somewhere — and "somewhere" in someone's file is the defect forbidden
    // by ADR 0015.
    let inteiro = "impl F {\n    pub fn h(&mut self) {\n        \n    }\n}\n";
    assert_eq!(ponto_de_digitacao(inteiro, "outro"), None);
    assert_eq!(ponto_de_digitacao(inteiro, ""), None);

    let pela_metade = "impl F {\n    pub fn h(&mut self) {\n";
    assert_eq!(ponto_de_digitacao(pela_metade, "h"), None);
}

#[test]
fn a_chamada_conta_e_a_mencao_em_comentario_nao() {
    // The question a classic RAD IDE does not ask before deleting. `dono` is the range
    // of the definition itself: without excluding it, every handler would appear called
    // by itself and nothing could ever be removed.
    let fonte = "impl FormMain {\n\
                 pub fn btn_okClick(&mut self) {}\n\
                 pub fn outro(&mut self) {\n\
                 // um dia chamar btn_okClick daqui\n\
                 let _ = \"btn_okClick\";\n\
                 let _ = btn_okClickDuplo;\n\
                 }\n\
                 }\n";
    let e = extensao_de(fonte, "btn_okClick");
    assert!(
        !citado_fora(fonte, "btn_okClick", e.item.clone()),
        "comment, string, and prefix of another name are not a call"
    );

    let chamando = fonte.replace("// um dia chamar", "self.btn_okClick(); //");
    let e = extensao_de(&chamando, "btn_okClick");
    assert!(
        citado_fora(&chamando, "btn_okClick", e.item),
        "the real call must be seen — deleting here would break the build"
    );
}
