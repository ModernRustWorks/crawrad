//! Formal trait definition for Slint component code generation lifecycle.

use crate::Erro;
use crawrad_ir::{Document, Node};

/// Contract implemented by every component in the Slint target.
pub trait SlintComponentCodegen: Send + Sync {
    /// Canonical component class name in the CrawRAD catalog (e.g. "Button").
    fn class(&self) -> &'static str;

    /// Class name identifier.
    fn class_name(&self) -> &'static str {
        self.class()
    }

    /// Generates the Rust struct definition for this component in the user's form.
    fn struct_definition(&self) -> String;

    /// Emits the default value instantiation expression in `Form::default()`.
    fn emit_default(&self, node: &Node) -> String;

    /// Emits default value with access to document context.
    fn emit_default_doc(&self, _doc: &Document, node: &Node) -> String {
        self.emit_default(node)
    }

    /// Emits UI state synchronization logic from `self.form` to Slint handles.
    fn emit_sync(&self, _doc: &Document, _node: &Node, _field: &str) -> String {
        String::new()
    }

    /// Emits action and callback wiring logic for this component.
    fn emit_actions(
        &self,
        // Document containing nodes
        _doc: &Document,
        // Node being wired
        _node: &Node,
        // Target struct field name
        _field: &str,
        // All emitted nodes in form
        _emitted: &[&Node],
    ) -> Result<String, Erro> {
        Ok(String::new())
    }

    /// Emits the `.slint` markup element for this component.
    fn emit_slint(
        &self,
        // Source document
        _doc: &Document,
        // Registry
        _reg: &crawrad_registry::Registry,
        // Target component node
        _node: &Node,
        // Control coordinates and dimensions
        _x: f32,
        _y: f32,
        _w: f32,
        _h: f32,
    ) -> Result<String, Erro> {
        Ok(String::new())
    }

    /// Declares any Slint callbacks exposed by this component on the root window.
    fn slint_callbacks(&self, _node: &Node) -> Vec<String> {
        Vec::new()
    }

    /// Declares any Slint in-out properties exposed by this component on the root window.
    fn slint_properties(&self, _node: &Node) -> Vec<String> {
        Vec::new()
    }

    /// Declares any std-widgets required by this component from "std-widgets.slint".
    fn std_widgets(&self) -> &'static [&'static str] {
        &[]
    }
}
