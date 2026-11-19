use crate::components::base::page_props;
use crate::{Children, ComponentDesc, Events, Props, component};
use crawrad_note::note;

#[note("components-page-control-tab-strip-height")]
pub const TAB_STRIP_HEIGHT: f32 = 24.0;

#[note("components-page-control-tab-width")]
pub const TAB_WIDTH: f32 = 92.0;

#[note("components-page-control-plus-button-width")]
pub const PLUS_BUTTON_WIDTH: f32 = 22.0;

#[note("components-page-control-tab-scroll-button-width")]
pub const TAB_SCROLL_BUTTON_WIDTH: f32 = 16.0;

#[note("components-page-control-tabwindow")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TabWindow {
    /// Position (among the shown tabs) of the first tab drawn.
    pub first: usize,
    /// How many tabs are drawn, starting at `first`.
    pub count: usize,
    /// Whether the `<` `>` buttons are shown (the tabs overflow).
    pub arrows: bool,
}

#[note("components-page-control-tab-window")]
pub fn tab_window(width: f32, shown: usize, active: usize, plus: bool) -> TabWindow {
    if shown == 0 {
        return TabWindow {
            first: 0,
            count: 0,
            arrows: false,
        };
    }
    let plus_w = if plus { PLUS_BUTTON_WIDTH } else { 0.0 };
    let mut avail = width - plus_w;
    if shown as f32 * TAB_WIDTH <= avail {
        return TabWindow {
            first: 0,
            count: shown,
            arrows: false,
        };
    }
    avail -= 2.0 * TAB_SCROLL_BUTTON_WIDTH;
    let per = (avail / TAB_WIDTH).floor().max(1.0) as usize;
    let count = per.min(shown);
    let active = active.min(shown - 1);
    let mut first = active.saturating_sub(count - 1);
    first = first.min(shown - count);
    TabWindow {
        first,
        count,
        arrows: true,
    }
}

#[note("components-page-control-tab-scroll-left-x")]
pub fn tab_scroll_left_x(width: f32, plus: bool) -> f32 {
    width - (if plus { PLUS_BUTTON_WIDTH } else { 0.0 }) - 2.0 * TAB_SCROLL_BUTTON_WIDTH
}

#[note("components-page-control-tabs-at-bottom")]
pub fn tabs_at_bottom(node: &crawrad_ir::Node) -> bool {
    matches!(node.props.get("tab_position"), Some(crawrad_ir::Value::Enum(p)) if p == "tpBottom")
}

#[note("components-page-control-tab-strip-top")]
pub fn tab_strip_top(h: f32, bottom: bool) -> f32 {
    if bottom {
        (h - TAB_STRIP_HEIGHT).max(0.0)
    } else {
        0.0
    }
}

#[note("components-page-control-pages-top")]
pub fn pages_top(bottom: bool) -> f32 {
    if bottom { 0.0 } else { TAB_STRIP_HEIGHT }
}

#[note("components-page-control-page-control")]
pub fn page_control() -> ComponentDesc {
    let mut d = component!(
        "PageControl",
        display: "PageControl",
        base_name: "page_control",
        palette: "Standard",
        prefix: "pgc",
        size: (320.0, 200.0),
        children: Children::Area,
        props: [Props::active_page(0), Props::tab_position("tpTop")],
        // Both targets already switched pages; now the Inspector offers them
        events: [Events::CHANGING, Events::CHANGE, Events::RESIZE],
    );
    d.props.extend(crate::components::base::appearance_props());
    d
}

#[note("components-page-control-tab-sheet")]
pub fn tab_sheet() -> ComponentDesc {
    component!(
        "TabSheet",
        display: "TabSheet",
        base_name: "tab_sheet",
        palette: "",
        prefix: "tab",
        size: (320.0, 176.0),
        children: Children::Area,
        derived_geometry: true,
        base: page_props(),
        props: [Props::caption("TabSheet"), Props::tab_visible(true)],
    )
}
