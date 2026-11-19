//! # crawrad-runtime-egui — the RXP
//!
//! The second real runtime (ADR 0002),
//! and the design surface outside Windows (ADR 0007).
//!
//! ## What it proves
//!
//! The RCL, removed in ADR 0013, was **retained**: it created real windows.
//! egui is **immediate**: there is no "button" object that persists between frames —
//! everything is redrawn from scratch, always. They are two opposite models, and the question
//! left open by ADR 0002 was whether the [`Runtime`] trait could accommodate both
//! without changing its shape.
//!
//! It did, and the reason is `frame()`: the shell calls it from within egui's
//! own frame, so this runtime has right there exactly what it needs to
//! paint and to sample the pointer. **No trait method changed** for the RXP
//! to exist — and it is this fact, not the drawing, that validates the boundary.
//!
//! ## What it draws
//!
//! The visual language that the RCL brought from a classic RAD IDE: light paper, grid
//! dots, blue handles, amber/magenta/hatched-blue guides, red tab order
//! badge. Two runtimes drawing the *same* document in different
//! ways would be two designers, and the user would feel the switch.
//!
//! ## And it draws outside the canvas too
//!
//! [`pintar_amostra`] paints a document inside any rectangle, without
//! grid, ruler, or handle — this is the **palette tile**
//! (ADR 0020).
//! Per-class drawing lives in a standalone function (`draw_control`) that does
//! not need a live runtime, which is why the Toolbox shows the actual
//! component instead of an icon that could disagree with it.
//!
//! The layering does not change: the shell builds the document via the catalog. This
//! crate continues receiving `Document` and returning painting, and still knows
//! nothing about `crawrad-registry` (ADR 0002/0013).
//!
//! ## What it does not do yet
//!
//! Controls are **painted**, not instantiated: there is no egui widget
//! behind the form button here. At design time this is correct — the RCL
//! did the same with `HTTRANSPARENT`
//! also neutralizes control mouse events with `HTTRANSPARENT`, otherwise clicks
//! went to the button instead of the designer. When `crawrad-codegen` generates
//! an egui application, the generated button will be a real `egui::Button`; what
//! must match is the **appearance**, which is why painting here uses egui's
//! own `Style` instead of made-up colors.

// This crate has no `unsafe`, and the compiler now guarantees it. The CrawRAD
// does not have a SINGLE line of unsafe code: the FFI adapter was removed in ADR 0013 — the rest of the system is safe Rust, and this boundary is as
// important as the one in ADR 0002.
#![forbid(unsafe_code)]

pub mod data_grid;
pub mod dialogs;
pub use dialogs::render_message_dialog;

use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Deref, DerefMut};

use crawrad_ir::{Document, NodeId, Rect, Value};
use crawrad_runtime::{
    GuideKind, GuideLine, Mode, Runtime, RuntimeError, RuntimeEvent, Scale, ViewportPx,
    WindowControlBase,
};
use egui::{Align2, Color32, FontId, Pos2, Rect as ERect, Stroke, StrokeKind, Vec2};

// ------------------------------------------------- single source of drawing
//
// `desenho_comum.rs` is the SAME text that `crawrad-codegen` copies into the
// user's `<form>.designer.rs`. It comes in here via `include!`, not `mod`,
// because `mod` would require each item there to be `pub(crate)` for this
// file to reach it — and that visibility, which only serves our crate
// layout, would travel along into the project of whoever uses CrawRAD. The header of
// `desenho_comum.rs` itself contains the rest of the contract.
//
// What comes from here: `TAB_H`, `TAB_W`, `page_control`, `timer_due`, `timer_wait`.
include!("desenho_comum.rs");

// **This declaration compiles nothing, and is not leftover code.** `cargo fmt` follows
// `mod` and DOES NOT follow `include!` — measured: with the file intentionally corrupted
// and without this line, `cargo fmt --all -- --check` exited with 0. With it, it exits
// with 1 and points to the file. `cfg(any())` is false in every possible
// configuration, so the compiler never sees this module and there is no duplicate item;
// rustfmt sees it, because it formats before `cfg` takes effect.
#[cfg(any())]
mod desenho_comum;

/// The target of this runtime — matches the property prefix from ADR 0002.
pub const ALVO: &str = "egui";

/// The icon size of a non-visual, in design units. Mirrors
/// `crawrad_registry::ICONE`, which this crate cannot import without inverting
/// the dependency (the runtime is below the registry).
const ICONE: f32 = 28.0;

/// The ruler thickness, **in screen points**.
///
/// It is not a design unit, and it is the only canvas measurement that is not: the ruler
/// is not part of the drawing, it measures it. A ruler that shrank with zoom
/// would carry along the number written on it, and the number is the reason it exists.
const REGUA: f32 = 18.0;

// ---------------------------------------------------------------------- view

/// The canvas transformation: origin and scale.
///
/// **This is all of ADR 0019.** Nobody, in this file or outside it, calculates
/// screen coordinates by any other path — the origin disappears inside here, and the zoom
/// factor does too. A caller writes `na_tela(rect)` without knowing that
/// zoom exists, which prevents a second copy of the rule.
///
/// `Copy` on purpose: it is frozen inside the gesture (`EguiRuntime::gesto`),
/// and freezing requires being able to store a copy of the current value.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Transformacao {
    /// Where design coordinate `(0, 0)` lands, in screen points.
    origem: Pos2,
    /// How many screen points one design unit is worth.
    escala: f32,
}

impl Transformacao {
    fn na_tela(&self, r: Rect) -> ERect {
        ERect::from_min_size(
            self.ponto_na_tela(r.x, r.y),
            Vec2::new(self.medida(r.w.max(0.0)), self.medida(r.h.max(0.0))),
        )
    }

    fn ponto_na_tela(&self, x: f32, y: f32) -> Pos2 {
        Pos2::new(
            self.origem.x + x * self.escala,
            self.origem.y + y * self.escala,
        )
    }

    /// Screen point → design unit. The return path, and what the round-trip
    /// test of ADR 0019 measures.
    fn em_design(&self, p: Pos2) -> (f32, f32) {
        let e = self.escala.max(f32::EPSILON);
        ((p.x - self.origem.x) / e, (p.y - self.origem.y) / e)
    }

    /// A design **length** in screen points.
    ///
    /// Exists for numbers carried by the drawing that are not coordinates — the
    /// 4pt text indent inside an `Edit`, the tab strip height, the handle
    /// side. Each of them is a form measurement, and all must grow
    /// along with it; writing raw `4.0` in a painter is the forgotten conversion
    /// described in ADR 0019, but on a length instead of a point.
    fn medida(&self, v: f32) -> f32 {
        v * self.escala
    }

    /// A form text font, at the size requested by zoom.
    ///
    /// egui **relayouts** text at the requested size, so the glyph stays sharp
    /// at any zoom. Scaling the pre-built mesh (which egui's `set_transform_layer`
    /// would do for free) would look blurry — and component captions are
    /// precisely what users read under zoom.
    fn fonte(&self, tam: f32) -> FontId {
        FontId::proportional(self.medida(tam))
    }
}

// ------------------------------------------------------------------- colors
// Inherited from the RCL: both designers were the same designer, and the drawing
// continues to be that.

const SUPERFICIE: Color32 = Color32::from_rgb(0x30, 0x30, 0x30);
const PAPEL: Color32 = Color32::from_rgb(0xF0, 0xF0, 0xF0);
const PONTO_GRADE: Color32 = Color32::from_rgb(0xB0, 0xB0, 0xB0);
/// The container must be VISIBLE over the paper.
///
/// The first version used the theme's `faint_bg_color`, which on the designer's
/// light paper yields almost the form's color: the panel vanished and only handles
/// showed where it was. In a classic RAD IDE a `TPanel` is always visible — the raised
/// bevel is the reason it exists in the palette.
const PAINEL: Color32 = Color32::from_rgb(0xE2, 0xE2, 0xE6);
const PAINEL_BORDA: Color32 = Color32::from_rgb(0x9A, 0x9A, 0xA2);
const ALCA_COMPONENTE: Color32 = Color32::from_rgb(0x00, 0x50, 0xC0);
const ALCA_FORM: Color32 = Color32::from_rgb(0x00, 0x80, 0xE0);
const NAO_VISUAL_FUNDO: Color32 = Color32::from_rgb(0xD0, 0xE0, 0xE8);
const NAO_VISUAL_BORDA: Color32 = Color32::from_rgb(0x40, 0x60, 0x80);
const NAO_VISUAL_TEXTO: Color32 = Color32::from_rgb(0x20, 0x40, 0x60);
const NOME_NAO_VISUAL: Color32 = Color32::from_rgb(0x40, 0x40, 0x40);
const SELO: Color32 = Color32::from_rgb(0xC0, 0x20, 0x20);
const GUIA_COMPONENTE: Color32 = Color32::from_rgb(0xFF, 0x60, 0x00);
const GUIA_TEXTO: Color32 = Color32::from_rgb(0xFF, 0x00, 0xFF);
const GUIA_ESPACO: Color32 = Color32::from_rgb(0x00, 0x80, 0xC0);

// The ruler. It is canvas CHROME, not part of the form: light enough
// for numbers to be read on it, dark enough not to compete with the
// paper — the drawing should draw the eye, not its frame.
const REGUA_FUNDO: Color32 = Color32::from_rgb(0x3C, 0x3C, 0x3C);
const REGUA_TRACO: Color32 = Color32::from_rgb(0x80, 0x80, 0x80);
const REGUA_TEXTO: Color32 = Color32::from_rgb(0xC8, 0xC8, 0xC8);
/// Where the pointer is, marked on both rulers. Red, matching the
/// badge: the color this canvas already uses for "look here".
const REGUA_PONTEIRO: Color32 = Color32::from_rgb(0xC0, 0x20, 0x20);

// ------------------------------------------------ widgets INSIDE the paper
//
// The user's form is drawn with THESE colors, not with the IDE's
// `Visuals`.
//
// Previously it used `ctx.style_of(ctx.theme())` — the **chrome** style. With
// chrome in dark mode, this placed `#E7E7EA` text over `#F0F0F0` paper: **1.08:1
// measured**, meaning the form label disappeared. And the text field came out
// `#101214`, a black box in the middle of a light form — neither the look of the
// IDE, nor the look the generated application will have.
//
// The underlying error was not the shade: it was the **provenance**. The canvas shows
// the user's form and does not participate in any design system (the WYSIWYG
// law); it cannot inherit the shell theme, because its paper is light
// by contract and chrome may be dark. Paper and widget now come from the same
// place — this block — and can no longer disagree.
//
// Shades are for a light form, and contrast is measured against `PAPEL`:
// text 15.25:1, disabled text 4.87:1, and control border 3.74:1 against
// paper — 3.27:1 in the worst case, which is against the button face. All pass the
// WCAG 3:1 floor for the contour identifying a control, and the test
// `o_contorno_diz_onde_o_controle_esta` measures all three.
const CTL_FUNDO: Color32 = Color32::from_rgb(0xFF, 0xFF, 0xFF);
/// A button face at rest.
///
/// It used to be `#E1E1E4`, Windows 10 gray. The screenshot of a classic RAD IDE running on Windows
/// 11 shows an ALMOST WHITE button with a thin border — and that running classic RAD IDE screenshot is
/// the contract for what Preview must show.
const CTL_FACE: Color32 = Color32::from_rgb(0xFD, 0xFD, 0xFD);
/// A control border.
///
/// It used to be `#7A7A80`, and that was the most visible difference between our Preview and
/// a classic RAD IDE running side by side: our border was dark and heavy, Windows 11's
/// is light. A single value for field and button — Windows uses two,
/// with the field base darker, and that nuance is not worth the cost right now.
const CTL_BORDA: Color32 = Color32::from_rgb(0x88, 0x88, 0x88);
const CTL_TEXTO: Color32 = Color32::from_rgb(0x1A, 0x1A, 0x1C);
const CTL_TEXTO_FRACO: Color32 = Color32::from_rgb(0x68, 0x68, 0x6C);
const CTL_SELECAO: Color32 = Color32::from_rgb(0xCC, 0xE0, 0xF5);

/// The `Style` of everything drawn **inside the paper**.
///
/// Derived from [`PAPEL`], not from the IDE theme: as long as paper is light,
/// widgets are those of a light form, whether the IDE is in dark mode or not. That is
/// what keeps WYSIWYG honest — and when paper follows the theme, this follows
/// along, because it comes from the same constant.
fn estilo_do_papel() -> egui::Style {
    let claro = PAPEL.r() >= 0x80;
    let mut v = if claro {
        egui::Visuals::light()
    } else {
        egui::Visuals::dark()
    };
    v.panel_fill = PAPEL;
    v.window_fill = PAPEL;
    v.extreme_bg_color = CTL_FUNDO;
    v.faint_bg_color = CTL_FACE;
    v.selection = egui::style::Selection {
        bg_fill: CTL_SELECAO,
        stroke: Stroke::new(1.0, CTL_TEXTO),
    };
    v.weak_text_color = Some(CTL_TEXTO_FRACO);
    for (w, fundo) in [
        (&mut v.widgets.noninteractive, PAPEL),
        (&mut v.widgets.inactive, CTL_FACE),
        (&mut v.widgets.hovered, CTL_FACE),
        (&mut v.widgets.active, CTL_FACE),
        (&mut v.widgets.open, CTL_FACE),
    ] {
        w.bg_fill = fundo;
        w.weak_bg_fill = fundo;
        w.bg_stroke = Stroke::new(1.0, CTL_BORDA);
        w.fg_stroke = Stroke::new(1.0, CTL_TEXTO);
    }
    // Non-interactive items have no outline: a label with a box around it
    // is what would make the user's form look like something else.
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, CTL_BORDA);
    // Windows 11 corner radius. Design mode already paints buttons with 4.0;
    // Preview uses real widgets and takes the radius from HERE. If the two
    // disagree, the same button changes shape when pressing Preview — and
    // Preview exists precisely so nothing changes.
    for w in [
        &mut v.widgets.noninteractive,
        &mut v.widgets.inactive,
        &mut v.widgets.hovered,
        &mut v.widgets.active,
        &mut v.widgets.open,
    ] {
        w.corner_radius = egui::CornerRadius::same(4);
    }
    egui::Style {
        visuals: v,
        ..Default::default()
    }
}

/// The `egui::Style` the generated application receives, based on the Form's theme.
///
/// In `Mode::Live` (the Preview), the form is drawn with the same style the
/// generated egui application gets, instead of `estilo_do_papel`.
/// `thLight` gives light visuals, `thDark` dark visuals, and `thSystem` queries
/// `ctx.system_theme()`, falling back to dark when `None` (what eframe does).
/// The paper surface in `Mode::Design` remains a drawing board by the owner's decision.
fn app_style_for(ctx: &egui::Context, theme: &str) -> egui::Style {
    let visuals = match theme {
        "thLight" => egui::Visuals::light(),
        "thDark" => egui::Visuals::dark(),
        _ => match ctx.system_theme() {
            Some(egui::Theme::Light) => egui::Visuals::light(),
            _ => egui::Visuals::dark(),
        },
    };
    egui::Style {
        visuals,
        ..Default::default()
    }
}

/// The form's `Theme`; a document without one follows the system.
fn theme_of(root: &crawrad_ir::Node) -> String {
    match root.props.get("theme") {
        Some(Value::Enum(s)) => s.clone(),
        _ => "thSystem".to_string(),
    }
}

// ------------------------------------------------------------------ state

/// A component as the RXP knows it. It mirrors RCL's `Ctl`, minus the
/// `HWND`: here there is no window, only what to draw.
///
/// Hierarchically inherits from [`WindowControlBase`] via [`Deref`],
/// concentrating common geometry, identity, parent, and visibility properties
/// without code rewriting.
#[derive(Debug, Clone)]
struct Ctl {
    base: WindowControlBase,
    /// The `ActivePageIndex` of a `PageControl`. Zero for everything else.
    pagina_ativa: usize,
    /// `Some(c)` hides Edit contents behind character `c`.
    ///
    /// egui draws its bullet and ignores WHICH character it is. The document
    /// keeps the user's character anyway — it is user information, and a future
    /// target can honor it.
    senha: Option<char>,
    /// The `ReadOnly` of a classic RAD IDE: shows and allows selection, does not allow typing.
    somente_leitura: bool,
    /// The tree collection, for controls that have one. Comes from the document.
    itens: Vec<crawrad_ir::Item>,
    /// The chosen item of the flat collection, or `None` for none. It is the
    /// `ItemIndex` of a classic RAD IDE, where `-1` is *none*.
    escolhido: Option<usize>,
    /// The `Checked` of `CheckBox` and `RadioButton`.
    marcado: bool,
    /// The `Interval` of `Timer`, in milliseconds. Zero for everything else — and,
    /// for `Timer` itself, zero means *stopped*, as in a classic RAD IDE.
    intervalo: i64,
    /// The `Columns` of a `RadioGroup`; 1 for the rest.
    columns: i64,
    /// The `Value` of a `SpinEdit`; 0 for the rest.
    spin: i64,
    /// The `Selected` of a `ColorBox`; black for the rest.
    selected: crawrad_ir::Color,
    /// The `URL` of a `LinkLabel`; empty for the rest.
    url: String,
    /// The frame thickness of `Panel` — the `BorderWidth` of `TPanel`. Zero
    /// draws no contour, as in a classic RAD IDE.
    borda: f32,
    /// The `ScrollBars` of `Memo`: "ssNone", "ssHorizontal", "ssVertical", "ssBoth".
    scroll_bars: String,
    /// The `Position` of `ScrollBar`.
    posicao_scroll: f32,
    /// The image of `Image` (`Picture`).
    picture: Option<crawrad_ir::Picture>,
    col_count: i64,
    row_count: i64,
    fixed_cols: i64,
    fixed_rows: i64,
    default_col_width: f32,
    default_row_height: f32,
    grid_lines: bool,
    shape_kind: String,
    beveled: bool,
    bevel_shape: String,
    bevel_style: String,
    collapsed: bool,
    columns_info: Vec<(String, i64, String)>,
    show_column_headers: bool,
    row_select: bool,
    subtitle: String,
    corner_radius: f32,
    elevation: i64,
    show_header: bool,
    kind: String,
    outline: bool,
    count: i64,
    max_count: i64,
    associate: String,
    removable: bool,
    separator: String,
    style: String,
    segment_color: Color32,
    current_color: Color32,
    page_count: i64,
    page_index: i64,
    max_buttons: i64,
    show_arrows: bool,
    side: String,
    opened: bool,
    overlay: bool,
    selected_items: Vec<String>,
    checked_items: Vec<String>,
    text_hint: String,
    /// A `MaskEdit`'s `EditMask`; empty for every other class.
    edit_mask: String,
    date_time: String,
    min_date: String,
    max_date: String,
}

impl Deref for Ctl {
    type Target = WindowControlBase;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for Ctl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// The egui runtime.
pub struct EguiRuntime {
    ctx: egui::Context,
    ctls: BTreeMap<NodeId, Ctl>,
    /// Everything the document has, in paint order — WITHOUT the screen filter.
    ///
    /// `ordem` is what is painted and what is clicked. This is what is KNOWN, and
    /// the two are genuinely different: the tab of a closed page must
    /// appear (it is used to open the page), but its content must not. With
    /// a single list, filtering content would take the tab with it and `PageControl`
    /// would have no way to switch back.
    ordem_toda: Vec<NodeId>,
    /// Z-order: painted first is furthest back, as in `SetWindowPos`.
    ordem: Vec<NodeId>,
    form_rect: Rect,
    form_id: NodeId,
    form_theme: String,
    sel: Vec<NodeId>,
    viewport: ViewportPx,
    scale: Scale,
    grade: f32,
    marquee: Option<Rect>,
    guias: Vec<GuideLine>,
    selos: Vec<(NodeId, String)>,
    eventos: Vec<RuntimeEvent>,
    /// Gesture in progress — the equivalent of RCL's `SetCapture`.
    capturando: bool,
    /// The **frozen** transformation while a gesture is in progress.
    ///
    /// Screen→form conversion subtracts the surface origin, and that
    /// origin changes when a panel opens, closes, or resizes. If it
    /// changes IN THE MIDDLE of a drag, gesture `delta` absorbs the entire
    /// displacement: the component jumps the distance the surface moved.
    ///
    /// This actually happened — opening the Object Inspector (320px) instead of the
    /// Components panel (190px) pushed the surface to the right, and the
    /// component being dragged ended up at `x = -116`, outside the
    /// form, under the inspector.
    ///
    /// **Freezes scale as well, not just origin** (ADR 0019): zooming in the
    /// middle of a drag is the same bug under another name. With the factor changing
    /// beneath the gesture, the same screen pixel represents a different design
    /// distance, and `delta` swallows the difference — the component jumps, and jumps
    /// more the larger the zoom jump is.
    ///
    /// Freezing is the same idea as `SetCapture` below, carried to its conclusion:
    /// a captured gesture owns the interaction **and reference frame** until released. A
    /// gesture is a continuous act, and the ground cannot shift beneath it.
    gesto: Option<Transformacao>,
    /// Canvas zoom — how many screen points one design unit is worth.
    ///
    /// IDE state, and nothing more: does not enter `.crawform` or generated
    /// code, and `open`/`update` never touch it. The shell moves it via trait
    /// `set_zoom`.
    ///
    /// **Do not confuse with egui's `zoom_factor`**, which scales the entire IDE
    /// (the `Ctrl +` / `Ctrl -` shown in Settings). That is about seeing
    /// chrome; this is about seeing the form, and the two multiply
    /// without interference — the `Scale` arriving in `set_viewport` already
    /// carries the former.
    zoom: f32,
    modo: Mode,
    /// What the user typed while the form was live. Discarded when
    /// returning to design: live mode DOES NOT edit the document, as in
    /// a classic RAD IDE — running a form does not modify the `.dfm`.
    texto_vivo: BTreeMap<NodeId, String>,
    /// The selected item in each live tree, by index path.
    selecao_viva: BTreeMap<NodeId, Vec<usize>>,
    /// Collapsed nodes in each live tree. Absent = expanded, which is
    /// how the sample starts.
    fechados_vivos: BTreeMap<NodeId, BTreeSet<Vec<usize>>>,
    /// The `Checked` of each checkbox and radio in the live form.
    marcados_vivos: BTreeMap<NodeId, bool>,
    /// The open page of each `PageControl` in the live form.
    ///
    /// **Here, not in the document — and that is the decision of this fix.** Switching
    /// tabs in Preview is *using* the application, exactly like typing into an
    /// `Edit`: the `ActivePageIndex` in `.crawform` remains what the
    /// designer chose, and returning to design shows its page. If clicking
    /// wrote to the document, running the form would modify the file — the
    /// invariant guarded by test `o_modo_vivo_semeia_os_campos_e_nao_toca_no_documento`,
    /// and the same reason typed text lives in `texto_vivo`.
    ///
    /// Starts seeded from the document in `set_modo`, like the others: Preview
    /// begins on the page the designer left open.
    paginas_vivas: BTreeMap<NodeId, usize>,
    /// The deadline of each live `Timer`, on egui's clock, in seconds.
    ///
    /// `None` means *has not started counting yet* — the first frame in which the
    /// timer appears marks the starting point instead of firing immediately, which
    /// is what Windows `SetTimer` does.
    prazos_vivos: BTreeMap<NodeId, Option<f64>>,
    /// How many times each `Timer` fired since Preview began.
    ///
    /// In Preview there is no user code — its handler lives in the
    /// generated project, not in the IDE. Without a count, a timer in Preview would
    /// be indistinguishable from a broken timer, and the only way to check would
    /// be generating the project. See [`EguiRuntime::tiques`].
    tiques_vivos: BTreeMap<NodeId, u64>,
    aberto: bool,
    /// Class → its palette icon, handed in by the shell; the non-visual badge
    /// wears it and falls back to the initials when the class has none.
    class_icons: BTreeMap<String, Vec<crawrad_ir::icon::IconStep>>,
}

impl EguiRuntime {
    /// egui's context is cheap to clone (internally an `Arc`) and is the
    /// analogue of the `HWND` that RCL receives: the identity of the surface.
    pub fn new(ctx: egui::Context) -> Self {
        Self {
            ctx,
            ctls: BTreeMap::new(),
            ordem_toda: Vec::new(),
            ordem: Vec::new(),
            form_rect: Rect::new(0.0, 0.0, 640.0, 420.0),
            form_id: String::new(),
            form_theme: "thSystem".to_string(),
            sel: Vec::new(),
            viewport: ViewportPx {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
            },
            scale: Scale(1.0),
            grade: 8.0,
            marquee: None,
            guias: Vec::new(),
            selos: Vec::new(),
            eventos: Vec::new(),
            capturando: false,
            gesto: None,
            zoom: crawrad_runtime::ZOOM_ONE,
            modo: Mode::Design,
            texto_vivo: BTreeMap::new(),
            selecao_viva: BTreeMap::new(),
            fechados_vivos: BTreeMap::new(),
            marcados_vivos: BTreeMap::new(),
            paginas_vivas: BTreeMap::new(),
            prazos_vivos: BTreeMap::new(),
            tiques_vivos: BTreeMap::new(),
            aberto: false,
            class_icons: BTreeMap::new(),
        }
    }

