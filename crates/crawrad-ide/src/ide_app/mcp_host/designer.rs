//! MCP host tools: Form Designer & Visual Tree (designer.*). The tools live in
//! `crawrad_mcp::designer_tool`; here the IDE lends them the form and its actions.

use super::*;
use crawrad_mcp::designer_tool::{self as tool, Designer};
use crawrad_mcp::protocol::ToolCallResult;
use crawrad_note::note;
use makepad_widgets::Cx;
use serde_json::Value;

#[note("ide-app-mcp-host-designer-live")]
struct Live<'a> {
    app: &'a mut IdeApp,
    cx: &'a mut Cx,
    session: &'a str,
}

impl Designer for Live<'_> {
    fn doc(&self) -> Option<&crawrad_doc::Doc> {
        self.app.doc()
    }

    fn doc_mut(&mut self) -> Option<&mut crawrad_doc::Doc> {
        self.app.doc_mut()
    }

    fn run(&mut self, name: &str, args: &Value) -> Result<(), String> {
        let action = crawrad_mcp::action_tool::check_designer_action(name, args)?;
        let app = &mut *self.app;
        let ctx = crawrad_actions::Context {
            doc: app.doc(),
            registry: &crawrad_project::installed::ide_catalog().catalog,
            clipboard: app.area_transferencia(),
            args: convert_json_args_to_crawrad(args),
            project: app.projeto_contexto(),
            author: crawrad_actions::Author::Agent(format!("mcp-{}", self.session)),
            editor_dirty: app.is_editor_dirty(),
            text: app.editor_text(),
        };
        if !action.enabled.allows(&ctx) {
            return Err(format!(
                "Action '{name}' is currently disabled in the IDE context."
            ));
        }
        let efeito = (action.execute)(&ctx);
        if let Some(status) = &efeito.status
            && efeito.transaction.is_none()
            && efeito.selection.is_none()
            && efeito.file.is_none()
        {
            return Err(status.clone());
        }
        app.apply_effect(self.cx, efeito).map_err(|e| e.to_string())
    }

    fn prop_keys(&self, class: &str) -> Vec<&'static str> {
        let catalog = &crawrad_project::installed::ide_catalog().catalog;
        catalog
            .get(class)
            .map(|d| d.props.iter().map(|p| p.key).collect())
            .unwrap_or_default()
    }
}

impl McpServer {
    #[note("ide-app-mcp-host-designer-mcpserver-tool-designer-on-form")]
    pub(super) fn tool_designer_on_form(
        &mut self,
        app: &mut IdeApp,
        cx: &mut Cx,
        name: &str,
        args: &Value,
    ) -> ToolCallResult {
        let session = self.session_id.clone();
        let l = &mut Live {
            app,
            cx,
            session: &session,
        };
        let a = match name {
            "designer.add_component" => tool::add_component(l, args),
            "designer.set_component_properties" => tool::set_component_properties(l, args),
            "designer.delete_component" => tool::delete_component(l, args),
            _ => wire_event(l, args),
        };
        self.audited(name, args, a)
    }
}

#[note("ide-app-mcp-host-designer-wire-event")]
fn wire_event(l: &mut Live<'_>, args: &Value) -> crawrad_mcp::run_tool::Answer {
    let w = match tool::wire_event(l, args) {
        Ok(w) => w,
        Err(answer) => return answer,
    };
    let unit = l
        .app
        .caminho_ativo()
        .map(crawrad_project::source::unit_path);
    let find = |u: &std::path::Path| {
        let text = std::fs::read_to_string(u).ok()?;
        crawrad_codegen::elo::localizar_handler(&text, &w.handler).map(|d| d.linha)
    };
    let existed = unit.as_deref().and_then(find).is_some();
    // The bind is applied either way; a refused generation is said, not hidden.
    let mut note = None;
    if !existed {
        l.app.gerar_projeto(l.cx);
        let status = l.app.status.clone();
        note = status.filter(|s| crawrad_project::generator::refused(s));
    }
    let line = unit.as_deref().and_then(find);
    let file = unit.as_ref().map(|u| u.display().to_string());
    let rev = l.doc().map(|d| d.revision()).unwrap_or(0);
    tool::wired(&w, rev, file, line, !existed && line.is_some(), note)
}
