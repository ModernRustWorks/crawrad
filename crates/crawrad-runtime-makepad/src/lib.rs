//! # crawrad-runtime-makepad — native Makepad runtime
//!
//! Runtime layer for [Makepad](https://github.com/makepad/makepad), providing a
//! modern GPU-accelerated rendering engine designed to support Desktop (Windows, macOS, Linux),
//! WebAssembly, and Mobile platforms.
//!
//! ## Architecture
//!
//! This crate implements the [`crawrad_runtime::Runtime`] contract for the `"makepad"` target.
//! It structures form nodes using a common property hierarchy
//! (ComponenteBase -> ControleVisualBase -> ControleJanelaBase) and draws them on
//! the canvas, frame by frame (ADR 0027).

#![forbid(unsafe_code)]

pub use makepad_widgets;

pub mod breadcrumb;
pub mod canvas;
pub mod chips;
pub mod components;
pub mod designer;
pub mod pagination;
pub mod shared_geometry;
pub mod shared_theme;
pub mod theme_dark;
pub mod theme_light;

pub use canvas::DesignCanvas;
pub use components::comum::*;
pub mod anchors;
pub mod badge;
pub mod bevel;
pub mod borderless;
pub mod card;
pub mod check;
pub mod check_list;
pub mod color_picker;
pub mod column_splitter;
pub mod combo_box;
pub mod cursor;
pub mod data_grid;
pub mod dialogs;
pub mod diff_view;
pub mod dock;
pub mod drawer;
pub mod drive;
pub mod edit;
pub mod enabled;
pub mod events;
pub mod glyph;
pub mod history_panel;
pub mod keys;
pub mod label;
pub mod list;
pub mod list_view;
pub mod look;
pub mod memo;
pub mod menu;
pub mod multi_select;
pub mod paint;
pub mod palette;
pub mod picture;
pub mod progress;
pub mod progress_card;
pub mod radio_group;
pub mod scheme_preview;
pub mod segment_list;
pub mod shape;
pub mod size_preview_view;
pub mod speed;
pub mod split;
pub mod status;
pub mod string_grid;
pub mod tab_order;
pub mod tabs;
pub mod timer;
pub mod translations_view;
pub mod welcome;
pub mod window;

pub use color_picker::{
    CrawradColorSquare, CrawradColorSquareAction, CrawradColorSquareRef,
    CrawradColorSquareWidgetExt, CrawradColorSquareWidgetRefExt, CrawradHueStrip,
    CrawradHueStripAction, CrawradHueStripRef, CrawradHueStripWidgetExt,
    CrawradHueStripWidgetRefExt, square_fractions, square_marker, strip_fraction,
};
pub use column_splitter::{CrawradColumnSplitterAction, CrawradColumnSplitterWidgetRefExt};
pub use data_grid::{
    CrawradDataGrid, CrawradDataGridRef, CrawradDataGridWidgetExt, CrawradDataGridWidgetRefExt,
};
pub use diff_view::{
    CrawradDiffView, CrawradDiffViewRef, CrawradDiffViewWidgetExt, CrawradDiffViewWidgetRefExt,
    DiffRowKind, DiffViewRow,
};
pub use history_panel::{
    CrawradHistoryPanel, CrawradHistoryPanelRef, CrawradHistoryPanelWidgetExt,
    CrawradHistoryPanelWidgetRefExt, HistoryEntryView, HistoryMode, HistoryPanelAction,
};
pub use progress_card::{
    CrawradProgressCard, CrawradProgressCardRef, CrawradProgressCardWidgetExt,
    CrawradProgressCardWidgetRefExt,
};
pub use scheme_preview::{
    CrawradSchemePreview, CrawradSchemePreviewRef, CrawradSchemePreviewWidgetExt,
    CrawradSchemePreviewWidgetRefExt, PreviewRow,
};
pub use segment_list::{
    CrawradSegmentList, CrawradSegmentListRef, CrawradSegmentListWidgetExt,
    CrawradSegmentListWidgetRefExt, Segment, SegmentKind,
};
pub use size_preview_view::{
    SizeFrame, SizeItem, SizePreviewAction, SizePreviewView, SizePreviewViewRef,
    SizePreviewViewWidgetRefExt,
};
pub use translations_view::{
    CellView, GridGroup, GridRow, LangChip, TranslationsAction, TranslationsView,
    TranslationsViewRef, TranslationsViewWidgetRefExt, cell_text, chip_label,
};
pub use welcome::{
    CrawradWelcomePage, CrawradWelcomePageRef, CrawradWelcomePageWidgetExt,
    CrawradWelcomePageWidgetRefExt, WelcomeAction, WelcomeRowView, reopen_note,
};

