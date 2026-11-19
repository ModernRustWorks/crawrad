//! External file change detection and watcher management.

use std::path::{Path, PathBuf};

use crawrad_actions::Panel;
use crawrad_host::disk_watch::{DiskAction, file_name};
use makepad_widgets::*;

use super::*;
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-disk-watch-ideapp-pump-disk-changes")]
    pub(crate) fn pump_disk_changes(&mut self, cx: &mut Cx) {
        let open_files: Vec<PathBuf> = self.abas.files().to_vec();
        let active = self.abas.active_file();
        let (redraw_explorer, reactions) = self.disk.drain(
            &open_files,
            active,
            &|p| self.host.buffer(p).map(str::to_string),
            &self.texto_na_vista,
        );
        if redraw_explorer {
            self.redraw_panel_list(cx, Panel::Explorer, id!(explorer_body.explorer_list));
        }
        for r in reactions {
            match r.action {
                DiskAction::Deleted => {
                    self.status = Some(format!("{} was deleted on disk", r.name));
                }
                DiskAction::Same => {}
                DiskAction::Reload => {
                    // Changed outside: its session starts again from the disk.
                    self.change_tracker.forget(&r.path);
                    if r.active {
                        let (line, byte) = self.cursor_do_code_view(cx);
                        self.show_center(cx, self.host.center());
                        self.pousar_cursor(
                            cx,
                            &crate::ide_app::PousoCodigo {
                                linha: line,
                                start_byte: byte,
                                end_byte: byte,
                            },
                        );
                        self.lsp_notificar_salvamento(&r.path);
                        self.status = Some(format!("{} reloaded — changed on disk", r.name));
                    }
                }
                DiskAction::Conflict => {
                    self.status = Some(format!(
                        "{} changed on disk — you have unsaved edits (nothing was overwritten)",
                        r.name
                    ));
                    self.update_disk_change_bar(cx);
                }
            }
        }
    }

    #[note("ide-app-disk-watch-ideapp-conflict-on-active")]
    fn conflict_on_active(&mut self) -> Option<(PathBuf, String)> {
        let path = self
            .active_file_path()
            .filter(|p| self.disk.conflict() == Some(p));
        if path.is_none() {
            self.status = Some("no change on disk to resolve".into());
        }
        path.map(|p| (p.clone(), file_name(&p)))
    }

    pub(super) fn keep_mine(&mut self, cx: &mut Cx) {
        let Some((path, name)) = self.conflict_on_active() else {
            return;
        };
        self.disk.keep_mine(&path);
        self.status = Some(format!(
            "kept your version of {name} — Save writes it over the disk copy"
        ));
        self.update_disk_change_bar(cx);
    }

    pub(super) fn reload_from_disk(&mut self, cx: &mut Cx) {
        let Some((path, name)) = self.conflict_on_active() else {
            return;
        };
        self.host.remove_buffer(&path);
        self.disk.clear_conflict();
        self.change_tracker.forget(&path);
        self.show_center(cx, self.host.center());
        self.redraw_editor_tabs(cx);
        self.lsp_notificar_salvamento(&path);
        self.status = Some(format!(
            "{name} reloaded from disk — your edits were dropped"
        ));
        self.update_disk_change_bar(cx);
    }

    pub(super) fn compare_with_disk(&mut self, cx: &mut Cx) {
        let Some((path, name)) = self.conflict_on_active() else {
            return;
        };
        let original = self
            .host
            .buffer(&path)
            .map(str::to_string)
            .unwrap_or_else(|| self.texto_na_vista.clone());
        let disk_raw = std::fs::read_to_string(&path).unwrap_or_default();
        let proposed = source::without_cr(&disk_raw);
        let _ = self.propose_diff_review(path, original, proposed, "disk");
        self.update_diff_review_bar(cx);
        self.disk.clear_conflict();
        self.status = Some(format!(
            "comparing {name} with the disk copy — Accept takes the disk, Reject keeps yours"
        ));
        self.update_disk_change_bar(cx);
    }

    pub fn update_disk_change_bar(&mut self, cx: &mut Cx) {
        let active = self.active_file_path();
        let has_conflict = active
            .as_ref()
            .is_some_and(|p| self.disk.conflict() == Some(p));
        let design = self.painel(Panel::Design);
        let bar = design.view(id!(disk_change_bar));
        if has_conflict {
            let name = file_name(active.as_deref().unwrap_or(Path::new("")));
            design.label(id!(disk_change_bar.lbl_disk_change)).set_text(
                cx,
                &format!("{name} changed on disk, and you have unsaved edits here."),
            );
            bar.set_visible(cx, true);
        } else {
            bar.set_visible(cx, false);
        }
        bar.redraw(cx);
    }
}
