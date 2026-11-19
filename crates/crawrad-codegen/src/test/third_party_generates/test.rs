//! **The generator sees THIRD-PARTY components** — and this is the gate that did
//! not exist.
//!
//! # What was broken, and how it surfaced
//!
//! `Gerador::gerar(doc, nome_do_crate)` did not receive a catalogue, and
//! `crawrad-codegen` constructed `Registry::builtin()` on its own in seven
//! places. Consequence: a component registered via `register_external`
//! appeared on the palette, in the Object Inspector, and in the `.crawform`, and **disappeared in
//! code generation**.
//!
//! It was the last privileged gate of
//! ADR 0035,
//! and no one was measuring it.
//!
//! It surfaced through a convoluted path worth recording: I wrote a test
//! for the new `emit_children` rule, and **the mutation passed**. Investigating
//! why the test did not measure it, the reason was that only one collection existed in the
//! catalogue and its item had no handler. To measure properly, a
//! THIRD-PARTY collection was required — and the generator did not see it.
//!
//! # What this file enforces
//!
//! That a class that **never existed in this repository**, registered at
//! run time as ADR 0033 mandates, passes through the entire generator.

use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;
use crawrad_registry::external::{ExternalChildren, ExternalComponent, ExternalProp};

use crate::Gerador;

/// The `name` of a third-party component — the gate refuses anything with no
/// properties (`NoProps`), and that refusal is correct: a component without
/// properties does not even have a name.
fn prop_nome() -> ExternalProp {
    ExternalProp {
        key: "name".to_string(),
        display: "Name".to_string(),
        default: crawrad_ir::Value::Str(String::new()),
        category: crawrad_registry::Category::Identity,
        kind: crawrad_registry::external::ExternalKind::Text,
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

/// The `rect` every visual component declares — a real `manifest.json` does
/// (`components/gauge-crawrad`); without it the generator refuses the node.
fn prop_rect() -> ExternalProp {
    ExternalProp {
        key: "rect".to_string(),
        display: "Rect".to_string(),
        default: crawrad_ir::Value::Rect(crawrad_ir::Rect::new(0.0, 0.0, 200.0, 120.0)),
        category: crawrad_registry::Category::Layout,
        kind: crawrad_registry::external::ExternalKind::Rect,
        ..prop_nome()
    }
}

/// A catalogue with a third-party GRID, which is a collection of `Column`.
///
/// Neither `Grid` nor `Column` exist in `crawrad-registry`: both originate here,
/// just as they would from a `manifest.json`.
fn catalogo_com_grade_de_terceiro() -> Registry {
    let mut r = Registry::builtin();

    // **The item is a `Button`, and the choice allows the test to MEASURE.**
    //
    // The first version of this scenario used a third-party `Column`, and the test
    // passed with and without the rule: `Column` has no handler in the Makepad target, and
    // `emit_dsl` returns empty for a class without a handler — the test measured empty
    // both ways, exactly like the previous one that I had already discarded.
    //
    // With `Button` as the item, an `emit_children` that ignores `Filhos` emits
    // a real `<Button>`, and the difference appears. A button collection is strange
    // as a component; as a SCENARIO it is the only way for the rule to be
    // falsifiable — and a rule that no test can break is not being
    // measured.
    let grade = ExternalComponent {
        inherits: None,
        class: "GradeExterna".to_string(),
        base_name: "grade_externa".to_string(),
        // `GradeExterna` comes from a crate — which gives its type an address.
        crate_module: "grade_de_terceiro".to_string(),
        desenho: Vec::new(),
        icon: Vec::new(),
        display: "GradeExterna".to_string(),
        palette: "Data".to_string(),
        prefix: "grd".to_string(),
        default_size: (200.0, 120.0),
        props: vec![prop_nome(), prop_rect()],
        events: Vec::new(),
        targets: vec!["*".to_string()],
        // **The typed collection of ADR 0037**, declared by a third party.
        children: ExternalChildren::Collection {
            item: "Button".to_string(),
        },
        derived_geometry: false,
        visual: true,
        emits_code: false,
        private_state: true,
        provides: Vec::new(),
        text_line: None,
        requires: Vec::new(),
    };
    r.register_external(grade)
        .expect("`GradeExterna` passes the seven gate checks");

    r
}

/// A form with the third-party grid and two columns inside it.
fn doc_com_grade(r: &Registry) -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("the root");
        raiz.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        raiz.set_name("form_main");
    }
    let g = r
        .instantiate(
            "GradeExterna",
            "grd1".to_string(),
            Rect::new(8.0, 8.0, 200.0, 120.0),
        )
        .expect("the registry instantiates what it registered");
    d.add_child("form_main", g).expect("added to the form");
    for id in ["btn_a", "btn_b"] {
        let mut n = r
            .instantiate("Button", id.to_string(), Rect::new(0.0, 0.0, 80.0, 24.0))
            .expect("instantiates");
        n.set_name(id);
        d.add_child("grd1", n).expect("added to the collection");
    }
    d
}

/// **THE CATALOGUE PASSES THROUGH.** The third-party class reaches the generator.
///
/// Before 2026-09-18 this was impossible to write: the generator had no way
/// to receive any catalogue other than the built-in one.
#[test]
fn o_gerador_recebe_o_catalogo_com_a_classe_de_terceiro() {
    let r = catalogo_com_grade_de_terceiro();
    assert!(
        r.get("GradeExterna").is_some(),
        "the test registry must include the new class"
    );
    let d = doc_com_grade(&r);

    for g in [
        &crate::GeradorMakepad as &dyn Gerador,
        &crate::GeradorRxp as &dyn Gerador,
    ] {
        let arquivos = g
            .gerar(&d, "app", &r)
            .unwrap_or_else(|e| panic!("target `{}` rejected the document: {e}", g.alvo()));
        assert!(
            !arquivos.is_empty(),
            "target `{}` produced no files",
            g.alvo()
        );
    }
}

/// **THE COLLECTION RULE HOLDS FOR THIRD PARTIES** — and this is what the mutation
/// breaks.
///
/// `Grid` is `visual: true`, so it reaches the DSL path. The columns
/// inside it **must not** become widgets: a collection item is owner data.
///
/// Deleting the check in `emit_children` causes this test to fail — which the
/// previous test, written only with the built-in catalogue, could not do.
#[test]
fn a_colecao_de_terceiro_nao_vira_widget() {
    let r = catalogo_com_grade_de_terceiro();
    let d = doc_com_grade(&r);
    let no = d.node("grd1").expect("the grid");

    let dsl =
        crate::targets::makepad::components::emit_children(&d, &r, no, "    ").expect("emitting");
    assert!(
        dsl.trim().is_empty(),
        "items of a third-party `Grid` became DSL:\n{dsl}\n\n\
         Collection items are owner DATA, and the descriptor's `Children` \
         decides — which now reaches the generator."
    );
}
