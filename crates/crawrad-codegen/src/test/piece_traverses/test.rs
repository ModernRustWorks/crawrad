//! **THE PIECE PASSES THROUGH** — what the component declares reaches the application.
//!
//! # The gap, and it was in the same family as the reference
//!
//! Both targets knew how to write five kinds of values: text, integer,
//! float, bool and enum. Everything else fell into a `_` and disappeared.
//!
//! The format accepts **eleven** property types. A third-party component
//! could declare `kind = "edges"` or `kind = "color"`, the Object Inspector
//! edited it, the `.crawform` stored it — and the application started with the field
//! at `Default`. The value died between the click and `cargo build`.
//!
//! # And the type had to travel along
//!
//! Writing the literal was not enough: `Edges { … }` in a project that does not know
//! `Edges` does not compile. Pieces live in the crate
//! (`crawrad-component/src/pieces/`) and are emitted into the user's
//! project — the same mechanics as `Strings` and `Anchors`.
//!
//! **And the question of WHICH to emit belongs to the descriptor**, not a list of class
//! names: no third-party component goes into a list written by us, and
//! it is precisely what can declare a `kind` that no native component uses.

use crawrad_ir::{Document, Rect, Value};
use crawrad_registry::external::{ExternalChildren, ExternalComponent, ExternalKind, ExternalProp};
use crawrad_registry::{Category, Registry};

use crate::Gerador;

fn prop(key: &str, kind: ExternalKind, default: Value) -> ExternalProp {
    ExternalProp {
        key: key.to_string(),
        display: key.to_string(),
        default,
        category: Category::Behavior,
        kind,
        readonly: false,
        bindable: false,
        options: Vec::new(),
        ref_class: None,
        tree_collection: false,
        item_class: None,
        multiline: false,
        hidden_when: None,
    }
}

/// A third-party component that declares the TWO new pieces.
fn catalogo() -> Registry {
    let mut r = Registry::builtin();
    r.register_external(ExternalComponent {
        inherits: None,
        class: "Medidor".to_string(),
        display: "Medidor".to_string(),
        base_name: "medidor".to_string(),
        palette: "Extras".to_string(),
        prefix: "med".to_string(),
        default_size: (120.0, 40.0),
        props: vec![
            ExternalProp {
                category: Category::Identity,
                ..prop("name", ExternalKind::Text, Value::Str(String::new()))
            },
            ExternalProp {
                category: Category::Layout,
                ..prop(
                    "rect",
                    ExternalKind::Rect,
                    Value::Rect(Rect::new(0.0, 0.0, 120.0, 40.0)),
                )
            },
            prop(
                "padding",
                ExternalKind::Edges,
                Value::Edges(crawrad_ir::Edges::uniform(3.0)),
            ),
            prop(
                "serie",
                ExternalKind::Color,
                Value::Color(crawrad_ir::Color::rgb(10, 20, 30)),
            ),
        ],
        events: Vec::new(),
        targets: vec!["*".to_string()],
        provides: Vec::new(),
        children: ExternalChildren::None,
        derived_geometry: false,
        visual: true,
        emits_code: true,
        private_state: true,
        text_line: None,
        requires: Vec::new(),
        crate_module: "medidor_de_terceiro".to_string(),
        desenho: Vec::new(),
        icon: Vec::new(),
    })
    .expect("`Medidor` passes the gate rejections");
    r
}

fn formulario(reg: &Registry) -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("the root");
        raiz.set_rect(Rect::new(0.0, 0.0, 420.0, 260.0));
        raiz.set_name("form_main");
    }
    let mut n = reg
        .instantiate(
            "Medidor",
            "med1".to_string(),
            Rect::new(8.0, 8.0, 120.0, 40.0),
        )
        .expect("the catalogue instantiates what it registered");
    n.set_name("med1");
    n.props.insert(
        "padding".to_string(),
        Value::Edges(crawrad_ir::Edges {
            left: 1.0,
            top: 2.0,
            right: 3.0,
            bottom: 4.0,
        }),
    );
    n.props.insert(
        "serie".to_string(),
        Value::Color(crawrad_ir::Color::rgb(200, 100, 50)),
    );
    d.add_child("form_main", n).expect("added to the form");
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

