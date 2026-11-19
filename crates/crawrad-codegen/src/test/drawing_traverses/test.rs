//! **DECLARED DRAWING CROSSES OVER TO THE TARGET** — in both.
//!
//! What is measured here is the text. That it COMPILES is tested in
//! `tests/gate_terceiro_desenha.rs`, which sets up a real project with the
//! component crate installed by path and runs `clippy` on it.
//!
//! # Why both targets in the same file
//!
//! Because the defect was the same and had two magnitudes: in egui the component
//! turned into code and did not draw; in Makepad — the DEFAULT target — it did not exist,
//! because the generator filtered nodes by the list of classes it had
//! registered itself. One file per target would make the second half easy to forget.

use crawrad_ir::drawing::{Area, Axis, Ink, Limit, Stroke, TextSource, Verb};
use crawrad_ir::{Document, Rect, Value};
use crawrad_package::manifest::Manifest;
use crawrad_registry::Registry;
use crawrad_registry::external::{ExternalComponent, ExternalProp};

use crate::Gerador;

/// The catalogue with `Gauge` read from the **real** `manifest.json`.
fn catalogo_com_gauge() -> Registry {
    let text = include_str!("../../../../../components/gauge-crawrad/manifest.json");
    let manifesto: Manifest = serde_json::from_str(text).expect("`manifest.json` is valid JSON");
    let mut r = Registry::builtin();
    for c in manifesto
        .to_components()
        .expect("the `gauge-crawrad` manifest passes conversion")
    {
        r.register_external(c)
            .expect("and passes all seven rejections");
    }
    r
}

fn doc_com(reg: &Registry, classe: &str, nome: &str) -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("the root");
        raiz.set_rect(Rect::new(0.0, 0.0, 420.0, 260.0));
        raiz.set_name("form_main");
    }
    let mut n = reg
        .instantiate(classe, nome.to_string(), Rect::new(8.0, 8.0, 180.0, 36.0))
        .expect("the catalog instantiates what it registered");
    n.set_name(nome);
    n.props
        .insert("text".to_string(), Value::Str("Disco".into()));
    n.props.insert("value".to_string(), Value::Int(95));
    d.add_child("form_main", n).expect("enters the form");
    d
}

fn designer(g: &dyn Gerador, reg: &Registry, d: &Document) -> String {
    g.gerar(d, "app", reg)
        .expect("generating")
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().contains("designer"))
        .expect("the designer file")
        .conteudo
}

// ---------------------------------------------------------------------------
// 1 — THE EGUI TARGET
// ---------------------------------------------------------------------------

/// **The gauge bar is emitted as painting, and it FOLLOWS THE VALUE.**
///
/// `value` is read from the component's field, not hardcoded during generation: a
/// gauge that starts at 95% and never moves again is not a gauge.
#[test]
fn no_egui_a_barra_le_o_campo_do_componente() {
    let r = catalogo_com_gauge();
    let fonte = designer(&crate::GeradorRxp, &r, &doc_com(&r, "Gauge", "gau1"));
    assert!(
        fonte.contains(
            "fraction(self.gau1.value as f32, self.gau1.min as f32, self.gau1.max as f32)"
        ),
        "fraction does not come from the field:\n{}",
        linhas_com(&fonte, "fraction")
    );
    assert!(
        fonte.contains("ui.painter().rect_filled(a,"),
        "the `box` verb did not become painting:\n{}",
        linhas_com(&fonte, "painter")
    );
}

/// **The component's rectangle IS USED.**
///
/// This was the warning that named this work: the generator wrote
/// `let r = anchor(...)` for the third party and never touched `r` again. The
/// application had the type, had the properties, and drew nothing.
#[test]
fn no_egui_o_retangulo_do_terceiro_deixou_de_ser_jogado_fora() {
    let r = catalogo_com_gauge();
    let fonte = designer(&crate::GeradorRxp, &r, &doc_com(&r, "Gauge", "gau1"));
    let com_r = fonte
        .lines()
        .filter(|l| l.contains("let r = anchor("))
        .count();
    assert_eq!(com_r, 1, "expected ONE rectangle, that of `gau1`");
    assert!(
        fonte.contains("let a = r;"),
        "rectangle is created and unused — `unused variable: r` returned"
    );
}

