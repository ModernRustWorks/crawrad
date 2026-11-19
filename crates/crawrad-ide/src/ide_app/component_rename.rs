//! Component Rename Everywhere confirm card.
//!
//! Confirms renaming a component, its default-named event handlers, and
//! its references across the code via rust-analyzer.

use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use crawrad_host::rename::{ComponentRename, FileChange, Refs};
use crawrad_ir::Value;
use crawrad_note::note;
use makepad_widgets::*;

use super::IdeApp;
use super::rename::change_of;
use crawrad_host::rename::find_field_caret;

static COMPONENT_RENAME_TICKET: AtomicU64 = AtomicU64::new(10_000_000);

impl IdeApp {
    #[note("ide-app-comp-rename-start")]
    pub(super) fn start_component_rename(
        &mut self,
        cx: &mut Cx,
        node_id: String,
        new_name: String,
    ) {
        let old = match self.host.doc().and_then(|d| d.ir().node(&node_id)) {
            Some(n) => n.name().to_string(),
            None => return,
        };
        if old == new_name {
            return;
        }

        let handlers = self
            .host
            .doc()
            .and_then(|d| d.ir().node(&node_id))
            .map(|n| crawrad_host::rename::find_default_handlers(&n.events, &old, &new_name))
            .unwrap_or_default();

        let active = self.host.active_file();
        let form_stem = active
            .as_ref()
            .and_then(|p| p.file_stem().and_then(|s| s.to_str()))
            .unwrap_or_else(|| {
                self.host
                    .doc()
                    .map(|d| d.ir().root.as_str())
                    .unwrap_or("form_main")
            });
        let form_designer_file = active
            .as_ref()
            .and_then(|p| {
                p.parent()
                    .map(|dir| dir.join(format!("{form_stem}{}", crawrad_codegen::SUFIXO_GERADO)))
            })
            .unwrap_or_else(|| {
                let root = self.host.workspace_root().unwrap_or_default();
                root.join("src")
                    .join(format!("{form_stem}{}", crawrad_codegen::SUFIXO_GERADO))
            });
        let form_file_display = form_designer_file
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| format!("{form_stem}{}", crawrad_codegen::SUFIXO_GERADO));

