//! The canvas **brush**: the only painting surface a component ever sees.
//!
//! It exists so that `src/componentes/<class>.rs` can draw without knowing
//! [`crate::canvas::DesignCanvas`], without touching `Cx2d` and — what matters
//! most — **without computing a coordinate**. The rectangle arrives already
//! converted by [`rect_on_screen`], which is the single [`Transformacao`] of
//! ADR 0019;
//! the brush re-exposes that conversion, never a second computation.
//!
//! The shaders come in from the outside by reference: in Makepad a `DrawQuad`
//! only paints if its class came from a live file
//! (`makepad-platform-1.0.0/src/draw_vars.rs:609-617`), so they are `#[live]`
//! fields of the widget and the brush borrows them. It is the framework's golden
//! rule: **one shader declared, N instances emitted**
//! (`makepad-draw-1.0.0/src/shader/draw_quad.rs:127`).

use makepad_widgets::*;

use crawrad_ir::Rect as RectIr;

use super::atlas::PictureAtlas;
use super::{
    DrawAlca, DrawCaixa, DrawChrome, DrawChromeArrow, DrawDot, DrawPicture, DrawRegua, DrawRisca,
    DrawSelo, DrawSeta, DrawTextoRegua, DrawTextoSelo, DrawTracejado, DrawVisto,
};
use crate::Transformacao;
use crate::components::comum::*;
use crawrad_note::note;

#[note("canvas-brush-contentline")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentLine {
    /// The indent level: 0 is the root.
    pub level: usize,
    /// What gets written.
    pub text: String,
    /// Does the line have children, and therefore earn the expand marker?
    pub branch: bool,
    /// Is the branch expanded?
    pub expanded: bool,
}

impl ContentLine {
    #[note("canvas-brush-contentline-leaf")]
    pub fn leaf(level: usize, text: impl Into<String>) -> Self {
        Self {
            level,
            text: text.into(),
            branch: false,
            expanded: true,
        }
    }

    #[note("canvas-brush-contentline-branch")]
    pub fn branch(level: usize, text: impl Into<String>) -> Self {
        Self {
            level,
            text: text.into(),
            branch: true,
            expanded: true,
        }
    }

    #[note("canvas-brush-contentline-branch-expanded")]
    pub fn branch_expanded(level: usize, text: impl Into<String>, expanded: bool) -> Self {
        Self {
            level,
            text: text.into(),
            branch: true,
            expanded,
        }
    }
}

#[note("canvas-brush-content")]
pub struct Content<'a> {
    pub lines: &'a [ContentLine],
    /// Height of one line, in design units.
    pub line_height: f32,
    pub color: [f32; 4],
    /// Which line is chosen, if any.
    pub selected: Option<usize>,
}

#[note("canvas-brush-lines-that-fit")]
pub fn lines_that_fit(h: f32, line_height: f32) -> usize {
    if line_height <= 0.0 || h < line_height {
        return 0;
    }
    (h / line_height).floor() as usize
}

#[note("canvas-brush-rect-on-screen")]
pub fn rect_on_screen(t: &Transformacao, r: RectIr) -> Rect {
    let (x, y) = t.screen_point(r.x, r.y);
    Rect {
        pos: dvec2(x as f64, y as f64),
        size: dvec2(t.medida(r.w) as f64, t.medida(r.h) as f64),
    }
}

#[note("canvas-is-double-tap")]
pub fn is_double_tap(previous: Option<(DVec2, f64)>, abs: DVec2, time: f64) -> bool {
    use makepad_widgets::makepad_platform::event::{TAP_COUNT_DISTANCE, TAP_COUNT_TIME};
    previous.is_some_and(|(at, when)| {
        time - when < TAP_COUNT_TIME && abs.distance(&at) < TAP_COUNT_DISTANCE
    })
}

#[note("canvas-brush-point-in-design")]
pub fn point_in_design(t: &Transformacao, p: DVec2) -> (f32, f32) {
    t.em_design(p.x as f32, p.y as f32)
}

#[note("canvas-brush-shrink")]
pub fn shrink(r: Rect, d: f64) -> Rect {
    if r.size.x <= d * 2.0 || r.size.y <= d * 2.0 {
        r
    } else {
        Rect {
            pos: dvec2(r.pos.x + d, r.pos.y + d),
            size: dvec2(r.size.x - d * 2.0, r.size.y - d * 2.0),
        }
    }
}

#[note("canvas-brush-frame-sides")]
fn frame_sides(r: Rect, thickness: f64) -> [Rect; 4] {
    let e = thickness.max(1.0);
    [
        Rect {
            pos: r.pos,
            size: dvec2(r.size.x, e),
        },
        Rect {
            pos: dvec2(r.pos.x, r.pos.y + r.size.y - e),
            size: dvec2(r.size.x, e),
        },
        Rect {
            pos: r.pos,
            size: dvec2(e, r.size.y),
        },
        Rect {
            pos: dvec2(r.pos.x + r.size.x - e, r.pos.y),
            size: dvec2(e, r.size.y),
        },
    ]
}

#[note("canvas-brush-rgba")]
pub fn rgba(c: [f32; 4]) -> Vec4 {
    Vec4 {
        x: c[0],
        y: c[1],
        z: c[2],
        w: c[3],
    }
}