    #[crawrad_note::note("egui-runtime-set-class-icons")]
    pub fn set_class_icons(&mut self, icons: BTreeMap<String, Vec<crawrad_ir::icon::IconStep>>) {
        self.class_icons = icons;
    }

    /// How many times this `Timer` fired since Preview began.
    ///
    /// This is what makes the timer **verifiable without generating the project**: in Preview
    /// there is no user handler to produce an on-screen effect, so the only proof
    /// that it is running is the count. Resets to zero on exiting Preview,
    /// like all other live state.
    pub fn tiques(&self, id: &NodeId) -> u64 {
        self.tiques_vivos.get(id).copied().unwrap_or(0)
    }

    /// The surface in egui points — the rectangle handed over by the shell.
    ///
    /// The viewport arrives in device pixels; `Scale` is the same
    /// `pixels_per_point`, so dividing by it yields points.
    ///
    /// **It is not the transformation, and no one outside here calls it.** This is what
    /// ADR 0019
    /// calls for: a loose `superficie()` in coordinate calculation is a bug that
    /// sleeps until someone zooms. The only two readers are
    /// [`EguiRuntime::area_de_desenho`] right below and
    /// [`EguiRuntime::transformacao`] after it.
    fn superficie(&self) -> ERect {
        let p = self.scale.0.max(0.01);
        ERect::from_min_size(
            Pos2::new(self.viewport.x as f32 / p, self.viewport.y as f32 / p),
            Vec2::new(self.viewport.w as f32 / p, self.viewport.h as f32 / p),
        )
    }

    /// The portion of the surface where the artboard is drawn — the surface minus
    /// the rulers.
    ///
    /// It is what counts for **storing**, **clipping**, and **deciding if the gesture belongs
    /// to the designer**. Using the whole surface for these three points places the
    /// form under the ruler and causes a click on the ruler to start dragging.
    ///
    /// In live mode there is no ruler: Preview shows the application, and the
    /// application has no artboard.
    fn area_de_desenho(&self) -> ERect {
        let sup = self.superficie();
        if self.modo.is_live() {
            return sup;
        }
        // `intersect` with the surface itself: in too narrow a window,
        // `shrink` would produce an inverted rectangle, and an inverted rectangle
        // passes unnoticed in every `contains` — clicks would cease to exist
        // instead of the ruler ceasing to fit.
        ERect::from_min_max(sup.min + Vec2::splat(REGUA), sup.max).intersect(sup)
    }

    /// **The** canvas transformation: where the form origin lands on screen,
    /// and how many points one design unit is worth.
    ///
    /// A single function, which is the point of ADR 0019: what paints and what hits clicks
    /// reads from here, not from `superficie()`.
    ///
    /// **Live mode has neither zoom nor ruler.** In Preview what is on screen is the
    /// application, and magnifying the application would lie twice: the widgets
    /// belong to egui and their font would **not** track the rectangle — a button
    /// twice the size with the caption at normal size. Preview at 100%
    /// measures what the user will see, which is the only thing it is meant to
    /// measure.
    fn transformacao(&self) -> Transformacao {
        if self.modo.is_live() {
            return Transformacao {
                origem: self.superficie().min,
                escala: crawrad_runtime::ZOOM_ONE,
            };
        }
        Transformacao {
            origem: self.area_de_desenho().min,
            escala: self.zoom,
        }
    }

    /// The **frozen** transformation of the in-progress gesture, or the current one.
    ///
    /// See [`EguiRuntime::gesto`]: a captured gesture owns the reference frame until
    /// released, and the reference frame includes the scale.
    fn transformacao_do_gesto(&self) -> Transformacao {
        self.gesto.unwrap_or_else(|| self.transformacao())
    }

    /// Design unit → screen point.
    fn na_tela(&self, r: Rect) -> ERect {
        self.transformacao().na_tela(r)
    }

    fn ponto_na_tela(&self, x: f32, y: f32) -> Pos2 {
        self.transformacao().ponto_na_tela(x, y)
    }

    /// Screen point → design unit. This is the return path, through which
    /// the user's gesture reaches the designer already converted.
    fn em_design(&self, p: Pos2) -> (f32, f32) {
        self.transformacao_do_gesto().em_design(p)
    }

    fn ler_no(&mut self, id: &NodeId, n: &crawrad_ir::Node, doc: &Document) {
        self.ctls.insert(id.clone(), ctl_do_no(id, n, doc));
    }
}

/// A [`Ctl`] from a document node.
///
/// Standalone function because the **palette** needs it without an open runtime: the
/// tile reads component properties through the same path the canvas
/// reads, not through a second read that might disagree with this one.
fn ctl_do_no(id: &NodeId, n: &crawrad_ir::Node, doc: &Document) -> Ctl {
    // The SAME answer as `no_canvas`, not a second duplicate list: the
    // two were once copies, and list copies are the defect category that
    // cost the most here.
    let visual = e_visual_no_canvas(&n.class);

    // Extracts identity, geometry, parent, visibility, and tab_order properties
    // through the standardized hierarchy in `WindowControlBase`.
    let base = WindowControlBase::from_doc(id, n, doc, visual);

    let booleano = |no: &crawrad_ir::Node, k: &str, padrao: bool| match no.props.get(k) {
        Some(Value::Bool(b)) => *b,
        _ => padrao,
    };

    // `props_for` is the contract of ADR 0002 in action: what belongs to another target
    // does not even reach here, and what is common reaches both.
    // From COMMON CORE since ADR 0013. Previously the registry declared
    // `win32.PasswordChar` and this code searched for `egui.PasswordChar`: the
    // property existed on both sides and worked on neither here.
    let senha = n
        .props_for(ALVO)
        .find(|(k, _)| k.to_string() == "password_char")
        .and_then(|(_, v)| match v {
            Value::Str(s) => s.chars().next(),
            _ => None,
        });

    Ctl {
        base,
        pagina_ativa: match n.props.get("active_page") {
            Some(Value::Int(i)) if *i >= 0 => *i as usize,
            _ => 0,
        },
        senha,
        somente_leitura: booleano(n, "readonly", false),
        itens: match n.props.get("items") {
            Some(v) => crawrad_ir::Item::list_from(v),
            None => Vec::new(),
        },
        // `-1` from the document means *none*, as in a classic RAD IDE.
        escolhido: match n.props.get("item_index") {
            Some(Value::Int(i)) if *i >= 0 => Some(*i as usize),
            _ => None,
        },
        marcado: booleano(n, "checked", false),
        intervalo: match n.props.get("interval") {
            Some(Value::Int(i)) => *i,
            _ => 0,
        },
        columns: match n.props.get("columns") {
            Some(Value::Int(c)) => *c,
            _ => 1,
        },
        spin: match n.props.get("value") {
            Some(Value::Int(v)) => *v,
            _ => 0,
        },
        url: match n.props.get("url") {
            Some(Value::Str(s)) => s.clone(),
            _ => String::new(),
        },
        selected: match n.props.get("selected") {
            Some(Value::Color(k)) => *k,
            _ => crawrad_ir::Color::STANDARD[0].1,
        },
        borda: match n
            .props
            .get("border_width")
            .or_else(|| n.props.get("pen_width"))
        {
            Some(Value::Int(i)) => (*i).max(0) as f32,
            _ => 1.0,
        },
        scroll_bars: match n.props.get("scroll_bars") {
            Some(Value::Enum(s)) => s.clone(),
            Some(Value::Str(s)) => s.clone(),
            _ => "ssNone".to_string(),
        },
        posicao_scroll: match n.props.get("position") {
            Some(Value::Int(i)) => *i as f32,
            _ => 0.0,
        },
        picture: match n.props.get("picture") {
            Some(Value::Picture(p)) => Some(p.clone()),
            _ => None,
        },
        col_count: match n.props.get("col_count") {
            Some(Value::Int(i)) => (*i).max(0),
            _ => 5,
        },
        row_count: match n.props.get("row_count") {
            Some(Value::Int(i)) => (*i).max(0),
            _ => 5,
        },
        fixed_cols: match n.props.get("fixed_cols") {
            Some(Value::Int(i)) => (*i).max(0),
            _ => 0,
        },
        fixed_rows: match n.props.get("fixed_rows") {
            Some(Value::Int(i)) => (*i).max(0),
            _ => 1,
        },
        default_col_width: match n.props.get("default_col_width") {
            Some(Value::Int(i)) => (*i).max(10) as f32,
            _ => 80.0,
        },
        default_row_height: match n.props.get("default_row_height") {
            Some(Value::Int(i)) => (*i).max(10) as f32,
            _ => 24.0,
        },
        grid_lines: booleano(n, "grid_lines", true),
        shape_kind: match n.props.get("shape").or_else(|| n.props.get("shape_kind")) {
            Some(Value::Str(s)) => s.clone(),
            Some(Value::Enum(s)) => s.clone(),
            _ => "Rectangle".to_string(),
        },
        beveled: booleano(n, "beveled", true),
        bevel_shape: enum_text(n, "bevel_shape", "Box"),
        bevel_style: enum_text(n, "bevel_style", "Lowered"),
        collapsed: booleano(n, "collapsed", false),
        columns_info: n
            .children
            .iter()
            .filter_map(|cid| doc.node(cid))
            .filter(|cn| cn.class == "ListColumn")
            .map(|cn| {
                let text = match cn.props.get("text") {
                    Some(Value::Str(s)) => s.clone(),
                    _ => String::new(),
                };
                let width = match cn.props.get("width") {
                    Some(Value::Int(w)) => *w,
                    _ => 80,
                };
                let alignment = enum_text(cn, "alignment", "taLeftJustify");
                (text, width, alignment)
            })
            .collect(),
        show_column_headers: booleano(n, "show_column_headers", true),
        row_select: booleano(n, "row_select", true),
        subtitle: match n.props.get("subtitle") {
            Some(Value::Str(s)) => s.clone(),
            _ => String::new(),
        },
        corner_radius: match n.props.get("corner_radius") {
            Some(Value::Int(i)) => *i as f32,
            _ => 8.0,
        },
        elevation: match n.props.get("elevation") {
            Some(Value::Int(i)) => *i,
            _ => 1,
        },
        show_header: booleano(n, "show_header", true),
        kind: enum_text(n, "kind", "bkPrimary"),
        outline: booleano(n, "outline", false),
        count: match n.props.get("count") {
            Some(Value::Int(i)) => *i,
            _ => 0,
        },
        max_count: match n.props.get("max_count") {
            Some(Value::Int(i)) => *i,
            _ => 99,
        },
        associate: match n.props.get("associate") {
            Some(Value::Ref(s)) | Some(Value::Str(s)) => s.clone(),
            _ => String::new(),
        },
        removable: booleano(n, "removable", false),
        separator: enum_text(n, "separator", "›"),
        style: enum_text(n, "style", "bsText"),
        segment_color: match n.props.get("segment_color") {
            Some(Value::Color(c)) => Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a),
            _ => Color32::from_rgb(0x45, 0x46, 0x4B),
        },
        current_color: match n.props.get("current_color") {
            Some(Value::Color(c)) => Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a),
            _ => Color32::from_rgb(0xE0, 0x61, 0x2B),
        },
        page_count: match n.props.get("page_count") {
            Some(Value::Int(i)) => *i,
            _ => 10,
        },
        page_index: match n.props.get("page_index") {
            Some(Value::Int(i)) => *i,
            _ => 0,
        },
        max_buttons: match n.props.get("max_buttons") {
            Some(Value::Int(i)) => *i,
            _ => 7,
        },
        show_arrows: booleano(n, "show_arrows", true),
        side: enum_text(n, "side", "dsLeft"),
        opened: booleano(n, "opened", false),
        overlay: booleano(n, "overlay", true),
        selected_items: match n.props.get("selected_items") {
            Some(v) => crawrad_ir::Item::list_from(v)
                .into_iter()
                .map(|it| it.text)
                .collect(),
            None => Vec::new(),
        },
        checked_items: match n.props.get("checked_items") {
            Some(v) => crawrad_ir::Item::list_from(v)
                .into_iter()
                .map(|it| it.text)
                .collect(),
            None => Vec::new(),
        },
        text_hint: enum_text(n, "text_hint", "Select…"),
        edit_mask: enum_text(n, "edit_mask", ""),
        date_time: enum_text(n, "date_time", ""),
        min_date: enum_text(n, "min_date", ""),
        max_date: enum_text(n, "max_date", ""),
    }
}

/// The text of an enum (or string) property, or `default` when unset.
fn enum_text(n: &crawrad_ir::Node, key: &str, default: &str) -> String {
    match n.props.get(key) {
        Some(Value::Str(s)) | Some(Value::Enum(s)) => s.clone(),
        _ => default.to_string(),
    }
}

impl Runtime for EguiRuntime {
    fn target(&self) -> &'static str {
        ALVO
    }

    fn classes(&self) -> &'static [&'static str] {
        CLASSES
    }

    fn open(&mut self, doc: &Document) -> Result<(), RuntimeError> {
        let raiz = doc
            .node(&doc.root)
            .ok_or_else(|| RuntimeError::Backend("the document has no root".into()))?;
        self.form_rect = raiz.rect().unwrap_or(Rect::new(0.0, 0.0, 640.0, 420.0));
        self.form_id = doc.root.clone();
        self.form_theme = theme_of(raiz);
        self.ctls.clear();
        self.ordem.clear();
        self.ordem_toda.clear();

        // The ENTIRE tree, parent before children — not just root children.
        // Minus what lives INSIDE a non-visual: see `no_canvas`.
        // Order is paint order, and within it Z-order is per parent
        // (ADR 0012).
        for id in doc.in_paint_order() {
            let Some(n) = doc.node(&id) else { continue };
            self.ler_no(&id, n, doc);
            self.ordem_toda.push(id.clone());
            if no_canvas(doc, &id) {
                self.ordem.push(id);
            }
        }
        self.ordem.sort_by_key(|id| {
            if self
                .ctls
                .get(id)
                .is_some_and(|c| c.class == "Badge" && !c.associate.is_empty())
            {
                1
            } else {
                0
            }
        });
        self.aberto = true;
        self.ctx.request_repaint();
        Ok(())
    }

    fn update(&mut self, doc: &Document, touched: &BTreeSet<NodeId>) -> Result<(), RuntimeError> {
        if !self.aberto {
            return Err(RuntimeError::NotStarted);
        }
        // what vanished from the document leaves the screen
        self.ordem.retain(|id| doc.node(id).is_some());
        self.ordem_toda.retain(|id| doc.node(id).is_some());
        self.ctls.retain(|id, _| doc.node(id).is_some());

        let raiz = doc
            .node(&doc.root)
            .ok_or_else(|| RuntimeError::Backend("the document has no root".into()))?;
        if touched.contains(&doc.root) {
            self.form_theme = theme_of(raiz);
        }
        if touched.contains(&doc.root)
            && let Some(r) = raiz.rect()
        {
            self.form_rect = r;
        }

        // The ENTIRE TREE, as in `open` — not just root children.
        //
        // This was once wrong: `open` descended the tree when containers
        // arrived and `update` kept reading only the root. The result is the worst kind
        // of defect — the component EXISTS in the document and structure tree,
        // yet does not appear on screen, because `self.ordem` was rebuilt on each
        // transaction discarding grandchildren.
        let ordem_toda: Vec<NodeId> = doc.in_paint_order();
        let mut ordem: Vec<NodeId> = ordem_toda
            .iter()
            .filter(|id| no_canvas(doc, id))
            .cloned()
            .collect();
        for id in &ordem_toda {
            let Some(n) = doc.node(id) else { continue };
            // Parent moving moves the child: when the rectangle is relative, moving the
            // panel changes the screen position of everything inside it.
            let pai_mexeu = doc.ancestors(id).iter().any(|a| touched.contains(a));
            if touched.contains(id) || pai_mexeu || !self.ctls.contains_key(id) {
                self.ler_no(id, n, doc);
            }
        }
        ordem.sort_by_key(|id| {
            if self
                .ctls
                .get(id)
                .is_some_and(|c| c.class == "Badge" && !c.associate.is_empty())
            {
                1
            } else {
                0
            }
        });
        self.ordem = ordem;
        self.ordem_toda = ordem_toda;
        self.ctx.request_repaint();
        Ok(())
    }

    fn set_viewport(&mut self, vp: ViewportPx, scale: Scale) {
        self.viewport = vp;
        self.scale = scale;
    }

    fn set_selection(&mut self, sel: &[NodeId]) {
        self.sel = sel.to_vec();
    }

    fn set_marquee(&mut self, rect: Option<Rect>) {
        self.marquee = rect;
    }

    fn set_guides(&mut self, guias: &[GuideLine]) {
        self.guias = guias.to_vec();
    }

    fn set_badges(&mut self, badges: &[(NodeId, String)]) {
        self.selos = badges.to_vec();
    }

    fn set_grid(&mut self, grid: f32) {
        self.grade = grid;
    }

    fn set_zoom(&mut self, zoom: f32) {
        // Bound to the ladder, not to what arrived: the caller may be an MCP
        // agent (ADR 0005), and a `0.0` from there would divide by zero on the
        // return conversion — the entire form would collapse to a point and
        // no clicks would find anything anymore. The boundary lives in what applies it,
        // not in what requests it.
        //
        // `is_finite` comes BEFORE `clamp`, and it is not an excess of caution: Rust's `clamp`
        // **returns NaN when given NaN** — it compares, and every comparison
        // with NaN is false. A NaN scale causes `na_tela` to produce a NaN rectangle
        // (egui paints nothing) and `em_design` to return NaN: the canvas disappears
        // and no click finds anything anymore. A standalone clamp looked like
        // a boundary and let through precisely the value that causes the most damage.
        //
        // Invalid values move nothing, instead of falling back to minimum: whoever sent
        // garbage did not choose 25%, and reframing the canvas because of that would
        // obey a command nobody gave.
        if !zoom.is_finite() {
            return;
        }
        let menor = crawrad_runtime::ZOOMS[0];
        let maior = crawrad_runtime::ZOOMS[crawrad_runtime::ZOOMS.len() - 1];
        self.zoom = zoom.clamp(menor, maior);
    }

    fn set_mode(&mut self, modo: Mode) {
        if self.modo == modo {
            return;
        }
        self.modo = modo;
        // Entering live mode seeds fields with what the document specifies;
        // exiting discards what was typed.
        self.texto_vivo.clear();
        self.selecao_viva.clear();
        self.fechados_vivos.clear();
        self.marcados_vivos.clear();
        self.paginas_vivas.clear();
        self.prazos_vivos.clear();
        self.tiques_vivos.clear();
        if modo.is_live() {
            for (id, c) in &self.ctls {
                // The deadline starts as `None`: the first frame fills it,
                // because only then do we know what time it is. Seeding with a number
                // here would require reading the clock outside the frame — and egui's
                // clock only exists inside it.
                if c.class == "Timer" {
                    self.prazos_vivos.insert(id.clone(), None);
                }
                // a MaskEdit opens on its masked text (no mask: as is)
                if matches!(c.class.as_str(), "Edit" | "Memo" | "MaskEdit") {
                    let shown = crawrad_runtime::mask::shown_text(&c.edit_mask, &c.text);
                    self.texto_vivo.insert(id.clone(), shown);
                }
                // DateTimePicker opens on its formatted date or time
                if c.class == "DateTimePicker" {
                    let kind = crawrad_runtime::date_time::DateTimeKind::parse_kind(&c.kind);
                    let shown = crawrad_runtime::date_time::iso_to_shown(kind, &c.date_time);
                    self.texto_vivo.insert(id.clone(), shown);
                }
                if matches!(c.class.as_str(), "CheckBox" | "RadioButton") {
                    self.marcados_vivos.insert(id.clone(), c.marcado);
                }
                // a CategoryPanel starts folded as the designer left it
                if c.class == "CategoryPanel" {
                    self.marcados_vivos.insert(id.clone(), c.collapsed);
                }
                // Preview's open tab starts from the one the designer left
                // open. Starting at zero would make pressing Preview switch
                // pages on its own, which is the same kind of lie as a text
                // field starting empty.
                if c.class == "PageControl" {
                    self.paginas_vivas.insert(id.clone(), c.pagina_ativa);
                }
                if matches!(c.class.as_str(), "ComboBox" | "ListBox")
                    && let Some(i) = c.escolhido
                {
                    self.selecao_viva.insert(id.clone(), vec![i]);
                }
            }
        }
        self.capturando = false;
        self.gesto = None;
        self.ctx.request_repaint();
    }

    fn busy(&self) -> bool {
        self.capturando
    }

    /// The heart of the immediate runtime: capture the gesture and paint, within the frame
    /// the shell already opened.
    fn frame(&mut self) {
        if !self.aberto {
            return;
        }
        if self.modo.is_live() {
            // In live mode the gesture belongs to the form user, not the
            // designer: no events bubble up.
            self.viver();
        } else {
            self.colher_entrada();
            self.pintar();
        }
    }

    fn drain_events(&mut self) -> Vec<RuntimeEvent> {
        std::mem::take(&mut self.eventos)
    }

    fn close(&mut self) {
        self.ctls.clear();
        self.ordem.clear();
        self.ordem_toda.clear();
        self.sel.clear();
        self.eventos.clear();
        self.capturando = false;
        self.gesto = None;
        self.aberto = false;
        self.form_theme = "thSystem".to_string();
    }
}

// ------------------------------------------------------------------ input

impl EguiRuntime {
    /// Translates the egui pointer into [`RuntimeEvent`] **already in design
    /// units** — the runtime translates, the designer decides.
    fn colher_entrada(&mut self) {
        // The DRAWING area, not the surface: this is where user gestures
        // arrive, and the ruler strip is not artboard. With the full surface,
        // a click on the ruler started marquee selection from a negative design
        // coordinate — dragging over nothing, starting from a place where there is no
        // form.
        let area = self.area_de_desenho();
        let (pos, desceu, subiu, duplo, ctrl, esc) = self.ctx.input(|i| {
            (
                i.pointer.interact_pos(),
                i.pointer.primary_pressed(),
                i.pointer.primary_released(),
                i.pointer
                    .button_double_clicked(egui::PointerButton::Primary),
                i.modifiers.ctrl,
                i.key_pressed(egui::Key::Escape),
            )
        });

        let Some(p) = pos else {
            // Pointer outside window: an in-progress drag cannot get stuck.
            if self.capturando {
                self.capturando = false;
                self.gesto = None;
                self.eventos
                    .push(RuntimeEvent::PointerUp { x: 0.0, y: 0.0 });
            }
            return;
        };
        let (x, y) = self.em_design(p);
        let dentro = area.contains(p);

        if esc && self.capturando {
            self.capturando = false;
            self.gesto = None;
            self.eventos.push(RuntimeEvent::Cancel);
            return;
        }

        // Only start a gesture inside the drawing area — clicks on the toolbar
        // and ruler do not belong to the designer. Once started, the
        // gesture continues even if the pointer leaves: RCL's `SetCapture`.
        if desceu && dentro {
            self.capturando = true;
            self.gesto = Some(self.transformacao());
            self.eventos.push(RuntimeEvent::PointerDown { x, y, ctrl });
            return;
        }
        if self.capturando {
            if subiu {
                self.capturando = false;
                self.gesto = None;
                self.eventos.push(RuntimeEvent::PointerUp { x, y });
                // egui only recognizes double click when the button **releases**:
                // the second `Released` closes the pair. Win32 notifies on
                // press, with `WM_LBUTTONDBLCLK`. The difference is toolkit-specific, and
                // the runtime absorbs it — the designer receives the same
                // `PointerDblClick` from both, and knows nothing about this.
                if duplo {
                    self.eventos.push(RuntimeEvent::PointerDblClick { x, y });
                }
            } else {
                self.eventos
                    .push(RuntimeEvent::PointerMove { x, y, free: false });
            }
        }
    }
}

// -------------------------------------------------------------- live mode

