//! The designer canvas in Makepad — an immediate-mode custom drawing widget.
//!
//! It is the decision of ADR 0027:
//! the form is **not** assembled via `live_design!` text (there is no public API
//! to apply DSL at runtime), but drawn every frame with
//! `draw_abs` from [`crate::MakepadRuntime`], which in turn originates from
//! `crawrad_ir::Document`.
//!
//! Three rules govern this file, and all three come from outside it:
//!
//! 1. **It makes no decisions.** The gesture is TRANSLATED — `Hit::Finger*` becomes
//!    [`RuntimeEvent`] and enters the runtime queue. What resolves selection,
//!    dragging, snap, guides, and undo is `crawrad-designer`, which is headless and has
//!    155 tests. Here there is no component hit-test or resizing
//!    arithmetic.
//! 2. **Coordinate conversion lives in only one place** — the
//!    [`Transformacao`] from ADR 0019.
//!    Screen → design and design → screen go through it, and nothing computes coordinates
//!    outside it.
//! 3. **It knows nothing about drawing components.** What knows how to paint an `Edit` is
//!    `componentes/edit.rs`, and the canvas only asks the catalog who is responsible
//!    for the class ([`crate::components::desenho_de`]). This file takes care of
//!    what belongs to the CANVAS: the form paper, the grid, the designer adornments, and
//!    pointer translation.
//!
//! The DSL below declares **shaders**, and nothing else: one per primitive type,
//! instantiated N times by code (`draw_quad.rs:127`), which is the standard of the
//! framework itself (`scroll_shadow.rs:40-59`).

use std::collections::BTreeMap;

use makepad_widgets::*;

use crawrad_ir::{Font, NodeId, Rect as RectIr, Value};
use crawrad_runtime::{GuideKind, HANDLE_SIDE, RuntimeEvent};

use crate::components::comum::*;
use crate::components::{Scene, desenho_de, e_visual};
use crate::designer::{
    RULER_THICKNESS, calculate_grid_points, handle_at_point, handle_at_point_in_form, seta, ticks,
};
use crate::{MakepadCtl, MakepadRuntime, Transformacao};

pub mod atlas;
mod brush;

use atlas::PictureAtlas;
use brush::Primitives;
pub use brush::{
    Brush, Content, ContentLine, LAYERS, Layer, Mark, PROBE_CHAR_WIDTH, PROBE_FONT_POINTS,
    PROBE_LINE_HEIGHT, Probe, is_double_tap, lines_that_fit, point_in_design, rect_on_screen, rgba,
    shrink,
};

// The tab strip is `PageControl` geometry and lives in its file. The canvas
// only re-exports it because it is a contract with `crawrad-designer` — and contracts are read
// from one place only.
pub use crate::components::page_control::{
    ALTURA_DA_TIRA_DE_ABAS, LARGURA_DA_ABA, LARGURA_DO_MAIS, aba_i, plus_rectangle,
};
use crawrad_note::note;

