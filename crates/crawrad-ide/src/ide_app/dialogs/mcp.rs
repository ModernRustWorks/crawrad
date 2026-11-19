//! The connected MCP Servers dialog (Ref 04: `04_modal_mcp_servers.png`).
//!
//! Displays the list of configured MCP servers (built-in and custom)
//! with connection status and transport details.

use super::super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-dialogs-mcp-ideapp-open-mcp")]
    pub(in super::super) fn open_mcp(&mut self, cx: &mut Cx) {
        self.open_options(cx);
        self.secao = options::Category::AiAcp;
        self.redraw_options(cx);
    }

    #[note("ide-app-dialogs-mcp-ideapp-draw-mcp")]
    pub(in super::super) fn draw_mcp(&mut self, cx: &mut Cx) {
        self.show_veil(cx, id!(mcp_dialog), self.mcp_aberto);
        if !self.mcp_aberto {
            return;
        }

        let reg = crawrad_agent::McpRegistry::new();
        let mut resumo = String::new();
        for s in &reg.servers {
            let badge = match s.source {
                crawrad_agent::McpSource::BuiltIn => "BUILT-IN",
                crawrad_agent::McpSource::Addon => "ADDON",
                crawrad_agent::McpSource::Custom => "CUSTOM",
            };
            let is_available = s.probe();
            let status_icon = if is_available { "🟢" } else { "⚪" };
            let status_text = if is_available {
                "Installed"
            } else {
                "Not installed"
            };
            resumo.push_str(&format!(
                "{status_icon} {} [{badge}] — {:?} ({}) [{status_text}]\n",
                s.name, s.transport, s.command
            ));
        }

        let dlg = self.ui.view(id!(mcp_dialog.dlg));
        dlg.label(id!(mcp_servers_info)).set_text(cx, &resumo);
        dlg.button(id!(mcp_close)).set_text(cx, "Close");
    }

    #[note("ide-app-dialogs-mcp-ideapp-atender-mcp")]
    pub(in super::super) fn atender_mcp(&mut self, cx: &mut Cx, actions: &Actions) {
        if !self.mcp_aberto {
            return;
        }
        let dlg = self.ui.view(id!(mcp_dialog.dlg));
        if dlg.button(id!(mcp_close)).clicked(actions) {
            self.mcp_aberto = false;
            self.refresh(cx);
        }
    }
}
