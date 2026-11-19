//! The typed columns of a `Grid`, as the document holds them.
//!
//! Only the vocabulary lives here, beside the document: the Designer canvas
//! reads it to draw a bound grid and the generators write it into the app,
//! where `crawrad_runtime::grid` turns it into the columns shown.

use crate::{Color, Document, Edges, Font, Node, Value};
use crawrad_note::note;

#[note("grid-columns-columnspec")]
#[derive(Clone, Debug, PartialEq)]
pub struct ColumnSpec {
    pub field: String,
    pub title: String,
    pub width: i64,
    pub align: String,
    pub display_format: String,
    pub visible: bool,
    pub null_text: String,
    pub boolean_texts: String,
    pub value_map: String,
    pub renderer: String,
    pub title_alignment: String,
    pub min_width: i64,
    pub max_width: i64,
    pub color: Color,
    pub text_color: Color,
    pub title_color: Color,
    pub vertical_alignment: String,
    pub padding: Edges,
    pub ellipsis: bool,
    pub word_wrap: bool,
    pub title_word_wrap: bool,
    pub title_hint: String,
    pub cell_hint: bool,
    pub title_font: Font,
    pub font: Font,
    pub auto_size: bool,
    pub resizable: bool,
    pub movable: bool,
    pub sortable: bool,
    pub sort_order: String,
    pub sort_priority: i64,
    pub nulls_position: String,
    pub groupable: bool,
    pub group_index: i64,
    pub summary: String,
    pub summary_format: String,
    pub group_summary: String,
    pub filterable: bool,
    pub filter_kind: String,
    pub read_only: bool,
    pub required: bool,
    pub default_value: String,
    pub max_length: i64,
    pub min_value: i64,
    pub max_value: i64,
    pub tab_stop: bool,
    pub editor: String,
    pub pick_list: Vec<String>,
    pub edit_mask: String,
    pub get_text_hook: bool,
    pub set_text_hook: bool,
    pub validate_hook: bool,
    pub lookup_key: String,
    pub lookup_display: Vec<String>,
    pub button_hook: bool,
    /// How many rows the column's open list shows before it scrolls.
    pub drop_down_count: i64,
    /// The design FieldType (`DATE`, `INTEGER`…); `UNKNOWN` when nobody said.
    pub field_type: String,
    pub draw_cell_hook: bool,
    pub style_rules: Vec<StyleRuleSpec>,
}

/// Specification of a conditional style rule for a grid column.
#[derive(Clone, Debug, PartialEq)]
pub struct StyleRuleSpec {
    pub when: String,
    pub color: Color,
    pub text_color: Color,
    pub bold: bool,
    pub italic: bool,
}

