//! # Headless Screen Model for the Visual Grid
//!
//! Why this module exists:
//! One headless model, two runtimes (Makepad and egui) paint it. Every visual decision
//! (texts, arrows, formatting, opacity, anti-flicker timing) is here and tested,
//! ensuring both runtimes render identical strings, numbers, and states without duplicating logic.

use crawrad_note::note;

#[note("grid-columnalign")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColumnAlign {
    Left,
    Center,
    Right,
}

impl ColumnAlign {
    #[note("grid-columnalign-from-catalogue")]
    pub fn from_catalogue(s: &str) -> Self {
        match s {
            "taCenter" => Self::Center,
            "taRightJustify" => Self::Right,
            _ => Self::Left,
        }
    }
}

/// Vertical alignment of cell text within a grid row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColumnVAlign {
    Top,
    Center,
    Bottom,
}

impl ColumnVAlign {
    /// Maps catalogue alignment strings (`tlTop`, `tlCenter`, `tlBottom`) to `ColumnVAlign`.
    pub fn from_catalogue(s: &str) -> Self {
        match s {
            "tlTop" => Self::Top,
            "tlBottom" => Self::Bottom,
            _ => Self::Center,
        }
    }
}

/// How a data cell is drawn in the visual grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CellRenderer {
    #[default]
    Text,
    CheckBox,
    ProgressBar,
    Link,
    ColourSwatch,
}

impl CellRenderer {
    /// Maps catalogue renderer strings to `CellRenderer` (unknown defaults to `Text`).
    pub fn from_key(s: &str) -> Self {
        match s {
            "crCheckBox" => Self::CheckBox,
            "crProgressBar" => Self::ProgressBar,
            "crLink" => Self::Link,
            "crColourSwatch" => Self::ColourSwatch,
            _ => Self::Text,
        }
    }

    /// Key of the renderer in the component catalogue.
    pub fn to_key(self) -> &'static str {
        match self {
            Self::Text => "crText",
            Self::CheckBox => "crCheckBox",
            Self::ProgressBar => "crProgressBar",
            Self::Link => "crLink",
            Self::ColourSwatch => "crColourSwatch",
        }
    }
}

/// Maps a raw value using key=text pairs separated by semicolon.
///
/// Returns `Some(text)` if `raw` matches a key (exact match, trimmed).
pub fn map_value<'a>(map: &'a str, raw: &str) -> Option<&'a str> {
    if map.trim().is_empty() {
        return None;
    }
    let raw_trimmed = raw.trim();
    for pair in map.split(';') {
        let pair = pair.trim();
        if let Some((k, v)) = pair.split_once('=')
            && k.trim() == raw_trimmed
        {
            return Some(v.trim());
        }
    }
    None
}

/// Determines boolean checkbox state from a cell's raw string value.
///
/// Returns `Some(true)` for "1" / "true" (case-insensitive),
/// `Some(false)` for "0" / "false" (case-insensitive),
/// and `None` for any other value.
pub fn checkbox_state(raw: &str) -> Option<bool> {
    let s = raw.trim();
    if s == "1" || s.eq_ignore_ascii_case("true") {
        Some(true)
    } else if s == "0" || s.eq_ignore_ascii_case("false") {
        Some(false)
    } else {
        None
    }
}

/// Computes progress fraction (0.0..=1.0) from a raw value clamped to 0..=100.
///
/// Returns `None` if `raw` is non-numeric, NaN, or infinite.
pub fn progress_fraction(raw: &str) -> Option<f64> {
    let val = raw.trim().parse::<f64>().ok()?;
    if val.is_nan() || val.is_infinite() {
        return None;
    }
    let clamped = val.clamp(0.0, 100.0);
    Some(clamped / 100.0)
}

/// Parses a hex colour string in `#rrggbb` or `#rrggbbaa` format.
pub fn parse_hex_color(raw: &str) -> Option<Color> {
    let trimmed = raw.trim();
    if !trimmed.starts_with('#') {
        return None;
    }
    Color::from_hex(trimmed)
}

pub use crawrad_ir::grid_columns::{
    ColumnSpec, GRID_COLUMN_CLASS, StyleRuleSpec, title_from_field,
};
pub use crawrad_ir::style_rule;
pub use crawrad_ir::{Color, Edges, Font};

/// The widths of a grid with no designed columns: `width` shared by how long the
/// longest word of each column is (title or cell, 4 to 24 characters). The Designer
/// and the running app use this same rule, so the columns look alike in both.
#[note("grid-text-share-widths")]
pub fn text_share_widths<'a>(
    titles: &[&str],
    cells: impl Fn(usize) -> Vec<&'a str>,
    width: f64,
) -> Vec<f64> {
    let weights: Vec<f64> = titles
        .iter()
        .enumerate()
        .map(|(col, title)| {
            std::iter::once(*title)
                .chain(cells(col))
                .map(|t| t.chars().count())
                .max()
                .unwrap_or(0)
                .clamp(4, 24) as f64
        })
        .collect();
    let sum: f64 = weights.iter().sum::<f64>().max(1.0);
    weights.iter().map(|w| width * w / sum).collect()
}

/// Shares available width among columns, clamping free columns to their [min, max] bounds
/// and redistributing any freed or taken space among the remaining free columns.
pub fn share_widths_clamped(
    available: f64,
    widths: &[Option<f64>],
    bounds: &[(f64, Option<f64>)],
) -> Vec<f64> {
    let mut result = vec![0.0; widths.len()];
    let mut fixed_sum = 0.0;
    let mut active_indices = Vec::new();

    for (i, &w) in widths.iter().enumerate() {
        if let Some(fixed) = w {
            let val = fixed.max(0.0);
            result[i] = val;
            fixed_sum += val;
        } else {
            active_indices.push(i);
        }
    }

    let mut remaining = (available - fixed_sum).max(0.0);

    while !active_indices.is_empty() {
        let share = remaining / active_indices.len() as f64;
        let mut violations = Vec::new();

        for (idx, &col_i) in active_indices.iter().enumerate() {
            let (min, max) = bounds.get(col_i).copied().unwrap_or((0.0, None));
            let min = min.max(0.0);
            if share < min {
                violations.push((idx, col_i, min));
            } else if let Some(max_val) = max {
                let max_val = max_val.max(min);
                if share > max_val {
                    violations.push((idx, col_i, max_val));
                }
            }
        }

        if violations.is_empty() {
            for &col_i in &active_indices {
                result[col_i] = share;
            }
            break;
        }
        // One direction per pass: a column raised to its minimum shrinks the
        // others' share, so a maximum seen in the same pass may no longer hold.
        if violations.iter().any(|&(_, _, w)| w > share) {
            violations.retain(|&(_, _, w)| w > share);
        }

        for &(idx_in_active, col_i, clamped_w) in violations.iter().rev() {
            result[col_i] = clamped_w;
            remaining = (remaining - clamped_w).max(0.0);
            active_indices.remove(idx_in_active);
        }
    }

    result
}

/// Computes column widths for a slice of `GridColumn`, clamping free columns.
pub fn share_column_widths(available: f64, columns: &[GridColumn]) -> Vec<f64> {
    let asked: Vec<Option<f64>> = columns.iter().map(|c| c.width.map(f64::from)).collect();
    let bounds: Vec<(f64, Option<f64>)> = columns
        .iter()
        .map(|c| (c.min_width as f64, c.max_width.map(f64::from)))
        .collect();
    share_widths_clamped(available, &asked, &bounds)
}

#[note("grid-share-widths")]
pub fn share_widths(available: f64, widths: &[Option<f64>]) -> Vec<f64> {
    share_widths_clamped(available, widths, &[])
}

/// Pure calculation of a column's auto-width: widest text (title or visible cell) + padding, clamped to [min, max].
pub fn auto_width(
    title: &str,
    cells: &[&str],
    padding: f64,
    min: f64,
    max: Option<f64>,
    mut width_of: impl FnMut(&str) -> f64,
) -> f64 {
    let mut widest = width_of(title);
    for &c in cells {
        let w = width_of(c);
        if w > widest {
            widest = w;
        }
    }
    let total = widest + padding;
    let min_val = min.max(0.0);
    let mut clamped = total.max(min_val);
    if let Some(max_val) = max {
        let max_val = max_val.max(min_val);
        clamped = clamped.min(max_val);
    }
    clamped
}

/// Moves an item at `from` to `to` in `order`.
pub fn move_column(order: &mut Vec<usize>, from: usize, to: usize) {
    if from < order.len() && to < order.len() && from != to {
        let item = order.remove(from);
        order.insert(to, item);
    }
}

/// Clamps a resized column width to at least 24 px and within [min, max] bounds.
pub fn clamp_column_width(width: f64, min: f64, max: Option<f64>) -> f64 {
    let lower = min.max(24.0);
    let mut clamped = width.max(lower);
    if let Some(upper) = max {
        clamped = clamped.min(upper.max(lower));
    }
    clamped
}

pub use clamp_column_width as clamp_resize;

/// Fits text within a given maximum width, optionally truncating with an ellipsis ('…').
pub fn fit_text(
    text: &str,
    max: f64,
    ellipsis: bool,
    mut width_of: impl FnMut(&str) -> f64,
) -> String {
    if width_of(text) <= max {
        return text.to_string();
    }
    if !ellipsis {
        let mut end = 0;
        for (i, ch) in text.char_indices() {
            let next = i + ch.len_utf8();
            if width_of(&text[..next]) > max {
                break;
            }
            end = next;
        }
        return text[..end].to_string();
    }
    const ELLIPSIS: &str = "…";
    if width_of(ELLIPSIS) > max {
        return String::new();
    }
    let mut candidate = String::with_capacity(text.len() + ELLIPSIS.len());
    let mut last_fit = 0;
    let mut any_fit = false;
    for (i, ch) in text.char_indices() {
        let next = i + ch.len_utf8();
        candidate.clear();
        candidate.push_str(&text[..next]);
        candidate.push_str(ELLIPSIS);
        if width_of(&candidate) <= max {
            last_fit = next;
            any_fit = true;
        } else {
            break;
        }
    }
    if any_fit {
        format!("{}{}", &text[..last_fit], ELLIPSIS)
    } else {
        ELLIPSIS.to_string()
    }
}

/// Wraps text inside a given width across at most `max_lines` lines.
///
/// Breaks at word spaces. If a single word is wider than `max_width`, it is cut by `fit_text`.
/// The last line fits remaining text using `fit_text`, adding an ellipsis if `ellipsis` is enabled.
pub fn wrap_text(
    text: &str,
    max_width: f64,
    max_lines: usize,
    ellipsis: bool,
    mut width_of: impl FnMut(&str) -> f64,
) -> Vec<String> {
    if text.is_empty() || max_width <= 0.0 {
        return Vec::new();
    }
    let max_lines = max_lines.max(1);
    let mut words: std::collections::VecDeque<String> =
        text.split_whitespace().map(str::to_string).collect();
    let mut lines = Vec::new();
    while !words.is_empty() {
        if lines.len() + 1 == max_lines {
            let remainder = Vec::from(words).join(" ");
            let last_line = fit_text(&remainder, max_width, ellipsis, &mut width_of);
            // A cut that ends on a space leaves it out.
            let last_line = last_line.trim_end();
            if !last_line.is_empty() {
                lines.push(last_line.to_string());
            }
            break;
        }
        let mut current_line = String::new();
        while let Some(word) = words.front() {
            if current_line.is_empty() && width_of(word) > max_width {
                // A word wider than the box is cut; the rest starts the next line.
                let cut = fit_text(word, max_width, false, &mut width_of);
                if cut.is_empty() {
                    words.pop_front();
                    break;
                }
                let rest = word[cut.len()..].to_string();
                words.pop_front();
                if !rest.is_empty() {
                    words.push_front(rest);
                }
                current_line = cut;
                break;
            }
            let candidate = if current_line.is_empty() {
                word.clone()
            } else {
                format!("{current_line} {word}")
            };
            if width_of(&candidate) > max_width {
                break;
            }
            current_line = candidate;
            words.pop_front();
        }
        if current_line.is_empty() {
            break;
        }
        lines.push(current_line);
    }
    lines
}

#[note("grid-gridcolumn")]
#[derive(Clone, Debug, PartialEq)]
pub struct GridColumn {
    pub name: String,
    pub title: String,
    pub align: ColumnAlign,
    /// The field's place in the source row; `None` reads the cell at the column's own place.
    pub source: Option<usize>,
    pub width: Option<f32>,
    pub display_format: String,
    pub title_align: ColumnAlign,
    pub min_width: f32,
    pub max_width: Option<f32>,
    pub null_text: String,
    pub boolean_texts: String,
    pub value_map: String,
    pub renderer: CellRenderer,
    pub color: Color,
    pub text_color: Color,
    pub title_color: Color,
    pub valign: ColumnVAlign,
    pub padding: Option<Edges>,
    pub kind: Option<String>,
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
    /// How many rows the open list shows before it scrolls.
    pub drop_down_count: usize,
    pub lookup: LookupHandle,
    pub draw_cell_hook: bool,
    pub style_rules: Vec<StyleRule>,
}

/// What the hooks saw of the visible rows: each row's position, id and a hash of its cells,
/// so an edited value asks OnGetText / the style rules again even when the row did not move.
fn screen_signature(screen: &datagrid::Screen) -> Vec<(usize, u64, u64)> {
    use std::hash::{Hash, Hasher};
    screen
        .lines
        .iter()
        .filter_map(|l| match l {
            datagrid::ScreenLine::Row { index, id, cells } => {
                let mut h = std::collections::hash_map::DefaultHasher::new();
                cells.hash(&mut h);
                Some((*index, *id, h.finish()))
            }
            _ => None,
        })
        .collect()
}

/// A conditional style rule for a grid column.
#[derive(Clone, Debug, PartialEq)]
pub struct StyleRule {
    pub when: crawrad_ir::style_rule::Expr,
    pub color: Option<Color>,
    pub text_color: Option<Color>,
    pub bold: bool,
    pub italic: bool,
}

