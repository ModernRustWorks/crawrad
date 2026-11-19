//! LSP (Language Server Protocol) 3.17 data transfer objects and types.

use crawrad_note::note;
use serde::{Deserialize, Serialize};

#[note("protocol-position")]
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Position {
    /// Zero-based line position.
    pub line: u32,
    /// Zero-based character offset on a line.
    pub character: u32,
}

impl Position {
    #[must_use]
    pub const fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }
}

#[note("protocol-range")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Range {
    /// The range's start position.
    pub start: Position,
    /// The range's end position.
    pub end: Position,
}

impl Range {
    #[must_use]
    pub const fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

#[note("protocol-location")]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Location {
    /// The URI of the document.
    pub uri: String,
    /// The range inside the document.
    pub range: Range,
}

#[note("protocol-locations-from")]
pub fn locations_from(value: serde_json::Value) -> Vec<Location> {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct LocationLink {
        target_uri: String,
        target_selection_range: Range,
    }
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Place {
        Location(Location),
        Link(LocationLink),
    }
    let places: Vec<Place> = match value {
        serde_json::Value::Array(_) => serde_json::from_value(value).unwrap_or_default(),
        serde_json::Value::Null => Vec::new(),
        one => serde_json::from_value(one)
            .map(|p| vec![p])
            .unwrap_or_default(),
    };
    places
        .into_iter()
        .map(|p| match p {
            Place::Location(l) => l,
            Place::Link(l) => Location {
                uri: l.target_uri,
                range: l.target_selection_range,
            },
        })
        .collect()
}

#[note("protocol-diagnosticseverity")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

impl From<u8> for DiagnosticSeverity {
    fn from(val: u8) -> Self {
        match val {
            1 => Self::Error,
            2 => Self::Warning,
            3 => Self::Information,
            _ => Self::Hint,
        }
    }
}

impl From<DiagnosticSeverity> for u8 {
    fn from(s: DiagnosticSeverity) -> Self {
        match s {
            DiagnosticSeverity::Error => 1,
            DiagnosticSeverity::Warning => 2,
            DiagnosticSeverity::Information => 3,
            DiagnosticSeverity::Hint => 4,
        }
    }
}

#[note("protocol-diagnostic")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// The range at which the message applies.
    pub range: Range,
    /// The diagnostic's severity. Can be omitted. If omitted it is up to the client to interpret.
    pub severity: Option<DiagnosticSeverity>,
    /// The diagnostic's code (e.g., E0308, unused_variables).
    pub code: Option<String>,
    /// A human-readable string describing the source of this diagnostic (e.g. "rustc", "clippy").
    pub source: Option<String>,
    /// The diagnostic's message.
    pub message: String,
    /// LSP `DiagnosticTag`s: 1 Unnecessary (unused, faded), 2 Deprecated
    /// (struck through).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<u32>,
}

#[note("protocol-publishdiagnosticsparams")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishDiagnosticsParams {
    /// The URI for which diagnostic information is reported.
    pub uri: String,
    /// The version number of the document the diagnostics are published for.
    pub version: Option<i32>,
    /// An array of diagnostic information items.
    pub diagnostics: Vec<Diagnostic>,
}

#[note("protocol-completionitemkind")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "u32", into = "u32")]
pub enum CompletionItemKind {
    Text,
    Method,
    Function,
    Constructor,
    Field,
    Variable,
    Class,
    Interface,
    Module,
    Property,
    Unit,
    Value,
    Enum,
    Keyword,
    Snippet,
    Color,
    File,
    Reference,
    Folder,
    EnumMember,
    Constant,
    Struct,
    Event,
    Operator,
    TypeParameter,
    Unknown(u32),
}

