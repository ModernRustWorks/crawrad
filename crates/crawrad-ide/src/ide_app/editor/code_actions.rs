//! **Code actions and quick fixes**: the light bulb on the caret's
//! line when the server offers something there, `Ctrl+.` (or a click on
//! the bulb) for the list, and the chosen edit applied as Rename Symbol
//! applies one — every file or none, the generated form source refused, one
//! undo step per file. The requests and the offer are `crawrad_lsp::code_action`.

use super::*;
use crawrad_lsp::code_action::Asked;
use makepad_code_editor::{ActionBulbRef, ActionBulbWidgetRefExt, code_view::CodeView};
use std::path::{Path, PathBuf};

impl IdeApp {
    /// Every wake: the caret's place asked once, the answer taken, the bulb
    /// shown or hidden.
    pub(super) fn follow_code_actions(&mut self, cx: &mut Cx) {
        if let Some(place) = self.code_place(cx)
            && !self.code_offer.knows(&place)
            && self.code_view_focused(cx)
        {
            self.ask_code_actions(place, false);
        }
        let answer = self
            .code_offer
            .pending_ticket()
            .and_then(|t| self.lsp_hints.code_actions.take(t));
        let by_key = answer.and_then(|a| self.code_offer.answered(a));
        if by_key == Some(true) {
            self.open_code_actions(cx);
        }
        self.draw_bulb(cx);
    }

    /// `editor.code_actions`: the list now when the place is answered, else
    /// asked by key and opened when it is.
    pub(super) fn code_actions_at_caret(&mut self, cx: &mut Cx) {
        let Some(place) = self.code_place(cx) else {
            self.status = Some("Quick fixes need rust-analyzer and a code file".into());
            return;
        };
        match self.code_offer.at(&place) {
            Some(_) => self.open_code_actions(cx),
            None => self.ask_code_actions(place, true),
        }
    }

    fn ask_code_actions(&mut self, place: crawrad_lsp::code_action::Place, by_key: bool) {
        let Some(lsp) = self.lsp.clone() else {
            return;
        };
        let (file, version, caret) = place.clone();
        let ticket = self.code_offer.ask(place, by_key);
        let (file, text) = (&file, &self.texto_na_vista);
        let asked = Asked {
            file,
            version,
            text,
            caret,
        };
        self.lsp_hints.code_actions.request(lsp, ticket, &asked);
    }

    /// The active `.rs` and the caret's line and byte, with a server up.
    fn code_place(&self, cx: &Cx) -> Option<crawrad_lsp::code_action::Place> {
        let file = self.abas.active_file()?.to_path_buf();
        let is_rust = file.extension().is_some_and(|e| e == "rs");
        let place = (file, self.lsp_versao, self.cursor_do_code_view(cx));
        (is_rust && self.lsp.is_some() && self.host.center().is_text()).then_some(place)
    }

    pub(in super::super) fn code_view_focused(&self, cx: &Cx) -> bool {
        let view = self.code_view();
        cx.has_key_focus(view.area())
    }

    /// The actions at the caret's place, if answered.
    fn actions_here(&self, cx: &Cx) -> Option<&[crawrad_lsp::code_action::CodeAction]> {
        self.code_offer.at(&self.code_place(cx)?)
    }

    /// The list under the caret's line, or the sentence that there is none.
    fn open_code_actions(&mut self, cx: &mut Cx) {
        let view = self.code_view();
        let anchor = view.borrow::<CodeView>().and_then(|c| c.caret_anchor());
        let teach = self.caret_error_code(cx).is_some();
        match (
            teach || self.actions_here(cx).is_some_and(|a| !a.is_empty()),
            anchor,
        ) {
            (true, Some(a)) => {
                let em = dvec2(a.at.x, a.at.y + a.line_height);
                self.open_menu = Some(super::Aberto::CodeActions { em, teach });
                self.show_menu(cx);
            }
            _ => self.status = Some("No code actions here".into()),
        }
    }