/// Builds runtime `StyleRule`s from column design specs.
pub fn style_rules_from_specs(specs: &[crawrad_ir::grid_columns::StyleRuleSpec]) -> Vec<StyleRule> {
    specs
        .iter()
        .filter_map(|r| {
            let when = crawrad_ir::style_rule::parse(&r.when).ok()?;
            Some(StyleRule {
                when,
                color: (r.color.a > 0).then_some(r.color),
                text_color: (r.text_color.a > 0).then_some(r.text_color),
                bold: r.bold,
                italic: r.italic,
            })
        })
        .collect()
}

/// The lookup dataset of an edLookup column. Compared by identity and printed
/// as present/absent, so `GridColumn` keeps its derives.
#[derive(Clone, Default)]
pub struct LookupHandle(pub Option<std::sync::Arc<dyn datagrid::LookupSource>>);

impl LookupHandle {
    /// Wraps a lookup source.
    pub fn new(source: impl datagrid::LookupSource + 'static) -> Self {
        Self(Some(std::sync::Arc::new(source)))
    }
}

impl PartialEq for LookupHandle {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Some(a), Some(b)) => std::sync::Arc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        }
    }
}

impl std::fmt::Debug for LookupHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.0.is_some() {
            "LookupHandle(Some)"
        } else {
            "LookupHandle(None)"
        })
    }
}

impl GridColumn {
    /// The title the designer shows: with the sort arrow of its SortOrder, as the
    /// running grid shows it once that sort is applied.
    pub fn design_title(&self, marks: &Marks) -> String {
        let arrow = match self.sort_order.as_str() {
            "soAsc" => marks.ascending,
            "soDesc" => marks.descending,
            _ => "",
        };
        format!("{}{arrow}", self.title)
    }

    #[note("grid-gridcolumn-new")]
    pub fn new(name: impl Into<String>, title: impl Into<String>, align: ColumnAlign) -> Self {
        Self {
            name: name.into(),
            title: title.into(),
            align,
            source: None,
            width: None,
            display_format: String::new(),
            title_align: align,
            min_width: 0.0,
            max_width: None,
            null_text: String::new(),
            boolean_texts: String::new(),
            value_map: String::new(),
            renderer: CellRenderer::Text,
            color: Color::transparent(),
            text_color: Color::transparent(),
            title_color: Color::transparent(),
            valign: ColumnVAlign::Center,
            padding: None,
            kind: None,
            ellipsis: false,
            word_wrap: false,
            title_word_wrap: false,
            title_hint: String::new(),
            cell_hint: false,
            title_font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            font: Font {
                name: String::new(),
                size: 0.0,
                ..Default::default()
            },
            auto_size: false,
            resizable: true,
            movable: true,
            sortable: true,
            sort_order: "soNone".to_string(),
            sort_priority: 0,
            nulls_position: "npLast".to_string(),
            groupable: true,
            group_index: 0,
            summary: "smNone".to_string(),
            summary_format: String::new(),
            group_summary: "smNone".to_string(),
            filterable: false,
            filter_kind: "fkAuto".to_string(),
            read_only: false,
            required: false,
            default_value: String::new(),
            max_length: 0,
            min_value: 0,
            max_value: 0,
            tab_stop: true,
            editor: "edAuto".to_string(),
            pick_list: Vec::new(),
            edit_mask: String::new(),
            get_text_hook: false,
            set_text_hook: false,
            validate_hook: false,
            lookup_key: String::new(),
            lookup_display: Vec::new(),
            button_hook: false,
            drop_down_count: 8,
            lookup: LookupHandle::default(),
            draw_cell_hook: false,
            style_rules: Vec::new(),
        }
    }

    #[note("grid-gridcolumn-from-schema")]
    pub fn from_schema(columns: &[datagrid::ColumnInfo]) -> Vec<GridColumn> {
        Self::from_schema_and_design(columns, &[])
    }

    #[note("grid-gridcolumn-from-design")]
    pub fn from_design(
        names: &[String],
        numeric: &[bool],
        specs: &[ColumnSpec],
    ) -> Vec<GridColumn> {
        let is_numeric = |i: usize| numeric.get(i).copied().unwrap_or(false);
        if specs.is_empty() {
            return names
                .iter()
                .enumerate()
                .map(|(i, name)| GridColumn {
                    name: name.clone(),
                    title: title_from_field(name),
                    align: if is_numeric(i) {
                        ColumnAlign::Right
                    } else {
                        ColumnAlign::Left
                    },
                    source: Some(i),
                    width: None,
                    display_format: String::new(),
                    title_align: if is_numeric(i) {
                        ColumnAlign::Right
                    } else {
                        ColumnAlign::Left
                    },
                    min_width: 0.0,
                    max_width: None,
                    null_text: String::new(),
                    boolean_texts: String::new(),
                    value_map: String::new(),
                    renderer: CellRenderer::Text,
                    color: Color::transparent(),
                    text_color: Color::transparent(),
                    title_color: Color::transparent(),
                    valign: ColumnVAlign::Center,
                    padding: None,
                    kind: None,
                    ellipsis: false,
                    word_wrap: false,
                    title_word_wrap: false,
                    title_hint: String::new(),
                    cell_hint: false,
                    title_font: Font {
                        name: String::new(),
                        size: 0.0,
                        ..Default::default()
                    },
                    font: Font {
                        name: String::new(),
                        size: 0.0,
                        ..Default::default()
                    },
                    auto_size: false,
                    resizable: true,
                    movable: true,
                    sortable: true,
                    sort_order: "soNone".to_string(),
                    sort_priority: 0,
                    nulls_position: "npLast".to_string(),
                    groupable: true,
                    group_index: 0,
                    summary: "smNone".to_string(),
                    summary_format: String::new(),
                    group_summary: "smNone".to_string(),
                    filterable: false,
                    filter_kind: "fkAuto".to_string(),
                    read_only: false,
                    required: false,
                    default_value: String::new(),
                    max_length: 0,
                    min_value: 0,
                    max_value: 0,
                    tab_stop: true,
                    editor: "edAuto".to_string(),
                    pick_list: Vec::new(),
                    edit_mask: String::new(),
                    get_text_hook: false,
                    set_text_hook: false,
                    validate_hook: false,
                    lookup_key: String::new(),
                    lookup_display: Vec::new(),
                    button_hook: false,
                    drop_down_count: 8,
                    style_rules: Vec::new(),
                    draw_cell_hook: false,
                    lookup: LookupHandle::default(),
                })
                .collect();
        }
        specs
            .iter()
            .filter(|s| s.visible)
            .filter_map(|s| {
                let source = names.iter().position(|n| n == &s.field)?;
                let padding = if s.padding.left == 0.0
                    && s.padding.top == 0.0
                    && s.padding.right == 0.0
                    && s.padding.bottom == 0.0
                {
                    None
                } else {
                    Some(s.padding)
                };
                Some(GridColumn {
                    name: s.field.clone(),
                    title: if s.title.is_empty() {
                        title_from_field(&s.field)
                    } else {
                        s.title.clone()
                    },
                    align: ColumnAlign::from_catalogue(&s.align),
                    source: Some(source),
                    width: (s.width > 0).then_some(s.width as f32),
                    display_format: s.display_format.clone(),
                    title_align: ColumnAlign::from_catalogue(&s.title_alignment),
                    min_width: s.min_width.max(0) as f32,
                    max_width: (s.max_width > 0).then_some(s.max_width as f32),
                    null_text: s.null_text.clone(),
                    boolean_texts: s.boolean_texts.clone(),
                    value_map: s.value_map.clone(),
                    renderer: CellRenderer::from_key(&s.renderer),
                    color: s.color,
                    text_color: s.text_color,
                    title_color: s.title_color,
                    valign: ColumnVAlign::from_catalogue(&s.vertical_alignment),
                    padding,
                    // the design's FieldType wins: SQLite reports a DATE as text
                    kind: (!s.field_type.is_empty() && s.field_type != "UNKNOWN")
                        .then(|| s.field_type.clone()),
                    ellipsis: s.ellipsis,
                    word_wrap: s.word_wrap,
                    title_word_wrap: s.title_word_wrap,
                    title_hint: s.title_hint.clone(),
                    cell_hint: s.cell_hint,
                    title_font: s.title_font.clone(),
                    font: s.font.clone(),
                    auto_size: s.auto_size,
                    resizable: s.resizable,
                    movable: s.movable,
                    sortable: s.sortable,
                    sort_order: if s.sort_order.is_empty() {
                        "soNone".to_string()
                    } else {
                        s.sort_order.clone()
                    },
                    sort_priority: s.sort_priority,
                    nulls_position: if s.nulls_position.is_empty() {
                        "npLast".to_string()
                    } else {
                        s.nulls_position.clone()
                    },
                    groupable: s.groupable,
                    group_index: s.group_index,
                    summary: if s.summary.is_empty() {
                        "smNone".to_string()
                    } else {
                        s.summary.clone()
                    },
                    summary_format: s.summary_format.clone(),
                    group_summary: if s.group_summary.is_empty() {
                        "smNone".to_string()
                    } else {
                        s.group_summary.clone()
                    },
                    filterable: s.filterable,
                    filter_kind: if s.filter_kind.is_empty() {
                        "fkAuto".to_string()
                    } else {
                        s.filter_kind.clone()
                    },
                    read_only: s.read_only,
                    required: s.required,
                    default_value: s.default_value.clone(),
                    max_length: s.max_length,
                    min_value: s.min_value,
                    max_value: s.max_value,
                    tab_stop: s.tab_stop,
                    editor: if s.editor.is_empty() {
                        "edAuto".to_string()
                    } else {
                        s.editor.clone()
                    },
                    pick_list: s.pick_list.clone(),
                    edit_mask: s.edit_mask.clone(),
                    get_text_hook: s.get_text_hook,
                    set_text_hook: s.set_text_hook,
                    validate_hook: s.validate_hook,
                    lookup_key: s.lookup_key.clone(),
                    lookup_display: s.lookup_display.clone(),
                    button_hook: s.button_hook,
                    drop_down_count: s.drop_down_count.clamp(1, 100) as usize,
                    lookup: LookupHandle::default(),
                    draw_cell_hook: s.draw_cell_hook,
                    style_rules: style_rules_from_specs(&s.style_rules),
                })
            })
            .collect()
    }

    #[note("grid-gridcolumn-from-schema-and-design")]
    pub fn from_schema_and_design(
        columns: &[datagrid::ColumnInfo],
        specs: &[ColumnSpec],
    ) -> Vec<GridColumn> {
        let names: Vec<String> = columns.iter().map(|c| c.name.clone()).collect();
        let numeric: Vec<bool> = columns
            .iter()
            .map(|c| {
                matches!(
                    c.kind,
                    datagrid::ColumnKind::Int | datagrid::ColumnKind::Float
                )
            })
            .collect();
        let mut cols = Self::from_design(&names, &numeric, specs);
        for col in &mut cols {
            if col.kind.is_none()
                && let Some(src) = col.source
                && let Some(info) = columns.get(src)
            {
                col.kind = match info.kind {
                    datagrid::ColumnKind::Bool => Some("BOOLEAN".into()),
                    datagrid::ColumnKind::Int => Some("INTEGER".into()),
                    datagrid::ColumnKind::Float => Some("FLOAT".into()),
                    datagrid::ColumnKind::Text => Some("TEXT".into()),
                };
            }
        }
        cols
    }
}

#[note("grid-marks")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Marks {
    /// After the title of a column sorted ascending, e.g. `" ▲"`.
    pub ascending: &'static str,
    /// After the title of a column sorted descending, e.g. `" ▼"`.
    pub descending: &'static str,
    /// Before the title of an open group line, e.g. `"▼ "`.
    pub group_open: &'static str,
    /// After the caption of an fkList filter box, e.g. `" ▼"`.
    pub drop_down: &'static str,
    /// The hint of an fkText filter box; empty when the font has no magnifier.
    pub search: &'static str,
    /// In the indicator gutter, on the row being edited.
    pub edited_row: &'static str,
    /// In the indicator gutter, on the new row.
    pub new_row: &'static str,
}

impl Default for Marks {
    fn default() -> Self {
        Self {
            ascending: " ▲",
            descending: " ▼",
            group_open: "▼ ",
            drop_down: " ▼",
            // neither IBM Plex nor Noto Sans (the Makepad theme) has U+2315
            search: "",
            edited_row: "✎",
            new_row: "*",
        }
    }
}

/// Initial sort keys derived from column design-time specifications.
///
/// Filters columns whose `sort_order` is not "soNone", sorts by `sort_priority`
/// (lower comes first, preserving column order on ties), and respects `nulls_position`.
pub fn initial_sort(columns: &[GridColumn]) -> Vec<datagrid::SortKey> {
    let mut sortable_cols: Vec<(usize, &GridColumn)> = columns
        .iter()
        .enumerate()
        .filter(|(_, c)| matches!(c.sort_order.as_str(), "soAsc" | "soDesc"))
        .collect();

    sortable_cols.sort_by_key(|(_, c)| c.sort_priority);

    sortable_cols
        .into_iter()
        .map(|(_, c)| {
            let order = match c.sort_order.as_str() {
                "soDesc" => datagrid::Order::Desc,
                _ => datagrid::Order::Asc,
            };
            let nulls = match c.nulls_position.as_str() {
                "npFirst" => datagrid::Nulls::First,
                _ => datagrid::Nulls::Last,
            };
            datagrid::SortKey {
                column: c.name.clone(),
                order,
                nulls,
            }
        })
        .collect()
}

/// Initial grouping hierarchy derived from column design-time specifications.
///
/// The totals every group line carries: one per column with a GroupSummary.
pub fn group_totals(columns: &[GridColumn]) -> Vec<datagrid::Total> {
    columns
        .iter()
        .filter_map(|c| {
            parse_aggregate(&c.group_summary).map(|agg| datagrid::Total {
                column: c.name.clone(),
                aggregate: agg,
            })
        })
        .collect()
}