live_design! {
    link crawrad;

    use link::theme::*;
    use makepad_draw::shader::std::*;

    // A box with fill, border, and radius — the primitive from which every
    // canvas control is made. ONE shader, N instances.
    pub DrawCaixa = {{DrawCaixa}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let e = self.espessura;
            let w = self.rect_size.x - e;
            let h = self.rect_size.y - e;
            // WHY TWO SHAPES, and not just `sdf.box`.
            //
            // `Sdf2d::box` is NOT a signed distance: inside it equals
            // `-2r` and nothing else (makepad-draw-1.0.0/src/shader/std.rs:394-401,
            // `bp = max(...,0)` zeroes inside the box). With `r = 0` this yields
            // `dist = 0` across the entire interior, and the two consequences were
            // MEASURED on screen:
            //
            //  - `fill_keep` paints nothing (alpha = clamp(0 * aa) = 0);
            //  - `stroke` paints the ENTIRE box (alpha = clamp(width * aa)).
            //
            // The form paper (which asked for radius 0) came out **#70748e** — the BORDER
            // color at 71% over the canvas background, without a drop of
            // `MAKEPAD_PAPER`. It was the form without paper, and part of "everything is
            // the same rectangle" that the owner rejected.
            //
            // `Sdf2d::rect` (`std.rs:478-485`) is truly signed. Therefore:
            // straight corners use `rect`, and only what actually asks for radius uses `box` —
            // with enough depth for the fill to be opaque
            // (`2r >= 3`, which is the minimum radius of 3 below).
            //
            // The `* 0.5` in `box` is the same doubling: the radius that arrives here is the
            // corner radius as everyone understands it, and `Sdf2d` uses `2r`.
            if self.raio < 3.0 {
                sdf.rect(e * 0.5, e * 0.5, w, h);
            }
            else {
                sdf.box(e * 0.5, e * 0.5, w, h, self.raio * 0.5);
            }
            sdf.fill_keep(self.fundo);
            sdf.stroke(self.borda, e);
            return sdf.result;
        }
    }

    pub DrawPicture = {{DrawPicture}} {
        // `uv` is the Rust field below: declaring it here too is a double
        // declaration the DSL gate catches.
        texture image: texture2d
        fn pixel(self) -> vec4 {
            return sample2d(self.image, self.uv.xy + self.pos * self.uv.zw);
        }
    }

    // The dashed line is a SHADER, not a loop — it is what the Makepad designer itself
    // does (`designer_view.rs:130-142`). It serves the selection marquee and the lasso.
    pub DrawTracejado = {{DrawTracejado}} {
        fn pixel(self) -> vec4 {
            let p = self.pos * self.rect_size;
            let sdf = Sdf2d::viewport(p);
            sdf.rect(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0);
            let padrao = fract((p.x + p.y) / 8.0);
            let a = step(padrao, 0.5);
            sdf.stroke(mix(#0000, self.cor, a), 1.0);
            return sdf.result;
        }
    }

    pub DrawAlca = {{DrawAlca}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            sdf.rect(0.5, 0.5, self.rect_size.x - 1.0, self.rect_size.y - 1.0);
            sdf.fill_keep(self.cor);
            sdf.stroke(#000c, 1.0);
            return sdf.result;
        }
    }

    // The ComboBox arrow and the two of each ScrollBar.
    //
    // `direcao` is a unit vector — where the TIP points —, and not an
    // angle: rotating by angle would force this shader to remember that Makepad's
    // Y axis grows downward, and the triangle would come out mirrored if
    // someone changed the sign. With the vector, the base comes from the perpendicular and
    // there is no direction to remember.
    pub DrawSeta = {{DrawSeta}} {
        direcao: vec2(0.0, 1.0),
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let meio = self.rect_size * 0.5;
            let d = normalize(self.direcao);
            let n = vec2(-d.y, d.x);
            let s = min(self.rect_size.x, self.rect_size.y) * 0.5;
            let ponta = meio + d * s * 0.62;
            let a = meio - d * s * 0.38 + n * s * 0.62;
            let b = meio - d * s * 0.38 - n * s * 0.62;
            sdf.move_to(a.x, a.y);
            sdf.line_to(b.x, b.y);
            sdf.line_to(ponta.x, ponta.y);
            sdf.close_path();
            sdf.fill(self.cor);
            return sdf.result;
        }
    }

    // The checkmark of the checked CheckBox. It is a SHADER, not a glyph: the canvas font is
    // THEME_FONT_REGULAR from makepad-widgets, and a glyph it lacks appears
    // as an empty box on screen.
    pub DrawVisto = {{DrawVisto}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let w = self.rect_size.x;
            let h = self.rect_size.y;
            sdf.move_to(w * 0.22, h * 0.52);
            sdf.line_to(w * 0.42, h * 0.74);
            sdf.line_to(w * 0.78, h * 0.28);
            sdf.stroke(self.cor, max(1.0, w * 0.14));
            return sdf.result;
        }
    }

    // WHY THERE ARE THREE FILLED RECTANGLE SHADERS, and not just one `DrawColor`.
    //
    // **In Makepad, the painting order is by SHADER, not call order.**
    // `find_appendable_drawcall` scans the list backwards and APPENDS to
    // the first draw call of the same shader it finds
    // (makepad-platform-1.0.0/src/draw_list.rs:334-343), and `zbias` — which is the
    // real Z — is assigned per CALL at render time
    // (`os/windows/d3d11.rs`, the same loop as in `os/apple/metal.rs:109-111`).
    //
    // Measured here: with grid and background on the SAME `DrawColor`, the grid
    // dots appeared ON TOP of every component — `#7a7f9a` inside
    // `Button` and inside `Edit`, counted pixel by pixel in the screenshot. It was a good
    // part of the "dirty look".
    //
    // The fix is to give a type to each LAYER, in the order the first
    // instance of each is emitted:
    //
    //   DrawColor (background and paper)  ->  DrawDot (grid)  ->  DrawCaixa
    //   (components)  ->  DrawRisca/DrawSeta/DrawVisto  ->  DrawText  ->
    //   DrawTracejado/DrawAlca (adornments)
    //
    // Swapping the order of two calls in `draw_walk` no longer changes anything; this
    // list rules. Anyone adding a layer needs to know this.
    pub DrawDot = {{DrawDot}} {
        fn pixel(self) -> vec4 {
            return self.cor;
        }
    }

    pub DrawRisca = {{DrawRisca}} {
        fn pixel(self) -> vec4 {
            return self.cor;
        }
    }

    // THE NON-VISUAL BADGE HAS ITS OWN LAYER, and it is the owner's rule:
    // *"drawings of non-visual components in designer mode must always
    // stay in front of all visual components"*.
    //
    // In a classic RAD IDE the non-visual is a designer OVERLAY: it does not take part in the form's
    // Z order and disappears in the application. An `ActionList` hidden behind
    // a `Panel` is a component the user cannot click to
    // select — a gesture defect, not an aesthetic one.
    //
    // Two layers, and BOTH are necessary: if only the box was raised, the badge
    // would be in front and the initials behind it, because canvas text comes
    // from regular `DrawText`, which is a lower layer. `DrawTextoSelo`
    // derives from `DrawText` just as Makepad's own `DrawNodeText` does
    // (makepad-widgets-1.0.0/src/designer_outline_tree.rs:281-286), and being
    // another live class it gets another draw call.
    pub DrawSelo = {{DrawSelo}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let e = self.espessura;
            let w = self.rect_size.x - e;
            let h = self.rect_size.y - e;
            if self.raio < 3.0 {
                sdf.rect(e * 0.5, e * 0.5, w, h);
            }
            else {
                sdf.box(e * 0.5, e * 0.5, w, h, self.raio * 0.5);
            }
            sdf.fill_keep(self.fundo);
            sdf.stroke(self.borda, e);
            return sdf.result;
        }
    }

    pub DrawTextoSelo = {{DrawTextoSelo}} {}

    // THE CONTAINER CHROME — the bar that a `ScrollBox` puts over its
    // children, and now also over the MARQUEE.
    //
    // Why they are DEDICATED shaders, and not the same `DrawCaixa`/`DrawSeta`:
    // in Makepad, the painting order is by LAYER, and emission order only
    // breaks ties WITHIN a draw call. An earlier fix addressed the inside —
    // the face of `button3` pulled back to the bar —, but the marquee is
    // `DrawTracejado`, a layer ABOVE `DrawCaixa`, and therefore it
    // crossed the entire bar. The owner saw it and stated what was wrong
    // with it: it is the SAME rectangle drawn under two rules, half clipped and
    // half not.
    //
    // With its own layer between marquee and handle, the rule is written in the
    // stack and not in emission chance:
    //
    //   ... content ... -> Dashed -> Chrome -> ChromeArrow -> Handle -> Ruler
    //
    // The two halves have DIFFERENT reasons, and that is why the handle does not come
    // along: the marquee is decoration, no one grabs it; the handle IS what is
    // grabbed, and hit-testing does not know a bar exists. A handle hidden under the
    // bar and still draggable would be the interface lying — the exact opposite of
    // the earlier fix.
    pub DrawChrome = {{DrawChrome}} {
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let e = self.espessura;
            let w = self.rect_size.x - e;
            let h = self.rect_size.y - e;
            if self.raio < 3.0 {
                sdf.rect(e * 0.5, e * 0.5, w, h);
            }
            else {
                sdf.box(e * 0.5, e * 0.5, w, h, self.raio * 0.5);
            }
            sdf.fill_keep(self.fundo);
            sdf.stroke(self.borda, e);
            return sdf.result;
        }
    }

    pub DrawChromeArrow = {{DrawChromeArrow}} {
        direcao: vec2(0.0, 1.0),
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            let meio = self.rect_size * 0.5;
            let d = normalize(self.direcao);
            let n = vec2(-d.y, d.x);
            let s = min(self.rect_size.x, self.rect_size.y) * 0.5;
            let ponta = meio + d * s * 0.62;
            let a = meio - d * s * 0.38 + n * s * 0.62;
            let b = meio - d * s * 0.38 - n * s * 0.62;
            sdf.move_to(a.x, a.y);
            sdf.line_to(b.x, b.y);
            sdf.line_to(ponta.x, ponta.y);
            sdf.close_path();
            sdf.fill(self.cor);
            return sdf.result;
        }
    }

    // THE RULER is the LAST canvas layer, above even the adornments. It is what a
    // ruler must be: it MEASURES what is beneath it, so nothing can
    // pass over the number. Two types because strip and number are different
    // draw calls — see the layer note above.
    pub DrawRegua = {{DrawRegua}} {
        fn pixel(self) -> vec4 {
            return self.cor;
        }
    }

    pub DrawTextoRegua = {{DrawTextoRegua}} {}

    pub DesignCanvas = {{DesignCanvas}} {
        width: Fill, height: Fill,
        // With `Overlay` the turtle does not advance when a child uses `abs_pos`
        // (`turtle.rs:544-547`) — and here EVERYTHING is absolute.
        flow: Overlay,
        // The app's captions are the theme's paragraph size: the canvas
        // draws them the same, or a Label fits here and is clipped when it runs.
        draw_texto: {
            text_style: <THEME_FONT_REGULAR> { font_size: (THEME_FONT_SIZE_P) }
        }
        draw_texto_selo: {
            text_style: <THEME_FONT_REGULAR> { font_size: 8.0 }
        }
        // 7 POINTS, not 9 pixels: Makepad's `font_size` is in POINTS
        // (makepad-draw-1.0.0/src/text/layouter.rs:586-587 does `pts * 96/72`),
        // so this comes out as 9.3 px — the size of the 9 monospace that the RXP
        // uses on its ruler.
        draw_texto_regua: {
            text_style: <THEME_FONT_REGULAR> { font_size: 7.0 }
        }
    }
}

