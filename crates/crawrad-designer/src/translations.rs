//! Pure model for form translations.

use crawrad_ir::{Document, Node, NodeId, Value};
use crawrad_registry::Registry;
use std::collections::BTreeMap;

pub use crawrad_ir::translation_key::{TRANSLATABLE, text_key};

/// A translatable string found in the document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceText {
    pub key: String,
    pub label: String,
    pub node: NodeId,
    pub prop: String,
    pub text: String,
}

/// Extracts every non-empty translatable string from `doc` in paint order.
pub fn source_texts(doc: &Document, form: &str, reg: &Registry) -> Vec<SourceText> {
    let mut out = Vec::new();
    let node_ids = std::iter::once(doc.root.clone()).chain(doc.in_paint_order());
    for node_id in node_ids {
        let Some(node) = doc.node(&node_id) else {
            continue;
        };
        let Some(desc) = reg.get(&node.class) else {
            continue;
        };
        for &prop in TRANSLATABLE {
            if desc.prop(prop).is_none() {
                continue;
            }
            if let Some(Value::Str(text)) = node.props.get(prop)
                && !text.is_empty()
            {
                let label = if node.id == doc.root || node.class == "Form" {
                    format!("{form}.{prop}")
                } else {
                    format!("{}.{prop}", node.name())
                };
                out.push(SourceText {
                    key: text_key(form, &node.id, prop),
                    label,
                    node: node.id.clone(),
                    prop: prop.to_string(),
                    text: text.clone(),
                });
            }
        }
    }
    out
}

/// A translated entry with the English text it was translated from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Translation {
    pub text: String,
    pub from: String,
}

/// A set of translations for one target language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locale {
    pub code: String,
    pub entries: BTreeMap<String, Translation>,
}

impl Locale {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            entries: BTreeMap::new(),
        }
    }

    pub fn set(
        &mut self,
        key: impl Into<String>,
        text: impl Into<String>,
        from: impl Into<String>,
    ) {
        self.entries.insert(
            key.into(),
            Translation {
                text: text.into(),
                from: from.into(),
            },
        );
    }

    pub fn get(&self, key: &str) -> Option<&Translation> {
        self.entries.get(key)
    }

    pub fn from_tuples(code: impl Into<String>, tuples: Vec<(String, String, String)>) -> Self {
        let mut locale = Self::new(code);
        for (key, text, from) in tuples {
            locale.set(key, text, from);
        }
        locale
    }

    pub fn to_tuples(&self) -> Vec<(String, String, String)> {
        self.entries
            .iter()
            .map(|(k, v)| (k.clone(), v.text.clone(), v.from.clone()))
            .collect()
    }
}

/// The status of a translation cell in the grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CellState {
    Done(String),
    Stale(String),
    Missing,
}

/// Evaluates the state of a source text against a locale.
pub fn cell(source: &SourceText, locale: &Locale) -> CellState {
    let Some(translation) = locale.get(&source.key) else {
        return CellState::Missing;
    };
    if translation.text.is_empty() {
        return CellState::Missing;
    }
    if translation.from != source.text {
        CellState::Stale(translation.text.clone())
    } else {
        CellState::Done(translation.text.clone())
    }
}

/// Returns `(done_count, total_count)` for the given sources. Stale is not done.
pub fn progress(sources: &[SourceText], locale: &Locale) -> (usize, usize) {
    let done = sources
        .iter()
        .filter(|s| matches!(cell(s, locale), CellState::Done(_)))
        .count();
    (done, sources.len())
}

/// Clones `doc` replacing source texts with Done or Stale entries from `locale`.
pub fn localized(doc: &Document, form: &str, reg: &Registry, locale: &Locale) -> Document {
    let mut cloned = doc.clone();
    for source in source_texts(doc, form, reg) {
        match cell(&source, locale) {
            CellState::Done(text) | CellState::Stale(text) => {
                if let Some(node) = cloned.node_mut(&source.node) {
                    node.props.insert(source.prop, Value::Str(text));
                }
            }
            CellState::Missing => {}
        }
    }
    cloned
}

/// Returns the display name of a language code, in its own language where known.
pub fn language_name(code: &str) -> String {
    match code {
        "en" => "English",
        "pt-BR" => "Português (Brasil)",
        "pt-PT" => "Português (Portugal)",
        "es" => "Español",
        "fr" => "Français",
        "de" => "Deutsch",
        "it" => "Italiano",
        "ja" => "日本語",
        "zh-CN" | "zh-Hans" => "简体中文",
        "zh-TW" | "zh-Hant" => "繁體中文",
        "ru" => "Русский",
        "nl" => "Nederlands",
        "pl" => "Polski",
        "tr" => "Türkçe",
        _ => code,
    }
    .to_string()
}

