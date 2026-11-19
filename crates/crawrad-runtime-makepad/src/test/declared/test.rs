//! **Declared drawing** — what can be measured without a screen, and what cannot.
//!
//! This is the missing half of
//! ADR 0036.
//! The first half — the component becoming CODE — was proven with `Connection`,
//! which is non-visual. This is for the component that **appears on screen**, and which
//! until 2026-09-18 was rendered as a dashed rectangle with the class name inside.
//!
//! # What this file does NOT measure, and why
//!
//! **The painting.** [`crate::canvas::Brush`] requires a `Cx2d`, which does not
//! exist outside a framework frame — and inventing a mock `Brush` just
//! for the test would measure the mock, not the drawing.
//!
//! What is measured here is everything that is **pure**: the vocabulary being closed, the color
//! being a role and never a value, and the text coming from the right property. The painting is
//! proven by **looking**, with the IDE open — and that is how it was verified.

use crawrad_ir::drawing::{Area, Axis, Ink, Stroke, TextSource, Verb};

use crate::components::comum::*;

/// **The color is a ROLE, and never a value.**
///
/// This is what prevents a component published on crates.io from hardcoding
/// `#ff0000` and becoming immune to the IDE theme — and the reason has been documented
/// since before third parties existed, in `desenho_comum.rs` of the egui target:
/// *"the day the theme becomes a form property, a file containing `Color32::from_rgb`
/// would have to be rewritten entirely"*.
#[test]
fn a_tinta_e_papel_e_nunca_valor() {
    for t in Ink::ALL {
        assert_eq!(
            Ink::from_name(t.name()),
            Some(*t),
            "role `{}` does not round-trip by name",
            t.name()
        );
    }
    assert_eq!(
        Ink::from_name("#ff0000"),
        None,
        "literal color must be rejected in NAME, before reaching the canvas"
    );
    assert_eq!(Ink::from_name("vermelho"), None);
}

/// **The theme resolves all roles, without gaps.**
///
/// A role without color would be an invisible component, and the `match` in
/// `declarado::cor` is exhaustive — this test exists for the day someone
/// adds a role and forgets the theme.
#[test]
fn todo_papel_tem_cor_no_tema() {
    let cores: Vec<[f32; 4]> = Ink::ALL
        .iter()
        .map(|t| match t {
            Ink::Surface => MAKEPAD_WIDGET,
            Ink::Border => MAKEPAD_BORDER,
            Ink::Text => MAKEPAD_TEXT,
            Ink::TextDim => MAKEPAD_TEXT_DIM,
            Ink::Accent => MAKEPAD_ACCENT,
        })
        .collect();
    assert_eq!(cores.len(), Ink::ALL.len());
    // And they are not all the same color — which would be a single-color theme.
    let distintas: std::collections::BTreeSet<String> =
        cores.iter().map(|c| format!("{c:?}")).collect();
    assert!(
        distintas.len() >= 4,
        "roles collapsed in theme: {distintas:?}"
    );
}

/// **The text comes from the PROPERTY**, which is what makes the drawing follow the
/// Object Inspector instead of always displaying the same word.
#[test]
fn o_texto_declarado_le_a_propriedade_do_no() {
    let props: std::collections::BTreeMap<String, crawrad_ir::Value> = [
        ("text".to_string(), crawrad_ir::Value::Str("Sales".into())),
        ("n".to_string(), crawrad_ir::Value::Int(42)),
        (
            "r".to_string(),
            crawrad_ir::Value::Rect(crawrad_ir::Rect::new(1.0, 2.0, 3.0, 4.0)),
        ),
    ]
    .into_iter()
    .collect();

    let ler = |f: &TextSource| crate::components::declarado::texto_de_props(f, &props);

    assert_eq!(ler(&TextSource::Fixed("fixed".into())), "fixed");
    assert_eq!(
        ler(&TextSource::Property("text".into())),
        "Sales",
        "drawing must follow what the user typed"
    );
    assert_eq!(ler(&TextSource::Property("n".into())), "42");
    assert_eq!(
        ler(&TextSource::Property("missing".into())),
        "",
        "nonexistent property stays blank, and does not become an error on screen"
    );
    assert_eq!(
        ler(&TextSource::Property("r".into())),
        "",
        "value that canvas cannot render stays BLANK — does not leak internal \
         representation into the middle of the form"
    );
}

