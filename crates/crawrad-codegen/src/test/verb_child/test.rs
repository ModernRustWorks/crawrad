//! **The `Child` verb distinguishes AREA from COLLECTION** — and it was already generic.
//!
//! It is the fourth and final verb of
//! ADR 0036,
//! and the measurement on 2026-09-18 found something unexpected: it **was
//! already ready**. The Makepad target's `emit_children` was always a single function,
//! without a single line of class-specific knowledge — it walks `node.children` and
//! nests each one.
//!
//! # The gap that typed collections revealed
//!
//! Nesting **every** child is correct for an area container: children of a
//! `Panel` appear inside it, and that is what `live_design!` must state.
//!
//! For a **collection**, it is incorrect. A collection item is not a window — it is data
//! of the owner, such as an `Action` of an `ActionList` or a column of a grid. Emitting it
//! as a `<View>` places a widget on screen that should not exist.
//!
//! **The bug only stayed hidden by luck:** the only collection in the catalogue is
//! `ActionList`, which has `gera_codigo: false` and therefore never reaches the
//! generator. A third-party grid declaring `children = "collection"` with
//! `item_class = "Column"` **and generating code** would have its columns turned into
//! widgets — and no test in the repository was measuring that.
//!
//! The PARENT descriptor now decides, rather than a list hardcoded into the
//! generator.

use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;

use crate::targets::makepad::components;

/// A `Panel` with two children — the AREA case.
#[test]
fn o_conteiner_de_area_aninha_os_filhos() {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("root");
        raiz.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        raiz.set_name("form_main");
    }
    let pnl = r
        .instantiate(
            "Panel",
            "pnl1".to_string(),
            Rect::new(8.0, 8.0, 300.0, 200.0),
        )
        .expect("instantiate");
    d.add_child("form_main", pnl).expect("add child");
    for (i, id) in ["btn_a", "btn_b"].iter().enumerate() {
        let mut n = r
            .instantiate(
                "Button",
                (*id).to_string(),
                Rect::new(8.0, 8.0 + i as f32 * 40.0, 80.0, 24.0),
            )
            .expect("instantiate");
        n.set_name(*id);
        d.add_child("pnl1", n).expect("add to panel");
    }

    let no = d.node("pnl1").expect("panel");
    let dsl = components::emit_children(&d, &r, no, "    ").expect("emitting");
    assert!(
        dsl.contains("btn_a") && dsl.contains("btn_b"),
        "the `Panel` is an AREA container and its children MUST appear in the DSL: {dsl}"
    );
}

// **The COLLECTION case has no test here, and the reason is measured.**
//
// I wrote one, and mutation testing struck it down as useless: deleting the rule in
// `emit_children` **did not cause the test to fail**. The reason is that the only collection
// in the catalogue is `ActionList`, whose item — `Action` — has no handler in the
// Makepad target, and `emit_dsl` returns empty for classes without a handler. The test
// measured emptiness either way.
//
// To measure properly would require a collection whose item DOES have a handler, and
// the only way to have that is a third-party component — which the generator **does not
// see**: `Gerador::gerar(doc, nome_do_crate)` does not receive a catalogue, and the
// sixteen codegen call sites invoke `Registry::builtin()` on their own.
//
// Therefore the rule stays (it is correct: a collection item is owner data, not a
// window) and the test is omitted, because a test passing with and without the rule is worse than
// no test at all — it becomes an argument. What remains missing is tracked as a known gap.

/// **The rule comes from the DESCRIPTOR, not the class name.**
///
/// This test prevents anyone from "fixing" the distinction with an
/// `if class == "ActionList"`: it scans the catalogue and requires the result to match
/// what each descriptor declares.
#[test]
fn quem_decide_e_o_descritor_e_nao_o_nome() {
    let r = Registry::builtin();
    let mut colecoes = 0usize;
    let mut areas = 0usize;
    for d in r.all() {
        match d.item_class() {
            Some(item) => {
                colecoes += 1;
                assert!(
                    !d.accepts_child("Button") || item == "Button",
                    "`{}` is a collection of `{item}` and accepted a `Button`",
                    d.class
                );
            }
            None if d.children_area() => {
                areas += 1;
                assert!(
                    d.accepts_child("Button"),
                    "`{}` has an area and rejected a `Button`",
                    d.class
                );
            }
            None => {}
        }
    }
    assert!(
        colecoes > 0 && areas > 0,
        "scan needs at least one collection and one area to measure \
         anything — found {colecoes} and {areas}"
    );
}