impl EguiRuntime {
    /// Constructs the form with **real egui widgets**.
    ///
    /// This is where text input, focus, selection, and IME appear — and not
    /// a single line of them is ours: they belong to `egui::TextEdit`. Per §9.3 of the study,
    /// CrawRAD borrows layers 1 to 6 and writes layer 7; this function is the
    /// proof that borrowing works.
    ///
    /// And it is half of the RXP target codegen: if IR can be **interpreted** into
    /// widgets, emitting the equivalent Rust is mechanical.
    fn viver(&mut self) {
        self.rodar_timers();
        let form = self.na_tela(self.form_rect);
        // `area_de_desenho`, not `superficie`: in live mode both are the
        // SAME thing (Preview has no ruler, see `transformacao`), so
        // behavior does not change here. What changes is that this file gets
        // a single path to the usable rectangle — leaving a loose `superficie()`
        // here would force the next reader to rediscover that live mode is the
        // rulerless case, and when Preview gains one the ruler would cover
        // the form without anyone touching this line.
        let area = self.area_de_desenho();
        if area.width() < 1.0 || area.height() < 1.0 {
            return;
        }
        // The live transformation is identity (see `transformacao`), and
        // even so it comes from here instead of a hardcoded `1.0`: whatever
        // draws tabs and frames is the SAME function design uses, and it
        // asks the caller for the scale.
        let vista = self.transformacao();
        // the Preview shows what the app will show — the form's theme,
        // not the paper (the paper stays a drawing board, in `Mode::Design`)
        let app_style = app_style_for(&self.ctx, &self.form_theme);

        // Creation order is Tab order, not Z-order: it is what egui
        // uses to cycle focus. Ties are broken by document
        // order, which is what a classic RAD IDE does.
        //
        // The filter is `no_formulario_vivo`, and NOT `self.ordem`: that
        // list was assembled in `open`/`update` with the document's `active_page`,
        // and in Preview the open page is the one the user clicked.
        let mut ordem: Vec<NodeId> = self
            .ordem_toda
            .iter()
            .filter(|id| self.no_formulario_vivo(id))
            .cloned()
            .collect();
        ordem.sort_by_key(|id| self.ctls.get(id).map(|c| c.tab_order).unwrap_or(i64::MAX));

        // Tabs of each `PageControl`, resolved BEFORE state leaves
        // `self` — `paginas_de` reads `paginas_vivas`, and after the `take`
        // below it would answer with an empty map.
        let abas: BTreeMap<NodeId, (Vec<String>, usize)> = ordem
            .iter()
            .filter(|id| self.ctls.get(*id).is_some_and(|c| c.class == "PageControl"))
            .map(|id| (id.clone(), self.paginas_de(id)))
            .collect();

        // State leaves `self` so the closure can mutate it without
        // borrowing the entire runtime.
        let mut texto = std::mem::take(&mut self.texto_vivo);
        let mut selecao = std::mem::take(&mut self.selecao_viva);
        let mut fechados = std::mem::take(&mut self.fechados_vivos);
        let mut marcados = std::mem::take(&mut self.marcados_vivos);
        let mut paginas_abertas = std::mem::take(&mut self.paginas_vivas);
        // The list of what to paint is assembled at the start of the frame, so the
        // new page only appears on the NEXT frame, and this flag requests it.
        //
        // **This is not what makes the tab work today**, though the previous note said
        // it was. Measured, stopwatch in hand: at rest the frame asks for
        // `Duration::MAX` — never —, and on the click frame and next one it
        // asks for `0ns` **with or without this line**, because egui already schedules
        // the next frame after pointer input. That is: no headless gate
        // distinguishes between both cases, and writing one would be writing an
        // accidentally green test.
        //
        // It stays, for another reason: without it, page switching in Preview would
        // depend on an egui internal detail that is not ours and not in its contract.
        // A safety line against a frame that never comes is cheap;
        // discovering this with a user clicking the tab is not.
        let mut trocou_de_pagina = false;
        let ctls: Vec<(NodeId, Ctl, ERect)> = ordem
            .iter()
            .filter_map(|id| {
                let c = self.ctls.get(id)?;
                Some((id.clone(), c.clone(), self.na_tela(c.rect)))
            })
            .collect();

        egui::Area::new(egui::Id::new("rxp-vivo"))
            .fixed_pos(area.min)
            .order(egui::Order::Middle)
            .show(&self.ctx, |ui| {
                ui.set_clip_rect(area);
                // **The live form layer must OCCUPY the area.**
                //
                // Every component is drawn with `new_child`, and `new_child`
                // does not expand the parent `Ui`. Result: `Area` closed the
                // frame with near-zero size, and `Context::layer_id_at`
                // returned `Background` for any point on the form —
                // measured, with pointer in the middle of a `Memo`.
                //
                // Clicks continued working, which is why no one noticed:
                // they go through the widget registry, a different path.
                // What broke was anything asking *is pointer over me?*
                // via layer — and the first victim was `Memo` scrolling:
                // egui's `ScrollArea` only consumes the wheel if
                // `ui.rect_contains_pointer`, and that question answered no.
                ui.expand_to_include_rect(area);
                // In the LIVE form, real egui widgets do the painting,
                // and they read the style from `Ui`. Without this line, user
                // `TextEdit` looked like chrome — black box on light
                // paper — and paper `Style` would apply only to what
                // this file paints by hand.
                ui.set_style(app_style.clone());
                ui.painter().rect_filled(area, 0.0, SUPERFICIE);
                ui.painter().rect_filled(form, 0.0, app_style.visuals.panel_fill);

                for (id, c, r) in &ctls {
                    // A component disabled in the document starts disabled in the
                    // live form — inspector `Enabled` taking effect.
                    let mut construtor = egui::UiBuilder::new().max_rect(*r);
                    if !c.enabled {
                        construtor = construtor.disabled();
                    }
                    let mut filho = ui.new_child(construtor);
                    // The SAME rule as design, and for the SAME two reasons: a
                    // component does not paint outside its own box, and container
                    // children are clipped by their parent.
                    //
                    // Neither was present here. A `Button` dragged halfway out
                    // of a `GroupBox` appeared whole in Preview and clipped in
                    // the designer — 371 points of difference, measured —,
                    // which is exactly the WYSIWYG that ADR 0013 bought going
                    // down the drain. `UiBuilder`'s `max_rect` gives the PLACE,
                    // not the limit: widgets that do not fit expand.
                    //
                    // Intersects with existing clip: `set_clip_rect` replaces,
                    // and replacing would release the component from the drawing
                    // area clip.
                    //
                    // The margin of [`META_DA_BORDA`] is the same as design, and for
                    // the same reason: without it, clipping consumes the outer half
                    // of the frame stroke. Measured, regression was identical on
                    // both sides — `GroupBox` frame lost 0.5 pt on all four
                    // sides in design **and** Preview. Fixing one side only
                    // would have traded a regression for a divergence, which is worse.
                    let corte = match c.clip {
                        Some(pai) => r.intersect(vista.na_tela(pai)),
                        None => *r,
                    };
                    filho.set_clip_rect(filho.clip_rect().intersect(corte.expand(META_DA_BORDA)));
                    match c.class.as_str() {
                        "Button" | "BitBtn" | "SpeedButton" => {
                            let _ = filho
                                .add_sized(r.size(), egui::Button::new(sem_acelerador(&c.text)));
                        }
                        "Edit" | "MaskEdit" => {
                            let buf = texto.entry(id.clone()).or_default();
                            let before = buf.clone();
                            let mut campo =
                                egui::TextEdit::singleline(buf).interactive(!c.somente_leitura);
                            if let Some(ch) = c.senha {
                                // Win32 `PasswordChar` does not cross the
                                // contract, but egui has the concept — so
                                // RXP uses its own when the document asks.
                                let _ = ch;
                                campo = campo.password(true);
                            }
                            let res = filho.add_sized(r.size(), campo);
                            if res.changed() {
                                hold_text(filho.ctx(), res.id, buf, |t, at| crawrad_runtime::mask::EditMask::parse(&c.edit_mask).hold_edit(&before, t, at));
                            }
                        }
                        "DateTimePicker" => {
                            let buf = texto.entry(id.clone()).or_default();
                            let before = buf.clone();
                            let kind = crawrad_runtime::date_time::DateTimeKind::parse_kind(&c.kind);
                            let mask = kind.mask();
                            let field =
                                egui::TextEdit::singleline(buf).interactive(!c.somente_leitura);
                            let res = filho.add_sized(r.size(), field);
                            if res.has_focus() {
                                let (up, down) = filho.input(|i| {
                                    (
                                        i.key_pressed(egui::Key::ArrowUp),
                                        i.key_pressed(egui::Key::ArrowDown),
                                    )
                                });
                                if up || down {
                                    let cursor = text_cursor(filho.ctx(), res.id, buf);
                                    if let Some(iso) =
                                        crawrad_runtime::date_time::shown_to_iso(kind, buf)
                                        && let Some(stepped) = crawrad_runtime::date_time::step_iso(
                                            kind,
                                            &iso,
                                            cursor,
                                            up,
                                            &c.min_date,
                                            &c.max_date,
                                        )
                                    {
                                        *buf = crawrad_runtime::date_time::iso_to_shown(
                                            kind, &stepped,
                                        );
                                        set_text_cursor(filho.ctx(), res.id, cursor);
                                    }
                                }
                            }
                            if res.changed() {
                                hold_text(filho.ctx(), res.id, buf, |t, at| crawrad_runtime::mask::EditMask::parse(mask).hold_edit(&before, t, at));
                            }
                        }
                        "Label" => {
                            // D1: `add_sized` lays the child out centred
                            // and justified, and `.halign(LEFT)` did not undo
                            // it all; a `Ui` of its own, top-down from the left,
                            // puts the text where the designer does
                            filho.scope_builder(
                                egui::UiBuilder::new()
                                    .max_rect(*r)
                                    .layout(egui::Layout::top_down(egui::Align::Min)),
                                |ui| {
                                    ui.add(egui::Label::new(sem_acelerador(&c.text)));
                                },
                            );
                        }
                        "ComboBox" => {
                            // Here is egui's real dropdown: opens, selects,
                            // closes. RCL uses Windows `COMBOBOX`; both
                            // read the same collection from the document.
                            let escolha = selecao.entry(id.clone()).or_default();
                            let atual = escolha.first().copied();
                            let escrito = atual
                                .and_then(|i| c.itens.get(i))
                                .map(|i| i.text.clone())
                                .unwrap_or_default();
                            let mut novo = atual;
                            // Combo box height in egui DOES NOT come from the rectangle: `button_frame` does
                            // `outer_rect.set_height(at_least(interact_size.y))`, and the default is lower
                            // than an Edit's 24 px. Without adjusting this, the designer shows both at the
                            // same size and the application shows one smaller than the other.
                            filho.spacing_mut().interact_size.y = r.height();
                            egui::ComboBox::from_id_salt(id.clone())
                                .selected_text(escrito)
                                .width(r.width())
                                .show_ui(&mut filho, |ui| {
                                    for (i, item) in c.itens.iter().enumerate() {
                                        ui.selectable_value(&mut novo, Some(i), item.text.clone());
                                    }
                                });
                            *escolha = novo.map(|i| vec![i]).unwrap_or_default();
                        }
                        "Panel" => {
                            // In the live form, the panel is just the background: what
                            // appears are the children, drawn afterwards.
                            // In live mode the theme rules, but the panel still
                            // must be visible: `extreme_bg_color` contrasts
                            // with the background in both themes; `faint` does not.
                            filho
                                .painter()
                                .rect_filled(*r, 2.0, app_style.visuals.extreme_bg_color);
                            filho.painter().rect_stroke(
                                *r,
                                2.0,
                                Stroke::new(
                                    c.borda,
                                    app_style.visuals.widgets.noninteractive.fg_stroke.color,
                                ),
                                StrokeKind::Inside,
                            );
                            if !c.text.is_empty() {
                                filho.painter().text(
                                    Pos2::new(r.min.x + 6.0, r.min.y + 9.0),
                                    Align2::LEFT_CENTER,
                                    sem_acelerador(&c.text),
                                    egui::FontId::proportional(12.0),
                                    app_style.visuals.text_color(),
                                );
                            }
                        }
                        "PageControl" => {
                            // **The SAME function that the generated application calls.**
                            // It lives in `desenho_comum.rs`, and `crawrad-codegen`
                            // copies its text into the user's `designer.rs`.
                            // Strip, body, click target, and page switching
                            // are a single algorithm: Preview and the
                            // application cannot disagree on them.
                            //
                            // Without "+": adding a page is a design gesture, and
                            // applications do not gain tabs by clicking. What paints
                            // "+" is `tira_de_abas`, which remains the DESIGN
                            // branch — it scales with zoom, and egui widgets
                            // do not scale by rectangle (ADR 0019).
                            let (paginas, ativa) =
                                abas.get(id).cloned().unwrap_or_else(|| (Vec::new(), 0));
                            let legendas: Vec<(&str, &str)> =
                                paginas.iter().map(|p| (p.as_str(), "")).collect();
                            // Choice goes to LIVE state: the document is not
                            // touched.
                            let mut aberta = ativa;
                            // the IDE draws with Makepad (ADR 0032): this Preview shows every tab,
                            // at the top
                            let shown = vec![true; legendas.len()];
                            if page_control(&mut filho, id, *r, &legendas, &shown, false, &mut aberta) {
                                paginas_abertas.insert(id.clone(), aberta);
                                trocou_de_pagina = true;
                            }
                        }
                        "TabSheet" => {}
                        "GroupBox" => {
                            // Without background, like `TGroupBox`: children come
                            // afterwards, and the parent shows through it.
                            moldura_do_groupbox(
                                filho.painter(),
                                *r,
                                &sem_acelerador(&c.text),
                                egui::FontId::proportional(12.0),
                                app_style.visuals.widgets.noninteractive.fg_stroke.color,
                                app_style.visuals.text_color(),
                                vista,
                            );
                        }
                        "Toggle" => {
                            // a real switch: the click flips it (live state)
                            let ligado = marcados.get(id).copied().unwrap_or(c.marcado);
                            let resp = filho.interact(*r, egui::Id::new(("rxp-toggle", id)), egui::Sense::click());
                            let track = ERect::from_min_size(Pos2::new(r.min.x + 2.0, r.center().y - 8.0), Vec2::new(32.0, 16.0));
                            let fill = if ligado { app_style.visuals.selection.bg_fill } else { app_style.visuals.widgets.inactive.bg_fill };
                            filho.painter().rect(track, 8.0, fill, app_style.visuals.widgets.inactive.bg_stroke, StrokeKind::Inside);
                            let x = if ligado { track.right() - 8.0 } else { track.left() + 8.0 };
                            filho.painter().circle_filled(Pos2::new(x, track.center().y), 6.0, app_style.visuals.text_color());
                            filho.painter().text(Pos2::new(track.right() + 6.0, r.center().y), Align2::LEFT_CENTER, sem_acelerador(&c.text), egui::FontId::proportional(12.0), app_style.visuals.text_color());
                            if resp.clicked() && c.enabled {
                                marcados.insert(id.clone(), !ligado);
                            }
                        }
                        "LinkLabel" => {
                            // a real link; the preview opens nothing (the
                            // generated app does), it only shows the address
                            let mut w = filho.new_child(egui::UiBuilder::new().max_rect(*r));
                            if !c.enabled {
                                w.disable();
                            }
                            w.link(sem_acelerador(&c.text)).on_hover_text(c.url.as_str());
                        }
                        "ColorBox" => {
                            // the swatch and a real list of the standard
                            // colours; the choice is live state in egui's memory
                            let key = egui::Id::new(("rxp-colorbox", id));
                            let mut k: crawrad_ir::Color = filho.ctx().data(|d| d.get_temp(key)).unwrap_or(c.selected);
                            let mut builder = egui::UiBuilder::new().max_rect(*r);
                            if !c.enabled {
                                builder = builder.disabled();
                            }
                            let mut w = filho.new_child(builder);
                            let s = 14.0f32.min(r.height());
                            let swatch = ERect::from_min_size(Pos2::new(r.min.x + 4.0, r.center().y - s / 2.0), Vec2::splat(s));
                            w.painter().rect(swatch, 2.0, Color32::from_rgba_unmultiplied(k.r, k.g, k.b, k.a), app_style.visuals.widgets.inactive.bg_stroke, StrokeKind::Inside);
                            let name = crawrad_ir::Color::STANDARD.iter().find(|(_, x)| *x == k).map_or("Custom", |(n, _)| n);
                            let list = ERect::from_min_max(Pos2::new(swatch.max.x + 4.0, r.min.y), r.max);
                            w.put(list, |ui: &mut egui::Ui| {
                                egui::ComboBox::from_id_salt(("rxp-colorbox-list", id)).width(list.width()).selected_text(name).show_ui(ui, |ui| {
                                    for (n, x) in crawrad_ir::Color::STANDARD {
                                        ui.selectable_value(&mut k, x, n);
                                    }
                                }).response
                            });
                            filho.ctx().data_mut(|d| d.insert_temp(key, k));
                        }
                        "SpinEdit" => {
                            // a real number field with its step buttons;
                            // the number is live state in egui's memory
                            let key = egui::Id::new(("rxp-spin", id));
                            let mut v: i64 = filho.ctx().data(|d| d.get_temp(key)).unwrap_or(c.spin);
                            let mut builder = egui::UiBuilder::new().max_rect(*r);
                            if !c.enabled {
                                builder = builder.disabled();
                            }
                            let mut w = filho.new_child(builder);
                            let steps = 16.0f32.min(r.width());
                            let field = ERect::from_min_max(r.min, Pos2::new(r.max.x - steps, r.max.y));
                            w.put(field, egui::DragValue::new(&mut v).speed(0.0));
                            let up = ERect::from_min_max(Pos2::new(r.max.x - steps, r.min.y), Pos2::new(r.max.x, r.center().y));
                            let down = ERect::from_min_max(Pos2::new(r.max.x - steps, r.center().y), r.max);
                            if w.put(up, egui::Button::new("+").small()).clicked() {
                                v = v.saturating_add(1);
                            }
                            if w.put(down, egui::Button::new("-").small()).clicked() {
                                v = v.saturating_sub(1);
                            }
                            filho.ctx().data_mut(|d| d.insert_temp(key, v));
                        }
                        "RadioGroup" => {
                            // the frame, and a real radio per item in
                            // its `Columns`; the choice is live state, as the
                            // ListBox's
                            moldura_do_groupbox(
                                filho.painter(),
                                *r,
                                &sem_acelerador(&c.text),
                                egui::FontId::proportional(12.0),
                                app_style.visuals.widgets.noninteractive.fg_stroke.color,
                                app_style.visuals.text_color(),
                                vista,
                            );
                            let escolha = selecao
                                .entry(id.clone())
                                .or_insert_with(|| c.escolhido.into_iter().collect());
                            let atual = escolha.first().copied();
                            let columns = c.columns.max(1) as usize;
                            let rows = c.itens.len().div_ceil(columns).max(1);
                            let inner = ERect::from_min_max(
                                r.min + Vec2::new(8.0, 18.0),
                                r.max - Vec2::new(6.0, 6.0),
                            );
                            let cell = Vec2::new(
                                inner.width() / columns as f32,
                                (inner.height() / rows as f32).min(22.0),
                            );
                            let mut builder = egui::UiBuilder::new().max_rect(*r);
                            if !c.enabled {
                                builder = builder.disabled();
                            }
                            let mut w = filho.new_child(builder);
                            for (i, item) in c.itens.iter().enumerate() {
                                let at = inner.min
                                    + Vec2::new((i / rows) as f32 * cell.x, (i % rows) as f32 * cell.y);
                                if w
                                    .put(
                                        ERect::from_min_size(at, cell),
                                        egui::RadioButton::new(atual == Some(i), item.text.clone()),
                                    )
                                    .clicked()
                                {
                                    *escolha = vec![i];
                                }
                            }
                        }
                        "ListView" => {
                            // the same drawing as the designer, clipped
                            // to its box; a click on a row picks it, and the
                            // choice is live state, as the ListBox's
                            let escolha = selecao
                                .entry(id.clone())
                                .or_insert_with(|| c.escolhido.into_iter().collect());
                            let header = if c.show_column_headers && !c.columns_info.is_empty() {
                                LIST_VIEW_HEADER_H
                            } else {
                                0.0
                            };
                            let click = filho.interact(
                                *r,
                                filho.id().with(("rxp-listview", id)),
                                egui::Sense::click(),
                            );
                            if c.enabled && click.clicked() {
                                let row = click.interact_pointer_pos().and_then(|at| {
                                    let y = at.y - r.min.y - header;
                                    (y >= 0.0).then(|| (y / LIST_VIEW_ROW_H) as usize)
                                });
                                if let Some(row) = row.filter(|row| *row < c.itens.len()) {
                                    *escolha = vec![row];
                                }
                            }
                            let cols: Vec<(&str, f32, &str)> = c
                                .columns_info
                                .iter()
                                .map(|(cap, w, align)| (cap.as_str(), *w as f32, align.as_str()))
                                .collect();
                            let subs: Vec<Vec<&str>> = c
                                .itens
                                .iter()
                                .map(|it| it.children.iter().map(|s| s.text.as_str()).collect())
                                .collect();
                            let rows: Vec<(&str, &[&str])> = c
                                .itens
                                .iter()
                                .zip(&subs)
                                .map(|(it, sub)| (it.text.as_str(), sub.as_slice()))
                                .collect();
                            paint_list_view(
                                &filho.painter_at(*r),
                                *r,
                                &cols,
                                &rows,
                                escolha.first().copied(),
                                c.row_select,
                                c.grid_lines,
                                c.show_column_headers,
                                &app_style.visuals,
                                egui::FontId::proportional(12.0),
                            );
                        }
                        "Memo" => {
                            // **Clipping does not work here, and that is the boundary.**
                            // In design `Memo` shows what fits and clips the
                            // rest, like the designer of a classic RAD IDE; in the application
                            // it is a real edit field, and a field that swallows
                            // what the user typed would be worse than one that overflows.
                            // So in live mode it SCROLLS.
                            //
                            // Without this, `TextEdit::multiline` grew with text
                            // and ignored the rectangle: thirty lines in a 90-point
                            // box spilled 450 points — measured — over everything
                            // beneath it. `add_sized` does not restrain it: it provides
                            // DESIRED size, and `TextEdit` asks for more.
                            let buf = texto.entry(id.clone()).or_default();
                            let so_leitura = c.somente_leitura;
                            let edit_id = egui::Id::new(("rxp-memo-txt", id));
                            let is_focado = filho.ctx().memory(|m| m.has_focus(edit_id));
                            let borda = if is_focado {
                                filho.visuals().selection.stroke
                            } else {
                                filho.visuals().widgets.inactive.bg_stroke
                            };
                            let v_scroll = !matches!(c.scroll_bars.as_str(), "ssHorizontal");
                            let h_scroll = matches!(c.scroll_bars.as_str(), "ssHorizontal" | "ssBoth");
                            egui::Frame::group(filho.style())
                                .fill(filho.visuals().extreme_bg_color)
                                .stroke(borda)
                                .corner_radius(filho.visuals().widgets.inactive.corner_radius)
                                .inner_margin(egui::Margin::ZERO)
                                .show(&mut filho, |ui| {
                                    ui.set_min_size(r.size());
                                    ui.set_max_size(r.size());
                                    let area = egui::ScrollArea::new([h_scroll, v_scroll])
                                        .id_salt(("rxp-memo", id))
                                        .max_height(r.height().max(16.0))
                                        .max_width(r.width())
                                        .auto_shrink([false, false]);
                                    let area = if c.scroll_bars == "ssNone" {
                                        area.scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                                    } else {
                                        area.scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
                                    };
                                    area.show(ui, |ui| {
                                        if h_scroll {
                                            let mut memo_layouter = |ui: &egui::Ui, text: &dyn egui::TextBuffer, _wrap_width: f32| {
                                                let font_id = egui::TextStyle::Body.resolve(ui.style());
                                                let mut job = egui::text::LayoutJob::simple(
                                                    text.as_str().to_string(),
                                                    font_id,
                                                    ui.visuals().widgets.inactive.text_color(),
                                                    f32::INFINITY,
                                                );
                                                job.keep_trailing_whitespace = true;
                                                ui.fonts_mut(|f| f.layout_job(job))
                                            };
                                            ui.add(
                                                egui::TextEdit::multiline(buf)
                                                    .id(edit_id)
                                                    .frame(egui::Frame::NONE)
                                                    .desired_width(f32::INFINITY)
                                                    .layouter(&mut memo_layouter)
                                                    .interactive(!so_leitura),
                                            );
                                        } else {
                                            ui.add(
                                                egui::TextEdit::multiline(buf)
                                                    .id(edit_id)
                                                    .frame(egui::Frame::NONE)
                                                    .desired_width(f32::INFINITY)
                                                    .interactive(!so_leitura),
                                            );
                                        }
                                    });
                                });
                        }
                        "CheckBox" => {
                            let m = marcados.entry(id.clone()).or_default();
                            // D2: a `Ui` of its own, left to right and
                            // centred down, as the designer draws it
                            filho.scope_builder(
                                egui::UiBuilder::new()
                                    .max_rect(*r)
                                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
                                |ui| {
                                    ui.add(egui::Checkbox::new(m, sem_acelerador(&c.text)));
                                },
                            );
                        }
                        "RadioButton" => {
                            // Mutual exclusion is between siblings: see `irmaos_a_desmarcar`.
                            let ligado = marcados.get(id).copied().unwrap_or(false);
                            // D2: the same `Ui` as the CheckBox
                            let resp = filho
                                .scope_builder(
                                    egui::UiBuilder::new()
                                        .max_rect(*r)
                                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                                    |ui| ui.add(egui::RadioButton::new(ligado, sem_acelerador(&c.text))),
                                )
                                .inner;
                            if resp.clicked() {
                                marcados.insert(id.clone(), true);
                                for irmao in irmaos_a_desmarcar(
                                    ctls.iter().map(|(i, c, _)| (i, c)),
                                    id,
                                    &c.parent,
                                ) {
                                    marcados.insert(irmao, false);
                                }
                            }
                        }
                        "ListBox" => {
                            let escolha = selecao.entry(id.clone()).or_default();
                            let mut atual = escolha.first().copied();
                            let mut builder = egui::UiBuilder::new().max_rect(*r);
                            if !c.enabled {
                                builder = builder.disabled();
                            }
                            let mut w = filho.new_child(builder);
                            egui::Frame::group(w.style())
                                .fill(app_style.visuals.extreme_bg_color)
                                .stroke(app_style.visuals.widgets.inactive.bg_stroke)
                                .corner_radius(egui::CornerRadius::same(2))
                                .inner_margin(egui::Margin::ZERO)
                                .show(&mut w, |ui| {
                                    egui::ScrollArea::vertical()
                                        .id_salt(("rxp-listbox", id))
                                        .max_height(r.height())
                                        .max_width(r.width())
                                        .auto_shrink([false, false])
                                        .show(ui, |ui| {
                                             ui.set_width(ui.available_width());
                                            ui.spacing_mut().item_spacing.y = 0.0;
                                            for (i, item) in c.itens.iter().enumerate() {
                                                if ui
                                                    .add(
                                                        egui::Button::selectable(
                                                            atual == Some(i),
                                                            item.text.clone(),
                                                        )
                                                        .min_size(egui::vec2(ui.available_width(), LINHA_DA_LISTA)),
                                                    )
                                                    .clicked()
                                                {
                                                    atual = Some(i);
                                                }
                                            }
                                        });
                                });
                            *escolha = atual.map(|i| vec![i]).unwrap_or_default();
                        }
                        "TreeView" => {
                            // Real tree: click, select, collapse, and expand.
                            // In RCL this comes from `SysTreeView32`; here it is
                            // ours, and must behave identically — an adapter
                            // responding differently to the same document is what
                            // ADR 0002 forbids.
                            filho
                                .painter()
                                .rect_filled(*r, 2.0, app_style.visuals.extreme_bg_color);
                            filho.painter().rect_stroke(
                                *r,
                                2.0,
                                app_style.visuals.widgets.inactive.bg_stroke,
                                StrokeKind::Inside,
                            );
                            let sel = selecao.entry(id.clone()).or_default();
                            let fech = fechados.entry(id.clone()).or_default();
                            arvore_viva(&mut filho, id, *r, &c.itens, sel, fech);
                        }
                        "ScrollBar" => {
                            filho.painter().rect_filled(*r, 2.0, app_style.visuals.extreme_bg_color);
                            filho.painter().rect_stroke(*r, 2.0, app_style.visuals.widgets.inactive.bg_stroke, StrokeKind::Inside);
                            // The SAME `barra_de_rolagem` as the design branch.
                            let b = barra_de_rolagem(*r, c.posicao_scroll);
                            let fundo = app_style.visuals.widgets.inactive.bg_fill;
                            filho.painter().rect_filled(b.inicio, 2.0, fundo);
                            filho.painter().rect_filled(b.fim, 2.0, fundo);
                            filho.painter().rect_filled(b.polegar, 2.0, PAINEL_BORDA);
                        }
                        "ScrollBox" => {
                            // The SAME drawing as the design branch, for the same
                            // reason `ScrollBar` right above calls the same
                            // `barra_de_rolagem`: what the designer draws must
                            // match what Preview shows (ADR 0013).
                            filho.painter().rect_filled(*r, 2.0, app_style.visuals.extreme_bg_color);
                            filho.painter().rect_stroke(*r, 2.0, app_style.visuals.widgets.inactive.bg_stroke, StrokeKind::Inside);
                        }
                        "Grid" => {
                            filho.painter().rect_filled(*r, 2.0, app_style.visuals.extreme_bg_color);
                            filho.painter().rect_stroke(*r, 2.0, app_style.visuals.widgets.inactive.bg_stroke, StrokeKind::Inside);
                            let col_w = c.default_col_width.max(10.0);
                            let row_h = c.default_row_height.max(10.0);
                            let header_h = (c.fixed_rows as f32 * row_h).min(r.height());
                            if header_h > 0.0 {
                                let header_rect = ERect::from_min_size(r.min, Vec2::new(r.width(), header_h));
                                filho.painter().rect_filled(header_rect, 0.0, app_style.visuals.faint_bg_color);
                            }
                            let header_w = (c.fixed_cols as f32 * col_w).min(r.width());
                            if header_w > 0.0 {
                                let fixed_col_rect = ERect::from_min_size(r.min, Vec2::new(header_w, r.height()));
                                filho.painter().rect_filled(fixed_col_rect, 0.0, app_style.visuals.faint_bg_color);
                            }
                            if c.grid_lines {
                                let mut x = r.min.x;
                                for _ in 0..c.col_count {
                                    x += col_w;
                                    if x >= r.max.x {
                                        break;
                                    }
                                    filho.painter().line_segment([Pos2::new(x, r.min.y), Pos2::new(x, r.max.y)], Stroke::new(1.0, app_style.visuals.widgets.noninteractive.bg_stroke.color));
                                }
                                let mut y = r.min.y;
                                for _ in 0..c.row_count {
                                    y += row_h;
                                    if y >= r.max.y {
                                        break;
                                    }
                                    filho.painter().line_segment([Pos2::new(r.min.x, y), Pos2::new(r.max.x, y)], Stroke::new(1.0, app_style.visuals.widgets.noninteractive.bg_stroke.color));
                                }
                            }
                        }
                        "Image" => {
                            filho.painter().rect_filled(*r, 2.0, app_style.visuals.extreme_bg_color);
                            filho.painter().rect_stroke(*r, 2.0, app_style.visuals.widgets.inactive.bg_stroke, StrokeKind::Inside);
                            let tem_imagem = c.picture.as_ref().is_some_and(|pic| !pic.is_empty());
                            if tem_imagem {
                                let icone_rect = ERect::from_center_size(r.center(), Vec2::splat(24.0));
                                filho.painter().rect_stroke(
                                    icone_rect,
                                    2.0,
                                    egui::Stroke::new(1.5, egui::Color32::from_rgb(0x4A, 0x88, 0xC7)),
                                    StrokeKind::Inside,
                                );
                                filho.painter().text(
                                    r.center(),
                                    Align2::CENTER_CENTER,
                                    "🖼",
                                    egui::FontId::proportional(16.0),
                                    egui::Color32::from_rgb(0x4A, 0x88, 0xC7),
                                );
                            } else {
                                filho.painter().text(
                                    r.center(),
                                    Align2::CENTER_CENTER,
                                    "🖼",
                                    egui::FontId::proportional(18.0),
                                    CTL_TEXTO_FRACO,
                                );
                            }
                        }
                        "ToolBar" => {
                            filho.painter().rect_filled(*r, 2.0, app_style.visuals.extreme_bg_color);
                            filho.painter().rect_stroke(*r, 2.0, app_style.visuals.widgets.noninteractive.fg_stroke, StrokeKind::Inside);
                        }
                        "StatusBar" => {
                            filho.add_sized(
                                r.size(),
                                egui::Label::new(sem_acelerador(&c.text)).halign(egui::Align::LEFT),
                            );
                        }
                        "ProgressBar" => {
                            let frac = (c.posicao_scroll / 100.0).clamp(0.0, 1.0);
                            let _ = filho.add_sized(r.size(), egui::ProgressBar::new(frac));
                        }
                        "TrackBar" => {
                            let mut val = c.posicao_scroll;
                            let _ = filho.add_sized(r.size(), egui::Slider::new(&mut val, 0.0..=100.0).show_value(false));
                        }
                        "PaintBox" => {
                            let exibido = if !c.text.is_empty() { &c.text } else { &c.name };
                            filho.add_sized(
                                r.size(),
                                egui::Label::new(exibido).halign(egui::Align::Center),
                            );
                        }
                        "Bevel" => {
                            paint_bevel(filho.painter(), *r, &c.bevel_shape, c.bevel_style == "Raised", &app_style.visuals);
                        }
                        "Badge" => {
                            if let Some(badge_str) = badge_text(&c.text, c.count, c.max_count) {
                                let mut br = *r;
                                let h = br.height();
                                let w = if badge_str.chars().count() == 1 {
                                    h
                                } else {
                                    br.width().max(h)
                                };
                                if !c.associate.is_empty() {
                                    if let Some(target) = self.ctls.values().find(|target| target.id == c.associate || target.name == c.associate) {
                                        let target_r = self.na_tela(target.rect);
                                        br = ERect::from_center_size(target_r.right_top(), Vec2::new(w, h));
                                    } else {
                                        br = ERect::from_center_size(br.center(), Vec2::new(w, h));
                                    }
                                } else {
                                    br = ERect::from_center_size(br.center(), Vec2::new(w, h));
                                }
                                let effective_kind = if c.count > 0 && (c.kind == "bkPrimary" || c.kind.is_empty()) {
                                    "bkDanger"
                                } else {
                                    &c.kind
                                };
                                let mut painter = filho.painter().clone();
                                if !c.enabled {
                                    painter.multiply_opacity(0.5);
                                }
                                paint_badge(
                                    &painter,
                                    br,
                                    &badge_str,
                                    effective_kind,
                                    c.outline,
                                    &app_style.visuals,
                                    egui::FontId::proportional(11.0),
                                );
                            }
                        }
                        "Chips" => {
                            let escolha = selecao
                                .entry(id.clone())
                                .or_insert_with(|| c.escolhido.into_iter().collect());
                            let mut atual = escolha.first().copied();
                            let widths: Vec<f32> = c.itens.iter().map(|it| chip_width(&it.text, c.removable)).collect();
                            let positions = chips_layout(&widths, r.width(), CHIP_HEIGHT, CHIP_GAP);
                            let font = egui::FontId::proportional(12.0);
                            let mut painter = filho.painter_at(*r);
                            if !c.enabled {
                                painter.multiply_opacity(0.5);
                            }
                            for (i, (&(x, y), it)) in positions.iter().zip(c.itens.iter()).enumerate() {
                                let w = widths[i];
                                let chip_rect = ERect::from_min_size(egui::pos2(r.min.x + x, r.min.y + y), egui::vec2(w, CHIP_HEIGHT));
                                if chip_rect.max.y > r.max.y {
                                    break;
                                }
                                let is_sel = atual == Some(i);
                                paint_chip(&painter, chip_rect, &it.text, is_sel, c.removable, &app_style.visuals, font.clone());
                                if c.enabled {
                                    let resp = filho.interact(chip_rect, filho.id().with(("rxp-chip", id, i)), egui::Sense::click());
                                    if resp.clicked() {
                                        atual = Some(i);
                                    }
                                }
                            }
                            *escolha = atual.map(|i| vec![i]).unwrap_or_default();
                        }
                        "Breadcrumb" => {
                            let escolha = selecao
                                .entry(id.clone())
                                .or_insert_with(|| c.escolhido.into_iter().collect());
                            let mut atual = escolha.first().copied();
                            let item_strs: Vec<&str> = c.itens.iter().map(|it| it.text.as_str()).collect();
                            let font = egui::FontId::proportional(12.0);
                            let mut hovered = None;
                            if c.style == "bsChevron" {
                                let widths: Vec<f32> = item_strs.iter().map(|it| breadcrumb_text_width(filho.painter(), it, &font)).collect();
                                let pads = ChevronPads::default();
                                let positions = chevron_layout(&widths, CHEVRON_TIP, CHEVRON_OVERLAP, pads);
                                if c.enabled && !item_strs.is_empty() {
                                    for (i, (&(x, w), _)) in positions.iter().zip(item_strs.iter()).enumerate() {
                                        let seg_rect = ERect::from_min_size(egui::pos2(r.min.x + x, r.min.y), egui::vec2(w, r.height()));
                                        if seg_rect.min.x >= r.max.x {
                                            break;
                                        }
                                        let resp = filho.interact(seg_rect, filho.id().with(("rxp-brc", id, i)), egui::Sense::click());
                                        if resp.hovered() {
                                            hovered = Some(i);
                                        }
                                        if resp.clicked() {
                                            atual = Some(i);
                                        }
                                    }
                                }
                                let mut painter = filho.painter_at(*r);
                                if !c.enabled {
                                    painter.multiply_opacity(0.5);
                                }
                                let count = item_strs.len();
                                let active_idx = if let Some(i) = atual {
                                    i.min(count.saturating_sub(1))
                                } else if count > 0 {
                                    count - 1
                                } else {
                                    0
                                };
                                paint_breadcrumb_chevron(
                                    &painter,
                                    *r,
                                    &item_strs,
                                    active_idx,
                                    hovered,
                                    c.segment_color.into(),
                                    c.current_color.into(),
                                    font,
                                );
                            } else {
                                let widths: Vec<f32> = item_strs.iter().map(|it| breadcrumb_text_width(filho.painter(), it, &font)).collect();
                                let sep_w = breadcrumb_text_width(filho.painter(), &c.separator, &font);
                                let positions = breadcrumb_layout(&widths, sep_w, BREADCRUMB_GAP);
                                if c.enabled && item_strs.len() > 1 {
                                    for (i, (&x, &w)) in positions.iter().zip(widths.iter()).enumerate().take(item_strs.len() - 1) {
                                        let seg_rect = ERect::from_min_size(egui::pos2(r.min.x + x, r.min.y), egui::vec2(w, r.height()));
                                        if seg_rect.min.x >= r.max.x {
                                            break;
                                        }
                                        let resp = filho.interact(seg_rect, filho.id().with(("rxp-brc", id, i)), egui::Sense::click());
                                        if resp.hovered() {
                                            hovered = Some(i);
                                        }
                                        if resp.clicked() {
                                            atual = Some(i);
                                        }
                                    }
                                }
                                let mut painter = filho.painter_at(*r);
                                if !c.enabled {
                                    painter.multiply_opacity(0.5);
                                }
                                paint_breadcrumb(&painter, *r, &item_strs, &c.separator, hovered, &app_style.visuals, font);
                            }
                            *escolha = atual.map(|i| vec![i]).unwrap_or_default();
                        }
                        "Pagination" => {
                            let escolha = selecao
                                .entry(id.clone())
                                .or_insert_with(|| vec![c.page_index.max(0) as usize]);
                            let mut atual = escolha.first().copied().unwrap_or(c.page_index.max(0) as usize);
                            let font = egui::FontId::proportional(12.0);
                            let slots = pagination_slots(
                                c.page_count.max(1) as usize,
                                atual,
                                c.max_buttons.max(5) as usize,
                                c.show_arrows,
                            );
                            let mut hovered = None;
                            if c.enabled {
                                let mut x = r.min.x;
                                for (slot_idx, slot) in slots.iter().enumerate() {
                                    let label = match slot {
                                        PageSlot::Prev => "‹".to_string(),
                                        PageSlot::Next => "›".to_string(),
                                        PageSlot::Gap => "…".to_string(),
                                        PageSlot::Page(p) => (p + 1).to_string(),
                                    };
                                    let text_w = pagination_text_width(filho.painter(), &label, &font);
                                    let cell_w = r.height().max(text_w + 8.0);
                                    let cell_rect = ERect::from_min_size(egui::pos2(x, r.min.y), egui::vec2(cell_w, r.height()));
                                    if cell_rect.min.x >= r.max.x {
                                        break;
                                    }
                                    let is_clickable = match slot {
                                        PageSlot::Prev => atual > 0,
                                        PageSlot::Next => atual + 1 < c.page_count.max(1) as usize,
                                        PageSlot::Page(_) => true,
                                        PageSlot::Gap => false,
                                    };
                                    if is_clickable {
                                        let resp = filho.interact(cell_rect, filho.id().with(("rxp-pag", id, slot_idx)), egui::Sense::click());
                                        if resp.hovered() {
                                            hovered = Some(slot_idx);
                                        }
                                        if resp.clicked() {
                                            match slot {
                                                PageSlot::Prev => atual = atual.saturating_sub(1),
                                                PageSlot::Next => atual = (atual + 1).min(c.page_count.max(1) as usize - 1),
                                                PageSlot::Page(p) => atual = *p,
                                                PageSlot::Gap => {}
                                            }
                                        }
                                    }
                                    x += cell_w + PAGINATION_GAP;
                                }
                            }
                            let mut painter = filho.painter_at(*r);
                            if !c.enabled {
                                painter.multiply_opacity(0.5);
                            }
                            paint_pagination(&painter, *r, &slots, atual, hovered, &app_style.visuals, font);
                            *escolha = vec![atual];
                        }
                        "MultiSelect" => {
                            let item_strs: Vec<String> = c.itens.iter().map(|it| it.text.clone()).collect();
                            let summary = multi_select_summary(
                                &item_strs,
                                &c.selected_items,
                                &c.text_hint,
                                |text| {
                                    filho.painter().layout_no_wrap(
                                        text.to_owned(),
                                        egui::FontId::proportional(12.0),
                                        app_style.visuals.text_color(),
                                    ).size().x <= (r.width() - 24.0).max(0.0)
                                },
                            );
                            let mut painter = filho.painter_at(*r);
                            if !c.enabled {
                                painter.multiply_opacity(0.5);
                            }
                            paint_multi_select(&painter, *r, &summary, &app_style.visuals, egui::FontId::proportional(12.0));
                        }
                        "CheckListBox" => {
                            let mut builder = egui::UiBuilder::new().max_rect(*r);
                            if !c.enabled {
                                builder = builder.disabled();
                            }
                            let mut w = filho.new_child(builder);
                            egui::Frame::group(w.style())
                                .fill(app_style.visuals.extreme_bg_color)
                                .stroke(app_style.visuals.widgets.inactive.bg_stroke)
                                .corner_radius(egui::CornerRadius::same(2))
                                .inner_margin(egui::Margin::same(4))
                                .show(&mut w, |ui| {
                                    egui::ScrollArea::vertical()
                                        .id_salt(("rxp-checklistbox", id))
                                        .max_height(r.height())
                                        .max_width(r.width())
                                        .auto_shrink([false, false])
                                        .show(ui, |ui| {
                                            for item in &c.itens {
                                                let mut checked = c.checked_items.iter().any(|s| s == &item.text);
                                                ui.checkbox(&mut checked, item.text.as_str());
                                            }
                                        });
                                });
                        }
                        "Card" => {
                            paint_card(
                                filho.painter(),
                                *r,
                                &sem_acelerador(&c.text),
                                &c.subtitle,
                                c.corner_radius,
                                c.elevation,
                                c.show_header,
                                &app_style.visuals,
                                egui::FontId::proportional(12.0),
                            );
                        }
                        "Drawer" => {
                            if c.opened && c.overlay {
                                let scrim_color = app_style.visuals.extreme_bg_color.gamma_multiply(0.4);
                                filho.painter().rect_filled(form, 0.0, scrim_color);
                            }
                            let caption = sem_acelerador(&c.text);
                            let displayed_caption = if !c.opened {
                                format!("{caption} (closed)")
                            } else {
                                caption
                            };
                            paint_drawer(
                                filho.painter(),
                                *r,
                                &displayed_caption,
                                &c.side,
                                c.show_header,
                                false,
                                &app_style.visuals,
                                egui::FontId::proportional(12.0),
                            );
                            if !c.opened {
                                draw_dashed_rect(
                                    filho.painter(),
                                    *r,
                                    app_style.visuals.widgets.noninteractive.bg_stroke,
                                    4.0,
                                    4.0,
                                );
                            }
                        }
                        "CategoryPanel" => {
                            // the header folds it, live; its children
                            // come and go on the next frame
                            let folded = marcados.get(id).copied().unwrap_or(c.collapsed);
                            let header = paint_category(filho.painter(), *r, &sem_acelerador(&c.text), folded, egui::FontId::proportional(12.0), &app_style.visuals, 24.0);
                            let resp = filho.interact(header, egui::Id::new(("rxp-category", id)), egui::Sense::click());
                            if resp.clicked() && c.enabled {
                                marcados.insert(id.clone(), !folded);
                                trocou_de_pagina = true;
                            }
                        }
                        "Shape" => {
                            filho.painter().rect_filled(*r, 2.0, app_style.visuals.extreme_bg_color);
                            if c.shape_kind == "Circle" {
                                let radius = (r.width().min(r.height()) / 2.0).max(1.0);
                                filho.painter().circle_stroke(r.center(), radius, Stroke::new(c.borda, app_style.visuals.widgets.noninteractive.bg_stroke.color));
                            } else {
                                filho.painter().rect_stroke(*r, 2.0, Stroke::new(c.borda, app_style.visuals.widgets.noninteractive.bg_stroke.color), StrokeKind::Inside);
                            }
                        }
                        "Splitter" => {
                            filho.painter().rect_filled(*r, 0.0, PAINEL);
                            filho.painter().rect_stroke(*r, 0.0, Stroke::new(1.0, PAINEL_BORDA), StrokeKind::Inside);
                            if c.beveled {
                                let mid_x = r.center().x;
                                let mid_y = r.center().y;
                                let dot_radius = 1.5;
                                let dot_color = app_style.visuals.widgets.noninteractive.bg_stroke.color;
                                if r.height() >= r.width() {
                                    filho.painter().circle_filled(Pos2::new(mid_x, mid_y - 6.0), dot_radius, dot_color);
                                    filho.painter().circle_filled(Pos2::new(mid_x, mid_y), dot_radius, dot_color);
                                    filho.painter().circle_filled(Pos2::new(mid_x, mid_y + 6.0), dot_radius, dot_color);
                                } else {
                                    filho.painter().circle_filled(Pos2::new(mid_x - 6.0, mid_y), dot_radius, dot_color);
                                    filho.painter().circle_filled(Pos2::new(mid_x, mid_y), dot_radius, dot_color);
                                    filho.painter().circle_filled(Pos2::new(mid_x + 6.0, mid_y), dot_radius, dot_color);
                                }
                            }
                        }
                        _ => {
                            filho.add_sized(r.size(), egui::Label::new(c.class.clone()));
                        }
                    }
                }
            });

        self.texto_vivo = texto;
        self.selecao_viva = selecao;
        self.fechados_vivos = fechados;
        self.marcados_vivos = marcados;
        self.paginas_vivas = paginas_abertas;
        if trocou_de_pagina {
            self.ctx.request_repaint();
        }
    }

