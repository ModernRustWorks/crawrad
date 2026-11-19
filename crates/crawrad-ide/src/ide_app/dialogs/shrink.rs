//! The "Shrink the Release build" modal dialog.
//!
//! Provides a dedicated dialog allowing users to inspect and toggle
//! recommended `[profile.release]` optimizations in Cargo.toml.

use super::super::*;
use crawrad_project::release_size::{ShrinkDialog, apply_shrink, shrink_preview, shrink_status};
use makepad_code_editor::release_size_view::{ShrinkDialogAction, ShrinkDialogViewWidgetRefExt};

impl IdeApp {
    /// Opens the "Shrink the Release build" dialog reading the workspace root Cargo.toml.
    pub(in super::super) fn open_shrink(&mut self, cx: &mut Cx) {
        let root_opt = self.alvo_do_cargo().map(|(r, _)| r).or_else(|| self.raiz());
        let Some(root) = root_opt else {
            self.status = Some("Shrink tips: no Cargo workspace root found".into());
            return;
        };
        let manifest_path = root.join("Cargo.toml");
        let manifest = match std::fs::read_to_string(&manifest_path) {
            Ok(content) => content,
            Err(err) => {
                self.status = Some(format!(
                    "Shrink tips: cannot read {}: {err}",
                    manifest_path.display()
                ));
                return;
            }
        };

        let dialog = ShrinkDialog::new(manifest_path, manifest);
        let view = self.ui.shrink_dialog_view(id!(shrink_dialog.dlg));
        view.set_tips(cx, &dialog.tip_rows(), &dialog.checked_indices());
        let preview = shrink_preview(&dialog.manifest, &dialog.checked);
        view.set_preview(cx, &preview);
        self.shrink_dialog = Some(dialog);
        self.draw_shrink(cx);
        self.refresh(cx);
    }

    /// Closes the shrink dialog without applying changes.
    pub(in super::super) fn close_shrink(&mut self, cx: &mut Cx) {
        self.shrink_dialog = None;
        self.draw_shrink(cx);
        self.refresh(cx);
    }

    /// Draws the "Shrink the Release build" dialog.
    pub(in super::super) fn draw_shrink(&mut self, cx: &mut Cx) {
        let is_open = self.shrink_dialog.is_some();
        self.show_veil(cx, id!(shrink_dialog), is_open);
    }

    /// Handles actions and events inside the shrink dialog.
    pub(in super::super) fn handle_shrink(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.shrink_dialog.is_none() {
            return;
        }

        let view = self.ui.shrink_dialog_view(id!(shrink_dialog.dlg));
        match view.action(actions) {
            ShrinkDialogAction::None => {}
            ShrinkDialogAction::Toggled(indices) => {
                if let Some(dialog) = &mut self.shrink_dialog {
                    dialog.set_checked_from_indices(&indices);
                    let preview = shrink_preview(&dialog.manifest, &dialog.checked);
                    view.set_preview(cx, &preview);
                }
            }
            ShrinkDialogAction::Cancel => {
                self.close_shrink(cx);
            }
            ShrinkDialogAction::Apply(indices) => {
                if let Some(dialog) = &mut self.shrink_dialog {
                    dialog.set_checked_from_indices(&indices);
                }
                self.apply_shrink_action(cx, false);
            }
            ShrinkDialogAction::ApplyAndBuild(indices) => {
                if let Some(dialog) = &mut self.shrink_dialog {
                    dialog.set_checked_from_indices(&indices);
                }
                self.apply_shrink_action(cx, true);
            }
        }
    }

    /// Applies the selected shrink options to Cargo.toml and optionally triggers a Release build.
    fn apply_shrink_action(&mut self, cx: &mut Cx, and_build: bool) {
        let Some(dialog) = &self.shrink_dialog else {
            return;
        };

        let preview = shrink_preview(&dialog.manifest, &dialog.checked);
        if preview.is_empty() {
            return;
        }

        match apply_shrink(&dialog.manifest, &dialog.checked) {
            Ok(new_manifest) => {
                if let Err(err) = std::fs::write(&dialog.manifest_path, &new_manifest) {
                    self.status = Some(format!(
                        "Shrink tips: cannot write {}: {err}",
                        dialog.manifest_path.display()
                    ));
                    return;
                }

                let status_msg = shrink_status(&preview);
                self.close_shrink(cx);
                self.status = Some(status_msg);

                if and_build {
                    self.build.profile = crawrad_project::build_settings::BuildProfile::Release;
                    self.execute_build_action(
                        cx,
                        crawrad_project::build_settings::CargoCommand::Build,
                    );
                }
            }
            Err(err) => {
                self.status = Some(format!("Shrink tips: {err}"));
            }
        }
    }
}
