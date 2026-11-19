//! Unit tests for pure document and status bar helpers.

use super::{Counters, StatusBarInputs, active_node_name, status_bar_cells};

#[test]
fn active_node_name_none_is_dash() {
    assert_eq!(active_node_name(None), "—");
}

#[test]
fn status_bar_cells_structure() {
    let cells = status_bar_cells(StatusBarInputs {
        errors: "0".into(),
        warnings: "1".into(),
        lsp_name: "rust-analyzer".into(),
        lsp_progress: "".into(),
        target: "makepad".into(),
        project: "Project: my_app".into(),
        message: "ready".into(),
        counters: Counters {
            selection: "no selection".into(),
            nodes: "5".into(),
            revision: "2".into(),
        },
        zoom_cell: "100%".into(),
        framework: "Makepad",
    });
    assert_eq!(cells.len(), 12);
    assert_eq!(cells[0], "0");
    assert_eq!(cells[1], "1");
    assert_eq!(cells[2], "rust-analyzer");
    assert_eq!(cells[3], "");
    assert_eq!(cells[4], "makepad");
    assert_eq!(cells[5], "Project: my_app");
    assert_eq!(cells[6], "ready");
    assert_eq!(cells[7], "no selection");
    assert_eq!(cells[8], "5 nodes");
    assert_eq!(cells[9], "rev 2");
    assert_eq!(cells[10], "100%");
    assert_eq!(cells[11], "Makepad");
}
