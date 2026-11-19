//! Typography font editor — `Font`.
//!
//! # Folds open like `Margins`
//!
//! Shut, the row shows a summary of the font (`Segoe UI, 9.0pt, Bold`) with a fold chevron
//! and an ellipsis button opening the native font picker dialog.
//! Open, it unfolds into seven sub-rows:
//! - `Name`: installed system font families in a drop-down (`SubChoice`)
//! - `Size`: point size as decimal number (`SubText`)
//! - `Bold`, `Italic`, `Underline`, `StrikeOut`: checkbox toggles (`SubCheck`)
//! - `Color`: swatch and typed hexadecimal color (`SubColor`)
//!
//! # Every sub-row writes the WHOLE record
//!
//! Just as with `Margins`, editing any sub-row writes the entire updated `crawrad_ir::Font`
//! via `component.set_property`, preserving single-step Ctrl+Z undo semantics.

use crate::Args;
use crawrad_ir::{Color, Font, NodeId, Value};
use crawrad_registry::PropKind;

use super::super::editor::{Cell, Editor, Gesture, Outcome, PropEditor, Slot};
use super::super::icon;
use super::decimal::Decimal;
use super::{Kind, Subject, note};

pub const KIND: Kind = Kind { make, sample };

#[note("inspector-model-editors-font-min-size")]
const MIN_SIZE: f64 = 1.0;
const MAX_SIZE: f64 = 400.0;

#[note("inspector-model-editors-font-fonteditor")]
#[derive(Debug, Clone, PartialEq)]
pub struct FontEditor {
    node: NodeId,
    key: &'static str,
    value: Font,
    open: bool,
}

impl FontEditor {
    pub fn new(node: NodeId, key: &'static str, value: Font, open: bool) -> Self {
        Self {
            node,
            key,
            value,
            open,
        }
    }
}

impl PropEditor for FontEditor {
    fn template(&self) -> &'static str {
        "RowFoldOpen"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![
            Cell::Press {
                slot: Slot::Fold,
                text: if self.open {
                    icon::FOLD_OPEN.to_string()
                } else {
                    icon::FOLD_CLOSED.to_string()
                },
            },
            Cell::Caption {
                slot: Slot::Value,
                text: Value::Font(self.value.clone()).describe(),
            },
            Cell::Press {
                slot: Slot::Button,
                text: icon::OPEN_EDITOR.to_string(),
            },
        ]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Fold, Gesture::Pressed) => Outcome::Run(
                super::super::ACTION_TOGGLE_GROUP,
                Args::text("key", self.key.to_string()),
            ),
            (Slot::Button, Gesture::Pressed) => Outcome::Run(
                super::super::ACTION_PICK_FONT,
                Args::text("node", self.node.clone()).and("key", Value::Str(self.key.to_string())),
            ),
            _ => Outcome::Ignore,
        }
    }

    fn children(&self) -> Vec<(String, Editor)> {
        if !self.open {
            return Vec::new();
        }
        let installed = crawrad_os::font::installed_families();
        let options = installed.to_vec();
        let current = options.iter().position(|f| f == &self.value.name);

        vec![
            (
                "Name".to_string(),
                Box::new(FontNameSubRow {
                    whole: self.value.clone(),
                    options,
                    current,
                }) as Editor,
            ),
            (
                "Size".to_string(),
                Box::new(FontSizeSubRow {
                    whole: self.value.clone(),
                }) as Editor,
            ),
            (
                "Bold".to_string(),
                Box::new(FontCheckSubRow {
                    whole: self.value.clone(),
                    kind: FontCheckKind::Bold,
                }) as Editor,
            ),
            (
                "Italic".to_string(),
                Box::new(FontCheckSubRow {
                    whole: self.value.clone(),
                    kind: FontCheckKind::Italic,
                }) as Editor,
            ),
            (
                "Underline".to_string(),
                Box::new(FontCheckSubRow {
                    whole: self.value.clone(),
                    kind: FontCheckKind::Underline,
                }) as Editor,
            ),
            (
                "StrikeOut".to_string(),
                Box::new(FontCheckSubRow {
                    whole: self.value.clone(),
                    kind: FontCheckKind::StrikeOut,
                }) as Editor,
            ),
            (
                "Color".to_string(),
                Box::new(FontColorSubRow {
                    whole: self.value.clone(),
                }) as Editor,
            ),
        ]
    }
}

#[note("inspector-model-editors-font-fontnamesubrow")]
#[derive(Debug, Clone, PartialEq)]
struct FontNameSubRow {
    whole: Font,
    options: Vec<String>,
    current: Option<usize>,
}

impl PropEditor for FontNameSubRow {
    fn template(&self) -> &'static str {
        "SubChoice"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![Cell::Choice {
            slot: Slot::Value,
            options: self.options.clone(),
            current: self.current,
        }]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        match (slot, gesture) {
            (Slot::Value, Gesture::Chose(i)) => match self.options.get(i) {
                Some(chosen) => {
                    let mut font = self.whole.clone();
                    font.name = chosen.clone();
                    Outcome::Set(Value::Font(font))
                }
                None => Outcome::Ignore,
            },
            _ => Outcome::Ignore,
        }
    }
}

#[note("inspector-model-editors-font-fontsizesubrow")]
#[derive(Debug, Clone, PartialEq)]
struct FontSizeSubRow {
    whole: Font,
}