// ---------------------------------------------------------------------------
// The shaders, in Rust
// ---------------------------------------------------------------------------

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawCaixa {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub fundo: Vec4,
    #[live]
    pub borda: Vec4,
    #[live]
    pub raio: f32,
    #[live(1.0)]
    pub espessura: f32,
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawPicture {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub uv: Vec4,
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawTracejado {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub cor: Vec4,
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawAlca {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub cor: Vec4,
}

#[note("canvas-drawregua")]
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawRegua {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub cor: Vec4,
}

#[note("canvas-drawtextoregua")]
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawTextoRegua {
    #[deref]
    pub draw_super: DrawText,
}

#[note("canvas-drawselo")]
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawSelo {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub fundo: Vec4,
    #[live]
    pub borda: Vec4,
    #[live]
    pub raio: f32,
    #[live(1.0)]
    pub espessura: f32,
}

#[note("canvas-drawchrome")]
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawChrome {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub fundo: Vec4,
    #[live]
    pub borda: Vec4,
    #[live]
    pub raio: f32,
    #[live(1.0)]
    pub espessura: f32,
}

#[note("canvas-drawchromearrow")]
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawChromeArrow {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub cor: Vec4,
    #[live]
    pub direcao: Vec2,
}

#[note("canvas-drawtextoselo")]
#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawTextoSelo {
    #[deref]
    pub draw_super: DrawText,
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawDot {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub cor: Vec4,
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawRisca {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub cor: Vec4,
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawVisto {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub cor: Vec4,
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawSeta {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub cor: Vec4,
    /// Where the tip points. The starting value comes from the DSL (`vec2(0, 1)`,
    /// downward, which is the `ComboBox` arrow); whoever draws always
    /// writes it first, in [`Brush::arrow`].
    #[live]
    pub direcao: Vec2,
}

// ---------------------------------------------------------------------------
// Content in lines — where it comes from
// ---------------------------------------------------------------------------

#[note("canvas-control-lines")]
pub fn control_lines(c: &MakepadCtl) -> Vec<ContentLine> {
    desenho_de(&c.class).lines(c)
}

// ---------------------------------------------------------------------------
// Container clipping
// ---------------------------------------------------------------------------

#[note("canvas-clipped")]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Clipped<T> {
    /// What is painted — **whole**, exactly as the document sized it.
    pub item: T,
    /// The rectangle beyond which nothing of `item` is painted. `None` means
    /// nothing clips it.
    pub clip: Option<RectIr>,
}

impl<T> Clipped<T> {
    #[note("canvas-clipped-free")]
    pub fn free(item: T) -> Self {
        Self { item, clip: None }
    }
}

impl Clipped<RectIr> {
    #[note("canvas-clipped-of-control")]
    pub fn of_control(c: &MakepadCtl, form: RectIr) -> Self {
        Self {
            item: c.rect,
            clip: Some(clip_of(c, form)),
        }
    }
}

#[note("canvas-clip-of")]
pub fn clip_of(c: &MakepadCtl, form: RectIr) -> RectIr {
    c.clip.unwrap_or(form)
}

#[note("canvas-content-clip-of")]
pub fn content_clip_of(c: &MakepadCtl) -> Option<RectIr> {
    desenho_de(&c.class).clips_to_own_box().then_some(c.rect)
}

// ---------------------------------------------------------------------------
// Geometry belonging to the CANVAS, and not to any component
// ---------------------------------------------------------------------------

#[note("canvas-draw-order")]
pub fn draw_order<'a>(
    ordem: &'a [NodeId],
    ctls: &'a std::collections::BTreeMap<NodeId, MakepadCtl>,
    modo: crawrad_runtime::Mode,
) -> Vec<&'a NodeId> {
    let visiveis = || {
        ordem
            .iter()
            .filter(|id| ctls.get(*id).is_some_and(|c| c.visible))
    };
    let selos: Vec<&NodeId> = if modo.is_live() {
        Vec::new()
    } else {
        visiveis()
            .filter(|id| ctls.get(*id).is_some_and(|c| !e_visual(&c.class)))
            .collect()
    };
    visiveis()
        .filter(|id| ctls.get(*id).is_some_and(|c| e_visual(&c.class)))
        .chain(selos)
        .collect()
}

#[note("canvas-step")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step<'a> {
    /// Paint the control itself. Its children, if any, come next.
    Paint(&'a NodeId),
    /// Paint what the container puts **above** its own children — the step
    /// that closes right after the last of them
    /// ([`crate::components::ComponentDrawing::draw_over`]).
    Overlay(&'a NodeId),
}

#[note("canvas-draw-program")]
pub fn draw_program<'a>(
    ordem: &[&'a NodeId],
    ctls: &std::collections::BTreeMap<NodeId, MakepadCtl>,
) -> Vec<Step<'a>> {
    let mut steps = Vec::with_capacity(ordem.len());
    let mut open: Vec<&'a NodeId> = Vec::new();
    for id in ordem {
        while open
            .last()
            .is_some_and(|top| !is_inside(ctls, id, top) && id != top)
        {
            if let Some(top) = open.pop() {
                steps.push(Step::Overlay(top));
            }
        }
        steps.push(Step::Paint(id));
        if ctls
            .get(*id)
            .is_some_and(|c| desenho_de(&c.class).draws_over_children())
        {
            open.push(id);
        }
    }
    while let Some(top) = open.pop() {
        steps.push(Step::Overlay(top));
    }
    steps
}

#[note("canvas-is-inside")]
fn is_inside(
    ctls: &std::collections::BTreeMap<NodeId, MakepadCtl>,
    node: &NodeId,
    ancestor: &NodeId,
) -> bool {
    let mut atual = node;
    for _ in 0..64 {
        let Some(c) = ctls.get(atual) else {
            return false;
        };
        if c.parent == *ancestor {
            return true;
        }
        if c.parent == *atual {
            return false;
        }
        atual = &c.parent;
    }
    false
}

#[note("canvas-badge-rect")]
pub fn badge_rect(r: Rect, scale: f64) -> Rect {
    // A square, so the half-height corner closes a circle: 18x13 drew an oval (owner, 11/11).
    const SIDE: f64 = 15.0;
    let scale = scale.max(0.01);
    let (w, h) = (SIDE * scale, SIDE * scale);
    Rect {
        pos: dvec2(r.pos.x, r.pos.y - h),
        size: dvec2(w, h),
    }
}

/// A form-check pin: a disc on the control's top-right corner.
pub fn pin_rect(r: Rect, scale: f64) -> Rect {
    let side = 16.0 * scale.max(0.01);
    Rect {
        pos: dvec2(r.pos.x + r.size.x - side / 2.0, r.pos.y - side / 2.0),
        size: dvec2(side, side),
    }
}

#[note("canvas-inherited-badge-rect")]
pub(crate) fn inherited_badge_rect(r: Rect, scale: f64) -> Rect {
    const SIDE: f64 = 14.0;
    let side = (SIDE * scale.max(0.01)).min(r.size.x).min(r.size.y);
    Rect {
        pos: dvec2(r.pos.x + r.size.x - side, r.pos.y),
        size: dvec2(side, side),
    }
}

#[note("canvas-inherited-badge")]
fn inherited_badge(p: &mut Brush, r: Rect, scale: f64) {
    let b = inherited_badge_rect(r, scale);
    p.rounded_box(b, MAKEPAD_INHERITED, MAKEPAD_INHERITED, 2.0);
    // The hooked arrow: down, then right into the head.
    let u = b.size.x / 14.0;
    let at = |x: f64, y: f64, w: f64, h: f64| Rect {
        pos: dvec2(b.pos.x + x * u, b.pos.y + y * u),
        size: dvec2(w * u, h * u),
    };
    p.solid(at(4.0, 3.0, 1.6, 6.5), MAKEPAD_BG);
    p.solid(at(4.0, 7.9, 4.5, 1.6), MAKEPAD_BG);
    p.arrow(at(8.0, 5.7, 3.2, 6.0), dvec2(1.0, 0.0), MAKEPAD_BG);
}

#[note("canvas-touch")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Touch {
    Down {
        ctrl: bool,
        double: bool,
    },
    /// `free`: `Alt` held, the classic IDE's place off the grid.
    Move {
        free: bool,
    },
    Up,
    Cancel,
}

#[note("canvas-translate")]
pub fn translate(toque: Touch, t: &Transformacao, abs: DVec2) -> Vec<RuntimeEvent> {
    let (x, y) = point_in_design(t, abs);
    match toque {
        Touch::Down { ctrl, double } => {
            let mut v = vec![RuntimeEvent::PointerDown { x, y, ctrl }];
            if double {
                v.push(RuntimeEvent::PointerDblClick { x, y });
            }
            v
        }
        Touch::Move { free } => vec![RuntimeEvent::PointerMove { x, y, free }],
        Touch::Up => vec![RuntimeEvent::PointerUp { x, y }],
        Touch::Cancel => vec![RuntimeEvent::Cancel],
    }
}

#[note("canvas-cursor-for")]
pub fn cursor_for(selecao: Option<RectIr>, formulario: RectIr, x: f32, y: f32) -> MouseCursor {
    if let Some(r) = selecao
        && let Some(e) = handle_at_point(r, x, y)
    {
        return seta(e);
    }
    if let Some(e) = handle_at_point_in_form(formulario, x, y) {
        return seta(e);
    }
    MouseCursor::Default
}

// ---------------------------------------------------------------------------
// Designer adornments — read from the model BEFORE painting
// ---------------------------------------------------------------------------

#[note("canvas-adornments")]
#[derive(Default)]
struct Adornments {
    marquee: Option<RectIr>,
    guides: Vec<crawrad_runtime::GuideLine>,
    /// The Tab-order badges; `true` is a tied TabOrder.
    badges: Vec<Clipped<(RectIr, String, bool)>>,
    /// The form checker's pins: the control, its number, an error.
    pins: Vec<(RectIr, usize, bool)>,
    /// The components that come from the base form.
    inherited: Vec<Clipped<RectIr>>,
    selection: Vec<Clipped<RectIr>>,
    handles: Clipped<Vec<(f32, f32)>>,
    /// The landing and the pointer's place of a snapped move.
    ghost: Option<(RectIr, RectIr)>,
    /// The controls that overflow their frame in the size preview.
    overflow: Vec<RectIr>,
    /// The red bars on the frame's edge where a control is cut off.
    overflow_marks: Vec<RectIr>,
}

/// Width of the red bar on a frame's edge where a control is cut off.
const OVERFLOW_MARK: f32 = 4.0;

/// The bars on `form`'s right and bottom edges where `ctl` runs past them: along the
/// part of the edge the control spans, so a control wholly outside still shows where it is.
pub fn overflow_marks(form: RectIr, ctl: RectIr) -> Vec<RectIr> {
    let (right, bottom) = (form.x + form.w, form.y + form.h);
    let mut marks = Vec::new();
    let (top, low) = (ctl.y.max(form.y), (ctl.y + ctl.h).min(bottom));
    if ctl.x + ctl.w > right && low > top {
        marks.push(RectIr::new(
            right - OVERFLOW_MARK,
            top,
            OVERFLOW_MARK,
            low - top,
        ));
    }
    let (left, end) = (ctl.x.max(form.x), (ctl.x + ctl.w).min(right));
    if ctl.y + ctl.h > bottom && end > left {
        marks.push(RectIr::new(
            left,
            bottom - OVERFLOW_MARK,
            end - left,
            OVERFLOW_MARK,
        ));
    }
    marks
}

// ---------------------------------------------------------------------------
// The widget
// ---------------------------------------------------------------------------

#[note("canvas-designcanvas")]
#[derive(Live, Widget)]
pub struct DesignCanvas {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[redraw]
    #[rust]
    area: Area,

    #[live]
    draw_fundo: DrawColor,
    #[live]
    draw_ponto: DrawDot,
    #[live]
    draw_risca: DrawRisca,
    #[live]
    draw_caixa: DrawCaixa,
    #[live]
    draw_picture: DrawPicture,
    #[live]
    draw_seta: DrawSeta,
    #[live]
    draw_visto: DrawVisto,
    #[live]
    draw_tracejado: DrawTracejado,
    #[live]
    draw_alca: DrawAlca,
    #[live]
    draw_chrome: DrawChrome,
    #[live]
    draw_chrome_arrow: DrawChromeArrow,
    #[live]
    draw_texto: DrawText,
    #[live]
    draw_selo: DrawSelo,
    #[live]
    draw_texto_selo: DrawTextoSelo,
    #[live]
    draw_regua: DrawRegua,
    #[live]
    draw_texto_regua: DrawTextoRegua,

    /// The model. Everything that is drawn comes from here, and nothing else enters.
    #[rust]
    rt: MakepadRuntime,
    #[rust]
    atlas: PictureAtlas,
    /// Where the form's (0,0) corner landed on screen, in the last frame.
    /// In PASS coordinates — `abs_pos` is absolute in the pass, not relative to
    /// the parent (`turtle.rs:279-281`).
    #[rust]
    origem: DVec2,
    /// **The canvas's OWN draw list.**
    ///
    /// Without it, the layer order documented in the DSL above **does not hold**, and the
    /// defect was seen on the owner's screen: in the example the buttons came out with
    /// captions, and inside the IDE they came out **empty, with the accelerator underline
    /// alone in the middle**. Also missing were the `ListBox` item, the `Memo` line,
    /// the `TreeView` node, the `ComboBox` text, and the badge initials — everything
    /// that is TEXT with something drawn over it.
    ///
    /// The mechanism is `find_appendable_drawcall`
    /// (`makepad-platform-1.0.0/src/draw_list.rs:334-343`): it searches for a
    /// draw call of the same shader **in the current draw list**. Without
    /// its own list, the current list is the HOST's — and the canvas `DrawText`
    /// was appended to a call the shell had already opened earlier,
    /// long before any box of ours. Layer order came to
    /// depend on what the shell drew before the canvas, which is why the example
    /// (thin bar and nothing else) got it right and the IDE (Toolbox, Structure, ruler,
    /// AI panel) got it wrong.
    ///
    /// With its own list, `get_draw_call` only sees items FROM HERE
    /// (`makepad-draw-1.0.0/src/draw_list_2d.rs:240-262`): the order becomes an
    /// invariant of the canvas, and no host can invert it. It is the same
    /// mechanism that `View` from `makepad-widgets` uses when `optimize` requests it
    /// (`view.rs:232-233` and `:817`).
    /// Where the pointer last was, in screen points.
    ///
    /// This is what turns the ruler from ornament into instrument: aligning a
    /// component *"at 120"* requires seeing the 120 move together with the hand. The RXP reads
    /// the same from `ctx.pointer_latest_pos()` (`crawrad-runtime-egui/src/lib.rs:2040`);
    /// here `Hit::FingerHoverOver` already provides the point, and storing it is enough.
    #[rust]
    ponteiro: Option<DVec2>,
    #[rust]
    lista: Option<DrawList2d>,
    /// Canvas offset (the "pan"). Subtracted from the origin when drawing and
    /// added back on input — which is why it does not leak outside
    /// [`Transformacao`].
    #[rust]
    deslocamento: DVec2,
    /// Whether the canvas is read-only (size preview).
    #[rust]
    read_only: bool,
    /// Overflowing control ids (size preview).
    #[rust]
    overflow: Vec<NodeId>,
    /// How the form's paper looks: see [`PageLook`].
    #[rust]
    page_look: PageLook,
}

/// How the designer paints the form's paper.
///
/// `light` paints the form with the light theme the generated app links;
/// `web` is a browser page: white, no dot grid and no window frame, since the
/// browser has neither. Absolute positions stay as they are.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PageLook {
    pub light: bool,
    pub web: bool,
}

impl LiveHook for DesignCanvas {
    #[note("canvas-designcanvas-after-apply")]
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if self.lista.is_none() {
            self.lista = Some(DrawList2d::new(cx));
        }
    }
}