    /// Makes form `Timer`s run, in Preview.
    ///
    /// **This is the piece that does not exist in an immediate mode toolkit.** In
    /// a classic RAD IDE `TTimer` is a system message: Windows wakes up the
    /// program. In egui nothing wakes it — a frame only happens when
    /// someone asks for it. If nobody asks, the application stays idle and the timer never
    /// fires. So **requesting the next frame is the timer**, and `request_repaint_after`
    /// below plays the role of `SetTimer`.
    ///
    /// Time comes from the frame clock (`input.time`, in seconds), not from
    /// frame counting: a timer firing "once per frame if time elapsed" behaves
    /// differently on a slow machine, with an unfocused window, or with a minimized
    /// window — precisely when egui stops drawing.
    fn rodar_timers(&mut self) {
        if self.prazos_vivos.is_empty() {
            return;
        }
        let agora = self.ctx.input(|i| i.time);
        let mut proximo: Option<f64> = None;
        for (id, prazo) in &mut self.prazos_vivos {
            let Some(c) = self.ctls.get(id) else { continue };
            // `Enabled = False` and `Interval = 0` stop the timer, both as
            // in a classic RAD IDE — there `UpdateTimer` only calls `SetTimer` with
            // `FEnabled` and `FInterval > 0`.
            if !c.enabled || c.intervalo <= 0 {
                *prazo = None;
                continue;
            }
            // `as u64` is exact, not a truncation: the guard right above already
            // returned on every `intervalo <= 0`.
            if timer_due(agora, c.intervalo as u64, prazo) {
                *self.tiques_vivos.entry(id.clone()).or_insert(0) += 1;
            }
            if let Some(p) = *prazo {
                proximo = Some(proximo.map_or(p, |q: f64| q.min(p)));
            }
        }
        // A single request, for the nearest timer: egui already wakes everyone
        // in the same frame, and N requests would be N duplicates of the same request.
        if let Some(p) = proximo {
            self.ctx.request_repaint_after(timer_wait(agora, Some(p)));
        }
    }
}

// The tick rule — `timer_due` and `timer_wait` — used to live here in Portuguese,
// and a second copy lived inside a string in `rxp.rs`. Today it is unified in
// `desenho_comum.rs`, and Preview and the generated application call the SAME function:
// lost ticks are dropped, never queued, and the reason for that is explained there.

// ------------------------------------------------------------------ painting

impl EguiRuntime {
    fn pintar(&self) {
        // The guard measures the DRAWING area, not the surface: in a 20pt
        // strip the surface has width and the artboard has none, and
        // drawing the form inside the ruler itself is worse than not drawing.
        // Painter clipping remains the whole surface, because the ruler
        // must fit within it too.
        let sup = self.superficie();
        let area = self.area_de_desenho();
        if area.width() < 1.0 || area.height() < 1.0 {
            return;
        }
        let p = self
            .ctx
            .layer_painter(egui::LayerId::new(
                egui::Order::Middle,
                egui::Id::new("rxp-design-surface"),
            ))
            .with_clip_rect(sup);

        p.rect_filled(sup, 0.0, SUPERFICIE);
        self.pintar_formulario(&p);
        self.pintar_controles(&p);
        self.pintar_selecao(&p);
        self.pintar_guias(&p);
        self.pintar_marquee(&p);
        self.pintar_selos(&p);
        // Last: the ruler is the canvas frame, and a component dragged
        // beyond the border passes UNDERNEATH it, not over it.
        self.pintar_reguas(&p);
    }