impl PropEditor for FontSizeSubRow {
    fn template(&self) -> &'static str {
        "SubText"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![Cell::Field {
            slot: Slot::Value,
            text: Decimal::show(f64::from(self.whole.size)),
        }]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        let (Slot::Value, Gesture::Typed(text)) = (slot, gesture) else {
            return Outcome::Ignore;
        };
        // Clamped like every number of this panel (`Decimal::parse`): `0` is
        // the smallest size and not an error, `abc` is refused.
        let parsed = match Decimal::parse(text, MIN_SIZE, MAX_SIZE) {
            Ok(v) => v as f32,
            Err(why) => return Outcome::Refuse(why),
        };
        let mut font = self.whole.clone();
        font.size = parsed;
        Outcome::Set(Value::Font(font))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FontCheckKind {
    Bold,
    Italic,
    Underline,
    StrikeOut,
}

#[note("inspector-model-editors-font-fontchecksubrow")]
#[derive(Debug, Clone, PartialEq)]
struct FontCheckSubRow {
    whole: Font,
    kind: FontCheckKind,
}

impl PropEditor for FontCheckSubRow {
    fn template(&self) -> &'static str {
        "SubCheck"
    }

    fn cells(&self) -> Vec<Cell> {
        let on = match self.kind {
            FontCheckKind::Bold => self.whole.bold,
            FontCheckKind::Italic => self.whole.italic,
            FontCheckKind::Underline => self.whole.underline,
            FontCheckKind::StrikeOut => self.whole.strike_out,
        };
        vec![Cell::Check {
            slot: Slot::Value,
            on,
        }]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        let (Slot::Value, Gesture::Toggled(on)) = (slot, gesture) else {
            return Outcome::Ignore;
        };
        let mut font = self.whole.clone();
        match self.kind {
            FontCheckKind::Bold => font.bold = on,
            FontCheckKind::Italic => font.italic = on,
            FontCheckKind::Underline => font.underline = on,
            FontCheckKind::StrikeOut => font.strike_out = on,
        }
        Outcome::Set(Value::Font(font))
    }
}

#[note("inspector-model-editors-font-fontcolorsubrow")]
#[derive(Debug, Clone, PartialEq)]
struct FontColorSubRow {
    whole: Font,
}

impl PropEditor for FontColorSubRow {
    fn template(&self) -> &'static str {
        "SubColor"
    }

    fn cells(&self) -> Vec<Cell> {
        vec![
            Cell::Swatch {
                slot: Slot::Swatch,
                rgba: [
                    self.whole.color.r,
                    self.whole.color.g,
                    self.whole.color.b,
                    self.whole.color.a,
                ],
            },
            Cell::Field {
                slot: Slot::Value,
                text: Value::Color(self.whole.color).describe(),
            },
        ]
    }

    fn gesture(&self, slot: Slot, gesture: Gesture<'_>) -> Outcome {
        let (Slot::Value, Gesture::Typed(text)) = (slot, gesture) else {
            return Outcome::Ignore;
        };
        match Color::from_hex(text.trim()) {
            Some(c) => {
                let mut font = self.whole.clone();
                font.color = c;
                Outcome::Set(Value::Font(font))
            }
            None => Outcome::Refuse(format!(
                "`{text}` is not a colour — write #RRGGBB or #RRGGBBAA"
            )),
        }
    }
}

#[note("inspector-model-editors-font-font-to-choice")]
pub fn font_to_choice(font: &Font) -> crawrad_os::font::FontChoice {
    crawrad_os::font::FontChoice {
        name: font.name.clone(),
        size: font.size,
        bold: font.bold,
        italic: font.italic,
        underline: font.underline,
        strike_out: font.strike_out,
        rgb: [font.color.r, font.color.g, font.color.b],
    }
}

#[note("inspector-model-editors-font-font-from-choice")]
pub fn font_from_choice(choice: &crawrad_os::font::FontChoice, initial: &Font) -> Font {
    let color = if [initial.color.r, initial.color.g, initial.color.b] == choice.rgb {
        initial.color
    } else {
        Color::rgb(choice.rgb[0], choice.rgb[1], choice.rgb[2])
    };
    Font {
        name: choice.name.clone(),
        size: choice.size,
        bold: choice.bold,
        italic: choice.italic,
        underline: choice.underline,
        strike_out: choice.strike_out,
        color,
    }
}

#[note("inspector-model-editors-font-pick-font")]
pub fn pick_font(
    doc: Option<&crawrad_ir::Document>,
    node: &NodeId,
    key: &str,
    font: Option<&str>,
) -> Option<Result<Font, String>> {
    let initial = match doc.and_then(|d| d.node(node)?.props.get(key)) {
        Some(Value::Font(f)) => f.clone(),
        _ => Font::default(),
    };
    let answer = font
        .map(str::to_string)
        .or_else(crawrad_os::font::test_answer);
    match answer {
        Some(text) if text.trim().is_empty() => None,
        Some(text) => {
            let read = Font::from_str_formatted(&text).filter(|f| !f.name.is_empty());
            Some(read.ok_or(format!("`{text}` is not a font — write `Name, 9pt, Bold`")))
        }
        None => crawrad_os::font::choose_font(&font_to_choice(&initial))
            .map(|choice| Ok(font_from_choice(&choice, &initial))),
    }
}

fn make(s: &Subject<'_>) -> Option<Editor> {
    match (&s.desc.kind, s.value) {
        (PropKind::Font, Value::Font(f)) => Some(Box::new(FontEditor::new(
            s.node.clone(),
            s.desc.key,
            f.clone(),
            s.open,
        ))),
        _ => None,
    }
}

#[note("inspector-model-editors-font-sample")]
fn sample() -> Editor {
    Box::new(FontEditor::new(
        "sample".to_string(),
        "font",
        Font::new("Segoe UI", 9.0).bold(true),
        true,
    ))
}