impl DesignCanvas {
    #[note("canvas-designcanvas-runtime")]
    pub fn runtime(&self) -> &MakepadRuntime {
        &self.rt
    }

    #[note("canvas-designcanvas-runtime-mut")]
    pub fn runtime_mut(&mut self) -> &mut MakepadRuntime {
        &mut self.rt
    }

    #[note("canvas-designcanvas-transformacao")]
    pub fn transformacao(&self) -> Transformacao {
        Transformacao::new(self.origem.x as f32, self.origem.y as f32, self.rt.escala())
    }

    #[note("canvas-designcanvas-inherited-badge-at")]
    pub fn inherited_badge_at(&self, at: DVec2) -> Option<NodeId> {
        let t = self.transformacao();
        let scale = f64::from(t.medida(1.0));
        let rt = &self.rt;
        rt.inherited.iter().rev().find_map(|id| {
            let c = rt.ctls.get(id).filter(|_| rt.desenhavel(id))?;
            let r = inherited_badge_rect(rect_on_screen(&t, c.rect), scale);
            r.contains(at).then(|| id.clone())
        })
    }

    /// Hover hit-test for form check pins. Last drawn wins.
    pub fn pin_at(&self, at: DVec2) -> Option<usize> {
        let t = self.transformacao();
        let scale = f64::from(t.medida(1.0));
        control_pins(&self.rt)
            .into_iter()
            .rev()
            .find_map(|(rect, number, _)| {
                let r = pin_rect(rect_on_screen(&t, rect), scale);
                r.contains(at).then_some(number)
            })
    }

