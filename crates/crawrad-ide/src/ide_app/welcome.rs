//! The Welcome page: the centre when there is no project to reopen.

use crawrad_runtime_makepad::{
    CrawradWelcomePageRef, CrawradWelcomePageWidgetRefExt, WelcomeAction as Click, WelcomeRowView,
};

use std::path::Path;

use super::*;
use crawrad_note::note;

impl IdeApp {
    fn welcome_page(&self) -> CrawradWelcomePageRef {
        self.painel(Panel::Design)
            .crawrad_welcome_page(id!(designer_row.welcome_container.welcome_page))
    }

    #[note("ide-app-welcome-ideapp-show-welcome")]
    pub(super) fn show_welcome(&mut self, cx: &mut Cx) {
        let rows = crawrad_project::workspace::welcome_rows();
        self.welcome = rows.iter().map(|r| r.folder.clone()).collect();
        let view = |r: &crawrad_project::workspace::WelcomeRow| WelcomeRowView {
            name: r.name.clone(),
            place: r.place.clone(),
            forms: r.forms.clone(),
            age: r.age.clone(),
        };
        let page = self.welcome_page();
        page.set_rows(cx, &rows.iter().map(view).collect::<Vec<_>>());
        let reopen = !crawrad_project::workspace::read_preferences().welcome_on_startup;
        page.set_reopen(cx, reopen);
        self.show_center(cx, Center::Welcome);
    }

    #[note("ide-app-welcome-ideapp-open-project")]
    pub(super) fn open_project(&mut self, cx: &mut Cx, folder: Option<PathBuf>) {
        if self
            .host
            .doc()
            .is_some_and(|d| d.revision() != self.rev_salva)
        {
            self.status = Some(Self::TRABALHO_NAO_SALVO.into());
            return self.sincronizar(cx);
        }
        let Some(folder) = folder.or_else(|| rfd::FileDialog::new().pick_folder()) else {
            return;
        };
        if !folder.is_dir() {
            self.status = Some(format!("{} is not a folder", folder.display()));
            return self.sincronizar(cx);
        }
        Self::remember_recent(&folder);
        self.open_created_project(cx, &folder);
        self.sincronizar(cx);
    }

    #[note("ide-app-welcome-ideapp-remember-recent")]
    pub(super) fn remember_recent(folder: &Path) {
        if !cfg!(test) {
            crawrad_project::workspace::remember_recent(folder);
        }
    }

    #[note("ide-app-welcome-ideapp-open-form-file")]
    pub(super) fn open_form_file(&mut self, cx: &mut Cx, form: Option<PathBuf>) {
        let pick = || {
            rfd::FileDialog::new()
                .add_filter("CrawRAD form", &["crawform"])
                .pick_file()
        };
        if let Some(form) = form.or_else(pick) {
            self.abrir_formulario(cx, &form);
        }
    }

    #[note("ide-app-welcome-ideapp-forget-recent")]
    pub(super) fn forget_recent(&mut self, cx: &mut Cx, folder: &Path) {
        crawrad_project::workspace::forget_recent(folder);
        if self.host.center() == Center::Welcome {
            self.show_welcome(cx);
        }
    }

    #[note("ide-app-welcome-ideapp-handle-welcome-actions")]
    pub(super) fn handle_welcome_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let folder = |i: usize, app: &Self| app.welcome.get(i).map(|f| f.display().to_string());
        match self.welcome_page().clicked(actions) {
            Some(Click::NewProject) => self.execute_action(cx, "project.new"),
            Some(Click::OpenFolder) => self.execute_action(cx, "project.open_folder"),
            Some(Click::OpenForm) => self.execute_action(cx, "file.open_form"),
            Some(Click::Open(i)) => {
                if let Some(f) = folder(i, self) {
                    let path = crawrad_actions::Args::text("path", f);
                    self.execute_with(cx, "project.open_folder", path);
                }
            }
            Some(Click::Forget(i)) => {
                if let Some(f) = folder(i, self) {
                    let args = crawrad_actions::Args::text("path", f);
                    self.execute_with(cx, "project.forget_recent", args);
                }
            }
            Some(Click::Reopen(on)) => {
                crawrad_project::workspace::update_preferences(|p| p.welcome_on_startup = !on);
                self.welcome_page().set_reopen(cx, on);
            }
            Some(Click::Close) => self.show_center(cx, Center::Design),
            None => {}
        }
    }

    #[note("ide-app-welcome-ideapp-welcome-json")]
    pub(super) fn welcome_json(&self, cx: &Cx) -> serde_json::Value {
        let rect = |(name, r): (String, Rect)| serde_json::json!({ "name": name, "rect": xywh(r) });
        let controls: Vec<_> = self
            .welcome_page()
            .drawn_controls(cx)
            .into_iter()
            .map(rect)
            .collect();
        let recent: Vec<_> = self
            .welcome
            .iter()
            .map(|f| f.display().to_string())
            .collect();
        serde_json::json!({
            "open": self.host.center() == Center::Welcome,
            "reopen_last_project": !crawrad_project::workspace::read_preferences().welcome_on_startup,
            "recent": recent,
            "controls": controls,
        })
    }
}