    /// The rows of the open list: the last place answered, which is the one
    /// the list opened on (applying checks the place again).
    pub(super) fn code_action_rows(&self, teach: bool) -> Vec<crawrad_actions::menu::Row> {
        let here = self.code_offer.here.as_ref().map(|h| h.1.as_slice());
        let titles: Vec<(String, bool)> = here
            .unwrap_or_default()
            .iter()
            .map(|a| (a.title.clone(), a.preferred))
            .collect();
        crawrad_actions::menu::code_action_rows(&titles, teach)
    }

    /// `editor.apply_code_action`: the edit planned on the text the editor
    /// holds, refused whole if it reaches the generated form source or a file
    /// outside the project, then applied as a rename is.
    pub(super) fn apply_code_action(&mut self, cx: &mut Cx, index: usize) {
        let Some(action) = self.actions_here(cx).and_then(|a| a.get(index)).cloned() else {
            self.status = Some("That code action is gone: the code changed, ask again".into());
            return;
        };
        let files: Vec<PathBuf> = action
            .edit
            .keys()
            .filter_map(|u| crawrad_lsp::uri_to_path(u))
            .collect();
        let project = self.host.workspace_root().unwrap_or_default();
        let roots = crawrad_host::navigation::foreign_roots();
        let suffix = crawrad_codegen::SUFIXO_GERADO;
        if let Some(names) = crawrad_host::rename::refused(&files, &project, &roots, suffix) {
            self.status = Some(format!(
                "Code action refused, nothing changed: it would edit {names}"
            ));
            return;
        }
        let enc = self.lsp_encoding();
        let mut read = |f: &Path| self.text_of(f);
        match crawrad_lsp::rename::plan(&action.edit, enc, &mut read) {
            Ok(planned) => {
                let changes: Vec<_> = planned.into_iter().map(super::rename::change_of).collect();
                self.apply_file_changes(cx, &changes);
                self.status = Some(format!("{}, not saved", action.title));
                self.redraw_editor_tabs(cx);
            }
            Err(f) => self.status = Some(format!("Cannot read {}", f.display())),
        }
    }

    /// The bulb at the start of the caret's line while the place has actions.
    fn draw_bulb(&mut self, cx: &mut Cx) {
        let bulb = self.bulb();
        let here = self
            .actions_here(cx)
            .filter(|a| !a.is_empty())
            .map(|a| a.iter().any(|x| x.preferred))
            .or_else(|| self.caret_error_code(cx).map(|_| false));
        let view = self.code_view();
        let line = self.code_place(cx).map(|p| p.2.0);
        let anchor = line.and_then(|l| view.borrow::<CodeView>().and_then(|c| c.line_anchor(l)));
        let shown = bulb.visible();
        match (here, anchor) {
            (Some(preferred), Some(a)) => {
                let origin = self.painel(Panel::Design).widget(id!(code_container));
                let origin = origin.area().rect(cx).pos;
                let (x, y) = (a.at.x - origin.x - 23.0, a.at.y - origin.y + 1.0);
                bulb.set_preferred(cx, preferred);
                bulb.apply_over(cx, live! { margin: { left: (x), top: (y) } });
                bulb.set_visible(cx, true);
            }
            _ => bulb.set_visible(cx, false),
        }
        if shown != bulb.visible() || shown {
            self.ui.redraw(cx);
        }
    }

    /// A click on the bulb: the list.
    pub(super) fn bulb_clicked(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.bulb().clicked(actions) {
            self.open_code_actions(cx);
        }
    }

    /// What `editor.get_code_actions` reads: the place, whether its answer
    /// is in, and its actions.
    pub(super) fn code_actions_json(&self, cx: &Cx) -> serde_json::Value {
        let place = self.code_place(cx);
        let bulb = self.bulb();
        self.code_offer.to_json(place, bulb.visible())
    }

    pub(super) fn bulb(&self) -> ActionBulbRef {
        self.painel(Panel::Design)
            .action_bulb(id!(code_container.code_bulb))
    }
}
