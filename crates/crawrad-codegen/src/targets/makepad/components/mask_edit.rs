//! Makepad component implementation for `MaskEdit`: an `Edit` whose
//! every change is shaped by its `EditMask` again
//! (`crawrad_runtime_makepad::edit::hold_mask`, over `crawrad_runtime::mask`).

use super::codegen::{EnterKey, MakepadComponentCodegen};
use super::edit;
use crawrad_ir::{Document, Node};

pub struct MaskEditCodegen;

impl MakepadComponentCodegen for MaskEditCodegen {
    fn class(&self) -> &'static str {
        "MaskEdit"
    }

    fn default_cursor(&self) -> &'static str {
        "text"
    }

    fn enter_key(&self) -> EnterKey {
        EnterKey::Swallowed
    }

    fn struct_definition(&self) -> String {
        edit::struct_of(&crawrad_registry::components::mask_edit())
    }

    fn emit_default(&self, node: &Node) -> String {
        let text = format!(
            "crawrad_runtime::mask::shown_text({}, {})",
            crate::literal(edit_mask(node)),
            edit::cased_text_literal(node)
        );
        edit::default_of(node, &crawrad_registry::components::mask_edit(), &text)
    }

    fn emit_sync(&self, doc: &Document, node: &Node, field: &str) -> String {
        edit::emit_sync(doc, node, field)
    }

    fn emit_actions(
        &self,
        document: &Document,
        node: &Node,
        field: &str,
        _emitted: &[&Node],
    ) -> Result<String, crate::Erro> {
        let hold = format!(
            "            // the mask shapes what was typed; the caret goes to the next editable position\n\
             \x20           let _val = crawrad_runtime_makepad::edit::hold_mask(cx, &self.ui.text_input(id!({field})), &self.form.{field}.text, &_val, &self.form.{field}.edit_mask).unwrap_or(_val);\n"
        );
        edit::actions_of(document, node, field, &hold)
    }

    fn emit_dsl(
        &self,
        _doc: &Document,
        _reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, crate::Erro> {
        edit::emit_dsl(node, x, y, w, h)
    }
}

#[crawrad_note::note("targets-makepad-components-mask-edit-edit-mask")]
fn edit_mask(node: &Node) -> &str {
    match node.props.get("edit_mask") {
        Some(crawrad_ir::Value::Str(s)) => s.as_str(),
        _ => "",
    }
}