/// **The vocabulary is closed and small.**
///
/// Five verbs. For scale: `Brush`, which is the production painting vocabulary,
/// has twelve — and this is a subset of it turned into data.
///
/// If it starts growing with each new component, the conclusion is not to loosen it:
/// it is that it is not a vocabulary, it is a list, and ADR 0036 is wrong in this part.
#[test]
fn o_vocabulario_de_desenho_e_pequeno() {
    // The `match` is exhaustive by design: adding a variant without going
    // through here will not compile, and that is what keeps the count honest.
    let todos = [
        Stroke::Box {
            fill: Ink::Surface,
            border: Ink::Border,
            radius: 0.0,
        },
        Stroke::Frame {
            color: Ink::Border,
            thickness: 1.0,
        },
        Stroke::Text {
            from: TextSource::Fixed(String::new()),
            color: Ink::Text,
            centered: false,
        },
        Stroke::Dashed {
            color: Ink::TextDim,
        },
        Stroke::Check { color: Ink::Accent },
    ];
    for v in &todos {
        match v {
            Stroke::Box { .. }
            | Stroke::Frame { .. }
            | Stroke::Text { .. }
            | Stroke::Dashed { .. }
            | Stroke::Check { .. } => {}
        }
    }
    assert_eq!(todos.len(), 5, "vocabulary changed size");
}

// ---------------------------------------------------------------------------
// THE AREA — what almost made the vocabulary a sixth verb
// ---------------------------------------------------------------------------

/// **The fraction turns into size, and is clamped.**
///
/// This is the test for the component that broke the format: a gauge that displays a
/// VALUE did not fit into five verbs that only knew how to paint the entire box. The
/// answer was not a `bar` verb — it was separating WHERE from WHAT.
#[test]
fn a_fracao_segue_o_valor_e_e_aparada() {
    let casos = [
        (0, 0.0),
        (20, 0.2),
        (65, 0.65),
        (100, 1.0),
        // Out of bounds, on both sides: a component does not paint outside its
        // own box, nor does it fail to paint due to a negative value.
        (-30, 0.0),
        (500, 1.0),
    ];
    for (valor, esperado) in casos {
        let f = crate::components::declarado::fracao_para_teste(valor as f32, 0.0, 100.0);
        assert!(
            (f - esperado).abs() < 1e-6,
            "value {valor} should yield fraction {esperado}, but gave {f}"
        );
    }
}

/// **Degenerate range does not divide by zero.**
///
/// `min == max` is an invalid manifest, and the answer is an EMPTY bar — not a
/// `NaN` painted on screen or a panic during form drawing.
#[test]
fn faixa_sem_largura_da_barra_vazia() {
    let f = crate::components::declarado::fracao_para_teste(50.0, 10.0, 10.0);
    assert_eq!(f, 0.0, "zero-width range must yield an empty bar");
}

/// **The axis round-trips by name**, like ink.
#[test]
fn o_eixo_e_fechado() {
    assert_eq!(Axis::from_name("x"), Some(Axis::X));
    assert_eq!(Axis::from_name("y"), Some(Axis::Y));
    assert_eq!(Axis::from_name("z"), None, "there is no third axis here");
    assert_eq!(Axis::from_name("horizontal"), None);
}

/// The default area is the entire rectangle — what the five used to do before.
#[test]
fn a_area_padrao_e_tudo() {
    assert_eq!(Area::default(), Area::All);
    assert_eq!(
        Verb::all(Stroke::Dashed {
            color: Ink::TextDim
        })
        .area,
        Area::All
    );
}