/// **A third-party non-visual gets no rectangle at all.**
///
/// `Connection` has no screen presence. While the query was a list of native
/// names (`ActionList | Action | ImageList | Timer`), it fell into the visual
/// branch and the user's project was born with `unused variable: r`.
#[test]
fn no_egui_o_nao_visual_de_terceiro_nao_ganha_retangulo() {
    let text = include_str!("../../../../../components/portadb-crawrad/manifest.json");
    let manifesto: Manifest = serde_json::from_str(text).expect("valid JSON");
    let mut r = Registry::builtin();
    for c in manifesto.to_components().expect("converts") {
        r.register_external(c).expect("registers");
    }
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("the root");
        raiz.set_rect(Rect::new(0.0, 0.0, 420.0, 260.0));
        raiz.set_name("form_main");
    }
    let mut n = r
        .instantiate(
            "Connection",
            "eng1".to_string(),
            Rect::new(8.0, 8.0, 28.0, 28.0),
        )
        .expect("instantiates");
    n.set_name("eng1");
    d.add_child("form_main", n).expect("enters");

    let fonte = designer(&crate::GeradorRxp, &r, &d);
    assert!(
        !fonte.contains("let r = anchor("),
        "non-visual got a rectangle that nothing draws:\n{}",
        linhas_com(&fonte, "let r =")
    );
    assert!(
        fonte.contains("portadb_crawrad::Connection"),
        "and it still becomes code"
    );
}

/// **The third party does not drag the dummy `Widget` along.**
///
/// `tipo_usado` of a third party is `""`, and while `estado` had a trailing
/// `_`, that `""` fell back to generic `Widget`: the user's project was born with
/// `struct Widget is never constructed`.
#[test]
fn no_egui_o_terceiro_nao_traz_o_widget_de_mentira() {
    let r = catalogo_com_gauge();
    let fonte = designer(&crate::GeradorRxp, &r, &doc_com(&r, "Gauge", "gau1"));
    assert!(
        !fonte.contains("pub struct Widget {"),
        "generic `Widget` was included, and nothing constructs it"
    );
}

// ---------------------------------------------------------------------------
// 2 — THE MAKEPAD TARGET
// ---------------------------------------------------------------------------

/// **The third party EXISTS in the default target.**
///
/// It did not exist. `GeradorMakepad` filtered nodes by the list of classes
/// it registered, and an installed `Gauge` was excluded: the form came out with
/// `pub struct FormMain {}`, empty — no field, no property, no drawing, and
/// no warning.
#[test]
fn no_makepad_o_terceiro_vira_campo_tipado() {
    let r = catalogo_com_gauge();
    let fonte = designer(&crate::GeradorMakepad, &r, &doc_com(&r, "Gauge", "gau1"));
    assert!(
        fonte.contains("pub gau1: gauge_crawrad::Gauge,"),
        "field was not emitted typed:\n{}",
        linhas_com(&fonte, "gau1")
    );
    assert!(
        fonte.contains("gauge_crawrad::Gauge { text: \"Disco\".to_string(), value: 95"),
        "designer values did not reach the component:\n{}",
        linhas_com(&fonte, "gau1")
    );
    // **THE LITERAL CLOSES BY ITSELF, and `private_state = false` governs it.**
    //
    // The manifest of `gauge-crawrad` is written by `#[derive(Component)]`, which
    // publishes ALL fields of the `struct` and states so in the file. Thus no
    // field remains for `..Default::default()` to fill — and it stops being
    // harmless: it becomes `needless_update`, and **fails the user's project**
    // under `-D warnings`. Measured on 18/09/2026, the day the macro was born.
    assert!(
        !fonte.contains("..Default::default()"),
        "struct update remained in a literal that is already complete:\n{}",
        linhas_com(&fonte, "gau1")
    );
}

/// **And the component WITH private state continues starting with `Default`.**
///
/// The other half of the same decision, and one that must not be lost: a
/// hand-written manifest does not know which fields the crate's `struct` has, so it specifies
/// `private_state = true` by default. Without `..Default::default()` there, the
/// literal would be incomplete and the user's project **would not compile**.
///
/// Without this test, the `needless_update` fix would apply to everyone
/// and break precisely what it should not touch.
#[test]
fn no_makepad_o_terceiro_com_estado_privado_comeca_no_default() {
    let mut r = catalogo_com_gauge();
    // The same `Gauge`, with another class and the conservative default of a
    // hand-written manifest.
    let mut escondido = crawrad_registry::external::ExternalComponent::from(
        r.get("Gauge").expect("`Gauge` is registered"),
    );
    escondido.class = "GaugeOculto".to_string();
    escondido.private_state = true;
    r.register_external(escondido)
        .expect("passes the same rejections");

    let fonte = designer(
        &crate::GeradorMakepad,
        &r,
        &doc_com(&r, "GaugeOculto", "gau2"),
    );
    assert!(
        fonte.contains("..Default::default()"),
        "structs with fields undeclared in the manifest need crate default:\n{}",
        linhas_com(&fonte, "gau2")
    );
}