/// Checks whether `code` is a valid BCP 47 locale code for a translation file.
pub fn valid_code(code: &str) -> bool {
    if code == "en" {
        return false;
    }
    let mut parts = code.split('-');
    let Some(lang) = parts.next() else {
        return false;
    };
    if !matches!(lang.len(), 2 | 3) || !lang.chars().all(|c| c.is_ascii_lowercase()) {
        return false;
    }
    match parts.next() {
        None => true,
        Some(sub) => {
            if parts.next().is_some() {
                return false;
            }
            let is_two_upper = sub.len() == 2 && sub.chars().all(|c| c.is_ascii_uppercase());
            let is_four_letters = sub.len() == 4 && sub.chars().all(|c| c.is_ascii_alphabetic());
            is_two_upper || is_four_letters
        }
    }
}

/// One row in the translations table for a single translatable source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRow {
    pub key: String,
    pub label: String,
    pub source: String,
    pub cells: Vec<CellState>,
}

/// A group of translation rows belonging to one form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableGroup {
    pub form: String,
    pub rows: Vec<TableRow>,
}

/// Builds the translation table grouped by form.
///
/// If `missing_only` is true, keeps only rows with at least one Missing or Stale cell,
/// and drops groups that have no remaining rows.
pub fn table(
    forms: &[(String, Vec<SourceText>)],
    locales: &[Locale],
    missing_only: bool,
) -> Vec<TableGroup> {
    let mut groups = Vec::new();
    for (form_name, sources) in forms {
        let mut rows = Vec::new();
        for source in sources {
            let cells: Vec<CellState> = locales.iter().map(|loc| cell(source, loc)).collect();
            let is_missing_or_stale = cells
                .iter()
                .any(|c| matches!(c, CellState::Missing | CellState::Stale(_)));
            if !missing_only || is_missing_or_stale {
                rows.push(TableRow {
                    key: source.key.clone(),
                    label: source.label.clone(),
                    source: source.text.clone(),
                    cells,
                });
            }
        }
        if !missing_only || !rows.is_empty() {
            groups.push(TableGroup {
                form: form_name.clone(),
                rows,
            });
        }
    }
    groups
}

/// Returns the number of rows that have at least one Missing or Stale translation cell.
pub fn missing_count(groups: &[TableGroup]) -> usize {
    groups
        .iter()
        .flat_map(|g| &g.rows)
        .filter(|r| {
            r.cells
                .iter()
                .any(|c| matches!(c, CellState::Missing | CellState::Stale(_)))
        })
        .count()
}

/// Generates JSON report for the `strings.list` MCP tool.
pub fn report(
    forms: &[(String, Vec<SourceText>)],
    locales: &[Locale],
    form_filter: Option<&str>,
    lang_filter: Option<&str>,
) -> serde_json::Value {
    let languages: Vec<serde_json::Value> = locales
        .iter()
        .filter(|loc| lang_filter.is_none_or(|l| loc.code == l))
        .map(|loc| {
            let all_sources: Vec<_> = forms
                .iter()
                .filter(|(f, _)| form_filter.is_none_or(|ff| f == ff))
                .flat_map(|(_, s)| s.iter().cloned())
                .collect();
            let (done, total) = progress(&all_sources, loc);
            serde_json::json!({
                "code": loc.code,
                "name": language_name(&loc.code),
                "done": done,
                "total": total,
            })
        })
        .collect();

    let mut rows = Vec::new();
    for (form_name, sources) in forms {
        if form_filter.is_some_and(|f| form_name != f) {
            continue;
        }
        for source in sources {
            let mut cells = serde_json::Map::new();
            for loc in locales {
                if lang_filter.is_some_and(|l| loc.code != l) {
                    continue;
                }
                let (state_str, text_str) = match cell(source, loc) {
                    CellState::Done(s) => ("done", s),
                    CellState::Stale(s) => ("stale", s),
                    CellState::Missing => ("missing", String::new()),
                };
                cells.insert(
                    loc.code.clone(),
                    serde_json::json!({
                        "state": state_str,
                        "text": text_str,
                    }),
                );
            }
            rows.push(serde_json::json!({
                "key": source.key,
                "label": source.label,
                "source": source.text,
                "cells": cells,
            }));
        }
    }

    serde_json::json!({
        "languages": languages,
        "rows": rows,
    })
}