#[note("live-design")]
pub fn live_design(cx: &mut makepad_widgets::Cx) {
    crate::theme_dark::live_design(cx);
    crate::theme_light::live_design(cx);
    crate::borderless::live_design(cx);
    crate::breadcrumb::live_design(cx);
    crate::canvas::live_design(cx);
    crate::card::live_design(cx);
    crate::check_list::live_design(cx);
    crate::color_picker::live_design(cx);
    crate::column_splitter::live_design(cx);
    crate::chips::live_design(cx);
    crate::data_grid::live_design(cx);
    crate::dialogs::live_design(cx);
    crate::diff_view::live_design(cx);
    crate::drawer::live_design(cx);
    crate::history_panel::live_design(cx);
    crate::welcome::live_design(cx);
    crate::list::live_design(cx);
    crate::list_view::live_design(cx);
    crate::multi_select::live_design(cx);
    crate::combo_box::live_design(cx);
    crate::pagination::live_design(cx);
    crate::paint::live_design(cx);
    crate::progress_card::live_design(cx);
    crate::radio_group::live_design(cx);
    crate::scheme_preview::live_design(cx);
    crate::segment_list::live_design(cx);
    crate::size_preview_view::live_design(cx);
    crate::string_grid::live_design(cx);
    crate::translations_view::live_design(cx);
}

use std::collections::{BTreeMap, BTreeSet};
use std::sync::LazyLock;

use crawrad_ir::{Document, NodeId, Rect, Value};
use crawrad_note::note;
use crawrad_runtime::{
    GuideLine, Mode, Runtime, RuntimeError, RuntimeEvent, Scale, ViewportPx, WindowControlBase,
};

#[note("target-name")]
pub const ALVO: &str = "makepad";

#[note("classes")]
pub static CLASSES: LazyLock<Vec<&'static str>> = LazyLock::new(components::classes);

#[note("transformacao")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transformacao {
    pub origem_x: f32,
    pub origem_y: f32,
    pub escala: f32,
}

impl Transformacao {
    pub fn new(origem_x: f32, origem_y: f32, escala: f32) -> Self {
        Self {
            origem_x,
            origem_y,
            escala,
        }
    }

    pub fn screen_point(&self, x: f32, y: f32) -> (f32, f32) {
        (
            self.origem_x + x * self.escala,
            self.origem_y + y * self.escala,
        )
    }

    pub fn em_design(&self, x: f32, y: f32) -> (f32, f32) {
        let e = self.escala.max(f32::EPSILON);
        ((x - self.origem_x) / e, (y - self.origem_y) / e)
    }

    pub fn medida(&self, v: f32) -> f32 {
        v * self.escala
    }
}