        let mut cr = ComponentRename::new(
            node_id,
            old,
            new_name,
            form_file_display,
            handlers,
            Refs::Loading,
        );
        cr.designer = form_designer_file;
        self.component_rename = Some(cr);
        if !self.ask_component_refs() {
            if let Some(cr) = self.component_rename.as_mut() {
                cr.refs = Refs::Unavailable;
            }
            if !self
                .component_rename
                .as_ref()
                .is_some_and(ComponentRename::needs_card)
            {
                return self.execute_component_rename_all(cx);
            }
        }
        self.draw_component_rename(cx);
    }

    /// Asks rust-analyzer to rename the component's field in the generated
    /// file; false when it cannot be asked (no server, no field).
    #[note("ide-app-comp-rename-ask")]
    fn ask_component_refs(&mut self) -> bool {
        // The references need rust-analyzer: a form open in the Designer may
        // not have started it yet.
        self.garantir_lsp();
        let (Some(lsp), Some(cr)) = (self.lsp.clone(), self.component_rename.as_ref()) else {
            return false;
        };
        let (file, old, new) = (cr.designer.clone(), cr.old.clone(), cr.new.clone());
        let Some(text) = self.text_of(&file) else {
            return false;
        };
        let Some(caret) = find_field_caret(&text, &old) else {
            return false;
        };
        // rust-analyzer only renames from a file it holds: the generated one
        // is never open in an editor tab.
        let _ = lsp.did_open(&file, "rust", self.lsp_versao, &text);
        let ticket = COMPONENT_RENAME_TICKET.fetch_add(1, Ordering::Relaxed);
        self.component_rename_ticket = Some(ticket);
        let asked = crawrad_lsp::rename::Asked {
            file: &file,
            version: self.lsp_versao,
            text: &text,
            caret,
        };
        self.lsp_hints.rename.rename(lsp, ticket, asked, &new);
        true
    }

    #[note("ide-app-comp-rename-poll")]
    pub(super) fn poll_component_rename(&mut self, cx: &mut Cx) {
        let Some(ticket) = self.component_rename_ticket else {
            let due = self
                .component_rename
                .as_ref()
                .and_then(|cr| cr.retry_at)
                .map(|at| std::time::Instant::now() >= at);
            match due {
                Some(true) => {
                    if let Some(cr) = self.component_rename.as_mut() {
                        cr.retry_at = None;
                    }
                    let _ = self.ask_component_refs();
                }
                Some(false) => {
                    cx.new_next_frame();
                }
                None => {}
            }
            return;
        };
        let Some(answer) = self.lsp_hints.rename.take(ticket) else {
            return;
        };
        self.component_rename_ticket = None;

        let mut changes: Vec<FileChange> = Vec::new();
        let answered = matches!(answer.outcome, crawrad_lsp::rename::Outcome::Edits(_));
        if let crawrad_lsp::rename::Outcome::Edits(mut edits) = answer.outcome {
            edits.retain(|uri, _| {
                if let Some(path) = crawrad_lsp::uri_to_path(uri) {
                    !path
                        .to_string_lossy()
                        .ends_with(crawrad_codegen::SUFIXO_GERADO)
                } else {
                    true
                }
            });
            let enc = self.lsp_encoding();
            let mut read = |f: &Path| self.text_of(f);
            if let Ok(planned) = crawrad_lsp::rename::plan(&edits, enc, &mut read) {
                changes = planned.into_iter().map(change_of).collect();
            }
        }

        if let Some(ref mut cr) = self.component_rename {
            let total: usize = changes.iter().map(|f| f.count).sum();
            cr.rename_refs = total > 0;
            // No answer (still indexing, or it refused) is "not ready", never
            // "0 places": the card says so and leaves the box off.
            cr.refs = if answered {
                Refs::Found(changes)
            } else {
                Refs::Unavailable
            };

            // rust-analyzer still loading: ask again in 2 s while the card is up.
            if !answered && cr.needs_card() {
                cr.retry_at = Some(std::time::Instant::now() + std::time::Duration::from_secs(2));
                cx.new_next_frame();
                return self.draw_component_rename(cx);
            }
            if !cr.needs_card() {
                let node = cr.node.clone();
                let new_name = cr.new.clone();
                self.component_rename = None;
                let args =
                    crawrad_actions::Args::text("node", node).and("name", Value::Str(new_name));
                self.execute_with(cx, "component.rename", args);
                self.draw_component_rename(cx);
                return;
            }
            self.draw_component_rename(cx);
        }
    }

    #[note("ide-app-comp-rename-draw")]
    pub(super) fn draw_component_rename(&mut self, cx: &mut Cx) {
        let dialog = self.ui.widget(id!(component_rename_dialog));
        let Some(ref cr) = self.component_rename else {
            dialog.set_visible(cx, false);
            return;
        };
        if !cr.needs_card() {
            dialog.set_visible(cx, false);
            return;
        }
        dialog.set_visible(cx, true);

        let dlg = self.ui.view(id!(component_rename_dialog.dlg));
        dlg.label(id!(comp_rename_title))
            .set_text(cx, &format!("Rename {} \u{2192} {}", cr.old, cr.new));

        if let Some(action) = crawrad_actions::by_id("component.rename") {
            dlg.button(id!(comp_rename_all)).set_text(cx, action.label);
        }
        // Row 1: fixed, always checked, greyed box
        dlg.check_box(id!(check_comp)).set_active(cx, true);
        dlg.label(id!(detail_comp)).set_text(
            cx,
            &format!(
                "{} \u{00b7} field {} \u{2192} {}",
                cr.form_file, cr.old, cr.new
            ),
        );

        // Row 2: Default-named handlers (hidden if none)
        let has_handlers = !cr.handlers.is_empty();
        dlg.view(id!(row_handlers)).set_visible(cx, has_handlers);
        if has_handlers {
            dlg.check_box(id!(check_handlers))
                .set_active(cx, cr.rename_handlers);
            dlg.label(id!(detail_handlers))
                .set_text(cx, &cr.handler_line());
        }

        // Row 3: References
        let refs_active = match &cr.refs {
            Refs::Found(files) => cr.rename_refs && files.iter().any(|f| f.count > 0),
            Refs::Loading | Refs::Unavailable => false,
        };
        dlg.check_box(id!(check_refs)).set_active(cx, refs_active);
        dlg.label(id!(detail_refs)).set_text(cx, &cr.refs_line());

        self.ui.redraw(cx);
    }

    #[note("ide-app-comp-rename-atender")]
    pub(super) fn atender_component_rename(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.component_rename.is_none() {
            return;
        }
        let dlg = self.ui.view(id!(component_rename_dialog.dlg));
        if dlg.button(id!(comp_rename_all)).clicked(actions) {
            return self.execute_component_rename_all(cx);
        }
        if dlg.button(id!(comp_rename_cancel)).clicked(actions) {
            return self.cancel_component_rename(cx);
        }
        let handlers = dlg.check_box(id!(check_handlers)).changed(actions);
        let refs = dlg.check_box(id!(check_refs)).changed(actions);
        if let Some(cr) = self.component_rename.as_mut() {
            if let Some(on) = handlers {
                cr.rename_handlers = on;
            }
            // Only a found answer can be renamed; the draw puts the box back off.
            if let Some(on) = refs {
                cr.rename_refs = on && matches!(cr.refs, Refs::Found(_));
            }
        }
        if handlers.is_some() || refs.is_some() {
            self.draw_component_rename(cx);
        }
    }

    #[note("ide-app-comp-rename-execute-all")]
    pub(super) fn execute_component_rename_all(&mut self, cx: &mut Cx) {
        let Some(cr) = self.component_rename.take() else {
            return;
        };
        self.component_rename_ticket = None;
        self.draw_component_rename(cx);

        let everywhere = cr.everywhere();
        let node = cr.node.clone();
        let old = cr.old.clone();
        let new_name = cr.new.clone();
        let edits = cr.edits_to_apply().to_vec();

        let args = crawrad_actions::Args::text("node", node)
            .and("name", Value::Str(new_name.clone()))
            .and("everywhere", Value::Bool(everywhere));
        self.execute_with(cx, "component.rename", args);

        if !edits.is_empty() {
            self.apply_file_changes(cx, &edits);
        }

        let n_edits: usize = edits.iter().map(|f| f.count).sum();
        let m_files = edits.len();
        self.status = Some(format!(
            "Renamed {old} \u{2192} {new_name} \u{00b7} {n_edits} edit(s) in {m_files} file(s), not saved"
        ));
    }

    /// Cancel (or Esc): nothing is renamed, and the Inspector shows the old
    /// Name again.
    #[note("ide-app-comp-rename-cancel")]
    pub(super) fn cancel_component_rename(&mut self, cx: &mut Cx) {
        if self.component_rename.take().is_none() {
            return;
        }
        self.component_rename_ticket = None;
        self.draw_component_rename(cx);
        self.redraw_panel_list(
            cx,
            crawrad_actions::Panel::Inspector,
            id!(inspector_panel.inspector_list),
        );
        self.status = Some("Rename cancelled \u{2014} nothing changed".into());
    }
}
