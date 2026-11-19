//! The project desktop: kept when the IDE closes or leaves a project,
//! given back when the project opens. The file and its rules are
//! `crawrad_project::desktop`; this is only where the IDE's state is read and set.

use crawrad_actions::ProfileCommand::{self, Apply, Delete, Save};
use crawrad_project::desktop::{Desktop, WindowPlace};
use crawrad_project::profile::{self, Profile};
use crawrad_project::workspace::{read_preferences, update_preferences};

use super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-desktop-ideapp-switch-tab")]
    pub(super) fn switch_tab(&mut self, cx: &mut Cx, to: impl FnOnce(&mut EditorTabs)) {
        self.remember_front_caret(cx);
        to(&mut self.abas);
        let back = self.abas.active_file().and_then(|f| self.abas.caret_of(f));
        self.pouso_pendente = back
            .map(|(l, b)| PousoCodigo::at(l, b))
            .or(self.pouso_pendente);
    }

    fn remember_front_caret(&mut self, cx: &Cx) {
        if let Some(h) = self.here(cx) {
            self.abas.remember_caret(&h.file, h.line, h.byte);
        }
    }

    #[note("ide-app-desktop-ideapp-save-desktop")]
    pub(super) fn save_desktop(&mut self, cx: &mut Cx) {
        self.recover.close();
        let keep = read_preferences().save_desktop;
        let Some(root) = self.desktop_root.clone().filter(|_| keep) else {
            return;
        };
        self.remember_front_caret(cx);
        let d = Desktop::new(self.window_place(cx), layout::encode(&self.arranjo()));
        let tabs = &self.abas;
        let caret = |f: &std::path::Path| tabs.caret_of(f);
        let _ = d
            .with_tabs(&root, tabs.files(), tabs.active_index(), caret)
            .write(&root);
    }

    #[note("ide-app-desktop-ideapp-window-place")]
    fn window_place(&self, cx: &Cx) -> WindowPlace {
        let geom = cx.windows[self.window_id(cx)].window_geom.clone();
        let r = (self.restore_to.filter(|_| self.maximizada))
            .unwrap_or_else(|| maximize::on_screen(&geom));
        WindowPlace::new([r.x, r.y, r.width, r.height], self.maximizada)
    }

    #[note("ide-app-desktop-ideapp-run-profile")]
    pub(super) fn run_profile(&mut self, cx: &mut Cx, command: ProfileCommand) {
        let dir = profile::folder().unwrap_or_default();
        let done = match &command {
            Apply(n) => Profile::read(&dir, n).map(|p| self.apply_profile(cx, p)),
            Save(n) => self.current_profile(cx).write(&dir, n),
            Delete(n) => Profile::delete(&dir, n),
        };
        self.status = Some(profile::settle(&command, done));
        self.redraw_options(cx);
    }

    #[note("ide-app-desktop-ideapp-current-profile")]
    fn current_profile(&self, cx: &Cx) -> Profile {
        Profile {
            window: Some(self.window_place(cx)),
            name_width: Some(self.name_splitter.requested),
            editor_font: Some(read_preferences().editor_font_size),
            zoom: Some(self.zoom_da_ide),
            grid: Some(self.designer.grid as u32),
            dock: layout::encode(&self.arranjo()),
        }
    }

    #[note("ide-app-desktop-ideapp-apply-profile")]
    fn apply_profile(&mut self, cx: &mut Cx, p: Profile) {
        self.load_state(
            cx,
            layout::decode(&p.dock).unwrap_or_else(Self::arranjo_padrao),
        );
        self.show_center(cx, self.host.center());
        self.name_splitter.requested = p.name_width.unwrap_or(inspector::NAME_WIDTH_DEFAULT);
        let font = p
            .editor_font
            .unwrap_or(crawrad_project::workspace::Preferences::default().editor_font_size);
        update_preferences(|pref| pref.editor_font_size = font);
        self.aplicar_tema_editor(cx);
        // The interface zoom is an IDE setting, not a project one (owner, 13/11).
        self.set_ide_zoom(cx, crawrad_project::workspace::read_preferences().ide_scale);
        let grid = p
            .grid
            .unwrap_or(crawrad_project::workspace::Preferences::default().designer_grid);
        self.set_grid_size(cx, grid as f32);
        if let Some(w) = p.window {
            if w.maximized != self.maximizada {
                self.toggle_maximized(cx);
            }
            self.startup_window = Some(w);
            self.place_startup_window();
            self.startup_window = None;
        }
        self.restore_root_if_lost(cx);
        self.escrever_rotulos_das_abas(cx);
    }

    #[note("ide-app-desktop-ideapp-restore-desktop")]
    pub(super) fn restore_desktop(&mut self, cx: &mut Cx, root: &std::path::Path, startup: bool) {
        self.desktop_root = Some(root.to_path_buf());
        self.read_project_sizes(root);
        let (kind, without_form) = self.project_kind_now();
        self.project_kind = kind.map(str::to_string);
        self.project_without_form = without_form;
        self.refresh_footer(cx);
        let keep = read_preferences().save_desktop;
        let Some(d) = Desktop::read(root).filter(|_| keep) else {
            return;
        };
        if let Some(arranjo) = layout::decode(&d.dock).filter(|_| startup) {
            self.load_state(cx, arranjo);
            self.startup_window = d.window;
        }
        self.desktop_tabs = Some((root.to_path_buf(), d));
    }

    #[note("ide-app-desktop-ideapp-restore-desktop-tabs")]
    pub(super) fn restore_desktop_tabs(&mut self, cx: &mut Cx) {
        let canvas = id!(designer_row.designer_canvas);
        let drawn = self.painel(Panel::Design).view(canvas).area() != Area::Empty;
        let Some((root, d)) = self.desktop_tabs.take_if(|_| drawn) else {
            return;
        };
        let mut front = None;
        for (i, t) in d.tabs.iter().enumerate() {
            let file = root.join(&t.file);
            let at = self.abas.open(file.clone());
            self.abas.remember_caret(&file, t.line, t.byte);
            front = front.or((d.active == Some(i)).then_some(at));
        }
        if !d.tabs.is_empty() {
            self.abas.show_design();
            if let Some(i) = front {
                self.switch_tab(cx, |a| a.activate(i));
            }
            self.show_center(cx, self.centro_das_abas());
        }
    }

    #[note("ide-app-desktop-ideapp-place-startup-window")]
    pub(super) fn place_startup_window(&mut self) -> bool {
        let Some(w) = self.startup_window.filter(|w| !w.maximized) else {
            return false;
        };
        let rect = crawrad_os::ScreenRect::of_window([w.x, w.y], [w.width, w.height], 1.0);
        let centre = (rect.x + rect.width / 2.0, rect.y + rect.height / 2.0);
        let fits = crawrad_os::monitor_at(centre).is_some_and(|m| rect.fits_in(&m.full));
        fits && crawrad_os::place_main_window(rect)
    }
}