#[cfg(test)]
#[path = "test/translations/test.rs"]
mod tests;

/// A locale as read from disk: its code and its `(key, text, from)` entries.
pub type LocaleFile = (String, Vec<(String, String, String)>);

/// Every form's source texts and every locale of a project: what the Translations grid, the
/// `strings.*` MCP tools and a commit all read, built once from the documents and the files.
pub struct Workbook {
    pub forms: Vec<(String, Vec<SourceText>)>,
    pub locales: Vec<Locale>,
}

impl Workbook {
    /// Collects the source texts of `docs` and turns the files into locales (in the given order).
    pub fn new(docs: &[(String, Document)], reg: &Registry, files: Vec<LocaleFile>) -> Self {
        let forms = docs
            .iter()
            .map(|(name, doc)| (name.clone(), source_texts(doc, name, reg)))
            .collect();
        let locales = files
            .into_iter()
            .map(|(code, entries)| Locale::from_tuples(code, entries))
            .collect();
        Self { forms, locales }
    }

    /// The grid, grouped by form; see [`table`].
    pub fn table(&self, missing_only: bool) -> Vec<TableGroup> {
        table(&self.forms, &self.locales, missing_only)
    }

    /// Rows with a missing or stale cell in any language.
    pub fn missing(&self) -> usize {
        missing_count(&self.table(false))
    }

    /// One `(language name, done, total)` per locale, for the toolbar chips.
    pub fn chips(&self) -> Vec<(String, usize, usize)> {
        let all: Vec<SourceText> = self
            .forms
            .iter()
            .flat_map(|(_, s)| s.iter().cloned())
            .collect();
        self.locales
            .iter()
            .map(|loc| {
                let (done, total) = progress(&all, loc);
                (language_name(&loc.code), done, total)
            })
            .collect()
    }

    /// The English text a key holds today — the `from` of a new translation.
    pub fn source_text(&self, key: &str) -> Option<&str> {
        self.forms
            .iter()
            .flat_map(|(_, s)| s)
            .find(|s| s.key == key)
            .map(|s| s.text.as_str())
    }

    /// The code of the locale in column `index`.
    pub fn code(&self, index: usize) -> Option<&str> {
        self.locales.get(index).map(|l| l.code.as_str())
    }

    /// `locales/pt-BR.toml · locales/de.toml`, for the header.
    pub fn files_line(&self) -> String {
        let files: Vec<String> = self
            .locales
            .iter()
            .map(|l| format!("locales/{}.toml", l.code))
            .collect();
        files.join(" · ")
    }

    /// The `strings.list` answer; see [`report`].
    pub fn report(&self, form: Option<&str>, lang: Option<&str>) -> serde_json::Value {
        report(&self.forms, &self.locales, form, lang)
    }
}

/// Builds the rows for the Designer language picker popup menu.
///
/// Returns `(code, label, checked)` with `"en"` (`English (source)`) always first.
pub fn language_rows(codes: &[String], current: Option<&str>) -> Vec<(String, String, bool)> {
    let cur = current.unwrap_or("en");
    let is_en = cur.is_empty() || cur == "en";
    let mut rows = Vec::with_capacity(codes.len() + 1);
    rows.push(("en".to_string(), "English (source)".to_string(), is_en));
    for code in codes {
        if code != "en" {
            let checked = !is_en && cur == code;
            rows.push((code.clone(), language_name(code), checked));
        }
    }
    rows
}

/// Builds the rows for the `Run in a language` section of the Run split button menu.
///
/// Returns `(code, label, checked)` with `None` (`System language`) first, then `English`,
/// then each project language code.
pub fn run_language_rows(
    codes: &[String],
    current: Option<&str>,
) -> Vec<(Option<String>, String, bool)> {
    let is_sys = current.is_none() || current == Some("");
    let is_en = current == Some("en");
    let mut rows = Vec::with_capacity(codes.len() + 2);
    rows.push((None, "System language".to_string(), is_sys));
    rows.push((Some("en".to_string()), "English".to_string(), is_en));
    for code in codes {
        if code != "en" {
            let checked = current == Some(code.as_str());
            rows.push((Some(code.clone()), language_name(code), checked));
        }
    }
    rows
}

/// Estimated average character width ratio relative to font size.
pub const CHAR_WIDTH_RATIO: f32 = 0.55;

/// Estimated total horizontal padding for a control's text.
pub const HORIZONTAL_PADDING: f32 = 8.0;

