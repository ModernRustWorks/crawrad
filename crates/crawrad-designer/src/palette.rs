//! The Makepad shell's Toolbox — its rows, and where they come from.
//!
//! # A single source, and the defect this closes
//!
//! Until this turn the palette was **two** central lists: seventeen lines
//! written by hand in `live_design!` (`item_btn = <CrawradListRow> { text:
//! "Button" }`) and a fourteen-entry table in `handle_actions` linking
//! each `item_*` back to the class. The class name appeared twice, the row id
//! as well, and adding a component required editing two places.
//!
//! Worse: forgetting the second was **silent**. The rows `ActionList`,
//! `ImageList`, and `Timer` were in the DSL and were **not** in the table — the three
//! appeared on screen and armed nothing. Nobody noticed, because nothing
//! complains.
//!
//! The component list already exists and is already the single source: the catalog
//! `crawrad-registry`, which `crawrad-ide` already consumes (`Registry::palette`,
//! `crates/crawrad-registry/src/lib.rs:544`) and which egui's Toolbox already reads
//! (`crates/crawrad-ide/src/paleta.rs:262`). This module is the translation of the
//! catalog into what `PortalList` needs: a flat sequence of rows.
//!
//! # Why a FLAT sequence
//!
//! Because that is what a `PortalList` consumes: it asks for the index
//! (`next_visible_item`) and returns the item at that index
//! (`portal_list.rs:563-592`). Section label and component item are
//! sibling rows in the same count — as they already are in egui's Toolbox, where the
//! label purposely has the same line height
//! (`crates/crawrad-ide/src/paleta.rs:46`).
//!
//! # The filter lives HERE, and not in drawing
//!
//! The `Filter` field of the Makepad shell existed in the DSL and was **dead UI**:
//! nobody read what the user typed. The missing rule is the same one egui's
//! Toolbox already applies (`crates/crawrad-ide/src/paleta.rs:470-480`), and
//! it goes into the pure function instead of becoming an `if` scattered through the drawing —
//! otherwise "section that became empty disappears" would be a decision of whoever paints, and
//! could not be measured without opening a window.
//!
//! # What this module does NOT do
//!
//! It does not draw and does not know Makepad: it is a pure function over the catalog, and that
//! is why the order and content of the palette can be measured by tests without
//! opening any window.

use crawrad_note::note;
use crawrad_registry::{ComponentDesc, Registry};

#[note("palette-no-match")]
pub const NO_MATCH: &str = "no component matches";

#[note("palette-row")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Row {
    /// The label of a group — `Additional`, `Standard`, `System`.
    Section(&'static str),
    /// A component from the catalog.
    Item {
        /// The class, which is what the tool arms.
        class: &'static str,
        /// The name the row shows — the descriptor's `display`.
        label: &'static str,
        /// Why the project cannot use it (greyed, not armed), e.g. SQLite on the web.
        blocked: Option<&'static str>,
    },
    /// The filter matched nothing, and the list says so instead of remaining
    /// blank. It is a row like the others because `PortalList` only knows how to draw
    /// rows.
    Empty,
}

impl Row {
    #[note("palette-row-class")]
    pub fn class(&self) -> Option<&'static str> {
        match self {
            Row::Section(_) | Row::Empty => None,
            Row::Item {
                blocked: Some(_), ..
            } => None,
            Row::Item { class, .. } => Some(class),
        }
    }

    /// The greyed row's tooltip, worded as a greyed menu row's.
    #[note("palette-row-hint")]
    pub fn hint(&self) -> Option<String> {
        match self {
            Row::Item {
                label,
                blocked: Some(why),
                ..
            } => Some(format!("{label} needs a desktop project: {why}")),
            _ => None,
        }
    }

    #[note("palette-row-text")]
    pub fn text(&self) -> &'static str {
        match self {
            Row::Section(title) => title,
            Row::Item { label, .. } => label,
            Row::Empty => NO_MATCH,
        }
    }
}

#[note("palette-rows")]
pub fn rows(registry: &Registry, target: &str, filter: &str, web: bool) -> Vec<Row> {
    let query = filter.trim().to_lowercase();
    let mut out = Vec::new();
    for (group, items) in registry.palette(target) {
        let items: Vec<&ComponentDesc> = items
            .into_iter()
            .filter(|d| matches_filter(d, &query))
            .collect();
        if items.is_empty() {
            continue;
        }
        out.push(Row::Section(group));
        for d in items {
            out.push(Row::Item {
                class: d.class,
                label: d.display,
                blocked: web.then(|| web_block(d.class)).flatten(),
            });
        }
    }
    if out.is_empty() {
        out.push(Row::Empty);
    }
    out
}

/// Why a component cannot work in a web page (WebAssembly in the browser), if it cannot.
#[note("palette-web-block")]
pub fn web_block(class: &str) -> Option<&'static str> {
    match class {
        "SqliteProvider" | "SqliteSqlxProvider" => {
            Some("a web page cannot open a database file on the user's disk")
        }
        "PostgresProvider" | "RedisProvider" => {
            Some("a web page cannot open a network socket to a database server")
        }
        "OpenDialog" | "SaveDialog" => Some("a web page has no native file dialog"),
        _ => None,
    }
}

#[note("palette-matches-filter")]
fn matches_filter(d: &ComponentDesc, query: &str) -> bool {
    query.is_empty()
        || d.display.to_lowercase().contains(query)
        || d.class.to_lowercase().contains(query)
}

/// The palette as the MCP `view.get_toolbox` reads it: the filter and every row.
#[note("palette-to-json")]
pub fn to_json(rows: &[Row], filter: &str) -> serde_json::Value {
    let rows: Vec<serde_json::Value> = rows
        .iter()
        .map(|r| match r {
            Row::Section(title) => serde_json::json!({ "section": title }),
            Row::Item {
                class,
                label,
                blocked,
            } => serde_json::json!({ "class": class, "label": label, "disabled": blocked }),
            Row::Empty => serde_json::json!({ "empty": NO_MATCH }),
        })
        .collect();
    serde_json::json!({ "empty_state": null, "filter": filter, "rows": rows })
}
