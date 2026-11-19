//! The **action** components — the classic RAD IDE `TActionList`, `TAction` and `TImageList`,
//! which are the bridge between what the IDE does internally and what it
//! delivers to the user.
//!
//! The idea, and the reason they exist so early: CrawRAD already has an internal
//! action catalog (ADR 0005).
//! Here this same concept becomes a **component**, for the application the user
//! builds to have the same thing — and, when the IDE shell is rewritten over its
//! own palette, it will use exactly these components. That is what the
//! classic RAD IDE did: the
//! components with which the IDE was built are the ones it sells.
//!
//! They are the first **non-visuals**: they appear as an icon in the designer and do
//! not become any window in the generated application.

pub use crate::components::action_list::{action, action_list};
pub use crate::components::image_list::image_list;
use crawrad_note::note;

#[note("actions-standardaction")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandardAction {
    /// Template identifier (e.g.: "Action", "ControlAction", "EditCut").
    pub name: &'static str,
    /// Identifier base name in Rust's snake_case convention.
    pub base_name: &'static str,
    /// Category for grouping (e.g.: "(No Category)", "Edit", "File", "Dialog").
    pub category: &'static str,
    /// Initial action caption.
    pub text: &'static str,
    /// Initial action hint.
    pub hint: &'static str,
    /// Initial keyboard shortcut ("Ctrl+C", etc.), or empty.
    pub shortcut: &'static str,
    /// Icon index in ImageList (-1 = none).
    pub image_index: i64,
    /// Dataset operation name ("None", "First", "Prior", "Next", "Last", "Insert", "Edit", "Delete", "Post", "Cancel", "Refresh").
    pub dataset_operation: &'static str,
}

