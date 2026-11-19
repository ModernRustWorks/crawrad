//! The trees the extensions fill for their views, and the Extensions
//! panel they become: a header per view, the items as rows, a filter that
//! keeps a branch while anything under it matches.

use std::collections::{BTreeMap, BTreeSet};

use crawrad_extension_api::views::{ItemLocation, SetTreeParams, ViewContribution, ViewItem};
use crawrad_note::note;

type Waker = Box<dyn Fn() + Send>;

#[note("views-extensionview")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionView {
    pub extension: String,
    pub id: String,
    pub title: String,
    pub items: Vec<ViewItem>,
}

#[note("views-viewstore")]
#[derive(Default)]
pub struct ViewStore {
    /// In the order the extensions registered.
    declared: Vec<(String, ViewContribution)>,
    /// (extension id, view id) → its tree.
    trees: BTreeMap<(String, String), Vec<ViewItem>>,
    revision: u64,
    /// Wakes the window: a tree arrives on an extension's reader thread.
    waker: Option<Waker>,
}

impl ViewStore {
    pub fn set_waker(&mut self, wake: impl Fn() + Send + 'static) {
        self.waker = Some(Box::new(wake));
    }

    fn changed(&mut self) {
        self.revision += 1;
        if let Some(wake) = &self.waker {
            wake();
        }
    }

    #[note("views-viewstore-declare")]
    pub fn declare(&mut self, extension: &str, views: &[ViewContribution]) {
        self.declared.retain(|(ext, _)| ext != extension);
        self.declared
            .extend(views.iter().map(|v| (extension.to_string(), v.clone())));
        self.changed();
    }

    #[note("views-viewstore-set")]
    pub fn set(&mut self, extension: &str, params: SetTreeParams) -> bool {
        let known = self
            .declared
            .iter()
            .any(|(ext, v)| ext == extension && v.id == params.view_id);
        if !known {
            return false;
        }
        self.trees
            .insert((extension.to_string(), params.view_id), params.items);
        self.changed();
        true
    }

    #[note("views-viewstore-forget")]
    pub fn forget(&mut self, extension: &str) {
        let before = (self.declared.len(), self.trees.len());
        self.declared.retain(|(ext, _)| ext != extension);
        self.trees.retain(|(ext, _), _| ext != extension);
        if (self.declared.len(), self.trees.len()) != before {
            self.changed();
        }
    }

    #[note("views-viewstore-revision")]
    pub fn revision(&self) -> u64 {
        self.revision
    }

    #[note("views-viewstore-views")]
    pub fn views(&self) -> Vec<ExtensionView> {
        self.declared
            .iter()
            .map(|(ext, v)| ExtensionView {
                extension: ext.clone(),
                id: v.id.clone(),
                title: v.title.clone(),
                items: self
                    .trees
                    .get(&(ext.clone(), v.id.clone()))
                    .cloned()
                    .unwrap_or_default(),
            })
            .collect()
    }
}

#[note("views-installed")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Installed {
    pub id: String,
    pub name: String,
    pub version: String,
}

#[note("views-viewrow")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewRow {
    /// A view's header: its title, how many leaves show, open or closed.
    Header {
        view: usize,
        title: String,
        count: usize,
        open: bool,
    },
    /// An item, indented by its depth.
    Item {
        view: usize,
        id: String,
        depth: usize,
        label: String,
        detail: String,
        kind: String,
        /// `Some(open)` for an item with children.
        branch: Option<bool>,
        location: Option<ItemLocation>,
    },
    /// The title of the installed extensions' section, with how many
    /// (none for the hint shown while nothing is installed).
    Section { title: String, count: Option<usize> },
    /// An installed extension, with its uninstall button.
    Installed {
        id: String,
        name: String,
        version: String,
    },
}

impl ViewRow {
    #[note("views-viewrow-template")]
    pub fn template(&self) -> &'static str {
        match self {
            Self::Header { .. } | Self::Section { .. } => "ViewHeader",
            Self::Item { .. } => "ViewItem",
            Self::Installed { .. } => "ViewInstalled",
        }
    }
}

#[note("views-itempick")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemPick {
    pub extension: String,
    pub view: String,
    pub item: String,
    pub location: Option<ItemLocation>,
}

#[note("views-viewpanel")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ViewPanel {
    pub views: Vec<ExtensionView>,
    /// The installed extensions, the last section.
    pub installed: Vec<Installed>,
    /// Where each installed row's uninstall button is (row, x, y, w, h).
    pub buttons: Vec<(usize, [f64; 4])>,
    /// The store's revision `views` was read at.
    seen: Option<u64>,
    pub filter: String,
    /// Closed views (extension, view) and items (extension, view, item).
    closed_views: BTreeSet<(String, String)>,
    closed_items: BTreeSet<(String, String, String)>,
    pub rects: Vec<(usize, [f64; 4])>,
}

#[note("views-says")]
fn says(item: &ViewItem, needle: &str) -> bool {
    [
        Some(item.label.as_str()),
        item.detail.as_deref(),
        item.kind.as_deref(),
    ]
    .into_iter()
    .flatten()
    .any(|t| t.to_lowercase().contains(needle))
}

