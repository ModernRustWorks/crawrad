//! **DECLARED DRAWING IN THE MAKEPAD TARGET** — third-party visual components
//! appear in the application, and not just on the designer canvas.
//!
//! # The gap was bigger on this side
//!
//! In the egui target, third-party components already turned into code and only
//! the drawing was missing. Here they **did not exist**: `GeradorMakepad` filtered
//! nodes by the list of classes registered for this target, and third-party classes
//! were dropped without a word. A form with a `Gauge` generated `pub struct FormMain {}`
//! — empty. No field, no property, no drawing.
//!
//! It was the last privileged door of
//! ADR 0035, and
//! it was in the DEFAULT target.
//!
//! # What crosses both sides, and what does not
//!
//! **Arithmetic** crosses over: the bar fraction comes from `fraction` in
//! `shared_geometry.rs`, and the five colors come from the roles in `shared_theme.rs` —
//! the same values the canvas paints. This is what makes a gauge at 65% in the
//! designer become a gauge at 65%, with the same color, in the application.
//!
//! **Painting** does not, and that is Amendment 1 of ADR 0036. The canvas uses `Brush` in
//! an immediate drawing widget; the application builds a retained tree from
//! `live_design!`. This file translates the closed vocabulary to the latter.
//!
//! # The two strokes the framework does not have
//!
//! `Tracejado` and `Marca` are not any widget in `makepad-widgets`. They are emitted
//! as a **shader** in the `draw_bg` of a `<View>`, which is the same route the framework's
//! own `CheckBox` uses for its checkmark
//! (`makepad-widgets-1.0.0/src/check_box.rs:185-189`). The other three are
//! `<RoundedView>` and `<Label>`, and do not need a single line of shader.

use crawrad_ir::drawing::{Area, Axis, Ink, Limit, Stroke, TextSource, Verb};
use crawrad_registry::{ComponentDesc, PropKind};

use crate::numero;
use crate::targets::compartilhado::{hex, tema};
use crawrad_note::note;

#[note("targets-makepad-declarado-declarado")]
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Declarado {
    /// Child widgets, placed inside the component's `<View>`.
    pub dsl: String,
    /// Lines for `sync_ui` — what depends on values that change while running.
    pub sync: String,
    /// This drawing calls `fraction`.
    pub usa_fracao: bool,
    /// This drawing has a stroke rendered via SHADER.
    ///
    /// Only these require `use makepad_draw::shader::std::*;` at the top of
    /// `live_design!`, and a form having none does not carry it:
    /// `.designer.rs` is a file someone opens and reads.
    pub usa_shader: bool,
}

#[note("targets-makepad-declarado-desenho")]
pub fn desenho(field: &str, desc: &ComponentDesc, w: f32, h: f32) -> Option<Declarado> {
    if desc.desenho.is_empty() {
        return None;
    }
    let mut d = Declarado::default();
    for (i, v) in desc.desenho.iter().enumerate() {
        verbo(field, desc, v, i, w, h, &mut d);
    }
    if d.dsl.is_empty() {
        return None;
    }
    Some(d)
}

#[note("targets-makepad-declarado-verbo")]
fn verbo(field: &str, desc: &ComponentDesc, v: &Verb, i: usize, w: f32, h: f32, d: &mut Declarado) {
    let id = format!("d{i}");
    let Some(area) = area(field, desc, &v.area, &id, w, h, d) else {
        return;
    };
    let Some((widget, dentro, sync)) = pincelada(field, desc, &v.stroke, &id, d) else {
        return;
    };
    d.dsl.push_str(&format!(
        "                {id} = <{widget}> {{\n\
         {area}{dentro}\
         \x20               }}\n"
    ));
    d.sync.push_str(&sync);
}