/// Filters columns with a positive `group_index` (0 means not grouped, as on a
/// SpeedButton) and
/// orders by `group_index` ascending (preserving column order on ties).
pub fn initial_groups(columns: &[GridColumn]) -> Vec<datagrid::GroupLevel> {
    let group_totals = group_totals(columns);

    let mut group_cols: Vec<(usize, &GridColumn)> = columns
        .iter()
        .enumerate()
        .filter(|(_, c)| c.group_index > 0)
        .collect();

    group_cols.sort_by_key(|(_, c)| c.group_index);

    group_cols
        .into_iter()
        .map(|(_, c)| datagrid::GroupLevel {
            column: c.name.clone(),
            totals: group_totals.clone(),
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TypeCategory {
    Bool,
    Numeric,
    Date,
    Text,
}

fn type_category(kind: Option<&str>) -> TypeCategory {
    let k = kind.unwrap_or("UNKNOWN").to_ascii_uppercase();
    if k == "BOOLEAN" || k == "BOOL" {
        TypeCategory::Bool
    } else if k.contains("INT")
        || k.contains("REAL")
        || k.contains("NUMERIC")
        || k.contains("DECIMAL")
        || k.contains("FLOAT")
        || k.contains("DOUB")
    {
        TypeCategory::Numeric
    } else if k.contains("DATE") || k.contains("TIME") {
        TypeCategory::Date
    } else {
        TypeCategory::Text
    }
}

/// The effective filter kind of a column, picking the editor kind according to FieldType when fkAuto.
pub fn effective_filter_kind(col: &GridColumn) -> &'static str {
    match col.filter_kind.as_str() {
        "fkText" => "fkText",
        "fkList" => "fkList",
        "fkRange" => "fkRange",
        "fkDate" => "fkDate",
        _ => match type_category(col.kind.as_deref()) {
            TypeCategory::Bool => "fkList",
            TypeCategory::Numeric => "fkRange",
            TypeCategory::Date => "fkDate",
            TypeCategory::Text => "fkText",
        },
    }
}

/// The effective editor kind for in-cell editing.
pub fn effective_editor(col: &GridColumn) -> &'static str {
    match col.editor.as_str() {
        "edText" => "edText",
        "edNumber" => "edNumber",
        "edDate" => "edDate",
        "edList" => "edList",
        "edCheck" => "edCheck",
        "edLookup" => {
            if col.lookup.0.is_some()
                && !col.lookup_key.is_empty()
                && !col.lookup_display.is_empty()
            {
                "edLookup"
            } else {
                "edText"
            }
        }
        "edButton" => {
            if col.button_hook {
                "edButton"
            } else {
                "edText"
            }
        }
        _ => {
            if !col.pick_list.is_empty() {
                "edList"
            } else {
                match type_category(col.kind.as_deref()) {
                    TypeCategory::Bool => "edCheck",
                    TypeCategory::Numeric => "edNumber",
                    TypeCategory::Date => "edDate",
                    TypeCategory::Text => "edText",
                }
            }
        }
    }
}

fn column_display_title(col: &GridColumn) -> &str {
    if !col.title.is_empty() {
        &col.title
    } else {
        &col.name
    }
}

/// Parses a cell's text into the value to write, by the column's editor and FieldType:
/// empty → NULL; numbers take `.` decimals and `,` thousands; dates → ISO text.
pub fn parse_cell(col: &GridColumn, text: &str) -> Result<Option<datagrid::filter::Owned>, String> {
    use datagrid::filter::Owned;
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    let title = column_display_title(col);
    let not_a = |what: &str| format!("{title} is not {what}");
    // The value's type follows the column's FieldType; the editor decides only for a
    // text or unknown FieldType (an edList over a REAL column still writes a number).
    let category = match type_category(col.kind.as_deref()) {
        TypeCategory::Text => match effective_editor(col) {
            "edCheck" => TypeCategory::Bool,
            "edNumber" => TypeCategory::Numeric,
            "edDate" => TypeCategory::Date,
            _ => TypeCategory::Text,
        },
        other => other,
    };
    match category {
        TypeCategory::Bool => match trimmed.to_ascii_lowercase().as_str() {
            "1" | "true" => Ok(Some(Owned::Bool(true))),
            "0" | "false" => Ok(Some(Owned::Bool(false))),
            _ => Err(not_a("true or false")),
        },
        TypeCategory::Date => parse_filter_date(trimmed)
            .map(|iso| Some(Owned::Text(iso)))
            .ok_or_else(|| not_a("a date")),
        TypeCategory::Numeric => {
            let Some(plain) = without_thousands(trimmed) else {
                return Err(not_a("a number"));
            };
            let integer = col
                .kind
                .as_deref()
                .is_some_and(|k| k.to_ascii_uppercase().contains("INT"));
            if integer {
                plain
                    .parse::<i64>()
                    .map(|n| Some(Owned::Int(n)))
                    .map_err(|_| not_a("a whole number"))
            } else {
                match plain.parse::<f64>() {
                    Ok(f) if f.is_finite() => Ok(Some(Owned::Float(f))),
                    _ => Err(not_a("a number")),
                }
            }
        }
        TypeCategory::Text => Ok(Some(Owned::Text(trimmed.to_string()))),
    }
}

/// `1,234.5` → `1234.5`: a `,` is accepted only between groups of three digits before
/// the decimal point, so `1.234,5` (a comma decimal) is refused instead of misread.
fn without_thousands(text: &str) -> Option<String> {
    let (whole, fraction) = text.split_once('.').unwrap_or((text, ""));
    if fraction.contains(',') {
        return None;
    }
    let digits = whole.trim_start_matches(['+', '-']);
    let mut groups = digits.split(',');
    let first = groups.next().unwrap_or_default();
    let grouped_ok =
        !digits.contains(',') || ((1..=3).contains(&first.len()) && groups.all(|g| g.len() == 3));
    grouped_ok.then(|| text.replace(',', ""))
}

/// Checks a cell's text against Required, MaxLength, its type and MinValue/MaxValue, and
/// returns the value to write. MinValue = MaxValue = 0 checks no bounds; otherwise a 0
/// MaxValue above a positive MinValue means "no upper bound".
pub fn validate_cell(
    col: &GridColumn,
    text: &str,
) -> Result<Option<datagrid::filter::Owned>, String> {
    use datagrid::filter::Owned;
    let title = column_display_title(col);
    let trimmed = text.trim();
    if col.required && trimmed.is_empty() {
        return Err(format!("{title} is required"));
    }
    let to_parse = if !col.edit_mask.is_empty() {
        let mask = crawrad_ir::mask::EditMask::parse(&col.edit_mask);
        if !trimmed.is_empty() && !mask.is_complete(trimmed) {
            return Err(format!("{title} is incomplete"));
        }
        mask.text(trimmed)
    } else {
        trimmed.to_string()
    };
    if col.max_length > 0 && to_parse.chars().count() as i64 > col.max_length {
        return Err(format!(
            "{title} is longer than {} characters",
            col.max_length
        ));
    }
    let parsed = parse_cell(col, &to_parse)?;
    let number = match parsed {
        Some(Owned::Int(n)) => Some(n as f64),
        Some(Owned::Float(f)) => Some(f),
        _ => None,
    };
    if let Some(n) = number
        && (col.min_value, col.max_value) != (0, 0)
    {
        if n < col.min_value as f64 {
            return Err(format!("{title} must be at least {}", col.min_value));
        }
        let has_upper = col.max_value != 0 || col.min_value < 0;
        if has_upper && n > col.max_value as f64 {
            return Err(format!("{title} must be at most {}", col.max_value));
        }
    }
    Ok(parsed)
}

/// Computes the number of days in a given month and year.
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// Computes the next calendar day in ISO format (`yyyy-mm-dd`).
pub fn next_day_iso(iso: &str) -> String {
    let parts: Vec<&str> = iso.split('-').collect();
    if parts.len() == 3
        && let Ok(y) = parts[0].parse::<i32>()
        && let Ok(m) = parts[1].parse::<u32>()
        && let Ok(d) = parts[2].parse::<u32>()
    {
        let max_d = days_in_month(y, m);
        let (next_y, next_m, next_d) = if d < max_d {
            (y, m, d + 1)
        } else if m < 12 {
            (y, m + 1, 1)
        } else {
            (y + 1, 1, 1)
        };
        format!("{next_y:04}-{next_m:02}-{next_d:02}")
    } else {
        iso.to_string()
    }
}

/// Parses a date string in `dd/mm/yyyy`, `dd/mm/yy` (20yy) or `yyyy-mm-dd` format into ISO `yyyy-mm-dd`.
/// Returns `None` if the input is invalid or day/month is out of range.
pub fn parse_filter_date(text: &str) -> Option<String> {
    let trimmed = text.trim();
    let (sep, day_first) = if trimmed.contains('/') {
        ('/', true)
    } else {
        ('-', false)
    };
    let parts: Vec<&str> = trimmed.split(sep).map(str::trim).collect();
    let [a, m_str, c] = parts.as_slice() else {
        return None;
    };
    let (d_str, y_str) = if day_first { (*a, *c) } else { (*c, *a) };
    let year_ok = if day_first {
        matches!(y_str.len(), 2 | 4)
    } else {
        y_str.len() == 4
    };
    if !year_ok || !(1..=2).contains(&d_str.len()) || !(1..=2).contains(&m_str.len()) {
        return None;
    }
    let d: u32 = d_str.parse().ok()?;
    let m: u32 = m_str.parse().ok()?;
    let raw_y: i32 = y_str.parse().ok()?;
    let y = if y_str.len() == 2 {
        2000 + raw_y
    } else {
        raw_y
    };
    if !(1..=12).contains(&m) || d == 0 || d > days_in_month(y, m) {
        return None;
    }
    Some(format!("{y:04}-{m:02}-{d:02}"))
}

/// The state of an active filter editor in the grid filter row.
#[derive(Clone, Debug, PartialEq)]
pub enum FilterInput {
    Text(String),
    List(Vec<Option<String>>),
    Range { min: String, max: String },
    Date { from: String, to: String },
}

/// Builds a datagrid filter predicate from column specification and input state.
/// Returns `None` when the input is empty or invalid.
pub fn filter_of(col: &GridColumn, input: &FilterInput) -> Option<datagrid::Filter> {
    match input {
        FilterInput::Text(s) => {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(datagrid::Filter::Contains {
                    column: col.name.clone(),
                    text: trimmed.to_string(),
                    ignore_case: true,
                })
            }
        }
        FilterInput::List(list) => {
            if list.is_empty() {
                return None;
            }
            let has_empty = list
                .iter()
                .any(|item| item.as_ref().is_none_or(|s| s.is_empty()));
            let non_empty: Vec<datagrid::filter::Owned> = list
                .iter()
                .filter_map(|opt| opt.as_ref())
                .filter(|s| !s.is_empty())
                .map(|s| list_value(col, s))
                .collect();

            let empty_filter = has_empty.then(|| {
                datagrid::Filter::Or(vec![
                    datagrid::Filter::IsNull(col.name.clone()),
                    datagrid::Filter::Eq(
                        col.name.clone(),
                        datagrid::filter::Owned::Text(String::new()),
                    ),
                ])
            });

            let in_filter =
                (!non_empty.is_empty()).then(|| datagrid::Filter::In(col.name.clone(), non_empty));

            match (empty_filter, in_filter) {
                (Some(ef), Some(inf)) => Some(datagrid::Filter::Or(vec![ef, inf])),
                (Some(ef), None) => Some(ef),
                (None, Some(inf)) => Some(inf),
                (None, None) => None,
            }
        }
        FilterInput::Range { min, max } => {
            let min_val = min.trim().parse::<f64>().ok();
            let max_val = max.trim().parse::<f64>().ok();
            match (min_val, max_val) {
                (Some(lo), Some(hi)) => Some(datagrid::Filter::Between(
                    col.name.clone(),
                    datagrid::filter::Owned::Float(lo),
                    datagrid::filter::Owned::Float(hi),
                )),
                (Some(lo), None) => Some(datagrid::Filter::Ge(
                    col.name.clone(),
                    datagrid::filter::Owned::Float(lo),
                )),
                (None, Some(hi)) => Some(datagrid::Filter::Le(
                    col.name.clone(),
                    datagrid::filter::Owned::Float(hi),
                )),
                (None, None) => None,
            }
        }
        FilterInput::Date { from, to } => {
            let from_iso = parse_filter_date(from);
            let to_iso = parse_filter_date(to);
            match (from_iso, to_iso) {
                (Some(f), Some(t)) => {
                    let next = next_day_iso(&t);
                    Some(datagrid::Filter::And(vec![
                        datagrid::Filter::Ge(col.name.clone(), datagrid::filter::Owned::Text(f)),
                        datagrid::Filter::Lt(col.name.clone(), datagrid::filter::Owned::Text(next)),
                    ]))
                }
                (Some(f), None) => Some(datagrid::Filter::Ge(
                    col.name.clone(),
                    datagrid::filter::Owned::Text(f),
                )),
                (None, Some(t)) => {
                    let next = next_day_iso(&t);
                    Some(datagrid::Filter::Lt(
                        col.name.clone(),
                        datagrid::filter::Owned::Text(next),
                    ))
                }
                (None, None) => None,
            }
        }
    }
}

/// A checked fkList value typed like its column, so `IN (..)` compares a
/// number with a number and a flag with a flag.
fn list_value(col: &GridColumn, text: &str) -> datagrid::filter::Owned {
    use datagrid::filter::Owned;
    let kind = col.kind.as_deref().unwrap_or("").to_ascii_uppercase();
    let bool_of = |t: &str| match t {
        "true" | "1" => Some(true),
        "false" | "0" => Some(false),
        _ => None,
    };
    let typed = if kind.starts_with("BOOL") {
        bool_of(text).map(Owned::Bool)
    } else if kind.contains("INT") {
        text.parse().ok().map(Owned::Int)
    } else if effective_filter_kind(col) == "fkRange" {
        text.parse().ok().map(Owned::Float)
    } else {
        None
    };
    typed.unwrap_or_else(|| Owned::Text(text.to_string()))
}

