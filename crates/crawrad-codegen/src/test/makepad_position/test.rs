//! **Every component is positioned by `margin`**, and never by `abs_pos`.
//!
//! # The two defects this sits between
//!
//! On 17/09/2026 the generated Makepad application drew the background and
//! **nothing else** — not one component appeared. Every emitter
//! wrote a margin, and a margin did not draw.
//!
//! The first answer was `abs_pos`, and it drew. It was also wrong, and the
//! owner found out in one glance: a `ScrollBox` whose bar moved and whose
//! second button never came into view. *"It's there, but doesn't scroll like in egui
//! to view the 2nd button at the bottom."* **`abs_pos` is absolute, so it
//! ignores the container's scroll.**
//!
//! What was actually broken is narrower than it looked: `Label::draw_walk`
//! spends the walk twice (`makepad-widgets-1.0.0/src/label.rs:228-238`), while
//! `Button` (`button.rs:721-725`) and `View` spend it once. So the margin was
//! fine for everything except the `Label`, and the `Label` is now wrapped in a
//! `View` that carries the margin for it.
//!
//! This file guards the pair: positioning is a margin, and the `Label` comes
//! wrapped. Losing either one brings back a defect that is already paid for.
//!
//! # Why it sweeps the catalogue
//!
//! Fifteen emitters, and the sixteenth is the one written tomorrow. A list of
//! classes here would cover today's and miss that one — in a form with fifteen
//! right components and one that does not scroll.

use crate::targets::makepad::components;
use crate::targets::makepad::layout::posicao;
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;

/// The one point every component in this test is put at. Two different numbers
/// on purpose: an `x` equal to `y` would let a transposed position pass.
const X: f32 = 137.0;
const Y: f32 = 241.0;

/// The DSL for one instance of `class`, alone on a form.
fn dsl(class: &str) -> String {
    let r = Registry::builtin();
    let desc = r.get(class).expect("a class of the catalogue");
    let (w, h) = desc.default_size;
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .expect("the root")
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
    let mut n = r
        .instantiate(class, "sonda1", Rect::new(X, Y, w, h))
        .expect("the catalogue instantiates what it lists");
    n.set_name("sonda1");
    d.add_child("form_main", n).expect("goes on the form");
    d.validate().expect("valid");
    components::emit_dsl(
        &d,
        &crawrad_registry::Registry::builtin(),
        d.node("sonda1").expect("the probe"),
        X,
        Y,
        w,
        h,
    )
    .expect("the target emits it")
}

/// Which classes the Makepad target draws at all.
///
/// A non-visual (`Timer`, `ActionList`) emits nothing, and asking it for a
/// position would be asking the wrong question.
fn desenhaveis() -> Vec<&'static str> {
    Registry::builtin()
        .all()
        .map(|d| d.class)
        .filter(|c| *c != "Form")
        .filter(|c| !dsl(c).trim().is_empty())
        .collect()
}

#[test]
fn every_component_is_positioned_by_margin() {
    let esperado = posicao(X, Y);
    let mut sem = Vec::new();
    for class in desenhaveis() {
        if !dsl(class).contains(&esperado) {
            sem.push(class);
        }
    }
    assert!(
        sem.is_empty(),
        "these components do not come out with `{esperado}`: {sem:?}\n\n\
         Position here is a MARGIN, and it comes from `layout::posicao`. A \
         margin is measured from the parent's turtle, and the parent's turtle \
         already carries its scroll — that is what makes a child inside a \
         `ScrollBox` move when the user drags the bar."
    );
}

#[test]
fn no_component_is_positioned_by_abs_pos() {
    let mut culpados = Vec::new();
    for class in desenhaveis() {
        if dsl(class).contains("abs_pos") {
            culpados.push(class);
        }
    }
    assert!(
        culpados.is_empty(),
        "`abs_pos` is back in {culpados:?}.\n\n\
         It draws, and that is the trap: it is ABSOLUTE in the pass\
         (`makepad-draw-1.0.0/src/turtle.rs:534`), so it ignores the \
         container's scroll. The symptom is a box whose bar moves and whose \
         content stays — measured on screen on 17/09/2026."
    );
}

#[test]
fn the_label_comes_wrapped_in_a_view() {
    let saida = dsl("Label");
    assert!(
        saida.contains("sonda1 = <View>") && saida.contains("<Label>"),
        "the `Label` lost its wrapper:\n{saida}\n\n\
         `Label::draw_walk` spends the walk TWICE \
         (`makepad-widgets-1.0.0/src/label.rs:228-238`), so a margin on the \
         `Label` itself is applied twice and the text lands outside its own \
         turtle, clipped away. The `View` carries the margin; the `Label` \
         inside has none, and doubling zero is zero."
    );
    assert!(
        !saida.contains("sonda1 = <Label>"),
        "the margin went back onto the `Label` itself, and it will not draw"
    );
}

#[test]
fn position_is_written_in_one_place_only() {
    // The mutation caught by this test is the usual one: someone adds a new
    // component and writes the position line manually, "just like the others".
    // It would work today and diverge on the first change.
    let fontes = [
        include_str!("../../targets/makepad/components/button.rs"),
        include_str!("../../targets/makepad/components/label.rs"),
        include_str!("../../targets/makepad/components/edit.rs"),
        include_str!("../../targets/makepad/components/panel.rs"),
        include_str!("../../targets/makepad/components/tree_view.rs"),
    ];
    for f in fontes {
        assert!(
            f.contains("layout::posicao(x, y)"),
            "an emitter stopped asking `layout::posicao` for the position. \
             Fifteen copies of the same decision are what left the whole \
             target invisible, and the sixteenth copy starts like this."
        );
    }
}

#[test]
fn the_position_is_a_margin_and_not_an_absolute_point() {
    assert_eq!(
        posicao(10.0, 20.0),
        "margin: {left: 10, top: 20}",
        "`posicao` changed shape. It is the one place in the house that \
         decides how a component lands on the form of the Makepad target"
    );
}