impl From<u32> for CompletionItemKind {
    fn from(val: u32) -> Self {
        match val {
            1 => Self::Text,
            2 => Self::Method,
            3 => Self::Function,
            4 => Self::Constructor,
            5 => Self::Field,
            6 => Self::Variable,
            7 => Self::Class,
            8 => Self::Interface,
            9 => Self::Module,
            10 => Self::Property,
            11 => Self::Unit,
            12 => Self::Value,
            13 => Self::Enum,
            14 => Self::Keyword,
            15 => Self::Snippet,
            16 => Self::Color,
            17 => Self::File,
            18 => Self::Reference,
            19 => Self::Folder,
            20 => Self::EnumMember,
            21 => Self::Constant,
            22 => Self::Struct,
            23 => Self::Event,
            24 => Self::Operator,
            25 => Self::TypeParameter,
            other => Self::Unknown(other),
        }
    }
}

impl From<CompletionItemKind> for u32 {
    fn from(kind: CompletionItemKind) -> Self {
        match kind {
            CompletionItemKind::Text => 1,
            CompletionItemKind::Method => 2,
            CompletionItemKind::Function => 3,
            CompletionItemKind::Constructor => 4,
            CompletionItemKind::Field => 5,
            CompletionItemKind::Variable => 6,
            CompletionItemKind::Class => 7,
            CompletionItemKind::Interface => 8,
            CompletionItemKind::Module => 9,
            CompletionItemKind::Property => 10,
            CompletionItemKind::Unit => 11,
            CompletionItemKind::Value => 12,
            CompletionItemKind::Enum => 13,
            CompletionItemKind::Keyword => 14,
            CompletionItemKind::Snippet => 15,
            CompletionItemKind::Color => 16,
            CompletionItemKind::File => 17,
            CompletionItemKind::Reference => 18,
            CompletionItemKind::Folder => 19,
            CompletionItemKind::EnumMember => 20,
            CompletionItemKind::Constant => 21,
            CompletionItemKind::Struct => 22,
            CompletionItemKind::Event => 23,
            CompletionItemKind::Operator => 24,
            CompletionItemKind::TypeParameter => 25,
            CompletionItemKind::Unknown(val) => val,
        }
    }
}

#[note("protocol-documentation")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Documentation {
    String(String),
    MarkupContent { kind: String, value: String },
}

impl Documentation {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::String(s) => s.as_str(),
            Self::MarkupContent { value, .. } => value.as_str(),
        }
    }
}

#[note("protocol-completionitem")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionItem {
    /// The label of this completion item (by default also the text that is inserted).
    pub label: String,
    /// The kind of this completion item.
    pub kind: Option<CompletionItemKind>,
    /// A human-readable string with additional information about this item, like type or symbol information.
    pub detail: Option<String>,
    /// A human-readable string that represents a doc-comment.
    pub documentation: Option<Documentation>,
    /// A string that should be inserted into a document when selecting this completion.
    #[serde(rename = "insertText")]
    pub insert_text: Option<String>,
    /// A string that should be used when filtering a set of completion items.
    #[serde(rename = "filterText")]
    pub filter_text: Option<String>,
    /// A string that should be used when comparing this item with other items.
    #[serde(rename = "sortText")]
    pub sort_text: Option<String>,
    /// The edit the server wants; rust-analyzer sends this, not `insertText`.
    #[serde(rename = "textEdit", default, skip_serializing_if = "Option::is_none")]
    pub text_edit: Option<CompletionTextEdit>,
    /// `(as Clone)`, `fn(&self) -> String`: shown beside the label.
    #[serde(
        rename = "labelDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub label_details: Option<LabelDetails>,
    /// `2` when the text to insert is a snippet (`$1`, `${1:a}`, `$0`).
    #[serde(
        rename = "insertTextFormat",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub insert_text_format: Option<u8>,
    /// What the server needs back to resolve this item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// `CompletionItemTag`s: `1` is Deprecated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<u32>>,
    /// The tag's older form, which rust-analyzer still sends beside it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}

#[note("protocol-completiontextedit")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionTextEdit {
    #[serde(rename = "newText")]
    pub new_text: String,
}