/// Formats an Owned value as a plain string representation.
pub fn owned_to_string(val: &datagrid::filter::Owned) -> String {
    match val {
        datagrid::filter::Owned::Int(i) => i.to_string(),
        datagrid::filter::Owned::Float(f) => f.to_string(),
        datagrid::filter::Owned::Bool(b) => b.to_string(),
        datagrid::filter::Owned::Text(s) => s.clone(),
    }
}

/// Converts a text cell value into an Owned filter literal matching column type.
pub fn string_to_owned(col: &GridColumn, text: &str) -> datagrid::filter::Owned {
    let kind = col.kind.as_deref().unwrap_or("").to_ascii_uppercase();
    if kind.contains("INT") {
        if let Ok(i) = text.trim().parse::<i64>() {
            return datagrid::filter::Owned::Int(i);
        }
    } else if kind.starts_with("BOOL") {
        match text.trim().to_ascii_lowercase().as_str() {
            "true" | "1" => return datagrid::filter::Owned::Bool(true),
            "false" | "0" => return datagrid::filter::Owned::Bool(false),
            _ => {}
        }
    } else if (kind.contains("FLOAT") || kind.contains("REAL"))
        && let Ok(f) = text.trim().parse::<f64>()
    {
        return datagrid::filter::Owned::Float(f);
    }
    datagrid::filter::Owned::Text(text.to_string())
}

/// A cell in the filter row rendered below the header.
#[derive(Clone, Debug, PartialEq)]
pub struct FilterCell {
    pub column: usize,
    pub kind: &'static str,
    pub enabled: bool,
    pub caption: String,
}

/// Screen row values accessible by field name.
#[derive(Clone, Debug, PartialEq)]
pub struct RowValues {
    pub values: Vec<(String, Option<String>)>,
}

impl RowValues {
    pub fn new(values: Vec<(String, Option<String>)>) -> Self {
        Self { values }
    }

    pub fn get(&self, field: &str) -> Option<&str> {
        self.values
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(field))
            .and_then(|(_, v)| v.as_deref())
    }

    pub fn fields(&self) -> impl Iterator<Item = &str> {
        self.values.iter().map(|(k, _)| k.as_str())
    }
}

/// A request for custom cell text via OnGetText.
#[derive(Clone, Debug, PartialEq)]
pub struct TextRequest {
    pub id: u64,
    pub column: usize,
    pub field: String,
    pub row: RowValues,
    pub text: String,
}

/// The computed or customized visual style of a grid cell.
#[derive(Clone, Debug, PartialEq)]
pub struct CellStyle {
    pub color: Option<Color>,
    pub text_color: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub text: String,
}

/// A request for custom cell styling via OnDrawCell.
#[derive(Clone, Debug, PartialEq)]
pub struct StyleRequest {
    pub id: u64,
    pub column: usize,
    pub field: String,
    pub row: RowValues,
    pub style: CellStyle,
}

/// Evaluates column style rules against row values, returning the first matching rule's cell style.
pub fn rule_style(col: &GridColumn, row: &RowValues, text: &str) -> Option<CellStyle> {
    for rule in &col.style_rules {
        if rule.when.eval(&|f| row.get(f).map(str::to_string)) {
            return Some(CellStyle {
                color: rule.color,
                text_color: rule.text_color,
                bold: rule.bold,
                italic: rule.italic,
                text: text.to_string(),
            });
        }
    }
    None
}

/// Evaluates style rules on a sample design-time row.
pub fn design_cell_style(
    col: &GridColumn,
    header: &[String],
    row: &[Option<String>],
    text: &str,
) -> Option<CellStyle> {
    if col.style_rules.is_empty() {
        return None;
    }
    let values = header
        .iter()
        .enumerate()
        .map(|(i, h)| (h.clone(), row.get(i).cloned().flatten()))
        .collect();
    let row_values = RowValues::new(values);
    rule_style(col, &row_values, text)
}

/// A cell's text and hooks to check prior to write.
#[derive(Clone, Debug, PartialEq)]
pub struct CellCheck {
    pub column: usize,
    pub field: String,
    pub text: String,
    pub set_text: bool,
    pub validate: bool,
    pub error: String,
}

/// State of an open lookup editor's drop-down list.
#[derive(Clone, Debug, PartialEq)]
pub struct LookupList {
    pub prefix: String,
    pub rows: Vec<(datagrid::filter::Owned, Vec<String>)>,
    pub highlighted: usize,
    pub total_hint: Option<u64>,
}

/// Request sent when an edButton editor's button is clicked or Ctrl+Enter is pressed.
#[derive(Clone, Debug, PartialEq)]
pub struct ButtonRequest {
    pub column: usize,
    pub field: String,
    pub row: RowValues,
    pub text: String,
}

/// The row buffer and cell checks for OnSetText and OnValidate.
#[derive(Clone, Debug, PartialEq)]
pub struct RowCheck {
    pub row: RowValues,
    pub cells: Vec<CellCheck>,
}

/// An in-progress row buffer holding edited cell texts prior to posting.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RowEdit {
    /// Row index in the current query result; `None` indicates a new unposted row.
    pub index: Option<usize>,
    /// Stable row identifier from the source; `None` for a new row.
    pub id: Option<u64>,
    /// Edited text per column (its index in `GridState::columns`, stable when columns move).
    pub texts: std::collections::BTreeMap<usize, String>,
    /// The column (index in `GridState::columns`) whose editor is open.
    pub open: Option<usize>,
    /// What is typed in the open editor; it enters `texts` on Enter, Tab or leaving the
    /// row, and Esc drops it.
    pub editor_text: String,
    /// The text the open editor started from: committing it unchanged is no edit.
    pub editor_from: String,
    /// The column (index in `GridState::columns`) the error belongs to, and the message.
    pub error: Option<(usize, String)>,
    /// Write operation ticket currently in flight with the background worker.
    pub ticket: Option<u64>,
    /// Whether hooks have already checked this buffer.
    pub checked: bool,
    /// The raw cell values as originally drawn.
    pub original_cells: Vec<Option<String>>,
    /// Lookup editor state when an edLookup column editor is open.
    pub lookup: Option<LookupList>,
}

/// A key the grid's editing reacts to; each app translates its own events.
#[derive(Clone, Debug, PartialEq)]
pub enum EditKey {
    Escape,
    Enter,
    F2,
    Tab {
        back: bool,
    },
    Insert,
    Left,
    Right,
    Up,
    Down,
    /// Ctrl+Enter (triggers button on edButton column).
    CtrlEnter,
    /// A typed character (a space flips a check column).
    Char(String),
}

/// The row the user is on, as the app drew it: its index, source id, raw cells in
/// visual order, and whether it is the new row.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CurrentRow<'a> {
    pub index: usize,
    pub id: u64,
    pub raw: &'a [Option<String>],
    pub new: bool,
}

/// What the app does after a key or a click.
#[derive(Clone, Debug, PartialEq)]
pub enum EditOutcome {
    /// Not an editing key: the app may use it.
    Ignored,
    Handled,
    /// Send it with `LiveRows::write`, then `GridState::sent(ticket)`.
    Write(datagrid::Write),
    /// Pending row check via OnSetText and OnValidate hooks.
    Check(RowCheck),
    /// Button click or Ctrl+Enter on an edButton column.
    Button(ButtonRequest),
}

#[note("grid-gridstate")]
#[derive(Clone, Debug, PartialEq)]
pub struct GridState {
    pub columns: Vec<GridColumn>,
    pub column_order: Vec<usize>,
    pub sort: Vec<datagrid::SortKey>,
    pub group_by: Vec<datagrid::GroupLevel>,
    pub timing: crawrad_component::BusyTiming,
    /// Opacity of the old rows at the height of a wait (approved: 0.45).
    pub dim: f64,
    /// The row the user is on (0-based), shown in the footer.
    pub rec_no: usize,
    /// The glyphs the painting runtime's font can draw.
    pub marks: Marks,
    pub filters: std::collections::BTreeMap<usize, FilterInput>,
    /// The row being edited.
    pub edit: Option<RowEdit>,
    /// A posted row whose write the source has not answered yet.
    pub in_flight: Option<RowEdit>,
    /// Whether the source writes (`RowSource::can_write`).
    pub can_write: bool,
    /// The visual position of the focused cell's column.
    pub focus_col: usize,
    /// Generation signature of rows seen by `text_requests`.
    pub seen_screen: Option<Vec<(usize, u64, u64)>>,
    /// Custom texts answered by OnGetText keyed by (id, col_idx).
    pub custom_texts: std::collections::BTreeMap<(u64, usize), String>,
    /// Pending row check via OnSetText and OnValidate hooks.
    pub pending_check: Option<RowCheck>,
    /// The row a move was heading to when its row had to be checked first.
    pub check_move: Option<usize>,
    /// The label shown for each key text of an edLookup column, per column index.
    pub lookup_cache: std::collections::BTreeMap<usize, std::collections::HashMap<String, String>>,
    /// Pending button request awaiting button_done.
    pub pending_button: Option<ButtonRequest>,
    /// Generation signature of rows seen by `style_requests`.
    pub seen_style_screen: Option<Vec<(usize, u64, u64)>>,
    /// Precomputed rule styles for cells keyed by (id, col_idx).
    pub rule_styles: std::collections::BTreeMap<(u64, usize), CellStyle>,
    /// Custom styles answered by OnDrawCell keyed by (id, col_idx).
    pub custom_styles: std::collections::BTreeMap<(u64, usize), CellStyle>,
    /// Whether OnDrawCell was already asked about the rows `seen_style_screen` holds.
    pub styles_asked: bool,
}

impl GridState {
    #[note("grid-gridstate-new")]
    pub fn new(columns: Vec<GridColumn>) -> Self {
        let column_order = (0..columns.len()).collect();
        let sort = initial_sort(&columns);
        let group_by = initial_groups(&columns);
        Self {
            columns,
            column_order,
            sort,
            group_by,
            timing: crawrad_component::BusyTiming::default(),
            dim: 0.45,
            rec_no: 0,
            marks: Marks::default(),
            filters: std::collections::BTreeMap::new(),
            edit: None,
            in_flight: None,
            can_write: false,
            focus_col: 0,
            seen_screen: None,
            custom_texts: std::collections::BTreeMap::new(),
            pending_check: None,
            check_move: None,
            lookup_cache: std::collections::BTreeMap::new(),
            pending_button: None,
            seen_style_screen: None,
            rule_styles: std::collections::BTreeMap::new(),
            custom_styles: std::collections::BTreeMap::new(),
            styles_asked: false,
        }
    }

    #[note("grid-gridstate-query")]
    pub fn query(&self) -> datagrid::Query {
        let summary: Vec<datagrid::Total> = self
            .columns
            .iter()
            .filter_map(|c| {
                parse_aggregate(&c.summary).map(|agg| datagrid::Total {
                    column: c.name.clone(),
                    aggregate: agg,
                })
            })
            .collect();

        let group_totals = group_totals(&self.columns);

        let mut group_by = self.group_by.clone();
        for gl in &mut group_by {
            for tot in &group_totals {
                if !gl.totals.contains(tot) {
                    gl.totals.push(tot.clone());
                }
            }
        }

        let filter = self.column_filters(None);

        datagrid::Query {
            filter,
            sort: self.sort.clone(),
            group_by,
            summary,
        }
    }

    /// Returns the active query without the filter on `col`.
    pub fn query_without(&self, col: usize) -> datagrid::Query {
        let mut q = self.query();
        q.filter = self.column_filters(Some(col));
        q
    }

    /// The `And` of every column filter but `skip`'s; `None` when none applies.
    fn column_filters(&self, skip: Option<usize>) -> Option<datagrid::Filter> {
        let mut active: Vec<datagrid::Filter> = self
            .filters
            .iter()
            .filter(|(i, _)| Some(**i) != skip)
            .filter_map(|(i, input)| filter_of(self.columns.get(*i)?, input))
            .collect();
        match active.len() {
            0 | 1 => active.pop(),
            _ => Some(datagrid::Filter::And(active)),
        }
    }

    /// Returns `true` if any column filter is currently active.
    pub fn is_filtered(&self) -> bool {
        self.column_filters(None).is_some()
    }

    #[note("grid-gridstate-click-header")]
    pub fn click_header(&mut self, column: usize) -> datagrid::Query {
        let col_idx = self.column_order.get(column).copied().unwrap_or(column);
        if let Some(col) = self.columns.get(col_idx) {
            if !col.sortable {
                return self.query();
            }
            let col_name = col.name.clone();
            let new_order = match self.sort.as_slice() {
                [current] if current.column == col_name => match current.order {
                    datagrid::Order::Asc => datagrid::Order::Desc,
                    datagrid::Order::Desc => datagrid::Order::Asc,
                },
                _ => datagrid::Order::Asc,
            };
            let nulls = match col.nulls_position.as_str() {
                "npFirst" => datagrid::Nulls::First,
                _ => datagrid::Nulls::Last,
            };
            self.sort = vec![datagrid::SortKey {
                column: col_name,
                order: new_order,
                nulls,
            }];
        }
        self.query()
    }

    #[note("grid-gridstate-group-by-column")]
    pub fn group_by_column(&mut self, column: usize) -> datagrid::Query {
        let col_idx = self.column_order.get(column).copied().unwrap_or(column);
        if let Some(col) = self.columns.get(col_idx)
            && col.groupable
            && !self.group_by.iter().any(|g| g.column == col.name)
        {
            let group_totals = group_totals(&self.columns);
            self.group_by.push(datagrid::GroupLevel {
                column: col.name.clone(),
                totals: group_totals,
            });
        }
        self.query()
    }

    /// The columns in the order the user sees them (Movable).
    pub fn ordered_columns(&self) -> Vec<GridColumn> {
        self.column_order
            .iter()
            .filter_map(|&i| self.columns.get(i).cloned())
            .collect()
    }