    #[note("canvas-designcanvas-cursor-em")]
    pub fn cursor_em(&self, x: f32, y: f32) -> MouseCursor {
        // `rect_da_alca` is the SAME reading that `draw_adornments` uses. When
        // there were two, the resize arrow appeared over an off-screen node
        // that `Designer::hit_handle` refused to drag.
        cursor_for(self.rt.rect_da_alca(), self.rt.form_rect, x, y)
    }

    fn enfileirar(&mut self, toque: Touch, t: &Transformacao, abs: DVec2) {
        for e in translate(toque, t, abs) {
            self.rt.injetar(e);
        }
    }

    #[note("canvas-designcanvas-pincel")]
    fn pincel<'a, 'b, 'c>(
        &'a mut self,
        cx: &'a mut Cx2d<'b, 'c>,
        t: Transformacao,
    ) -> Brush<'a, 'b, 'c> {
        Brush::new(
            cx,
            Primitives {
                rounded_box: &mut self.draw_caixa,
                picture: &mut self.draw_picture,
                text: &mut self.draw_texto,
                badge: &mut self.draw_selo,
                badge_text: &mut self.draw_texto_selo,
                ruler: &mut self.draw_regua,
                ruler_text: &mut self.draw_texto_regua,
                arrow: &mut self.draw_seta,
                check: &mut self.draw_visto,
                dashed: &mut self.draw_tracejado,
                handle: &mut self.draw_alca,
                chrome_box: &mut self.draw_chrome,
                chrome_arrow: &mut self.draw_chrome_arrow,
                background: &mut self.draw_fundo,
                grid_dot: &mut self.draw_ponto,
                stroke: &mut self.draw_risca,
                atlas: &mut self.atlas,
            },
            t,
        )
    }

    /// Whether the canvas is read-only (size preview).
    pub fn read_only(&self) -> bool {
        self.read_only
    }

    /// Sets whether the canvas is read-only.
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    /// Sets how the form's paper looks; `true` when it changed.
    pub fn set_page_look(&mut self, look: PageLook) -> bool {
        let changed = self.page_look != look;
        self.page_look = look;
        changed
    }

    /// Returns the overflowing control ids.
    pub fn overflow(&self) -> &[NodeId] {
        &self.overflow
    }

    /// Sets which controls are overflowing and need a red outline.
    pub fn set_overflow(&mut self, overflow: &[NodeId]) {
        self.overflow = overflow.to_vec();
    }
}

/// The per-control pin list drawn by adornments and hit-tested by pin_at.
fn control_pins(rt: &MakepadRuntime) -> Vec<(RectIr, usize, bool)> {
    let mut pins = Vec::new();
    for pin in &rt.pins {
        let error = pin.kind == crawrad_runtime::PinKind::Error;
        if pin.kind == crawrad_runtime::PinKind::TabClash || !rt.desenhavel(&pin.node) {
            continue;
        }
        let Some(c) = rt.ctls.get(&pin.node) else {
            continue;
        };
        match pins.iter_mut().find(|(r, _, _)| *r == c.rect) {
            Some(seen) => seen.2 |= error,
            None => pins.push((c.rect, pin.number, error)),
        }
    }
    pins
}

#[note("canvas-adornments-2")]
fn adornments(rt: &MakepadRuntime) -> Adornments {
    let mut a = Adornments {
        marquee: rt.marquee,
        guides: rt.guias.clone(),
        ghost: rt.ghost,
        pins: control_pins(rt),
        ..Adornments::default()
    };

    let clash = |id: &NodeId| {
        rt.pins
            .iter()
            .any(|p| p.node == *id && p.kind == crawrad_runtime::PinKind::TabClash)
    };
    for (id, texto) in &rt.badges {
        if !rt.desenhavel(id) {
            continue;
        }
        if let Some(ctl) = rt.ctls.get(id) {
            // The tab order badge is clipped by the CONTAINER, not
            // by the form: it originates ABOVE the top border of the component
            // ([`badge_rect`]), so clipping it by the form's
            // wall would erase the badge for the entire top row. The RXP does not
            // have this problem because its badge sits INSIDE the rectangle
            // (`crawrad-runtime-egui/src/lib.rs:1926`); the container rule, which
            // is what the RXP writes (`:1921`), is the same in both.
            a.badges.push(Clipped {
                item: (ctl.rect, texto.clone(), clash(id)),
                clip: ctl.clip,
            });
        }
    }

    for id in &rt.inherited {
        if !rt.desenhavel(id) {
            continue;
        }
        if let Some(c) = rt.ctls.get(id) {
            a.inherited.push(Clipped::of_control(c, rt.form_rect));
        }
    }

    for id in &rt.sel {
        if !rt.desenhavel(id) {
            continue;
        }
        // The form is the surface: what would clip it is itself, and its
        // outline lives right on the border.
        let alvo = if *id == rt.form_id {
            Some(Clipped::free(rt.form_rect))
        } else {
            rt.ctls
                .get(id)
                .map(|c| Clipped::of_control(c, rt.form_rect))
        };
        if let Some(r) = alvo {
            a.selection.push(r);
        }
    }

    // What gets handles is the runtime's `rect_da_alca`, and nothing else.
    //
    // **And the handle is clipped along with the component.** Without this, the defect
    // that already died twice here returns by another path: eight
    // little blue squares over a piece of component that the container does not
    // allow to show, and `Designer::hit_handle` refusing to drag. Where there is
    // no component on screen, there is nothing to grab — the RXP says the same in
    // `crawrad-runtime-egui/src/lib.rs:1820-1830`.
    let primeiro = rt.sel.first().map(|s| s.as_str());
    a.handles = if primeiro == Some(rt.form_id.as_str()) {
        // The THREE of the form: east, south, and the corner, as in a classic RAD IDE.
        Clipped::free(crate::designer::alcas_do_formulario(rt.form_rect).to_vec())
    } else if let Some(r) = rt.rect_da_alca() {
        Clipped {
            item: crawrad_runtime::handles(r.x, r.y, r.w, r.h).to_vec(),
            clip: rt
                .sel
                .first()
                .and_then(|id| rt.ctls.get(id))
                .map(|c| clip_of(c, rt.form_rect)),
        }
    } else {
        Clipped::default()
    };

    a
}