#[note("views-shows")]
fn shows(item: &ViewItem, needle: &str) -> bool {
    says(item, needle) || item.children.iter().any(|c| shows(c, needle))
}

impl ViewPanel {
    #[note("views-viewpanel-follow")]
    pub fn follow(&mut self, store: &ViewStore) -> bool {
        if self.seen == Some(store.revision()) {
            return false;
        }
        self.seen = Some(store.revision());
        self.views = store.views();
        true
    }

    #[note("views-viewpanel-set-installed")]
    pub fn set_installed(&mut self, now: Vec<Installed>) -> bool {
        let changed = now != self.installed;
        self.installed = now;
        changed
    }

    #[note("views-viewpanel-uninstall-at")]
    pub fn uninstall_at(&self, row: usize) -> Option<String> {
        match self.shown().get(row)? {
            ViewRow::Installed { id, .. } => Some(id.clone()),
            _ => None,
        }
    }

    #[note("views-viewpanel-shown")]
    pub fn shown(&self) -> Vec<ViewRow> {
        self.rows(&self.views)
    }

    #[note("views-viewpanel-click")]
    pub fn click(&mut self, row: usize) -> Option<ItemPick> {
        match self.shown().get(row)? {
            ViewRow::Header { view, .. } => {
                let view = self.views.get(*view)?.clone();
                self.toggle_view(&view);
                None
            }
            ViewRow::Section { .. } | ViewRow::Installed { .. } => None,
            ViewRow::Item {
                view,
                id,
                branch,
                location,
                ..
            } => {
                let view = self.views.get(*view)?.clone();
                if branch.is_some() {
                    self.toggle_item(&view, id);
                }
                Some(ItemPick {
                    extension: view.extension,
                    view: view.id,
                    item: id.clone(),
                    location: location.clone(),
                })
            }
        }
    }

    pub fn toggle_view(&mut self, view: &ExtensionView) {
        let key = (view.extension.clone(), view.id.clone());
        if !self.closed_views.remove(&key) {
            self.closed_views.insert(key);
        }
    }

    pub fn toggle_item(&mut self, view: &ExtensionView, item: &str) {
        let key = (view.extension.clone(), view.id.clone(), item.to_string());
        if !self.closed_items.remove(&key) {
            self.closed_items.insert(key);
        }
    }

    #[note("views-viewpanel-rows")]
    pub fn rows(&self, views: &[ExtensionView]) -> Vec<ViewRow> {
        let needle = self.filter.trim().to_lowercase();
        let mut rows = Vec::new();
        for (vi, view) in views.iter().enumerate() {
            let open = !needle.is_empty()
                || !self
                    .closed_views
                    .contains(&(view.extension.clone(), view.id.clone()));
            let header = rows.len();
            rows.push(ViewRow::Header {
                view: vi,
                title: view.title.clone(),
                count: 0,
                open,
            });
            let mut items = Vec::new();
            let count = self.walk(view, vi, &view.items, 0, &needle, &mut items);
            if open {
                rows.extend(items);
            }
            if let ViewRow::Header { count: c, .. } = &mut rows[header] {
                *c = count;
            }
        }
        if !self.installed.is_empty() {
            let count = Some(self.installed.len());
            rows.push(ViewRow::Section {
                title: "Installed".into(),
                count,
            });
            rows.extend(self.installed.iter().map(|e| ViewRow::Installed {
                id: e.id.clone(),
                name: e.name.clone(),
                version: e.version.clone(),
            }));
        } else if rows.is_empty() && needle.is_empty() {
            let title = "No extensions yet. Click + to add one".into();
            rows.push(ViewRow::Section { title, count: None });
        }
        rows
    }

    #[note("views-viewpanel-walk")]
    fn walk(
        &self,
        view: &ExtensionView,
        vi: usize,
        items: &[ViewItem],
        depth: usize,
        needle: &str,
        rows: &mut Vec<ViewRow>,
    ) -> usize {
        let mut leaves = 0;
        for item in items {
            if !needle.is_empty() && !shows(item, needle) {
                continue;
            }
            let open = !needle.is_empty()
                || !self.closed_items.contains(&(
                    view.extension.clone(),
                    view.id.clone(),
                    item.id.clone(),
                ));
            rows.push(ViewRow::Item {
                view: vi,
                id: item.id.clone(),
                depth,
                label: item.label.clone(),
                detail: item.detail.clone().unwrap_or_default(),
                kind: item.kind.clone().unwrap_or_default(),
                branch: (!item.children.is_empty()).then_some(open),
                location: item.location.clone(),
            });
            if item.children.is_empty() {
                leaves += 1;
                continue;
            }
            // a matching branch brings everything under it
            let inner = if says(item, needle) { "" } else { needle };
            let mut under = Vec::new();
            leaves += self.walk(view, vi, &item.children, depth + 1, inner, &mut under);
            if open {
                rows.extend(under);
            }
        }
        leaves
    }
}

#[cfg(test)]
#[path = "test/views/test.rs"]
mod tests;
