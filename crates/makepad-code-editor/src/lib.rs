#![forbid(unsafe_code)]

pub use makepad_widgets;
use makepad_widgets::*;

pub mod action_bulb_view;
pub mod char;
pub mod code_editor;
pub mod code_view;
pub mod current_line;
pub mod decoration;
pub mod document;
pub mod environment_view;
pub mod form_check_view;
pub mod gutter;
pub mod gutter_mark;
pub mod history;
pub mod inlays;
pub mod iter;
pub mod layout;
pub mod line_change;
pub mod line_mark;
pub mod minimap;
pub mod minimap_model;
pub mod modal_veil;
pub mod note_anchor;
pub mod note_icon;
pub mod note_popup;
pub mod notes_panel_view;
pub mod release_size_view;
pub mod review;
pub mod scope_guides;
pub mod selection;
pub mod semantic;
pub mod session;
pub mod settings;
pub mod snippet;
pub mod str;
pub mod tests_panel_view;
pub mod text;
pub mod token;
pub mod tokenizer;
pub mod widgets;
pub mod wrap;

pub use self::{
    action_bulb_view::{ActionBulb, ActionBulbAction, ActionBulbRef, ActionBulbWidgetRefExt},
    code_editor::{CodeEditor, CodeEditorAction, LandingIndicator},
    current_line::{highlighted_lines, highlighted_lines_from_selections},
    document::CodeDocument,
    environment_view::{
        EnvironmentCheckView, EnvironmentCheckViewRef, EnvironmentCheckViewWidgetRefExt,
        EnvironmentHeadParts, EnvironmentRowParts, EnvironmentViewAction,
    },
    gutter::{calculate_gutter_width, draw_gutter_layer},
    history::History,
    layout::Line,
    line_change::{ChangeTracker, LineChange, line_changes},
    note_icon::{NoteKindIcon, NoteKindIconRef, NoteKindIconWidgetRefExt},
    notes_panel_view::{
        NotesHeadParts, NotesPanelAction, NotesPanelView, NotesPanelViewRef,
        NotesPanelViewWidgetRefExt, NotesRowParts,
    },
    release_size_view::{
        ReleaseSizeAction, ReleaseSizeView, ReleaseSizeViewRef, ReleaseSizeViewWidgetRefExt,
        ShrinkDialogAction, ShrinkDialogView, ShrinkDialogViewRef, ShrinkDialogViewWidgetRefExt,
        SizePageParts,
    },
    scope_guides::{ScopeBlock, compute_scope_blocks, draw_indent_guides, draw_scope_header},
    selection::Selection,
    session::CodeSession,
    settings::Settings,
    token::Token,
    tokenizer::{Tokenizer, is_keyword},
};

pub fn live_design(cx: &mut Cx) {
    crate::action_bulb_view::live_design(cx);
    crate::note_icon::live_design(cx);
    crate::notes_panel_view::live_design(cx);
    crate::tests_panel_view::live_design(cx);
    crate::environment_view::live_design(cx);
    crate::form_check_view::live_design(cx);
    crate::release_size_view::live_design(cx);
    crate::code_editor::live_design(cx);
    crate::code_view::live_design(cx);
}

#[cfg(test)]
mod scope_tests {
    use super::*;

    #[test]
    fn test_scope_blocks_nested() {
        let code = vec![
            "fn handle_event() {".to_string(),                     // 0 (col 0)
            "    if let Event::Draw = event {".to_string(),        // 1 (col 4)
            "        while let Some(item) = next {".to_string(),   // 2 (col 8)
            "            for i in 0..10 {".to_string(),            // 3 (col 12)
            "                // Comment with { brace".to_string(), // 4 (ignored)
            "                let s = \"string with { brace }\";".to_string(), // 5 (ignored)
            "            }".to_string(),                           // 6 (close 3)
            "        }".to_string(),                               // 7 (close 2)
            "    }".to_string(),                                   // 8 (close 1)
            "}".to_string(),                                       // 9 (close 0)
        ];

        let blocks = compute_scope_blocks(&code, 4);
        assert_eq!(blocks.len(), 4);

        // Innermost block (for loop: lines 3..6, col 12, depth 3)
        assert_eq!(blocks[0].start_line, 3);
        assert_eq!(blocks[0].end_line, 6);
        assert_eq!(blocks[0].indent_col, 12);
        assert_eq!(blocks[0].depth, 3);

        // while let block: lines 2..7, col 8, depth 2
        assert_eq!(blocks[1].start_line, 2);
        assert_eq!(blocks[1].end_line, 7);
        assert_eq!(blocks[1].indent_col, 8);
        assert_eq!(blocks[1].depth, 2);

        // if let block: lines 1..8, col 4, depth 1
        assert_eq!(blocks[2].start_line, 1);
        assert_eq!(blocks[2].end_line, 8);
        assert_eq!(blocks[2].indent_col, 4);
        assert_eq!(blocks[2].depth, 1);

        // fn block: lines 0..9, col 0, depth 0
        assert_eq!(blocks[3].start_line, 0);
        assert_eq!(blocks[3].end_line, 9);
        assert_eq!(blocks[3].indent_col, 0);
        assert_eq!(blocks[3].depth, 0);

        // Where each opening brace sits: the header underline of the active
        // scope ends there.
        let open_cols: Vec<usize> = blocks.iter().map(|b| b.open_col).collect();
        assert_eq!(open_cols, [27, 36, 31, 18]);
    }
}

#[cfg(test)]
#[path = "test/current_line/test.rs"]
mod current_line_test;

#[cfg(test)]
#[path = "test/note_popup/test.rs"]
mod note_popup_test;
