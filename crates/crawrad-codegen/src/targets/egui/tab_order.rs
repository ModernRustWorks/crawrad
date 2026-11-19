//! **The Tab order of the form's controls** in the egui target.
//!
//! egui walks the focus in the order widgets ask for it, which here is the
//! drawing order — the Z order. So every control with a place in the Tab
//! sequence (`targets::tab_places`) leaves a stop while it draws: its place
//! and its rect. The form hands the stops to `CrawradTabOrder`
//! (`crawrad-runtime-egui/src/tab_order.rs`, copied into the designer file),
//! which answers Tab and Shift+Tab in that order. A container that never
//! takes the focus has no stop.

use super::recorte;
use crate::targets::tab_places;
use crawrad_ir::{Document, Node, NodeId};
use crawrad_note::note;
use std::collections::BTreeMap;

#[note("targets-egui-tab-order-tab-order")]
const TAB_ORDER: &str = include_str!("../../../../crawrad-runtime-egui/src/tab_order.rs");

#[note("targets-egui-tab-order-taborder")]
pub struct TabOrder {
    /// Each stop's place in the sequence, as a slice literal.
    places: BTreeMap<NodeId, String>,
    /// A drawn control has a `FocusControl` to honour.
    links: bool,
}

#[note("targets-egui-tab-order-plan")]
pub fn plan(reg: &crawrad_registry::Registry, doc: &Document, drawn: &[&Node]) -> Option<TabOrder> {
    let places: BTreeMap<NodeId, String> = drawn
        .iter()
        .filter_map(|n| {
            let place = tab_places::place(reg, doc, n)?;
            Some((n.id.clone(), tab_places::literal(&place)))
        })
        .collect();
    let links = drawn.iter().any(|n| {
        reg.get(&n.class)
            .is_some_and(|d| d.props.iter().any(|p| p.key == "focus_control"))
    });
    (!places.is_empty()).then_some(TabOrder { places, links })
}

impl TabOrder {
    #[note("targets-egui-tab-order-taborder-stop-line")]
    pub fn stop_line(&self, n: &Node, name: &str) -> String {
        let Some(place) = self.places.get(&n.id) else {
            return String::new();
        };
        // a Memo with WantTabs types the Tab instead of leaving
        let keeps_tab = if n.class == "Memo" {
            format!("self.{name}.want_tabs")
        } else {
            "false".to_string()
        };
        format!(
            "        if self.{name}.visible && self.{name}.enabled {{\n\
             \x20           crawrad_tab.push(({place}, r, {keeps_tab}, \"{name}\"));\n\
             \x20       }}\n"
        )
    }

    /// A `Label`'s click: its `FocusControl` takes the keyboard,
    /// through the stops the form keeps.
    pub fn link_line(&self, reg: &crawrad_registry::Registry, n: &Node, name: &str) -> String {
        let links = reg
            .get(&n.class)
            .is_some_and(|d| d.props.iter().any(|p| p.key == "focus_control"));
        if !links {
            return String::new();
        }
        format!(
            "        if self.{name}.visible && self.{name}.enabled && !self.{name}.focus_control.is_empty()\n\
             \x20           && ui.interact(r, ui.id().with(\"{name}_focus_link\"), egui::Sense::click()).clicked()\n\
             \x20       {{\n\
             \x20           self.crawrad_tab.focus(ui, &self.{name}.focus_control);\n\
             \x20       }}\n"
        )
    }

    #[note("targets-egui-tab-order-taborder-before")]
    pub fn before(&self) -> &'static str {
        "        // Tab walks the controls in TabOrder (see `CrawradTabOrder`).\n\
         \x20       self.crawrad_tab.before(ui);\n\
         \x20       let mut crawrad_tab: Vec<CrawradTabStop> = Vec::new();\n"
    }

    #[note("targets-egui-tab-order-taborder-after")]
    pub fn after(&self) -> &'static str {
        "        self.crawrad_tab.after(ui, &mut crawrad_tab);\n"
    }

    #[note("targets-egui-tab-order-taborder-field")]
    pub fn field(&self) -> (&'static str, &'static str) {
        (
            "    crawrad_tab: CrawradTabOrder,\n",
            "            crawrad_tab: CrawradTabOrder::default(),\n",
        )
    }

    #[note("targets-egui-tab-order-taborder-items")]
    pub fn items(&self) -> String {
        let mut items = recorte(TAB_ORDER, "tab_order");
        if self.links {
            items.push_str(&recorte(TAB_ORDER, "tab_focus"));
        }
        items
    }
}
