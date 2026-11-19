//! **A control linked to an `Action`** — generated from the link, never
//! written per class.
//!
//! Any control whose `action` property points to an `Action` gets, as in a
//! classic RAD form:
//!
//! - its own click (the `on_click` row of the `Bind` table) runs the action's
//!   `OnExecute`, while the action is enabled;
//! - before each sync, the action's `OnUpdate` runs and its `Enabled`,
//!   `Visible` and caption pass to every control linked to it — the action
//!   owns that state.
//!
//! A new class that has an `action` property and a click in the table gets
//! all of this without a line here.

use super::binding;
use super::components;
use super::layout::handler;
use crate::{Erro, identificador};
use crawrad_ir::{Document, Node, Value};
use crawrad_note::note;

#[note("targets-makepad-action-link-actionlinks")]
#[derive(Default)]
pub struct ActionLinks {
    pub dispatch: String,
    pub pre_sync: String,
}

#[note("targets-makepad-action-link-linked-action")]
fn linked_action<'a>(doc: &'a Document, n: &Node) -> Option<&'a Node> {
    match n.props.get("action") {
        Some(Value::Ref(id)) if !id.trim().is_empty() => {
            doc.node(id).filter(|a| a.class == "Action")
        }
        _ => None,
    }
}

#[note("targets-makepad-action-link-has-field")]
fn has_field(class: &str, field: &str) -> bool {
    components::for_class(class)
        .is_some_and(|c| c.struct_definition().contains(&format!("pub {field}:")))
}

pub fn action_links(doc: &Document, emitted: &[&Node]) -> Result<ActionLinks, Erro> {
    let mut out = ActionLinks::default();
    // `OnUpdate` first: the state it sets is what the controls then take
    for n in emitted.iter().filter(|n| n.class == "Action") {
        if let Some(h) = handler(n, "on_update") {
            out.pre_sync.push_str(&format!(
                "        // the action decides its own state\n\
                 \x20       self.form.{}();\n",
                identificador(&h)?
            ));
        }
    }
    for n in emitted {
        let Some(act) = linked_action(doc, n) else {
            continue;
        };
        let (ctl, a) = (identificador(n.name())?, identificador(act.name())?);
        for prop in ["enabled", "visible"] {
            if has_field(&n.class, prop) {
                out.pre_sync.push_str(&format!(
                    "        self.form.{ctl}.{prop} = self.form.{a}.{prop};\n"
                ));
            }
        }
        // an action with a caption gives it to its controls
        let caption = matches!(act.props.get("text"), Some(Value::Str(s)) if !s.is_empty());
        if caption && has_field(&n.class, "text") {
            out.pre_sync.push_str(&format!(
                "        if self.form.{ctl}.text != self.form.{a}.text {{\n\
                 \x20           self.form.{ctl}.text = self.form.{a}.text.clone();\n\
                 \x20       }}\n"
            ));
        }
        let Some(b) = binding::binding_of(&n.class, "on_click") else {
            continue;
        };
        let exec = handler(act, "on_execute");
        let call = if let Some(exec) = exec {
            // An assigned OnExecute REPLACES the default behavior
            binding::emitir(&b, &ctl, &identificador(&exec)?)
        } else if let Some(da) = crate::targets::data_binding::dataset_action_of(doc, act)? {
            let op_call = crate::targets::data_binding::dataset_operation_call(
                &da.operation,
                &format!("self.{}_adapter", da.dataset_name),
                "            ",
            );
            if op_call.is_empty() {
                continue;
            }
            binding::emitir_com_corpo(&b, &ctl, &op_call)
        } else {
            continue;
        };
        out.dispatch
            .push_str(&format!("        // `{ctl}` runs `{a}`\n"));
        out.dispatch.push_str(&super::actions::guarded(
            &format!("self.form.{a}.enabled"),
            &call,
        ));
    }
    Ok(out)
}

#[cfg(test)]
#[path = "../../test/action_link/test.rs"]
mod tests;