#[note("canvas-draw-adornments")]
fn draw_adornments(p: &mut Brush, a: &Adornments) {
    // The selection marquee — the trait's `set_marquee`.
    if let Some(m) = a.marquee {
        let r = p.na_tela(m);
        p.dashed(r, MAKEPAD_ACCENT);
    }

    // The alignment guides — the trait's `set_guides`.
    for g in &a.guides {
        let c = match g.kind {
            GuideKind::Component => MAKEPAD_GUIA_COMPONENTE,
            GuideKind::Text => MAKEPAD_GUIA_TEXTO,
            GuideKind::Spacing => MAKEPAD_GUIA_ESPACO,
        };
        let esp = if g.thickness > 0.0 { g.thickness } else { 1.0 };
        let r = if g.vertical {
            RectIr::new(g.pos - esp * 0.5, g.from, esp, g.to - g.from)
        } else {
            RectIr::new(g.from, g.pos - esp * 0.5, g.to - g.from, esp)
        };
        let r = p.na_tela(r);
        p.solid(r, c);
    }

    // The tab order badges — the trait's `set_badges`.
    for s in &a.badges {
        let (rect, texto, clash) = &s.item;
        let selo = badge_rect(p.na_tela(*rect), p.medida(1.0));
        let clip = s.clip.map(|c| p.na_tela(c));
        let [fill, ink] = if *clash {
            MAKEPAD_TAB_CLASH
        } else {
            MAKEPAD_TAB_BADGE
        };
        paint_clipped(p, clip, |p| {
            p.rounded_box(selo, fill, fill, selo.size.y / 2.0);
            p.write(selo, texto, ink, true);
        });
    }

    // The form checker's pins, over everything but the selection.
    for (rect, number, error) in &a.pins {
        let pin = pin_rect(p.na_tela(*rect), p.medida(1.0));
        let [fill, ink] = if *error {
            MAKEPAD_PIN_ERROR
        } else {
            MAKEPAD_PIN_WARNING
        };
        p.rounded_box(pin, fill, fill, pin.size.x / 2.0);
        p.write(pin, &number.to_string(), ink, true);
    }

    // The corner badge of each inherited component — `set_inherited`.
    for s in &a.inherited {
        let r = p.na_tela(s.item);
        let clip = s.clip.map(|c| p.na_tela(c));
        let scale = p.medida(1.0);
        paint_clipped(p, clip, |p| inherited_badge(p, r, scale));
    }

    // The selection — the trait's `set_selection`.
    for s in &a.selection {
        let r = p.na_tela(s.item);
        let clip = s.clip.map(|c| p.na_tela(c));
        paint_clipped(p, clip, |p| p.dashed(r, MAKEPAD_ACCENT));
    }

    // Snap made visible: the control sits where it lands, outlined in
    // the accent over the selection; a fainter outline follows the pointer.
    if let Some((landing, pointer)) = a.ghost {
        let (landing, pointer) = (p.na_tela(landing), p.na_tela(pointer));
        p.dashed(pointer, MAKEPAD_SNAP_POINTER);
        p.dashed(landing, MAKEPAD_SNAP);
    }

    // The handles, clipped by the SAME rectangle that clips the component.
    if !a.handles.item.is_empty() {
        let lado = p.medida(HANDLE_SIDE).max(5.0);
        let clip = a.handles.clip.map(|c| p.na_tela(c));
        paint_clipped(p, clip, |p| p.handles(&a.handles.item, lado));
    }

    // The overflow outlines for size preview.
    for rect in &a.overflow {
        let r = p.na_tela(*rect);
        p.frame(r, MAKEPAD_PIN_ERROR[0], 2.0);
    }
    for rect in &a.overflow_marks {
        let r = p.na_tela(*rect);
        p.background(r, MAKEPAD_PIN_ERROR[0]);
    }
}

/// Resolves the effective font size and colour for a control.
///
/// Returns `(size, color)`:
/// - `size` is `None` when 0 (theme size is kept)
/// - `color` is `None` when alpha is 0 (theme colour is kept)
/// - `parent_font` true or absent with no own font uses the form's font
/// - A control without a `font` prop returns `(None, None)`
pub fn effective_font(
    props: &BTreeMap<String, Value>,
    form: Option<&Font>,
) -> (Option<f32>, Option<[f32; 4]>) {
    let font_prop = props.get("font");
    let parent_font_prop = props.get("parent_font");

    if font_prop.is_none() && parent_font_prop.is_none() {
        return (None, None);
    }

    let own_font = font_prop.and_then(Value::as_font);
    let parent_font = parent_font_prop.and_then(Value::as_bool);

    let use_form = match parent_font {
        Some(true) => true,
        Some(false) => false,
        None => own_font.is_none(),
    };

    let font = if use_form { form } else { own_font };

    let Some(font) = font else {
        return (None, None);
    };

    let size = if font.size > 0.0 {
        Some(font.size)
    } else {
        None
    };

    let color = if font.color.a > 0 {
        Some([
            font.color.r as f32 / 255.0,
            font.color.g as f32 / 255.0,
            font.color.b as f32 / 255.0,
            font.color.a as f32 / 255.0,
        ])
    } else {
        None
    };

    (size, color)
}

#[note("canvas-paint-controls")]
pub(crate) fn paint_controls(
    p: &mut Brush,
    ctls: &std::collections::BTreeMap<NodeId, MakepadCtl>,
    ordem: &[NodeId],
    modo: crawrad_runtime::Mode,
    forma: RectIr,
    form_font: Option<&Font>,
) {
    let scene = Scene::new(ctls);
    for passo in draw_program(&draw_order(ordem, ctls, modo), ctls) {
        let (Step::Paint(id) | Step::Overlay(id)) = passo;
        let Some(c) = ctls.get(id) else { continue };
        let request = Clipped::of_control(c, forma);
        let r = p.na_tela(request.item);
        let clip = request.clip.map(|x| p.na_tela(x));
        paint_clipped(p, clip, |p| match passo {
            // TWO nested clips, and they are different rules: the outer one is the
            // PARENT's (`clip_of`) and the inner one is the BOX'S OWN
            // ([`content_clip_of`]) — the `ListBox` line that appeared
            // beneath its border. What intersects the two is Makepad's
            // turtle stack (`turtle.rs:749-758`), so neither
            // needs to know about the other.
            Step::Paint(_) => {
                let own = content_clip_of(c).map(|x| p.na_tela(x));
                paint_clipped(p, own, |p| {
                    let (font_size, font_color) = effective_font(&c.props, form_font);
                    let font_color = if c.enabled { font_color } else { None };
                    p.with_font(font_size, font_color, |p| {
                        // **WHAT IS DECLARED COMES FIRST, and order is the rule.**
                        //
                        // A class that says how it paints controls its own
                        // painting: it is the third-party component of ADR 0036, which
                        // until 18/09/2026 appeared as a dashed rectangle with the
                        // name inside.
                        //
                        // Built-in declares nothing, so `pintar` returns
                        // `false` and the usual chain continues intact —
                        // `desenho_de` and, at the bottom, the dashed line.
                        if !crate::components::declarado::pintar(p, c, r, &c.desenho) {
                            desenho_de(&c.class).draw(p, c, r, &scene);
                        }
                    });
                });
            }
            // The container CHROME is **not** clipped to its own box: it
            // is a sheet over the children, and that is why it exists.
            // What it gets is LAYER — above the marquee and below the
            // handle —, and [`Brush::as_chrome`] puts it there.
            Step::Overlay(_) => {
                p.as_chrome(|p| desenho_de(&c.class).draw_over(p, c, r, &scene));
            }
        });
    }

    // The form strip (e.g. MainMenu's bar), along the top of the form
    // and over the controls — where the generated applications draw it
    for c in ctls.values() {
        desenho_de(&c.class).draw_form_strip(p, c, forma, &scene);
    }
}