#[note("canvas-brush-layer")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    /// The canvas backdrop, the form paper and its frame.
    Background,
    /// The grid dots.
    GridDot,
    /// Every component's box, the highlight of the chosen line and the tab
    /// order badge.
    Box,
    /// Decoded picture atlas instances.
    Picture,
    /// Every component's text.
    Text,
    /// Accelerator underline, `GroupBox` frame, tree marker and alignment
    /// guides — **above the text**, because the underline belongs to the text.
    Stroke,
    /// The `ComboBox` and scroll bar arrows.
    Arrow,
    /// The `CheckBox` check mark.
    Check,
    /// The box of the non-visual badge. The owner's rule: *"the drawings of
    /// non-visual components in designer mode must always sit in front of every
    /// visual component"*.
    Badge,
    /// The badge initials and name, right above it.
    BadgeText,
    /// The marquee and the selection outline.
    Dashed,
    /// The chrome a container paints over its own children — the trough, the
    /// buttons and the thumb of a `ScrollBox` bar.
    ///
    /// **Above the marquee**, and the owner is the one who asked: the dashed
    /// outline of a `button3` cut by the container was crossing the whole bar
    /// while the face of that very button stopped at it — *the same rectangle
    /// drawn under two laws*. The marquee is decoration; nobody grabs it, so
    /// the bar may cover it.
    Chrome,
    /// The chrome arrow, right above the chrome box.
    ChromeArrow,
    /// The handles — what the user grabs.
    ///
    /// **Above the chrome**, and for a different reason than the marquee's: the
    /// handle **is** what gets grabbed, and `Designer::hit_handle` does not know
    /// a bar exists. A handle hidden under the bar and still draggable would be
    /// the interface lying — the exact opposite of what was fixed.
    Handle,
    /// The ruler band and its strokes.
    Ruler,
    /// The ruler numbers — **the top of the stack**. A ruler measures what is
    /// underneath it; nothing may pass over it.
    RulerText,
}

#[note("canvas-brush-layers")]
pub const LAYERS: [Layer; 16] = [
    Layer::Background,
    Layer::GridDot,
    Layer::Box,
    Layer::Picture,
    Layer::Text,
    Layer::Stroke,
    Layer::Arrow,
    Layer::Check,
    Layer::Badge,
    Layer::BadgeText,
    Layer::Dashed,
    Layer::Chrome,
    Layer::ChromeArrow,
    Layer::Handle,
    Layer::Ruler,
    Layer::RulerText,
];

#[note("canvas-brush-primitives")]
pub struct Primitives<'a> {
    pub rounded_box: &'a mut DrawCaixa,
    pub picture: &'a mut DrawPicture,
    pub text: &'a mut DrawText,
    pub badge: &'a mut DrawSelo,
    pub badge_text: &'a mut DrawTextoSelo,
    pub ruler: &'a mut DrawRegua,
    pub ruler_text: &'a mut DrawTextoRegua,
    pub arrow: &'a mut DrawSeta,
    pub check: &'a mut DrawVisto,
    pub dashed: &'a mut DrawTracejado,
    pub handle: &'a mut DrawAlca,
    pub chrome_box: &'a mut DrawChrome,
    pub chrome_arrow: &'a mut DrawChromeArrow,
    pub background: &'a mut DrawColor,
    pub grid_dot: &'a mut DrawDot,
    pub stroke: &'a mut DrawRisca,
    pub atlas: &'a mut PictureAtlas,
}

#[note("canvas-brush-mark")]
#[derive(Debug, Clone, PartialEq)]
pub struct Mark {
    pub layer: Layer,
    /// Where the ink landed, already cut by every clip around it.
    pub rect: Rect,
    /// The fill, or the colour of a line, an arrow or a text.
    pub color: [f32; 4],
    /// The border, for the primitives that have one.
    pub border: Option<[f32; 4]>,
    /// What was written, for the text primitives.
    pub text: Option<String>,
    /// The font size in points, or None for the theme font.
    pub font_size: Option<f32>,
}

#[note("canvas-brush-probe-char-width")]
pub const PROBE_CHAR_WIDTH: f64 = 7.0;

/// Default font points used as the baseline for probe scaling.
pub const PROBE_FONT_POINTS: f64 = 10.0;

#[note("canvas-brush-probe-line-height")]
pub const PROBE_LINE_HEIGHT: f64 = 14.0;

#[note("canvas-brush-probe")]
#[derive(Debug, Default)]
pub struct Probe {
    marks: Vec<Mark>,
    clips: Vec<Rect>,
}

impl Probe {
    #[note("canvas-brush-probe-marks")]
    pub fn marks(&self) -> &[Mark] {
        &self.marks
    }