#[note("targets-makepad-declarado-area")]
fn area(
    field: &str,
    desc: &ComponentDesc,
    a: &Area,
    id: &str,
    w: f32,
    h: f32,
    d: &mut Declarado,
) -> Option<String> {
    match a {
        Area::All => Some("                    width: Fill, height: Fill\n".to_string()),
        Area::Inset(n) => Some(format!(
            "                    margin: {{left: {}, top: {}}}\n\
             \x20                   width: {}, height: {}\n",
            numero(*n),
            numero(*n),
            numero((w - n * 2.0).max(0.0)),
            numero((h - n * 2.0).max(0.0)),
        )),
        Area::Fraction {
            from,
            axis,
            min,
            max,
        } => {
            let valor = numerico(field, desc, from)?;
            // **THE RANGE CAN ALSO FOLLOW A PROPERTY.** While it was a
            // fixed number, a component with its own `min`/`max` had one
            // calculation for the painted bar and another for its method.
            let lim_min = limite(field, desc, min)?;
            let lim_max = limite(field, desc, max)?;
            d.usa_fracao = true;
            // **The initial size is ZERO, and `sync_ui` fills it in.**
            //
            // `live_design!` is text compiled once; the bar value changes
            // while the application runs. Hardcoding the design value here would
            // yield a gauge that starts right and never moves again — which is
            // the defect this file exists to prevent.
            let (inicial, campo, total) = match axis {
                Axis::X => ("width: 0.0, height: Fill", "width", w),
                Axis::Y => ("width: Fill, height: 0.0", "height", h),
            };
            d.sync.push_str(&format!(
                "        self.ui.view(id!({field}.{id})).apply_over(cx, live!{{\n\
                 \x20           {campo}: (({} * fraction({valor}, {lim_min}, {lim_max})))\n\
                 \x20       }});\n",
                numero(total),
            ));
            Some(format!("                    {inicial}\n"))
        }
    }
}

#[note("targets-makepad-declarado-pincelada")]
fn pincelada(
    field: &str,
    desc: &ComponentDesc,
    b: &Stroke,
    id: &str,
    d: &mut Declarado,
) -> Option<(&'static str, String, String)> {
    match b {
        Stroke::Box {
            fill,
            border,
            radius,
        } => Some((
            "RoundedView",
            format!(
                "                    show_bg: true\n\
                 \x20                   draw_bg: {{ color: {}, border_color: {}, border_size: 1.0, border_radius: {} }}\n",
                cor(*fill),
                cor(*border),
                numero(*radius),
            ),
            String::new(),
        )),
        Stroke::Frame { color, thickness } => Some((
            "RoundedView",
            format!(
                // `#0000` is Makepad's transparent: the frame is an outline, and
                // an outline that paints the center erases whatever is underneath it.
                "                    show_bg: true\n\
                 \x20                   draw_bg: {{ color: #0000, border_color: {}, border_size: {}, border_radius: 0.0 }}\n",
                cor(*color),
                numero(*thickness),
            ),
            String::new(),
        )),
        Stroke::Text {
            from,
            color,
            centered,
        } => texto(field, desc, from, *color, *centered, id),
        Stroke::Dashed { color } => {
            d.usa_shader = true;
            Some((
                "View",
                format!(
                    "                    show_bg: true\n\
                 \x20                   draw_bg: {{\n\
                 \x20                       color: {}\n\
                 \x20                       fn pixel(self) -> vec4 {{\n\
                 \x20                           let p = self.pos * self.rect_size;\n\
                 \x20                           let d = min(min(p.x, p.y), min(self.rect_size.x - p.x, self.rect_size.y - p.y));\n\
                 \x20                           if d <= 1.0 && mod(p.x + p.y, 7.0) < 4.0 {{\n\
                 \x20                               return self.color;\n\
                 \x20                           }}\n\
                 \x20                           return vec4(0.0, 0.0, 0.0, 0.0);\n\
                 \x20                       }}\n\
                 \x20                   }}\n",
                    cor(*color),
                ),
                String::new(),
            ))
        }
        Stroke::Check { color } => {
            d.usa_shader = true;
            Some((
                "View",
                format!(
                    // The mark scales with the SHORTER SIDE, staying a checkmark in a box
                    // of any shape instead of turning into a scribble in a wide one.
                    "                    show_bg: true\n\
                 \x20                   draw_bg: {{\n\
                 \x20                       color: {}\n\
                 \x20                       fn pixel(self) -> vec4 {{\n\
                 \x20                           let sdf = Sdf2d::viewport(self.pos * self.rect_size);\n\
                 \x20                           let s = min(self.rect_size.x, self.rect_size.y) * 0.3;\n\
                 \x20                           let c = self.rect_size * 0.5;\n\
                 \x20                           sdf.move_to(c.x - s, c.y);\n\
                 \x20                           sdf.line_to(c.x - s * 0.25, c.y + s * 0.7);\n\
                 \x20                           sdf.line_to(c.x + s, c.y - s * 0.7);\n\
                 \x20                           sdf.stroke(self.color, 2.0);\n\
                 \x20                           return sdf.result;\n\
                 \x20                       }}\n\
                 \x20                   }}\n",
                    cor(*color),
                ),
                String::new(),
            ))
        }
    }
}