    fn pintar_formulario(&self, p: &egui::Painter) {
        let form = self.na_tela(self.form_rect);
        p.rect_filled(form, 0.0, PAPEL);

        // Grid dots, as in a classic RAD IDE.
        //
        // Each dot is ONE DEVICE PIXEL, not a logical point. It was once
        // `Vec2::splat(1.0)` in points, and with fractional `pixels_per_point`
        // (1.5 on a 150% display) that gives 1.5 pixels: the dot lands on two
        // pixels, half-covered, and final size shifts depending on whether the
        // position lands towards one side or the other. Hundreds of them spaced by 8
        // became a flickering moiré across the entire form — users
        // saw this and asked whether the dots had an effect. They did.
        //
        // One whole pixel, aligned with the physical grid: all identical,
        // without flicker, at any scale.
        let ppp = self.scale.0.max(0.01);
        let um_pixel = 1.0 / ppp;
        let ao_pixel = |v: f32| (v * ppp).round() / ppp;
        let t = self.transformacao();
        // Step is DESIGN units and the loop advances in SCREEN coordinates: without
        // conversion, the grid remained 8 by 8 points while the form doubled in size,
        // and what the grid promises — *this is where the component snaps* — became
        // false the moment someone zoomed.
        let passo = t.medida(self.grade.max(2.0));
        // Grids that do not fit are not drawn.
        if passo >= 2.0 {
            let mut y = form.min.y;
            while y <= form.max.y {
                let mut x = form.min.x;
                while x <= form.max.x {
                    p.rect_filled(
                        ERect::from_min_size(
                            Pos2::new(ao_pixel(x), ao_pixel(y)),
                            Vec2::splat(um_pixel),
                        ),
                        0.0,
                        PONTO_GRADE,
                    );
                    x += passo;
                }
                y += passo;
            }
        }

        self.cursor_das_alcas();

        // The three form handles — E, S, and SE, as the designer accepts.
        //
        // They come from the SAME `crawrad_runtime::alcas` as component handles,
        // not from `form.max.x` as they used to. The reason is not tidiness:
        // `Designer::hit_form_handle` already looked for clicks at
        // `Handle::E.anchor(rect)`, which is `x + w + ALCA_FORA` — 3.5 units
        // OUTSIDE the border. The drawing was 3.5 short of the target, and only
        // did not hurt because the target has 7 margin. At 4x zoom the divergence
        // becomes 14 screen points and starts hurting.
        for (ax, ay) in self.alcas_do_formulario() {
            self.pintar_alca(p, t.ponto_na_tela(ax, ay), ALCA_FORM);
        }
    }

    /// The three form handles — E, S, and SE —, in design units.
    ///
    /// Indices follow the contract of `crawrad_runtime::alcas`, the same as
    /// the designer's `Handle::indice` uses: 3 = E, 5 = S, 4 = SE.
    fn alcas_do_formulario(&self) -> [(f32, f32); 3] {
        let r = self.form_rect;
        let a = crawrad_runtime::handles(r.x, r.y, r.w, r.h);
        [a[3], a[5], a[4]]
    }

    /// A handle square, centered at its corresponding screen point.
    ///
    /// **Handles GROW with zoom**, which is a decision, not an oversight.
    /// Drawing tools often keep handles at a fixed screen size, but here
    /// that would be costly: click resolution is handled by
    /// `Designer::hit_handle`, which operates in DESIGN units and looks for the
    /// handle in `crawrad_runtime::alcas` — the same function this drawing uses.
    /// Fixed-size screen handles would force the designer to divide `ALCA_FORA`
    /// and `handle_size` by the factor, thus **knowing about zoom** — precisely
    /// what ADR 0019 forbids, and how handle positions end up written in two places again.
    ///
    /// What is gained in return is the guarantee that matters: where the handle appears
    /// is where it hits, at any zoom, by construction.
    fn pintar_alca(&self, p: &egui::Painter, centro: Pos2, cor: Color32) {
        let lado = self.transformacao().medida(crawrad_runtime::HANDLE_SIDE);
        p.rect_filled(ERect::from_center_size(centro, Vec2::splat(lado)), 0.0, cor);
    }

    /// The resize arrow when the pointer is over a handle.
    ///
    /// This only existed in RCL — `SetCursor` inside the Win32 subclass
    /// — and was removed with it in ADR 0013. Whatever draws the handle
    /// should display the arrow: they represent the same information, and
    /// separating them made the cursor disappear unnoticed.
    fn cursor_das_alcas(&self) {
        let Some(p) = self.ctx.pointer_latest_pos() else {
            return;
        };
        // The check is performed in DESIGN units, not in screen points.
        //
        // Two reasons, with the second fixing a longstanding defect:
        //
        // 1. the target is `ALCA_ALVO`, a form measurement — on screen it
        //    grows with zoom, along with the handle it covers;
        // 2. what decides whether dragging starts is `Designer::hit_handle`,
        //    comparing in design units with that same margin. Here the margin was
        //    `ALCA + 1.0` = 4.5 points, half of that. The arrow appeared in a
        //    smaller crown than the area that actually hits: the pointer became a
        //    regular arrow yet dragging still worked, an inconsistency
        //    no one reports but everyone feels.
        let (px, py) = self.em_design(p);
        let folga = crawrad_runtime::HANDLE_TARGET / 2.0;
        let pegou = |ax: f32, ay: f32| (px - ax).abs() <= folga && (py - ay).abs() <= folga;

        // Order matters: selected component takes precedence over the form,
        // because its handle is on top.
        for id in self.sel.iter().rev() {
            let Some(c) = self.ctls.get(id) else { continue };
            if !c.visual || c.derived_geometry {
                continue;
            }
            for (i, (ax, ay)) in cantos(c.rect).iter().enumerate() {
                if pegou(*ax, *ay) {
                    self.ctx.set_cursor_icon(seta(EIXO_DAS_ALCAS[i]));
                    return;
                }
            }
        }
        // And the three form handles: E, S, and SE, in `alcas_do_formulario` order.
        for ((ax, ay), eixo) in self.alcas_do_formulario().into_iter().zip([
            crawrad_runtime::Axis::We,
            crawrad_runtime::Axis::Ns,
            crawrad_runtime::Axis::Nwse,
        ]) {
            if pegou(ax, ay) {
                self.ctx.set_cursor_icon(seta(eixo));
                return;
            }
        }
    }

    /// Page captions of a `PageControl`, in order, and which one is
    /// open.
    ///
    /// Reads from `self.ordem_toda`, not the document: `ordem_toda` already comes from
    /// `em_ordem_de_desenho`, so siblings appear in Z-order — the order
    /// of tabs, as in a classic RAD IDE.
    ///
    /// **The open page might not be the one in the document.** In Preview the open
    /// page is the tab the user clicked, stored in `paginas_vivas`; in design the
    /// map is empty (`set_modo` clears it on leaving live mode) and the answer reverts
    /// to `.crawform`. This single-line branch lets Preview switch
    /// pages without altering the document.
    fn paginas_de(&self, dono: &NodeId) -> (Vec<String>, usize) {
        if self.ctls.get(dono).is_none_or(|c| c.class != "PageControl") {
            return (Vec::new(), 0);
        }
        let (paginas, ativa) = paginas_de(&self.ctls, &self.ordem_toda, dono);
        match self.paginas_vivas.get(dono) {
            // Out-of-bounds falls back to the first — the SAME rule as `paginas_de`,
            // which is that of a classic RAD IDE.
            Some(i) if *i < paginas.len() => (paginas, *i),
            _ => (paginas, ativa),
        }
    }

    /// Is this `TabSheet` the currently open page of its `PageControl`?
    ///
    /// Mirrors `crawrad_runtime::e_a_pagina_ativa`, with the only difference that
    /// matters: the page comes from [`EguiRuntime::paginas_de`] above, which in
    /// Preview reports what the user opened. A page without a parent `PageControl`
    /// is an abnormal document, and the honest answer there is *yes* — better for it
    /// to appear so the issue stays visible.
    fn e_a_pagina_aberta(&self, pagina: &NodeId, dono: &NodeId) -> bool {
        if self.ctls.get(dono).is_none_or(|c| c.class != "PageControl") {
            return true;
        }
        let (_, ativa) = self.paginas_de(dono);
        ids_das_paginas(&self.ctls, &self.ordem_toda, dono)
            .get(ativa)
            .is_some_and(|i| i == pagina)
    }

    /// Does the node appear in the **live** form?
    ///
    /// This is [`crawrad_runtime::na_superficie`] evaluated with Preview state
    /// instead of the document, and it must exist because `self.ordem` **does
    /// not work here**: that list is filtered in `open`/`update` by the
    /// DOCUMENT's `active_page`, whereas in Preview the open page is what the
    /// user clicked. With a frozen list, the tab lit up while its
    /// content remained off-screen — which was half of the bug.
    ///
    /// Climbs the entire tree, not just up to the parent: `PageControl` inside
    /// `TabSheet` is the case that previously broke the generator (see `visibility_guard`
    /// in `crawrad-codegen`). A node in an ancestor closed page does not appear,
    /// no matter how open the child page is.
    ///
    /// **And the gate, which is the other half of the rule.** The 07/09 amendment of
    /// ADR 0012
    /// requires two things of every copy of this rule: this note, and a test exercising
    /// the **composite case**. The test is
    /// `a_aba_de_fora_fechada_esconde_a_pagina_de_dentro_inteira` — three
    /// nested `PageControl`s with a button at the bottom. Replacing `return false`
    /// below with `break`, which was the defect fixed in the generator, causes
    /// the frame to paint the inner strip and bottom button over a
    /// CLOSED outer page; without this test, the entire workspace passed green.
    fn no_formulario_vivo(&self, id: &NodeId) -> bool {
        // The node's own `Visible`. Only its own: invisible panel hiding
        // children is another matter, not handled by RXP today — changing this here
        // would alter drawing alongside a tab fix.
        if !self.ctls.get(id).is_some_and(|c| c.visible) {
            return false;
        }
        // **What terminates the ascent is `else { break }` below**, not a
        // root guard: the form is NOT in `ctls`. The map is filled
        // from `em_ordem_de_desenho`, which pushes children and never the
        // root itself — `o_formulario_nao_entra_no_mapa_de_controles`
        // enforces this, because the entire ascent depends on it being true.
        //
        // There used to be a guard `if c.pai == no { break }` intended to prevent
        // infinite loops caused by a root being its own parent. It was removed because
        // it promised two things that do not happen: the root never reaches this loop,
        // and deleting the guard leaves all tests passing without hanging. Worse,
        // it would not catch real cycles: a corrupted `.crawform` where two
        // nodes are parents of each other never has `c.pai == no`. That case hangs before
        // reaching here, inside `crawrad_runtime::in_hidden_page`, which
        // climbs the same way — measured, and tracked as a known gap for both
        // copies at once: fixing only this one would introduce the divergence
        // this file spends entire comments avoiding.
        let mut atual = Some(id.clone());
        while let Some(no) = atual {
            let Some(c) = self.ctls.get(&no) else { break };
            // Inside a non-visual — the `Action` of an `ActionList` — never
            // belonged on the form, neither in design nor live mode.
            if !c.visual {
                return false;
            }
            if c.class == "TabSheet" && !self.e_a_pagina_aberta(&no, &c.parent) {
                return false;
            }
            // a folded CategoryPanel hides what is inside it
            if no != *id
                && c.class == "CategoryPanel"
                && self.marcados_vivos.get(&no).copied().unwrap_or(c.collapsed)
            {
                return false;
            }
            atual = Some(c.parent.clone());
        }
        true
    }

    fn pintar_controles(&self, p: &egui::Painter) {
        // Style comes from PAPEL, not the IDE theme: see the note in the colors
        // block. With dark chrome, inheriting from it put light text over
        // light paper — 1.08:1 measured, and form labels vanished.
        let estilo = estilo_do_papel();
        let form = self.na_tela(self.form_rect);
        // Components extending beyond the form area are clipped by it, as in
        // a classic RAD IDE — children do not paint over the desk.
        //
        // The second clip is `area_de_desenho`, and **today it changes nothing**:
        // with the form at origin, `form ∩ surface` and `form ∩ area` are the
        // same rectangle — both share `max`, and form `min`
        // already falls after the ruler. What protects the ruler today is this
        // clipping by the form itself, plus the ruler being painted last.
        //
        // It remains this way, and the reason is **pan**: with panning the form can
        // begin ABOVE the ruler, where `surface` lets through what `area`
        // clips. Writing the rectangle that respects the ruler costs one
        // word now; discovering later that panning leaked through here costs the same
        // as handles floating outside containers.
        let t = self.transformacao();
        let p = p.with_clip_rect(form.intersect(self.area_de_desenho()));

        for id in &self.ordem {
            let Some(c) = self.ctls.get(id) else { continue };
            if !c.visible {
                continue;
            }
            let mut r = self.na_tela(c.rect);
            if c.class == "Badge"
                && !c.associate.is_empty()
                && let Some(target) = self
                    .ctls
                    .values()
                    .find(|target| target.id == c.associate || target.name == c.associate)
            {
                let target_r = self.na_tela(target.rect);
                r = ERect::from_center_size(target_r.right_top(), r.size());
            }
            if !c.visual {
                self.pintar_nao_visual(&p, id, r);
                continue;
            }
            // The SAME function that clips selection: the clipping rule exists
            // once, rather than twice needing to agree.
            let p = self.pintor_do_filho(&p, c);

            let pages = if c.class == "PageControl" {
                self.paginas_de(id)
            } else {
                (Vec::new(), 0)
            };
            draw_control(&p, r, c, pages, &estilo, t);
        }
    }

    /// Non-visual icon: the badge with initials, and the **name** underneath —
    /// as a classic RAD IDE displays `TActionList` on the form.
    fn pintar_nao_visual(&self, p: &egui::Painter, id: &str, r: ERect) {
        let t = self.transformacao();
        let Some(c) = self.ctls.get(id) else { return };
        let icon = self
            .class_icons
            .get(&c.class)
            .map_or(&[][..], Vec::as_slice);
        let caixa = selo_nao_visual(p, r.min, &c.class, icon, t);
        // The name only on the form, not on the palette tile: here it
        // distinguishes two `ActionList`s having the same badge; there the
        // tile label already identifies it, and repeating it would waste half the height
        // available for the badge to scale.
        p.text(
            Pos2::new(caixa.center().x, caixa.max.y + t.medida(2.0)),
            Align2::CENTER_TOP,
            &c.name,
            t.fonte(10.0),
            NOME_NAO_VISUAL,
        );
    }

    /// Is the node being drawn in this frame?
    ///
    /// `ctls` holds EVERYTHING in the document — including what lives in a
    /// closed page —, because that page's tab needs its caption.
    /// `ordem` is what actually goes to the screen.
    ///
    /// This exists because separating the two lists opened a gap: selection and
    /// badges iterated over `ctls` and began painting handles for components not
    /// on screen. Eight handles floating over the open page, with nothing
    /// underneath, and `hit_handle` rejecting drags — the UI lying.
    ///
    /// There are SIX paths traversing the tree, not four: `open`, `update`,
    /// `hit`, `hit_handle`, and these two painting paths.
    fn desenhavel(&self, id: &NodeId) -> bool {
        self.ordem.iter().any(|i| i == id)
    }

    /// The painter clipped by the PARENT, just as the component is clipped.
    ///
    /// Container children are clipped by their parent — in Win32 this came for free
    /// with `WS_CLIPCHILDREN`; here the adapter clips. What was missing was for
    /// SELECTION to obey the same rule: dragging a `CheckBox` out of a
    /// `GroupBox` clipped the component while leaving **handles floating outside**,
    /// over nothing — eight blue squares where no component can be grabbed.
    ///
    /// Clipping handles together is honest: where a component does not appear, there is
    /// nothing to grab. The visible part retains its handles, and the rest
    /// remains selectable via the `Structure` tree and arrow keys.
    fn pintor_do_filho(&self, p: &egui::Painter, c: &Ctl) -> egui::Painter {
        recortar_pelo_pai(p, c, self.transformacao())
    }

    fn pintar_selecao(&self, p: &egui::Painter) {
        let t = self.transformacao();
        for id in &self.sel {
            if !self.desenhavel(id) {
                continue;
            }
            let Some(c) = self.ctls.get(id) else { continue };
            let p = &self.pintor_do_filho(p, c);
            let r = self.na_tela(c.rect);
            // Non-visuals do not resize: the icon has a fixed size, so
            // it gets a frame instead of handles — the same rule as the designer.
            if !c.visual {
                let caixa = ERect::from_min_size(r.min, Vec2::splat(t.medida(ICONE)));
                p.rect_stroke(
                    caixa.expand(1.0),
                    3.0,
                    Stroke::new(1.0, ALCA_COMPONENTE),
                    StrokeKind::Outside,
                );
                continue;
            }
            if c.derived_geometry {
                continue;
            }
            // Handles originate from the DESIGN rectangle and only then become screen coordinates:
            // the exact calculation `Designer::hit_handle` performs on the other side.
            for (ax, ay) in cantos(c.rect) {
                self.pintar_alca(p, t.ponto_na_tela(ax, ay), ALCA_COMPONENTE);
            }
        }
    }

    fn pintar_guias(&self, p: &egui::Painter) {
        for g in &self.guias {
            let cor = match g.kind {
                GuideKind::Component => GUIA_COMPONENTE,
                GuideKind::Text => GUIA_TEXTO,
                GuideKind::Spacing => GUIA_ESPACO,
            };
            // Thickness > 0 is a rectangle, which is how spacing gaps
            // appear as bands rather than lines.
            if g.thickness > 0.0 {
                let r = if g.vertical {
                    Rect::new(
                        g.pos - g.thickness / 2.0,
                        g.from,
                        g.thickness,
                        g.to - g.from,
                    )
                } else {
                    Rect::new(
                        g.from,
                        g.pos - g.thickness / 2.0,
                        g.to - g.from,
                        g.thickness,
                    )
                };
                let faixa = self.na_tela(r);
                p.rect_filled(faixa, 0.0, cor.gamma_multiply(0.35));
                p.rect_stroke(faixa, 0.0, Stroke::new(1.0, cor), StrokeKind::Inside);
                continue;
            }
            let (a, b) = if g.vertical {
                (
                    self.ponto_na_tela(g.pos, g.from),
                    self.ponto_na_tela(g.pos, g.to),
                )
            } else {
                (
                    self.ponto_na_tela(g.from, g.pos),
                    self.ponto_na_tela(g.to, g.pos),
                )
            };
            p.line_segment([a, b], Stroke::new(1.0, cor));
        }
    }

    fn pintar_marquee(&self, p: &egui::Painter) {
        let Some(m) = self.marquee else { return };
        let r = self.na_tela(m);
        p.rect_filled(r, 0.0, ALCA_COMPONENTE.gamma_multiply(0.15));
        p.rect_stroke(
            r,
            0.0,
            Stroke::new(1.0, ALCA_COMPONENTE),
            StrokeKind::Inside,
        );
    }

    fn pintar_selos(&self, p: &egui::Painter) {
        let t = self.transformacao();
        for (id, texto) in &self.selos {
            if !self.desenhavel(id) {
                continue;
            }
            let Some(c) = self.ctls.get(id) else { continue };
            // Tab order badge follows the same rule as handles: it
            // originates at the component corner, which may lie outside the parent.
            let p = &self.pintor_do_filho(p, c);
            let r = self.na_tela(c.rect);
            // Badges scale with the form for the same reason handles do: they
            // reside at a component's corner, and a fixed-size badge over
            // a 24-point `Edit` at 400% would cover only an eighth of what it labels.
            let caixa = ERect::from_min_size(r.min, Vec2::new(t.medida(16.0), t.medida(13.0)));
            p.rect_filled(caixa, 2.0, SELO);
            p.text(
                caixa.center(),
                Align2::CENTER_CENTER,
                texto,
                FontId::monospace(t.medida(9.0)),
                Color32::WHITE,
            );
        }
    }

    /// Both rulers: one on top, one on the left, and the corner square.
    ///
    /// **It has no arithmetic of its own.** Every tick comes from
    /// `ponto_na_tela(valor, valor)` — the same function that positions
    /// components. A ruler recalculating "what one point is worth" would be
    /// the same rule written twice, and a ruler disagreeing with the drawing is
    /// worse than no ruler at all: it is the very tool used to verify that the
    /// drawing is correct.
    fn pintar_reguas(&self, p: &egui::Painter) {
        // Preview has no artboard to measure — see `transformacao`.
        if self.modo.is_live() {
            return;
        }
        let sup = self.superficie();
        let area = self.area_de_desenho();
        let t = self.transformacao();

        let topo = ERect::from_min_max(sup.min, Pos2::new(sup.max.x, sup.min.y + REGUA));
        let lado = ERect::from_min_max(sup.min, Pos2::new(sup.min.x + REGUA, sup.max.y));
        p.rect_filled(topo, 0.0, REGUA_FUNDO);
        p.rect_filled(lado, 0.0, REGUA_FUNDO);
        let borda = Stroke::new(1.0, REGUA_TRACO.gamma_multiply(0.6));
        p.line_segment(
            [
                Pos2::new(area.min.x, topo.max.y),
                Pos2::new(sup.max.x, topo.max.y),
            ],
            borda,
        );
        p.line_segment(
            [
                Pos2::new(lado.max.x, area.min.y),
                Pos2::new(lado.max.x, sup.max.y),
            ],
            borda,
        );

        let passo = passo_da_regua(t.escala);
        // Minor ticks are a fifth of the step, and disappear when crowded: five
        // ticks across two screen points become a solid line, measuring nothing
        // and obscuring numbered ticks.
        let miudo = passo / 5.0;
        let com_miudo = t.medida(miudo) >= 5.0;
        let fonte = FontId::monospace(9.0);

        // Horizontal. `n` counts steps from the form origin, where
        // the ruler marks zero — as in a classic RAD IDE, where the coordinate shown
        // by the Object Inspector is relative to the form, not the window.
        let mut n = 0u32;
        loop {
            let v = n as f32 * miudo;
            let x = t.ponto_na_tela(v, 0.0).x;
            if x > area.max.x {
                break;
            }
            let grande = n.is_multiple_of(5);
            if x >= area.min.x && (grande || com_miudo) {
                let de = topo.max.y - if grande { REGUA * 0.55 } else { REGUA * 0.3 };
                p.line_segment(
                    [Pos2::new(x, de), Pos2::new(x, topo.max.y)],
                    Stroke::new(1.0, REGUA_TRACO),
                );
                if grande {
                    p.text(
                        Pos2::new(x + 2.0, topo.min.y + 1.0),
                        Align2::LEFT_TOP,
                        format!("{v:.0}"),
                        fonte.clone(),
                        REGUA_TEXTO,
                    );
                }
            }
            n += 1;
        }

        // Vertical. Numbers appear horizontally, not rotated: rotating would
        // require a galley per tick just to save width the ruler already has.
        let mut n = 0u32;
        loop {
            let v = n as f32 * miudo;
            let y = t.ponto_na_tela(0.0, v).y;
            if y > area.max.y {
                break;
            }
            let grande = n.is_multiple_of(5);
            if y >= area.min.y && (grande || com_miudo) {
                let de = lado.max.x - if grande { REGUA * 0.55 } else { REGUA * 0.3 };
                p.line_segment(
                    [Pos2::new(de, y), Pos2::new(lado.max.x, y)],
                    Stroke::new(1.0, REGUA_TRACO),
                );
                if grande {
                    p.text(
                        Pos2::new(lado.min.x + 1.0, y + 1.0),
                        Align2::LEFT_TOP,
                        format!("{v:.0}"),
                        fonte.clone(),
                        REGUA_TEXTO,
                    );
                }
            }
            n += 1;
        }

        // Where the pointer is, on both rulers.
        //
        // This turns the ruler from decoration into an instrument: aligning a
        // component "at 120" requires seeing 120 move with your hand. Without
        // this, the ruler only shows where stationary items are.
        if let Some(m) = self.ctx.pointer_latest_pos()
            && area.contains(m)
        {
            let c = Stroke::new(1.0, REGUA_PONTEIRO);
            p.line_segment([Pos2::new(m.x, topo.min.y), Pos2::new(m.x, topo.max.y)], c);
            p.line_segment([Pos2::new(lado.min.x, m.y), Pos2::new(lado.max.x, m.y)], c);
        }
    }
}

/// Centered text with a classic RAD IDE's `&` turned into an underline, as Win32 does.
///
/// Free-standing function, not a method: it never reads `self`, and the palette needs it
/// to draw the tile of a `Button` without a live runtime around.
fn texto_centrado(p: &egui::Painter, r: ERect, texto: &str, fonte: FontId, cor: Color32) {
    let limpo = sem_acelerador(texto);
    let galeria = p.layout_no_wrap(limpo.clone(), fonte.clone(), cor);
    let pos = Pos2::new(
        r.center().x - galeria.size().x / 2.0,
        r.center().y - galeria.size().y / 2.0,
    );
    let base = pos.y + galeria.size().y;
    p.galley(pos, galeria, cor);

    if let Some(i) = indice_do_acelerador(texto) {
        let antes = p.layout_no_wrap(limpo[..i].to_string(), fonte.clone(), cor);
        let ate = p.layout_no_wrap(
            limpo[..i + limpo[i..].chars().next().map_or(0, char::len_utf8)].to_string(),
            fonte,
            cor,
        );
        let y = base - 1.5;
        p.line_segment(
            [
                Pos2::new(pos.x + antes.size().x, y),
                Pos2::new(pos.x + ate.size().x, y),
            ],
            Stroke::new(1.0, cor),
        );
    }
}