#[note("makepadctl")]
#[derive(Debug, Clone, PartialEq)]
pub struct MakepadCtl {
    pub base: WindowControlBase,
    pub props: BTreeMap<String, Value>,
    pub itens: Vec<crawrad_ir::Item>,
    pub item_symbols: Vec<String>,
    pub escolhido: Option<usize>,
    pub marcado: bool,
    pub pagina_ativa: usize,
    pub somente_leitura: bool,
    pub intervalo: i64,
    pub borda: f32,
    pub scroll_y: f32,
    /// Which bars the container shows: `ssNone`, `ssHorizontal`, `ssVertical`
    /// or `ssBoth`. Read the same way as the RXP (`crawrad-runtime-egui/src/lib.rs:655`).
    pub scroll_bars: String,
    /// May the `ScrollBox` scroll sideways? `HorzScrollBar` and `AutoScroll`,
    /// by [`crawrad_ir::scrolling::scroll_box_axes`].
    pub barra_h: bool,
    /// May the `ScrollBox` scroll up and down? `VertScrollBar` and
    /// `AutoScroll`, by [`crawrad_ir::scrolling::scroll_box_axes`].
    pub barra_v: bool,
    /// The `Position` of a `ScrollBar`, from 0 to 100.
    ///
    /// Makepad's `ScrollBar` previously drew a
    /// 30% thumb stationary at the start of the bar and `desenhar`'s
    /// `_c: &MakepadCtl` confessed the discard: the RXP read `posicao_scroll` in
    /// four places and this runtime in zero.
    pub posicao_scroll: f32,
    /// **HOW THIS CLASS PAINTS ITSELF**, when it does not bring code for that.
    ///
    /// Empty for the twenty native ones: their drawing is Rust, in
    /// `componentes/`, and stays that way. This is for third-party components, which
    /// **cannot** bring code — the IDE loads no one's library to draw (ADR 0036).
    ///
    /// Populated by whoever builds `MakepadCtl` from the catalogue; painted
    /// by [`components::declarado::pintar`].
    pub desenho: crawrad_ir::drawing::Drawing,
    /// The first rows of a bound `Grid`'s table, read at design time.
    pub design_rows: Option<crawrad_ir::design_rows::GridRows>,
    /// Column specifications for a `Grid`.
    pub columns: Vec<crawrad_runtime::grid::ColumnSpec>,
    /// The class's palette icon, handed in by the shell with the drawings; a
    /// non-visual badge wears it, and falls back to the initials when empty.
    pub icon: Vec<crawrad_ir::icon::IconStep>,
    /// The document's class, before any painting substitution (native base,
    /// declared non-visual badge): the badge writes its initials.
    pub declared_class: String,
}

impl std::ops::Deref for MakepadCtl {
    type Target = WindowControlBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for MakepadCtl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[note("makepadruntime")]
#[derive(Debug)]
pub struct MakepadRuntime {
    ctls: BTreeMap<NodeId, MakepadCtl>,
    ordem: Vec<NodeId>,
    sel: Vec<NodeId>,
    viewport: Option<ViewportPx>,
    scale: f32,
    zoom: f32,
    grid: f32,
    modo: Mode,
    form_rect: Rect,
    form_id: NodeId,
    /// The form (root) font, if defined.
    pub form_font: Option<crawrad_ir::Font>,
    aberto: bool,
    marquee: Option<Rect>,
    badges: Vec<(NodeId, String)>,
    /// The form checker's marks.
    pins: Vec<crawrad_runtime::FindingPin>,
    /// The components that come from the base form.
    inherited: Vec<NodeId>,
    guias: Vec<GuideLine>,
    /// Snap is on: the grid is drawn strong, else faint.
    snap: bool,
    /// While a move snaps: where it lands and where the pointer would have
    /// put it, in form coordinates.
    ghost: Option<(Rect, Rect)>,
    pendentes: Vec<RuntimeEvent>,
    capturando: bool,
    /// **The DECLARED drawings, by class.**
    ///
    /// Empty is normal, and is the state of an IDE without third-party components
    /// installed: the twenty native ones draw via code.
    ///
    /// Populated by the shell, which IS what has the catalogue — this adapter
    /// intentionally does not depend on it (ADR 0027: widget translates gesture, designer
    /// decides). Passing the data instead of importing the catalogue is what keeps this
    /// boundary standing.
    desenhos: crawrad_ir::drawing::DeclaredLooks,
    /// The design-time rows of the bound grids, by node. The shell
    /// hands them in, like the drawings: this adapter reads no database.
    design_rows: BTreeMap<NodeId, crawrad_ir::design_rows::GridRows>,
}

impl Default for MakepadRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl MakepadRuntime {
    #[note("makepadruntime-new")]
    pub fn new() -> Self {
        Self {
            ctls: BTreeMap::new(),
            ordem: Vec::new(),
            sel: Vec::new(),
            viewport: None,
            scale: 1.0,
            zoom: 1.0,
            grid: 8.0,
            modo: Mode::Design,
            form_rect: Rect::new(0.0, 0.0, 640.0, 480.0),
            form_id: String::new(),
            form_font: None,
            aberto: false,
            marquee: None,
            badges: Vec::new(),
            pins: Vec::new(),
            inherited: Vec::new(),
            guias: Vec::new(),
            snap: true,
            ghost: None,
            pendentes: Vec::new(),
            capturando: false,
            desenhos: crawrad_ir::drawing::DeclaredLooks::default(),
            design_rows: BTreeMap::new(),
        }
    }

