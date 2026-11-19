//! Rust Teacher chat commands and grounding helpers.

use super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-chat-teacher-ideapp-caret-diagnostic")]
    pub(crate) fn caret_diagnostic(&self, cx: &Cx) -> Option<crawrad_lsp::Diagnostic> {
        let diags = self
            .lsp
            .as_ref()?
            .diagnostics_of(self.abas.active_file()?)?;
        let line0 = self.cursor_do_code_view(cx).0 as u32;
        let error = Some(crawrad_lsp::DiagnosticSeverity::Error);
        let span = |d: &crawrad_lsp::Diagnostic| {
            (d.range.start.line, d.range.end.line, d.severity == error)
        };
        crawrad_agent::teacher::pick_on_line(&diags, line0, span).cloned()
    }

    #[note("ide-app-chat-teacher-ideapp-caret-error-code")]
    pub(crate) fn caret_error_code(&self, cx: &Cx) -> Option<String> {
        let diag = self.caret_diagnostic(cx)?;
        crawrad_agent::teacher::error_code(diag.code.as_deref())
    }

    #[note("ide-app-chat-teacher-ideapp-teacher-grounding")]
    fn teacher_grounding(&self, cx: &Cx) -> Option<crawrad_agent::teacher::Grounding> {
        let file = self.abas.active_file()?.to_string_lossy().into_owned();
        let diag = self.caret_diagnostic(cx)?;
        let line0 = self.cursor_do_code_view(cx).0;
        let code = crawrad_agent::teacher::error_code(diag.code.as_deref());
        Some(crawrad_agent::teacher::Grounding {
            explain: code
                .as_deref()
                .and_then(crawrad_agent::teacher::rustc_explain),
            code,
            message: diag.message,
            file,
            line: line0 as u32 + 1,
            snippet: crawrad_agent::teacher::snippet_around(&self.texto_na_vista, line0, 3),
        })
    }

    #[note("ide-app-chat-teacher-ideapp-teacher-command")]
    pub(crate) fn teacher_command(
        &mut self,
        cx: &mut Cx,
        cmd: &str,
        rest: &str,
    ) -> super::chat::Slash {
        let lang = match self.language {
            crawrad_actions::i18n::Language::Portuguese => "Brazilian Portuguese",
            _ => "English",
        };
        let selected = self.extrair_selecao_do_editor().is_some();
        let grounding = matches!(cmd, "why" | "fix-explain")
            .then(|| self.teacher_grounding(cx))
            .flatten();
        match crawrad_agent::teacher::command(cmd, rest, lang, selected, grounding) {
            crawrad_agent::teacher::Reply::Prompt(p) => super::chat::Slash::Send(p),
            crawrad_agent::teacher::Reply::Say(s) => {
                self.display_chat_response(cx, &s);
                super::chat::Slash::Done
            }
        }
    }
}