#[note("targets-makepad-declarado-texto")]
fn texto(
    field: &str,
    desc: &ComponentDesc,
    de: &TextSource,
    color: Ink,
    centered: bool,
    id: &str,
) -> Option<(&'static str, String, String)> {
    let align = if centered {
        "                    align: {x: 0.5, y: 0.5}\n"
    } else {
        "                    align: {x: 0.0, y: 0.5}\n"
    };
    let (inicial, sync) = match de {
        TextSource::Fixed(t) if t.is_empty() => return None,
        TextSource::Fixed(t) => (escapar(t), String::new()),
        TextSource::Property(chave) => {
            let prop = desc.props.iter().find(|p| p.key == *chave)?;
            // The design-time value starts in the text; `sync_ui` tracks it afterwards.
            let leitura = match prop.kind {
                PropKind::String | PropKind::Multiline | PropKind::Enum(_) => {
                    format!("self.form.{field}.{chave}.clone()")
                }
                PropKind::Int { .. } | PropKind::Float { .. } | PropKind::Bool => {
                    format!("self.form.{field}.{chave}.to_string()")
                }
                // What this target cannot write is left out, instead of
                // turning into the `Debug` output of a value in the middle of the form.
                _ => return None,
            };
            (
                String::new(),
                format!(
                    "        {{\n\
                     \x20           let t = {leitura};\n\
                     \x20           let lbl = self.ui.label(id!({field}.{id}.{id}_text));\n\
                     \x20           if lbl.text() != t {{\n\
                     \x20               lbl.set_text(cx, &t);\n\
                     \x20           }}\n\
                     \x20       }}\n"
                ),
            )
        }
    };
    let dentro = format!(
        "{align}\
         \x20                   {id}_text = <Label> {{\n\
         \x20                       draw_text: {{ color: {} }}\n\
         \x20                       text: \"{inicial}\"\n\
         \x20                   }}\n",
        cor(color),
    );
    Some(("View", dentro, sync))
}

#[note("targets-makepad-declarado-limite")]
fn limite(field: &str, desc: &ComponentDesc, l: &Limit) -> Option<String> {
    match l {
        Limit::Fixed(n) => Some(numero(*n)),
        Limit::Property(k) => numerico(field, desc, k),
    }
}

#[note("targets-makepad-declarado-numerico")]
fn numerico(field: &str, desc: &ComponentDesc, chave: &str) -> Option<String> {
    let prop = desc.props.iter().find(|p| p.key == chave)?;
    match prop.kind {
        PropKind::Int { .. } | PropKind::Float { .. } => {
            Some(format!("self.form.{field}.{chave} as f32"))
        }
        _ => None,
    }
}

#[note("targets-makepad-declarado-cor")]
fn cor(t: Ink) -> String {
    hex(match t {
        Ink::Surface => tema::ROLE_SURFACE,
        Ink::Border => tema::ROLE_BORDER,
        Ink::Text => tema::ROLE_TEXT,
        Ink::TextDim => tema::ROLE_TEXT_DIM,
        Ink::Accent => tema::ROLE_ACCENT,
    })
}

#[note("targets-makepad-declarado-escapar")]
fn escapar(t: &str) -> String {
    t.replace('\\', "\\\\").replace('"', "\\\"")
}
