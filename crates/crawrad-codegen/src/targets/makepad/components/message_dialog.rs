//! Makepad component implementation for `MessageDialog`.

use super::codegen::MakepadComponentCodegen;
use crate::identificador;
use crate::targets::makepad::state;
use crawrad_ir::{Document, Node};

pub struct MessageDialogCodegen;

impl MakepadComponentCodegen for MessageDialogCodegen {
    fn class(&self) -> &'static str {
        "MessageDialog"
    }

    fn struct_definition(&self) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("MessageDialog").expect("MessageDialog desc");
        let fields = state::fields_of(desc);
        let body = fields
            .iter()
            .map(|(n, t)| format!("    pub {n}: {t},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "#[derive(Debug, Clone, Default)]\n\
             pub struct MessageDialog {{\n\
             {body}\n\
             }}\n\n\
             impl MessageDialog {{\n\
                 /// Shows the message box; the answer arrives in `modal_result` and `OnClose`.\n\
                 pub fn execute(&mut self) {{\n\
                     self.answer.open(&self.title, &self.text, DialogKind::from_name(&self.kind), DialogButtons::from_name(&self.buttons));\n\
                 }}\n\
             }}\n"
        )
    }

    fn emit_default(&self, node: &Node) -> String {
        let r = crawrad_registry::Registry::builtin();
        let desc = r.get("MessageDialog").expect("MessageDialog desc");
        let mut inits: Vec<String> = state::inits_of(node, desc)
            .iter()
            .map(|(n, v)| format!("{n}: {v}"))
            .collect();
        inits.push("modal_result: ModalResult::None".to_string());
        inits.push("answer: PendingAnswer::default()".to_string());
        format!("MessageDialog {{ {} }}", inits.join(", "))
    }

    fn emit_sync(&self, _doc: &Document, _node: &Node, _field: &str) -> String {
        String::new()
    }

    fn emit_actions(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let on_close_stmt = match node.events.get("on_close").filter(|h| !h.trim().is_empty()) {
            Some(h) => format!("\n            self.form.{}();", identificador(h)?),
            None => String::new(),
        };
        Ok(format!(
            "        if let Some(r) = self.form.{field}.answer.take() {{\n\
             \x20           self.form.{field}.modal_result = ModalResult::from_i32(r.as_i32()).unwrap_or_default();{on_close_stmt}\n\
             \x20       }}\n"
        ))
    }

    fn emit_dsl(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        _node: &Node,
        _x: f32,
        _y: f32,
        _w: f32,
        _h: f32,
    ) -> Result<String, crate::Erro> {
        Ok(String::new())
    }
}
