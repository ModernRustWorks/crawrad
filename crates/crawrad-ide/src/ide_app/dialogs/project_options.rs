//! Project › Options…: the IDE draws `crawrad_actions::project_options_dialog`
//! through the one field binder and hands the files to `crawrad_project::project_options`.

use super::super::*;
use crawrad_actions::project_options_dialog::{IconSource, Page, ProjectOptionsState};
use crawrad_project::project_options as po;

/// The page list's rows, the view each one shows, and its page.
const PAGES: [(&[LiveId], &[LiveId], Page); 2] = [
    (id!(popt_page_app), id!(popt_view_app), Page::Application),
    (id!(popt_page_log), id!(popt_view_log), Page::Log),
];

/// The previews beside each other, and the frame each one shows.
const PREVIEWS: [(u32, &[LiveId]); 4] = [
    (256, id!(popt_icon_256)),
    (48, id!(popt_icon_48)),
    (32, id!(popt_icon_32)),
    (16, id!(popt_icon_16)),
];

impl IdeApp {
    pub(in super::super) fn open_project_options(&mut self, cx: &mut Cx) {
        let Some(dir) = self.membro() else {
            self.status = Some("Project Options: no project is open".into());
            return;
        };
        let opts = po::read(&dir);
        let package = crawrad_project::generator::package_name_in(&dir);
        let custom = opts.icon == po::IconChoice::Custom;
        let st =
            ProjectOptionsState::new(&opts.title, custom, &package).with_log(po::read_log(&dir));
        self.project_options = Some((dir.clone(), st));
        self.show_icon_previews(cx, &po::current_icon(&dir, opts.icon));
        self.refresh(cx);
    }

    pub(in super::super) fn draw_project_options(&mut self, cx: &mut Cx) {
        let open = self.project_options.is_some();
        self.show_veil(cx, id!(project_options_dialog), open);
        let Some((_, st)) = self.project_options.as_ref() else {
            return;
        };
        let dlg = self.ui.widget(id!(project_options_dialog.dlg));
        super::fields::paint(cx, &dlg, st);
        for (row, view, page) in PAGES {
            let on = page == st.current_page;
            dlg.view(view).set_visible(cx, on);
            let on = if on { 1.0 } else { 0.0 };
            dlg.view(row)
                .apply_over(cx, live! { draw_bg: { selecionada: (on) } });
        }
        dlg.view(id!(popt_error_wrap))
            .set_visible(cx, st.error.is_some());
    }

    pub(in super::super) fn atender_project_options(&mut self, cx: &mut Cx, actions: &Actions) {
        let dlg = self.ui.widget(id!(project_options_dialog.dlg));
        let Some((dir, st)) = self.project_options.as_mut() else {
            return;
        };
        let _ = super::fields::take_input(cx, &dlg, actions, st);
        let mut icon = None;
        for (row, _, page) in PAGES {
            if super::fields::row_clicked(&dlg, row, actions) {
                st.select_page(page);
            }
        }
        if dlg.button(id!(popt_default_btn)).clicked(actions) {
            st.default_icon();
            icon = Some(po::DEFAULT_ICON.to_vec());
        }
        if dlg.button(id!(popt_load_btn)).clicked(actions)
            && let Some(path) = rfd::FileDialog::new()
                .set_title("Load icon")
                .add_filter("Icon (*.ico, *.png)", &["ico", "png"])
                .pick_file()
        {
            icon = st.load_result(path.clone(), po::icon_bytes(&IconSource::File(path)));
        }
        if dlg.button(id!(plog_browse)).clicked(actions)
            && let Some(folder) = rfd::FileDialog::new().set_title("Log folder").pick_folder()
        {
            st.set_custom_folder(folder.display().to_string());
        }
        if dlg.button(id!(plog_sample)).clicked(actions) {
            match po::write_sample_override(dir, &st.package, &st.log) {
                Ok(path) => self.status = Some(format!("Wrote {}", path.display())),
                Err(e) => st.set_error(e),
            }
        }
        let mut close = dlg.button(id!(popt_close)).clicked(actions)
            || dlg.button(id!(popt_cancel)).clicked(actions);
        // A failed OK keeps the dialog and the pending icon: the user reads why and retries.
        if dlg.button(id!(popt_ok)).clicked(actions) {
            match po::save_dialog(dir, st) {
                Ok(saved) => (self.status, close) = (Some(saved), true),
                Err(e) => st.set_error(e),
            }
        }
        if close {
            self.project_options = None;
        }
        if let Some(ico) = icon {
            self.show_icon_previews(cx, &ico);
        }
        self.refresh(cx);
    }

    fn show_icon_previews(&self, cx: &mut Cx, ico: &[u8]) {
        let dlg = self.ui.view(id!(project_options_dialog.dlg));
        for (size, image) in PREVIEWS {
            if let Some(png) = po::preview_png(ico, size) {
                let _ = dlg.image(image).load_png_from_data(cx, &png);
            }
        }
    }
}
