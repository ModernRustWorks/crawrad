use crawrad_note::note;
use std::collections::HashMap;

use crawrad_actions::Panel;
use makepad_widgets::*;

#[note("ide-app-layout-dock")]
pub const DOCK: &[LiveId] = id!(dock);

#[note("ide-app-layout-left-group")]
pub const LEFT_GROUP: LiveId = live_id!(dock_left);
pub const CENTER_GROUP: LiveId = live_id!(dock_center);
pub const RIGHT_GROUP: LiveId = live_id!(dock_right);

#[note("ide-app-layout-tab-of")]
pub fn tab_of(panel: Panel) -> LiveId {
    LiveId::from_str(&panel.tab_name())
}

#[note("ide-app-layout-register-tab-names")]
pub fn register_tab_names() {
    crawrad_runtime_makepad::dock::register_tab_names(Panel::ALL.iter().map(|p| p.tab_name()));
}

#[note("ide-app-layout-panel-of-tab")]
pub fn panel_of_tab(tab: LiveId) -> Option<Panel> {
    Panel::ALL.iter().copied().find(|p| tab_of(*p) == tab)
}

#[note("ide-app-layout-template-of")]
pub fn template_of(panel: Panel) -> LiveId {
    LiveId::from_str(&panel.template_name())
}

#[note("ide-app-layout-tab-template")]
pub const TAB_TEMPLATE: LiveId = live_id!(PermanentTab);

#[note("ide-app-layout-tab-item")]
pub fn tab_item(panel: Panel) -> DockItem {
    DockItem::Tab {
        name: panel.label().to_string(),
        template: TAB_TEMPLATE,
        kind: template_of(panel),
    }
}

#[note("ide-app-layout-lone-group")]
pub fn lone_group(panel: Panel) -> DockItem {
    DockItem::Tabs {
        tabs: vec![tab_of(panel)],
        selected: 0,
        closable: true,
    }
}

// --------------------------------------------------------------- persistence

pub use crawrad_runtime_makepad::dock::{encode, side_by_side, with_root_restored};

#[note("ide-app-layout-decode")]
pub fn decode(text: &str) -> Option<HashMap<LiveId, DockItem>> {
    crawrad_runtime_makepad::dock::decode(text, |tab| panel_of_tab(tab).is_some())
}

pub use crawrad_project::desktop::{
    CHAT_WIDTH, DISCARDED, DOCK_WIDTH, dock_layout_file_path as file_path,
};