impl ColumnSpec {
    #[note("grid-columns-columnspec-of-grid")]
    pub fn of_grid(doc: &Document, grid: &Node) -> Vec<ColumnSpec> {
        let text = |n: &Node, key: &str| match n.props.get(key) {
            Some(Value::Str(s) | Value::Enum(s)) => s.clone(),
            _ => String::new(),
        };
        let int_prop = |n: &Node, key: &str| match n.props.get(key) {
            Some(Value::Int(w)) => *w,
            _ => 0,
        };
        let color_prop = |n: &Node, key: &str| match n.props.get(key) {
            Some(Value::Color(c)) => *c,
            _ => Color::transparent(),
        };
        let font_prop = |n: &Node, key: &str| match n.props.get(key) {
            Some(Value::Font(f)) => f.clone(),
            _ => Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
        };
        let edges_prop = |n: &Node, key: &str| match n.props.get(key) {
            Some(Value::Edges(e)) => *e,
            _ => Edges::uniform(0.0),
        };
        let bool_prop = |n: &Node, key: &str| matches!(n.props.get(key), Some(Value::Bool(true)));
        grid.children
            .iter()
            .filter_map(|id| doc.node(id))
            .filter(|n| n.class == GRID_COLUMN_CLASS)
            .map(|n| ColumnSpec {
                field: text(n, "field"),
                title: text(n, "title"),
                width: int_prop(n, "width"),
                align: text(n, "alignment"),
                display_format: text(n, "display_format"),
                visible: !matches!(n.props.get("visible"), Some(Value::Bool(false))),
                null_text: text(n, "null_text"),
                boolean_texts: text(n, "boolean_texts"),
                value_map: text(n, "value_map"),
                renderer: match n.props.get("renderer") {
                    Some(Value::Str(s) | Value::Enum(s)) if !s.is_empty() => s.clone(),
                    _ => "crText".to_string(),
                },
                title_alignment: match n.props.get("title_alignment") {
                    Some(Value::Str(s) | Value::Enum(s)) if !s.is_empty() => s.clone(),
                    _ => "taLeftJustify".to_string(),
                },
                min_width: int_prop(n, "min_width"),
                max_width: int_prop(n, "max_width"),
                color: color_prop(n, "color"),
                text_color: color_prop(n, "text_color"),
                title_color: color_prop(n, "title_color"),
                vertical_alignment: match n.props.get("vertical_alignment") {
                    Some(Value::Str(s) | Value::Enum(s)) if !s.is_empty() => s.clone(),
                    _ => "tlCenter".to_string(),
                },
                padding: edges_prop(n, "padding"),
                ellipsis: bool_prop(n, "ellipsis"),
                word_wrap: bool_prop(n, "word_wrap"),
                title_word_wrap: bool_prop(n, "title_word_wrap"),
                title_hint: text(n, "title_hint"),
                cell_hint: bool_prop(n, "cell_hint"),
                title_font: font_prop(n, "title_font"),
                font: font_prop(n, "font"),
                auto_size: bool_prop(n, "auto_size"),
                resizable: !matches!(n.props.get("resizable"), Some(Value::Bool(false))),
                movable: !matches!(n.props.get("movable"), Some(Value::Bool(false))),
                sortable: !matches!(n.props.get("sortable"), Some(Value::Bool(false))),
                sort_order: match n.props.get("sort_order") {
                    Some(Value::Str(s) | Value::Enum(s)) if !s.is_empty() => s.clone(),
                    _ => "soNone".to_string(),
                },
                sort_priority: int_prop(n, "sort_priority"),
                nulls_position: match n.props.get("nulls_position") {
                    Some(Value::Str(s) | Value::Enum(s)) if !s.is_empty() => s.clone(),
                    _ => "npLast".to_string(),
                },
                groupable: !matches!(n.props.get("groupable"), Some(Value::Bool(false))),
                group_index: match n.props.get("group_index") {
                    Some(Value::Int(v)) => *v,
                    _ => 0,
                },
                summary: match n.props.get("summary") {
                    Some(Value::Str(s) | Value::Enum(s)) if !s.is_empty() => s.clone(),
                    _ => "smNone".to_string(),
                },
                summary_format: text(n, "summary_format"),
                group_summary: match n.props.get("group_summary") {
                    Some(Value::Str(s) | Value::Enum(s)) if !s.is_empty() => s.clone(),
                    _ => "smNone".to_string(),
                },
                filterable: bool_prop(n, "filterable"),
                filter_kind: match n.props.get("filter_kind") {
                    Some(Value::Str(s) | Value::Enum(s)) if !s.is_empty() => s.clone(),
                    _ => "fkAuto".to_string(),
                },
                read_only: bool_prop(n, "readonly"),
                required: bool_prop(n, "required"),
                default_value: text(n, "default_value"),
                max_length: int_prop(n, "max_length"),
                min_value: int_prop(n, "min_value"),
                max_value: int_prop(n, "max_value"),
                tab_stop: !matches!(n.props.get("tab_stop"), Some(Value::Bool(false))),
                editor: match n.props.get("editor") {
                    Some(Value::Str(s) | Value::Enum(s)) if !s.is_empty() => s.clone(),
                    _ => "edAuto".to_string(),
                },
                pick_list: match n.props.get("pick_list") {
                    Some(Value::Str(s) | Value::Enum(s)) => s
                        .lines()
                        .map(str::trim)
                        .filter(|line| !line.is_empty())
                        .map(str::to_string)
                        .collect(),
                    _ => Vec::new(),
                },
                edit_mask: text(n, "edit_mask"),
                get_text_hook: n
                    .events
                    .get("on_get_text")
                    .is_some_and(|h| !h.trim().is_empty()),
                set_text_hook: n
                    .events
                    .get("on_set_text")
                    .is_some_and(|h| !h.trim().is_empty()),
                validate_hook: n
                    .events
                    .get("on_validate")
                    .is_some_and(|h| !h.trim().is_empty()),
                lookup_key: text(n, "lookup_key"),
                lookup_display: match n.props.get("lookup_display") {
                    Some(Value::Str(s) | Value::Enum(s)) => s
                        .split(';')
                        .map(str::trim)
                        .filter(|part| !part.is_empty())
                        .map(str::to_string)
                        .collect(),
                    _ => Vec::new(),
                },
                button_hook: n
                    .events
                    .get("on_button_click")
                    .is_some_and(|h| !h.trim().is_empty()),
                drop_down_count: match int_prop(n, "drop_down_count") {
                    0 => 8,
                    count => count,
                },
                field_type: match n.props.get("field_type") {
                    Some(Value::Str(s) | Value::Enum(s)) if !s.is_empty() => s.clone(),
                    _ => "UNKNOWN".to_string(),
                },
                draw_cell_hook: n
                    .events
                    .get("on_draw_cell")
                    .is_some_and(|h| !h.trim().is_empty()),
                style_rules: n
                    .children
                    .iter()
                    .filter_map(|id| doc.node(id))
                    .filter(|child| child.class == "StyleRule")
                    .map(|child| {
                        let (bold, italic) = match child.props.get("font_style") {
                            Some(Value::Map(m)) => (
                                matches!(m.get("Bold"), Some(Value::Bool(true))),
                                matches!(m.get("Italic"), Some(Value::Bool(true))),
                            ),
                            _ => (false, false),
                        };
                        StyleRuleSpec {
                            when: text(child, "when"),
                            color: color_prop(child, "color"),
                            text_color: color_prop(child, "text_color"),
                            bold,
                            italic,
                        }
                    })
                    .collect(),
            })
            .collect()
    }
}

#[note("grid-columns-grid-column-class")]
pub const GRID_COLUMN_CLASS: &str = "GridColumn";

#[note("grid-columns-title-from-field")]
pub fn title_from_field(field: &str) -> String {
    let spaced = field.replace('_', " ");
    let mut chars = spaced.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}
