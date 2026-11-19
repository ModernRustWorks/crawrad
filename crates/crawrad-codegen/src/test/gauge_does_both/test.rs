//! **ONE component handles BOTH halves** — draws and turns into code.
//!
//! Until 18/09/2026 none did. `Connection` turned into code and did not draw,
//! because it is non-visual; `Gauge` drew and did not turn into code, because there
//! was no crate with its type.
//!
//! This file checks the second half of `Gauge`. The first — painting — is
//! measured in `crawrad-runtime-makepad/src/test/declarado/`, and was verified with
//! the IDE open: three gauges with `value` 20, 65, and 95 drew three proportional
//! bars.
//!
//! # Both read the SAME properties
//!
//! `text` and `value` come from the same `manifest.json` for both sides. That is
//! what makes what you see in the designer match what the application shows — and it is
//! why the manifest is a single file instead of two.

use crawrad_ir::{Document, Rect, Value};
use crawrad_package::manifest::Manifest;
use crawrad_registry::Registry;

use crate::Gerador;

/// The catalogue with `Gauge` read from the **real** `manifest.json`.
fn catalogo_com_gauge() -> Registry {
    let texto = include_str!("../../../../../components/gauge-crawrad/manifest.json");
    let manifesto: Manifest =
        serde_json::from_str(texto).expect("the `manifest.json` is valid JSON");
    let mut r = Registry::builtin();
    for c in manifesto
        .to_components()
        .expect("the `gauge-crawrad` manifest passes conversion")
    {
        r.register_external(c)
            .expect("and passes the gate's seven rejections");
    }
    r
}

fn doc_com_gauge(r: &Registry) -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("the root");
        raiz.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        raiz.set_name("form_main");
    }
    let mut n = r
        .instantiate(
            "Gauge",
            "gau1".to_string(),
            Rect::new(8.0, 8.0, 180.0, 36.0),
        )
        .expect("the catalogue instantiates what it registered");
    n.set_name("gau1");
    n.props
        .insert("text".to_string(), Value::Str("Disco".into()));
    n.props.insert("value".to_string(), Value::Int(95));
    d.add_child("form_main", n).expect("added to the form");
    d
}

fn designer(r: &Registry, d: &Document) -> String {
    crate::GeradorRxp
        .gerar(d, "app", r)
        .expect("generating")
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().contains("designer"))
        .expect("the designer file")
        .conteudo
}

/// **THE SECOND HALF.** `Gauge` becomes the type from its crate.
#[test]
fn o_gauge_vira_o_tipo_do_crate_dele() {
    let r = catalogo_com_gauge();
    let fonte = designer(&r, &doc_com_gauge(&r));
    assert!(
        fonte.contains("pub gau1: gauge_crawrad::Gauge,"),
        "field was not typed:\n{}",
        linhas_com(&fonte, "gau1")
    );
    assert!(
        fonte.contains("gau1: gauge_crawrad::Gauge::default(),"),
        "the `Create` did not become `default()`:\n{}",
        linhas_com(&fonte, "gau1")
    );
}

/// **The properties that DRAWING uses are the ones that code receives.**
///
/// This prevents the designer and the application from diverging: if the canvas paints the
/// bar from `value` and the code did not receive `value`, the IDE would be
/// showing one thing and delivering another.
#[test]
fn as_propriedades_do_desenho_sao_as_do_codigo() {
    let r = catalogo_com_gauge();
    let fonte = designer(&r, &doc_com_gauge(&r));
    assert!(
        fonte.contains("self.gau1.text = \"Disco\".to_string();"),
        "the label written by drawing did not reach the code:\n{}",
        linhas_com(&fonte, "gau1")
    );
    assert!(
        fonte.contains("self.gau1.value = 95;"),
        "the value used by drawing for the bar did not reach the code:\n{}",
        linhas_com(&fonte, "gau1")
    );
}

/// **Declared drawing and code read the SAME manifest.**
///
/// This test ties the two halves together: every property read by a drawing
/// verb must exist in the descriptor. A verb pointing to a nonexistent
/// property would draw blank and nobody would know why.
#[test]
fn todo_verbo_aponta_para_propriedade_que_existe() {
    use crawrad_ir::drawing::{Area, Stroke, TextSource};
    let r = catalogo_com_gauge();
    let d = r.get("Gauge").expect("`Gauge` loaded");
    assert!(!d.desenho.is_empty(), "`Gauge` declares drawing");

    let tem = |k: &str| d.props.iter().any(|p| p.key == k);
    for v in &d.desenho {
        if let Stroke::Text {
            from: TextSource::Property(k),
            ..
        } = &v.stroke
        {
            assert!(tem(k), "verb writes `{k}`, and class does not declare it");
        }
        if let Area::Fraction { from, min, max, .. } = &v.area {
            assert!(
                tem(from),
                "area follows `{from}`, and class does not declare it"
            );
            // **THE RANGE ALSO POINTS**, since it became able to track a property.
            // Without these two lines, the limit would be the only place in the
            // vocabulary where a made-up key would slip through — and the
            // symptom would be an always-empty bar, without a word.
            for (ponta, l) in [("min", min), ("max", max)] {
                if let Some(k) = l.property() {
                    assert!(
                        tem(k),
                        "range `{ponta}` follows `{k}`, and class does not declare it"
                    );
                }
            }
        }
    }
}

fn linhas_com(fonte: &str, agulha: &str) -> String {
    fonte
        .lines()
        .filter(|l| l.contains(agulha))
        .collect::<Vec<_>>()
        .join("\n")
}

/// **The generator STATES which crates the project needs.**
///
/// Without this the code comes out right and the project fails to compile: `gauge_crawrad::Gauge`
/// only resolves if `Cargo.toml` depends on the crate. The shell adds it
/// with `cargo add` (ADR 0018), and this is the question it asks beforehand.
#[test]
fn o_formulario_diz_de_que_terceiros_ele_depende() {
    let r = catalogo_com_gauge();
    let d = doc_com_gauge(&r);
    assert_eq!(
        crate::classes_de_terceiro(&d, &r),
        vec!["Gauge"],
        "the form uses a `Gauge`, and it comes from outside"
    );
}

/// **A form with only native controls requests no dependencies.**
///
/// This is the common case, and a non-empty list here would make the shell run `cargo add`
/// for nothing in every project someone creates.
#[test]
fn formulario_sem_terceiro_nao_pede_nada() {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("the root");
        raiz.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        raiz.set_name("form_main");
    }
    let n = r
        .instantiate(
            "Button",
            "btn1".to_string(),
            Rect::new(8.0, 8.0, 80.0, 24.0),
        )
        .expect("instantiates");
    d.add_child("form_main", n).expect("added");
    assert!(
        crate::classes_de_terceiro(&d, &r).is_empty(),
        "native type is EMITTED into the project, not imported from any crate"
    );
}