    /// Reorders column positions from `from` to `to`.
    pub fn move_column(&mut self, from: usize, to: usize) {
        move_column(&mut self.column_order, from, to);
    }

    /// Sets the width of the column at visual index `column`.
    pub fn set_column_width(&mut self, column: usize, width: f32) {
        let col_idx = self.column_order.get(column).copied().unwrap_or(column);
        if let Some(col) = self.columns.get_mut(col_idx) {
            let clamped = clamp_column_width(
                width as f64,
                col.min_width as f64,
                col.max_width.map(f64::from),
            ) as f32;
            col.width = Some(clamped);
            col.auto_size = false;
        }
    }

    #[note("grid-gridstate-ungroup")]
    pub fn ungroup(&mut self, level: usize) -> datagrid::Query {
        if level < self.group_by.len() {
            self.group_by.remove(level);
        }
        self.query()
    }

    /// Takes what `source` allows: whether it writes, and its key column, which the grid
    /// never edits (the database owns it).
    pub fn writes_through(&mut self, source: &dyn datagrid::RowSource) {
        self.can_write = source.can_write();
        if let Some(key) = source.key_column() {
            for column in self
                .columns
                .iter_mut()
                .filter(|c| c.name.eq_ignore_ascii_case(key))
            {
                column.read_only = true;
            }
        }
    }

    /// Whether the grid edits at all: the source writes and some column is not ReadOnly.
    pub fn is_editable(&self) -> bool {
        self.can_write && self.columns.iter().any(|c| !c.read_only)
    }

    /// The column shown at visual `position` (its index in `columns`).
    fn column_at(&self, position: usize) -> Option<usize> {
        self.column_order.get(position).copied()
    }

    /// The visual position of column `col_idx`.
    pub fn position_of(&self, col_idx: usize) -> Option<usize> {
        self.column_order.iter().position(|&c| c == col_idx)
    }

    /// Whether the column at visual `position` can be edited (the grid edits, the column is
    /// not ReadOnly).
    pub fn column_editable(&self, position: usize) -> bool {
        self.is_editable()
            && self
                .column_at(position)
                .and_then(|c| self.columns.get(c))
                .is_some_and(|c| !c.read_only)
    }

    /// Starts (or keeps, for the same row) the row buffer of row `index` / source `id`.
    fn begin_edit(&mut self, index: usize, id: u64, raw: Option<&[Option<String>]>) {
        if self.edit.as_ref().is_some_and(|e| e.index == Some(index)) {
            return;
        }
        self.edit = Some(RowEdit {
            index: Some(index),
            id: Some(id),
            original_cells: raw.map(|r| r.to_vec()).unwrap_or_default(),
            ..RowEdit::default()
        });
    }

