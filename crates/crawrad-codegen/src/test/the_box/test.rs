//! **THE BOX has ONE source, and four readers.**
//!
//! # The owner's question, answered by this file
//!
//! > *"We are making a bunch of Lego pieces, but we have to offer the
//! > box. (…) Where is our box so people can create components?"*
//!
//! `crawrad-component` is the box. What this gate guards is the property that
//! makes it useful instead of dangerous: the piece that the box offers is **the same**
//! that the designer canvas uses and **the same** that the generated application receives.
//!
//! A box with a copy of the calculation would be worse than none: anyone writing
//! a component with it would see the gauge show one thing in the designer and another on
//! the screen — the bug family that this project has tracked since `LIST_ROW_HEIGHT`.

/// The box, as text.
const CAIXA: &str = include_str!("../../../../crawrad-component/src/fraction.rs");

/// **What the target emits is what the box holds, byte for byte.**
#[test]
fn o_texto_emitido_e_o_da_caixa() {
    let recortado = crate::targets::compartilhado::trecho_da_geometria("fraction");
    assert!(
        recortado.contains("pub fn fraction(value: f32, min: f32, max: f32) -> f32"),
        "extracted snippet did not include the function: {recortado}"
    );
    assert!(
        CAIXA.replace("\r\n", "\n").contains(recortado.trim_end()),
        "generator emits text that is not in the box"
    );
}

/// **The designer canvas CALLS the box** — it has no calculation of its own.
///
/// Reads the source as text, rather than the result: a calculation rewritten by hand in the canvas
/// would give the same numbers today and diverge on the first change. What must
/// stay locked is the call existing.
#[test]
fn o_canvas_chama_a_caixa_em_vez_de_ter_conta_propria() {
    let canvas = include_str!("../../../../crawrad-runtime-makepad/src/components/declarado.rs");
    assert!(
        canvas.contains("crawrad_component::fraction("),
        "canvas stopped calling the box"
    );
    assert!(
        !canvas.contains("f32::EPSILON"),
        "a handwritten fraction calculation returned in canvas"
    );
}

/// **The suite component also calls the box.**
///
/// It was the THIRD copy: `Gauge::fraction()` reimplemented the same clamp.
/// A component that does not use the box is proof that the box is useless — and it
/// is the first one anyone opens to learn how to write their own.
#[test]
fn o_componente_da_suite_chama_a_caixa() {
    let gauge = include_str!("../../../../../components/gauge-crawrad/src/lib.rs");
    assert!(
        gauge.contains("crawrad_component::fraction("),
        "`Gauge` has its own calculation again"
    );
    let cargo = include_str!("../../../../../components/gauge-crawrad/Cargo.toml");
    assert!(
        cargo.contains("crawrad-component"),
        "`Gauge` does not depend on the box — and that was the hole it came to close"
    );
}

/// **THE BOX DOES NOT CARRY THE IDE ALONG.**
///
/// That is the constraint making it installable: it enters the binary of someone who only
/// dragged an icon. A dependency on Makepad, egui, or `Document`
/// would make every component cost minutes of build time (ADR 0033 §3) — and would make the
/// user application carry the IDE inside, which is the exact thing this product
/// exists not to be.
#[test]
fn a_caixa_nao_tem_dependencia_nenhuma() {
    let cargo = include_str!("../../../../crawrad-component/Cargo.toml");
    let deps = cargo
        .split("[dependencies]")
        .nth(1)
        .expect("dependencies section");
    let linhas: Vec<&str> = deps
        .lines()
        .map(str::trim)
        .take_while(|l| !l.starts_with('['))
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect();
    assert!(
        linhas.is_empty(),
        "the box gained a dependency, and it enters the binary of anyone using a \
         component: {linhas:?}"
    );
}

/// **And it does not know any crate of ours.**
///
/// What would prove interpretation would be passing the document and the catalogue. A `Strings`
/// type and a division prove none of that — but a `crawrad_ir` here
/// would.
#[test]
fn a_caixa_nao_conhece_o_resto_da_casa() {
    for (nome, fonte) in [
        (
            "lib.rs",
            include_str!("../../../../crawrad-component/src/lib.rs"),
        ),
        ("fraction.rs", CAIXA),
        (
            "manifest.rs",
            include_str!("../../../../crawrad-component/src/manifest.rs"),
        ),
        (
            "pieces/mod.rs",
            include_str!("../../../../crawrad-component/src/pieces/mod.rs"),
        ),
        // THE PIECE itself, which moved down from `crawrad-runtime` to here on 2026-09-18. It
        // is the most important in this list: it is emitted VERBATIM into
        // the user's project, so any crate of ours cited here
        // would travel along.
        (
            "pieces/strings.rs",
            include_str!("../../../../crawrad-component/src/pieces/strings.rs"),
        ),
    ] {
        for (i, linha) in fonte.lines().enumerate() {
            let t = linha.trim_start();
            if t.starts_with("//") {
                continue;
            }
            assert!(
                !t.contains("crawrad_ir")
                    && !t.contains("crawrad_registry")
                    && !t.contains("crawrad_doc"),
                "{nome}:{}: the box cited an IDE crate — `{}`",
                i + 1,
                linha.trim()
            );
        }
    }
}
