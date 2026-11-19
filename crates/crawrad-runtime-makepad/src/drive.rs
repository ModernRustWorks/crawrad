//! **Driving a widget as the user would**: an MCP tool presses a
//! button, flips a check box or types into a field by emitting the very action
//! the widget sends on a real click or keystroke. The shell's handlers find it
//! in `actions` by the widget's uid, so they cannot tell the two apart.
//! Beside them, the reads and fills the shell's dialogs repeat by name.

use crate::list::CrawradListBoxWidgetRefExt;
use crawrad_note::note;
use makepad_widgets::{
    Actions, ButtonAction, CheckBoxAction, CheckBoxWidgetRefExt, Cx, DropDownAction,
    DropDownWidgetRefExt, HeapLiveIdPath, KeyModifiers, LiveId, LiveValue, Rect, TextInputAction,
    WidgetActionCxExt, WidgetRef, WidgetUid,
};
use std::cell::RefCell;
use std::collections::HashMap;

/// Why a drive did not happen: the code of the refusal the MCP answers.
pub type Refused = &'static str;

thread_local! {
    /// The items each drop-down was last filled with, by widget uid. Makepad
    /// 1.0 keeps `labels` private, and its `selected_label` indexes them with
    /// no check — an empty drop-down panics there — so the reads and the pick
    /// below go by this copy, written by [`fill_drop`].
    static DROP_ITEMS: RefCell<HashMap<WidgetUid, Vec<String>>> = RefCell::default();
}

#[note("drive-shown")]
pub fn shown(cx: &Cx, widget: &WidgetRef) -> Result<(), Refused> {
    let rect = widget.area().rect(cx);
    let drawn = widget.visible() && rect.size.x > 0.0 && rect.size.y > 0.0;
    drawn.then_some(()).ok_or("hidden-control")
}

#[note("drive-press")]
pub fn press(cx: &mut Cx, root: &WidgetRef, path: &[LiveId]) -> Result<(), Refused> {
    let button = root.widget(path);
    shown(cx, &button)?;
    let click = ButtonAction::Clicked(KeyModifiers::default());
    cx.widget_action(button.widget_uid(), &HeapLiveIdPath::default(), click);
    Ok(())
}

#[note("drive-set-checked")]
pub fn set_checked(
    cx: &mut Cx,
    root: &WidgetRef,
    path: &[LiveId],
    value: bool,
) -> Result<(), Refused> {
    shown(cx, &root.widget(path))?;
    let check = root.check_box(path);
    check.set_active(cx, value);
    let change = CheckBoxAction::Change(value);
    cx.widget_action(check.widget_uid(), &HeapLiveIdPath::default(), change);
    Ok(())
}

#[note("drive-toggle")]
pub fn toggle(cx: &mut Cx, root: &WidgetRef, path: &[LiveId]) -> Result<(), Refused> {
    let next = !root.check_box(path).active(cx);
    set_checked(cx, root, path, next)
}

#[note("drive-type-text")]
pub fn type_text(
    cx: &mut Cx,
    root: &WidgetRef,
    path: &[LiveId],
    value: &str,
) -> Result<(), Refused> {
    let field = root.widget(path);
    shown(cx, &field)?;
    field.set_text(cx, value);
    let changed = TextInputAction::Changed(value.to_string());
    cx.widget_action(field.widget_uid(), &HeapLiveIdPath::default(), changed);
    Ok(())
}

#[note("drive-click-card")]
pub fn click_card(cx: &mut Cx, root: &WidgetRef, path: &[LiveId]) -> Result<(), Refused> {
    let card = root.widget(path);
    shown(cx, &card)?;
    let click = crate::card::CrawradFormCardAction::Clicked;
    cx.widget_action(card.widget_uid(), &HeapLiveIdPath::default(), click);
    Ok(())
}

#[note("drive-pick")]
pub fn pick(cx: &mut Cx, root: &WidgetRef, path: &[LiveId], label: &str) -> Result<(), Refused> {
    let widget = root.widget(path);
    shown(cx, &widget)?;
    let items = drop_items(&widget);
    let index = items
        .iter()
        .position(|l| l == label)
        .ok_or("unknown-item")?;
    let drop = root.drop_down(path);
    drop.set_selected_item(cx, index);
    let select = DropDownAction::Select(index, LiveValue::None);
    cx.widget_action(drop.widget_uid(), &HeapLiveIdPath::default(), select);
    Ok(())
}

#[note("drive-drop-items")]
pub fn drop_items(widget: &WidgetRef) -> Vec<String> {
    let uid = widget.widget_uid();
    DROP_ITEMS.with(|items| items.borrow().get(&uid).cloned().unwrap_or_default())
}

#[note("drive-drop-text")]
pub fn drop_text(widget: &WidgetRef) -> Option<String> {
    let at = widget.as_drop_down().selected_item();
    drop_items(widget).get(at).cloned()
}

#[note("drive-list-rows")]
pub fn list_rows(cx: &Cx, root: &WidgetRef, name: &str) -> Vec<(String, Rect)> {
    let list = root.widget(&[LiveId::from_str(name)]);
    list.as_crawrad_list_box().drawn_rows(cx)
}

#[note("drive-checked-in")]
pub fn checked_in(dlg: &WidgetRef, actions: &Actions, names: &[&str]) -> Option<bool> {
    names
        .iter()
        .find_map(|n| dlg.check_box(&[LiveId::from_str(n)]).changed(actions))
}

#[note("drive-picked-in")]
pub fn picked_in(dlg: &WidgetRef, actions: &Actions, names: &[&str]) -> Option<usize> {
    names
        .iter()
        .find_map(|n| dlg.drop_down(&[LiveId::from_str(n)]).selected(actions))
}

#[note("drive-fill-drop")]
pub fn fill_drop(
    cx: &mut Cx,
    dlg: &WidgetRef,
    drop: &[LiveId],
    labels: Vec<String>,
    at: Option<usize>,
) {
    let drop = dlg.drop_down(drop);
    let uid = drop.widget_uid();
    DROP_ITEMS.with(|items| items.borrow_mut().insert(uid, labels.clone()));
    drop.set_labels(cx, labels);
    if let Some(at) = at {
        drop.set_selected_item(cx, at);
    }
}