    fn mark(
        &mut self,
        layer: Layer,
        r: Rect,
        color: [f32; 4],
        border: Option<[f32; 4]>,
        text: Option<&str>,
        font_size: Option<f32>,
    ) {
        // The GPU clamps every vertex to the clip around it; the probe keeps
        // the same ink, and a mark cut down to nothing was never painted.
        let mut rect = r;
        if let Some(clip) = self.clips.last() {
            let x0 = rect.pos.x.max(clip.pos.x);
            let y0 = rect.pos.y.max(clip.pos.y);
            let x1 = (rect.pos.x + rect.size.x).min(clip.pos.x + clip.size.x);
            let y1 = (rect.pos.y + rect.size.y).min(clip.pos.y + clip.size.y);
            if x1 <= x0 || y1 <= y0 {
                return;
            }
            rect = Rect {
                pos: dvec2(x0, y0),
                size: dvec2(x1 - x0, y1 - y0),
            };
        }
        self.marks.push(Mark {
            layer,
            rect,
            color,
            border,
            text: text.map(str::to_string),
            font_size,
        });
    }

    fn push_clip(&mut self, clip: Rect) {
        let clip = match self.clips.last() {
            Some(outer) => {
                let x0 = clip.pos.x.max(outer.pos.x);
                let y0 = clip.pos.y.max(outer.pos.y);
                let x1 = (clip.pos.x + clip.size.x).min(outer.pos.x + outer.size.x);
                let y1 = (clip.pos.y + clip.size.y).min(outer.pos.y + outer.size.y);
                Rect {
                    pos: dvec2(x0, y0),
                    size: dvec2((x1 - x0).max(0.0), (y1 - y0).max(0.0)),
                }
            }
            None => clip,
        };
        self.clips.push(clip);
    }
}

#[note("canvas-brush-surface")]
enum Surface<'a, 'b, 'c> {
    Gpu {
        cx: &'a mut Cx2d<'b, 'c>,
        p: Primitives<'a>,
    },
    Probe(&'a mut Probe),
}

#[note("canvas-brush-brush")]
pub struct Brush<'a, 'b, 'c> {
    s: Surface<'a, 'b, 'c>,
    t: Transformacao,
    /// While this is on, [`Brush::rounded_box`] and [`Brush::arrow`] emit on the
    /// **container chrome** layers instead of the component ones.
    ///
    /// A flag, and not a second `paint_bar`: there is exactly ONE scroll bar
    /// drawing in this crate and it is shared by the `ScrollBar` control, by
    /// the `Memo`/`ListBox`/`TreeView` and by the `ScrollBox`. Copying it to
    /// change two shaders would be the fourth copy of the thing the RXP already
    /// paid for having copied four times
    /// (`crawrad-runtime-egui/src/lib.rs:2950`).
    chrome: bool,
    probe_atlas: Option<PictureAtlas>,
    font_size: Option<f32>,
    font_color: Option<[f32; 4]>,
    /// The light page: every role colour goes through
    /// [`light_page_ink`] before it reaches a shader.
    light: bool,
}

impl<'a, 'b, 'c> Brush<'a, 'b, 'c> {
    #[note("canvas-brush-brush-new")]
    pub(crate) fn new(cx: &'a mut Cx2d<'b, 'c>, p: Primitives<'a>, t: Transformacao) -> Self {
        Self {
            s: Surface::Gpu { cx, p },
            t,
            chrome: false,
            probe_atlas: None,
            font_size: None,
            font_color: None,
            light: false,
        }
    }

    /// Paints the form's roles in their light twins from here on.
    pub(crate) fn set_light(&mut self, light: bool) {
        self.light = light;
    }

    fn ink(&self, c: [f32; 4]) -> [f32; 4] {
        if self.light {
            crate::components::comum::light_page_ink(c)
        } else {
            c
        }
    }

    #[note("canvas-brush-brush-probing")]
    pub fn probing(probe: &'a mut Probe, t: Transformacao) -> Self {
        Self {
            s: Surface::Probe(probe),
            t,
            chrome: false,
            probe_atlas: Some(PictureAtlas::new()),
            font_size: None,
            font_color: None,
            light: false,
        }
    }

    #[note("canvas-brush-brush-measure-on")]
    fn measure_on(&mut self, layer: Layer, text: &str) -> DVec2 {
        if text.is_empty() {
            return dvec2(0.0, 0.0);
        }
        let scale = self.t.escala.max(0.01) as f64;
        let (cx, shader) = match &mut self.s {
            Surface::Probe(_) => {
                let font_scale = match layer {
                    Layer::BadgeText | Layer::RulerText => 1.0,
                    _ => self.font_size.map_or(1.0, |s| s as f64 / PROBE_FONT_POINTS),
                };
                return dvec2(
                    text.chars().count() as f64 * PROBE_CHAR_WIDTH * scale * font_scale,
                    PROBE_LINE_HEIGHT * scale * font_scale,
                );
            }
            Surface::Gpu { cx, p } => match layer {
                Layer::BadgeText => (cx, &mut p.badge_text.draw_super),
                _ => (cx, &mut *p.text),
            },
        };
        let m = shader.layout(cx, 0.0, 0.0, None, false, Align::default(), text);
        let e = shader.font_scale as f64;
        dvec2(
            m.size_in_lpxs.width as f64 * e,
            m.size_in_lpxs.height as f64 * e,
        )
    }

