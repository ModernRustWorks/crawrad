//! The rows a list keeps selected, one rule for both targets.
//!
//! A `ListBox` holds `selected` — the row in focus, the one clicked last — and
//! `selection`, every row marked. With `MultiSelect` off they say the same
//! thing: `selection` is `selected`, or nothing. With it on, a Ctrl+click
//! toggles a row and a Shift+click marks the run from the row in focus to the
//! clicked one, as in the classic RAD IDE.
//!
//! The generated code of both targets calls these functions, so the gesture
//! cannot mean one thing on Makepad and another on egui.
//!
//! `Sorted` lives here too: sorting moves rows, and the rows marked have to
//! move with them.

use crawrad_note::note;
use std::cmp::Ordering;

#[note("list-selection-click")]
pub fn click(
    selection: &mut Vec<usize>,
    focus: Option<usize>,
    row: usize,
    multi_select: bool,
    ctrl: bool,
    shift: bool,
) {
    if multi_select && shift {
        let anchor = focus.unwrap_or(row);
        *selection = (anchor.min(row)..=anchor.max(row)).collect();
    } else if multi_select && ctrl {
        match selection.binary_search(&row) {
            Ok(at) => {
                selection.remove(at);
            }
            Err(at) => selection.insert(at, row),
        }
    } else {
        *selection = vec![row];
    }
}

#[note("list-selection-settle")]
pub fn settle(selection: &mut Vec<usize>, selected: Option<usize>, len: usize, multi_select: bool) {
    if !multi_select {
        selection.clear();
        selection.extend(selected);
    }
    selection.retain(|&i| i < len);
}

#[note("list-selection-compare")]
pub fn compare(a: &str, b: &str) -> Ordering {
    a.chars()
        .flat_map(char::to_lowercase)
        .cmp(b.chars().flat_map(char::to_lowercase))
}

#[note("list-selection-sorted-order")]
pub fn sorted_order<S: AsRef<str>>(items: &[S]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..items.len()).collect();
    order.sort_by(|&a, &b| compare(items[a].as_ref(), items[b].as_ref()));
    order
}

#[note("list-selection-sort")]
pub fn sort(items: &mut [String], selected: &mut Option<usize>, selection: &mut [usize]) {
    if items
        .windows(2)
        .all(|w| compare(&w[0], &w[1]) != Ordering::Greater)
    {
        return;
    }
    let order = sorted_order(items);
    let mut moved_to = vec![0; order.len()];
    for (new, &old) in order.iter().enumerate() {
        moved_to[old] = new;
    }
    let sorted: Vec<String> = order
        .iter()
        .map(|&i| std::mem::take(&mut items[i]))
        .collect();
    for (slot, text) in items.iter_mut().zip(sorted) {
        *slot = text;
    }
    if let Some(row) = selected
        && let Some(&new) = moved_to.get(*row)
    {
        *row = new;
    }
    for row in selection.iter_mut() {
        if let Some(&new) = moved_to.get(*row) {
            *row = new;
        }
    }
    selection.sort_unstable();
}

#[cfg(test)]
#[path = "test/list_selection/test.rs"]
mod test;

/// The items whose text starts with `typed`, ignoring case; empty = all.
///
/// The editable `ComboBox` filters its list with it on both targets.
///
/// # Examples
///
/// ```
/// use crawrad_runtime::list_selection::matching;
///
/// let items = vec!["Apple".to_string(), "Banana".to_string(), "Apricot".to_string()];
/// assert_eq!(matching(&items, "ap"), vec![0, 2]);
/// assert_eq!(matching(&items, ""), vec![0, 1, 2]);
/// ```
pub fn matching(items: &[String], typed: &str) -> Vec<usize> {
    let typed = typed.to_lowercase();
    items
        .iter()
        .enumerate()
        .filter(|(_, item)| item.to_lowercase().starts_with(&typed))
        .map(|(index, _)| index)
        .collect()
}
