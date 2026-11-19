//! **DECLARED DRAWING CROSSES OVER TO THE TARGET** — the third-party visual
//! component paints itself inside the generated application.
//!
//! # The gap this file closes, and why it was the worst of all
//!
//! On 18/09/2026 `Gauge` was already drawn on the IDE canvas from
//! `manifest.json` and already became `gauge_crawrad::Gauge` in generated code. Both
//! halves of ADR 0036
//! existed — and the application opened with the gauge **invisible**: the generator
//! calculated its rectangle and discarded it, and the user's `cargo build`
//! said so out loud (*"unused variable: `r`"*).
//!
//! This is worse than not supporting third-party visual components. The designer
//! showed one thing and the application showed another — the same family of defects
//! that `shared_geometry.rs` exists to prevent, sneaking in through the door of external
//! components.
//!
//! # What is shared, and what is not
//!
//! **Arithmetic**, always: the bar fraction comes from `fraction`, in
//! `shared_geometry.rs`, the same file that the canvas calls. A gauge showing
//! 65% in the designer shows 65% in the application because it is the same computation, not
//! because two separate computations happen to agree today.
//!
//! **Painting**, never — this is Amendment 1 of ADR 0036. The canvas paints with `Brush`
//! in an immediate-mode drawing widget; the egui application paints with `ui.painter()`.
//! There is no drawing text that compiles under both. What this file does is
//! translate the same closed vocabulary to the second.
//!
//! # Colors remain ROLES
//!
//! A verb declares `Tinta::Acento`, and here is where it becomes
//! `ui.visuals().selection.bg_fill`. This `match` is the only place in the egui target
//! where a role becomes a color — the canvas one is `cor()` in
//! `crawrad-runtime-makepad/src/componentes/declarado.rs`, and there are two because there are
//! two themes, not because anyone copied.

use crawrad_ir::drawing::{Area, Axis, Ink, Limit, Stroke, TextSource, Verb};
use crawrad_registry::{ComponentDesc, PropKind};

use crate::{literal, numero};
use crawrad_note::note;

#[note("targets-egui-declarado-pintura")]
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Pintura {
    /// The lines that go inside `show()`, already indented.
    pub corpo: String,
    /// This drawing calls `declared_dashed`.
    pub usa_tracejado: bool,
    /// This drawing calls `declared_check`.
    pub usa_marca: bool,
    /// This drawing calls `fraction`.
    pub usa_fracao: bool,
}

#[note("targets-egui-declarado-pintar")]
pub fn pintar(nome: &str, desc: &ComponentDesc, ligacoes: &[(String, String)]) -> Option<Pintura> {
    if desc.desenho.is_empty() {
        return None;
    }
    let mut p = Pintura::default();
    let mut dentro = String::new();
    for v in &desc.desenho {
        if let Some(t) = verbo(nome, desc, v, &mut p) {
            dentro.push_str(&t);
        }
    }
    if dentro.is_empty() {
        return None;
    }
    p.corpo = format!(
        "        if self.{nome}.visible {{\n\
         {}{dentro}{}\x20       }}\n",
        abre_interacao(nome, ligacoes),
        despacho(nome, ligacoes),
    );
    Some(p)
}

#[note("targets-egui-declarado-abre-interacao")]
fn abre_interacao(nome: &str, ligacoes: &[(String, String)]) -> String {
    if ligacoes.is_empty() {
        return String::new();
    }
    // The discriminant is the INSTANCE NAME, not a fixed word: the
    // entire form draws in a single `Ui`, so `ui.id()` has the same value
    // for all components. Two gauges with the same word would share
    // the click target, and clicking one would trigger the other.
    format!(
        "\x20           let _resp = ui.interact(\n\
         \x20               r,\n\
         \x20               ui.id().with((\"crawrad_declared\", \"{nome}\")),\n\
         \x20               egui::Sense::click(),\n\
         \x20           );\n"
    )
}

#[note("targets-egui-declarado-despacho")]
fn despacho(nome: &str, ligacoes: &[(String, String)]) -> String {
    let mut out = String::new();
    for (evento, handler) in ligacoes {
        let consulta = match evento.as_str() {
            "on_click" => "_resp.clicked()",
            // The table and this `match` grow together. An event arriving here
            // without a pair is an event the table declares and this target does not deliver —
            // and the `verbo_bind` gate enforces exactly this set.
            _ => continue,
        };
        out.push_str(&format!(
            "\x20           if self.{nome}.enabled && {consulta} {{\n\
             \x20               self.{handler}();\n\
             \x20           }}\n"
        ));
    }
    out
}

#[note("targets-egui-declarado-verbo")]
fn verbo(nome: &str, desc: &ComponentDesc, v: &Verb, p: &mut Pintura) -> Option<String> {
    let area = area(nome, desc, &v.area, p)?;
    let pincelada = pincelada(nome, desc, &v.stroke, p)?;
    Some(format!(
        "\x20           {{\n\
         \x20               let a = {area};\n\
         {pincelada}\x20           }}\n"
    ))
}