const ALVOS: [(&str, &dyn Gerador); 2] = [
    ("makepad", &crate::GeradorMakepad),
    ("egui", &crate::GeradorRxp),
];

/// **The VALUE arrives, on both targets.**
///
/// The four sides and four channels, exactly as the designer stored them.
#[test]
fn o_valor_da_peca_chega_ao_codigo() {
    let reg = catalogo();
    let doc = formulario(&reg);
    for (alvo, g) in ALVOS {
        let fonte = designer(g, &reg, &doc);
        assert!(
            fonte.contains("Edges { left: 1.0, top: 2.0, right: 3.0, bottom: 4.0 }"),
            "[{alvo}] `padding` did not reach:\n{}",
            linhas_com(&fonte, "med1")
        );
        assert!(
            fonte.contains("Color { r: 200, g: 100, b: 50, a: 255 }"),
            "[{alvo}] `serie` did not reach:\n{}",
            linhas_com(&fonte, "med1")
        );
    }
}

/// **The TYPE travels along**, otherwise the literal above does not compile.
#[test]
fn o_tipo_da_peca_viaja_junto() {
    let reg = catalogo();
    let doc = formulario(&reg);
    for (alvo, g) in ALVOS {
        let fonte = designer(g, &reg, &doc);
        assert!(
            fonte.contains("pub struct Edges {"),
            "[{alvo}] literal uses `Edges` and type was not emitted"
        );
        assert!(
            fonte.contains("pub struct Color {"),
            "[{alvo}] literal uses `Color` and type was not emitted"
        );
    }
}

/// **Pieces nobody uses do NOT travel.**
///
/// A type that nobody references is `dead_code` in the binary of whoever just dragged an
/// icon — and the designer file is rewritten whole, so warning there means
/// warning someone who cannot fix it.
#[test]
fn peca_que_ninguem_usa_nao_viaja() {
    let reg = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("the root");
        raiz.set_rect(Rect::new(0.0, 0.0, 420.0, 260.0));
        raiz.set_name("form_main");
    }
    let mut b = reg
        .instantiate(
            // a ProgressBar: no Font, no Color, no Margins of its own (an earlier change gave
            // the Button a Font and a Color, so it carries those pieces now)
            "ProgressBar",
            "btn1".to_string(),
            Rect::new(8.0, 8.0, 96.0, 32.0),
        )
        .expect("instantiates");
    b.set_name("btn1");
    d.add_child("form_main", b).expect("added");

    for (alvo, g) in ALVOS {
        let fonte = designer(g, &reg, &d);
        assert!(
            !fonte.contains("pub struct Edges {"),
            "[{alvo}] form with only `ProgressBar` carried the `Edges` piece along"
        );
        assert!(
            !fonte.contains("pub struct Color {"),
            "[{alvo}] form with only `ProgressBar` carried the `Color` piece along"
        );
    }
}

/// **The check inspects the DESCRIPTOR, not the class name.**
///
/// `Medidor` is in no list of this repository — it is registered
/// in the test, just like an installed package would be. If emission went back to checking
/// by class name, this is the test that turns red.
#[test]
fn a_peca_viaja_para_uma_classe_que_ninguem_listou() {
    let reg = catalogo();
    let doc = formulario(&reg);
    for (alvo, g) in ALVOS {
        let fonte = designer(g, &reg, &doc);
        assert!(
            fonte.contains("medidor_de_terceiro::Medidor"),
            "[{alvo}] external class did not even become a type"
        );
        assert!(
            fonte.contains("pub struct Edges {"),
            "[{alvo}] the piece would only travel if emission queried the descriptor"
        );
    }
}

fn linhas_com(fonte: &str, agulha: &str) -> String {
    fonte
        .lines()
        .filter(|l| l.contains(agulha))
        .collect::<Vec<_>>()
        .join("\n")
}
