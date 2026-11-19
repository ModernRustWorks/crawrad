//! The Sizes preview glue: model in `crawrad_designer::size_preview`,
//! drawing in `size_preview_view` and `DesignCanvas`; here only the wiring.

use std::path::Path;

use crawrad_actions::Panel;
use crawrad_runtime_makepad::size_preview_view::{
    SizeFrame, SizeItem, SizePreviewAction, SizePreviewViewRef, SizePreviewViewWidgetRefExt,
};
use makepad_widgets::*;

use super::{Center, IdeApp};

impl IdeApp {
    fn sizes_panel(&self) -> SizePreviewViewRef {
        self.painel(Panel::Design)
            .size_preview_view(id!(designer_row.sizes_container.sizes_panel))
    }

    /// Lays the form out at every shown size and hands the frames to the view.
    pub(super) fn paint_sizes(&mut self, cx: &mut Cx) {
        let doc = self.host.doc();
        let mut rev = doc.map_or(0, |d| d.revision());
        if let Some(ref loc) = self.design_locale {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            loc.code.hash(&mut hasher);
            rev ^= hasher.finish();
        }
        self.size_preview_rev = rev;
        let reg = self.designer.registry.clone().unwrap_or_default();
        let plans = doc.map_or_else(Vec::new, |d| match &self.design_locale {
            Some(loc) => {
                let ir = crawrad_designer::translations::localized(d.ir(), &d.ir().root, &reg, loc);
                self.size_preview.plans(&ir, &reg)
            }
            None => self.size_preview.plans(d.ir(), &reg),
        });
        let to_item = |p: &crawrad_designer::size_preview::SizePreset, s: bool, c: bool| SizeItem {
            name: p.name.clone(),
            width: p.width,
            height: p.height,
            shown: s,
            custom: c,
        };
        let items: Vec<SizeItem> = self
            .size_preview
            .sizes
            .iter()
            .map(|(p, s)| to_item(p, *s, false))
            .chain(
                self.size_preview
                    .custom
                    .iter()
                    .map(|(p, s)| to_item(p, *s, true)),
            )
            .collect();
        let frames: Vec<SizeFrame> = (plans.iter())
            .map(|p| SizeFrame {
                name: &p.preset.name,
                width: p.preset.width,
                height: p.preset.height,
                doc: &p.framed,
                note: &p.note,
                warning: p.warning,
                overflow: p.overflows.iter().map(|o| o.node.clone()).collect(),
            })
            .collect();
        let declared = crawrad_project::installed::declared_drawings();
        (self.sizes_panel()).set_view(cx, &items, &frames, &declared);
    }

    /// Dispatches user gestures from the Sizes tab.
    pub(super) fn sizes_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let panel = self.sizes_panel();
        let Some(action) = panel.action(cx, actions) else {
            return;
        };

        match action {
            SizePreviewAction::Toggle(i) => {
                self.size_preview.toggle(i);
                self.paint_sizes(cx);
            }
            SizePreviewAction::ToggleCustomCard => {
                panel.toggle_custom_card(cx);
            }
            SizePreviewAction::CloseCustomCard => {
                panel.set_custom_card_visible(cx, false);
            }
            SizePreviewAction::AddCustom {
                name,
                width,
                height,
            } => match self.size_preview.add_custom(&name, width, height) {
                Ok(()) => {
                    panel.set_custom_card_visible(cx, false);
                    self.save_custom_sizes();
                    self.paint_sizes(cx);
                }
                Err(err) => {
                    panel.set_custom_error(cx, &err);
                }
            },
            SizePreviewAction::None => {}
            SizePreviewAction::Fit => {
                self.paint_sizes(cx);
            }
        }
    }

    /// Follows document edits while the Sizes tab is in front.
    pub(super) fn check_sizes(&mut self, cx: &mut Cx) {
        if self.host.center() != Center::Sizes {
            return;
        }
        let mut rev = self.host.doc().map_or(0, |d| d.revision());
        if let Some(ref lang) = self.design_language {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            lang.hash(&mut hasher);
            rev ^= hasher.finish();
        }
        if rev != self.size_preview_rev {
            self.paint_sizes(cx);
        }
    }

    /// Reads custom sizes from the project's Cargo.toml into the size preview state.
    pub(super) fn read_project_sizes(&mut self, root: &Path) {
        self.size_preview
            .set_custom(crawrad_project::project_options::read_sizes(root));
    }

    /// Saves the current custom sizes into the project's Cargo.toml.
    pub(super) fn save_custom_sizes(&self) {
        let Some(root) = self.desktop_root.clone().or_else(|| self.raiz()) else {
            return;
        };
        let _ = crawrad_project::project_options::write_sizes(
            &root,
            &self.size_preview.custom_tuples(),
        );
    }

    /// Returns JSON data for the `view.sizes` MCP tool.
    pub(crate) fn sizes_json(&self) -> serde_json::Value {
        let Some(doc) = self.host.doc() else {
            return serde_json::json!({ "sizes": [] });
        };
        let reg = self.designer.registry.clone().unwrap_or_default();
        let doc_localized;
        let ir = if let Some(ref loc) = self.design_locale {
            doc_localized =
                crawrad_designer::translations::localized(doc.ir(), &doc.ir().root, &reg, loc);
            &doc_localized
        } else {
            doc.ir()
        };
        self.size_preview.report(ir, &reg)
    }
}
