//! Verification gate: every `crawrad-*` crate emitted by the generator MUST be
//! declared in `dependencias_do_projeto` for that target.
//!
//! Prevents regressions where the generator emitted `crawrad_runtime_makepad::...`
//! without registering the crate in the generated project's single dependency list.

use crate::dependencias::dependencias_do_projeto;
use crate::{AlvoGerador, Gerador};
use crawrad_ir::binding::{BindingDef, BindingMode};
use crawrad_ir::{Document, Rect};
use crawrad_package::manifest::Manifest;
use crawrad_registry::Registry;
use std::collections::BTreeSet;

/// Extracts all `crawrad-*` crate names referenced in generated Rust code.
/// Searches for `crawrad_<name>::` and `use crawrad_<name>`.
pub fn extrair_crates_crawrad_referenciados(codigo: &str) -> BTreeSet<String> {
    let mut crates = BTreeSet::new();
    for linha in codigo.lines() {
        let sem_comentario = match linha.find("//") {
            Some(idx) => &linha[..idx],
            None => linha,
        };

        let mut restante = sem_comentario;
        while let Some(pos) = restante.find("crawrad_") {
            let eh_inicio = if pos == 0 {
                true
            } else {
                let ant = restante.as_bytes()[pos - 1];
                !ant.is_ascii_alphanumeric() && ant != b'_'
            };

            let apos_crawrad = &restante[pos..];
            if eh_inicio {
                let mut fim_ident = 0;
                for c in apos_crawrad.chars() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        fim_ident += c.len_utf8();
                    } else {
                        break;
                    }
                }
                let ident = &apos_crawrad[..fim_ident];
                let depois = &apos_crawrad[fim_ident..];

                let antes = &sem_comentario[..sem_comentario.len() - restante.len() + pos];
                let eh_use = antes
                    .split_whitespace()
                    .any(|w| w == "use" || w.ends_with("::use"));
                let eh_qualificador = depois.starts_with("::");

                if eh_qualificador || eh_use {
                    let nome_crate = ident.replace('_', "-");
                    crates.insert(nome_crate);
                }
            }

            restante = &apos_crawrad[5..];
        }
    }
    crates
}

fn catalogo_com_terceiros() -> Registry {
    let mut reg = Registry::builtin();
    for texto in [
        include_str!("../../../../../components/gauge-crawrad/manifest.json"),
        include_str!("../../../../../components/portadb-crawrad/manifest.json"),
    ] {
        let manifesto: Manifest =
            serde_json::from_str(texto).expect("valid third-party component manifest.json");
        for c in manifesto
            .to_components()
            .expect("manifest converts to components")
        {
            reg.register_external(c)
                .expect("registers external component");
        }
    }
    reg
}

fn construir_documento(reg: &Registry, usa_binding: bool, com_terceiro: bool) -> Document {
    let mut doc = Document::new("form_main", "Form");
    {
        let raiz = doc.node_mut("form_main").expect("document root");
        raiz.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        raiz.set_name("form_main");
        raiz.set_text("Main Form");
    }

    // Standard native button
    let mut btn = reg
        .instantiate("Button", "btn_ok", Rect::new(10.0, 10.0, 80.0, 30.0))
        .expect("instantiates Button");
    btn.set_name("btn_ok");
    btn.set_text("OK");

    if usa_binding {
        btn.bindings.insert(
            "text".to_string(),
            BindingDef::new("ds_cliente", BindingMode::TwoWay).with_field("nome"),
        );
    }
    doc.add_child("form_main", btn).expect("adds button");

    if com_terceiro {
        let mut gau = reg
            .instantiate("Gauge", "gau1", Rect::new(100.0, 10.0, 120.0, 120.0))
            .expect("instantiates third-party Gauge");
        gau.set_name("gau1");
        doc.add_child("form_main", gau).expect("adds third party");
    }

    doc.validate().expect("valid document");
    doc
}