    #[note("canvas-brush-brush-ink-text")]
    fn ink_text(&mut self, layer: Layer, at: DVec2, size: DVec2, text: &str, c: [f32; 4]) {
        let c = self.ink(c);
        match &mut self.s {
            Surface::Probe(probe) => {
                let (color, font_size) = match layer {
                    Layer::BadgeText | Layer::RulerText => (c, None),
                    _ => (self.font_color.unwrap_or(c), self.font_size),
                };
                probe.mark(
                    layer,
                    Rect { pos: at, size },
                    color,
                    None,
                    Some(text),
                    font_size,
                );
            }
            Surface::Gpu { cx, p } => match layer {
                Layer::BadgeText => {
                    p.badge_text.color = rgba(c);
                    p.badge_text.draw_abs(cx, at, text);
                }
                Layer::RulerText => {
                    p.ruler_text.color = rgba(c);
                    p.ruler_text.draw_abs(cx, at, text);
                }
                _ => {
                    p.text.color = rgba(self.font_color.unwrap_or(c));
                    p.text.draw_abs(cx, at, text);
                }
            },
        }
    }

    #[note("canvas-brush-brush-as-chrome")]
    pub(crate) fn as_chrome(&mut self, paint: impl FnOnce(&mut Self)) {
        let previous = self.chrome;
        self.chrome = true;
        paint(self);
        self.chrome = previous;
    }

    /// Scopes an effective font size and colour for the duration of `paint`.
    ///
    /// Saves and restores the previous font state. Only affects plain text drawing
    /// (`p.text`), never badge or ruler text.
    pub fn with_font(
        &mut self,
        size: Option<f32>,
        color: Option<[f32; 4]>,
        paint: impl FnOnce(&mut Self),
    ) {
        let prev_size = self.font_size;
        let prev_color = self.font_color;
        let prev_gpu_size = match &mut self.s {
            Surface::Gpu { p, .. } => {
                let prev = p.text.text_style.font_size;
                if let Some(s) = size {
                    p.text.text_style.font_size = s;
                }
                Some(prev)
            }
            Surface::Probe(_) => None,
        };

        self.font_size = size;
        self.font_color = color;

        paint(self);

        self.font_size = prev_size;
        self.font_color = prev_color;
        if let Surface::Gpu { p, .. } = &mut self.s
            && let Some(prev) = prev_gpu_size
        {
            p.text.text_style.font_size = prev;
        }
    }

    #[note("canvas-brush-brush-open-layers")]
    pub(crate) fn open_layers(&mut self, at: DVec2) {
        let Surface::Gpu { cx, p } = &mut self.s else {
            return;
        };
        let nothing = Rect {
            pos: at,
            size: dvec2(0.0, 0.0),
        };
        let invisible = rgba([0.0, 0.0, 0.0, 0.0]);
        for layer in LAYERS {
            match layer {
                Layer::Background => {
                    p.background.color = invisible;
                    p.background.draw_abs(cx, nothing);
                }
                Layer::GridDot => {
                    p.grid_dot.cor = invisible;
                    p.grid_dot.draw_abs(cx, nothing);
                }
                Layer::Box => {
                    p.rounded_box.fundo = invisible;
                    p.rounded_box.borda = invisible;
                    p.rounded_box.draw_abs(cx, nothing);
                }
                Layer::Picture => {
                    if let Some(texture) = p.atlas.texture() {
                        p.picture.draw_vars.set_texture(0, texture);
                    } else {
                        p.picture.draw_vars.empty_texture(0);
                    }
                    p.picture.uv = Vec4::default();
                    p.picture.draw_abs(cx, nothing);
                }
                Layer::Text => {
                    p.text.color = invisible;
                    p.text.draw_abs(cx, at, " ");
                }
                Layer::Stroke => {
                    p.stroke.cor = invisible;
                    p.stroke.draw_abs(cx, nothing);
                }
                Layer::Arrow => {
                    p.arrow.cor = invisible;
                    p.arrow.draw_abs(cx, nothing);
                }
                Layer::Check => {
                    p.check.cor = invisible;
                    p.check.draw_abs(cx, nothing);
                }
                Layer::Badge => {
                    p.badge.fundo = invisible;
                    p.badge.borda = invisible;
                    p.badge.draw_abs(cx, nothing);
                }
                Layer::BadgeText => {
                    p.badge_text.color = invisible;
                    p.badge_text.draw_abs(cx, at, " ");
                }
                Layer::Dashed => {
                    p.dashed.cor = invisible;
                    p.dashed.draw_abs(cx, nothing);
                }
                Layer::Chrome => {
                    p.chrome_box.fundo = invisible;
                    p.chrome_box.borda = invisible;
                    p.chrome_box.draw_abs(cx, nothing);
                }
                Layer::ChromeArrow => {
                    p.chrome_arrow.cor = invisible;
                    p.chrome_arrow.draw_abs(cx, nothing);
                }
                Layer::Handle => {
                    p.handle.cor = invisible;
                    p.handle.draw_abs(cx, nothing);
                }
                Layer::Ruler => {
                    p.ruler.cor = invisible;
                    p.ruler.draw_abs(cx, nothing);
                }
                Layer::RulerText => {
                    p.ruler_text.color = invisible;
                    p.ruler_text.draw_abs(cx, at, " ");
                }
            }
        }
    }