    /// Starts a new row holding every column's DefaultValue; it is the row the user is on.
    fn begin_new_row(&mut self, new_index: usize) {
        let texts = self
            .columns
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.default_value.is_empty())
            .map(|(i, c)| (i, c.default_value.clone()))
            .collect();
        self.edit = Some(RowEdit {
            texts,
            ..RowEdit::default()
        });
        self.rec_no = new_index;
    }

    /// Opens the editor of the cell at visual `position` of `row`. A check column has no
    /// editor: it flips at once. `typed` is the character that opened it (it replaces the text).
    fn open_cell(
        &mut self,
        row: Option<&CurrentRow>,
        position: usize,
        typed: Option<String>,
    ) -> bool {
        if !self.column_editable(position) {
            return false;
        }
        let Some(col_idx) = self.column_at(position) else {
            return false;
        };
        let editing_new = self.edit.as_ref().is_some_and(|e| e.index.is_none());
        if !editing_new {
            let Some(row) = row else {
                return false;
            };
            self.begin_edit(row.index, row.id, Some(row.raw));
        }
        let check = self
            .columns
            .get(col_idx)
            .is_some_and(|c| effective_editor(c) == "edCheck");
        let raw = row.and_then(|r| r.raw.get(position)).cloned().flatten();
        let Some(edit) = &mut self.edit else {
            return false;
        };
        let current = edit
            .texts
            .get(&col_idx)
            .cloned()
            .or(raw)
            .unwrap_or_default();
        edit.error = None;
        if check {
            if typed.as_deref().is_some_and(|t| t != " ") {
                return false;
            }
            let on = matches!(current.to_ascii_lowercase().as_str(), "true" | "1");
            edit.texts.insert(col_idx, (!on).to_string());
            edit.checked = false;
            edit.open = None;
            edit.editor_text.clear();
        } else {
            edit.open = Some(col_idx);
            // `editor_from` is what the cell held (as the mask shows it): a typed key starts
            // the editor but is a change, so it must never become the origin.
            let lookup = self
                .columns
                .get(col_idx)
                .filter(|c| effective_editor(c) == "edLookup")
                .and_then(|c| c.lookup.0.clone());
            if let Some(lookup) = lookup {
                // An edLookup editor is a search box: it starts with what was typed (or
                // empty, listing from the top) and the current key is the highlighted row.
                let prefix = typed.unwrap_or_default();
                let rows = lookup.search(&prefix, 50).unwrap_or_default();
                let highlighted = rows
                    .iter()
                    .position(|(key, _)| owned_to_string(key) == current)
                    .unwrap_or(0);
                edit.editor_text = prefix.clone();
                edit.editor_from = String::new();
                edit.lookup = Some(LookupList {
                    prefix,
                    rows,
                    highlighted,
                    total_hint: None,
                });
            } else {
                let shown = match self.columns.get(col_idx) {
                    Some(c) if !c.edit_mask.is_empty() => {
                        crawrad_ir::mask::shown_text(&c.edit_mask, &current)
                    }
                    _ => current,
                };
                edit.editor_text = typed.unwrap_or_else(|| shown.clone());
                edit.editor_from = shown;
                edit.lookup = None;
            }
        }
        true
    }

    /// The open editor's text goes into the row buffer and the editor closes.
    pub fn commit_editor(&mut self) {
        let open_lookup = self
            .edit
            .as_ref()
            .and_then(|e| e.open)
            .and_then(|c| self.columns.get(c))
            .is_some_and(|c| effective_editor(c) == "edLookup");
        if let Some(edit) = &mut self.edit
            && let Some(col) = edit.open.take()
        {
            edit.lookup = None;
            let text = std::mem::take(&mut edit.editor_text);
            // An edLookup editor is a search box over labels: only a pick writes (the key).
            if open_lookup {
                return;
            }
            if text != edit.editor_from || edit.texts.contains_key(&col) {
                if edit.texts.get(&col) != Some(&text) {
                    edit.checked = false;
                }
                edit.texts.insert(col, text);
            }
        }
    }

    /// The row buffer's text for column `col_idx`, when it was edited.
    pub fn edited_text(&self, col_idx: usize) -> Option<&str> {
        self.edit.as_ref()?.texts.get(&col_idx).map(String::as_str)
    }

    /// Esc on an open editor: closes it, dropping what was typed in it.
    fn close_editor(&mut self) {
        if let Some(edit) = &mut self.edit {
            edit.open = None;
            edit.editor_text.clear();
            edit.lookup = None;
            edit.error = None;
        }
    }

    /// Esc with no editor open: drops every change of the row (a new row disappears).
    fn undo_row(&mut self) {
        if let Some(edit) = self.edit.take()
            && edit.index.is_none()
        {
            self.rec_no = self.rec_no.saturating_sub(1);
        }
    }

    /// Whether Tab stops on the column at visual `position` (editable and a TabStop).
    fn is_tab_stop(&self, position: usize) -> bool {
        self.column_editable(position)
            && self
                .column_at(position)
                .and_then(|c| self.columns.get(c))
                .is_some_and(|c| c.tab_stop)
    }

    /// The next (or previous) visual position after `position` where Tab stops.
    pub fn next_editable(&self, position: usize, backwards: bool) -> Option<usize> {
        let count = self.column_order.len();
        if backwards {
            (0..position.min(count))
                .rev()
                .find(|&p| self.is_tab_stop(p))
        } else {
            (position + 1..count).find(|&p| self.is_tab_stop(p))
        }
    }

    /// The raw and edited field values of a row for OnGetText, OnSetText, and OnValidate.
    pub fn row_values(
        &self,
        index: usize,
        _id: u64,
        cells: Option<&[Option<String>]>,
    ) -> RowValues {
        let is_editing = self
            .edit
            .as_ref()
            .is_some_and(|e| e.index == Some(index) || (e.index.is_none() && cells.is_none()));
        let mut values = Vec::with_capacity(self.columns.len());
        for (col_idx, col) in self.columns.iter().enumerate() {
            let val = if is_editing && let Some(edited) = self.edited_text(col_idx) {
                Some(edited.to_string())
            } else {
                cells
                    .and_then(|c| c.get(col.source.unwrap_or(col_idx)))
                    .cloned()
                    .flatten()
                    .or_else(|| {
                        self.edit
                            .as_ref()
                            .and_then(|e| e.original_cells.get(col.source.unwrap_or(col_idx)))
                            .cloned()
                            .flatten()
                    })
            };
            values.push((col.name.clone(), val));
        }
        RowValues { values }
    }

    /// Whether OnGetText or OnDrawCell has not answered for the visible rows yet (they moved,
    /// scrolled or were edited): the app then asks (`text_requests` / `style_requests`).
    pub fn has_stale_screen(&self, screen: &datagrid::Screen) -> bool {
        let texts = self.columns.iter().any(|c| c.get_text_hook);
        let styles = self.columns.iter().any(|c| c.draw_cell_hook);
        if !texts && !styles {
            return false;
        }
        let sig = screen_signature(screen);
        (texts && self.seen_screen.as_ref() != Some(&sig))
            || (styles && (!self.styles_asked || self.seen_style_screen.as_ref() != Some(&sig)))
    }

    /// Requests custom display texts for visible rows from columns with `get_text_hook`.
    pub fn text_requests(&mut self, screen: &datagrid::Screen) -> Vec<TextRequest> {
        let sig = screen_signature(screen);
        if self.seen_screen.as_ref() == Some(&sig) {
            return Vec::new();
        }
        self.seen_screen = Some(sig);
        self.custom_texts.clear();
        let mut reqs = Vec::new();
        for line in &screen.lines {
            let datagrid::ScreenLine::Row { index, id, cells } = line else {
                continue;
            };
            for &col_idx in &self.column_order {
                let Some(col) = self.columns.get(col_idx) else {
                    continue;
                };
                if !col.get_text_hook {
                    continue;
                }
                let raw_val = cells
                    .get(col.source.unwrap_or(col_idx))
                    .and_then(|opt| opt.as_deref());
                let initial_text = cell_text(col, col.kind.as_deref(), raw_val);
                let row_values = self.row_values(*index, *id, Some(cells));
                reqs.push(TextRequest {
                    id: *id,
                    column: col_idx,
                    field: col.name.clone(),
                    row: row_values,
                    text: initial_text,
                });
            }
        }
        reqs
    }

    /// Stores answered custom texts for cells.
    pub fn set_texts(&mut self, reqs: Vec<TextRequest>) {
        for req in reqs {
            if let Some(style) = self.rule_styles.get_mut(&(req.id, req.column)) {
                style.text.clone_from(&req.text);
            }
            self.custom_texts.insert((req.id, req.column), req.text);
        }
    }

    /// The text a cell shows before any style: OnGetText's answer, else the formatted value.
    fn base_text(
        &self,
        id: u64,
        col_idx: usize,
        col: &GridColumn,
        cells: &[Option<String>],
    ) -> String {
        match self.custom_texts.get(&(id, col_idx)) {
            Some(custom) => custom.clone(),
            None => cell_text(
                col,
                col.kind.as_deref(),
                cells
                    .get(col.source.unwrap_or(col_idx))
                    .and_then(|v| v.as_deref()),
            ),
        }
    }

    /// Checks the style rules against the visible rows when they changed (moved, scrolled or
    /// edited); the widget calls it before painting, so rules work with no OnDrawCell at all.
    pub fn refresh_rule_styles(&mut self, screen: &datagrid::Screen) {
        if !self
            .columns
            .iter()
            .any(|c| c.draw_cell_hook || !c.style_rules.is_empty())
        {
            return;
        }
        let sig = screen_signature(screen);
        if self.seen_style_screen.as_ref() == Some(&sig) {
            return;
        }
        self.seen_style_screen = Some(sig);
        self.rule_styles.clear();
        self.custom_styles.clear();
        self.styles_asked = false;
        for line in &screen.lines {
            let datagrid::ScreenLine::Row { index, id, cells } = line else {
                continue;
            };
            let row_values = self.row_values(*index, *id, Some(cells));
            for &col_idx in &self.column_order {
                let Some(col) = self.columns.get(col_idx) else {
                    continue;
                };
                if col.style_rules.is_empty() {
                    continue;
                }
                let text = self.base_text(*id, col_idx, col, cells);
                if let Some(style) = rule_style(col, &row_values, &text) {
                    self.rule_styles.insert((*id, col_idx), style);
                }
            }
        }
    }

    /// Requests custom cell styles for the visible cells of columns with `draw_cell_hook`, once
    /// per change of the visible rows; each carries the rule style, or a plain one.
    pub fn style_requests(&mut self, screen: &datagrid::Screen) -> Vec<StyleRequest> {
        self.refresh_rule_styles(screen);
        if self.styles_asked || !self.columns.iter().any(|c| c.draw_cell_hook) {
            return Vec::new();
        }
        self.styles_asked = true;
        let mut reqs = Vec::new();
        for line in &screen.lines {
            let datagrid::ScreenLine::Row { index, id, cells } = line else {
                continue;
            };
            let row_values = self.row_values(*index, *id, Some(cells));
            for &col_idx in &self.column_order {
                let Some(col) = self.columns.get(col_idx) else {
                    continue;
                };
                if !col.draw_cell_hook {
                    continue;
                }
                let text = self.base_text(*id, col_idx, col, cells);
                let style = match self.rule_styles.get(&(*id, col_idx)) {
                    Some(rule) => CellStyle {
                        text,
                        ..rule.clone()
                    },
                    None => CellStyle {
                        color: None,
                        text_color: None,
                        bold: false,
                        italic: false,
                        text,
                    },
                };
                reqs.push(StyleRequest {
                    id: *id,
                    column: col_idx,
                    field: col.name.clone(),
                    row: row_values.clone(),
                    style,
                });
            }
        }
        reqs
    }

    /// Stores answered custom cell styles.
    pub fn set_styles(&mut self, reqs: Vec<StyleRequest>) {
        for req in reqs {
            self.custom_styles.insert((req.id, req.column), req.style);
        }
    }

    /// Returns the effective cell style for a cell, if any rule matched or draw_cell_hook answered.
    pub fn cell_style(&self, row_id: u64, col_idx: usize) -> Option<&CellStyle> {
        self.custom_styles
            .get(&(row_id, col_idx))
            .or_else(|| self.rule_styles.get(&(row_id, col_idx)))
    }

    /// Returns the active row check waiting for OnSetText / OnValidate answers.
    pub fn pending_check(&self) -> Option<RowCheck> {
        self.pending_check.clone()
    }

    /// Returns the active button request waiting for OnButtonClick.
    pub fn pending_button(&self) -> Option<ButtonRequest> {
        self.pending_button.clone()
    }

    /// Applies answered cell checks, setting errors or proceeding to write.
    pub fn checked(&mut self, check: RowCheck) -> EditOutcome {
        self.pending_check = None;
        let Some(edit) = &mut self.edit else {
            return EditOutcome::Handled;
        };
        for cell in &check.cells {
            edit.texts.insert(cell.column, cell.text.clone());
            if edit.open == Some(cell.column) {
                edit.editor_text = cell.text.clone();
            }
        }
        if let Some(err_cell) = check.cells.into_iter().find(|c| !c.error.is_empty()) {
            edit.open = Some(err_cell.column);
            edit.editor_text = err_cell.text;
            edit.error = Some((err_cell.column, err_cell.error));
            self.check_move = None;
            return EditOutcome::Handled;
        }
        edit.checked = true;
        let outcome = self.post();
        let to = self.check_move.take();
        if let Some(to) = to
            && (self.edit.is_none() || matches!(outcome, EditOutcome::Write(_)))
        {
            self.rec_no = to;
        }
        outcome
    }

    /// The label an edLookup column shows for the key `text` (the key itself until known).
    fn lookup_label(
        cache: &std::collections::BTreeMap<usize, std::collections::HashMap<String, String>>,
        col_idx: usize,
        text: &str,
    ) -> String {
        cache
            .get(&col_idx)
            .and_then(|c| c.get(text))
            .cloned()
            .unwrap_or_else(|| text.to_string())
    }

    /// Fetches missing display labels for visible rows in edLookup columns. The app
    /// calls it before each drawing: only keys the cache has never seen reach the lookup.
    pub fn update_lookups(&mut self, screen: &datagrid::Screen) {
        for &col_idx in &self.column_order {
            let Some(col) = self.columns.get(col_idx) else {
                continue;
            };
            if effective_editor(col) != "edLookup" {
                continue;
            }
            let Some(lookup) = &col.lookup.0 else {
                continue;
            };
            let cache = self.lookup_cache.entry(col_idx).or_default();
            let mut missing_keys = Vec::new();
            let mut missing_set = std::collections::HashSet::new();

            for line in &screen.lines {
                let datagrid::ScreenLine::Row { cells, .. } = line else {
                    continue;
                };
                let raw_val = cells
                    .get(col.source.unwrap_or(col_idx))
                    .and_then(|opt| opt.as_deref());
                let Some(raw_str) = raw_val else {
                    continue;
                };
                if !cache.contains_key(raw_str) && missing_set.insert(raw_str.to_string()) {
                    missing_keys.push(string_to_owned(col, raw_str));
                }
            }

            if !missing_keys.is_empty() {
                if let Ok(labels) = lookup.labels(&missing_keys) {
                    for (k, display_vec) in labels {
                        let label = display_vec.first().cloned().unwrap_or_default();
                        cache.insert(owned_to_string(&k), label);
                    }
                }
                // A key the lookup does not know shows as itself, and is not asked again.
                for key in missing_keys {
                    let text = owned_to_string(&key);
                    cache.entry(text.clone()).or_insert(text);
                }
            }
        }
    }

    /// Picks a row from the open lookup dropdown, storing the key in the buffer.
    pub fn pick_lookup(&mut self, row_idx: usize) {
        let Some(edit) = &mut self.edit else {
            return;
        };
        let Some(col_idx) = edit.open else {
            return;
        };
        let Some(lookup_list) = &mut edit.lookup else {
            return;
        };
        if let Some((key, display)) = lookup_list.rows.get(row_idx) {
            let key_str = owned_to_string(key);
            let label = display.first().cloned().unwrap_or_default();
            self.lookup_cache
                .entry(col_idx)
                .or_default()
                .insert(key_str.clone(), label.clone());
            edit.texts.insert(col_idx, key_str);
            // The editor now shows the picked label: committing it must keep the key.
            edit.editor_from = label.clone();
            edit.editor_text = label;
            edit.checked = false;
            edit.lookup = None;
        }
    }

    /// Updates the search prefix and results for an open lookup dropdown.
    pub fn update_lookup_search(&mut self, prefix: &str) {
        let Some(edit) = &mut self.edit else {
            return;
        };
        let Some(col_idx) = edit.open else {
            return;
        };
        let Some(col) = self.columns.get(col_idx) else {
            return;
        };
        let Some(lookup) = &col.lookup.0 else {
            return;
        };
        // Typing again after a pick opens the list again.
        let lookup_list = edit.lookup.get_or_insert_with(|| LookupList {
            prefix: String::new(),
            rows: Vec::new(),
            highlighted: 0,
            total_hint: None,
        });
        lookup_list.prefix = prefix.to_string();
        lookup_list.rows = lookup.search(prefix, 50).unwrap_or_default();
        lookup_list.highlighted = 0;
    }

    /// Triggers an OnButtonClick event for the open edButton editor.
    pub fn trigger_button(&mut self) -> EditOutcome {
        let Some(edit) = &self.edit else {
            return EditOutcome::Ignored;
        };
        let Some(col_idx) = edit.open else {
            return EditOutcome::Ignored;
        };
        let Some(col) = self.columns.get(col_idx) else {
            return EditOutcome::Ignored;
        };
        if effective_editor(col) != "edButton" {
            return EditOutcome::Ignored;
        }
        let row_vals = self.row_values(
            edit.index.unwrap_or(self.rec_no),
            edit.id.unwrap_or_default(),
            None,
        );
        let req = ButtonRequest {
            column: col_idx,
            field: col.name.clone(),
            row: row_vals,
            text: edit.editor_text.clone(),
        };
        self.pending_button = Some(req.clone());
        EditOutcome::Button(req)
    }

    /// Handles completion of an OnButtonClick event, updating the buffer and text.
    pub fn button_done(&mut self, req: ButtonRequest) {
        if let Some(edit) = &mut self.edit {
            edit.texts.insert(req.column, req.text.clone());
            if edit.open == Some(req.column) {
                edit.editor_text = req.text;
            }
            edit.checked = false;
        }
        self.pending_button = None;
    }

    /// Validates the row buffer (committing the open editor first) and makes its write: an
    /// existing row writes only its edited columns (`Ok(None)` and the buffer dropped when
    /// nothing changed); a new row writes its non-empty values, the rest take the database
    /// default, and every Required column is checked even when not edited. A write moves the
    /// buffer to `in_flight` until `on_written`; an error stays on the row and is returned.
    pub fn post(&mut self) -> EditOutcome {
        self.commit_editor();
        let Some(edit) = &self.edit else {
            return EditOutcome::Handled;
        };
        let is_new = edit.index.is_none();

        if !edit.checked {
            let mut checks = Vec::new();
            for (col_idx, col) in self.columns.iter().enumerate() {
                if col.read_only {
                    continue;
                }
                let edited = edit.texts.contains_key(&col_idx);
                if (is_new || edited) && (col.set_text_hook || col.validate_hook) {
                    let text = edit.texts.get(&col_idx).cloned().unwrap_or_else(|| {
                        if is_new {
                            col.default_value.clone()
                        } else {
                            String::new()
                        }
                    });
                    checks.push(CellCheck {
                        column: col_idx,
                        field: col.name.clone(),
                        text,
                        set_text: col.set_text_hook,
                        validate: col.validate_hook,
                        error: String::new(),
                    });
                }
            }
            if !checks.is_empty() {
                let row_vals = self.row_values(
                    edit.index.unwrap_or(self.rec_no),
                    edit.id.unwrap_or_default(),
                    None,
                );
                let row_check = RowCheck {
                    row: row_vals,
                    cells: checks,
                };
                self.pending_check = Some(row_check.clone());
                return EditOutcome::Check(row_check);
            }
        }

        let mut changes = Vec::new();
        let mut failure = None;
        for &col_idx in &self.column_order {
            let Some(col) = self.columns.get(col_idx) else {
                continue;
            };
            let text = edit.texts.get(&col_idx);
            if col.read_only || (text.is_none() && !(is_new && col.required)) {
                continue;
            }
            match validate_cell(col, text.map_or("", String::as_str)) {
                Ok(None) if is_new => {}
                Ok(value) => changes.push((col.name.clone(), value)),
                Err(message) => {
                    failure = Some((col_idx, message));
                    break;
                }
            }
        }
        let Some(mut edit) = self.edit.take() else {
            return EditOutcome::Handled;
        };
        if let Some((col_idx, message)) = failure {
            edit.open = Some(col_idx);
            edit.editor_text = edit.texts.get(&col_idx).cloned().unwrap_or_default();
            edit.editor_from = edit.editor_text.clone();
            edit.error = Some((col_idx, message));
            self.edit = Some(edit);
            return EditOutcome::Handled;
        }
        let write = match (is_new, edit.id) {
            (true, _) => datagrid::Write::Insert(changes),
            (false, _) if changes.is_empty() => return EditOutcome::Handled,
            (false, id) => datagrid::Write::Update {
                id: id.unwrap_or_default(),
                changes,
            },
        };
        self.in_flight = Some(edit);
        EditOutcome::Write(write)
    }

    /// The write made by `post` was sent under `ticket`.
    pub fn sent(&mut self, ticket: u64) {
        if let Some(edit) = &mut self.in_flight {
            edit.ticket = Some(ticket);
        }
    }

    /// The outcome of the write in flight: saved drops it; refused brings the row back to
    /// the user — its buffer, the source's message on the first edited column, and the
    /// cursor on it (the user may have moved on while it was being written). Returns whether
    /// it was the answer to the write in flight — the screen keeps the last answer, so the
    /// same one is seen again on every later poll and must change nothing then.
    pub fn on_written(&mut self, written: &datagrid::Written) -> bool {
        if self.in_flight.as_ref().and_then(|e| e.ticket) != Some(written.ticket) {
            return false;
        }
        let Some(mut edit) = self.in_flight.take() else {
            return false;
        };
        if let Err(message) = &written.result {
            let col = edit.texts.keys().next().copied().unwrap_or_default();
            edit.error = Some((col, message.clone()));
            edit.ticket = None;
            if let Some(index) = edit.index {
                self.rec_no = index;
            }
            self.edit = Some(edit);
        }
        true
    }

    /// Posts the row being left; the cursor moves to `to` unless the row is in error.
    fn leave_row(&mut self, to: usize) -> EditOutcome {
        let outcome = self.post();
        if self.edit.is_none() || matches!(outcome, EditOutcome::Write(_)) {
            self.rec_no = to;
        } else if matches!(outcome, EditOutcome::Check(_)) {
            self.check_move = Some(to);
        }
        outcome
    }

    /// Posts what is being edited and starts a new row after the last one (`total_rows`).
    fn add_row(&mut self, total_rows: usize) -> EditOutcome {
        if !self.is_editable() {
            return EditOutcome::Ignored;
        }
        let outcome = self.leave_row(self.rec_no);
        if self.edit.is_some() {
            return outcome; // the row being left is in error
        }
        self.begin_new_row(total_rows);
        self.focus_col = if self.is_tab_stop(0) {
            0
        } else {
            self.next_editable(0, false).unwrap_or(0)
        };
        self.open_cell(None, self.focus_col, None);
        match outcome {
            EditOutcome::Ignored => EditOutcome::Handled,
            other => other,
        }
    }

    /// A key on the grid (mockup B): F2 / Enter / a character open the focused cell;
    /// Tab moves between editable cells; Enter posts the row; Esc closes the editor, then
    /// undoes the row; Up / Down leave the row (posting it); Insert, or Down on the last row,
    /// adds a new row. `row` is the row the user is on; `total_rows` the query's row count.
    pub fn edit_key(
        &mut self,
        key: EditKey,
        row: Option<&CurrentRow>,
        total_rows: usize,
    ) -> EditOutcome {
        let open = self.edit.as_ref().is_some_and(|e| e.open.is_some());
        let on_new = self.edit.as_ref().is_some_and(|e| e.index.is_none());
        let opened = |grid: &mut Self, typed| {
            if grid.open_cell(row, grid.focus_col, typed) {
                EditOutcome::Handled
            } else {
                EditOutcome::Ignored
            }
        };
        match key {
            EditKey::Escape if open => {
                if let Some(edit) = &mut self.edit
                    && edit.lookup.is_some()
                {
                    edit.lookup = None;
                } else {
                    self.close_editor();
                }
            }
            EditKey::Escape if self.edit.is_some() => self.undo_row(),
            EditKey::CtrlEnter if open => {
                let col_idx = self.edit.as_ref().and_then(|e| e.open);
                if let Some(col_idx) = col_idx
                    && let Some(col) = self.columns.get(col_idx)
                    && effective_editor(col) == "edButton"
                {
                    return self.trigger_button();
                }
            }
            EditKey::Enter if open => {
                if let Some(edit) = &self.edit
                    && let Some(lookup_list) = &edit.lookup
                {
                    let highlighted = lookup_list.highlighted;
                    self.pick_lookup(highlighted);
                    return EditOutcome::Handled;
                }
                if self.edit.is_some() {
                    return self.post();
                }
            }
            EditKey::Enter if self.edit.is_some() => {
                return self.post();
            }
            EditKey::Enter | EditKey::F2 if !open => return opened(self, None),
            EditKey::Char(typed) if !open => return opened(self, Some(typed)),
            EditKey::Tab { back } => {
                let Some(next) = self.next_editable(self.focus_col, back) else {
                    return EditOutcome::Ignored;
                };
                let editing = self.edit.is_some();
                self.commit_editor();
                self.focus_col = next;
                if editing {
                    self.open_cell(row, next, None);
                }
            }
            EditKey::Left if !open => self.focus_col = self.focus_col.saturating_sub(1),
            EditKey::Right if !open => {
                self.focus_col =
                    (self.focus_col + 1).min(self.column_order.len().saturating_sub(1));
            }
            // Up / Down move highlighted in lookup when open
            EditKey::Up if open && self.edit.as_ref().is_some_and(|e| e.lookup.is_some()) => {
                if let Some(edit) = &mut self.edit
                    && let Some(lookup_list) = &mut edit.lookup
                {
                    lookup_list.highlighted = lookup_list.highlighted.saturating_sub(1);
                    return EditOutcome::Handled;
                }
            }
            EditKey::Down if open && self.edit.as_ref().is_some_and(|e| e.lookup.is_some()) => {
                if let Some(edit) = &mut self.edit
                    && let Some(lookup_list) = &mut edit.lookup
                {
                    let max_idx = lookup_list.rows.len().saturating_sub(1);
                    if lookup_list.highlighted < max_idx {
                        lookup_list.highlighted += 1;
                    }
                    return EditOutcome::Handled;
                }
            }
            // Up / Down leave the row even from an open (one-line) editor, committing it.
            EditKey::Up if on_new || self.rec_no > 0 => {
                return self.leave_row(self.rec_no.saturating_sub(1));
            }
            EditKey::Down if !on_new && self.rec_no + 1 < total_rows => {
                return self.leave_row(self.rec_no + 1);
            }
            EditKey::Down if !on_new => return self.add_row(total_rows),
            EditKey::Insert if !on_new => return self.add_row(total_rows),
            _ => return EditOutcome::Ignored,
        }
        EditOutcome::Handled
    }

    /// A click on the cell at visual `position` of `row`: another row is left (posting the
    /// one being edited); a click on the row the user is already on opens that cell.
    pub fn click_cell(&mut self, row: &CurrentRow, position: usize) -> EditOutcome {
        let same_row = if row.new {
            self.edit.as_ref().is_some_and(|e| e.index.is_none())
        } else {
            self.rec_no == row.index && self.edit.as_ref().is_none_or(|e| e.index.is_some())
        };
        if !same_row {
            let outcome = if self.edit.is_some() {
                self.leave_row(row.index)
            } else {
                self.rec_no = row.index;
                EditOutcome::Handled
            };
            if self.rec_no == row.index {
                self.focus_col = position;
            }
            return outcome;
        }
        self.commit_editor();
        self.focus_col = position;
        self.open_cell(Some(row), position, None);
        EditOutcome::Handled
    }

    #[note("grid-gridstate-title-of")]
    fn title_of<'a>(&'a self, name: &'a str) -> &'a str {
        self.columns
            .iter()
            .find(|c| c.name == name)
            .map(|c| c.title.as_str())
            .unwrap_or(name)
    }

    #[note("grid-gridstate-header-cells")]
    fn header_cells(&self) -> Vec<HeaderCell> {
        self.column_order
            .iter()
            .filter_map(|&col_idx| self.columns.get(col_idx))
            .map(|col| {
                let arrow_str = match self.sort.iter().find(|k| k.column == col.name) {
                    Some(k) => self.arrow(k.order),
                    None => "",
                };
                HeaderCell {
                    title: col.title.clone(),
                    arrow: arrow_str,
                    align: col.title_align,
                    title_color: col.title_color,
                    title_hint: col.title_hint.clone(),
                    title_font: col.title_font.clone(),
                    title_word_wrap: col.title_word_wrap,
                    auto_size: col.auto_size,
                    resizable: col.resizable,
                    movable: col.movable,
                }
            })
            .collect()
    }

    #[note("grid-gridstate-chips")]
    fn chips(&self) -> Vec<String> {
        self.group_by
            .iter()
            .map(|level| self.title_of(&level.column).to_string())
            .collect()
    }

    #[note("grid-gridstate-totals-text")]
    fn totals_text(&self, level: usize, column: &str, totals: &[Option<f64>]) -> String {
        let group_level = self
            .group_by
            .get(level)
            .filter(|g| g.column == column)
            .or_else(|| self.group_by.iter().find(|g| g.column == column));

        if let Some(gl) = group_level {
            let parts: Vec<String> = gl
                .totals
                .iter()
                .enumerate()
                .map(|(i, total_spec)| {
                    let tot_title = self.title_of(&total_spec.column);
                    let val_str = match totals.get(i).copied().flatten() {
                        Some(val) => format_float_thousands(val),
                        None => "—".to_string(),
                    };
                    match total_spec.aggregate {
                        datagrid::Aggregate::Sum => {
                            format!("Σ {} {}", tot_title, val_str)
                        }
                        datagrid::Aggregate::Count => {
                            format!("{} count {}", tot_title, val_str)
                        }
                        datagrid::Aggregate::Min => {
                            format!("min {} {}", tot_title, val_str)
                        }
                        datagrid::Aggregate::Max => {
                            format!("max {} {}", tot_title, val_str)
                        }
                        datagrid::Aggregate::Avg => {
                            format!("avg {} {}", tot_title, val_str)
                        }
                    }
                })
                .collect();
            parts.join(" · ")
        } else {
            String::new()
        }
    }

    #[note("grid-gridstate-arrow")]
    fn arrow(&self, order: datagrid::Order) -> &'static str {
        match order {
            datagrid::Order::Asc => self.marks.ascending,
            datagrid::Order::Desc => self.marks.descending,
        }
    }

    #[note("grid-gridstate-view-line")]
    fn view_line(&self, line: &datagrid::ScreenLine) -> ViewLine {
        match line {
            datagrid::ScreenLine::Header {
                level,
                column,
                key,
                row_count,
                totals,
            } => {
                let col_title = self.title_of(column);
                // The key reads as its cells do (ValueMap, BooleanTexts, DisplayFormat).
                let shown_key = self.columns.iter().find(|c| c.name == *column).map_or_else(
                    || key.clone(),
                    |c| cell_text(c, c.kind.as_deref(), Some(key)),
                );
                let title = format!("{}{}: {}", self.marks.group_open, col_title, shown_key);
                let count = if *row_count == 1 {
                    "1 row".to_string()
                } else {
                    format!("{} rows", format_thousands_int(*row_count))
                };
                let totals_str = self.totals_text(*level, column, totals);
                ViewLine::Header {
                    level: *level,
                    title,
                    count,
                    totals: totals_str,
                }
            }
            datagrid::ScreenLine::Row { index, id, cells } => {
                let edited = self.edit.as_ref().is_some_and(|e| e.index == Some(*index));
                let marker = if edited { self.marks.edited_row } else { "" };
                self.row_line(*index, *id, Some(cells), marker)
            }
        }
    }

    /// One data row as drawn: the source cells (none for a new row), with the row buffer's
    /// edited texts over them when `marker` says the row is being edited.
    fn row_line(
        &self,
        index: usize,
        id: u64,
        cells: Option<&[Option<String>]>,
        marker: &'static str,
    ) -> ViewLine {
        let mut shown = Vec::with_capacity(self.column_order.len());
        let mut raw = Vec::with_capacity(self.column_order.len());
        for &col_idx in &self.column_order {
            let Some(col) = self.columns.get(col_idx) else {
                continue;
            };
            let edited = (!marker.is_empty())
                .then(|| self.edited_text(col_idx))
                .flatten();
            if let Some(text) = edited {
                let display = if effective_editor(col) == "edLookup" {
                    Self::lookup_label(&self.lookup_cache, col_idx, text)
                } else {
                    text.to_string()
                };
                shown.push(display);
                raw.push((!text.is_empty()).then(|| text.to_string()));
            } else {
                let text = cells
                    .and_then(|c| c.get(col.source.unwrap_or(col_idx)))
                    .and_then(|opt| opt.as_deref());
                let cell_str = if let Some(style) = self.cell_style(id, col_idx) {
                    style.text.clone()
                } else if let Some(custom) = self.custom_texts.get(&(id, col_idx)) {
                    custom.clone()
                } else if effective_editor(col) == "edLookup" {
                    match text {
                        Some(raw_str) => Self::lookup_label(&self.lookup_cache, col_idx, raw_str),
                        None => col.null_text.clone(),
                    }
                } else {
                    cell_text(col, col.kind.as_deref(), text)
                };
                shown.push(cell_str);
                raw.push(text.map(str::to_string));
            }
        }
        ViewLine::Row {
            index,
            id,
            cells: shown,
            raw_cells: raw,
            odd: index % 2 == 1,
            selected: cells.is_none() || index == self.rec_no,
            marker,
        }
    }

    #[note("grid-gridstate-overlay")]
    fn overlay(
        &self,
        screen: &datagrid::Screen,
        now: std::time::Instant,
    ) -> (Option<Overlay>, f64, bool) {
        match screen.busy {
            Some(busy) => {
                let since_start = now.saturating_duration_since(busy.started);
                let ended_dur = busy
                    .ended
                    .map(|e| e.saturating_duration_since(busy.started));
                let opacity = self.timing.opacity(since_start, ended_dur);
                let animating = self.timing.animating(since_start, ended_dur);
                let rows_alpha = if opacity > 0.0 {
                    1.0 - (1.0 - self.dim) * opacity
                } else {
                    1.0
                };
                let overlay = if opacity > 0.0 {
                    let title = match screen.pending {
                        Some((first, count)) => {
                            let start = first + 1;
                            let end = first + count;
                            format!(
                                "Loading rows {} – {}",
                                format_thousands_int(start),
                                format_thousands_int(end)
                            )
                        }
                        None => "Loading…".to_string(),
                    };
                    let elapsed = match busy.ended {
                        Some(ended) => ended.saturating_duration_since(busy.started),
                        None => since_start,
                    };
                    let detail = format!("{:.2} s · you can keep scrolling", elapsed.as_secs_f64());
                    Some(Overlay {
                        opacity,
                        title,
                        detail,
                    })
                } else {
                    None
                };
                (overlay, rows_alpha, animating)
            }
            None => (None, 1.0, false),
        }
    }

    #[note("grid-gridstate-footer")]
    fn footer(&self, screen: &datagrid::Screen) -> Footer {
        let mut position = match screen.total {
            Some(total) => format!(
                "Row {} of {}",
                format_thousands_int(self.rec_no + 1),
                format_thousands_int(total)
            ),
            None => format!("Row {}", format_thousands_int(self.rec_no + 1)),
        };
        if self.is_filtered() {
            position.push_str(" · filtered");
        }

        let state = if screen.pending.is_some() {
            "fetching…"
        } else {
            ""
        };

        let order = {
            let mut parts = Vec::new();
            if !self.sort.is_empty() {
                let sort_strs: Vec<String> = self
                    .sort
                    .iter()
                    .map(|k| {
                        let col_title = self.title_of(&k.column);
                        format!("{}{}", col_title, self.arrow(k.order))
                    })
                    .collect();
                parts.push(format!("sorted by {}", sort_strs.join(", ")));
            }
            if !self.group_by.is_empty() {
                let group_strs: Vec<String> = self
                    .group_by
                    .iter()
                    .map(|g| self.title_of(&g.column).to_string())
                    .collect();
                parts.push(format!("grouped by {}", group_strs.join(", ")));
            }
            parts.join(" · ")
        };

        let timing = match screen.busy {
            Some(busy) => match busy.ended {
                Some(ended) => {
                    let ms = ended.saturating_duration_since(busy.started).as_secs_f64() * 1000.0;
                    format!("last page {:.1} ms", ms)
                }
                None => String::new(),
            },
            None => String::new(),
        };

        Footer {
            position,
            state,
            order,
            timing,
        }
    }

    #[note("grid-gridstate-view")]
    pub fn view(&self, screen: &datagrid::Screen, now: std::time::Instant) -> GridView {
        let (overlay, rows_alpha, animating) = self.overlay(screen, now);
        let has_summary = self
            .columns
            .iter()
            .any(|c| parse_aggregate(&c.summary).is_some());
        let summary = if has_summary {
            let summary_cols: Vec<(&str, usize)> = self
                .columns
                .iter()
                .filter(|c| parse_aggregate(&c.summary).is_some())
                .enumerate()
                .map(|(idx, c)| (c.name.as_str(), idx))
                .collect();

            self.column_order
                .iter()
                .map(|&col_idx| {
                    if let Some(col) = self.columns.get(col_idx) {
                        if parse_aggregate(&col.summary).is_some() {
                            let val_opt = summary_cols
                                .iter()
                                .find(|(name, _)| *name == col.name)
                                .and_then(|(_, s_idx)| {
                                    screen.summary.get(*s_idx).copied().flatten()
                                });
                            summary_text(col, val_opt)
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    }
                })
                .collect()
        } else {
            Vec::new()
        };
        let filter_row = if self.columns.iter().any(|c| c.filterable) {
            self.column_order
                .iter()
                .map(|&col_idx| {
                    if let Some(col) = self.columns.get(col_idx) {
                        let kind = effective_filter_kind(col);
                        let caption = if kind == "fkList" {
                            match self.filters.get(&col_idx) {
                                Some(FilterInput::List(list)) if !list.is_empty() => {
                                    let items: Vec<String> = list
                                        .iter()
                                        .map(|opt| match opt {
                                            Some(s) if !s.is_empty() => s.clone(),
                                            _ => "(empty)".to_string(),
                                        })
                                        .collect();
                                    format!("{}{}", items.join(", "), self.marks.drop_down)
                                }
                                _ => format!("All{}", self.marks.drop_down),
                            }
                        } else {
                            String::new()
                        };
                        FilterCell {
                            column: col_idx,
                            kind,
                            enabled: col.filterable,
                            caption,
                        }
                    } else {
                        FilterCell {
                            column: col_idx,
                            kind: "fkAuto",
                            enabled: false,
                            caption: String::new(),
                        }
                    }
                })
                .collect()
        } else {
            Vec::new()
        };
        let mut lines: Vec<ViewLine> = screen
            .lines
            .iter()
            .map(|line| self.view_line(line))
            .collect();

        if self.edit.as_ref().is_some_and(|e| e.index.is_none()) {
            // The new row is the one after the last: its index is the row count.
            let new_index = screen.total.unwrap_or(self.rec_no);
            lines.push(self.row_line(new_index, 0, None, self.marks.new_row));
        }

        let gutter = self.is_editable();
        let open_editor = self
            .edit
            .as_ref()
            .and_then(|e| e.open)
            .and_then(|c| self.position_of(c));
        let error_cell = self
            .edit
            .as_ref()
            .and_then(|e| e.error.as_ref())
            .and_then(|(c, message)| Some((self.position_of(*c)?, message.clone())));
        let status_text = if self.edit.is_some() {
            "editing — Enter or row change saves, Esc undoes the row".to_string()
        } else {
            String::new()
        };

        GridView {
            header: self.header_cells(),
            chips: self.chips(),
            group_hint: "drag a column header here to group",
            lines,
            rows_alpha,
            overlay,
            footer: self.footer(screen),
            error: screen.error.clone(),
            animating,
            summary,
            filter_row,
            gutter,
            open_editor,
            error_cell,
            status_text,
        }
    }
}