    #[note("makepadruntime-set-snap")]
    pub fn set_snap(&mut self, on: bool) {
        self.snap = on;
    }

    #[note("makepadruntime-set-snap-ghost")]
    pub fn set_snap_ghost(&mut self, ghost: Option<(Rect, Rect)>) {
        self.ghost = ghost;
    }

    #[note("makepadruntime-com-desenhos")]
    pub fn com_desenhos(&mut self, desenhos: crawrad_ir::drawing::DeclaredLooks) {
        self.desenhos = desenhos;
    }

    #[note("makepadruntime-set-design-rows")]
    pub fn set_design_rows(
        &mut self,
        rows: BTreeMap<NodeId, crawrad_ir::design_rows::GridRows>,
    ) -> bool {
        if rows == self.design_rows {
            return false;
        }
        for (id, ctl) in &mut self.ctls {
            ctl.design_rows = rows.get(id).cloned();
        }
        self.design_rows = rows;
        true
    }

    #[note("makepadruntime-quantidade")]
    pub fn quantidade(&self) -> usize {
        self.ctls.len()
    }

    #[note("makepadruntime-ctl")]
    pub fn ctl(&self, id: &str) -> Option<&MakepadCtl> {
        self.ctls.get(id)
    }

    #[note("makepadruntime-injetar")]
    pub fn injetar(&mut self, e: RuntimeEvent) {
        self.pendentes.push(e);
    }

    #[note("makepadruntime-desenhavel")]
    pub fn desenhavel(&self, id: &str) -> bool {
        id == self.form_id || self.ordem.iter().any(|i| i == id)
    }

    #[note("makepadruntime-rect-da-alca")]
    pub fn rect_da_alca(&self) -> Option<Rect> {
        let id = self.sel.first()?;
        if *id == self.form_id || !self.desenhavel(id) {
            return None;
        }
        // 4. never a **non-visual**: its badge has fixed size and does not
        //    resize, and `Designer::hit_handle` already rejected dragging
        //    (`crawrad-designer/src/lib.rs:519`). While this rejection was missing, a
        //    selected `ActionList` appeared with eight handles and the resize
        //    arrow — and neither did anything.
        // 5. never a control with **derived geometry** (e.g. `TabSheet`),
        //    whose rectangle is imposed by the parent and does not resize freely.
        self.ctls
            .get(id)
            .filter(|c| c.visual && !c.derived_geometry)
            .map(|c| c.rect)
    }

    #[note("makepadruntime-form-rect")]
    pub fn form_rect(&self) -> Rect {
        self.form_rect
    }

    #[note("makepadruntime-escala")]
    pub fn escala(&self) -> f32 {
        self.scale * self.zoom
    }

    #[note("makepadruntime-transformacao")]
    pub fn transformacao(&self) -> Transformacao {
        let (ox, oy) = if let Some(vp) = self.viewport {
            (vp.x as f32, vp.y as f32)
        } else {
            (0.0, 0.0)
        };
        Transformacao::new(ox, oy, self.scale * self.zoom)
    }