    #[note("canvas-brush-brush-with-clip")]
    pub fn with_clip(&mut self, clip: Rect, paint: impl FnOnce(&mut Self)) {
        if clip.size.x <= 0.0 || clip.size.y <= 0.0 {
            return;
        }
        match &mut self.s {
            Surface::Probe(probe) => probe.push_clip(clip),
            Surface::Gpu { cx, .. } => cx.begin_turtle(
                Walk::abs_rect(clip),
                Layout {
                    clip_x: true,
                    clip_y: true,
                    ..Layout::default()
                },
            ),
        }
        paint(self);
        match &mut self.s {
            Surface::Probe(probe) => {
                probe.clips.pop();
            }
            Surface::Gpu { cx, .. } => {
                cx.end_turtle();
            }
        }
    }

    #[note("canvas-brush-brush-transformacao")]
    pub fn transformacao(&self) -> &Transformacao {
        &self.t
    }

    #[note("canvas-brush-brush-na-tela")]
    pub fn na_tela(&self, r: RectIr) -> Rect {
        rect_on_screen(&self.t, r)
    }

    #[note("canvas-brush-brush-medida")]
    pub fn medida(&self, v: f32) -> f64 {
        self.t.medida(v) as f64
    }

    #[note("canvas-brush-brush-inset")]
    pub fn inset(&self, r: Rect) -> Rect {
        shrink(r, self.medida(4.0).max(2.0))
    }

    #[note("canvas-brush-brush-rounded-box")]
    pub fn rounded_box(&mut self, r: Rect, fill: [f32; 4], border: [f32; 4], radius: f64) {
        let (fill, border) = (self.ink(fill), self.ink(border));
        if r.size.x < 1.0 || r.size.y < 1.0 {
            return;
        }
        // The SAME drawing, on two layers: inside [`Brush::as_chrome`] it comes
        // out above the marquee; outside, together with the components.
        match &mut self.s {
            Surface::Probe(probe) => {
                let layer = if self.chrome {
                    Layer::Chrome
                } else {
                    Layer::Box
                };
                probe.mark(layer, r, fill, Some(border), None, None);
            }
            Surface::Gpu { cx, p } if self.chrome => {
                p.chrome_box.fundo = rgba(fill);
                p.chrome_box.borda = rgba(border);
                p.chrome_box.raio = radius as f32;
                p.chrome_box.espessura = 1.0;
                p.chrome_box.draw_abs(cx, r);
            }
            Surface::Gpu { cx, p } => {
                p.rounded_box.fundo = rgba(fill);
                p.rounded_box.borda = rgba(border);
                p.rounded_box.raio = radius as f32;
                p.rounded_box.espessura = 1.0;
                p.rounded_box.draw_abs(cx, r);
            }
        }
    }

    #[note("canvas-brush-brush-picture")]
    pub fn picture(&mut self, r: Rect, bytes: &[u8]) -> bool {
        self.picture_part(r, bytes, [0.0, 0.0, 1.0, 1.0])
    }

    #[note("canvas-brush-brush-picture-part")]
    pub fn picture_part(&mut self, r: Rect, bytes: &[u8], part: [f32; 4]) -> bool {
        if bytes.is_empty() || r.size.x < 1.0 || r.size.y < 1.0 {
            return false;
        }
        match &mut self.s {
            Surface::Probe(probe) => {
                let Some(atlas) = self.probe_atlas.as_mut() else {
                    return false;
                };
                if atlas.uv_of(bytes).is_none() {
                    return false;
                }
                probe.mark(Layer::Picture, r, [1.0; 4], None, None, None);
                true
            }
            Surface::Gpu { cx, p } => {
                let Some(uv) = p.atlas.uv_of(bytes) else {
                    return false;
                };
                if let Some(texture) = p.atlas.texture() {
                    p.picture.draw_vars.set_texture(0, texture);
                }
                let sub_u = uv[0] + part[0] * uv[2];
                let sub_v = uv[1] + part[1] * uv[3];
                let sub_w = part[2] * uv[2];
                let sub_h = part[3] * uv[3];
                p.picture.uv = Vec4 {
                    x: sub_u,
                    y: sub_v,
                    z: sub_w,
                    w: sub_h,
                };
                p.picture.draw_abs(cx, r);
                true
            }
        }
    }

    #[note("canvas-brush-brush-picture-size")]
    pub fn picture_size(&mut self, bytes: &[u8]) -> Option<(usize, usize)> {
        match &mut self.s {
            Surface::Probe(_) => self.probe_atlas.as_mut().and_then(|a| a.size_of(bytes)),
            Surface::Gpu { p, .. } => p.atlas.size_of(bytes),
        }
    }

    #[note("canvas-brush-brush-badge")]
    pub fn badge(&mut self, r: Rect, fill: [f32; 4], border: [f32; 4], radius: f64) {
        if r.size.x < 1.0 || r.size.y < 1.0 {
            return;
        }
        match &mut self.s {
            Surface::Probe(probe) => probe.mark(Layer::Badge, r, fill, Some(border), None, None),
            Surface::Gpu { cx, p } => {
                p.badge.fundo = rgba(fill);
                p.badge.borda = rgba(border);
                p.badge.raio = radius as f32;
                p.badge.espessura = 1.0;
                p.badge.draw_abs(cx, r);
            }
        }
    }

