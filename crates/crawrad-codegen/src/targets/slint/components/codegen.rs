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
    fn emit_sync(&self, doc: &Document, node: &Node, field: &str) -> String;

    /// Emits action and callback wiring logic for this component.
    fn emit_actions(
        &self,
        // Document containing nodes
        doc: &Document,
        // Node being wired
        node: &Node,
        // Target struct field name
        field: &str,
        // All emitted nodes in form
        emitted: &[&Node],
    ) -> Result<String, Erro>;

    /// Emits the `.slint` markup element for this component.
    fn emit_slint(
        &self,
        // Source document
        doc: &Document,
        // Registry
        reg: &crawrad_registry::Registry,
        // Target component node
        node: &Node,
        // Control coordinates and dimensions
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, Erro>;

    /// Declares any Slint callbacks exposed by this component on the root window.
    fn slint_callbacks(&self, _node: &Node) -> Vec<String> {
        Vec::new()
    }

    /// Declares any Slint in-out properties exposed by this component on the root window.
    fn slint_properties(&self, _node: &Node) -> Vec<String> {
        Vec::new()
    }
}