/// The **live** tree: each row responds to clicks, and nodes with children
/// collapse and expand. It is the same drawing as `draw_items`, but with
/// responsiveness — the difference between a portrait of the component and the component itself.
///
/// `nome` is what distinguishes TWO trees on the same form, and it is not
/// decorative: `ui.id()` **does not change from one sibling to another**. A child `Ui`
/// is born with `pai.id().with("child")` — a hardcoded word (`egui-0.36.1`,
/// `src/ui.rs:251`, and `Ui::id()` on line 344 returns this `stable_id`, not
/// the `unique_id` that exists beside it). Without `nome`, clicking a row in the
/// second tree selected a row in the first. It is the same bug that the
/// `page_control` in `desenho_comum.rs` had, and the same fix — here and in the
/// `tree` emitted by the generator, because they are three copies of the same mistake.
fn arvore_viva(
    ui: &mut egui::Ui,
    nome: &str,
    r: ERect,
    itens: &[crawrad_ir::Item],
    selecao: &mut Vec<usize>,
    fechados: &mut std::collections::BTreeSet<Vec<usize>>,
) {
    const LINHA: f32 = 18.0;
    const RECUO: f32 = 15.0;
    let mut y = r.min.y + 2.0;
    let mut abrir_fechar: Option<Vec<usize>> = None;
    let mut escolher: Option<Vec<usize>> = None;

    // Stack instead of recursion: drawing order matches reading order, and the
    // path travels along with the item.
    let mut pilha: Vec<(&crawrad_ir::Item, Vec<usize>)> = itens
        .iter()
        .enumerate()
        .map(|(i, it)| (it, vec![i]))
        .rev()
        .collect();

    while let Some((item, caminho)) = pilha.pop() {
        if y + LINHA > r.max.y {
            break;
        }
        let nivel = caminho.len() - 1;
        let x = r.min.x + 3.0 + nivel as f32 * RECUO;
        let linha = ERect::from_min_size(
            Pos2::new(r.min.x + 1.0, y),
            Vec2::new(r.width() - 2.0, LINHA),
        );
        let tem_filhos = !item.children.is_empty();
        let recolhido = fechados.contains(&caminho);

        // The expand marker is its own hit target: clicking it opens and closes
        // without affecting selection, as in Windows. The row target starts
        // AFTER it: egui delivers clicks to the last registered target, and
        // the whole row on top of the marker swallowed the click.
        let mut row_target = linha;
        if tem_filhos {
            let marca = ERect::from_center_size(
                Pos2::new(x + 5.0, y + LINHA / 2.0),
                Vec2::splat(LINHA - 4.0),
            );
            let resp = ui.interact(
                marca,
                ui.id().with(("exp", nome, &caminho)),
                egui::Sense::click(),
            );
            ui.painter().text(
                marca.center(),
                Align2::CENTER_CENTER,
                if recolhido { "+" } else { "\u{2212}" },
                FontId::proportional(12.0),
                ui.visuals().weak_text_color(),
            );
            if resp.clicked() {
                abrir_fechar = Some(caminho.clone());
            }
            row_target.min.x = marca.max.x;
        }

        let resp = ui.interact(
            row_target,
            ui.id().with(("item", nome, &caminho)),
            egui::Sense::click(),
        );
        let escolhido = selecao.as_slice() == caminho.as_slice();
        if escolhido {
            ui.painter()
                .rect_filled(linha, 2.0, ui.visuals().selection.bg_fill);
        } else if resp.hovered() {
            ui.painter()
                .rect_filled(linha, 2.0, ui.visuals().widgets.hovered.bg_fill);
        }
        ui.painter().text(
            Pos2::new(x + 14.0, y + LINHA / 2.0),
            Align2::LEFT_CENTER,
            &item.text,
            FontId::proportional(12.5),
            if escolhido {
                ui.visuals().selection.stroke.color
            } else {
                ui.visuals().text_color()
            },
        );
        if resp.clicked() {
            escolher = Some(caminho.clone());
        }

        y += LINHA;
        if !recolhido {
            for (i, filho) in item.children.iter().enumerate().rev() {
                let mut c = caminho.clone();
                c.push(i);
                pilha.push((filho, c));
            }
        }
    }

    if let Some(c) = escolher {
        *selecao = c;
    }
    if let Some(c) = abrir_fechar
        && !fechados.remove(&c)
    {
        fechados.insert(c);
    }
}

/// The items of a tree collection, indented, with the marker for nodes with
/// children. Free-standing function because BOTH paths need it: design painting
/// and the live form.
fn draw_items(
    p: &egui::Painter,
    r: ERect,
    itens: &[crawrad_ir::Item],
    cor: Color32,
    t: Transformacao,
) {
    // Row height and indentation are measurements of the CONTROL, not of the chrome: it is
    // the `TreeView` drawn inside the form. Without conversion, zooming into
    // the TreeView left items at their default height, crammed into the
    // top corner of a box four times larger.
    let row_height = t.medida(18.0);
    let recuo = t.medida(15.0);
    let p = p.with_clip_rect(r.shrink(1.0));
    let fonte = t.fonte(12.0);
    let mut y = r.min.y + t.medida(3.0);

    // Stack instead of recursion: drawing order matches reading order, and
    // depth travels along with the item.
    let mut pilha: Vec<(&crawrad_ir::Item, usize)> =
        itens.iter().rev().map(|i| (i, 0usize)).collect();
    while let Some((item, nivel)) = pilha.pop() {
        if y > r.max.y {
            break;
        }
        let x = r.min.x + t.medida(3.0) + nivel as f32 * recuo;
        if !item.children.is_empty() {
            // The same `-` as the expanded TreeView in Windows.
            p.text(
                Pos2::new(x, y + row_height / 2.0),
                Align2::LEFT_CENTER,
                "\u{2212}",
                fonte.clone(),
                cor.gamma_multiply(0.7),
            );
        }
        p.text(
            Pos2::new(x + t.medida(14.0), y + row_height / 2.0),
            Align2::LEFT_CENTER,
            &item.text,
            fonte.clone(),
            cor,
        );
        y += row_height;
        for filho in item.children.iter().rev() {
            pilha.push((filho, nivel + 1));
        }
    }
}

/// The arrow of the closed `ComboBox` — the same triangle drawn by Windows.
fn seta_para_baixo(p: &egui::Painter, r: ERect, cor: Color32, t: Transformacao) {
    let cx = r.max.x - t.medida(12.0);
    let cy = r.center().y;
    p.add(egui::Shape::convex_polygon(
        vec![
            Pos2::new(cx - t.medida(4.0), cy - t.medida(2.0)),
            Pos2::new(cx + t.medida(4.0), cy - t.medida(2.0)),
            Pos2::new(cx, cy + t.medida(3.0)),
        ],
        cor,
        egui::Stroke::NONE,
    ));
}

/// The designer axis translated to the egui cursor arrow. This is the translation
/// each adapter does on its own — four lines.
fn seta(e: crawrad_runtime::Axis) -> egui::CursorIcon {
    match e {
        crawrad_runtime::Axis::Nwse => egui::CursorIcon::ResizeNwSe,
        crawrad_runtime::Axis::Nesw => egui::CursorIcon::ResizeNeSw,
        crawrad_runtime::Axis::We => egui::CursorIcon::ResizeHorizontal,
        crawrad_runtime::Axis::Ns => egui::CursorIcon::ResizeVertical,
    }
}

/// The axis of each handle, in the SAME order returned by `cantos`.
///
/// Clockwise starting from the top-left corner, like the designer's `Handle`.
const EIXO_DAS_ALCAS: [crawrad_runtime::Axis; 8] = [
    crawrad_runtime::Axis::Nwse,
    crawrad_runtime::Axis::Ns,
    crawrad_runtime::Axis::Nesw,
    crawrad_runtime::Axis::We,
    crawrad_runtime::Axis::Nwse,
    crawrad_runtime::Axis::Ns,
    crawrad_runtime::Axis::Nesw,
    crawrad_runtime::Axis::We,
];

/// The four colors of the tab strip.
///
/// Grouped in a type because design and live paint the SAME strip with different
/// palettes — design uses the paper colors, live uses the theme colors — and four
/// loose `Color32` in the call swap order without the compiler complaining.
#[derive(Clone, Copy)]
struct CoresDaAba {
    fundo: Color32,
    fundo_inativa: Color32,
    borda: Color32,
    texto: Color32,
}

/// The painter clipped by the PARENT. See [`EguiRuntime::pintor_do_filho`] for the
/// rationale; here is the calculation, so the palette tile obeys the same rule
/// without a live runtime around.
fn recortar_pelo_pai(p: &egui::Painter, c: &Ctl, t: Transformacao) -> egui::Painter {
    match c.clip {
        Some(pai) => p.with_clip_rect(p.clip_rect().intersect(t.na_tela(pai))),
        None => p.clone(),
    }
}

/// The **ids** of the pages in a `PageControl`, in tab order.
///
/// Reads from drawing order, not from the document: it already comes from
/// `em_ordem_de_desenho`, so siblings appear in Z-order — which is the tab
/// order, like in a classic RAD IDE.
///
/// Separated from [`paginas_de`] because two different questions need this
/// SAME list: *which caption goes on each tab* and *is this page the open one*. The
/// second came with clicking a tab in Preview, and writing it again would be
/// a second page ordering, free to disagree with the first.
fn ids_das_paginas(
    ctls: &BTreeMap<NodeId, Ctl>,
    ordem_toda: &[NodeId],
    dono: &NodeId,
) -> Vec<NodeId> {
    if ctls.get(dono).is_none_or(|c| c.class != "PageControl") {
        return Vec::new();
    }
    ordem_toda
        .iter()
        .filter(|i| {
            ctls.get(*i)
                .is_some_and(|c| c.class == "TabSheet" && &c.parent == dono)
        })
        .cloned()
        .collect()
}

/// The captions of the pages in a `PageControl`, in order, and which one is open.
///
/// Reads from drawing order, not from the document: it already comes from
/// `em_ordem_de_desenho`, so siblings appear in Z-order — which is the tab
/// order, like in a classic RAD IDE.
fn paginas_de(
    ctls: &BTreeMap<NodeId, Ctl>,
    ordem_toda: &[NodeId],
    dono: &NodeId,
) -> (Vec<String>, usize) {
    let paginas: Vec<String> = ids_das_paginas(ctls, ordem_toda, dono)
        .iter()
        .filter_map(|i| ctls.get(i))
        .map(|c| sem_acelerador(&c.text))
        .collect();
    // Out-of-bounds index falls back to the FIRST page, not the last.
    //
    // This already disagreed with itself in three places: here the highlighted tab was the
    // last, `crawrad_runtime::e_a_pagina_ativa` showed the content of the first,
    // and generated code showed nothing. Result: the "Notes" tab painted
    // as selected and "Data" content underneath. There is only one rule, and it is that of
    // a classic RAD IDE.
    let ativa = ctls.get(dono).map(|c| c.pagina_ativa).unwrap_or(0);
    let ativa = if ativa < paginas.len() { ativa } else { 0 };
    (paginas, ativa)
}

/// The badge of a non-visual component: the square with the class initials.
///
/// Returns the drawn box, for anyone who wants to write something
/// underneath it.
///
/// Separated from the name on purpose: the badge is the **drawing** of the component, and the
/// name is the identification of an instance. The palette tile wants the
/// former and not the latter — a `Timer` has no face, and its face here is
/// this very one, not a second one invented for the palette.
fn selo_nao_visual(
    p: &egui::Painter,
    canto: Pos2,
    classe: &str,
    icon: &[crawrad_ir::icon::IconStep],
    t: Transformacao,
) -> ERect {
    // `ICONE` is a design measurement — it is the same 28 that `crawrad_registry` gives
    // as the node size, and it is how `Designer::hit` finds the click. Fixed in
    // screen points, the drawn square and the area selecting ActionList
    // drifted apart with each zoom step.
    let caixa = ERect::from_min_size(canto, Vec2::splat(t.medida(ICONE)));
    p.rect_filled(caixa, 3.0, NAO_VISUAL_FUNDO);
    p.rect_stroke(
        caixa,
        3.0,
        Stroke::new(1.0, NAO_VISUAL_BORDA),
        StrokeKind::Inside,
    );
    // The class icon — the palette tile's drawing — and the initials only
    // for a class that has none, the same rule as the Makepad canvas.
    let side = t.medida(16.0).min(caixa.width()).round();
    if !icon.is_empty() && side >= 4.0 {
        let px = side as u32;
        let origin = (caixa.center() - Vec2::splat(side * 0.5)).round();
        let coverage = crawrad_ir::icon::rasterize(icon, crawrad_ir::icon::STROKE * 0.5, px);
        for (row, line) in coverage.chunks(px as usize).enumerate() {
            for (col, a) in line.iter().enumerate() {
                let a = crawrad_ir::icon::ink_alpha(*a);
                if a <= 0.0 {
                    continue;
                }
                let pixel = ERect::from_min_size(
                    origin + Vec2::new(col as f32, row as f32),
                    Vec2::splat(1.0),
                );
                p.rect_filled(pixel, 0.0, NAO_VISUAL_TEXTO.gamma_multiply(a));
            }
        }
        return caixa;
    }
    let iniciais: String = classe.chars().filter(|c| c.is_uppercase()).collect();
    p.text(
        caixa.center(),
        Align2::CENTER_CENTER,
        iniciais,
        t.fonte(11.0),
        NAO_VISUAL_TEXTO,
    );
    caixa
}