    #[note("canvas-brush-brush-badge-icon")]
    pub fn badge_icon(
        &mut self,
        r: Rect,
        steps: &[crawrad_ir::icon::IconStep],
        ink: [f32; 4],
    ) -> bool {
        let side = r.size.x.min(r.size.y).round();
        if steps.is_empty() || side < 4.0 {
            return false;
        }
        let px = side as u32;
        let coverage = crawrad_ir::icon::rasterize(steps, crawrad_ir::icon::STROKE * 0.5, px);
        match &mut self.s {
            // One mark for the whole icon: the probe measures where the ink
            // lands, not each pixel of it.
            Surface::Probe(probe) => probe.mark(Layer::Badge, r, ink, None, None, None),
            // On the badge layer itself, after the badge box: the same draw
            // call, so the icon sits above the box and below the name.
            Surface::Gpu { cx, p } => {
                p.badge.raio = 0.0;
                p.badge.espessura = 0.0;
                for (row, line) in coverage.chunks(px as usize).enumerate() {
                    let mut col = 0;
                    while col < line.len() {
                        let alpha = crawrad_ir::icon::ink_alpha(line[col]);
                        let start = col;
                        while col < line.len() && crawrad_ir::icon::ink_alpha(line[col]) == alpha {
                            col += 1;
                        }
                        if alpha <= 0.0 {
                            continue;
                        }
                        let c = rgba([ink[0], ink[1], ink[2], ink[3] * alpha]);
                        p.badge.fundo = c;
                        p.badge.borda = c;
                        p.badge.draw_abs(
                            cx,
                            Rect {
                                // Half a pixel past each side: the badge SDF
                                // fades its edge, and a 1px rect drawn exactly
                                // came out at about half its alpha.
                                pos: dvec2(
                                    r.pos.x + start as f64 - 0.5,
                                    r.pos.y + row as f64 - 0.5,
                                ),
                                size: dvec2((col - start) as f64 + 1.0, 2.0),
                            },
                        );
                    }
                }
            }
        }
        true
    }

    #[note("canvas-brush-brush-write-in-badge")]
    pub fn write_in_badge(&mut self, area: Rect, text: &str, c: [f32; 4], centered: bool) {
        if text.is_empty() || area.size.x < 4.0 {
            return;
        }
        let size = self.measure_on(Layer::BadgeText, text);
        let x = if centered {
            area.pos.x + ((area.size.x - size.x) * 0.5).max(0.0)
        } else {
            area.pos.x
        };
        let y = area.pos.y + ((area.size.y - size.y) * 0.5).max(0.0);
        self.ink_text(Layer::BadgeText, dvec2(x, y), size, text, c);
    }

    #[note("canvas-brush-brush-ruler")]
    pub(crate) fn ruler(&mut self, r: Rect, c: [f32; 4]) {
        if r.size.x <= 0.0 || r.size.y <= 0.0 {
            return;
        }
        match &mut self.s {
            Surface::Probe(probe) => probe.mark(Layer::Ruler, r, c, None, None, None),
            Surface::Gpu { cx, p } => {
                p.ruler.cor = rgba(c);
                p.ruler.draw_abs(cx, r);
            }
        }
    }

    #[note("canvas-brush-brush-write-on-ruler")]
    pub(crate) fn write_on_ruler(&mut self, at: DVec2, text: &str, c: [f32; 4]) {
        if text.is_empty() {
            return;
        }
        let size = dvec2(
            text.chars().count() as f64 * PROBE_CHAR_WIDTH,
            PROBE_LINE_HEIGHT,
        );
        self.ink_text(Layer::RulerText, at, size, text, c);
    }

    #[note("canvas-brush-brush-dashed")]
    pub fn dashed(&mut self, r: Rect, c: [f32; 4]) {
        match &mut self.s {
            Surface::Probe(probe) => probe.mark(Layer::Dashed, r, c, None, None, None),
            Surface::Gpu { cx, p } => {
                p.dashed.cor = rgba(c);
                p.dashed.draw_abs(cx, r);
            }
        }
    }

    #[note("canvas-brush-brush-arrow")]
    pub fn arrow(&mut self, r: Rect, direction: DVec2, c: [f32; 4]) {
        let c = self.ink(c);
        if r.size.x < 2.0 || r.size.y < 2.0 {
            return;
        }
        let d = Vec2 {
            x: direction.x as f32,
            y: direction.y as f32,
        };
        match &mut self.s {
            Surface::Probe(probe) => {
                let layer = if self.chrome {
                    Layer::ChromeArrow
                } else {
                    Layer::Arrow
                };
                probe.mark(layer, r, c, None, None, None);
            }
            Surface::Gpu { cx, p } if self.chrome => {
                p.chrome_arrow.cor = rgba(c);
                p.chrome_arrow.direcao = d;
                p.chrome_arrow.draw_abs(cx, r);
            }
            Surface::Gpu { cx, p } => {
                p.arrow.cor = rgba(c);
                p.arrow.direcao = d;
                p.arrow.draw_abs(cx, r);
            }
        }
    }

    #[note("canvas-brush-brush-check")]
    pub fn check(&mut self, r: Rect, c: [f32; 4]) {
        let c = self.ink(c);
        if r.size.x < 3.0 || r.size.y < 3.0 {
            return;
        }
        match &mut self.s {
            Surface::Probe(probe) => probe.mark(Layer::Check, r, c, None, None, None),
            Surface::Gpu { cx, p } => {
                p.check.cor = rgba(c);
                p.check.draw_abs(cx, r);
            }
        }
    }

