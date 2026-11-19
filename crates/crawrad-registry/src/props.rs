use crate::{Category, PropDesc};
use crawrad_ir::{Edges, Rect, Value};
use crawrad_note::note;

#[note("props-propkind")]
#[derive(Debug, Clone, PartialEq)]
pub enum PropKind {
    /// Single-line text (simple `TextEdit`).
    String,
    /// Multiline text (opens lines dialog/editor).
    Multiline,
    /// Boolean (Checkbox / Toggle).
    Bool,
    /// Integer with gesture/editing bounds.
    Int { min: i64, max: i64 },
    /// Floating point with bounds and step.
    Float { min: f64, max: f64, step: f64 },
    /// RGBA color (Color Picker with hex preview).
    Color,
    /// Margins / Padding (four edges `left, top, right, bottom`).
    Edges,
    /// **Named flags set** — the classic RAD IDE set.
    ///
    /// `[akLeft, akTop]` of `Anchors`, `[biMinimize, biMaximize]` of
    /// `BorderIcons`. The value is a `Value::Map` of booleans, and the editor is a
    /// checkbox by name.
    ///
    /// # It exists because `anchors` was LYING
    ///
    /// Until 18/09/2026 `anchors` held four booleans and declared
    /// `PropKind::Edges`, which are four **numbers**. It worked because the
    /// Inspector disambiguates by the PAIR `(kind, value)` — there are two editors for
    /// `Edges`, and `anchors.rs` itself admitted: *"the same kind over two
    /// different shapes"*.
    ///
    /// It worked because we are on both sides. **A third party writing
    /// `kind = "edges"` in a `manifest.json` would have no way to know which of the two
    /// editors it would get** — it would depend on the shape of the embedded value.
    /// ADR 0037 states that the
    /// piece's label must decide the editor on its own, and this label decides.
    Set(&'static [&'static str]),
    /// Enumerated options (ComboBox with options).
    Enum(&'static [&'static str]),
    /// Collection of items (opens Items Editor; can be flat or hierarchical, and typed by `item_class`).
    Collection {
        hierarchical: bool,
        item_class: Option<&'static str>,
    },
    /// Reference to another component by expected class (e.g. `Action`, `ImageList`).
    Reference(&'static str),
    /// The name of a field of the dataset that the **sibling** property named here
    /// references (`LookupKey` names a field of `LookupSource`). The editor is a
    /// choice over that dataset's columns, and plain text while they are unknown.
    FieldOf(&'static str),
    /// Rectangular geometry (`x, y, w, h`).
    Rect,
    /// Date and time (`YYYY-MM-DD HH:MM:SS`).
    DateTime,
    /// Path to file on disk (`FilePath`).
    FilePath,
    /// Mouse cursor (`Cursor`).
    Cursor,
    /// Modal dialog result (`ModalResult`).
    ModalResult,
    /// Keyboard shortcut (`Shortcut`).
    Shortcut,
    /// Typography (`Font`: family, size, style).
    Font,
    /// Image / Graphic (`Picture`: PNG, SVG, ICO, JPEG, BMP).
    Picture,
}

#[note("props-anchor-flags")]
pub const ANCHOR_FLAGS: &[&str] = &["left", "top", "right", "bottom"];

/// The capability every control that takes the keyboard answers to.
pub const WIN_CONTROL: &str = "WinControl";

/// What a Grid's `data_source` asks for: a Connection or a Dataset (ADR 0040).
#[note("props-row-source")]
pub const ROW_SOURCE: &str = "DbRowSource";

/// The capability a dataset answers to: a Grid bound to one ignores `table_name`.
#[note("props-db-dataset")]
pub const DB_DATASET: &str = "DbDataset";

#[note("props-align-options")]
pub const ALIGN_OPTIONS: &[&str] = &["None", "Top", "Bottom", "Left", "Right", "Client"];

#[note("props-message-dialog-kinds")]
pub const MESSAGE_DIALOG_KINDS: &[&str] =
    &["mtInformation", "mtWarning", "mtError", "mtConfirmation"];

#[note("props-message-dialog-buttons")]
pub const MESSAGE_DIALOG_BUTTONS: &[&str] = &["mbOK", "mbOKCancel", "mbYesNo", "mbYesNoCancel"];

#[note("props-badge-kinds")]
pub const BADGE_KINDS: &[&str] = &["bkDefault", "bkPrimary", "bkInfo", "bkWarning", "bkDanger"];

#[note("props-drawer-sides")]
pub const DRAWER_SIDES: &[&str] = &["dsLeft", "dsRight"];

#[note("props-breadcrumb-styles")]
pub const BREADCRUMB_STYLES: &[&str] = &["bsText", "bsChevron"];

pub const COMBO_BOX_STYLES: &[&str] = &["DropDownList", "DropDown"];
pub const FONT_STYLE_FLAGS: &[&str] = &["Bold", "Italic"];

#[note("props-date-time-picker-kinds")]
pub const DATE_TIME_PICKER_KINDS: &[&str] = &["dtkDate", "dtkTime"];

#[note("props-dataset-operations")]
pub const DATASET_OPERATIONS: &[&str] = &[
    "None", "First", "Prior", "Next", "Last", "Insert", "Edit", "Delete", "Post", "Cancel",
    "Refresh",
];

#[note("props-binding-modes")]
pub const BINDING_MODES: &[&str] = &["OneWay", "TwoWay", "OneWayToSource", "OneTime"];

pub const SUMMARY_OPTIONS: &[&str] = &["smNone", "smSum", "smCount", "smAvg", "smMin", "smMax"];

pub const FILTER_KIND_OPTIONS: &[&str] = &["fkAuto", "fkText", "fkList", "fkRange", "fkDate"];

pub const COLUMN_EDITOR_OPTIONS: &[&str] = &[
    "edAuto", "edText", "edNumber", "edDate", "edList", "edCheck", "edLookup", "edButton",
];

#[note("props-props")]
pub struct Props;

impl Props {
    #[note("props-props-name")]
    pub fn name() -> PropDesc {
        PropDesc::new_typed(
            "name",
            "Name",
            Value::Str(String::new()),
            Category::Identity,
            PropKind::String,
        )
    }

    #[note("props-props-rect")]
    pub fn rect(default: Rect) -> PropDesc {
        PropDesc::new_typed(
            "rect",
            "Rect",
            Value::Rect(default),
            Category::Layout,
            PropKind::Rect,
        )
    }

    #[note("props-props-caption")]
    pub fn caption(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "text",
            "Caption",
            Value::Str(default.to_string()),
            Category::Appearance,
            PropKind::String,
        )
    }

    #[note("props-props-text")]
    pub fn text(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "text",
            "Text",
            Value::Str(default.to_string()),
            Category::Appearance,
            PropKind::String,
        )
    }

    #[note("props-props-lines")]
    pub fn lines(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "lines",
            "Lines",
            Value::Str(default.to_string()),
            Category::Appearance,
            PropKind::Multiline,
        )
        .multiline()
    }

    #[note("props-props-cell-text")]
    pub fn cell_text(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "cell_text",
            "Cells",
            Value::Str(default.to_string()),
            Category::Appearance,
            PropKind::Multiline,
        )
        .multiline()
    }

    #[note("props-props-enabled")]
    pub fn enabled(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "enabled",
            "Enabled",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-visible")]
    pub fn visible(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "visible",
            "Visible",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-parent-font")]
    pub fn parent_font(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "parent_font",
            "ParentFont",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-parent-color")]
    pub fn parent_color(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "parent_color",
            "ParentColor",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-show-hint")]
    pub fn show_hint(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "show_hint",
            "ShowHint",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    pub fn hint(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "hint",
            "Hint",
            Value::Str(default.to_string()),
            Category::Behavior,
            PropKind::String,
        )
    }

    #[note("props-props-checked")]
    pub fn checked(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "checked",
            "Checked",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-removable")]
    pub fn removable(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "removable",
            "Removable",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-separator")]
    pub fn separator(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "separator",
            "Separator",
            Value::Str(default.to_string()),
            Category::Appearance,
            PropKind::String,
        )
    }

    #[note("props-props-page-count")]
    pub fn page_count(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "page_count",
            "PageCount",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: 1,
                max: i32::MAX as i64,
            },
        )
    }

    #[note("props-props-page-index")]
    pub fn page_index(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "page_index",
            "PageIndex",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: 0,
                max: i32::MAX as i64,
            },
        )
    }

    #[note("props-props-max-buttons")]
    pub fn max_buttons(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "max_buttons",
            "MaxButtons",
            Value::Int(default),
            Category::Appearance,
            PropKind::Int { min: 1, max: 100 },
        )
    }

    #[note("props-props-show-arrows")]
    pub fn show_arrows(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "show_arrows",
            "ShowArrows",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-check-state")]
    pub fn check_state(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "state",
            "State",
            Value::Enum(default.into()),
            Category::Behavior,
            PropKind::Enum(&["cbUnchecked", "cbChecked", "cbGrayed"]),
        )
        .with_options(&["cbUnchecked", "cbChecked", "cbGrayed"])
    }

    #[note("props-props-allow-grayed")]
    pub fn allow_grayed(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "allow_grayed",
            "AllowGrayed",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-align")]
    pub fn align() -> PropDesc {
        PropDesc::new_typed(
            "align",
            "Align",
            Value::Enum("None".into()),
            Category::Layout,
            PropKind::Enum(ALIGN_OPTIONS),
        )
        .with_options(ALIGN_OPTIONS)
    }

    #[note("props-props-anchors")]
    pub fn anchors() -> PropDesc {
        PropDesc::new_typed(
            "anchors",
            "Anchors",
            Value::Map(
                [
                    ("left".to_string(), Value::Bool(true)),
                    ("top".to_string(), Value::Bool(true)),
                    ("right".to_string(), Value::Bool(false)),
                    ("bottom".to_string(), Value::Bool(false)),
                ]
                .into_iter()
                .collect(),
            ),
            Category::Layout,
            PropKind::Set(ANCHOR_FLAGS),
        )
    }

    #[note("props-props-margins")]
    pub fn margins(default: Edges) -> PropDesc {
        PropDesc::new_typed(
            "margins",
            "Margins",
            Value::Edges(default),
            Category::Layout,
            PropKind::Edges,
        )
    }

    #[note("props-props-align-with-margins")]
    pub fn align_with_margins(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "align_with_margins",
            "AlignWithMargins",
            Value::Bool(default),
            Category::Layout,
            PropKind::Bool,
        )
    }

    #[note("props-props-tab-order")]
    pub fn tab_order(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "tab_order",
            "TabOrder",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int { min: 0, max: 1000 },
        )
    }

    #[note("props-props-tab-stop")]
    pub fn tab_stop(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "tab_stop",
            "TabStop",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-action")]
    pub fn action() -> PropDesc {
        PropDesc::new_typed(
            "action",
            "Action",
            Value::Ref(String::new()),
            Category::Behavior,
            PropKind::Reference("Action"),
        )
        .references("Action")
    }

    /// The control a Label hands the focus to when clicked: classic
    /// RAD's `FocusControl`. Any control that takes the keyboard answers.
    pub fn focus_control() -> PropDesc {
        PropDesc::new_typed(
            "focus_control",
            "FocusControl",
            Value::Ref(String::new()),
            Category::Behavior,
            PropKind::Reference(WIN_CONTROL),
        )
        .references(WIN_CONTROL)
    }

    #[note("props-props-dataset")]
    pub fn dataset() -> PropDesc {
        PropDesc::new_typed(
            "dataset",
            "Dataset",
            Value::Ref(String::new()),
            Category::Behavior,
            PropKind::Reference("DbDataset"),
        )
        .references("DbDataset")
    }

    #[note("props-props-binding-dataset")]
    pub fn binding_dataset() -> PropDesc {
        PropDesc::new_typed(
            "binding.dataset",
            "Dataset",
            Value::Ref(String::new()),
            Category::Behavior,
            PropKind::Reference("DbDataset"),
        )
        .references("DbDataset")
    }

    #[note("props-props-binding-field")]
    pub fn binding_field() -> PropDesc {
        PropDesc::new_typed(
            "binding.field",
            "Field",
            Value::Str(String::new()),
            Category::Behavior,
            PropKind::String,
        )
    }

    #[note("props-props-binding-mode")]
    pub fn binding_mode() -> PropDesc {
        PropDesc::new_typed(
            "binding.mode",
            "Mode",
            Value::Enum("TwoWay".into()),
            Category::Behavior,
            PropKind::Enum(BINDING_MODES),
        )
        .with_options(BINDING_MODES)
    }

    #[note("props-props-dataset-operation")]
    pub fn dataset_operation(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "dataset_operation",
            "Dataset Operation",
            Value::Enum(default.into()),
            Category::Behavior,
            PropKind::Enum(DATASET_OPERATIONS),
        )
        .with_options(DATASET_OPERATIONS)
    }

    #[note("props-props-popup-menu")]
    pub fn popup_menu() -> PropDesc {
        PropDesc::new_typed(
            "popup_menu",
            "PopupMenu",
            Value::Ref(String::new()),
            Category::Behavior,
            PropKind::Reference("PopupMenu"),
        )
        .references("PopupMenu")
    }

    #[note("props-props-images")]
    pub fn images() -> PropDesc {
        PropDesc::new_typed(
            "images",
            "Images",
            Value::Ref(String::new()),
            Category::Appearance,
            PropKind::Reference("ImageList"),
        )
        .references("ImageList")
    }

    #[note("props-props-tree-items")]
    pub fn tree_items() -> PropDesc {
        PropDesc::new_typed(
            "items",
            "Items",
            Value::List(Vec::new()),
            Category::Appearance,
            PropKind::Collection {
                hierarchical: true,
                item_class: None,
            },
        )
        .in_tree()
    }

    #[note("props-props-datetime")]
    pub fn datetime(default: crawrad_ir::DateTime) -> PropDesc {
        PropDesc::new_typed(
            "datetime",
            "DateTime",
            Value::DateTime(default),
            Category::Behavior,
            PropKind::DateTime,
        )
    }

    #[note("props-props-file-path")]
    pub fn file_path(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "file_path",
            "FilePath",
            Value::FilePath(crawrad_ir::FilePath::new(default)),
            Category::Behavior,
            PropKind::FilePath,
        )
    }

    #[note("props-props-cursor")]
    pub fn cursor(default: crawrad_ir::Cursor) -> PropDesc {
        PropDesc::new_typed(
            "cursor",
            "Cursor",
            Value::Cursor(default),
            Category::Appearance,
            PropKind::Cursor,
        )
    }

    #[note("props-props-modal-result")]
    pub fn modal_result(default: crawrad_ir::ModalResult) -> PropDesc {
        PropDesc::new_typed(
            "modal_result",
            "ModalResult",
            Value::ModalResult(default),
            Category::Behavior,
            PropKind::ModalResult,
        )
    }

    #[note("props-props-shortcut")]
    pub fn shortcut(default: impl Into<crawrad_ir::Shortcut>) -> PropDesc {
        PropDesc::new_typed(
            "shortcut",
            "ShortCut",
            Value::Shortcut(default.into()),
            Category::Behavior,
            PropKind::Shortcut,
        )
    }

    #[note("props-props-font")]
    pub fn font(default: impl Into<crawrad_ir::Font>) -> PropDesc {
        PropDesc::new_typed(
            "font",
            "Font",
            Value::Font(default.into()),
            Category::Appearance,
            PropKind::Font,
        )
    }

    #[note("props-props-picture")]
    pub fn picture(default: impl Into<crawrad_ir::Picture>) -> PropDesc {
        PropDesc::new_typed(
            "picture",
            "Picture",
            Value::Picture(default.into()),
            Category::Appearance,
            PropKind::Picture,
        )
    }

    #[note("props-props-tree-items-sample")]
    pub fn tree_items_sample() -> PropDesc {
        PropDesc::new_typed(
            "items",
            "Items",
            crawrad_ir::Item::list_to(&[
                crawrad_ir::Item::with_children(
                    "Node1",
                    vec![
                        crawrad_ir::Item::new("SubNode1"),
                        crawrad_ir::Item::new("SubNode2"),
                    ],
                ),
                crawrad_ir::Item::new("Node2"),
            ]),
            Category::Appearance,
            PropKind::Collection {
                hierarchical: true,
                item_class: None,
            },
        )
        .in_tree()
        .read_only()
    }

    #[note("props-props-list-view-items-sample")]
    pub fn list_view_items_sample() -> PropDesc {
        PropDesc::new_typed(
            "items",
            "Items",
            crawrad_ir::Item::list_to(&[
                crawrad_ir::Item::with_children("Item 1", vec![crawrad_ir::Item::new("SubItem 1")]),
                crawrad_ir::Item::with_children("Item 2", vec![crawrad_ir::Item::new("SubItem 2")]),
            ]),
            Category::Appearance,
            PropKind::Collection {
                hierarchical: true,
                item_class: None,
            },
        )
        // A row's sub-items are its children: the Items editor nests them only
        // for a tree collection, and the package manifest keeps `hierarchical`
        // only through `tree_collection`.
        .in_tree()
        .read_only()
    }

    #[note("props-props-flat-items")]
    pub fn flat_items() -> PropDesc {
        PropDesc::new_typed(
            "items",
            "Items",
            Value::List(Vec::new()),
            Category::Appearance,
            PropKind::Collection {
                hierarchical: false,
                item_class: None,
            },
        )
    }

    #[note("props-props-flat-items-sample")]
    pub fn flat_items_sample(samples: &[&str]) -> PropDesc {
        let items: Vec<crawrad_ir::Item> =
            samples.iter().map(|s| crawrad_ir::Item::new(*s)).collect();
        PropDesc::new_typed(
            "items",
            "Items",
            crawrad_ir::Item::list_to(&items),
            Category::Appearance,
            PropKind::Collection {
                hierarchical: false,
                item_class: None,
            },
        )
        .read_only()
    }

    #[note("props-props-selected-items")]
    pub fn selected_items() -> PropDesc {
        PropDesc::new_typed(
            "selected_items",
            "SelectedItems",
            Value::List(Vec::new()),
            Category::Appearance,
            PropKind::Collection {
                hierarchical: false,
                item_class: None,
            },
        )
    }

    #[note("props-props-checked-items")]
    pub fn checked_items() -> PropDesc {
        PropDesc::new_typed(
            "checked_items",
            "CheckedItems",
            Value::List(Vec::new()),
            Category::Appearance,
            PropKind::Collection {
                hierarchical: false,
                item_class: None,
            },
        )
    }

    #[note("props-props-spin-value")]
    pub fn spin_value(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "value",
            "Value",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: i32::MIN as i64,
                max: i32::MAX as i64,
            },
        )
    }

    #[note("props-props-min-value")]
    pub fn min_value(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "min_value",
            "MinValue",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: i32::MIN as i64,
                max: i32::MAX as i64,
            },
        )
    }

    #[note("props-props-max-value")]
    pub fn max_value(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "max_value",
            "MaxValue",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: i32::MIN as i64,
                max: i32::MAX as i64,
            },
        )
    }

    #[note("props-props-increment")]
    pub fn increment(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "increment",
            "Increment",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: 1,
                max: i32::MAX as i64,
            },
        )
    }

    #[note("props-props-editor-enabled")]
    pub fn editor_enabled(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "editor_enabled",
            "EditorEnabled",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-columns")]
    pub fn columns(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "columns",
            "Columns",
            Value::Int(default),
            Category::Appearance,
            PropKind::Int { min: 1, max: 16 },
        )
    }

    pub fn item_index(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "item_index",
            "ItemIndex",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: -1,
                max: 100_000,
            },
        )
    }

    #[note("props-props-multi-select")]
    pub fn multi_select(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "multi_select",
            "MultiSelect",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-sorted")]
    pub fn sorted(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "sorted",
            "Sorted",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-border-width")]
    pub fn border_width(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "border_width",
            "BorderWidth",
            Value::Int(default),
            Category::Appearance,
            PropKind::Int { min: 0, max: 20 },
        )
    }

    #[note("props-props-active-page")]
    pub fn active_page(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "active_page",
            "ActivePageIndex",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int { min: 0, max: 100 },
        )
    }

    #[note("props-props-tab-position")]
    pub fn tab_position(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "tab_position",
            "TabPosition",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(&["tpTop", "tpBottom"]),
        )
        .with_options(&["tpTop", "tpBottom"])
    }

    #[note("props-props-tab-visible")]
    pub fn tab_visible(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "tab_visible",
            "TabVisible",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-scroll-bar-kind")]
    pub fn scroll_bar_kind() -> PropDesc {
        PropDesc::new_typed(
            "kind",
            "Kind",
            Value::Enum("sbHorizontal".into()),
            Category::Appearance,
            PropKind::Enum(&["sbHorizontal", "sbVertical"]),
        )
        .with_options(&["sbHorizontal", "sbVertical"])
    }

    #[note("props-props-min")]
    pub fn min(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "min",
            "Min",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: -1_000_000,
                max: 1_000_000,
            },
        )
    }

    #[note("props-props-max")]
    pub fn max(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "max",
            "Max",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: -1_000_000,
                max: 1_000_000,
            },
        )
    }

    #[note("props-props-position")]
    pub fn position(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "position",
            "Position",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: 0,
                max: 1_000_000,
            },
        )
    }

    #[note("props-props-small-change")]
    pub fn small_change(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "small_change",
            "SmallChange",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: 1,
                max: 10_000,
            },
        )
    }

    #[note("props-props-large-change")]
    pub fn large_change(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "large_change",
            "LargeChange",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: 1,
                max: 10_000,
            },
        )
    }

    #[note("props-props-auto-scroll")]
    pub fn auto_scroll(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "auto_scroll",
            "AutoScroll",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-horz-scroll-bar")]
    pub fn horz_scroll_bar(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "horz_scroll_bar",
            "HorzScrollBar",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-vert-scroll-bar")]
    pub fn vert_scroll_bar(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "vert_scroll_bar",
            "VertScrollBar",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-image-width")]
    pub fn image_width(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "image_width",
            "Width",
            Value::Int(default),
            Category::Appearance,
            PropKind::Int { min: 8, max: 256 },
        )
    }

    #[note("props-props-image-height")]
    pub fn image_height(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "image_height",
            "Height",
            Value::Int(default),
            Category::Appearance,
            PropKind::Int { min: 8, max: 256 },
        )
    }

    #[note("props-props-images-data")]
    pub fn images_data() -> PropDesc {
        PropDesc::new_typed(
            "images_data",
            "Images",
            Value::List(Vec::new()),
            Category::Advanced,
            PropKind::Collection {
                hierarchical: false,
                item_class: None,
            },
        )
        .read_only()
    }

    #[note("props-props-image-index")]
    pub fn image_index(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "image_index",
            "ImageIndex",
            Value::Int(default),
            Category::Appearance,
            PropKind::Int {
                min: -1,
                max: 10_000,
            },
        )
    }

    #[note("props-props-action-category")]
    pub fn action_category(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "category",
            "Category",
            Value::Str(default.to_string()),
            Category::Identity,
            PropKind::String,
        )
    }

    #[note("props-props-color")]
    pub fn color(default: crawrad_ir::Color) -> PropDesc {
        PropDesc::new_typed(
            "color",
            "Color",
            Value::Color(default),
            Category::Appearance,
            PropKind::Color,
        )
    }

    #[note("props-props-selected-color")]
    pub fn selected_color(default: crawrad_ir::Color) -> PropDesc {
        PropDesc::new_typed(
            "selected",
            "Selected",
            Value::Color(default),
            Category::Behavior,
            PropKind::Color,
        )
    }

    #[note("props-props-url")]
    pub fn url(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "url",
            "URL",
            Value::Str(default.to_string()),
            Category::Behavior,
            PropKind::String,
        )
    }

    #[note("props-props-edit-mask")]
    pub fn edit_mask(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "edit_mask",
            "EditMask",
            Value::Str(default.to_string()),
            Category::Behavior,
            PropKind::String,
        )
    }

    #[note("props-props-collapsed")]
    pub fn collapsed(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "collapsed",
            "Collapsed",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-bevel-shape")]
    pub fn bevel_shape(default: &'static str) -> PropDesc {
        const KINDS: &[&str] = &[
            "Box",
            "Frame",
            "TopLine",
            "BottomLine",
            "LeftLine",
            "RightLine",
            "Spacer",
        ];
        PropDesc::new_typed(
            "bevel_shape",
            "Shape",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(KINDS),
        )
        .with_options(KINDS)
    }

    #[note("props-props-bevel-style")]
    pub fn bevel_style(default: &'static str) -> PropDesc {
        const STYLES: &[&str] = &["Lowered", "Raised"];
        PropDesc::new_typed(
            "bevel_style",
            "Style",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(STYLES),
        )
        .with_options(STYLES)
    }

    #[note("props-props-font-size")]
    pub fn font_size(default: f64) -> PropDesc {
        PropDesc::new_typed(
            "font_size",
            "FontSize",
            Value::Float(default),
            Category::Appearance,
            PropKind::Float {
                min: 6.0,
                max: 72.0,
                step: 1.0,
            },
        )
    }

    #[note("props-props-interval")]
    pub fn interval(default_ms: i64) -> PropDesc {
        PropDesc::new_typed(
            "interval",
            "Interval",
            Value::Int(default_ms),
            Category::Behavior,
            PropKind::Int {
                min: 1,
                max: 3_600_000,
            },
        )
    }

    #[note("props-props-password-char")]
    pub fn password_char(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "password_char",
            "PasswordChar",
            Value::Str(default.to_string()),
            Category::Behavior,
            PropKind::String,
        )
    }

    #[note("props-props-form-position")]
    pub fn form_position(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "position",
            "Position",
            Value::Enum(default.into()),
            Category::Behavior,
            PropKind::Enum(&["poDesigned", "poScreenCenter"]),
        )
        .with_options(&["poDesigned", "poScreenCenter"])
    }

    #[note("props-props-window-state")]
    pub fn window_state(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "window_state",
            "WindowState",
            Value::Enum(default.into()),
            Category::Behavior,
            PropKind::Enum(&["wsNormal", "wsMaximized", "wsMinimized"]),
        )
        .with_options(&["wsNormal", "wsMaximized", "wsMinimized"])
    }

    #[note("props-props-form-theme")]
    pub fn form_theme(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "theme",
            "Theme",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(&["thSystem", "thLight", "thDark"]),
        )
        .with_options(&["thSystem", "thLight", "thDark"])
    }
    #[note("props-props-alignment")]
    pub fn alignment(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "alignment",
            "Alignment",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(&["taLeftJustify", "taCenter", "taRightJustify"]),
        )
        .with_options(&["taLeftJustify", "taCenter", "taRightJustify"])
    }

    #[note("props-props-check-alignment")]
    pub fn check_alignment(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "alignment",
            "Alignment",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(&["taLeftJustify", "taRightJustify"]),
        )
        .with_options(&["taLeftJustify", "taRightJustify"])
    }

    #[note("props-props-text-layout")]
    pub fn text_layout(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "layout",
            "Layout",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(&["tlTop", "tlCenter", "tlBottom"]),
        )
        .with_options(&["tlTop", "tlCenter", "tlBottom"])
    }
    #[note("props-props-word-wrap")]
    pub fn word_wrap(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "word_wrap",
            "WordWrap",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-want-returns")]
    pub fn want_returns(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "want_returns",
            "WantReturns",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-want-tabs")]
    pub fn want_tabs(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "want_tabs",
            "WantTabs",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-max-length")]
    pub fn max_length(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "max_length",
            "MaxLength",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: 0,
                max: i32::MAX as i64,
            },
        )
    }
    pub fn readonly(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "readonly",
            "ReadOnly",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-char-case")]
    pub fn char_case(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "char_case",
            "CharCase",
            Value::Enum(default.into()),
            Category::Behavior,
            PropKind::Enum(&["ecNormal", "ecUpperCase", "ecLowerCase"]),
        )
        .with_options(&["ecNormal", "ecUpperCase", "ecLowerCase"])
    }

    #[note("props-props-numbers-only")]
    pub fn numbers_only(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "numbers_only",
            "NumbersOnly",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-multiline-text")]
    pub fn multiline_text(key: &'static str, display: &'static str, default: &str) -> PropDesc {
        PropDesc::new_typed(
            key,
            display,
            Value::Str(default.to_string()),
            Category::Appearance,
            PropKind::Multiline,
        )
        .multiline()
    }

    #[note("props-props-enum-options")]
    pub fn enum_options(
        key: &'static str,
        display: &'static str,
        default: &str,
        options: &'static [&'static str],
        category: Category,
    ) -> PropDesc {
        PropDesc::new_typed(
            key,
            display,
            Value::Enum(default.to_string()),
            category,
            PropKind::Enum(options),
        )
        .with_options(options)
    }

    #[note("props-props-value-int")]
    pub fn value_int(
        key: &'static str,
        display: &'static str,
        default: i64,
        min: i64,
        max: i64,
    ) -> PropDesc {
        PropDesc::new_typed(
            key,
            display,
            Value::Int(default),
            Category::Behavior,
            PropKind::Int { min, max },
        )
    }

    #[note("props-props-stretch")]
    pub fn stretch(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "stretch",
            "Stretch",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-proportional")]
    pub fn proportional(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "proportional",
            "Proportional",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-center")]
    pub fn center(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "center",
            "Center",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-auto-size")]
    pub fn auto_size(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "auto_size",
            "AutoSize",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-transparent")]
    pub fn transparent(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "transparent",
            "Transparent",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-bit-btn-kind")]
    pub fn bit_btn_kind(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "kind",
            "Kind",
            Value::Enum(default.into()),
            Category::Behavior,
            PropKind::Enum(&[
                "bkCustom", "bkOK", "bkCancel", "bkHelp", "bkClose", "bkYes", "bkNo",
            ]),
        )
        .with_options(&[
            "bkCustom", "bkOK", "bkCancel", "bkHelp", "bkClose", "bkYes", "bkNo",
        ])
    }

    #[note("props-props-step")]
    pub fn step(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "step",
            "Step",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: 1,
                max: 10_000,
            },
        )
    }

    #[note("props-props-orientation")]
    pub fn orientation(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "orientation",
            "Orientation",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(&["Horizontal", "Vertical"]),
        )
        .with_options(&["Horizontal", "Vertical"])
    }

    #[note("props-props-makepad-slider-look")]
    pub fn makepad_slider_look(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "makepad.look",
            "Look",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(&["Minimal", "Box", "Round"]),
        )
        .with_options(&["Minimal", "Box", "Round"])
    }

    #[note("props-props-shape-kind")]
    pub fn shape_kind(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "shape",
            "Shape",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(&[
                "Rectangle",
                "Square",
                "RoundRect",
                "RoundSquare",
                "Ellipse",
                "Circle",
            ]),
        )
        .with_options(&[
            "Rectangle",
            "Square",
            "RoundRect",
            "RoundSquare",
            "Ellipse",
            "Circle",
        ])
    }

    #[note("props-props-brush-color")]
    pub fn brush_color(default: crawrad_ir::Color) -> PropDesc {
        PropDesc::new_typed(
            "brush_color",
            "BrushColor",
            Value::Color(default),
            Category::Appearance,
            PropKind::Color,
        )
    }

    #[note("props-props-pen-color")]
    pub fn pen_color(default: crawrad_ir::Color) -> PropDesc {
        PropDesc::new_typed(
            "pen_color",
            "PenColor",
            Value::Color(default),
            Category::Appearance,
            PropKind::Color,
        )
    }

    pub fn pen_width(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "pen_width",
            "PenWidth",
            Value::Int(default),
            Category::Appearance,
            PropKind::Int { min: 0, max: 100 },
        )
    }

    #[note("props-props-flat")]
    pub fn flat(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "flat",
            "Flat",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-down")]
    pub fn down(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "down",
            "Down",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-group-index")]
    pub fn group_index(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "group_index",
            "GroupIndex",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int { min: 0, max: 1_000 },
        )
    }

    #[note("props-props-allow-all-up")]
    pub fn allow_all_up(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "allow_all_up",
            "AllowAllUp",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-simple-text")]
    pub fn simple_text(default: impl Into<String>) -> PropDesc {
        PropDesc::new_typed(
            "simple_text",
            "SimpleText",
            Value::Str(default.into()),
            Category::Appearance,
            PropKind::String,
        )
    }

    #[note("props-props-panel-width")]
    pub fn panel_width(default: i64) -> PropDesc {
        Self::appearance_extent("width", "Width", default)
    }

    pub fn simple_panel(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "simple_panel",
            "SimplePanel",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-show-captions")]
    pub fn show_captions(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "show_captions",
            "ShowCaptions",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-button-width")]
    pub fn button_width(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "button_width",
            "ButtonWidth",
            Value::Int(default),
            Category::Appearance,
            PropKind::Int { min: 16, max: 256 },
        )
    }

    #[note("props-props-button-height")]
    pub fn button_height(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "button_height",
            "ButtonHeight",
            Value::Int(default),
            Category::Appearance,
            PropKind::Int { min: 16, max: 256 },
        )
    }

    #[note("props-props-title")]
    pub fn title(default: impl Into<String>) -> PropDesc {
        PropDesc::new_typed(
            "title",
            "Title",
            Value::Str(default.into()),
            Category::Appearance,
            PropKind::String,
        )
    }

    #[note("props-props-filter")]
    pub fn filter(default: impl Into<String>) -> PropDesc {
        PropDesc::new_typed(
            "filter",
            "Filter",
            Value::Str(default.into()),
            Category::Behavior,
            PropKind::String,
        )
    }

    #[note("props-props-file-name")]
    pub fn file_name(default: impl Into<String>) -> PropDesc {
        PropDesc::new_typed(
            "file_name",
            "FileName",
            Value::Str(default.into()),
            Category::Behavior,
            PropKind::String,
        )
    }

    #[note("props-props-initial-dir")]
    pub fn initial_dir(default: impl Into<String>) -> PropDesc {
        PropDesc::new_typed(
            "initial_dir",
            "InitialDir",
            Value::Str(default.into()),
            Category::Behavior,
            PropKind::String,
        )
    }

    #[note("props-props-default-ext")]
    pub fn default_ext(default: impl Into<String>) -> PropDesc {
        PropDesc::new_typed(
            "default_ext",
            "DefaultExt",
            Value::Str(default.into()),
            Category::Behavior,
            PropKind::String,
        )
    }

    #[note("props-props-glyph")]
    pub fn glyph() -> PropDesc {
        PropDesc::new_typed(
            "glyph",
            "Glyph",
            Value::Picture(crawrad_ir::Picture::empty()),
            Category::Appearance,
            PropKind::Picture,
        )
    }

    #[note("props-props-num-glyphs")]
    pub fn num_glyphs(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "num_glyphs",
            "NumGlyphs",
            Value::Int(default),
            Category::Appearance,
            PropKind::Int { min: 1, max: 4 },
        )
    }

    #[note("props-props-smooth")]
    pub fn smooth(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "smooth",
            "Smooth",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-page-size")]
    pub fn page_size(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "page_size",
            "PageSize",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int { min: 1, max: 1_000 },
        )
    }

    #[note("props-props-col-count")]
    pub fn col_count(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "col_count",
            "ColCount",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int { min: 1, max: 100 },
        )
    }

    #[note("props-props-row-count")]
    pub fn row_count(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "row_count",
            "RowCount",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: 0,
                max: 100_000,
            },
        )
    }

    #[note("props-props-fixed-cols")]
    pub fn fixed_cols(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "fixed_cols",
            "FixedCols",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int { min: 0, max: 10 },
        )
    }

    #[note("props-props-fixed-rows")]
    pub fn fixed_rows(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "fixed_rows",
            "FixedRows",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int { min: 0, max: 10 },
        )
    }

    #[note("props-props-default-col-width")]
    pub fn default_col_width(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "default_col_width",
            "DefaultColWidth",
            Value::Int(default),
            Category::Appearance,
            PropKind::Int {
                min: 20,
                max: 1_000,
            },
        )
    }

    #[note("props-props-default-row-height")]
    pub fn default_row_height(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "default_row_height",
            "DefaultRowHeight",
            Value::Int(default),
            Category::Appearance,
            PropKind::Int { min: 16, max: 500 },
        )
    }

    #[note("props-props-grid-lines")]
    pub fn grid_lines(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "grid_lines",
            "GridLines",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-group-bar")]
    pub fn group_bar(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "group_bar",
            "GroupBar",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-editing")]
    pub fn editing(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "editing",
            "Editing",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-data-source")]
    pub fn data_source() -> PropDesc {
        PropDesc::new_typed(
            "data_source",
            "DataSource",
            Value::Ref(String::new()),
            Category::Data,
            PropKind::Reference(ROW_SOURCE),
        )
        .references(ROW_SOURCE)
    }

    #[note("props-props-table-name")]
    pub fn table_name(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "table_name",
            "TableName",
            Value::Str(default.to_string()),
            Category::Data,
            PropKind::String,
        )
        // a Dataset brings its own query: the table is not asked for
        .hidden_when_ref("data_source", DB_DATASET)
    }

    #[note("props-props-design-rows")]
    pub fn design_rows() -> PropDesc {
        PropDesc::new_typed(
            crawrad_ir::design_rows::DESIGN_ROWS_KEY,
            "DesignRows",
            Value::Enum("8".to_string()),
            Category::Data,
            PropKind::Enum(&crawrad_ir::design_rows::CHOICES),
        )
        .with_options(&crawrad_ir::design_rows::CHOICES)
    }

    #[note("props-props-min-size")]
    pub fn min_size(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "min_size",
            "MinSize",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int { min: 1, max: 2_000 },
        )
    }

    #[note("props-props-auto-snap")]
    pub fn auto_snap(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "auto_snap",
            "AutoSnap",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-beveled")]
    pub fn beveled(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "beveled",
            "Beveled",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-row-select")]
    pub fn row_select(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "row_select",
            "RowSelect",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-show-column-headers")]
    pub fn show_column_headers(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "show_column_headers",
            "ShowColumnHeaders",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-message-text")]
    pub fn message_text(default: impl Into<String>) -> PropDesc {
        PropDesc::new_typed(
            "text",
            "Text",
            Value::Str(default.into()),
            Category::Appearance,
            PropKind::String,
        )
    }

    #[note("props-props-message-dialog-kind")]
    pub fn message_dialog_kind(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "kind",
            "Kind",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(MESSAGE_DIALOG_KINDS),
        )
        .with_options(MESSAGE_DIALOG_KINDS)
    }

    #[note("props-props-message-dialog-buttons")]
    pub fn message_dialog_buttons(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "buttons",
            "Buttons",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(MESSAGE_DIALOG_BUTTONS),
        )
        .with_options(MESSAGE_DIALOG_BUTTONS)
    }

    #[note("props-props-subtitle")]
    pub fn subtitle(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "subtitle",
            "Subtitle",
            Value::Str(default.to_string()),
            Category::Appearance,
            PropKind::String,
        )
    }

    #[note("props-props-corner-radius")]
    pub fn corner_radius(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "corner_radius",
            "CornerRadius",
            Value::Int(default),
            Category::Appearance,
            PropKind::Int { min: 0, max: 100 },
        )
    }

    #[note("props-props-elevation")]
    pub fn elevation(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "elevation",
            "Elevation",
            Value::Int(default),
            Category::Appearance,
            PropKind::Int { min: 0, max: 3 },
        )
    }

    #[note("props-props-show-header")]
    pub fn show_header(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "show_header",
            "ShowHeader",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-badge-kind")]
    pub fn badge_kind(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "kind",
            "Kind",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(BADGE_KINDS),
        )
        .with_options(BADGE_KINDS)
    }

    #[note("props-props-outline")]
    pub fn outline(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "outline",
            "Outline",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-count")]
    pub fn count(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "count",
            "Count",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int { min: 0, max: 99999 },
        )
    }

    #[note("props-props-max-count")]
    pub fn max_count(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "max_count",
            "MaxCount",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int { min: 1, max: 99999 },
        )
    }

    #[note("props-props-associate")]
    pub fn associate() -> PropDesc {
        PropDesc::new_typed(
            "associate",
            "Associate",
            Value::Ref(String::new()),
            Category::Behavior,
            PropKind::Reference("Control"),
        )
        .references("Control")
    }

    #[note("props-props-side")]
    pub fn side(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "side",
            "Side",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(DRAWER_SIDES),
        )
        .with_options(DRAWER_SIDES)
    }

    #[note("props-props-opened")]
    pub fn opened(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "opened",
            "Opened",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    #[note("props-props-overlay")]
    pub fn overlay(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "overlay",
            "Overlay",
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    #[note("props-props-text-hint")]
    pub fn text_hint(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "text_hint",
            "TextHint",
            Value::Str(default.to_string()),
            Category::Appearance,
            PropKind::String,
        )
    }

    #[note("props-props-drop-down-count")]
    pub fn drop_down_count(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "drop_down_count",
            "DropDownCount",
            Value::Int(default),
            Category::Appearance,
            PropKind::Int { min: 1, max: 100 },
        )
    }

    pub fn combo_box_style(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "style",
            "Style",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(COMBO_BOX_STYLES),
        )
        .with_options(COMBO_BOX_STYLES)
    }

    #[note("props-props-breadcrumb-style")]
    pub fn breadcrumb_style(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "style",
            "Style",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(BREADCRUMB_STYLES),
        )
        .with_options(BREADCRUMB_STYLES)
    }

    #[note("props-props-segment-color")]
    pub fn segment_color(default: crawrad_ir::Color) -> PropDesc {
        PropDesc::new_typed(
            "segment_color",
            "SegmentColor",
            Value::Color(default),
            Category::Appearance,
            PropKind::Color,
        )
    }

    #[note("props-props-current-color")]
    pub fn current_color(default: crawrad_ir::Color) -> PropDesc {
        PropDesc::new_typed(
            "current_color",
            "CurrentColor",
            Value::Color(default),
            Category::Appearance,
            PropKind::Color,
        )
    }

    #[note("props-props-date-time-picker-kind")]
    pub fn date_time_picker_kind(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "kind",
            "Kind",
            Value::Enum(default.into()),
            Category::Behavior,
            PropKind::Enum(DATE_TIME_PICKER_KINDS),
        )
        .with_options(DATE_TIME_PICKER_KINDS)
    }

    #[note("props-props-date-time-value")]
    pub fn date_time_value(default: &str) -> PropDesc {
        let mut p = PropDesc::new_typed(
            "date_time",
            "DateTime",
            Value::Str(default.to_string()),
            Category::Behavior,
            PropKind::String,
        );
        p.bindable = false;
        p
    }

    #[note("props-props-min-date")]
    pub fn min_date(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "min_date",
            "MinDate",
            Value::Str(default.to_string()),
            Category::Behavior,
            PropKind::String,
        )
    }

    #[note("props-props-max-date")]
    pub fn max_date(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "max_date",
            "MaxDate",
            Value::Str(default.to_string()),
            Category::Behavior,
            PropKind::String,
        )
    }

    #[note("props-props-field")]
    pub fn field(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "field",
            "Field",
            Value::Str(default.to_string()),
            Category::Data,
            PropKind::String,
        )
    }

    #[note("props-props-display-format")]
    pub fn display_format(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "display_format",
            "DisplayFormat",
            Value::Str(default.to_string()),
            Category::Data,
            PropKind::String,
        )
    }

    #[note("props-props-width")]
    pub fn width(default: i64) -> PropDesc {
        Self::appearance_extent("width", "Width", default)
    }

    /// Display text for NULL cells.
    pub fn null_text(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "null_text",
            "NullText",
            Value::Str(default.to_string()),
            Category::Data,
            PropKind::String,
        )
    }

    /// Custom display texts for boolean true and false separated by semicolon.
    pub fn boolean_texts(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "boolean_texts",
            "BooleanTexts",
            Value::Str(default.to_string()),
            Category::Data,
            PropKind::String,
        )
    }

    /// Key-text pairs separated by semicolon for value substitution.
    pub fn value_map(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "value_map",
            "ValueMap",
            Value::Str(default.to_string()),
            Category::Data,
            PropKind::String,
        )
    }

    /// How a data cell is drawn.
    pub fn renderer(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "renderer",
            "Renderer",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(&[
                "crText",
                "crCheckBox",
                "crProgressBar",
                "crLink",
                "crColourSwatch",
            ]),
        )
        .with_options(&[
            "crText",
            "crCheckBox",
            "crProgressBar",
            "crLink",
            "crColourSwatch",
        ])
    }

    /// Header text alignment independent of column cell alignment.
    pub fn title_alignment(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "title_alignment",
            "TitleAlignment",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(&["taLeftJustify", "taCenter", "taRightJustify"]),
        )
        .with_options(&["taLeftJustify", "taCenter", "taRightJustify"])
    }

    /// Minimum column width for free columns.
    pub fn min_width(default: i64) -> PropDesc {
        Self::appearance_extent("min_width", "MinWidth", default)
    }

    /// Maximum column width for free columns.
    pub fn max_width(default: i64) -> PropDesc {
        Self::appearance_extent("max_width", "MaxWidth", default)
    }

    /// Cell text colour.
    pub fn text_color(default: crawrad_ir::Color) -> PropDesc {
        Self::appearance_color("text_color", "TextColor", default)
    }

    /// Header cell background colour; transparent = the Grid's.
    pub fn title_color(default: crawrad_ir::Color) -> PropDesc {
        Self::appearance_color("title_color", "TitleColor", default)
    }

    /// Header text font independent of column cell font.
    pub fn title_font(default: impl Into<crawrad_ir::Font>) -> PropDesc {
        PropDesc::new_typed(
            "title_font",
            "TitleFont",
            Value::Font(default.into()),
            Category::Appearance,
            PropKind::Font,
        )
    }

    /// Data text vertical position in the row.
    pub fn vertical_alignment(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "vertical_alignment",
            "VerticalAlignment",
            Value::Enum(default.into()),
            Category::Appearance,
            PropKind::Enum(&["tlTop", "tlCenter", "tlBottom"]),
        )
        .with_options(&["tlTop", "tlCenter", "tlBottom"])
    }

    /// Inner space of the data cells.
    pub fn padding(default: Edges) -> PropDesc {
        PropDesc::new_typed(
            "padding",
            "Padding",
            Value::Edges(default),
            Category::Appearance,
            PropKind::Edges,
        )
    }

    /// Truncate overflowing cell text with an ellipsis symbol.
    pub fn ellipsis(default: bool) -> PropDesc {
        Self::appearance_bool("ellipsis", "Ellipsis", default)
    }

    /// Wrap header title text across multiple lines.
    pub fn title_word_wrap(default: bool) -> PropDesc {
        Self::appearance_bool("title_word_wrap", "TitleWordWrap", default)
    }

    /// Tooltip shown when the pointer rests on this column's header.
    pub fn title_hint(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "title_hint",
            "TitleHint",
            Value::Str(default.to_string()),
            Category::Appearance,
            PropKind::String,
        )
    }

    /// Tooltip with the cell's full text when overflowing.
    pub fn cell_hint(default: bool) -> PropDesc {
        Self::appearance_bool("cell_hint", "CellHint", default)
    }

    /// Whether the user can resize this column at runtime.
    pub fn resizable(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "resizable",
            "Resizable",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    /// Whether the user can drag and drop this column header to reorder at runtime.
    pub fn movable(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "movable",
            "Movable",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    /// Database type of the bound column (read-only).
    pub fn field_type(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "field_type",
            "FieldType",
            Value::Str(default.to_string()),
            Category::Data,
            PropKind::String,
        )
        .read_only()
    }

    /// An Appearance boolean.
    fn appearance_bool(key: &'static str, label: &'static str, default: bool) -> PropDesc {
        PropDesc::new_typed(
            key,
            label,
            Value::Bool(default),
            Category::Appearance,
            PropKind::Bool,
        )
    }

    /// An Appearance colour.
    fn appearance_color(
        key: &'static str,
        label: &'static str,
        default: crawrad_ir::Color,
    ) -> PropDesc {
        PropDesc::new_typed(
            key,
            label,
            Value::Color(default),
            Category::Appearance,
            PropKind::Color,
        )
    }

    /// An Appearance size in pixels, 0..=10,000.
    fn appearance_extent(key: &'static str, label: &'static str, default: i64) -> PropDesc {
        PropDesc::new_typed(
            key,
            label,
            Value::Int(default),
            Category::Appearance,
            PropKind::Int {
                min: 0,
                max: 10_000,
            },
        )
    }

    /// Whether a header click sorts by this column.
    pub fn sortable(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "sortable",
            "Sortable",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    /// The sort direction the grid opens with.
    pub fn sort_order(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "sort_order",
            "SortOrder",
            Value::Enum(default.into()),
            Category::Behavior,
            PropKind::Enum(&["soNone", "soAsc", "soDesc"]),
        )
        .with_options(&["soNone", "soAsc", "soDesc"])
    }

    /// Sort precedence among columns with a SortOrder (lower comes first).
    pub fn sort_priority(default: i64) -> PropDesc {
        PropDesc::new_typed(
            "sort_priority",
            "SortPriority",
            Value::Int(default),
            Category::Behavior,
            PropKind::Int {
                min: 0,
                max: 10_000,
            },
        )
    }

    /// Where NULLs go when sorting by this column.
    pub fn nulls_position(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "nulls_position",
            "NullsPosition",
            Value::Enum(default.into()),
            Category::Behavior,
            PropKind::Enum(&["npLast", "npFirst"]),
        )
        .with_options(&["npLast", "npFirst"])
    }

    /// Whether dropping this header on the "Group by" bar groups by this column.
    pub fn groupable(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "groupable",
            "Groupable",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    /// A grand total of this column in a footer row under the data.
    pub fn summary(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "summary",
            "Summary",
            Value::Enum(default.into()),
            Category::Data,
            PropKind::Enum(SUMMARY_OPTIONS),
        )
        .with_options(SUMMARY_OPTIONS)
    }

    /// The mask for the footer value; empty = the column's DisplayFormat.
    pub fn summary_format(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "summary_format",
            "SummaryFormat",
            Value::Str(default.to_string()),
            Category::Data,
            PropKind::String,
        )
    }

    /// The total of this column shown on each group line.
    pub fn group_summary(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "group_summary",
            "GroupSummary",
            Value::Enum(default.into()),
            Category::Data,
            PropKind::Enum(SUMMARY_OPTIONS),
        )
        .with_options(SUMMARY_OPTIONS)
    }

    /// Whether the user can filter by this column using the filter row.
    pub fn filterable(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "filterable",
            "Filterable",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    /// The kind of editor shown in the filter row for this column.
    pub fn filter_kind(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "filter_kind",
            "FilterKind",
            Value::Enum(default.into()),
            Category::Behavior,
            PropKind::Enum(FILTER_KIND_OPTIONS),
        )
        .with_options(FILTER_KIND_OPTIONS)
    }

    /// Whether this column requires a non-empty value.
    pub fn required(default: bool) -> PropDesc {
        PropDesc::new_typed(
            "required",
            "Required",
            Value::Bool(default),
            Category::Behavior,
            PropKind::Bool,
        )
    }

    /// Default value for a new row in this column.
    pub fn default_value(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "default_value",
            "DefaultValue",
            Value::Str(default.to_string()),
            Category::Behavior,
            PropKind::String,
        )
    }

    /// The kind of in-cell editor used for this column.
    pub fn editor(default: &'static str) -> PropDesc {
        PropDesc::new_typed(
            "editor",
            "Editor",
            Value::Enum(default.into()),
            Category::Behavior,
            PropKind::Enum(COLUMN_EDITOR_OPTIONS),
        )
        .with_options(COLUMN_EDITOR_OPTIONS)
    }

    /// Predefined options for a list editor (one value per line).
    pub fn pick_list(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "pick_list",
            "PickList",
            Value::Str(default.to_string()),
            Category::Behavior,
            PropKind::Multiline,
        )
        .multiline()
    }

    /// Data source referenced for lookup editing.
    pub fn lookup_source() -> PropDesc {
        PropDesc::new_typed(
            "lookup_source",
            "LookupSource",
            Value::Ref(String::new()),
            Category::Data,
            PropKind::Reference("DbDataset"),
        )
        .references("DbDataset")
    }

    /// Key column in the lookup dataset.
    pub fn lookup_key(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "lookup_key",
            "LookupKey",
            Value::Str(default.to_string()),
            Category::Data,
            PropKind::FieldOf("lookup_source"),
        )
    }

    /// Display column(s) in the lookup dataset separated by semicolon.
    pub fn lookup_display(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "lookup_display",
            "LookupDisplay",
            Value::Str(default.to_string()),
            Category::Data,
            PropKind::String,
        )
    }

    /// When expression for a style rule.
    pub fn when(default: &str) -> PropDesc {
        PropDesc::new_typed(
            "when",
            "When",
            Value::Str(default.to_string()),
            Category::Behavior,
            PropKind::String,
        )
    }

    /// Font style flags (Bold, Italic) for a style rule.
    pub fn font_style() -> PropDesc {
        PropDesc::new_typed(
            "font_style",
            "FontStyle",
            Value::Map(
                [
                    ("Bold".to_string(), Value::Bool(false)),
                    ("Italic".to_string(), Value::Bool(false)),
                ]
                .into_iter()
                .collect(),
            ),
            Category::Appearance,
            PropKind::Set(FONT_STYLE_FLAGS),
        )
    }
}
