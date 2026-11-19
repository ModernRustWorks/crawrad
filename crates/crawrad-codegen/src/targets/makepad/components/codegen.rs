//! Formal trait definition for Makepad component code generation lifecycle.

use crate::Erro;
use crawrad_ir::{Document, Node};

#[crawrad_note::note("targets-makepad-components-codegen-makepadcomponentcodegen")]
pub trait MakepadComponentCodegen: Send + Sync {
    #[crawrad_note::note("targets-makepad-components-codegen-makepadcomponentcodegen-class")]
    fn class(&self) -> &'static str;

    #[crawrad_note::note("targets-makepad-components-codegen-makepadcomponentcodegen-class-name")]
    fn class_name(&self) -> &'static str {
        self.class()
    }

    #[note("targets-makepad-components-codegen-makepadcomponentcodegen-whole-look-widget")]
    fn whole_look_widget(&self) -> Option<&'static str> {
        None
    }

    #[crawrad_note::note(
        "targets-makepad-components-codegen-makepadcomponentcodegen-default-cursor"
    )]
    fn default_cursor(&self) -> &'static str {
        "arrow"
    }

    #[crawrad_note::note("targets-makepad-components-codegen-makepadcomponentcodegen-text-path")]
    fn text_path(&self, _node: &Node, field: &str) -> String {
        field.to_string()
    }

    #[crawrad_note::note("targets-makepad-components-codegen-makepadcomponentcodegen-enter-key")]
    fn enter_key(&self) -> EnterKey {
        EnterKey::Untouched
    }

    #[crawrad_note::note(
        "targets-makepad-components-codegen-makepadcomponentcodegen-struct-definition"
    )]
    fn struct_definition(&self) -> String;

    #[crawrad_note::note("targets-makepad-components-codegen-makepadcomponentcodegen-emit-default")]
    fn emit_default(&self, node: &Node) -> String;

    #[crawrad_note::note(
        "targets-makepad-components-codegen-makepadcomponentcodegen-emit-default-doc"
    )]
    fn emit_default_doc(&self, _doc: &Document, node: &Node) -> String {
        self.emit_default(node)
    }

    #[crawrad_note::note("targets-makepad-components-codegen-makepadcomponentcodegen-emit-sync")]
    fn emit_sync(&self, doc: &Document, node: &Node, field: &str) -> String;

    /// Emits UI synchronization logic with catalogue access.
    fn emit_sync_with_reg(
        &self,
        _reg: &crawrad_registry::Registry,
        doc: &Document,
        node: &Node,
        field: &str,
    ) -> String {
        self.emit_sync(doc, node, field)
    }

    #[crawrad_note::note("targets-makepad-components-codegen-makepadcomponentcodegen-emit-actions")]
    fn emit_actions(
        &self,
        doc: &Document,
        node: &Node,
        field: &str,
        emitted: &[&Node],
    ) -> Result<String, Erro>;

    /// Emits action dispatch logic with catalogue access.
    fn emit_actions_with_reg(
        &self,
        _reg: &crawrad_registry::Registry,
        doc: &Document,
        node: &Node,
        field: &str,
        emitted: &[&Node],
    ) -> Result<String, Erro> {
        self.emit_actions(doc, node, field, emitted)
    }

    #[crawrad_note::note("targets-makepad-components-codegen-makepadcomponentcodegen-app-state")]
    fn app_state(&self, _node: &Node, _field: &str) -> Result<Option<AppState>, Erro> {
        Ok(None)
    }

    #[crawrad_note::note(
        "targets-makepad-components-codegen-makepadcomponentcodegen-app-state-doc"
    )]
    fn app_state_doc(
        &self,
        _doc: &Document,
        node: &Node,
        field: &str,
    ) -> Result<Option<AppState>, Erro> {
        self.app_state(node, field)
    }

    #[crawrad_note::note("targets-makepad-components-codegen-makepadcomponentcodegen-focus-path")]
    fn focus_path(&self, _doc: &Document, _node: &Node, _field: &str) -> Option<String> {
        None
    }

    #[crawrad_note::note("targets-makepad-components-codegen-makepadcomponentcodegen-emit-dsl")]
    fn emit_dsl(
        &self,
        doc: &Document,
        // O CATÁLOGO — inclusive o que veio de um `manifest.json`.
        reg: &crawrad_registry::Registry,
        node: &Node,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> Result<String, Erro>;
}

#[crawrad_note::note("targets-makepad-components-codegen-enterkey")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnterKey {
    /// Not a text box: the key goes on as it came.
    Untouched,
    /// A multi-line box: Enter breaks the line.
    BreaksLine,
    /// A single-line box: the box never gets an Enter.
    Swallowed,
}

#[crawrad_note::note("targets-makepad-components-codegen-appstate")]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AppState {
    /// A `use` line the field's type needs. Makepad 1.0.0's `#[derive(Live)]`
    /// rejects a field whose type is a path (`a::b::C`): measured, it reports
    /// "Unexpected field form" on the field BEFORE it, so the type goes in by
    /// its short name and this brings it in.
    pub uses: String,
    /// Its `#[rust]` field declaration(s) in `struct App`.
    pub field: String,
    /// Its line(s) in `handle_event`, before the widgets see the event.
    pub handle: String,
}

/// Backward-compatibility alias for MakepadComponentCodegen.
pub use MakepadComponentCodegen as ComponentCodegen;
use crawrad_note::note;