    #[note("canvas-brush-brush-stroke")]
    pub fn stroke(&mut self, r: Rect, c: [f32; 4]) {
        let c = self.ink(c);
        if r.size.x <= 0.0 || r.size.y <= 0.0 {
            return;
        }
        match &mut self.s {
            Surface::Probe(probe) => probe.mark(Layer::Stroke, r, c, None, None, None),
            Surface::Gpu { cx, p } => {
                p.stroke.cor = rgba(c);
                p.stroke.draw_abs(cx, r);
            }
        }
    }

    #[note("canvas-brush-brush-frame")]
    pub fn frame(&mut self, r: Rect, c: [f32; 4], thickness: f64) {
        for side in frame_sides(r, thickness) {
            self.stroke(side, c);
        }
    }

    #[note("canvas-brush-brush-text-width")]
    fn text_width(&mut self, text: &str) -> f64 {
        self.measure_on(Layer::Text, text).x
    }

    #[note("canvas-brush-brush-measure")]
    pub fn measure(&mut self, text: &str) -> DVec2 {
        self.measure_on(Layer::Text, text)
    }

    #[note("canvas-brush-brush-write")]
    pub fn write(&mut self, area: Rect, text: &str, c: [f32; 4], centered: bool) {
        self.write_into(area, text, c, centered);
    }

    #[note("canvas-brush-brush-write-aligned")]
    pub fn write_aligned(&mut self, area: Rect, text: &str, c: [f32; 4], alignment: &str) {
        if text.is_empty() || area.size.x < 4.0 {
            return;
        }
        let text = prefix_that_fits(text, area.size.x, |t| self.measure(t).x);
        let size = self.measure(text);
        let x = match alignment {
            "taCenter" => area.pos.x + ((area.size.x - size.x) * 0.5).max(0.0),
            "taRightJustify" => area.pos.x + (area.size.x - size.x).max(0.0),
            _ => area.pos.x,
        };
        let y = area.pos.y + ((area.size.y - size.y) * 0.5).max(0.0);
        self.ink_text(Layer::Text, dvec2(x, y), size, text, c);
    }

    #[note("canvas-brush-brush-write-caption")]
    pub fn write_caption(&mut self, area: Rect, text: &str, c: [f32; 4], centered: bool) {
        let limpo = sem_acelerador(text);
        let Some(inked) = self.write_into(area, &limpo, c, centered) else {
            return;
        };
        let Some(i) = indice_do_acelerador(text) else {
            return;
        };
        // The stroke runs from the end of the prefix to the end of the marked
        // character. Both measures come out of the SAME `layout` that drew the
        // text, so they have no way of disagreeing with the ink.
        let end = i + limpo[i..].chars().next().map_or(0, char::len_utf8);
        let before = self.text_width(&limpo[..i]);
        let up_to = self.text_width(&limpo[..end]);
        if up_to <= before {
            return;
        }
        let thickness = self.medida(1.0).max(1.0);
        self.stroke(
            Rect {
                pos: dvec2(inked.pos.x + before, inked.pos.y + inked.size.y - thickness),
                size: dvec2(up_to - before, thickness),
            },
            c,
        );
    }

    #[note("canvas-brush-brush-write-into")]
    fn write_into(&mut self, area: Rect, text: &str, c: [f32; 4], centered: bool) -> Option<Rect> {
        if text.is_empty() || area.size.x < 4.0 {
            return None;
        }
        // The app clips a caption at its box: the designer shows the same.
        let text = prefix_that_fits(text, area.size.x, |t| self.measure(t).x);
        let size = self.measure(text);
        let x = if centered {
            area.pos.x + ((area.size.x - size.x) * 0.5).max(0.0)
        } else {
            area.pos.x
        };
        let y = area.pos.y + ((area.size.y - size.y) * 0.5).max(0.0);
        self.ink_text(Layer::Text, dvec2(x, y), size, text, c);
        Some(Rect {
            pos: dvec2(x, y),
            size,
        })
    }

    #[note("canvas-brush-brush-content")]
    pub fn content(&mut self, area: Rect, ct: &Content) {
        let line_height = self.medida(ct.line_height).max(6.0);
        let fit = lines_that_fit(area.size.y as f32, line_height as f32);
        let indent_per_level = self.medida(15.0).max(4.0);
        let marker = self.medida(12.0).max(4.0);
        for (i, l) in ct.lines.iter().take(fit).enumerate() {
            let top = area.pos.y + line_height * i as f64;
            if ct.selected == Some(i) {
                // On `rounded_box`, and not on `stroke`: the stroke layer is
                // painted AFTER the text one (see the ordering note in the
                // canvas DSL), and the highlight would cover the very item it
                // highlights.
                self.rounded_box(
                    Rect {
                        pos: dvec2(area.pos.x, top),
                        size: dvec2(area.size.x, line_height),
                    },
                    MAKEPAD_SELECTION,
                    MAKEPAD_SELECTION,
                    0.0,
                );
            }
            let indent = indent_per_level * l.level as f64;
            let x = area.pos.x + indent;
            if l.branch {
                self.expand_marker(
                    Rect {
                        pos: dvec2(x, top + (line_height - marker) * 0.5),
                        size: dvec2(marker, marker),
                    },
                    ct.color,
                    l.expanded,
                );
            }
            let line = Rect {
                pos: dvec2(x + marker, top),
                size: dvec2((area.size.x - indent - marker).max(0.0), line_height),
            };
            self.write(line, &l.text, ct.color, false);
        }
    }

