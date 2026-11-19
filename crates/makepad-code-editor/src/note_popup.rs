//! The popup "Add documentation / Add note / Add comment" under the caret line.
//! Pure state and geometry — CodeEditor draws it above everything else.

use crate::note_anchor::AnchorKind;
use makepad_widgets::*;

/// A row as the IDE hands it over: kind (0 doc, 1 note, 2 comment), title,
/// description, shortcut and whether it can be chosen.
pub type PopupRowParts = (u8, String, String, &'static str, bool);

/// An item in the note popup.
#[derive(Clone, Debug, PartialEq)]
pub struct PopupItem {
    pub kind: AnchorKind,
    pub title: String,
    pub description: String,
    pub shortcut: &'static str,
    pub enabled: bool,
}

/// Result of handling a mouse down event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PopupMouseResult {
    Ignored,
    Consumed,
    Chosen(usize),
    Closed,
}

/// Pure state and geometry of the note popup.
#[derive(Clone, Debug, Default)]
pub struct NotePopupState {
    pub open: bool,
    pub header: String,
    pub items: Vec<PopupItem>,
    pub selected: Option<usize>,
    pub chosen: Option<usize>,
    pub panel_rect: Rect,
    pub row_rects: Vec<Rect>,
}

impl NotePopupState {
    /// Opens the popup with `header` and `items`, selecting the first enabled row.
    pub fn open(&mut self, header: String, items: Vec<PopupItem>) {
        self.open = true;
        self.header = header;
        self.items = items;
        self.chosen = None;
        self.selected = self.first_enabled();
        self.row_rects.clear();
    }

    /// Closes the popup.
    pub fn close(&mut self) {
        self.open = false;
        self.selected = None;
    }

    /// First enabled row index, if any.
    pub fn first_enabled(&self) -> Option<usize> {
        self.items.iter().position(|it| it.enabled)
    }

    /// Moves selection up, skipping disabled rows.
    pub fn move_up(&mut self) {
        if !self.open || self.items.is_empty() {
            return;
        }
        let current = self.selected.unwrap_or(0);
        for i in (0..current).rev() {
            if self.items[i].enabled {
                self.selected = Some(i);
                return;
            }
        }
    }

    /// Moves selection down, skipping disabled rows.
    pub fn move_down(&mut self) {
        if !self.open || self.items.is_empty() {
            return;
        }
        let current = self.selected.unwrap_or(0);
        for i in (current + 1)..self.items.len() {
            if self.items[i].enabled {
                self.selected = Some(i);
                return;
            }
        }
    }

    /// Handles single-letter shortcut: D / N / M choose directly if enabled.
    pub fn handle_letter(&mut self, ch: char) -> Option<usize> {
        if !self.open {
            return None;
        }
        let target = match ch {
            'd' | 'D' => 0,
            'n' | 'N' => 1,
            'm' | 'M' => 2,
            _ => return None,
        };
        if target < self.items.len() && self.items[target].enabled {
            self.chosen = Some(target);
            self.close();
            Some(target)
        } else {
            None
        }
    }

    /// Confirms current selection (Enter).
    pub fn choose_selected(&mut self) -> Option<usize> {
        if !self.open {
            return None;
        }
        if let Some(idx) = self.selected
            && let Some(item) = self.items.get(idx)
            && item.enabled
        {
            self.chosen = Some(idx);
            self.close();
            return Some(idx);
        }
        None
    }

    /// Hit tests a screen position against row rectangles.
    pub fn hit_test_row(&self, pos: DVec2) -> Option<usize> {
        self.row_rects.iter().position(|r| r.contains(pos))
    }

    /// Handles mouse movement for hover selection.
    pub fn handle_mouse_move(&mut self, pos: DVec2) {
        if !self.open {
            return;
        }
        if let Some(idx) = self.hit_test_row(pos)
            && self.items.get(idx).is_some_and(|it| it.enabled)
        {
            self.selected = Some(idx);
        }
    }

    /// Handles mouse down event.
    pub fn handle_mouse_down(&mut self, pos: DVec2) -> PopupMouseResult {
        if !self.open {
            return PopupMouseResult::Ignored;
        }
        if self.panel_rect.contains(pos) {
            if let Some(idx) = self.hit_test_row(pos)
                && self.items.get(idx).is_some_and(|it| it.enabled)
            {
                self.chosen = Some(idx);
                self.close();
                return PopupMouseResult::Chosen(idx);
            }
            return PopupMouseResult::Consumed;
        }
        self.close();
        PopupMouseResult::Closed
    }

    /// Takes the chosen row index once.
    pub fn take_choice(&mut self) -> Option<usize> {
        self.chosen.take()
    }

    /// Computes panel and row rectangles, anchoring under caret line and keeping inside viewport.
    pub fn compute_layout(
        &mut self,
        caret_pos: DVec2,
        line_height: f64,
        viewport: Rect,
        scale: f64,
        content_w: f64,
    ) {
        let padding = 6.0 * scale;
        // Row padding, icon and its gap around the widest text; never under 380.
        let panel_w = (content_w + padding * 2.0 + 62.0 * scale).max(380.0 * scale);
        let header_h = 22.0 * scale;
        let row_h = 48.0 * scale;
        let total_h = padding * 2.0 + header_h + row_h * self.items.len() as f64;

        let min_x = viewport.pos.x + 8.0;
        let max_x = (viewport.pos.x + viewport.size.x - panel_w - 8.0).max(min_x);
        let x = caret_pos.x.clamp(min_x, max_x);

        let space_below = (viewport.pos.y + viewport.size.y) - (caret_pos.y + line_height);
        let y = if space_below < total_h && (caret_pos.y - viewport.pos.y) > total_h {
            caret_pos.y - total_h
        } else {
            caret_pos.y + line_height + 2.0
        };

        self.panel_rect = Rect {
            pos: dvec2(x, y),
            size: dvec2(panel_w, total_h),
        };

        self.row_rects.clear();
        let row_w = panel_w - padding * 2.0;
        let mut row_y = y + padding + header_h;
        for _ in &self.items {
            self.row_rects.push(Rect {
                pos: dvec2(x + padding, row_y),
                size: dvec2(row_w, row_h),
            });
            row_y += row_h;
        }
    }
}
