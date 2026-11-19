//! The History tab: the copies of the file in the centre against its buffer.

use std::path::Path;

use crawrad_host::history_view::RowKind;
use crawrad_runtime_makepad::{
    CrawradHistoryPanelRef, CrawradHistoryPanelWidgetRefExt, DiffRowKind, DiffViewRow,
    HistoryEntryView, HistoryMode, HistoryPanelAction as Click,
};

use super::*;
use crawrad_note::note;

impl IdeApp {
    fn history_panel(&self) -> CrawradHistoryPanelRef {
        self.painel(Panel::Design)
            .crawrad_history_panel(id!(designer_row.history_container.history_panel))
    }

    #[note("ide-app-history-ideapp-history-buffer")]
    fn history_buffer(&self) -> String {
        let file = &self.history.file;
        let disk =
            || std::fs::read_to_string(file).map_or(String::new(), |t| t.replace("\r\n", "\n"));
        self.host.buffer(file).map_or_else(disk, str::to_string)
    }

    #[note("ide-app-history-ideapp-history-file")]
    pub(super) fn history_file(&self) -> Option<PathBuf> {
        let in_code = self.host.center() == Center::Code;
        (self
            .abas
            .active_file()
            .filter(|_| in_code)
            .map(Path::to_path_buf))
        .or_else(|| self.host.active_file().map(source::unit_path))
    }

    #[note("ide-app-history-ideapp-timeline-lines")]
    pub(super) fn timeline_lines(&self) -> Vec<String> {
        let (Some(file), Some(history)) = (self.history_file(), self.disk.history()) else {
            return Vec::new();
        };
        let (now, offset) = (
            crawrad_host::history::now(),
            crawrad_os::clock::local_offset_seconds(),
        );
        crawrad_host::history_view::timeline_lines(&history.entries(&file), now, offset)
    }

    #[note("ide-app-history-ideapp-open-history")]
    pub(super) fn open_history(&mut self, cx: &mut Cx) {
        let Some(file) = self.history_file() else {
            return;
        };
        let (now, offset) = (
            crawrad_host::history::now(),
            crawrad_os::clock::local_offset_seconds(),
        );
        let history = self.disk.history();
        self.history =
            crawrad_host::history_view::HistorySession::open(history.as_ref(), file, now, offset);
        self.show_center(cx, Center::History);
        self.history_panel().set_mode(cx, HistoryMode::Differences);
        self.sync_history(cx);
    }

    pub(super) fn sync_history(&mut self, cx: &mut Cx) {
        let (panel, buffer) = (self.history_panel(), self.history_buffer());
        let root = self.host.workspace_root().unwrap_or_default();
        let name = self
            .history
            .file
            .strip_prefix(&root)
            .unwrap_or(&self.history.file);
        panel.set_title(cx, &format!("History · {}", name.display()));
        let entry = |r: &crawrad_host::history_view::EntryRow| HistoryEntryView {
            time: r.time.clone(),
            when: r.when.clone(),
            origin: r.origin.to_string(),
            size: r.size.clone(),
            lines: r.lines,
        };
        panel.set_entries(cx, &self.history.rows.iter().map(entry).collect::<Vec<_>>());
        panel.set_selected(cx, self.history.selected);
        panel.set_contents(cx, self.history.copy());
        let view = self.history.diff(&buffer);
        fn row(r: &crawrad_host::history_view::DiffRow) -> DiffViewRow<'_> {
            let kind = match r.kind {
                RowKind::Same => DiffRowKind::Same,
                RowKind::Removed => DiffRowKind::Removed,
                RowKind::Added => DiffRowKind::Added,
                RowKind::Changed => DiffRowKind::Changed,
            };
            DiffViewRow {
                kind,
                left: r.left.as_ref().map(|(n, t)| (*n, t.as_str())),
                right: r.right.as_ref().map(|(n, t)| (*n, t.as_str())),
            }
        }
        let rows: Vec<_> = view.rows.iter().map(row).collect();
        panel.set_diff(cx, &rows, view.blocks.len(), self.history.block);
    }

    #[note("ide-app-history-ideapp-handle-history-actions")]
    pub(super) fn handle_history_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let panel = self.history_panel();
        let Some(click) = panel.clicked(actions) else {
            return;
        };
        let buffer = self.history_buffer();
        let new_text = match click {
            Click::Select(row) => {
                self.history.select(row);
                None
            }
            Click::Mode(mode) => {
                panel.set_mode(cx, mode);
                None
            }
            Click::Compare => {
                panel.set_mode(cx, HistoryMode::Differences);
                None
            }
            Click::Previous | Click::Next => {
                if let Some(row) = self.history.step(&buffer, click == Click::Next) {
                    panel.reveal_row(cx, row);
                }
                None
            }
            Click::RestoreBlock => self.history.restored(&buffer),
            Click::Revert => Some(self.history.copy().to_string()),
        };
        // a restore or a revert is the buffer's new text, one undo step back to the old
        if let Some(text) = new_text.filter(|t| *t != buffer) {
            let file = self.history.file.clone();
            self.editor_undo_stacks
                .entry(file.clone())
                .or_default()
                .push(buffer);
            self.set_editor_buffer(file, text);
            self.redraw_editor_tabs(cx);
        }
        self.sync_history(cx);
    }

    #[note("ide-app-history-ideapp-history-tab-json")]
    pub(super) fn history_tab_json(&self, cx: &Cx) -> serde_json::Value {
        let mut out = self.history.json(&self.history_buffer());
        let tab = self
            .painel(Panel::Design)
            .widget(id!(center_tabs_bar.tab_history));
        let drawn = self.history_panel().drawn_controls(cx).into_iter();
        let rect = |(name, r): (String, Rect)| serde_json::json!({ "name": name, "rect": xywh(r) });
        let controls: Vec<_> = drawn
            .chain([("tab_history".into(), tab.area().rect(cx))])
            .map(rect)
            .collect();
        out["open"] = (self.host.center() == Center::History).into();
        out["mode"] = self.history_panel().mode().as_str().into();
        out["controls"] = controls.into();
        out
    }
}