    fn sincronizar(&mut self, doc: &Document, id: &NodeId) {
        let Some(n) = doc.node(id) else {
            return;
        };
        // The SAME response as `no_canvas`, and not a second duplicate list: the
        // two used to be copies, and duplicate lists are the defect family that
        // cost the most here. The RXP does the same (`crawrad-runtime-egui/src/lib.rs:603`).
        // An inheriting class is painted as its native base.
        let mut painted = self.desenhos.painted_class(&n.class).to_string();
        // A catalogue class declared `visual: false` with no drawing of its own
        // is a badge, decided by the flag and never by its name.
        if self.desenhos.non_visual.contains(&n.class)
            && components::desenho_de(&painted).class_name() != painted
        {
            painted = components::non_visual::DECLARED_CLASS.to_string();
        }
        let visual = e_visual_no_canvas(&painted);
        let mut base = WindowControlBase::from_doc(id, n, doc, visual);
        base.class = painted;

        let booleano = |no: &crawrad_ir::Node, k: &str, padrao: bool| match no.props.get(k) {
            Some(Value::Bool(b)) => *b,
            _ => padrao,
        };

        let itens = match n.props.get("items") {
            Some(v) => crawrad_ir::Item::list_from(v),
            None => Vec::new(),
        };
        let item_symbols = crawrad_ir::Item::linked_symbols(doc, n);
        let escolhido = match n.props.get("item_index") {
            Some(Value::Int(i)) if *i >= 0 => Some(*i as usize),
            _ => None,
        };
        let marcado = booleano(n, "checked", false);
        let pagina_ativa = match n.props.get("active_page") {
            Some(Value::Int(i)) if *i >= 0 => *i as usize,
            _ => 0,
        };
        let somente_leitura = booleano(n, "readonly", false);
        let intervalo = match n.props.get("interval") {
            Some(Value::Int(i)) => *i,
            _ => 0,
        };
        let borda = match n.props.get("border_width") {
            Some(Value::Int(i)) => (*i).max(0) as f32,
            _ => 1.0,
        };

        let scroll_bars = match n.props.get("scroll_bars") {
            Some(Value::Enum(s)) => s.clone(),
            Some(Value::Str(s)) => s.clone(),
            _ => String::new(),
        };
        // The same rule the two generated apps follow, so a box with
        // `AutoScroll` off shows no bar here either
        let eixos = crawrad_ir::scrolling::scroll_box_axes(
            booleano(n, "auto_scroll", true),
            booleano(n, "horz_scroll_bar", false),
            booleano(n, "vert_scroll_bar", false),
        );
        let (barra_h, barra_v) = (eixos.horizontal, eixos.vertical);
        let posicao_scroll = match n.props.get("position") {
            Some(Value::Int(i)) => *i as f32,
            _ => 0.0,
        };

        let prev_scroll = self.ctls.get(id).map(|c| c.scroll_y).unwrap_or(0.0);
        let desenho = self
            .desenhos
            .drawings
            .get(&n.class)
            .cloned()
            .unwrap_or_default();
        let columns = crawrad_runtime::grid::ColumnSpec::of_grid(doc, n);
        let icon = self.desenhos.icon_of(&n.class).to_vec();
        let mut ctl = MakepadCtl {
            declared_class: n.class.clone(),
            icon,
            desenho,
            base,
            props: BTreeMap::new(),
            itens,
            item_symbols,
            escolhido,
            marcado,
            pagina_ativa,
            somente_leitura,
            intervalo,
            borda,
            scroll_y: prev_scroll,
            scroll_bars,
            barra_h,
            barra_v,
            posicao_scroll,
            design_rows: self.design_rows.get(id).cloned(),
            columns,
        };

        for (chave, valor) in n.props_for(ALVO) {
            ctl.props.insert(chave.to_string(), valor.clone());
        }

        self.ctls.insert(id.clone(), ctl);
    }
}

impl Runtime for MakepadRuntime {
    fn target(&self) -> &'static str {
        ALVO
    }