#[note("grid-gridview")]
#[derive(Clone, Debug, PartialEq)]
pub struct GridView {
    pub header: Vec<HeaderCell>,
    pub chips: Vec<String>,
    pub group_hint: &'static str,
    pub lines: Vec<ViewLine>,
    pub rows_alpha: f64,
    pub overlay: Option<Overlay>,
    pub footer: Footer,
    pub error: Option<String>,
    /// True while the overlay is waiting, shown or fading: the widget asks for the next frame.
    pub animating: bool,
    /// Grand total summary texts for each visible column, or empty if none configured.
    pub summary: Vec<String>,
    /// Filter row cells for each visible column, or empty if no column is filterable.
    pub filter_row: Vec<FilterCell>,
    /// Whether the indicator gutter (22 px) is visible.
    pub gutter: bool,
    /// The visual position of the column whose editor is open on the edited row.
    pub open_editor: Option<usize>,
    /// The visual position of the column in error on the edited row, and the message.
    pub error_cell: Option<(usize, String)>,
    /// Status text displayed in the grid footer when editing.
    pub status_text: String,
}

#[note("grid-headercell")]
#[derive(Clone, Debug, PartialEq)]
pub struct HeaderCell {
    pub title: String,
    pub arrow: &'static str,
    pub align: ColumnAlign,
    pub title_color: Color,
    pub title_hint: String,
    pub title_font: Font,
    pub title_word_wrap: bool,
    pub auto_size: bool,
    pub resizable: bool,
    pub movable: bool,
}