    #[note("canvas-brush-brush-expand-marker")]
    fn expand_marker(&mut self, r: Rect, c: [f32; 4], expanded: bool) {
        let frame = shrink(r, (r.size.x * 0.15).max(1.0));
        self.rounded_box(frame, MAKEPAD_WIDGET_INPUT, c, 1.0);
        let stroke = self.medida(1.0).max(1.0);
        let gap = (frame.size.x * 0.25).max(1.0);
        self.stroke(
            Rect {
                pos: dvec2(
                    frame.pos.x + gap,
                    frame.pos.y + (frame.size.y - stroke) * 0.5,
                ),
                size: dvec2((frame.size.x - gap * 2.0).max(1.0), stroke),
            },
            c,
        );
        if !expanded {
            self.stroke(
                Rect {
                    pos: dvec2(
                        frame.pos.x + (frame.size.x - stroke) * 0.5,
                        frame.pos.y + gap,
                    ),
                    size: dvec2(stroke, (frame.size.y - gap * 2.0).max(1.0)),
                },
                c,
            );
        }
    }

    // -- adornments: only the canvas paints these, no component reaches them --

    #[note("canvas-brush-brush-background")]
    pub(crate) fn background(&mut self, r: Rect, c: [f32; 4]) {
        let c = self.ink(c);
        match &mut self.s {
            Surface::Probe(probe) => probe.mark(Layer::Background, r, c, None, None, None),
            Surface::Gpu { cx, p } => {
                p.background.color = rgba(c);
                p.background.draw_abs(cx, r);
            }
        }
    }

    #[note("canvas-brush-brush-paper-frame")]
    pub(crate) fn paper_frame(&mut self, r: Rect, c: [f32; 4], thickness: f64) {
        for side in frame_sides(r, thickness) {
            self.background(side, c);
        }
    }

    #[note("canvas-brush-brush-solid")]
    pub(crate) fn solid(&mut self, r: Rect, c: [f32; 4]) {
        self.stroke(r, c);
    }

    #[note("canvas-brush-brush-grid")]
    pub(crate) fn grid(&mut self, points: &[(f32, f32)], side: f64, strong: bool) {
        let color = if strong {
            MAKEPAD_GRID
        } else {
            MAKEPAD_GRID_FAINT
        };
        let rects = points.iter().map(|(px, py)| {
            let (sx, sy) = self.t.screen_point(*px, *py);
            Rect {
                pos: dvec2(sx as f64, sy as f64),
                size: dvec2(side, side),
            }
        });
        let (cx, p) = match &mut self.s {
            Surface::Probe(probe) => {
                for r in rects {
                    probe.mark(Layer::GridDot, r, color, None, None, None);
                }
                return;
            }
            Surface::Gpu { cx, p } => (cx, p),
        };
        p.grid_dot.cor = rgba(color);
        p.grid_dot.begin_many_instances(cx);
        for (px, py) in points {
            let (sx, sy) = self.t.screen_point(*px, *py);
            p.grid_dot.draw_abs(
                cx,
                Rect {
                    pos: dvec2(sx as f64, sy as f64),
                    size: dvec2(side, side),
                },
            );
        }
        p.grid_dot.end_many_instances(cx);
    }

    #[note("canvas-brush-brush-handles")]
    pub(crate) fn handles(&mut self, points: &[(f32, f32)], side: f64) {
        let rects = points.iter().map(|(px, py)| {
            let (sx, sy) = self.t.screen_point(*px, *py);
            Rect {
                pos: dvec2(sx as f64 - side * 0.5, sy as f64 - side * 0.5),
                size: dvec2(side, side),
            }
        });
        let (cx, p) = match &mut self.s {
            Surface::Probe(probe) => {
                for r in rects {
                    probe.mark(Layer::Handle, r, MAKEPAD_HANDLE, None, None, None);
                }
                return;
            }
            Surface::Gpu { cx, p } => (cx, p),
        };
        p.handle.cor = rgba(MAKEPAD_HANDLE);
        p.handle.begin_many_instances(cx);
        for (ax, ay) in points {
            let (sx, sy) = self.t.screen_point(*ax, *ay);
            p.handle.draw_abs(
                cx,
                Rect {
                    pos: dvec2(sx as f64 - side * 0.5, sy as f64 - side * 0.5),
                    size: dvec2(side, side),
                },
            );
        }
        p.handle.end_many_instances(cx);
    }
}

#[note("canvas-brush-prefix-that-fits")]
pub fn prefix_that_fits(text: &str, max: f64, mut width_of: impl FnMut(&str) -> f64) -> &str {
    if width_of(text) <= max {
        return text;
    }
    let mut end = 0;
    for (i, ch) in text.char_indices() {
        let next = i + ch.len_utf8();
        if width_of(&text[..next]) > max {
            break;
        }
        end = next;
    }
    &text[..end]
}