#[note("protocol-labeldetails")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CompletionItem {
    #[note("protocol-completionitem-text-to-insert")]
    pub fn text_to_insert(&self) -> &str {
        self.text_edit
            .as_ref()
            .map(|e| e.new_text.as_str())
            .or(self.insert_text.as_deref())
            .unwrap_or(&self.label)
    }

    #[note("protocol-completionitem-signature")]
    pub fn signature(&self) -> Option<String> {
        self.label_details
            .as_ref()
            .and_then(|d| d.description.clone())
            .or_else(|| self.detail.clone())
    }

    #[note("protocol-completionitem-sort-key")]
    pub fn sort_key(&self) -> &str {
        self.sort_text.as_deref().unwrap_or(&self.label)
    }

    #[note("protocol-completionitem-is-deprecated")]
    pub fn is_deprecated(&self) -> bool {
        self.tags.as_ref().is_some_and(|t| t.contains(&1)) || self.deprecated == Some(true)
    }
}

#[note("protocol-completionresponse")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompletionResponse {
    Array(Vec<CompletionItem>),
    List {
        #[serde(rename = "isIncomplete")]
        is_incomplete: bool,
        items: Vec<CompletionItem>,
    },
}

impl CompletionResponse {
    #[note("protocol-completionresponse-is-incomplete")]
    #[must_use]
    pub fn is_incomplete(&self) -> bool {
        matches!(
            self,
            Self::List {
                is_incomplete: true,
                ..
            }
        )
    }

    #[must_use]
    pub fn into_items(self) -> Vec<CompletionItem> {
        match self {
            Self::Array(items) => items,
            Self::List { items, .. } => items,
        }
    }
}

#[note("protocol-hover")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hover {
    /// The hover's content.
    pub contents: Documentation,
    /// An optional range to highlight.
    pub range: Option<Range>,
}

#[note("protocol-line-messages")]
pub fn line_messages(diagnostics: &[Diagnostic]) -> Vec<(usize, String, u8)> {
    let fade_only =
        |d: &&Diagnostic| d.severity == Some(DiagnosticSeverity::Hint) && d.tags.contains(&1);
    let mut out: Vec<(usize, String, u8)> = diagnostics
        .iter()
        .filter(|d| !fade_only(d))
        .map(|d| {
            let severity = match d.severity {
                Some(DiagnosticSeverity::Error) => 0,
                Some(DiagnosticSeverity::Warning) => 1,
                _ => 2,
            };
            (d.range.start.line as usize, d.message.clone(), severity)
        })
        .collect();
    out.sort_by_key(|(line, _, severity)| (*line, *severity));
    out
}

#[note("protocol-tag-unnecessary")]
pub const TAG_UNNECESSARY: u32 = 1;

#[note("protocol-tag-deprecated")]
pub const TAG_DEPRECATED: u32 = 2;

#[note("protocol-mark")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mark {
    pub range: Range,
    /// The squiggle: red for an error, else the warning's.
    pub error: bool,
    /// Only Hints on the range: no squiggle, only the tags' look.
    pub hint: bool,
    /// Unnecessary: the text faded.
    pub faded: bool,
    /// Deprecated: the text struck through.
    pub struck: bool,
}

#[note("protocol-marks")]
pub fn marks(diagnostics: &[Diagnostic]) -> Vec<Mark> {
    let mut out: Vec<Mark> = Vec::new();
    for d in diagnostics {
        let error = d.severity == Some(DiagnosticSeverity::Error);
        let hint = d.severity == Some(DiagnosticSeverity::Hint);
        let faded = d.tags.contains(&TAG_UNNECESSARY);
        let struck = d.tags.contains(&TAG_DEPRECATED);
        match out.iter_mut().find(|m| m.range == d.range) {
            Some(m) => {
                m.error |= error;
                m.hint &= hint;
                m.faded |= faded;
                m.struck |= struck;
            }
            None => out.push(Mark {
                range: d.range,
                error,
                hint,
                faded,
                struck,
            }),
        }
    }
    out
}

impl Mark {
    #[note("protocol-mark-level")]
    pub fn level(&self) -> usize {
        if self.error {
            2
        } else {
            usize::from(!self.hint)
        }
    }
}

#[cfg(test)]
#[path = "test/protocol/test.rs"]
mod tests;
