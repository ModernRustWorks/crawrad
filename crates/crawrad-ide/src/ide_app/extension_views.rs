//! Extension views: the glue between the window and what is decided
//! in `crawrad_extension_host::views` — the store the extensions fill, the
//! panel's rows, filter and clicks. This file follows the store, draws a row,
//! and turns a click into a place and a message to the extension.

use crawrad_actions::Panel;
use crawrad_extension_host::views::{Installed, ViewRow};
use crawrad_host::navigation::Place;
use makepad_code_editor::line_mark::kind_rgb;
use makepad_widgets::*;

use super::{IdeApp, icon, layout, xywh};
use crawrad_note::note;

impl IdeApp {
    #[note("ide-app-extension-views-ideapp-follow-extension-views")]
    pub(crate) fn follow_extension_views(&mut self, cx: &mut Cx) {
        let Some(store) = self.extensions.views() else {
            return;
        };
        let Ok(mut store) = store.lock() else {
            return;
        };
        store.set_waker(SignalToUI::set_ui_signal);
        let views = self.ext_views.follow(&store);
        drop(store);
        let manifests = self.extensions.manifests.iter();
        let now = manifests.map(|m| Installed {
            id: m.id.clone(),
            name: m.name.clone(),
            version: m.version.clone(),
        });
        if self.ext_views.set_installed(now.collect()) | views {
            self.write_extension_views(cx);
        }
    }

    #[note("ide-app-extension-views-ideapp-extension-tool")]
    pub(super) fn extension_tool(
        &mut self,
        cx: &mut Cx,
        name: &str,
        args: &serde_json::Value,
    ) -> Result<String, String> {
        let key = if name.contains("uninstall") {
            "id"
        } else {
            "path"
        };
        let Some(arg) = args.get(key).and_then(|v| v.as_str()) else {
            return Err(format!("'{key}' is required"));
        };
        match key {
            "id" => self.uninstall_extension(cx, arg),
            _ => self.install_extension(cx, std::path::Path::new(arg)),
        }
    }

    #[note("ide-app-extension-views-ideapp-install-extension")]
    pub(super) fn install_extension(
        &mut self,
        cx: &mut Cx,
        folder: &std::path::Path,
    ) -> Result<String, String> {
        let root = self
            .raiz()
            .ok_or("Open a project to install an extension into")?;
        let done = match self.extensions.install(&root, folder) {
            Ok(m) => {
                let _ = self.capabilities.register_extension_manifest(&m);
                Ok(format!("Installed {} {}", m.name, m.version))
            }
            Err(e) => Err(format!("Could not install the extension: {e}")),
        };
        self.status = Some(done.clone().unwrap_or_else(|e| e));
        self.follow_extension_views(cx);
        done
    }

    #[note("ide-app-extension-views-ideapp-uninstall-extension")]
    pub(super) fn uninstall_extension(&mut self, cx: &mut Cx, id: &str) -> Result<String, String> {
        let root = self.raiz().ok_or("No project is open")?;
        self.capabilities.unregister_extension(id);
        let done = match self.extensions.uninstall(&root, id) {
            Ok(()) => Ok(format!("Uninstalled {id}")),
            Err(e) => Err(format!("Could not uninstall {id}: {e}")),
        };
        self.status = Some(done.clone().unwrap_or_else(|e| e));
        self.follow_extension_views(cx);
        done
    }

    #[note("ide-app-extension-views-ideapp-write-extension-views")]
    pub(super) fn write_extension_views(&mut self, cx: &mut Cx) {
        let panel = self.dock().item(layout::tab_of(Panel::Extensions));
        panel
            .button(id!(extensions_header.btn_install))
            .set_text(cx, icon::Chrome::New.glyph());
        self.redraw_panel_list(cx, Panel::Extensions, id!(extensions_body.views_list));
    }