#[note("canvas-paint-clipped")]
fn paint_clipped(p: &mut Brush, clip: Option<Rect>, paint: impl FnOnce(&mut Brush)) {
    match clip {
        Some(r) => p.with_clip(r, paint),
        None => paint(p),
    }
}

#[note("canvas-draw-ruler")]
fn draw_ruler(p: &mut Brush, area: Rect, ponteiro: Option<DVec2>, landing: Option<RectIr>) {
    let esp = RULER_THICKNESS as f64;
    if area.size.x <= esp || area.size.y <= esp {
        return;
    }
    let topo = Rect {
        pos: area.pos,
        size: dvec2(area.size.x, esp),
    };
    let lado = Rect {
        pos: area.pos,
        size: dvec2(esp, area.size.y),
    };
    p.ruler(topo, MAKEPAD_REGUA_FUNDO);
    p.ruler(lado, MAKEPAD_REGUA_FUNDO);

    // The line separating the ruler from the canvas board.
    let fio = 1.0;
    p.ruler(
        Rect {
            pos: dvec2(area.pos.x, area.pos.y + esp - fio),
            size: dvec2(area.size.x, fio),
        },
        MAKEPAD_REGUA_TRACO,
    );
    p.ruler(
        Rect {
            pos: dvec2(area.pos.x + esp - fio, area.pos.y),
            size: dvec2(fio, area.size.y),
        },
        MAKEPAD_REGUA_TRACO,
    );

    let t = *p.transformacao();
    let fim_x = area.pos.x + area.size.x;
    let fim_y = area.pos.y + area.size.y;

    for m in ticks(&t, false, area.pos.x + esp, fim_x) {
        let alto = if m.major { esp * 0.55 } else { esp * 0.3 };
        p.ruler(
            Rect {
                pos: dvec2(m.on_screen, area.pos.y + esp - alto),
                size: dvec2(fio, alto),
            },
            MAKEPAD_REGUA_TRACO,
        );
        if m.major {
            p.write_on_ruler(
                dvec2(m.on_screen + 2.0, area.pos.y + 1.0),
                &format!("{:.0}", m.value),
                MAKEPAD_REGUA_TEXTO,
            );
        }
    }

    // The number on the vertical ruler appears horizontally, not rotated: rotating would require
    // a gallery per tick just to save width that the ruler already has.
    for m in ticks(&t, true, area.pos.y + esp, fim_y) {
        let largo = if m.major { esp * 0.55 } else { esp * 0.3 };
        p.ruler(
            Rect {
                pos: dvec2(area.pos.x + esp - largo, m.on_screen),
                size: dvec2(largo, fio),
            },
            MAKEPAD_REGUA_TRACO,
        );
        if m.major {
            p.write_on_ruler(
                dvec2(area.pos.x + 1.0, m.on_screen + 1.0),
                &format!("{:.0}", m.value),
                MAKEPAD_REGUA_TEXTO,
            );
        }
    }

    // Where the pointer is, on both rulers.
    if let Some(m) = ponteiro {
        if m.x >= area.pos.x + esp && m.x <= fim_x {
            p.ruler(
                Rect {
                    pos: dvec2(m.x, area.pos.y),
                    size: dvec2(fio, esp),
                },
                MAKEPAD_REGUA_PONTEIRO,
            );
        }
        if m.y >= area.pos.y + esp && m.y <= fim_y {
            p.ruler(
                Rect {
                    pos: dvec2(area.pos.x, m.y),
                    size: dvec2(esp, fio),
                },
                MAKEPAD_REGUA_PONTEIRO,
            );
        }
    }

    // The snapped left and top edges of a move, in the accent, with
    // the number they snapped to.
    if let Some(l) = landing {
        let at = p.na_tela(l).pos;
        if at.x >= area.pos.x + esp && at.x <= fim_x {
            let mark = Rect {
                pos: dvec2(at.x, area.pos.y),
                size: dvec2(2.0, esp),
            };
            p.ruler(mark, MAKEPAD_SNAP);
            let number = format!("{:.0}", l.x);
            p.write_on_ruler(dvec2(at.x + 4.0, area.pos.y + 1.0), &number, MAKEPAD_SNAP);
        }
        if at.y >= area.pos.y + esp && at.y <= fim_y {
            let mark = Rect {
                pos: dvec2(area.pos.x, at.y),
                size: dvec2(esp, 2.0),
            };
            p.ruler(mark, MAKEPAD_SNAP);
            let number = format!("{:.0}", l.y);
            p.write_on_ruler(dvec2(area.pos.x + 1.0, at.y + 3.0), &number, MAKEPAD_SNAP);
        }
    }
}

impl Widget for DesignCanvas {
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        // The canvas's OWN list opens before the turtle, as in `View` (`view.rs:817`).
        // It is what makes layer order an invariant of the canvas — see the
        // [`DesignCanvas::lista`] field.
        if self.lista.is_none() {
            self.lista = Some(DrawList2d::new(cx));
        }
        let mut lista = self.lista.take().expect("a lista acabou de ser criada");
        lista.begin_always(cx);

        cx.begin_turtle(walk, self.layout);

        // `abs_pos` is absolute in the PASS, not relative to the parent
        // (`turtle.rs:279-281`). From here comes the transformation origin, and it is the
        // only place in the file that knows where the form landed.
        let area = cx.turtle().rect();
        // The DRAWING AREA is the surface MINUS the ruler — it is
        // `area_de_desenho` in the RXP (`crawrad-runtime-egui/src/lib.rs:529-546`), and
        // it exists for the same reason: using the whole surface puts the
        // form under the ruler and makes a click on the ruler start a drag.
        //
        // In Preview there is no ruler: what is on screen is the application, and
        // applications do not have a board.
        let com_regua = self.rt.modo == crawrad_runtime::Mode::Design && !self.read_only;
        let recuo = if com_regua {
            RULER_THICKNESS as f64
        } else {
            0.0
        };
        self.origem = area.pos + dvec2(recuo, recuo) - self.deslocamento;
        let t = self.transformacao();

