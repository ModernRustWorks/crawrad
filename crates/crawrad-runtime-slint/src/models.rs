//! Data model adapters for lists, combo boxes, and table views in Slint.

/// A simple list model holding text items and a selected index.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SimpleListModel {
    pub items: Vec<String>,
    pub selected_index: Option<usize>,
}

impl SimpleListModel {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            items,
            selected_index: None,
        }
    }

    pub fn selected_text(&self) -> Option<&str> {
        self.selected_index
            .and_then(|idx| self.items.get(idx))
            .map(|s| s.as_str())
    }

    pub fn select(&mut self, index: usize) {
        if index < self.items.len() {
            self.selected_index = Some(index);
        } else {
            self.selected_index = None;
        }
    }
}