    fn classes(&self) -> &'static [&'static str] {
        CLASSES.as_slice()
    }

    fn open(&mut self, doc: &Document) -> Result<(), RuntimeError> {
        let raiz = doc
            .node(&doc.root)
            .ok_or_else(|| RuntimeError::Backend("the document has no root".into()))?;
        self.form_rect = raiz.rect().unwrap_or(Rect::new(0.0, 0.0, 640.0, 480.0));
        self.form_id = doc.root.clone();
        self.form_font = raiz.props.get("font").and_then(Value::as_font).cloned();
        self.ctls.clear();
        self.ordem.clear();

        for id in doc.in_paint_order() {
            if id == doc.root {
                continue;
            }
            self.sincronizar(doc, &id);
            if no_canvas(doc, &id) {
                self.ordem.push(id);
            }
        }
        self.ordem.sort_by_key(|id| {
            doc.node(id).is_some_and(|n| {
                n.class == "Badge"
                    && n.props.get("associate").is_some_and(|v| match v {
                        Value::Ref(s) | Value::Str(s) => !s.is_empty(),
                        _ => false,
                    })
            })
        });
        self.aberto = true;
        Ok(())
    }

    fn update(&mut self, doc: &Document, touched: &BTreeSet<NodeId>) -> Result<(), RuntimeError> {
        if !self.aberto {
            return Err(RuntimeError::NotStarted);
        }

        self.ctls.retain(|id, _| doc.node(id).is_some());

        if touched.contains(&doc.root)
            && let Some(raiz) = doc.node(&doc.root)
        {
            self.form_rect = raiz.rect().unwrap_or(self.form_rect);
            self.form_font = raiz.props.get("font").and_then(Value::as_font).cloned();
        }

        self.ordem.clear();
        for id in doc.in_paint_order() {
            if id == doc.root {
                continue;
            }
            let mut sync = touched.contains(&id) || !self.ctls.contains_key(&id);
            if !sync
                && let Some(node) = doc.node(&id)
                && node.class == "TreeView"
                && let Some(Value::Ref(img_ref) | Value::Str(img_ref)) = node.props.get("images")
                && !img_ref.trim().is_empty()
            {
                let is_touched = touched.contains(img_ref.as_str())
                    || doc
                        .node_by_name(img_ref)
                        .is_some_and(|target| touched.contains(&target.id));
                if is_touched {
                    sync = true;
                }
            }
            if sync {
                self.sincronizar(doc, &id);
            }
            if no_canvas(doc, &id) {
                self.ordem.push(id);
            }
        }
        self.ordem.sort_by_key(|id| {
            doc.node(id).is_some_and(|n| {
                n.class == "Badge"
                    && n.props.get("associate").is_some_and(|v| match v {
                        Value::Ref(s) | Value::Str(s) => !s.is_empty(),
                        _ => false,
                    })
            })
        });
        Ok(())
    }

    fn set_viewport(&mut self, vp: ViewportPx, scale: Scale) {
        self.viewport = Some(vp);
        self.scale = scale.0;
    }

    fn set_selection(&mut self, sel: &[NodeId]) {
        self.sel = sel.to_vec();
    }

    fn set_marquee(&mut self, rect: Option<Rect>) {
        self.marquee = rect.filter(|r| r.w > 0.0 || r.h > 0.0);
    }

    fn set_badges(&mut self, badges: &[(NodeId, String)]) {
        self.badges = badges.to_vec();
    }

    fn set_findings(&mut self, pins: &[crawrad_runtime::FindingPin]) {
        self.pins = pins.to_vec();
    }

    fn set_inherited(&mut self, inherited: &[NodeId]) {
        self.inherited = inherited.to_vec();
    }

    fn set_grid(&mut self, grid: f32) {
        self.grid = grid;
    }

    fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }

    fn set_mode(&mut self, modo: Mode) {
        self.modo = modo;
    }

    fn set_guides(&mut self, guias: &[GuideLine]) {
        self.guias = guias.to_vec();
    }

    fn frame(&mut self) {}

    fn busy(&self) -> bool {
        self.capturando
    }

    fn drain_events(&mut self) -> Vec<RuntimeEvent> {
        std::mem::take(&mut self.pendentes)
    }

    fn close(&mut self) {
        self.ctls.clear();
        self.ordem.clear();
        self.form_font = None;
        self.aberto = false;
    }
}

#[note("e-visual-no-canvas")]
fn e_visual_no_canvas(class: &str) -> bool {
    components::e_visual(class)
}

#[note("no-canvas")]
pub fn no_canvas(doc: &Document, id: &str) -> bool {
    crawrad_runtime::on_surface(doc, id, e_visual_no_canvas) && !drawn_by_its_owner(doc, id)
}

#[note("drawn-by-its-owner")]
fn drawn_by_its_owner(doc: &Document, id: &str) -> bool {
    let Some(node) = doc.node(id) else {
        return false;
    };
    let Some(owner) = components::desenho_de(&node.class).drawn_by_owner() else {
        return false;
    };
    doc.parent_of(id)
        .and_then(|p| doc.node(p))
        .is_some_and(|p| p.class == owner)
}

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "test/declared/test.rs"]
mod declared_test;

#[cfg(test)]
#[path = "test/content/test.rs"]
mod content_test;

#[cfg(test)]
#[path = "test/probe/test.rs"]
mod probe_test;

#[cfg(test)]
#[path = "test/badge/test.rs"]
mod badge_test;

#[cfg(test)]
#[path = "test/data_grid/test.rs"]
mod data_grid_test;

#[cfg(test)]
#[path = "test/font/test.rs"]
mod font_test;