/// The drawing of ONE class, without a runtime around.
///
/// This is the `match` that used to be the body of [`EguiRuntime::pintar_controles`], and extracting it
/// is what allows the **palette** to draw a `Button` tile with this
/// exact same code. A separately drawn icon would mean the same rule written in two
/// places — rule 8 of the study, which here already cost the `Memo` drawing
/// being deleted under a green test suite.
///
/// `abas` arrives precomputed because the pages of a `PageControl` live in its
/// siblings in the document, and this function knows nothing of documents: whoever knows
/// queries [`paginas_de`] before calling.
fn draw_control(
    p: &egui::Painter,
    r: ERect,
    c: &Ctl,
    abas: (Vec<String>, usize),
    estilo: &egui::Style,
    t: Transformacao,
) {
    // **Nothing paints outside its own box.** The clipping lives HERE, not in
    // each `match` arm, because an arm that forgot would recreate the exact
    // bug that prompted this line: a 90-point `Memo` with twenty lines
    // inside spilled its text over the neighboring `ListBox` and outside the
    // form. It was like that in almost every component — `Edit`, `Label`,
    // `Button`, `CheckBox`, `Panel`, `GroupBox`, `ListBox`, `ComboBox` —, and the
    // only one spared was `TreeView`, because `draw_items` already
    // clipped on its own. One rule in thirteen places is thirteen chances
    // to make a mistake; here it is one place, and the 17th component is born with it.
    //
    // The cut is **without scrollbars**, and is the visual contract of a classic RAD IDE: a
    // `TMemo` with thirty lines and room for five shows five in the designer and
    // cuts the rest. Scrolling is an action of the form user, and in design the
    // action belongs to the designer. In the LIVE form the answer is different — there the `Memo`
    // actually scrolls, because there it is an edit control.
    //
    // Left out on purpose is what is NOT the component: the selection handle
    // (which is drawn half inside and half outside the border, and clipping it would leave
    // half a handle) and the non-visual badge (which has the name written BELOW the
    // icon, as in a classic RAD IDE). Both are painted through another path.
    //
    // And clipping uses the rectangle **plus [`META_DA_BORDA`]**, which is the fixed first
    // version of this. Clipping to the exact `r` ate the frame: a 1px stroke
    // drawn ON TOP of the border has half outside it, and
    // that half was lost in clipping. `GroupBox` was the one that appeared on screen —
    // it draws its frame with `line_segment` on the border axis, and the
    // right side and bottom line vanished —, but the damage affects all:
    // `epaint` places the antialiasing gradient half a point outside the
    // contour, and without this margin every frame becomes a hair thinner and harsher.
    //
    // egui does the same calculation for the same reason, discovered by the reviewer:
    // `widgets/text_edit/builder.rs:765` paints at
    // `ui.painter_at(text_clip_rect.expand(1.0))`.
    //
    // The margin is **half a border, not one point**: it is exactly where the
    // contour lives, and not a tenth beyond. Both sides are enforced by
    // `nenhum_componente_pinta_fora_da_propria_caixa` (which uses this same
    // number as tolerance) and `toda_moldura_sobrevive_ao_proprio_recorte`.
    let p = &p.with_clip_rect(p.clip_rect().intersect(r.expand(META_DA_BORDA)));
    let w = if c.enabled {
        estilo.visuals.widgets.inactive
    } else {
        estilo.visuals.widgets.noninteractive
    };
    let cor_texto = if c.enabled {
        estilo.visuals.text_color()
    } else {
        estilo.visuals.weak_text_color()
    };
    // The form font undergoes transformation like any
    // other measurement: a button twice the size with its caption
    // at the default size is not an enlarged button, it is a wrong
    // button. `t.fonte` reconstructs the `FontId` at the requested size and
    // egui relayouts, so the glyph remains crisp at 400%.
    let fonte = t.fonte(14.0);

    match c.class.as_str() {
        "Image" => {
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(
                r,
                2.0,
                egui::Stroke::new(1.0, egui::Color32::from_rgb(0xB0, 0xB0, 0xB8)),
                StrokeKind::Inside,
            );
            let tem_imagem = c.picture.as_ref().is_some_and(|pic| !pic.is_empty());
            if tem_imagem {
                let icone_rect = ERect::from_center_size(r.center(), Vec2::splat(t.medida(24.0)));
                p.rect_stroke(
                    icone_rect,
                    2.0,
                    egui::Stroke::new(1.5, egui::Color32::from_rgb(0x4A, 0x88, 0xC7)),
                    StrokeKind::Inside,
                );
                p.text(
                    r.center(),
                    Align2::CENTER_CENTER,
                    "🖼",
                    t.fonte(16.0),
                    egui::Color32::from_rgb(0x4A, 0x88, 0xC7),
                );
            } else {
                let centro = r.center();
                p.text(
                    centro,
                    Align2::CENTER_CENTER,
                    "🖼",
                    t.fonte(18.0),
                    CTL_TEXTO_FRACO,
                );
            }
        }
        "Grid" => {
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
            let col_w = t.medida(c.default_col_width).max(10.0);
            let row_h = t.medida(c.default_row_height).max(10.0);
            let header_h = (c.fixed_rows as f32 * row_h).min(r.height());
            if header_h > 0.0 {
                let header_rect = ERect::from_min_size(r.min, Vec2::new(r.width(), header_h));
                p.rect_filled(header_rect, 0.0, estilo.visuals.faint_bg_color);
            }
            let header_w = (c.fixed_cols as f32 * col_w).min(r.width());
            if header_w > 0.0 {
                let fixed_col_rect = ERect::from_min_size(r.min, Vec2::new(header_w, r.height()));
                p.rect_filled(fixed_col_rect, 0.0, estilo.visuals.faint_bg_color);
            }
            if c.grid_lines {
                let mut x = r.min.x;
                for _ in 0..c.col_count {
                    x += col_w;
                    if x >= r.max.x {
                        break;
                    }
                    p.line_segment(
                        [Pos2::new(x, r.min.y), Pos2::new(x, r.max.y)],
                        Stroke::new(1.0, w.bg_stroke.color),
                    );
                }
                let mut y = r.min.y;
                for _ in 0..c.row_count {
                    y += row_h;
                    if y >= r.max.y {
                        break;
                    }
                    p.line_segment(
                        [Pos2::new(r.min.x, y), Pos2::new(r.max.x, y)],
                        Stroke::new(1.0, w.bg_stroke.color),
                    );
                }
            }
        }
        "Button" | "BitBtn" | "SpeedButton" => {
            p.rect_filled(r, 4.0, w.bg_fill);
            p.rect_stroke(r, 4.0, w.bg_stroke, StrokeKind::Inside);
            texto_centrado(p, r, &c.text, fonte, cor_texto);
        }
        "ToolBar" => {
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
        }
        "StatusBar" => {
            p.rect_filled(r, 0.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 0.0, w.bg_stroke, StrokeKind::Inside);
            if !c.text.is_empty() {
                p.text(
                    Pos2::new(r.min.x + 4.0, r.center().y),
                    Align2::LEFT_CENTER,
                    sem_acelerador(&c.text),
                    fonte,
                    cor_texto,
                );
            }
        }
        "ProgressBar" => {
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
            if c.posicao_scroll > 0.0 {
                let frac = (c.posicao_scroll / 100.0).clamp(0.0, 1.0);
                let bar = ERect::from_min_size(r.min, Vec2::new(r.width() * frac, r.height()));
                p.rect_filled(bar, 2.0, estilo.visuals.selection.bg_fill);
            }
        }
        "TrackBar" => {
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
            let mid_y = r.center().y;
            let trilho = ERect::from_min_max(
                Pos2::new(r.min.x + 4.0, mid_y - 2.0),
                Pos2::new(r.max.x - 4.0, mid_y + 2.0),
            );
            p.rect_filled(trilho, 1.0, w.bg_stroke.color);
            let frac = (c.posicao_scroll / 100.0).clamp(0.0, 1.0);
            let thumb_x = r.min.x + 4.0 + (r.width() - 16.0).max(0.0) * frac;
            let thumb = ERect::from_min_size(Pos2::new(thumb_x, mid_y - 8.0), Vec2::new(8.0, 16.0));
            p.rect_filled(thumb, 2.0, estilo.visuals.selection.bg_fill);
        }
        "PaintBox" => {
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
            let exibido = if !c.text.is_empty() { &c.text } else { &c.name };
            texto_centrado(p, r, exibido, fonte, cor_texto);
        }
        "Card" => {
            paint_card(
                p,
                r,
                &sem_acelerador(&c.text),
                &c.subtitle,
                t.medida(c.corner_radius),
                c.elevation,
                c.show_header,
                &estilo.visuals,
                fonte.clone(),
            );
        }
        "Drawer" => {
            let caption = sem_acelerador(&c.text);
            let displayed_caption = if !c.opened {
                format!("{caption} (closed)")
            } else {
                caption
            };
            paint_drawer(
                p,
                r,
                &displayed_caption,
                &c.side,
                c.show_header,
                false,
                &estilo.visuals,
                fonte.clone(),
            );
            if !c.opened {
                draw_dashed_rect(
                    p,
                    r,
                    estilo.visuals.widgets.noninteractive.bg_stroke,
                    t.medida(4.0),
                    t.medida(4.0),
                );
            }
        }
        "CategoryPanel" => {
            paint_category(
                p,
                r,
                &sem_acelerador(&c.text),
                c.collapsed,
                fonte.clone(),
                &estilo.visuals,
                t.medida(24.0),
            );
        }
        "ListView" => {
            let cols: Vec<(&str, f32, &str)> = c
                .columns_info
                .iter()
                .map(|(cap, w, align)| (cap.as_str(), t.medida(*w as f32), align.as_str()))
                .collect();
            let items: Vec<(&str, Vec<&str>)> = c
                .itens
                .iter()
                .map(|it| {
                    let sub: Vec<&str> = it.children.iter().map(|c| c.text.as_str()).collect();
                    (it.text.as_str(), sub)
                })
                .collect();
            let items_slices: Vec<(&str, &[&str])> = items
                .iter()
                .map(|(cap, sub)| (*cap, sub.as_slice()))
                .collect();
            paint_list_view(
                p,
                r,
                &cols,
                &items_slices,
                c.escolhido,
                c.row_select,
                c.grid_lines,
                c.show_column_headers,
                &estilo.visuals,
                fonte.clone(),
            );
        }
        "Bevel" => {
            paint_bevel(
                p,
                r,
                &c.bevel_shape,
                c.bevel_style == "Raised",
                &estilo.visuals,
            );
            if c.bevel_shape == "Spacer" {
                p.rect_stroke(
                    r,
                    0.0,
                    Stroke::new(1.0, w.bg_stroke.color.gamma_multiply(0.4)),
                    StrokeKind::Inside,
                );
            }
        }
        "Badge" => {
            if let Some(badge_str) = badge_text(&c.text, c.count, c.max_count) {
                let h = r.height();
                let w = if badge_str.chars().count() == 1 {
                    h
                } else {
                    r.width().max(h)
                };
                let badge_rect = ERect::from_center_size(r.center(), Vec2::new(w, h));
                let effective_kind = if c.count > 0 && (c.kind == "bkPrimary" || c.kind.is_empty())
                {
                    "bkDanger"
                } else {
                    &c.kind
                };
                let mut painter = p.clone();
                if !c.enabled {
                    painter.multiply_opacity(0.5);
                }
                paint_badge(
                    &painter,
                    badge_rect,
                    &badge_str,
                    effective_kind,
                    c.outline,
                    &estilo.visuals,
                    t.fonte(11.0),
                );
            }
        }
        "Chips" => {
            let widths: Vec<f32> = c
                .itens
                .iter()
                .map(|it| t.medida(chip_width(&it.text, c.removable)))
                .collect();
            let chip_h = t.medida(CHIP_HEIGHT);
            let chip_gap = t.medida(CHIP_GAP);
            let positions = chips_layout(&widths, r.width(), chip_h, chip_gap);
            let font = t.fonte(12.0);
            let mut painter = p.clone();
            if !c.enabled {
                painter.multiply_opacity(0.5);
            }
            let sel = c.escolhido;
            for (i, (&(x, y), it)) in positions.iter().zip(c.itens.iter()).enumerate() {
                let w = widths[i];
                let chip_rect = ERect::from_min_size(
                    egui::pos2(r.min.x + x, r.min.y + y),
                    egui::vec2(w, chip_h),
                );
                if chip_rect.max.y > r.max.y {
                    break;
                }
                let is_sel = sel == Some(i);
                paint_chip(
                    &painter,
                    chip_rect,
                    &it.text,
                    is_sel,
                    c.removable,
                    &estilo.visuals,
                    font.clone(),
                );
            }
        }
        "Breadcrumb" => {
            let item_strs: Vec<&str> = c.itens.iter().map(|it| it.text.as_str()).collect();
            let font = t.fonte(12.0);
            let mut painter = p.clone();
            if !c.enabled {
                painter.multiply_opacity(0.5);
            }
            if c.style == "bsChevron" {
                let count = item_strs.len();
                let active_idx = if let Some(i) = c.escolhido {
                    i.min(count.saturating_sub(1))
                } else if count > 0 {
                    count - 1
                } else {
                    0
                };
                paint_breadcrumb_chevron(
                    &painter,
                    r,
                    &item_strs,
                    active_idx,
                    None,
                    c.segment_color.into(),
                    c.current_color.into(),
                    font,
                );
            } else {
                paint_breadcrumb(
                    &painter,
                    r,
                    &item_strs,
                    &c.separator,
                    None,
                    &estilo.visuals,
                    font,
                );
            }
        }
        "Pagination" => {
            let font = t.fonte(12.0);
            let mut painter = p.clone();
            if !c.enabled {
                painter.multiply_opacity(0.5);
            }
            let slots = pagination_slots(
                c.page_count.max(1) as usize,
                c.page_index.max(0) as usize,
                c.max_buttons.max(5) as usize,
                c.show_arrows,
            );
            paint_pagination(
                &painter,
                r,
                &slots,
                c.page_index.max(0) as usize,
                None,
                &estilo.visuals,
                font,
            );
        }
        "MultiSelect" => {
            let item_strs: Vec<String> = c.itens.iter().map(|it| it.text.clone()).collect();
            let font = t.fonte(12.0);
            let summary =
                multi_select_summary(&item_strs, &c.selected_items, &c.text_hint, |text| {
                    p.layout_no_wrap(text.to_owned(), font.clone(), estilo.visuals.text_color())
                        .size()
                        .x
                        <= (r.width() - t.medida(24.0)).max(0.0)
                });
            let mut painter = p.clone();
            if !c.enabled {
                painter.multiply_opacity(0.5);
            }
            paint_multi_select(&painter, r, &summary, &estilo.visuals, font);
        }
        "CheckListBox" => {
            let item_strs: Vec<String> = c.itens.iter().map(|it| it.text.clone()).collect();
            let font = t.fonte(12.0);
            let mut painter = p.clone();
            if !c.enabled {
                painter.multiply_opacity(0.5);
            }
            paint_check_list_box(
                &painter,
                r,
                &item_strs,
                &c.checked_items,
                &estilo.visuals,
                font,
            );
        }
        "Shape" => {
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            if c.shape_kind == "Circle" {
                let radius = (r.width().min(r.height()) / 2.0).max(1.0);
                p.circle_stroke(r.center(), radius, Stroke::new(c.borda, w.bg_stroke.color));
            } else {
                p.rect_stroke(
                    r,
                    2.0,
                    Stroke::new(c.borda, w.bg_stroke.color),
                    StrokeKind::Inside,
                );
            }
        }
        "Splitter" => {
            p.rect_filled(r, 0.0, PAINEL);
            p.rect_stroke(r, 0.0, Stroke::new(1.0, PAINEL_BORDA), StrokeKind::Inside);
            if c.beveled {
                let mid_x = r.center().x;
                let mid_y = r.center().y;
                let dot_radius = 1.5;
                let dot_color = w.bg_stroke.color;
                if r.height() >= r.width() {
                    p.circle_filled(Pos2::new(mid_x, mid_y - 6.0), dot_radius, dot_color);
                    p.circle_filled(Pos2::new(mid_x, mid_y), dot_radius, dot_color);
                    p.circle_filled(Pos2::new(mid_x, mid_y + 6.0), dot_radius, dot_color);
                } else {
                    p.circle_filled(Pos2::new(mid_x - 6.0, mid_y), dot_radius, dot_color);
                    p.circle_filled(Pos2::new(mid_x, mid_y), dot_radius, dot_color);
                    p.circle_filled(Pos2::new(mid_x + 6.0, mid_y), dot_radius, dot_color);
                }
            }
        }
        // a MaskEdit shows its masked text — the blank mask when empty
        "Edit" | "MaskEdit" => {
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
            let text = crawrad_runtime::mask::shown_text(&c.edit_mask, &c.text);
            let mostrado = match c.senha {
                Some(ch) => ch.to_string().repeat(text.chars().count()),
                None => text,
            };
            p.text(
                Pos2::new(r.min.x + t.medida(4.0), r.center().y),
                Align2::LEFT_CENTER,
                mostrado,
                fonte,
                cor_texto,
            );
        }
        // DateTimePicker shows formatted date or time
        "DateTimePicker" => {
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
            let kind = crawrad_runtime::date_time::DateTimeKind::parse_kind(&c.kind);
            let text = crawrad_runtime::date_time::iso_to_shown(kind, &c.date_time);
            p.text(
                Pos2::new(r.min.x + t.medida(4.0), r.center().y),
                Align2::LEFT_CENTER,
                text,
                fonte,
                cor_texto,
            );
        }
        "Label" => {
            p.text(
                Pos2::new(r.min.x, r.center().y),
                Align2::LEFT_CENTER,
                sem_acelerador(&c.text),
                fonte,
                cor_texto,
            );
        }
        "TreeView" => {
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
            draw_items(p, r, &c.itens, cor_texto, t);
        }
        "Panel" => {
            // Container: background, frame, and caption in the corner. What
            // is inside it is drawn later, because drawing order
            // puts the parent before its children.
            p.rect_filled(r, 2.0, PAINEL);
            // Border width comes from the document — `TPanel`'s `BorderWidth`.
            // It was hardcoded to 1.0, and the property existed in the catalog
            // without anything reading it.
            p.rect_stroke(
                r,
                2.0,
                egui::Stroke::new(t.medida(c.borda), PAINEL_BORDA),
                StrokeKind::Inside,
            );
            if !c.text.is_empty() {
                p.text(
                    Pos2::new(r.min.x + t.medida(6.0), r.min.y + t.medida(9.0)),
                    Align2::LEFT_CENTER,
                    sem_acelerador(&c.text),
                    fonte,
                    cor_texto,
                );
            }
        }
        "PageControl" => {
            let (paginas, ativa) = abas;
            tira_de_abas(
                p,
                r,
                &paginas,
                ativa,
                fonte.clone(),
                CoresDaAba {
                    fundo: PAPEL,
                    fundo_inativa: PAINEL,
                    borda: PAINEL_BORDA,
                    texto: cor_texto,
                },
                true,
                t,
            );
        }
        "TabSheet" => {
            // The page paints nothing: `PageControl` already painted the
            // body. It exists to carry children and caption.
        }
        "GroupBox" => {
            moldura_do_groupbox(
                p,
                r,
                &sem_acelerador(&c.text),
                fonte.clone(),
                PAINEL_BORDA,
                cor_texto,
                t,
            );
        }
        "Toggle" => {
            // the switch's track, its knob left (off) or right (on)
            let track = ERect::from_min_size(
                Pos2::new(r.min.x + t.medida(2.0), r.center().y - t.medida(8.0)),
                Vec2::new(t.medida(32.0), t.medida(16.0)),
            );
            let fill = if c.marcado {
                estilo.visuals.selection.bg_fill
            } else {
                estilo.visuals.widgets.inactive.bg_fill
            };
            p.rect(track, t.medida(8.0), fill, w.bg_stroke, StrokeKind::Inside);
            let x = if c.marcado {
                track.right() - t.medida(8.0)
            } else {
                track.left() + t.medida(8.0)
            };
            p.circle_filled(Pos2::new(x, track.center().y), t.medida(6.0), cor_texto);
            p.text(
                Pos2::new(track.right() + t.medida(6.0), r.center().y),
                Align2::LEFT_CENTER,
                sem_acelerador(&c.text),
                fonte.clone(),
                cor_texto,
            );
        }
        "LinkLabel" => {
            // the caption in the link colour, underlined as far as it runs
            let ink = if c.enabled {
                estilo.visuals.hyperlink_color
            } else {
                cor_texto
            };
            let galley = p.layout_no_wrap(sem_acelerador(&c.text), fonte.clone(), ink);
            let top = Pos2::new(r.min.x, r.center().y - galley.size().y / 2.0);
            let under = top.y + galley.size().y;
            let right = (r.min.x + galley.size().x).min(r.max.x);
            p.galley(top, galley, ink);
            p.line_segment(
                [Pos2::new(r.min.x, under), Pos2::new(right, under)],
                egui::Stroke::new(1.0, ink),
            );
        }
        "ColorBox" => {
            // the swatch of Selected, its name, the list's drop arrow
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
            let k = c.selected;
            let s = t.medida(14.0).min(r.height());
            let swatch = ERect::from_min_size(
                Pos2::new(r.min.x + t.medida(4.0), r.center().y - s / 2.0),
                Vec2::splat(s),
            );
            p.rect(
                swatch,
                2.0,
                Color32::from_rgba_unmultiplied(k.r, k.g, k.b, k.a),
                w.bg_stroke,
                StrokeKind::Inside,
            );
            let name = crawrad_ir::Color::STANDARD
                .iter()
                .find(|(_, x)| *x == k)
                .map_or("Custom", |(n, _)| n);
            p.text(
                Pos2::new(swatch.max.x + t.medida(4.0), r.center().y),
                Align2::LEFT_CENTER,
                name,
                fonte.clone(),
                cor_texto,
            );
            p.text(
                Pos2::new(r.max.x - t.medida(8.0), r.center().y),
                Align2::CENTER_CENTER,
                "v",
                fonte.clone(),
                cor_texto,
            );
        }
        "SpinEdit" => {
            // the field with its Value, the step buttons at its right
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
            let steps = t.medida(16.0).min(r.width());
            p.text(
                Pos2::new(r.min.x + t.medida(4.0), r.center().y),
                Align2::LEFT_CENTER,
                c.spin.to_string(),
                fonte.clone(),
                cor_texto,
            );
            for (i, arrow) in ["+", "-"].iter().enumerate() {
                let b = ERect::from_min_size(
                    Pos2::new(r.max.x - steps, r.min.y + i as f32 * r.height() / 2.0),
                    Vec2::new(steps, r.height() / 2.0),
                );
                p.rect_stroke(b, 1.0, w.bg_stroke, StrokeKind::Inside);
                p.text(
                    b.center(),
                    Align2::CENTER_CENTER,
                    *arrow,
                    fonte.clone(),
                    cor_texto,
                );
            }
        }
        "RadioGroup" => {
            // the group box's frame, and a ring per item in its
            // `Columns`, filled top down — the generated apps' layout
            moldura_do_groupbox(
                p,
                r,
                &sem_acelerador(&c.text),
                fonte.clone(),
                PAINEL_BORDA,
                cor_texto,
                t,
            );
            let columns = c.columns.max(1) as usize;
            let rows = c.itens.len().div_ceil(columns).max(1);
            let inner_w = (r.width() - t.medida(16.0)).max(0.0);
            let inner_h = (r.height() - t.medida(26.0)).max(0.0);
            let cell = Vec2::new(
                inner_w / columns as f32,
                (inner_h / rows as f32).min(t.medida(22.0)),
            );
            for (i, item) in c.itens.iter().enumerate() {
                let at = Pos2::new(
                    r.min.x + t.medida(8.0) + (i / rows) as f32 * cell.x,
                    r.min.y + t.medida(18.0) + (i % rows) as f32 * cell.y,
                );
                let center = Pos2::new(at.x + t.medida(6.0), at.y + cell.y / 2.0);
                p.circle_stroke(center, t.medida(6.0), egui::Stroke::new(1.0, cor_texto));
                if Some(i) == c.escolhido {
                    p.circle_filled(center, t.medida(3.0), cor_texto);
                }
                p.text(
                    Pos2::new(at.x + t.medida(18.0), center.y),
                    Align2::LEFT_CENTER,
                    &item.text,
                    fonte.clone(),
                    cor_texto,
                );
            }
        }
        "Memo" => {
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
            // One line per line of text, top down — what
            // the multiline `EDIT` displays when idle.
            //
            // Stops when exceeding the end of the box, not at a fixed number of
            // lines. The `take(12)` that used to live here failed in both directions: on
            // a 90-point `Memo` it told it to draw twelve lines where six fit
            // — the excess is what appeared on top of the neighboring `ListBox`
            // —, and on a 300-point `Memo` it stopped at the twelfth with
            // half the box empty.
            for (i, linha) in c.text.lines().enumerate() {
                let y = r.min.y + t.medida(10.0 + i as f32 * 15.0);
                if y > r.max.y {
                    break;
                }
                p.text(
                    Pos2::new(r.min.x + t.medida(4.0), y),
                    Align2::LEFT_CENTER,
                    linha,
                    fonte.clone(),
                    cor_texto,
                );
            }
        }
        "ListBox" => {
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
            // The SAME row as the live form and the emitted application: starts
            // one point below the border, has a height of [`LINHA_DA_LISTA`], and the
            // first one that does not fit entirely is not drawn.
            //
            // Here lived divergence **D3** from the backstage study: the
            // designer stepped by 16 and the application by 18, so a 90-point box
            // showed eight items in the designer and four in the application — and the
            // four leftover items ended up over the neighbor. The one that was
            // wrong was the designer: the live 18 is not our choice, it is the
            // `interact_size.y` with which egui actually draws the row, and
            // `rxp.rs` emits the same number. A freehand drawing that disagrees with the
            // widget is the drawing that changes.
            for (i, item) in c.itens.iter().enumerate() {
                let alt = t.medida(LINHA_DA_LISTA);
                let topo = r.min.y + t.medida(1.0) + i as f32 * alt;
                if topo + alt > r.max.y {
                    break;
                }
                if Some(i) == c.escolhido {
                    p.rect_filled(
                        ERect::from_min_max(
                            Pos2::new(r.min.x + t.medida(1.0), topo),
                            Pos2::new(r.max.x - t.medida(1.0), topo + alt),
                        ),
                        0.0,
                        estilo.visuals.selection.bg_fill,
                    );
                }
                p.text(
                    Pos2::new(r.min.x + t.medida(6.0), topo + alt / 2.0),
                    Align2::LEFT_CENTER,
                    &item.text,
                    fonte.clone(),
                    cor_texto,
                );
            }
        }
        "CheckBox" | "RadioButton" => {
            // The checkmark and caption, egui-style — the RCL uses
            // Windows `BUTTON`, and both read the same `checked`.
            let cy = r.center().y;
            let caixa = ERect::from_min_size(
                Pos2::new(r.min.x + t.medida(1.0), cy - t.medida(7.0)),
                Vec2::splat(t.medida(14.0)),
            );
            if c.class == "RadioButton" {
                p.circle_stroke(
                    caixa.center(),
                    t.medida(7.0),
                    egui::Stroke::new(1.0, cor_texto),
                );
                if c.marcado {
                    p.circle_filled(caixa.center(), t.medida(3.5), cor_texto);
                }
            } else {
                // Same reason as the panel: inactive `bg_stroke` is
                // too faint on light paper, and the box disappeared.
                p.rect_stroke(
                    caixa,
                    2.0,
                    egui::Stroke::new(1.0, cor_texto),
                    StrokeKind::Inside,
                );
                if c.marcado {
                    p.text(
                        caixa.center(),
                        Align2::CENTER_CENTER,
                        "✔",
                        fonte.clone(),
                        cor_texto,
                    );
                }
            }
            p.text(
                Pos2::new(caixa.max.x + t.medida(6.0), cy),
                Align2::LEFT_CENTER,
                sem_acelerador(&c.text),
                fonte,
                cor_texto,
            );
        }
        "ComboBox" => {
            // Closed, as it is born: the box, the selected item, and the
            // arrow. In the designer you don't open the dropdown — opening would be an
            // action of the form user, and here the action belongs to the designer.
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
            let escrito = c
                .escolhido
                .and_then(|i| c.itens.get(i))
                .map(|i| i.text.clone())
                .unwrap_or_default();
            p.text(
                Pos2::new(r.min.x + t.medida(6.0), r.center().y),
                Align2::LEFT_CENTER,
                escrito,
                fonte,
                cor_texto,
            );
            seta_para_baixo(p, r, cor_texto, t);
        }
        "ScrollBar" => {
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
            // Geometry comes from `barra_de_rolagem`, and live calls the SAME:
            // what the designer draws must be what Preview shows.
            let b = barra_de_rolagem(r, c.posicao_scroll);
            p.rect_filled(b.inicio, 2.0, w.bg_fill);
            p.rect_filled(b.fim, 2.0, w.bg_fill);
            p.rect_filled(b.polegar, 2.0, PAINEL_BORDA);
        }
        "ScrollBox" => {
            // **The box SINKS**, which is what separates it from a `Panel`.
            //
            // It was `PAINEL` — the same surface as `Panel` and `GroupBox` —,
            // and `ScrollBox` was indistinguishable from them on screen. A classic RAD IDE
            // separates the two by border: `TScrollBox` is born with
            // `BorderStyle = bsSingle` and sunken client area, `TPanel` with
            // raised bevel. It is also what the generated application displays:
            // `egui::Frame::canvas`
            // (`crawrad-codegen/src/targets/egui/components/scroll_box.rs`).
            p.rect_filled(r, 2.0, estilo.visuals.extreme_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
            // **And NO BAR, on purpose — this target would have no way to
            // display it.**
            //
            // The Makepad adapter draws the bar when content overflows, by the
            // neutral rule of `crawrad_runtime::rolagem`, and the Makepad application
            // shows the same bar. Here that is not possible, and the reason comes from the TARGET, not
            // the drawing: the `ScrollBox` generated for egui is an
            // **EMPTY** `egui::ScrollArea`, and its children are drawn
            // as SIBLINGS in the form's `show()`, each in its own anchored
            // rectangle (`targets/egui/components/scroll_box.rs`). A
            // container that does not contain never overflows, so the application
            // bar never appears.
            //
            // Drawing the bar here would be the designer promising what this
            // target does not deliver — the family of bug this iteration came
            // to close, inverted. It returns the day the egui target truly nests
            // children, and then it is a single line.
        }
        // A class RXP does not know how to draw yet does not disappear: it appears
        // as a labeled box, which is honest and debuggable. Disappearing in
        // silence would be the falsehood that ADR 0002 wants to prevent.
        outra => {
            p.rect_filled(r, 2.0, estilo.visuals.faint_bg_color);
            p.rect_stroke(r, 2.0, w.bg_stroke, StrokeKind::Inside);
            texto_centrado(p, r, outra, t.fonte(11.0), cor_texto);
        }
    }
}

/// The tab rectangles that **fit** in the strip, in order, in screen points.
///
/// **Calculated by the DESIGN arm, and only by it.** The live form once had a
/// reader here — it needed to know where to place the tab hit target —, and
/// today it no longer does: the Preview strip is painted by `page_control` from
/// `desenho_comum.rs`, the SAME function called by the generated application, and it
/// registers its own hit target. The design arm could not follow, and the reason is
/// zoom: here every measurement passes through `vista` (ADR 0019), and egui widgets
/// do not scale by rectangle.
///
/// The promise remains the same, which is that of [`tira_de_abas`]: where the tab is
/// drawn is where the click finds it, **by construction** — both read
/// this function.
///
/// Measurements belong to DESIGN: `TAB_H` and `TAB_W` from `desenho_comum.rs`,
/// mirroring the `crawrad_registry` constants with which the designer resolves
/// clicks on tabs and on `+`. In screen points, the drawn tab and clickable tab
/// drifted apart with zoom.
fn retangulos_das_abas(
    r: ERect,
    quantas: usize,
    com_mais: bool,
    vista: Transformacao,
) -> Vec<ERect> {
    let alt = vista.medida(TAB_H);
    let larg = vista.medida(TAB_W);
    // The "+" lives on the right border, so tabs stop before it.
    let limite = if com_mais {
        r.max.x - vista.medida(LARGURA_MAIS)
    } else {
        r.max.x
    };
    let mut fora = Vec::with_capacity(quantas);
    for i in 0..quantas {
        let aba = ERect::from_min_size(
            Pos2::new(r.min.x + i as f32 * larg, r.min.y),
            Vec2::new(larg, alt),
        );
        // A tab that does not fit completely is not drawn halfway: half a
        // clickable tab on top of "+" would be worse than an absent tab. Scrolling the strip
        // is the real fix, and it does not exist yet.
        if aba.max.x > limite {
            break;
        }
        fora.push(aba);
    }
    fora
}

/// The tab strip of a `PageControl` **in the DESIGN arm**, and the body below
/// it.
///
/// Preview and the generated application do not pass through here: both call
/// `page_control` from `desenho_comum.rs`, which is a single function and therefore has
/// no way to disagree with itself. This continues to exist because freehand
/// drawing scales with zoom and the egui widget does not (ADR 0019) — and because only it
/// paints the `+`, which is a design action.
///
/// The tab has a FIXED width (`TAB_W` from `desenho_comum.rs`, mirroring
/// `crawrad_registry::LARGURA_DA_ABA` which the designer uses to resolve clicks).
/// A classic RAD IDE sizes by text;
/// here that would cost measuring fonts in two places forced to agree
/// forever, which is the class of bug that already bit this project three times.
/// Fixed width trades fidelity for a guarantee: where the tab is drawn is
/// where the click finds it, by construction.
fn tira_de_abas(
    p: &egui::Painter,
    r: ERect,
    paginas: &[String],
    ativa: usize,
    fonte: egui::FontId,
    cores: CoresDaAba,
    com_mais: bool,
    vista: Transformacao,
) {
    let CoresDaAba {
        fundo,
        fundo_inativa,
        borda,
        texto: cor_texto,
    } = cores;
    let t = Stroke::new(1.0, borda);
    // The three tab measurements belong to DESIGN — they are the same constants from
    // `crawrad_registry` with which the designer resolves clicks on tabs and on `+`.
    // Leaving them in screen points placed the drawn tab in one place and the clickable tab
    // in another, and the distance between them grew with zoom.
    let alt = vista.medida(TAB_H);
    let larg_mais = vista.medida(LARGURA_MAIS);
    let corpo = ERect::from_min_max(Pos2::new(r.min.x, r.min.y + alt), r.max);
    p.rect_filled(corpo, 2.0, fundo);
    p.rect_stroke(corpo, 2.0, t, StrokeKind::Inside);
    for (i, aba) in retangulos_das_abas(r, paginas.len(), com_mais, vista)
        .into_iter()
        .enumerate()
    {
        let legenda = &paginas[i];
        p.rect_filled(aba, 2.0, if i == ativa { fundo } else { fundo_inativa });
        p.rect_stroke(aba, 2.0, t, StrokeKind::Inside);
        p.text(
            aba.center(),
            Align2::CENTER_CENTER,
            legenda,
            fonte.clone(),
            cor_texto,
        );
    }
    if com_mais {
        // The "+" right after the last tab. It is the ONLY way to create a page —
        // a classic RAD IDE uses a context menu, which does not exist here yet —, so
        // it always appears, including on a freshly dropped control that does not
        // have any pages yet. Without it `PageControl` would be born useless.
        //
        // Only in design: in Preview the user is simulating the application, and
        // applications do not gain tabs by clicking.
        let mais = ERect::from_min_size(
            Pos2::new(r.max.x - larg_mais, r.min.y),
            Vec2::new(larg_mais, alt),
        );
        p.rect_stroke(mais, 2.0, Stroke::new(1.0, borda), StrokeKind::Inside);
        p.text(mais.center(), Align2::CENTER_CENTER, "+", fonte, cor_texto);
    }
}

// Tab strip height and tab width used to live here, as `ALTURA_ABA` and
// `LARGURA_ABA`, and there was a THIRD copy in a string in `rxp.rs`. Today they are
// `TAB_H` and `TAB_W` in `desenho_comum.rs`, and all three arms — design,
// Preview, and application — read the same number. What continues mirroring
// `crawrad-registry` is the value, not the algorithm: the catalog is a **dev-dependency**
// here on purpose (the adapter knows nothing of the catalog, ADR 0002/0013), and the
// test preventing divergence is `as_medidas_da_aba_batem_com_o_registro`.

/// Width of the tab strip "+". Mirrors `crawrad_registry::LARGURA_DO_MAIS`.
///
/// This is NOT in `desenho_comum.rs`, and the reason is its scope: "+" is a
/// design action — applications do not gain tabs by clicking —, so it has no
/// place in the file that goes into the user's project.
const LARGURA_MAIS: f32 = 22.0;

/// Stroke thickness that a component draws **on top of** its own border.
///
/// It is 1 point across the entire file, and 1 point on purpose: the contour of a
/// control in Windows is one pixel. What the constant adds is having a
/// **single place** where the value lives, because three things depend on it and all three
/// had loose numbers: [`draw_control`] clipping (which needs
/// half the thickness as margin so as not to eat the frame), leakage scan
/// tolerance, and the frame guard.
///
/// It **does not** pass through `Transformacao`: contours do not thicken with zoom, neither here
/// nor in `rect_stroke` of any arm — what thickens is `Panel`'s frame,
/// which comes from the document and is drawn `Inside`, inward, without leaving the box.
const ESPESSURA_DA_BORDA: f32 = 1.0;

/// The outer half of the border stroke.
///
/// A stroke centered on the border has half inside and half outside. This is
/// the outer half — the one clipping eats if it cuts at the exact rectangle, and the only
/// ink a component is allowed to place beyond its own box.
const META_DA_BORDA: f32 = ESPESSURA_DA_BORDA / 2.0;

/// A length clamped between a floor and a ceiling **where the ceiling rules**.
///
/// This is `f32::clamp` that does not panic. Rust's clamp **panics** when
/// `min > max` — and that is exactly the shape of every drawing floor when a
/// rectangle shrinks: *"the thumb is at least 8 points, and never larger than
/// the track"* is a true statement until the track measures 7. The measured panic said
/// `min > max, or either was NaN. min = 8.0, max = 5.0`, and shut down the IDE.
///
/// The order here is `max` **and then** `min`: the floor is a wish, the ceiling is a
/// fact. When both cannot fit together, the winner is the fact — the thumb of
/// a 7-point track has 7 points, not 8 non-existent points.
///
/// **This is not a minimum component size.** The contract is that of a classic RAD IDE,
/// observed running: shrinking the form over a component
/// **clips** it, and the component remains intact beneath the border. Nothing
/// shrinks to a floor, nothing locks up, and nothing crashes. The entity that must handle the
/// degenerate rectangle is DRAWING — here, at the origin, not in four
/// scattered patches.
fn entre(v: f32, piso: f32, teto: f32) -> f32 {
    v.max(piso).min(teto)
}

/// The side of one of the two buttons of a `ScrollBar`, in screen points.
///
/// Square: a bar button is as wide as the bar is tall. The ceiling of 16
/// prevents a thick bar from turning into two buttons with no track.
const BOTAO_DA_BARRA: f32 = 16.0;

/// Proportion of the track the thumb occupies when it fits.
const POLEGAR_DA_BARRA: f32 = 0.3;

/// The smallest thumb that can still be grabbed — **when the track allows**.
const POLEGAR_MINIMO: f32 = 8.0;

/// Margin between the thumb and the bar wall, on the short axis.
const FOLGA_DO_POLEGAR: f32 = 2.0;

/// Geometry of a `ScrollBar`: the two buttons and thumb, in screen points.
///
/// **One calculation, four readers.** It was born from four copies — horizontal
/// design, vertical design, horizontal live, and vertical live — and all four
/// carried the same `clamp(8.0, pista)` that crashed the IDE when track length
/// dropped below 8. Four copies are four chances to fix only three; and design
/// and live drawing the SAME bar through two different paths is
/// precisely what ADR 0013
/// forbids.
///
/// **It tolerates any rectangle**, including empty and inverted ones:
/// `Transformacao::na_tela` already prevents negative sides, and what remains —
/// zero side, 3-point side — emerges as a zero-area rectangle that egui
/// paints as nothing. No panics and no invented floors.
struct BarraDeRolagem {
    /// The starting button: left when horizontal, top when vertical.
    inicio: ERect,
    /// The ending button.
    fim: ERect,
    /// The thumb, already placed at the component's `position`.
    polegar: ERect,
}

fn barra_de_rolagem(r: ERect, posicao: f32) -> BarraDeRolagem {
    let deitada = r.width() > r.height();
    // The short side dictates button size in both directions.
    let botao = if deitada {
        r.height().min(BOTAO_DA_BARRA)
    } else {
        r.width().min(BOTAO_DA_BARRA)
    };
    let comprido = if deitada { r.width() } else { r.height() };
    // Track is what REMAINS between the two buttons — and when nothing remains,
    // it measures zero. The `max(4.0)` that used to live here invented a 4-point
    // track for an 8-point thumb to fit inside: it was the `min > max` panic,
    // written two lines before it occurred.
    let pista = (comprido - botao * 2.0).max(0.0);
    let polegar = entre(pista * POLEGAR_DA_BARRA, POLEGAR_MINIMO, pista);
    let fracao = (posicao / 100.0).clamp(0.0, 1.0);
    let avanco = botao + fracao * (pista - polegar);
    // Thumb thickness is the short side minus margin on both sides; the
    // floor of 1 is what remains of the bar when thinner than margin, and is
    // stroke thickness, not component size.
    let grosso = if deitada { r.height() } else { r.width() };
    let espesso = (grosso - FOLGA_DO_POLEGAR * 2.0).max(1.0);
    if deitada {
        BarraDeRolagem {
            inicio: ERect::from_min_size(r.min, Vec2::new(botao, r.height())),
            fim: ERect::from_min_size(
                Pos2::new(r.max.x - botao, r.min.y),
                Vec2::new(botao, r.height()),
            ),
            polegar: ERect::from_min_size(
                Pos2::new(r.min.x + avanco, r.min.y + FOLGA_DO_POLEGAR),
                Vec2::new(polegar, espesso),
            ),
        }
    } else {
        BarraDeRolagem {
            inicio: ERect::from_min_size(r.min, Vec2::new(r.width(), botao)),
            fim: ERect::from_min_size(
                Pos2::new(r.min.x, r.max.y - botao),
                Vec2::new(r.width(), botao),
            ),
            polegar: ERect::from_min_size(
                Pos2::new(r.min.x + FOLGA_DO_POLEGAR, r.min.y + avanco),
                Vec2::new(espesso, polegar),
            ),
        }
    }
}

/// Height of a `ListBox` row, in design units.
///
/// **One constant, three readers**: the designer drawing, the live form, and
/// — by value, not by symbol — `rxp.rs`, which emits the same 18 in the
/// application. While the designer had 16 hardcoded and live had 18, a
/// 90-point box displayed eight items on one side and four on the other (
/// divergence D3 from the backstage study). 18 is not arbitrary: it is egui's default
/// `interact_size.y`, which is the height at which the live form's `selectable_label`
/// draws rows.
const LINHA_DA_LISTA: f32 = 18.0;

/// `GroupBox` frame, with gap for the caption.
///
/// A classic RAD IDE's `TGroupBox` **has no background** — the parent shows through —,
/// and the frame has a gap where the caption passes. Drawing it this way, instead of
/// painting a rectangle and erasing a piece of the stroke, avoids having to guess the
/// background color: inside a `Panel` it is not paper color, and the only
/// correct answer is not needing the answer.
///
/// Returns where caption should be written, or `None` when there is no caption.
fn moldura_do_groupbox(
    p: &egui::Painter,
    r: ERect,
    legenda: &str,
    fonte: egui::FontId,
    cor_borda: Color32,
    cor_texto: Color32,
    vista: Transformacao,
) {
    let t = Stroke::new(1.0, cor_borda);
    // The top line passes through the MIDDLE of the caption, as in Windows.
    let (topo, vao) = if legenda.is_empty() {
        (r.min.y, None)
    } else {
        let g = p.layout_no_wrap(legenda.to_string(), fonte.clone(), cor_texto);
        let x0 = r.min.x + vista.medida(8.0);
        (
            r.min.y + g.size().y / 2.0,
            Some((x0, g.size().x, g.size().y)),
        )
    };
    match vao {
        Some((x0, larg, _)) => {
            let vao = vista.medida(4.0);
            p.line_segment([Pos2::new(r.min.x, topo), Pos2::new(x0 - vao, topo)], t);
            // `.min(r.max.x)` is what prevents an **inverted segment**.
            //
            // The gap is measured by the caption width, and a caption that does not fit
            // put the start of the right segment AFTER its end: measured, a
            // 46-character caption in a 140-point box produced
            // `(372.84) -> (198.00)` — 174.84 points of line drawn
            // outside the component, backwards. egui's `line_segment`
            // does not care about order, so it complied and painted the
            // stroke over the neighbor.
            //
            // Clamped, the right segment simply has no remainder when the
            // caption occupies the entire width — which is correct: there is no line to
            // draw there, the caption is in its place.
            //
            // **This is not clipping, and does not depend on it.** The numbers above
            // are from segment endpoints, before any `clip_rect`. Clipping
            // only traded a stroke on the neighbor for a piece of
            // missing frame — what the owner saw.
            p.line_segment(
                [
                    Pos2::new((x0 + larg + vao).min(r.max.x), topo),
                    Pos2::new(r.max.x, topo),
                ],
                t,
            );
        }
        None => {
            p.line_segment([Pos2::new(r.min.x, topo), Pos2::new(r.max.x, topo)], t);
        }
    }
    p.line_segment([Pos2::new(r.min.x, topo), Pos2::new(r.min.x, r.max.y)], t);
    p.line_segment([Pos2::new(r.max.x, topo), Pos2::new(r.max.x, r.max.y)], t);
    p.line_segment(
        [Pos2::new(r.min.x, r.max.y), Pos2::new(r.max.x, r.max.y)],
        t,
    );
    if let Some((x0, _, alt)) = vao {
        p.text(
            Pos2::new(x0, r.min.y + alt / 2.0),
            Align2::LEFT_CENTER,
            legenda,
            fonte,
            cor_texto,
        );
    }
}

/// Nodes that must be UNCHECKED when `escolhido` is clicked.
///
/// This is `TurnSiblingsOff` from a classic RAD IDE, and the important word is *siblings*:
/// radio button exclusion is between **siblings of the same parent**, not across the whole
/// form and not via Win32's `WS_GROUP` — the classic library never used `WS_GROUP` for
/// this. Two side-by-side `GroupBox`es are two independent groups because they are
/// two parents, nothing more.
///
/// This was previously wrong here, with a comment that aged with it: it said
/// "no containers yet, parent is form" and unchecked every radio button in the
/// document. When containers arrived (ADR 0012), the comment became
/// false and the code became a bug — checking a radio button inside a `Panel`
/// unchecked those outside.
fn irmaos_a_desmarcar<'a>(
    ctls: impl IntoIterator<Item = (&'a NodeId, &'a Ctl)>,
    escolhido: &NodeId,
    pai_do_escolhido: &NodeId,
) -> Vec<NodeId> {
    ctls.into_iter()
        .filter(|(id, c)| {
            c.class == "RadioButton" && &c.parent == pai_do_escolhido && *id != escolhido
        })
        .map(|(id, _)| id.clone())
        .collect()
}