/// **The drawing becomes a widget tree, and the bar becomes `apply_over`.**
///
/// `live_design!` is text compiled once; the bar changes while the
/// application runs. Hardcoding the design value into the DSL would produce a gauge that starts
/// correct and never moves again.
#[test]
fn no_makepad_o_desenho_vira_widget_e_a_barra_vira_sync() {
    let r = catalogo_com_gauge();
    let fonte = designer(&crate::GeradorMakepad, &r, &doc_com(&r, "Gauge", "gau1"));
    assert!(
        fonte.contains("gau1 = <View> {"),
        "component did not become a widget:\n{}",
        linhas_com(&fonte, "gau1")
    );
    assert!(
        fonte.contains("d0 = <RoundedView> {"),
        "the `box` verb did not become `RoundedView`"
    );
    assert!(
        fonte.contains("self.ui.view(id!(gau1.d1)).apply_over(cx, live!{"),
        "fraction did not become runtime adjustment:\n{}",
        linhas_com(&fonte, "apply_over")
    );
    assert!(
        fonte.contains(
            "fraction(self.form.gau1.value as f32, self.form.gau1.min as f32, self.form.gau1.max as f32)"
        ),
        "bar calculation does not read the component field:\n{}",
        linhas_com(&fonte, "fraction")
    );
}

/// **The caption follows the property.**
#[test]
fn no_makepad_a_legenda_segue_a_propriedade() {
    let r = catalogo_com_gauge();
    let fonte = designer(&crate::GeradorMakepad, &r, &doc_com(&r, "Gauge", "gau1"));
    assert!(
        fonte.contains("let t = self.form.gau1.text.clone();"),
        "label does not read the property:\n{}",
        linhas_com(&fonte, "d3_text")
    );
}

// ---------------------------------------------------------------------------
// 3 — WHAT BOTH SHARE
// ---------------------------------------------------------------------------

/// **The emitted `fraction` is the SAME as the canvas, byte for byte.**
///
/// This is the gate that prevents the gauge from reading 65% in the designer and something else in the
/// application. It measures the text that goes into the user's project against the
/// file the canvas compiles — if someone rewrites the calculation on one side, this
/// test fails before the divergence reaches a screen.
#[test]
fn a_conta_da_barra_e_a_mesma_nos_dois_alvos_e_no_canvas() {
    let compartilhada = crate::targets::compartilhado::trecho_da_geometria("fraction");
    assert!(
        compartilhada.contains("pub fn fraction(value: f32, min: f32, max: f32) -> f32"),
        "snippet did not include function: {compartilhada}"
    );
    let r = catalogo_com_gauge();
    let d = doc_com(&r, "Gauge", "gau1");
    for (nome, fonte) in [
        ("egui", designer(&crate::GeradorRxp, &r, &d)),
        ("makepad", designer(&crate::GeradorMakepad, &r, &d)),
    ] {
        assert!(
            fonte
                .replace("\r\n", "\n")
                .contains(compartilhada.trim_end()),
            "[{nome}] target emitted calculation that differs from shared file"
        );
    }
}

/// **What nothing calls is not emitted.**
///
/// `Gauge` does not use dashes or checks. Emitting both functions would produce
/// `dead_code` in the project of someone who cannot fix it — `.designer.rs` is
/// rewritten on the next generation.
#[test]
fn a_peca_que_ninguem_chama_nao_viaja() {
    let r = catalogo_com_gauge();
    let fonte = designer(&crate::GeradorRxp, &r, &doc_com(&r, "Gauge", "gau1"));
    assert!(
        !fonte.contains("fn declared_dashed("),
        "dashed stroke was included, and `Gauge` does not use it"
    );
    assert!(
        !fonte.contains("fn declared_check("),
        "check mark was included, and `Gauge` does not use it"
    );
}

/// **And what SOMETHING calls is emitted** — otherwise the project does not compile.
///
/// The twin of the test above, and it is what makes the pair measure: without this, the
/// "do not emit" rule would pass green in a generation that never emits anything.
#[test]
fn a_peca_que_alguem_chama_viaja() {
    let r = catalogo_com_os_cinco_verbos();
    let d = doc_com(&r, "Cinco", "cin1");
    let fonte = designer(&crate::GeradorRxp, &r, &d);
    assert!(
        fonte.contains("fn declared_dashed("),
        "drawing calls dashed stroke and it was not included:\n{}",
        linhas_com(&fonte, "declared_")
    );
    assert!(
        fonte.contains("fn declared_check("),
        "drawing calls check mark and it was not included"
    );
    assert!(
        fonte.contains("declared_dashed(ui, a,") && fonte.contains("declared_check(ui, a,"),
        "calls were not emitted:\n{}",
        linhas_com(&fonte, "declared_")
    );
}

