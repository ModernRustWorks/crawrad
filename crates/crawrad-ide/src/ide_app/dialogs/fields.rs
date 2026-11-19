//! The one binder for [`DialogFields`]: reads what the user did to a dialog's
//! controls and paints what the state says, by the names the state lists.

use super::super::*;
use crawrad_actions::dialog_fields::{DialogFields, FieldKind, FieldValue};

/// A field's widget path: `row.label` is `label` inside `row`.
fn path(name: &str) -> Vec<LiveId> {
    name.split('.').map(LiveId::from_str).collect()
}

/// Hands the state what the user did to its controls this frame; `true` when
/// anything arrived, so the dialog repaints.
pub(in super::super) fn take_input(
    cx: &mut Cx,
    dlg: &WidgetRef,
    actions: &Actions,
    st: &mut impl DialogFields,
) -> bool {
    let mut any = false;
    for (name, kind) in st.fields() {
        let id = path(name);
        let value = match kind {
            FieldKind::Text => dlg.text_input(&id).changed(actions).map(FieldValue::Text),
            FieldKind::Check => dlg.check_box(&id).changed(actions).map(FieldValue::On),
            FieldKind::Drop(_) => dlg.drop_down(&id).selected(actions).map(FieldValue::Index),
            FieldKind::Button => dlg
                .button(&id)
                .clicked(actions)
                .then_some(FieldValue::On(true)),
            FieldKind::Radio(radios) => {
                let mut set = WidgetSet::empty();
                for r in *radios {
                    set.push(dlg.widget(&path(r)));
                }
                set.as_radio_button_set()
                    .selected(cx, actions)
                    .map(FieldValue::Index)
            }
            FieldKind::Label => None,
        };
        if let Some(v) = value {
            st.input(name, v);
            any = true;
        }
    }
    any
}

/// Paints every field as the state has it. A text box is written only when it
/// differs, so the caret of the one being typed in stays where it is.
pub(in super::super) fn paint(cx: &mut Cx, dlg: &WidgetRef, st: &impl DialogFields) {
    for (name, kind) in st.fields() {
        let id = path(name);
        match (kind, st.value(name)) {
            (FieldKind::Text, Some(FieldValue::Text(t))) => {
                let input = dlg.text_input(&id);
                if input.text() != t {
                    input.set_text(cx, &t);
                }
            }
            (FieldKind::Label, Some(FieldValue::Text(t))) => dlg.label(&id).set_text(cx, &t),
            (FieldKind::Button, Some(FieldValue::Text(t))) => dlg.button(&id).set_text(cx, &t),
            (FieldKind::Check, Some(FieldValue::On(on))) => dlg.check_box(&id).set_active(cx, on),
            (FieldKind::Drop(items), Some(FieldValue::Index(i))) => {
                let items = items.iter().map(|s| s.to_string()).collect();
                crawrad_runtime_makepad::drive::fill_drop(cx, dlg, &id, items, Some(i));
            }
            (FieldKind::Radio(radios), Some(FieldValue::Index(i))) => {
                if let Some(r) = radios.get(i) {
                    dlg.radio_button(&path(r)).select(cx, &mut Scope::empty());
                }
            }
            _ => {}
        }
    }
}

/// A list row clicked: by the mouse on its `alvo`, or by a script on the row.
pub(in super::super) fn row_clicked(dlg: &WidgetRef, row: &[LiveId], actions: &Actions) -> bool {
    let row = dlg.widget(row);
    row.button(id!(alvo)).clicked(actions) || actions.find_widget_action(row.widget_uid()).is_some()
}