/// The **tile** of a component: an entire document painted inside
/// `area`, without grid, without ruler, without handles, and without tab order badge.
///
/// This is how the IDE palette draws each of its entries — and the point is that
/// it **draws nothing on its own**: `draw_control` does the painting, the same code
/// that paints the form. A separately drawn icon would be a second
/// truth about the look of each component, with nothing forcing the two to
/// agree; thus no icon can disagree with what the user drops, and
/// a new component is born with a tile without extra work.
///
/// The adapter still knows nothing of the catalog (ADR 0002/0013): the shell builds the
/// document via `Registry::instantiate` — which is the path through which
/// the user's component is created (rule 9 of the study).
///
/// Scaling comes from **fit**, and never exceeds 100%: a non-visual badge
/// has 28 units and enlarging it to fill the tile would show a
/// component at a size it never actually has.
pub fn pintar_amostra(p: &egui::Painter, area: ERect, doc: &Document) {
    // Fitting is based on the box OF WHAT WILL BE PAINTED, not the form box:
    // the sample form is just the cradle of the component, and fitting by it
    // would leave the drawing tiny in the middle of an empty tile.
    pintar_dentro(p, area, doc, Encaixe::PeloDesenho);
}

/// The **preview of an entire form**: the document painted inside `area`,
/// fitted by the form and not by its components.
///
/// This is the difference separating the tile from the preview. The tile shows ONE
/// component and the form behind it is incidental; the preview shows the form,
/// and its empty space **is information** — a template with two buttons in a corner
/// and the rest free must appear like that, not with the two buttons inflated
/// occupying the whole screen.
///
/// What draws continues to be `draw_control`, the same canvas code
/// (ADR 0020
/// and ADR 0021).
/// A template picture would be a second truth about its look.
///
/// **Without title bar.** The form in this product does not have one drawn,
/// neither on the canvas nor in Preview: the window system provides the title bar when
/// the application runs. Drawing one here would be chrome existing on a single screen.
pub fn pintar_amostra_do_formulario(p: &egui::Painter, area: ERect, doc: &Document) {
    pintar_dentro(p, area, doc, Encaixe::PeloFormulario);
}

/// How the sample is fitted into `area`.
#[derive(Clone, Copy, PartialEq)]
enum Encaixe {
    /// By the bounding box of visible controls — palette tile.
    PeloDesenho,
    /// By the form rectangle — template preview.
    PeloFormulario,
}

/// Common body of tile and preview.
///
/// A single function on purpose: the two differ **in the fitting bounding box**, and
/// nothing else. Two copies would diverge on the first fix only one received
/// — the lesson `TipoDeProjeto` in the shell already documents, and which here cost
/// handles drawn in one place and queried in another.
fn pintar_dentro(p: &egui::Painter, area: ERect, doc: &Document, encaixe: Encaixe) {
    let ordem_toda = doc.in_paint_order();
    let ctls: BTreeMap<NodeId, Ctl> = ordem_toda
        .iter()
        .filter_map(|id| doc.node(id).map(|n| (id.clone(), ctl_do_no(id, n, doc))))
        .collect();
    let ordem: Vec<NodeId> = ordem_toda
        .iter()
        .filter(|id| no_canvas(doc, id))
        .cloned()
        .collect();

    let visiveis: Vec<&Ctl> = ordem
        .iter()
        .filter_map(|id| ctls.get(id))
        .filter(|c| c.visible)
        .collect();
    let caixa = match encaixe {
        Encaixe::PeloFormulario => doc
            .node(&doc.root)
            .and_then(crawrad_ir::Node::rect)
            .filter(|r| r.w >= 1.0 && r.h >= 1.0),
        Encaixe::PeloDesenho => {
            let (Some(x0), Some(y0), Some(x1), Some(y1)) = (
                visiveis.iter().map(|c| c.rect.x).reduce(f32::min),
                visiveis.iter().map(|c| c.rect.y).reduce(f32::min),
                visiveis
                    .iter()
                    .map(|c| c.rect.x + c.rect.w.max(1.0))
                    .reduce(f32::max),
                visiveis
                    .iter()
                    .map(|c| c.rect.y + c.rect.h.max(1.0))
                    .reduce(f32::max),
            ) else {
                return;
            };
            Some(Rect::new(x0, y0, x1 - x0, y1 - y0))
        }
    };
    let Some(caixa) = caixa else { return };

    // **Zero-space area paints nothing**, and exits before computing scale.
    //
    // A closed panel, or a window mid-resize, delivers
    // an `area` with zero side length. That produced `escala = 0`, and a zero scale becomes
    // zero-sized `t.fonte(14.0)` — which `epaint` rejects with
    // `Bad px_scale_factor: 0`, crashing the IDE. Measured: side 0 crashes, side 1
    // lives. Clipping a zero-sized drawing was already the final result; what
    // changes is not passing through the calculation that crashes on the way.
    if area.width() <= 0.0 || area.height() <= 0.0 {
        return;
    }
    let escala = (area.width() / caixa.w)
        .min(area.height() / caixa.h)
        .min(1.0);
    let canto = area.center() - Vec2::new(caixa.w * escala, caixa.h * escala) / 2.0;
    let t = Transformacao {
        origem: Pos2::new(canto.x - caixa.x * escala, canto.y - caixa.y * escala),
        escala,
    };

    // Paper enters here, not in the shell: the component is drawn with the
    // colors of a light form (`estilo_do_papel`), and on the IDE's dark chrome
    // a `Label` would appear black on black. The tile is a piece of
    // form, and must have the canvas of one.
    //
    // WHERE the paper is painted is the other half of the difference between the two. The
    // tile fills the entire `area`: it is a piece of form, and the
    // clipping is the frame that the palette draws around it. The preview paints only the
    // form rectangle — because its shape is what is being observed, and
    // paper stretched to the panel edge would conceal the template's
    // proportion.
    let estilo = estilo_do_papel();
    let papel = match encaixe {
        Encaixe::PeloDesenho => area,
        Encaixe::PeloFormulario => t.na_tela(caixa),
    };
    // Without border drawn around: the canvas does not have one either. What separates
    // the `#F0F0F0` paper from the IDE dark chrome is contrast itself, and a
    // border here alone would make the preview have a frame that the real form
    // lacks.
    p.rect_filled(papel, 0.0, PAPEL);
    let p = p.with_clip_rect(area.intersect(p.clip_rect()));
    for id in &ordem {
        let Some(c) = ctls.get(id) else { continue };
        if !c.visible {
            continue;
        }
        let r = t.na_tela(c.rect);
        if !c.visual {
            selo_nao_visual(&p, r.min, &c.class, &[], t);
            continue;
        }
        let p = recortar_pelo_pai(&p, c, t);
        draw_control(&p, r, c, paginas_de(&ctls, &ordem_toda, id), &estilo, t);
    }
}

/// Does the node appear in the design area?
///
/// The rule lives on the boundary (`crawrad_runtime::na_superficie`), not here,
/// because the designer needs the SAME answer — when it existed on both
/// sides, two places had to be changed together and one was always missed.
///
/// What this adapter contributes is only what it knows: which classes it
/// draws. Where "is visual" comes from is the only thing differing between the two
/// sides — the designer asks the catalog, and the adapter knows no
/// catalog (`crawrad-registry` is a dev-dependency here, on purpose).
pub fn no_canvas(doc: &Document, id: &str) -> bool {
    crawrad_runtime::on_surface(doc, id, e_visual_no_canvas) && !drawn_by_its_owner(doc, id)
}

/// An item its owner draws (a `StatusPanel` inside its `StatusBar`,
/// a `ListColumn` inside its `ListView`, a `GridColumn` inside its `Grid`)
/// has no place of its own on the canvas.
fn drawn_by_its_owner(doc: &Document, id: &str) -> bool {
    let panel_in_status_bar = doc.node(id).is_some_and(|n| n.class == "StatusPanel")
        && doc
            .parent_of(id)
            .and_then(|p| doc.node(p))
            .is_some_and(|p| p.class == "StatusBar");
    let column_in_list_view = doc.node(id).is_some_and(|n| n.class == "ListColumn")
        && doc
            .parent_of(id)
            .and_then(|p| doc.node(p))
            .is_some_and(|p| p.class == "ListView");
    let column_in_grid = doc.node(id).is_some_and(|n| n.class == "GridColumn")
        && doc
            .parent_of(id)
            .and_then(|p| doc.node(p))
            .is_some_and(|p| p.class == "Grid");
    panel_in_status_bar || column_in_list_view || column_in_grid
}

/// Does the class draw anything in the design area (icons count).
///
/// The list is hand-written on purpose: the adapter does not know the catalog
/// (`crawrad-registry` is a dev-dependency here). What guarantees it does not lag
/// behind is the test `todo_nao_visual_do_registro_e_icone_e_nao_widget`, which
/// compares both from inside `#[cfg(test)]`, where the catalog exists.
fn e_visual_no_canvas(class: &str) -> bool {
    !matches!(
        class,
        "ActionList"
            | "Action"
            | "ImageList"
            | "Timer"
            | "MainMenu"
            | "MenuItem"
            | "PopupMenu"
            | "OpenDialog"
            | "SaveDialog"
            | "MessageDialog"
            | "StatusPanel"
            | "ListColumn"
            | "GridColumn"
            | "StyleRule"
    )
}

/// The interval between two ruler numbers, in design units.
///
/// It **depends on zoom**, as it must: a tick every 50 units is
/// comfortable at 100% and becomes a solid black wall at 25%, where those same 50 land in
/// 12 screen points. The ruler picks the finest step that still leaves
/// [`FOLGA_DA_REGUA`] points between two numbers — below that, numbers
/// touch, and an unreadable ruler measures nothing.
///
/// The steps are those of any ruler: 5, 10, 25, 50… Multiples of 5 and 10
/// because calculations done by looking are mental math.
fn passo_da_regua(escala: f32) -> f32 {
    const PASSOS: [f32; 8] = [5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0];
    PASSOS
        .iter()
        .copied()
        .find(|passo| passo * escala >= FOLGA_DA_REGUA)
        .unwrap_or(PASSOS[PASSOS.len() - 1])
}

/// Minimum distance, in screen points, between two ruler numbers.
///
/// Measured in the worst case the ruler produces: `1000` in 9pt monospace font,
/// occupying roughly 22 points. The rest is breathing room separating a number
/// from the tick of the next.
const FOLGA_DA_REGUA: f32 = 40.0;

/// The eight handle points of a component, **in design units**.
///
/// Previously received the rectangle already converted to screen, and returned screen points.
/// With zoom this breaks silently: `alcas` adds `ALCA_FORA` to the rectangle
/// it receives, and adding 3.5 to screen points is not the same as adding 3.5 to
/// design units and converting afterwards. The designer does the latter; this
/// file did the former. With a 1.0 zoom factor both give the same number — which is
/// precisely how this bug is born dormant (ADR 0019).
fn cantos(r: Rect) -> [(f32, f32); 8] {
    // Geometry comes from the BOUNDARY, not here. The designer looks for clicks
    // using the same numbers — they used to be two lists agreeing by luck, and
    // diverging would place the drawing in one place and the click in another.
    crawrad_runtime::handles(r.x, r.y, r.w, r.h)
}

/// `&&` is a literal `&`; a lone `&` marks the accelerator and is not drawn.
fn sem_acelerador(t: &str) -> String {
    let mut saida = String::with_capacity(t.len());
    let mut chars = t.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '&' {
            if chars.peek() == Some(&'&') {
                chars.next();
                saida.push('&');
            }
            continue;
        }
        saida.push(c);
    }
    saida
}

/// Where, in text with `&` stripped, the underlined character starts.
fn indice_do_acelerador(t: &str) -> Option<usize> {
    let mut i = 0usize;
    let mut chars = t.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '&' {
            if chars.peek() == Some(&'&') {
                chars.next();
                i += 1;
                continue;
            }
            return chars.peek().map(|_| i);
        }
        i += c.len_utf8();
    }
    None
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod varredura;

#[cfg(test)]
#[path = "test/degenerate/test.rs"]
mod degenerate;

#[cfg(test)]
#[path = "test/badge/test.rs"]
mod badge_test;

#[cfg(test)]
#[path = "test/chips/test.rs"]
mod chips_test;

#[cfg(test)]
#[path = "test/breadcrumb/test.rs"]
mod breadcrumb_test;

#[cfg(test)]
#[path = "test/pagination/test.rs"]
mod pagination_test;

#[cfg(test)]
#[path = "test/drawer/test.rs"]
mod drawer_test;

#[cfg(test)]
#[path = "test/multi_select/test.rs"]
mod multi_select_test;

#[cfg(test)]
#[path = "test/check_list_box/test.rs"]
mod check_list_box_test;

#[cfg(test)]
#[path = "test/scroll_bar/test.rs"]
mod scroll_bar_test;

// The control event router (`control_router.rs`) is text that
// `crawrad-codegen` copies into the user's `designer.rs`. Here nothing calls it,
// so it is only compiled in tests — outside them it would be `dead_code`. The
// `cfg(any())` is the same bridge for `cargo fmt` as in `desenho_comum.rs`.
#[cfg(test)]
include!("control_router.rs");

#[cfg(any())]
mod control_router;

#[cfg(test)]
#[path = "test/control_router/test.rs"]
mod control_router_test;

// The Tab order (`tab_order.rs`): text the `crawrad-codegen` copies, like
// the router above, and compiled here only in the tests.
#[cfg(test)]
include!("tab_order.rs");

#[cfg(any())]
mod tab_order;

#[cfg(test)]
#[path = "test/tab_order/test.rs"]
mod tab_order_test;

#[cfg(test)]
#[path = "test/data_grid/test.rs"]
mod data_grid_test;

/// What RXP knows how to draw. See [`Runtime::classes`].
pub const CLASSES: &[&str] = &[
    "BitBtn",
    "Button",
    "CheckBox",
    "Chips",
    "Breadcrumb",
    "Pagination",
    "MultiSelect",
    "CheckListBox",
    "Bevel",
    "Badge",
    "Card",
    "Drawer",
    "CategoryPanel",
    "ColorBox",
    "ComboBox",
    "DateTimePicker",
    "Edit",
    "Grid",
    "GroupBox",
    "Image",
    "Label",
    "LinkLabel",
    "ListBox",
    "MaskEdit",
    "ListView",
    "Memo",
    "PageControl",
    "PaintBox",
    "Panel",
    "ProgressBar",
    "RadioButton",
    "RadioGroup",
    "ScrollBar",
    "ScrollBox",
    "Shape",
    "SpeedButton",
    "SpinEdit",
    "Splitter",
    "StatusBar",
    "TabSheet",
    "Toggle",
    "ToolBar",
    "TrackBar",
    "TreeView",
];