#[note("grid-viewline")]
#[derive(Clone, Debug, PartialEq)]
pub enum ViewLine {
    Header {
        level: usize,
        title: String,
        count: String,
        totals: String,
    },
    Row {
        index: usize,
        id: u64,
        cells: Vec<String>,
        raw_cells: Vec<Option<String>>,
        odd: bool,
        selected: bool,
        marker: &'static str,
    },
}

#[note("grid-overlay")]
#[derive(Clone, Debug, PartialEq)]
pub struct Overlay {
    pub opacity: f64,
    pub title: String,
    pub detail: String,
}

#[note("grid-footer")]
#[derive(Clone, Debug, PartialEq)]
pub struct Footer {
    pub position: String,
    pub state: &'static str,
    pub order: String,
    pub timing: String,
}

#[note("grid-group-digits")]
fn group_digits(digits: &str) -> String {
    let bytes = digits.as_bytes();
    let len = bytes.len();
    let mut out = String::with_capacity(len + len / 3);
    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(b as char);
    }
    out
}

#[note("grid-format-thousands-int")]
fn format_thousands_int(n: usize) -> String {
    group_digits(&n.to_string())
}

#[note("grid-format-float-thousands")]
fn format_float_thousands(val: f64) -> String {
    let formatted = format!("{:.2}", val.abs());
    let (int_part, dec_part) = formatted.split_once('.').unwrap_or((&formatted, "00"));
    let is_negative = val < 0.0 && !(int_part == "0" && dec_part == "00");
    let grouped = group_digits(int_part);
    let mut out = String::with_capacity(grouped.len() + 4 + if is_negative { 1 } else { 0 });
    if is_negative {
        out.push('-');
    }
    out.push_str(&grouped);
    out.push('.');
    out.push_str(dec_part);
    out
}

/// Formats a cell value according to column rules: null text, value map, boolean texts, and display format.
pub fn cell_text(col: &GridColumn, kind: Option<&str>, value: Option<&str>) -> String {
    let raw = match value {
        None => return col.null_text.clone(),
        Some(s) => s,
    };
    if !col.edit_mask.is_empty() && col.display_format.is_empty() {
        return crawrad_ir::mask::shown_text(&col.edit_mask, raw);
    }
    let text = if let Some(mapped) = map_value(&col.value_map, raw) {
        mapped
    } else {
        let effective_kind = kind.or(col.kind.as_deref());
        let is_bool = effective_kind
            .is_some_and(|k| k.eq_ignore_ascii_case("BOOLEAN") || k.eq_ignore_ascii_case("BOOL"));
        match (
            is_bool,
            col.boolean_texts.split_once(';'),
            checkbox_state(raw),
        ) {
            (true, Some((true_text, _)), Some(true)) => true_text,
            (true, Some((_, false_text)), Some(false)) => false_text,
            _ => raw,
        }
    };
    format_display(text, &col.display_format)
}

#[note("grid-format-display")]
pub fn format_display(text: &str, mask: &str) -> String {
    if mask.is_empty() {
        return text.to_string();
    }
    let trimmed = text.trim();
    let Ok(val) = trimmed.parse::<f64>() else {
        return text.to_string();
    };
    if val.is_nan() || val.is_infinite() {
        return text.to_string();
    }

    let Some(first_digit) = mask.find(['0', '#']) else {
        return text.to_string();
    };
    let Some(last_digit) = mask.rfind(['0', '#']) else {
        return text.to_string();
    };

    let prefix = &mask[..first_digit];
    let suffix = &mask[last_digit + 1..];
    let pattern = &mask[first_digit..=last_digit];

    let (int_mask, frac_mask) = pattern.split_once('.').unwrap_or((pattern, ""));

    let use_grouping = int_mask.contains(',');
    let min_int_digits = int_mask.chars().filter(|&c| c == '0').count();
    let min_decimals = frac_mask.chars().filter(|&c| c == '0').count();
    let max_decimals = frac_mask.chars().filter(|&c| c == '0' || c == '#').count();

    let abs_val = val.abs();
    let factor = 10_f64.powi(max_decimals as i32);
    let rounded_scaled = (abs_val * factor + 0.5).floor();
    let rounded_val = rounded_scaled / factor;

    let formatted = if max_decimals == 0 {
        format!("{:.0}", rounded_val)
    } else {
        format!("{:.precision$}", rounded_val, precision = max_decimals)
    };

    let (raw_int, raw_frac) = formatted.split_once('.').unwrap_or((&formatted, ""));

    let mut int_part = raw_int.to_string();
    while int_part.len() < min_int_digits {
        int_part.insert(0, '0');
    }

    let mut dec_part = raw_frac.to_string();
    while dec_part.len() > min_decimals && dec_part.ends_with('0') {
        dec_part.pop();
    }

    let grouped_int = if use_grouping {
        group_digits(&int_part)
    } else {
        int_part
    };

    let is_zero =
        grouped_int.chars().all(|c| c == '0' || c == ',') && dec_part.chars().all(|c| c == '0');
    let sign = if val < 0.0 && !is_zero { "-" } else { "" };

    let dec_dot = if dec_part.is_empty() { "" } else { "." };

    format!("{prefix}{sign}{grouped_int}{dec_dot}{dec_part}{suffix}")
}

/// Parses aggregate enumeration strings (`smNone`, `smSum`, `smCount`, `smAvg`, `smMin`, `smMax`).
pub fn parse_aggregate(s: &str) -> Option<datagrid::Aggregate> {
    match s {
        "smSum" => Some(datagrid::Aggregate::Sum),
        "smCount" => Some(datagrid::Aggregate::Count),
        "smAvg" => Some(datagrid::Aggregate::Avg),
        "smMin" => Some(datagrid::Aggregate::Min),
        "smMax" => Some(datagrid::Aggregate::Max),
        _ => None,
    }
}

/// Formats a column's footer summary total according to its aggregate and format mask.
pub fn summary_text(col: &GridColumn, value: Option<f64>) -> String {
    let Some(val) = value else {
        return String::new();
    };
    let agg = parse_aggregate(&col.summary);
    let mask = if !col.summary_format.is_empty() {
        &col.summary_format
    } else {
        &col.display_format
    };
    match agg {
        Some(datagrid::Aggregate::Count) => {
            format!("{:.0}", val)
        }
        Some(datagrid::Aggregate::Sum) => {
            let formatted = format_display(&val.to_string(), mask);
            format!("Σ {formatted}")
        }
        Some(datagrid::Aggregate::Avg) => {
            let formatted = format_display(&val.to_string(), mask);
            format!("avg {formatted}")
        }
        Some(datagrid::Aggregate::Min) => {
            let formatted = format_display(&val.to_string(), mask);
            format!("min {formatted}")
        }
        Some(datagrid::Aggregate::Max) => {
            let formatted = format_display(&val.to_string(), mask);
            format!("max {formatted}")
        }
        None => String::new(),
    }
}

/// Pure aggregation calculation over an optional slice of string cells for canvas design rows.
pub fn summarize(values: &[Option<&str>], aggregate: &str) -> Option<f64> {
    let agg = parse_aggregate(aggregate)?;
    if agg == datagrid::Aggregate::Count {
        return Some(values.len() as f64);
    }
    let mut count = 0usize;
    let mut sum = 0.0f64;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    for &val_opt in values {
        if let Some(s) = val_opt
            && let Ok(v) = s.trim().parse::<f64>()
            && !v.is_nan()
        {
            count += 1;
            sum += v;
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
        }
    }

    if count == 0 {
        return None;
    }

    match agg {
        datagrid::Aggregate::Count => Some(values.len() as f64),
        datagrid::Aggregate::Sum => Some(sum),
        datagrid::Aggregate::Avg => Some(sum / count as f64),
        datagrid::Aggregate::Min => Some(min),
        datagrid::Aggregate::Max => Some(max),
    }
}

#[note("grid-itemlines")]
#[derive(Clone, Debug, PartialEq)]
pub struct ItemLines<'a> {
    pub headers: Vec<&'a ViewLine>,
    pub row: Option<&'a ViewLine>,
    pub is_stale: bool,
}

#[note("grid-line-for-item")]
pub fn line_for_item(view: &GridView, requested_first: usize, item: usize) -> ItemLines<'_> {
    if let Some((pos, row)) = view.lines.iter().enumerate().find_map(|(i, l)| match l {
        ViewLine::Row { index, .. } if *index == item => Some((i, l)),
        _ => None,
    }) {
        let mut headers = Vec::new();
        let mut k = pos;
        while k > 0 {
            k -= 1;
            match &view.lines[k] {
                h @ ViewLine::Header { .. } => headers.push(h),
                ViewLine::Row { .. } => break,
            }
        }
        headers.reverse();
        ItemLines {
            headers,
            row: Some(row),
            is_stale: false,
        }
    } else {
        let first_row_idx = view.lines.iter().find_map(|l| match l {
            ViewLine::Row { index, .. } => Some(*index),
            _ => None,
        });

        let row = if let Some(first_idx) = first_row_idx {
            if item >= requested_first {
                let offset = item - requested_first;
                let target = first_idx + offset;
                view.lines.iter().find(|l| match l {
                    ViewLine::Row { index, .. } => *index == target,
                    _ => false,
                })
            } else {
                None
            }
        } else {
            None
        };

        ItemLines {
            headers: Vec::new(),
            row,
            is_stale: true,
        }
    }
}

#[note("grid-unavailablesource")]
pub struct UnavailableSource {
    message: String,
}

impl UnavailableSource {
    #[note("grid-unavailablesource-new")]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// From the error that kept the source from opening, its own words only:
    /// the grid adds "backend error:" itself, so it is not said twice.
    #[note("grid-unavailablesource-from-error")]
    pub fn from_error(error: &datagrid::SourceError) -> Self {
        match error {
            datagrid::SourceError::Backend(message) => Self::new(message.clone()),
            other => Self::new(other.to_string()),
        }
    }
}

impl datagrid::RowSource for UnavailableSource {
    fn schema(&self) -> &[datagrid::ColumnInfo] {
        &[]
    }

    fn capabilities(&self) -> datagrid::Capabilities {
        datagrid::Capabilities::default()
    }

    fn count(&self, _query: &datagrid::Query) -> Result<usize, datagrid::SourceError> {
        Err(datagrid::SourceError::Backend(self.message.clone()))
    }

    fn fetch(
        &self,
        _query: &datagrid::Query,
        _range: std::ops::Range<usize>,
    ) -> Result<datagrid::Rows, datagrid::SourceError> {
        Err(datagrid::SourceError::Backend(self.message.clone()))
    }
}

#[cfg(test)]
#[path = "test/grid/test.rs"]
mod tests;
