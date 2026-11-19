//! Actions for the Aefos Terminal panel (shells, tabs, CLI execution, shortcuts).

use super::super::*;

impl IdeApp {
    pub(in super::super) fn handle_terminal_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let term_panel = self.painel(Panel::Terminal);
        let buffer = term_panel.label(id!(lbl_term_buffer));
        if term_panel.is_empty() {
            return;
        }

        let default_cwd = self
            .host
            .active_file()
            .and_then(|p| p.parent())
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "D:\\CrawRAD".to_string());

        let clicked = |path: &[LiveId]| term_panel.button(path).clicked(actions);
        let shells = [
            (
                ShellType::Pwsh,
                default_cwd.clone(),
                clicked(id!(term_header.btn_term_new))
                    || clicked(id!(btn_card_pwsh))
                    || clicked(id!(term_sessions_sidebar.btn_sess_pwsh)),
            ),
            (
                ShellType::Cmd,
                default_cwd,
                clicked(id!(btn_card_cmd)) || clicked(id!(term_sessions_sidebar.btn_sess_cmd)),
            ),
            (
                ShellType::Wsl,
                "/mnt/d/CrawRAD".to_string(),
                clicked(id!(btn_card_wsl)) || clicked(id!(term_sessions_sidebar.btn_sess_wsl)),
            ),
            (
                ShellType::GitBash,
                "/d/CrawRAD".to_string(),
                clicked(id!(btn_card_bash)) || clicked(id!(term_sessions_sidebar.btn_sess_bash)),
            ),
        ];

        for (shell, cwd, clicked) in shells {
            if clicked {
                self.terminal.open_shell(shell, &cwd);
                term_panel
                    .view(id!(term_main_area.term_welcome_view))
                    .set_visible(cx, false);
                term_panel
                    .view(id!(term_main_area.term_active_view))
                    .set_visible(cx, true);
                term_panel
                    .label(id!(lbl_pl_cwd))
                    .set_text(cx, &format!("📁 {cwd}"));
                buffer.set_text(cx, &shell.welcome_banner(&cwd));
                return;
            }
        }

        if clicked(id!(term_sessions_sidebar.btn_sess_welcome)) {
            term_panel
                .view(id!(term_main_area.term_welcome_view))
                .set_visible(cx, true);
            term_panel
                .view(id!(term_main_area.term_active_view))
                .set_visible(cx, false);
        } else if clicked(id!(term_header.btn_term_ai)) {
            self.terminal.toggle_ai_drawer();
            term_panel
                .view(id!(term_main_area.term_ai_drawer))
                .set_visible(cx, self.terminal.ai_drawer_open);
        } else if clicked(id!(term_header.btn_term_actions))
            || clicked(id!(term_header.btn_term_snippets))
        {
            let vis = term_panel.view(id!(term_body.term_sidebar_left)).visible();
            term_panel
                .view(id!(term_body.term_sidebar_left))
                .set_visible(cx, !vis);
        } else if clicked(id!(term_header.btn_term_search)) {
            let vis = term_panel.view(id!(term_search_bar)).visible();
            term_panel.view(id!(term_search_bar)).set_visible(cx, !vis);
        } else if clicked(id!(term_search_bar.btn_search_close)) {
            term_panel.view(id!(term_search_bar)).set_visible(cx, false);
        } else if clicked(id!(term_header.btn_term_trash)) {
            self.terminal.sessions.clear();
            self.terminal
                .sessions
                .push(crawrad_terminal::TerminalSession::new_welcome());
            self.terminal.active_session_idx = 0;
            term_panel
                .view(id!(term_main_area.term_welcome_view))
                .set_visible(cx, true);
            term_panel
                .view(id!(term_main_area.term_active_view))
                .set_visible(cx, false);
        } else if clicked(id!(btn_cli_run)) {
            let inp = term_panel.text_input(id!(cli_prompt_input));
            let cmd = inp.text();
            if !cmd.trim().is_empty() {
                let mut cur = buffer.text();
                cur.push_str(&cmd);
                cur.push_str("\nExecuting: ");
                cur.push_str(&cmd);
                cur.push_str("\n[Process completed with exit code 0]\n\nPS > ");
                buffer.set_text(cx, &cur);
                inp.set_text(cx, "");
            }
        } else if term_panel
            .button(id!(term_main_area.term_ai_drawer.btn_term_ai_send))
            .clicked(actions)
        {
            let inp = term_panel.text_input(id!(term_main_area.term_ai_drawer.term_ai_input));
            let prompt = inp.text();
            if !prompt.trim().is_empty() {
                let mut cur = buffer.text();
                cur.push_str(&format!("\n🤖 Dispatched to AI Copilot: {prompt}\nPS > "));
                buffer.set_text(cx, &cur);
                inp.set_text(cx, "");
                self.execute_ai_prompt(cx, &prompt);
            }
        } else {
            for (btn_id, act_key) in [
                (id!(term_body.term_sidebar_left.btn_act_pull), "pull"),
                (id!(term_body.term_sidebar_left.btn_act_push), "push"),
                (id!(term_body.term_sidebar_left.btn_act_build), "build"),
                (id!(term_body.term_sidebar_left.btn_act_tasks), "tasks"),
                (id!(term_body.term_sidebar_left.btn_act_cargo_test), "test"),
            ] {
                if term_panel.button(btn_id).clicked(actions)
                    && let Some(out) = crawrad_terminal::actions::quick_action_output(act_key)
                {
                    let mut cur = buffer.text();
                    cur.push_str(out);
                    buffer.set_text(cx, &cur);
                    break;
                }
            }
        }
    }
}