/// Validates whether all `crawrad-*` crates referenced in generated files are in the dependency list.
/// Returns `Ok(())` if all are declared, or `Err(String)` detailing the missing crates.
pub fn validar_crates_gerados_vs_declarados(
    alvo: AlvoGerador,
    arquivos: &[crate::Arquivo],
    declarados: &BTreeSet<String>,
) -> Result<(), Vec<String>> {
    let mut erros = Vec::new();

    for arq in arquivos {
        let citados = extrair_crates_crawrad_referenciados(&arq.conteudo);
        for c in citados {
            if !declarados.contains(&c) {
                erros.push(format!(
                    "Crate `{c}` referenced in `{}` but not declared for target {:?}",
                    arq.caminho.display(),
                    alvo
                ));
            }
        }
    }

    if erros.is_empty() { Ok(()) } else { Err(erros) }
}

#[test]
fn todo_crate_crawrad_emitido_esta_declarado_na_matriz() {
    let reg_terceiro = catalogo_com_terceiros();
    let reg_builtin = Registry::builtin();

    let alvos = [AlvoGerador::Makepad, AlvoGerador::Egui];

    for alvo in alvos {
        let gerador: Box<dyn Gerador> = alvo.create_generator();

        for usa_binding in [false, true] {
            for com_terceiro in [false, true] {
                let reg = if com_terceiro {
                    &reg_terceiro
                } else {
                    &reg_builtin
                };
                let doc = construir_documento(reg, usa_binding, com_terceiro);

                let arquivos = gerador
                    .gerar(&doc, "app_test", reg)
                    .unwrap_or_else(|e| panic!("generator {:?} failed: {e}", alvo));

                let deps = dependencias_do_projeto(alvo, true, usa_binding);
                let declarados: BTreeSet<String> =
                    deps.iter().map(|d| d.nome().to_string()).collect();

                let res = validar_crates_gerados_vs_declarados(alvo, &arquivos, &declarados);
                assert!(
                    res.is_ok(),
                    "Integrity failure for target {:?} (usa_binding={}, com_terceiro={}):\n{:#?}",
                    alvo,
                    usa_binding,
                    com_terceiro,
                    res.err()
                );
            }
        }
    }
}

/// **Mutation proof 1**: Removing `crawrad-runtime-makepad` from the list fails the test and points out the crate.
#[test]
fn mutation_proof_tirar_crawrad_runtime_makepad_da_lista_falha() {
    let gerador: Box<dyn Gerador> = AlvoGerador::Makepad.create_generator();
    let reg = Registry::builtin();
    let doc = construir_documento(&reg, false, false);
    let arquivos = gerador.gerar(&doc, "app_test", &reg).unwrap();

    // Simulates dependency list WITHOUT crawrad-runtime-makepad
    let mut deps: BTreeSet<String> = dependencias_do_projeto(AlvoGerador::Makepad, true, false)
        .into_iter()
        .map(|d| d.nome().to_string())
        .collect();
    assert!(
        deps.remove("crawrad-runtime-makepad"),
        "crawrad-runtime-makepad must be in normal list"
    );

    let res = validar_crates_gerados_vs_declarados(AlvoGerador::Makepad, &arquivos, &deps);
    assert!(
        res.is_err(),
        "Must fail when crawrad-runtime-makepad is omitted from the list"
    );
    let erros = res.unwrap_err();
    assert!(
        erros.iter().any(|e| e.contains("crawrad-runtime-makepad")),
        "Error message must cite `crawrad-runtime-makepad` specifically:\n{:#?}",
        erros
    );
}

/// **Mutation proof 2**: Emitting `crawrad_qualquer::x` in generated code fails.
#[test]
fn mutation_proof_emitir_crawrad_qualquer_no_gerador_falha() {
    let arquivos = vec![crate::Arquivo {
        caminho: std::path::PathBuf::from("src/form_main.designer.rs"),
        conteudo: "use makepad_widgets::*;\nfn f() { crawrad_qualquer::x(); }\n".to_string(),
        dono: crate::Dono::Gerador,
        adicoes: Vec::new(),
    }];

    let deps: BTreeSet<String> = dependencias_do_projeto(AlvoGerador::Makepad, true, false)
        .into_iter()
        .map(|d| d.nome().to_string())
        .collect();

    let res = validar_crates_gerados_vs_declarados(AlvoGerador::Makepad, &arquivos, &deps);
    assert!(
        res.is_err(),
        "Must fail when citing undeclared crate crawrad_qualquer"
    );
    let erros = res.unwrap_err();
    assert!(
        erros.iter().any(|e| e.contains("crawrad-qualquer")),
        "Error message must cite `crawrad-qualquer` specifically:\n{:#?}",
        erros
    );
}
