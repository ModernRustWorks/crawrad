//! **DECLARED DRAWING** — a third-party component paints itself, without
//! a line of Rust.
//!
//! This is the other half of ADR 0036.
//! The first half — the component turning into CODE — was proven with `Connection`,
//! which is non-visual. This is for the **visual** component, which previously appeared
//! on the canvas as a dashed rectangle containing the class name
//! ([`super::generic::Desenho`]).
//!
//! # The IDE executes no third-party code, and that is the decision
//!
//! The manifest declares verbs; this file paints them with [`Brush`], which is the
//! same closed vocabulary that the twenty native drawings use. The component
//! **receives neither `Cx2d` nor shader**, and the IDE loads no library to
//! draw — the price WinForms and Lazarus paid for doing the opposite is
//! measured in ADR 0036.
//!
//! # Colors come from THEME
//!
//! The verb declares a ROLE (`Tinta`), and here it becomes a value. A
//! manifest cannot write `#ff0000`, so a component published on crates.io
//! follows the IDE theme instead of fighting it.

use crawrad_ir::drawing::{Area, Axis, Ink, Limit, Stroke, TextSource, Verb};
use makepad_widgets::Rect;

use crate::MakepadCtl;
use crate::canvas::Brush;
use crate::components::comum::*;
use crawrad_note::note;

#[note("components-declarado-cor")]
fn cor(t: Ink) -> [f32; 4] {
    match t {
        Ink::Surface => MAKEPAD_WIDGET,
        Ink::Border => MAKEPAD_BORDER,
        Ink::Text => MAKEPAD_TEXT,
        Ink::TextDim => MAKEPAD_TEXT_DIM,
        Ink::Accent => MAKEPAD_ACCENT,
    }
}

#[note("components-declarado-texto-de")]
fn texto_de(de: &TextSource, c: &MakepadCtl) -> String {
    texto_de_props(de, &c.props)
}

#[note("components-declarado-texto-de-props")]
pub(crate) fn texto_de_props(
    de: &TextSource,
    props: &std::collections::BTreeMap<String, crawrad_ir::Value>,
) -> String {
    match de {
        TextSource::Fixed(t) => t.clone(),
        TextSource::Property(chave) => match props.get(chave.as_str()) {
            Some(crawrad_ir::Value::Str(s)) => s.clone(),
            Some(crawrad_ir::Value::Enum(s)) => s.clone(),
            Some(crawrad_ir::Value::Int(i)) => i.to_string(),
            Some(crawrad_ir::Value::Bool(b)) => b.to_string(),
            // What the canvas cannot write remains BLANK, and does not become
            // the `Debug` output of a `Value` in the middle of the user's form.
            _ => String::new(),
        },
    }
}

#[note("components-declarado-pintar")]
pub fn pintar(p: &mut Brush, c: &MakepadCtl, r: Rect, verbos: &[Verb]) -> bool {
    if verbos.is_empty() {
        return false;
    }
    for v in verbos {
        // **WHERE comes before WHAT.** Area is the same question for all five,
        // and resolving it here is what kept the vocabulary at five when the
        // gauge requested a bar.
        let alvo = area_de(&v.area, c, r);
        match &v.stroke {
            Stroke::Box {
                fill,
                border,
                radius,
            } => p.rounded_box(alvo, cor(*fill), cor(*border), *radius as f64),
            Stroke::Frame { color, thickness } => p.frame(alvo, cor(*color), *thickness as f64),
            Stroke::Text {
                from,
                color,
                centered,
            } => {
                let t = texto_de(from, c);
                if !t.is_empty() {
                    p.write(p.inset(alvo), &t, cor(*color), *centered);
                }
            }
            Stroke::Dashed { color } => p.dashed(alvo, cor(*color)),
            Stroke::Check { color } => p.check(alvo, cor(*color)),
        }
    }
    true
}

#[note("components-declarado-area-de")]
fn area_de(a: &Area, c: &MakepadCtl, r: Rect) -> Rect {
    match a {
        Area::All => r,
        Area::Inset(n) => {
            let d = *n as f64;
            Rect {
                pos: makepad_widgets::dvec2(r.pos.x + d, r.pos.y + d),
                size: makepad_widgets::dvec2(
                    (r.size.x - d * 2.0).max(0.0),
                    (r.size.y - d * 2.0).max(0.0),
                ),
            }
        }
        Area::Fraction {
            from,
            axis,
            min,
            max,
        } => {
            let v = numero_de(from, c);
            // **THE RANGE CAN ALSO FOLLOW A PROPERTY**, which makes a
            // component's `min`/`max` take effect in DRAWING. While it was a
            // fixed number, a `Gauge` with `min = 50` had one calculation for
            // the painted bar and another for its `fraction()` method.
            let f = fracao(v, limite_de(min, c), limite_de(max, c)) as f64;
            match axis {
                Axis::X => Rect {
                    pos: r.pos,
                    size: makepad_widgets::dvec2(r.size.x * f, r.size.y),
                },
                Axis::Y => Rect {
                    pos: r.pos,
                    size: makepad_widgets::dvec2(r.size.x, r.size.y * f),
                },
            }
        }
    }
}

#[note("components-declarado-limite-de")]
fn limite_de(l: &Limit, c: &MakepadCtl) -> f32 {
    match l {
        Limit::Fixed(n) => *n,
        Limit::Property(k) => numero_de(k, c),
    }
}

#[note("components-declarado-numero-de")]
fn numero_de(chave: &str, c: &MakepadCtl) -> f32 {
    match c.props.get(chave) {
        Some(crawrad_ir::Value::Int(i)) => *i as f32,
        Some(crawrad_ir::Value::Float(f)) => *f as f32,
        _ => 0.0,
    }
}

#[note("components-declarado-fracao")]
fn fracao(v: f32, min: f32, max: f32) -> f32 {
    crawrad_component::fraction(v, min, max)
}

/// The same calculation, accessible by tests.
#[cfg(test)]
pub(crate) fn fracao_para_teste(v: f32, min: f32, max: f32) -> f32 {
    fracao(v, min, max)
}