#[note("targets-egui-declarado-area")]
fn area(nome: &str, desc: &ComponentDesc, a: &Area, p: &mut Pintura) -> Option<String> {
    match a {
        Area::All => Some("r".to_string()),
        Area::Inset(n) => Some(format!("r.shrink({})", numero(*n))),
        Area::Fraction {
            from,
            axis,
            min,
            max,
        } => {
            let valor = numerico(nome, desc, from)?;
            // **THE RANGE CAN ALSO FOLLOW A PROPERTY**, and that is what makes
            // the crate's `Gauge.min`/`Gauge.max` effective: while the range was a
            // fixed number, changing them at runtime altered the component's `fraction()`
            // without updating the painted bar.
            let lim_min = limite(nome, desc, min)?;
            let lim_max = limite(nome, desc, max)?;
            p.usa_fracao = true;
            let tamanho = match axis {
                Axis::X => "egui::vec2(r.width() * f, r.height())",
                Axis::Y => "egui::vec2(r.width(), r.height() * f)",
            };
            Some(format!(
                "{{\n\
                 \x20                   let f = fraction({valor}, {lim_min}, {lim_max});\n\
                 \x20                   egui::Rect::from_min_size(r.min, {tamanho})\n\
                 \x20               }}"
            ))
        }
    }
}

#[note("targets-egui-declarado-pincelada")]
fn pincelada(nome: &str, desc: &ComponentDesc, b: &Stroke, p: &mut Pintura) -> Option<String> {
    let linha = |t: &str| format!("\x20               {t}\n");
    match b {
        Stroke::Box {
            fill,
            border,
            radius,
        } => Some(format!(
            "{}{}",
            linha(&format!(
                "ui.painter().rect_filled(a, {}, {});",
                numero(*radius),
                cor(*fill)
            )),
            linha(&format!(
                "ui.painter().rect_stroke(a, {}, egui::Stroke::new(1.0, {}), egui::StrokeKind::Inside);",
                numero(*radius),
                cor(*border)
            )),
        )),
        Stroke::Frame { color, thickness } => Some(linha(&format!(
            "ui.painter().rect_stroke(a, 0.0, egui::Stroke::new({}, {}), egui::StrokeKind::Inside);",
            numero(*thickness),
            cor(*color)
        ))),
        Stroke::Text {
            from,
            color,
            centered,
        } => texto(nome, desc, from, *color, *centered),
        Stroke::Dashed { color } => {
            p.usa_tracejado = true;
            Some(linha(&format!(
                "declared_dashed(ui, a, egui::Stroke::new(1.0, {}));",
                cor(*color)
            )))
        }
        Stroke::Check { color } => {
            p.usa_marca = true;
            Some(linha(&format!(
                "declared_check(ui, a, egui::Stroke::new(2.0, {}));",
                cor(*color)
            )))
        }
    }
}

#[note("targets-egui-declarado-texto")]
fn texto(
    nome: &str,
    desc: &ComponentDesc,
    de: &TextSource,
    color: Ink,
    centered: bool,
) -> Option<String> {
    let (pos, align) = if centered {
        ("a.center()", "egui::Align2::CENTER_CENTER")
    } else {
        (
            "egui::pos2(a.min.x + 4.0, a.center().y)",
            "egui::Align2::LEFT_CENTER",
        )
    };
    let desenha = |texto: &str| {
        format!(
            "ui.painter().text(\n\
             \x20                       {pos},\n\
             \x20                       {align},\n\
             \x20                       {texto},\n\
             \x20                       egui::FontId::proportional(12.0),\n\
             \x20                       {},\n\
             \x20                   );\n",
            cor(color)
        )
    };
    match de {
        TextSource::Fixed(t) if t.is_empty() => None,
        TextSource::Fixed(t) => Some(format!("\x20               {}", desenha(&literal(t)))),
        TextSource::Property(chave) => {
            let prop = desc.props.iter().find(|p| p.key == *chave)?;
            let campo = format!("self.{nome}.{chave}");
            match prop.kind {
                // Text: empty guard, and `as_str` instead of cloning every
                // frame.
                PropKind::String | PropKind::Multiline | PropKind::Enum(_) => Some(format!(
                    "\x20               if !{campo}.is_empty() {{\n\
                     \x20                   {}\
                     \x20               }}\n",
                    desenha(&format!("{campo}.as_str()"))
                )),
                // Numbers and booleans are never empty, and egui's `text` accepts
                // any `ToString` — nothing to convert by hand.
                PropKind::Int { .. } | PropKind::Float { .. } | PropKind::Bool => {
                    Some(format!("\x20               {}", desenha(&campo)))
                }
                // What this target cannot format is omitted, rather than
                // becoming a `Debug` representation of a value in the middle of the form.
                _ => None,
            }
        }
    }
}

#[note("targets-egui-declarado-limite")]
fn limite(nome: &str, desc: &ComponentDesc, l: &Limit) -> Option<String> {
    match l {
        Limit::Fixed(n) => Some(numero(*n)),
        Limit::Property(k) => numerico(nome, desc, k),
    }
}

#[note("targets-egui-declarado-numerico")]
fn numerico(nome: &str, desc: &ComponentDesc, chave: &str) -> Option<String> {
    let prop = desc.props.iter().find(|p| p.key == chave)?;
    match prop.kind {
        PropKind::Int { .. } | PropKind::Float { .. } => {
            Some(format!("self.{nome}.{chave} as f32"))
        }
        _ => None,
    }
}

#[note("targets-egui-declarado-cor")]
fn cor(t: Ink) -> &'static str {
    match t {
        Ink::Surface => "ui.visuals().extreme_bg_color",
        Ink::Border => "ui.visuals().widgets.noninteractive.fg_stroke.color",
        Ink::Text => "ui.visuals().text_color()",
        Ink::TextDim => "ui.visuals().weak_text_color()",
        Ink::Accent => "ui.visuals().selection.bg_fill",
    }
}