/// Estimates whether `text` fits within `width` at `font_size`.
///
/// For multi-line text (separated by `\n`), the estimate is based on the widest line.
/// The estimated advance is `chars * font_size * CHAR_WIDTH_RATIO + HORIZONTAL_PADDING`.
pub fn text_fits(text: &str, font_size: f32, width: f32) -> bool {
    let max_chars = text
        .split('\n')
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);
    let chars_f32 = match u16::try_from(max_chars) {
        Ok(c) => f32::from(c),
        Err(_) => f32::MAX,
    };
    let estimated = chars_f32 * font_size * CHAR_WIDTH_RATIO + HORIZONTAL_PADDING;
    estimated <= width
}

fn value_to_f32(val: &Value) -> Option<f32> {
    match val {
        Value::Int(i) => i16::try_from(*i).map(f32::from).ok(),
        Value::Float(f) => format!("{f}").parse::<f32>().ok(),
        Value::Font(f) => Some(f.size),
        Value::Str(s) => s.trim().parse::<f32>().ok(),
        _ => None,
    }
}

fn node_font_size(node: &Node, reg: &Registry) -> f32 {
    if let Some(val) = node.props.get("font.size")
        && let Some(sz) = value_to_f32(val)
        && sz > 0.0
    {
        return sz;
    }
    if let Some(Value::Font(f)) = node.props.get("font")
        && f.size > 0.0
    {
        return f.size;
    }
    if let Some(desc) = reg.get(&node.class) {
        if let Some(prop) = desc.prop("font.size")
            && let Some(sz) = value_to_f32(&prop.default)
            && sz > 0.0
        {
            return sz;
        }
        if let Some(prop) = desc.prop("font")
            && let Value::Font(f) = &prop.default
            && f.size > 0.0
        {
            return f.size;
        }
    }
    crawrad_ir::Font::default().size
}

/// Returns the names of controls whose translated text overflows their width, in document order.
pub fn overflowing(doc: &Document, code: &str, reg: &Registry, locale: &Locale) -> Vec<String> {
    if code.is_empty() || code == "en" || code != locale.code {
        return Vec::new();
    }
    let form_name = doc.node(&doc.root).map(|n| n.name()).unwrap_or(&doc.root);
    let mut sources = source_texts(doc, form_name, reg);
    if !sources
        .iter()
        .any(|s| !matches!(cell(s, locale), CellState::Missing))
        && form_name != doc.root.as_str()
    {
        sources = source_texts(doc, &doc.root, reg);
    }
    let mut out = Vec::new();
    for source in sources {
        if !matches!(source.prop.as_str(), "text" | "caption") {
            continue;
        }
        if source.node == doc.root {
            continue;
        }
        let translated = match cell(&source, locale) {
            CellState::Done(t) | CellState::Stale(t) => t,
            CellState::Missing => continue,
        };
        let Some(node) = doc.node(&source.node) else {
            continue;
        };
        if node.class == "Form" {
            continue;
        }
        let Some(rect) = node.rect() else {
            continue;
        };
        let font_size = node_font_size(node, reg);
        if !text_fits(&translated, font_size, rect.w) {
            let name = node.name().to_string();
            if !out.contains(&name) {
                out.push(name);
            }
        }
    }
    out
}

/// Returns the one-line banner message shown under the Designer bar when previewing
/// a non-English language. Returns `None` for English or empty code.
pub fn banner(code: &str) -> Option<String> {
    if code.is_empty() || code == "en" {
        None
    } else {
        Some(format!(
            "Showing {} — read-only texts. Switch back to English to edit them here, or edit them in Translations.",
            language_name(code)
        ))
    }
}

/// Returns the Designer banner message and whether all translated texts fit their controls.
///
/// Returns `None` when `code` is empty or `"en"`.
pub fn banner_with_fit(code: &str, overflowing: &[String]) -> Option<(String, bool)> {
    let base = banner(code)?;
    if overflowing.is_empty() {
        Some((format!("{base} All texts fit."), true))
    } else {
        let names = if overflowing.len() > 3 {
            let first_three: Vec<&str> = overflowing.iter().take(3).map(String::as_str).collect();
            format!("{}…", first_three.join(", "))
        } else {
            overflowing.join(", ")
        };
        let count = overflowing.len();
        let verb = if count == 1 { "text does" } else { "texts do" };
        Some((format!("{base} {count} {verb} not fit: {names}"), false))
    }
}

/// Returns the caption for the Designer bar language button.
pub fn language_button_text(code: Option<&str>) -> String {
    match code {
        None | Some("") | Some("en") => "Language: English (source)".to_string(),
        Some(c) => format!("Language: {}", language_name(c)),
    }
}