        // The model is read HERE, before the brush borrows the shaders. After
        // this block nothing else queries the runtime in this frame.
        let modo = self.rt.modo;
        let design = modo == crawrad_runtime::Mode::Design;
        let forma = self.rt.form_rect;
        let pontos = if design {
            calculate_grid_points(forma, self.rt.grid)
        } else {
            Vec::new()
        };
        // The components, in the Z-order defined by the document —
        // `em_ordem_de_desenho()` was already resolved by the runtime (`lib.rs:318`)
        // and the rectangle is already `rect_no_formulario` (`crawrad-runtime/src/base.rs`).
        // None of this is recomputed here.
        let ctls = self.rt.ctls.clone();
        let ordem = self.rt.ordem.clone();
        let adornments = if self.read_only {
            let mut a = Adornments::default();
            for id in &self.overflow {
                if !self.rt.desenhavel(id) {
                    continue;
                }
                if let Some(c) = self.rt.ctls.get(id) {
                    a.overflow.push(c.rect);
                    a.overflow_marks
                        .extend(overflow_marks(self.rt.form_rect, c.rect));
                }
            }
            a
        } else if design {
            adornments(&self.rt)
        } else {
            Adornments::default()
        };

        // **FORM TEXT SCALES WITH ZOOM** — handled
        // here. `DrawText` has `font_scale`
        // (`makepad-draw-1.0.0/src/shader/draw_text.rs:102`), applied to the glyph
        // in `:403` and `:512`; without it, a button of twice the size came out with the
        // caption at regular size — *"it is not an enlarged button, it is a wrong
        // button"*, as the RXP wrote (`crawrad-runtime-egui/src/lib.rs:2480`).
        //
        // Two things intentionally do NOT enter this:
        //
        // - the **ruler**, which is chrome and measures the design: `draw_texto_regua` is
        //   another instance and stays at `font_scale` 1;
        // - the **measurement**, which already multiplies by `font_scale`
        //   ([`Brush::measure`]) — `layout` returns the size WITHOUT it
        //   (`draw_text.rs:179`), and without this multiplication the accelerator
        //   underline and the `GroupBox` caption would shift out of place with zoom.
        let escala = t.escala.max(0.01);
        self.draw_texto.font_scale = escala;
        self.draw_texto_selo.font_scale = escala;

        // Where the pointer is, for the ruler to mark it on both strips. Read
        // HERE, with the rest of the model, because after `pincel` nothing else
        // queries the widget.
        let ponteiro = self.ponteiro;
        let snap = self.rt.snap;
        let landing = self.rt.ghost.map(|(l, _)| l);
        let form_font = self.rt.form_font.clone();

        let read_only = self.read_only;
        let page_look = self.page_look;
        let mut p = self.pincel(cx, t);

        // **FIRST THING OF THE FRAME.** Without this, the layer order is the order
        // of the DOCUMENT — a form that begins with a `Label` opens the text
        // layer before the boxes layer, and then every box covers all text. See
        // [`crate::canvas::Layer`].
        p.open_layers(area.pos);

        if area.size.x.is_finite() && area.size.y.is_finite() {
            p.background(area, MAKEPAD_BG);
        }

        // The form paper — FLAT fill, like the RXP
        // (`crawrad-runtime-egui/src/lib.rs:1500`, `rect_filled(form, 0.0, PAPEL)`).
        //
        // It comes out via `fundo`, and not via `caixa`, and the reason is the layer
        // order explained in the DSL: if paper were the first `DrawCaixa` of the
        // frame, every component would be appended to ITS call and would end up
        // BELOW the grid.
        let papel = p.na_tela(forma);
        p.set_light(page_look.light);
        p.background(papel, MAKEPAD_PAPER);

        if !pontos.is_empty() && !page_look.web {
            let lado = p.medida(1.0).max(1.0);
            p.grid(&pontos, lado, snap);
        }

        // The paper frame. In line, not in box, for the same layer
        // reason — and because the form's border belongs to the CANVAS, not to a
        // control: it must not catch the eye more than an `Edit` outline.
        if !page_look.web {
            p.paper_frame(papel, MAKEPAD_PANEL_BORDER, 1.0);
        }

        // Each component resolves its own drawing: the canvas only hands over the
        // brush, the already-converted rectangle, and the neighborhood. **The order is that of
        // [`draw_order`]**, not of the document.
        //
        // **THE RECTANGLE PASSES WHOLE**. The brush clips it, from
        // above: the component stays the size the document gave it,
        // and whatever passes the parent's border simply is not painted — see
        // [`clip_of`] and [`Brush::with_clip`]. Shrinking `r` here to fit
        // would be the defect that the rule forbids, and is exactly the mutation that the
        // RXP test catches.
        //
        // **The order is that of [`draw_program`]**, not the raw list: it is what
        // puts the `ScrollBox` bar AFTER its children.
        if read_only {
            paint_clipped(&mut p, Some(papel), |p| {
                paint_controls(p, &ctls, &ordem, modo, forma, form_font.as_ref());
            });
        } else {
            paint_controls(&mut p, &ctls, &ordem, modo, forma, form_font.as_ref());
        }

        p.set_light(false);
        if design || read_only {
            draw_adornments(&mut p, &adornments);
            if design && !read_only {
                draw_ruler(&mut p, area, ponteiro, landing);
            }
        }

        drop(p);

        if self.atlas.upload(cx) {
            self.area.redraw(cx);
        }

        cx.end_turtle_with_area(&mut self.area);
        lista.end(cx);
        self.lista = Some(lista);
        DrawStep::done()
    }

    #[note("canvas-designcanvas-handle-event")]
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if self.read_only {
            if let Hit::FingerHoverIn(_) = event.hits(cx, self.area) {
                cx.set_cursor(MouseCursor::Default);
            }
            return;
        }
        // The Preview is a dry view; nothing in it answers, not even the designer.
        if self.rt.modo.is_live() {
            if let Hit::FingerHoverIn(_) = event.hits(cx, self.area) {
                cx.set_cursor(MouseCursor::Default);
            }
            return;
        }
        let t = self.transformacao();
        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(fe) | Hit::FingerHoverOver(fe) => {
                let (x, y) = point_in_design(&t, fe.abs);
                let c = self.cursor_em(x, y);
                cx.set_cursor(c);
                // The pointer mark on the ruler follows the hand, which is why the
                // frame is redrawn. It is the same cost the RXP pays, and is what
                // separates a ruler that measures from one that decorates.
                if self.ponteiro != Some(fe.abs) {
                    self.ponteiro = Some(fe.abs);
                    self.area.redraw(cx);
                }
            }
            // Pointer out: its mark leaves both rulers.
            Hit::FingerHoverOut(_) if self.ponteiro.take().is_some() => {
                self.area.redraw(cx);
            }
            Hit::FingerDown(fe) => {
                cx.set_key_focus(self.area);
                let toque = Touch::Down {
                    ctrl: fe.modifiers.control,
                    double: fe.tap_count == 2,
                };
                self.enfileirar(toque, &t, fe.abs);
                self.rt.capturando = true;
            }
            Hit::FingerMove(fe) => {
                // Capture is automatic: `FingerMove` keeps arriving with
                // the pointer OUTSIDE the area (`finger.rs:631-644`).
                let free = fe.modifiers.alt;
                self.enfileirar(Touch::Move { free }, &t, fe.abs);
            }
            Hit::FingerUp(fe) => {
                self.enfileirar(Touch::Up, &t, fe.abs);
                self.rt.capturando = false;
            }
            Hit::KeyDown(ke) if ke.key_code == KeyCode::Escape => {
                self.enfileirar(Touch::Cancel, &t, DVec2::default());
            }
            _ => (),
        }
    }
}

#[cfg(test)]
#[path = "test/canvas/test.rs"]
mod tests;