    #[note("ide-app-extension-views-ideapp-write-view-row")]
    pub(super) fn write_view_row(cx: &mut Cx, item: &WidgetRef, row: &ViewRow) {
        let chevron = |open: bool| match open {
            true => icon::Chrome::ChevronDown.glyph(),
            false => icon::Chrome::ChevronRight.glyph(),
        };
        match row {
            ViewRow::Header {
                title, count, open, ..
            } => {
                item.label(id!(face.chevron)).set_text(cx, chevron(*open));
                item.label(id!(face.title)).set_text(cx, title);
                item.label(id!(face.count)).set_text(cx, &count.to_string());
            }
            ViewRow::Section { title, count } => {
                item.label(id!(face.chevron)).set_text(cx, "");
                item.label(id!(face.title)).set_text(cx, title);
                let n = count.map_or_else(String::new, |c| c.to_string());
                item.label(id!(face.count)).set_text(cx, &n);
            }
            ViewRow::Installed { name, version, .. } => {
                item.label(id!(face.name)).set_text(cx, name);
                item.label(id!(face.version)).set_text(cx, version);
                let trash = icon::Chrome::ClearChat.glyph();
                item.button(id!(face.uninstall)).set_text(cx, trash);
            }
            ViewRow::Item {
                depth,
                label,
                detail,
                kind,
                branch,
                ..
            } => {
                let indent = 8.0 + 14.0 * *depth as f64;
                item.view(id!(face))
                    .apply_over(cx, live! { padding: { left: (indent) } });
                let glyph = branch.map(chevron).unwrap_or("");
                item.label(id!(face.chevron)).set_text(cx, glyph);
                let (r, g, b) = kind_rgb(kind);
                let a = if kind.is_empty() { 0.0 } else { 1.0 };
                let color = vec4(r, g, b, a);
                item.view(id!(face.dot))
                    .apply_over(cx, live! { draw_bg: { color: (color) } });
                item.label(id!(face.label)).set_text(cx, label);
                item.label(id!(face.detail)).set_text(cx, detail);
            }
        }
    }

    #[note("ide-app-extension-views-ideapp-extension-views-actions")]
    pub(super) fn extension_views_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let panel = self.dock().item(layout::tab_of(Panel::Extensions));
        let filter = panel.text_input(id!(extensions_filter.views_filter));
        if let Some(text) = filter.changed(actions) {
            self.ext_views.filter = text;
            self.write_extension_views(cx);
        }
        if panel.button(id!(btn_install)).clicked(actions)
            && let Some(folder) = rfd::FileDialog::new().pick_folder()
        {
            let _ = self.install_extension(cx, &folder);
        }
        let list = panel.portal_list(id!(extensions_body.views_list));
        if !list.any_items_with_actions(actions) {
            return;
        }
        let items = list.items_with_actions(actions);
        let trash = items
            .iter()
            .find(|(_, i)| i.button(id!(face.uninstall)).clicked(actions));
        if let Some(id) = trash.and_then(|(row, _)| self.ext_views.uninstall_at(*row)) {
            let _ = self.uninstall_extension(cx, &id);
            return;
        }
        let clicked = list
            .items_with_actions(actions)
            .into_iter()
            .find(|(_, item)| item.button(id!(target)).clicked(actions))
            .map(|(i, _)| i);
        let Some(row) = clicked else {
            return;
        };
        let pick = self.ext_views.click(row);
        self.write_extension_views(cx);
        let Some(pick) = pick else {
            return;
        };
        let _ = self
            .extensions
            .view_item_clicked(&pick.extension, &pick.view, &pick.item);
        if let Some(at) = pick.location {
            // a relative place is the project's, as the extension sees it
            let file = self.raiz_do_projeto().join(&at.file);
            let place = Place {
                file,
                line: at.line as usize,
                byte: 0,
            };
            self.go_to_place(cx, place);
        }
    }

    #[note("ide-app-extension-views-ideapp-extension-views-json")]
    pub(super) fn extension_views_json(&self, cx: &Cx) -> serde_json::Value {
        let views = self.ext_views.views.iter().map(|v| {
            serde_json::json!({
                "extension": v.extension, "id": v.id, "title": v.title,
                "items": serde_json::to_value(&v.items).unwrap_or_default(),
            })
        });
        let shown = self.ext_views.shown();
        let rows = self.ext_views.rects.iter().filter_map(|(i, r)| {
            let (kind, text, depth) = match shown.get(*i)? {
                ViewRow::Header { title, .. } => ("header", title.clone(), 0),
                ViewRow::Item { label, depth, .. } => ("item", label.clone(), *depth),
                ViewRow::Section { title, .. } => ("section", title.clone(), 0),
                ViewRow::Installed { id, .. } => ("installed", id.clone(), 0),
            };
            let button = self.ext_views.buttons.iter().find(|(b, _)| b == i);
            let button = button.map(|(_, b)| b);
            Some(serde_json::json!({ "kind": kind, "text": text, "depth": depth, "rect": r, "button": button }))
        });
        let panel = self.dock().item(layout::tab_of(Panel::Extensions));
        let filter = id!(extensions_filter.views_filter);
        let r = panel.text_input(filter).area().rect(cx);
        serde_json::json!({
            "views": views.collect::<Vec<_>>(),
            "filter": self.ext_views.filter,
            "filter_rect": xywh(r),
            "rows": rows.collect::<Vec<_>>(),
        })
    }
}