#[note("actions-standard-actions")]
pub fn standard_actions() -> Vec<StandardAction> {
    vec![
        // Base and control actions (no category)
        StandardAction {
            name: "Action",
            base_name: "action",
            category: "(No Category)",
            text: "Action",
            hint: "Standard generic action",
            shortcut: "",
            image_index: -1,
            dataset_operation: "None",
        },
        StandardAction {
            name: "ControlAction",
            base_name: "control_action",
            category: "(No Category)",
            text: "ControlAction",
            hint: "Action linked to control state",
            shortcut: "",
            image_index: -1,
            dataset_operation: "None",
        },
        // Edit
        StandardAction {
            name: "EditCut",
            base_name: "edit_cut",
            category: "Edit",
            text: "Cu&t",
            hint: "Cut|Cuts the selection and puts it on the Clipboard",
            shortcut: "Ctrl+X",
            image_index: -1,
            dataset_operation: "None",
        },
        StandardAction {
            name: "EditCopy",
            base_name: "edit_copy",
            category: "Edit",
            text: "&Copy",
            hint: "Copy|Copies the selection and puts it on the Clipboard",
            shortcut: "Ctrl+C",
            image_index: -1,
            dataset_operation: "None",
        },
        StandardAction {
            name: "EditPaste",
            base_name: "edit_paste",
            category: "Edit",
            text: "&Paste",
            hint: "Paste|Inserts Clipboard contents",
            shortcut: "Ctrl+V",
            image_index: -1,
            dataset_operation: "None",
        },
        StandardAction {
            name: "EditSelectAll",
            base_name: "edit_select_all",
            category: "Edit",
            text: "Select &All",
            hint: "Select All|Selects the entire document",
            shortcut: "Ctrl+A",
            image_index: -1,
            dataset_operation: "None",
        },
        StandardAction {
            name: "EditUndo",
            base_name: "edit_undo",
            category: "Edit",
            text: "&Undo",
            hint: "Undo|Reverts the last action",
            shortcut: "Ctrl+Z",
            image_index: -1,
            dataset_operation: "None",
        },
        StandardAction {
            name: "EditDelete",
            base_name: "edit_delete",
            category: "Edit",
            text: "&Delete",
            hint: "Delete|Erases the selection",
            shortcut: "Del",
            image_index: -1,
            dataset_operation: "None",
        },
        // File
        StandardAction {
            name: "FileOpen",
            base_name: "file_open",
            category: "File",
            text: "&Open...",
            hint: "Open|Opens an existing file",
            shortcut: "Ctrl+O",
            image_index: -1,
            dataset_operation: "None",
        },
        StandardAction {
            name: "FileSaveAs",
            base_name: "file_save_as",
            category: "File",
            text: "Save &As...",
            hint: "Save As|Saves the active file with a new name",
            shortcut: "",
            image_index: -1,
            dataset_operation: "None",
        },
        StandardAction {
            name: "FileExit",
            base_name: "file_exit",
            category: "File",
            text: "E&xit",
            hint: "Exit|Quits the application",
            shortcut: "Alt+F4",
            image_index: -1,
            dataset_operation: "None",
        },
        // Dialog
        StandardAction {
            name: "ColorSelect",
            base_name: "color_select",
            category: "Dialog",
            text: "Select &Color...",
            hint: "Color Select|Select a color",
            shortcut: "",
            image_index: -1,
            dataset_operation: "None",
        },
        StandardAction {
            name: "FontEdit",
            base_name: "font_edit",
            category: "Dialog",
            text: "Select &Font...",
            hint: "Font Select|Select font attributes",
            shortcut: "",
            image_index: -1,
            dataset_operation: "None",
        },
        // Dataset
        StandardAction {
            name: "DatasetFirst",
            base_name: "dataset_first",
            category: "Dataset",
            text: "&First",
            hint: "First|Goes to the first record",
            shortcut: "",
            image_index: -1,
            dataset_operation: "First",
        },
        StandardAction {
            name: "DatasetPrior",
            base_name: "dataset_prior",
            category: "Dataset",
            text: "&Prior",
            hint: "Prior|Goes to the prior record",
            shortcut: "",
            image_index: -1,
            dataset_operation: "Prior",
        },
        StandardAction {
            name: "DatasetNext",
            base_name: "dataset_next",
            category: "Dataset",
            text: "&Next",
            hint: "Next|Goes to the next record",
            shortcut: "",
            image_index: -1,
            dataset_operation: "Next",
        },
        StandardAction {
            name: "DatasetLast",
            base_name: "dataset_last",
            category: "Dataset",
            text: "&Last",
            hint: "Last|Goes to the last record",
            shortcut: "",
            image_index: -1,
            dataset_operation: "Last",
        },
        StandardAction {
            name: "DatasetInsert",
            base_name: "dataset_insert",
            category: "Dataset",
            text: "&Insert",
            hint: "Insert|Inserts a new record",
            shortcut: "",
            image_index: -1,
            dataset_operation: "Insert",
        },
        StandardAction {
            name: "DatasetEdit",
            base_name: "dataset_edit",
            category: "Dataset",
            text: "&Edit",
            hint: "Edit|Edits the current record",
            shortcut: "",
            image_index: -1,
            dataset_operation: "Edit",
        },
        StandardAction {
            name: "DatasetDelete",
            base_name: "dataset_delete",
            category: "Dataset",
            text: "&Delete",
            hint: "Delete|Deletes the current record",
            shortcut: "",
            image_index: -1,
            dataset_operation: "Delete",
        },
        StandardAction {
            name: "DatasetPost",
            base_name: "dataset_post",
            category: "Dataset",
            text: "P&ost",
            hint: "Post|Posts modifications to the dataset",
            shortcut: "Ctrl+Enter",
            image_index: -1,
            dataset_operation: "Post",
        },
        StandardAction {
            name: "DatasetCancel",
            base_name: "dataset_cancel",
            category: "Dataset",
            text: "&Cancel",
            hint: "Cancel|Cancels modifications to the dataset",
            shortcut: "Esc",
            image_index: -1,
            dataset_operation: "Cancel",
        },
        StandardAction {
            name: "DatasetRefresh",
            base_name: "dataset_refresh",
            category: "Dataset",
            text: "&Refresh",
            hint: "Refresh|Refreshes the dataset",
            shortcut: "F5",
            image_index: -1,
            dataset_operation: "Refresh",
        },
    ]
}

#[note("actions-standard-actions-by-category")]
pub fn standard_actions_by_category() -> Vec<(&'static str, Vec<StandardAction>)> {
    let all = standard_actions();
    let mut category_order: Vec<&'static str> = Vec::new();
    let mut map: std::collections::BTreeMap<&'static str, Vec<StandardAction>> =
        std::collections::BTreeMap::new();

    for t in all {
        if !category_order.contains(&t.category) {
            category_order.push(t.category);
        }
        map.entry(t.category).or_default().push(t);
    }

    category_order
        .into_iter()
        .map(|cat| (cat, map.remove(cat).unwrap_or_default()))
        .collect()
}