/// **All five verbs and all three areas cross over in both targets.**
///
/// `Gauge` uses four verbs and two areas. This case exists so that the fifth and
/// the inset do not stay outside all measurement — vocabulary exercised only
/// partially is vocabulary that breaks when someone uses the rest.
#[test]
fn os_cinco_verbos_atravessam_nos_dois_alvos() {
    let r = catalogo_com_os_cinco_verbos();
    let d = doc_com(&r, "Cinco", "cin1");

    let egui = designer(&crate::GeradorRxp, &r, &d);
    for agulha in [
        "ui.painter().rect_filled(a,",
        "ui.painter().rect_stroke(a, 0.0,",
        "ui.painter().text(",
        "declared_dashed(ui, a,",
        "declared_check(ui, a,",
        "r.shrink(4.0)",
        "fraction(",
    ] {
        assert!(egui.contains(agulha), "[egui] missing `{agulha}`");
    }

    let makepad = designer(&crate::GeradorMakepad, &r, &d);
    for agulha in [
        "<RoundedView> {",
        "<Label> {",
        "fn pixel(self) -> vec4 {",
        "Sdf2d::viewport(",
        "margin: {left: 4.0, top: 4.0}",
        "apply_over(cx, live!{",
    ] {
        assert!(makepad.contains(agulha), "[makepad] missing `{agulha}`");
    }
}

/// A third-party class that uses **all five verbs and all three areas**.
///
/// It originates here, not from a file, because no component in the suite uses
/// all five — and a vocabulary measured halfway is a vocabulary that breaks
/// on the day someone uses the other half.
fn catalogo_com_os_cinco_verbos() -> Registry {
    let prop = |chave: &str, tipo, padrao: crawrad_ir::Value| ExternalProp {
        key: chave.to_string(),
        display: chave.to_string(),
        default: padrao,
        category: crawrad_registry::Category::Appearance,
        kind: tipo,
        readonly: false,
        bindable: false,
        options: Vec::new(),
        ref_class: None,
        tree_collection: false,
        item_class: None,
        multiline: false,
        hidden_when: None,
    };
    use crawrad_registry::external::ExternalKind;
    let c = ExternalComponent {
        inherits: None,
        class: "Cinco".to_string(),
        display: "Cinco".to_string(),
        base_name: "cinco".to_string(),
        palette: "Additional".to_string(),
        prefix: "cin".to_string(),
        default_size: (180.0, 36.0),
        props: vec![
            ExternalProp {
                category: crawrad_registry::Category::Identity,
                ..prop(
                    "name",
                    ExternalKind::Text,
                    crawrad_ir::Value::Str(String::new()),
                )
            },
            ExternalProp {
                category: crawrad_registry::Category::Layout,
                ..prop(
                    "rect",
                    ExternalKind::Rect,
                    crawrad_ir::Value::Rect(Rect::new(0.0, 0.0, 180.0, 36.0)),
                )
            },
            prop(
                "text",
                ExternalKind::Text,
                crawrad_ir::Value::Str("Cinco".into()),
            ),
            prop(
                "value",
                ExternalKind::Int { min: 0, max: 100 },
                crawrad_ir::Value::Int(65),
            ),
        ],
        events: Vec::new(),
        targets: vec!["*".to_string()],
        children: crawrad_registry::external::ExternalChildren::None,
        derived_geometry: false,
        visual: true,
        emits_code: true,
        private_state: true,
        provides: Vec::new(),
        text_line: None,
        requires: Vec::new(),
        crate_module: "gauge_crawrad".to_string(),
        desenho: vec![
            Verb::all(Stroke::Box {
                fill: Ink::Surface,
                border: Ink::Border,
                radius: 2.0,
            }),
            Verb {
                area: Area::Fraction {
                    from: "value".to_string(),
                    axis: Axis::Y,
                    min: Limit::Fixed(0.0),
                    max: Limit::Fixed(100.0),
                },
                stroke: Stroke::Box {
                    fill: Ink::Accent,
                    border: Ink::Accent,
                    radius: 0.0,
                },
            },
            Verb::all(Stroke::Frame {
                color: Ink::Border,
                thickness: 1.0,
            }),
            Verb {
                area: Area::Inset(4.0),
                stroke: Stroke::Dashed {
                    color: Ink::TextDim,
                },
            },
            Verb {
                area: Area::Inset(4.0),
                stroke: Stroke::Check { color: Ink::Accent },
            },
            Verb::all(Stroke::Text {
                from: TextSource::Property("text".to_string()),
                color: Ink::Text,
                centered: true,
            }),
        ],
        icon: Vec::new(),
    };
    let mut r = Registry::builtin();
    r.register_external(c).expect("the port accepts");
    r
}

fn linhas_com(fonte: &str, agulha: &str) -> String {
    fonte
        .lines()
        .filter(|l| l.contains(agulha))
        .collect::<Vec<_>>()
        .join("\n")
}
