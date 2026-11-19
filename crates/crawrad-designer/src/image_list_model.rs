//! The **ImageList Editor** without screen: the built-in icon catalog, the entry
//! of an image, and window state.
//!
//! Lives in `crawrad-designer`: it is pure Rust, and the line count ratchet
//! of `crawrad-ide` (ADR 0043) asks that what is pure leaves the shell.
//!
//! # Why it is a separate file
//!
//! The same reason as the other three models (`items_editor`, `lines_editor`,
//! `action_list`): the Makepad shell is another `[[bin]]`, and a `mod` does not cross
//! from one binary to the other. Here what cannot diverge is the **format of
//! what goes to the document** — `images_data` is a list of `{name, symbol}` maps,
//! and two different readings of it would give two lists of icons for the same
//! form.
//!
//! # The license boundary, which was already written
//!
//! The `PRESET_ICONS` are **not** the IDE chrome's icon set.
//! `icones.rs` explains this at length: Codicons is read only by the IDE, and what
//! is here *"becomes an icon in the customer's application — it is their content, not our
//! chrome"*. That is why they remain characters, and not a glyph name from
//! one of our families.

use crawrad_ir::Value;
use crawrad_note::note;

#[note("image-list-model-iconpreset")]
#[derive(Debug, Clone)]
pub struct IconPreset {
    pub name: &'static str,
    pub symbol: &'static str,
    pub category: &'static str,
}

pub const PRESET_ICONS: &[IconPreset] = &[
    IconPreset {
        name: "FileNew",
        symbol: "📄",
        category: "File",
    },
    IconPreset {
        name: "FileOpen",
        symbol: "📂",
        category: "File",
    },
    IconPreset {
        name: "FileSave",
        symbol: "💾",
        category: "File",
    },
    IconPreset {
        name: "FileSaveAs",
        symbol: "📑",
        category: "File",
    },
    IconPreset {
        name: "FilePrint",
        symbol: "🖨",
        category: "File",
    },
    IconPreset {
        name: "EditCut",
        symbol: "✂",
        category: "Edit",
    },
    IconPreset {
        name: "EditCopy",
        symbol: "📋",
        category: "Edit",
    },
    IconPreset {
        name: "EditPaste",
        symbol: "📌",
        category: "Edit",
    },
    IconPreset {
        name: "EditDelete",
        symbol: "❌",
        category: "Edit",
    },
    IconPreset {
        name: "EditUndo",
        symbol: "↩",
        category: "Edit",
    },
    IconPreset {
        name: "EditRedo",
        symbol: "↪",
        category: "Edit",
    },
    IconPreset {
        name: "SearchFind",
        symbol: "🔍",
        category: "Search",
    },
    IconPreset {
        name: "SearchReplace",
        symbol: "🔁",
        category: "Search",
    },
    IconPreset {
        name: "NavUp",
        symbol: "▲",
        category: "Navigation",
    },
    IconPreset {
        name: "NavDown",
        symbol: "▼",
        category: "Navigation",
    },
    IconPreset {
        name: "NavLeft",
        symbol: "◀",
        category: "Navigation",
    },
    IconPreset {
        name: "NavRight",
        symbol: "▶",
        category: "Navigation",
    },
    IconPreset {
        name: "Settings",
        symbol: "⚙",
        category: "General",
    },
    IconPreset {
        name: "User",
        symbol: "👤",
        category: "General",
    },
    IconPreset {
        name: "Folder",
        symbol: "📁",
        category: "General",
    },
    IconPreset {
        name: "CheckOK",
        symbol: "✔",
        category: "General",
    },
    IconPreset {
        name: "Warning",
        symbol: "⚠",
        category: "General",
    },
    IconPreset {
        name: "Info",
        symbol: "ℹ",
        category: "General",
    },
    IconPreset {
        name: "ActionRun",
        symbol: "⚡",
        category: "General",
    },
];

// -------------------------------------------------------------- the GALLERY
//
// # What the owner asked, and the measured answer
//
// *"Is there no way to import icons from outside, only from this gallery? And can more
// be added to the gallery to use there?"* (09/17/2026).
//
// Until now: **only** the built-in table above, and it is Rust — adding an
// icon meant recompiling the IDE. Neither shell had a port for that.
//
// Now the gallery is the sum of TWO lists: the built-in one, which comes in the binary, and
// the USER's, which lives in a file in the configuration folder and grows via the
// `Add Icons from Catalog` window itself.
//
// # What it still does NOT have, and that is another conversation
//
// **Importing a PNG or an SVG from outside.** Today an icon is a CHARACTER —
// `images_data` stores `{name, symbol}` —, and accepting a file would change two
// things at once: the data model (the image would have to go into the document
// or into the project disk) and GENERATION, which would need to emit the image into
// the customer's application. And there is the license boundary that `icones.rs` already
// declares. It is noted as a separate decision, not an oversight.

#[note("image-list-model-galleryicon")]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GalleryIcon {
    pub name: String,
    pub symbol: String,
    pub category: String,
    /// Came from the binary? Built-in icons are not deleted — deleting what the IDE
    /// promises is the most direct way for an old form to lose the icon
    /// it references.
    #[serde(rename = "de_fabrica", default)]
    pub builtin: bool,
}

#[note("image-list-model-gallery-file")]
pub const GALLERY_FILE: &str = "icones.json";

#[note("image-list-model-gallery")]
pub fn gallery(user_icons: &[GalleryIcon]) -> Vec<GalleryIcon> {
    let mut out: Vec<GalleryIcon> = PRESET_ICONS
        .iter()
        .map(|p| GalleryIcon {
            name: p.name.to_string(),
            symbol: p.symbol.to_string(),
            category: p.category.to_string(),
            builtin: true,
        })
        .collect();
    for i in user_icons {
        // DUPLICATE NAMES DO NOT ENTER TWICE. The built-in wins: it is what
        // already-saved forms reference by name.
        if out.iter().any(|j| j.name == i.name) {
            continue;
        }
        out.push(GalleryIcon {
            builtin: false,
            ..i.clone()
        });
    }
    out
}

#[note("image-list-model-load-gallery")]
pub fn load_gallery(path: &std::path::Path) -> Vec<GalleryIcon> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|t| serde_json::from_str::<Vec<GalleryIcon>>(&t).ok())
        .unwrap_or_default()
}

#[note("image-list-model-save-gallery")]
pub fn save_gallery(path: &std::path::Path, all: &[GalleryIcon]) -> Option<String> {
    let user_only: Vec<&GalleryIcon> = all.iter().filter(|i| !i.builtin).collect();
    let text = match serde_json::to_string_pretty(&user_only) {
        Ok(t) => t,
        Err(e) => return Some(e.to_string()),
    };
    std::fs::write(path, text).err().map(|e| e.to_string())
}

#[note("image-list-model-icon-refusal")]
pub fn icon_refusal(
    name: &str,
    symbol: &str,
    already_exists: &dyn Fn(&str) -> bool,
) -> Option<&'static str> {
    let name = name.trim();
    if name.is_empty() {
        return Some("the icon needs a name");
    }
    if symbol.trim().is_empty() {
        return Some("the icon needs a symbol");
    }
    if already_exists(name) {
        return Some("there is already an icon with that name");
    }
    None
}

#[note("image-list-model-imageentry")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageEntry {
    pub name: String,
    pub symbol: String,
}

impl ImageEntry {
    pub fn from_value(v: &Value) -> Option<Self> {
        let Value::Map(m) = v else { return None };
        let text = |k: &str| match m.get(k) {
            Some(Value::Str(s)) => s.clone(),
            _ => String::new(),
        };
        Some(Self {
            name: text("name"),
            symbol: text("symbol"),
        })
    }

    pub fn to_value(&self) -> Value {
        Value::Map(std::collections::BTreeMap::from([
            ("name".to_string(), Value::Str(self.name.clone())),
            ("symbol".to_string(), Value::Str(self.symbol.clone())),
        ]))
    }
}

#[derive(Debug, Clone)]
pub struct ImageListEditorState {
    /// The stable identity of the node (ADR 0008) — addresses, does not appear.
    pub list_id: String,
    /// The label that the user sees in the window title.
    pub list_name: String,
    pub width: i32,
    pub height: i32,
    pub entries: Vec<ImageEntry>,
    pub selected: Option<usize>,
    pub add_dialog_open: bool,
    pub add_selected_preset: Option<usize>,
}

impl ImageListEditorState {
    pub fn from_node(list_id: &str, node: &crawrad_ir::Node) -> Self {
        let width = match node.props.get("image_width") {
            Some(Value::Int(w)) => *w as i32,
            _ => 16,
        };
        let height = match node.props.get("image_height") {
            Some(Value::Int(h)) => *h as i32,
            _ => 16,
        };
        // An empty list opens empty, as in the classic RAD IDE. Filling it with samples would cause OK
        // to write icons to the document that the user never placed there.
        let entries = match node.props.get("images_data") {
            Some(Value::List(items)) => items.iter().filter_map(ImageEntry::from_value).collect(),
            _ => Vec::new(),
        };

        Self {
            list_id: list_id.to_string(),
            list_name: node.name().to_string(),
            width,
            height,
            entries,
            selected: Some(0),
            add_dialog_open: false,
            add_selected_preset: None,
        }
    }
}

impl ImageListEditorState {
    #[note("image-list-model-imagelisteditorstate-has-selection")]
    pub fn has_selection(&self) -> bool {
        self.selected.is_some_and(|i| i < self.entries.len())
    }

    #[note("image-list-model-imagelisteditorstate-delete")]
    pub fn delete(&mut self) {
        let Some(i) = self.selected.filter(|i| *i < self.entries.len()) else {
            return;
        };
        self.entries.remove(i);
        self.selected = match self.entries.len() {
            0 => None,
            n => Some(i.min(n - 1)),
        };
    }

    #[note("image-list-model-imagelisteditorstate-add-from-gallery")]
    pub fn add_from_gallery(&mut self, icon: &GalleryIcon) {
        self.add_icon(&icon.name, &icon.symbol);
    }

    fn add_icon(&mut self, name: &str, symbol: &str) {
        self.entries.push(ImageEntry {
            name: name.to_string(),
            symbol: symbol.to_string(),
        });
        self.selected = Some(self.entries.len() - 1);
    }

    #[note("image-list-model-imagelisteditorstate-value")]
    pub fn value(&self) -> Value {
        Value::List(self.entries.iter().map(ImageEntry::to_value).collect())
    }
}

#[note("image-list-model-sizes")]
pub const SIZES: [i32; 5] = [16, 24, 32, 48, 64];

#[note("image-list-model-gallery-label")]
pub fn gallery_label(i: &GalleryIcon) -> String {
    format!("{}  {} ({})", i.symbol, i.name, i.category)
}

#[note("image-list-model-no-category")]
pub const NO_CATEGORY: &str = "(No Category)";

#[note("image-list-model-classe-image-list")]
pub const CLASSE_IMAGE_LIST: &str = "ImageList";

#[note("image-list-model-iml-colunas")]
pub const IML_COLUNAS: usize = 6;

pub const IML_TAMANHO: &str = "Size:";
pub const IML_CONTA: &str = "Count: ";
pub const IML_NOTA: &str = "Transparent: Auto\nColor Depth: 32-bit (RGBA)";
pub const IML_SEM_IMAGEM: &str = "No image selected";

#[note("image-list-model-iml-key")]
pub const IML_KEY: &str = "images_data";
pub const IML_KEY_L: &str = "image_width";
pub const IML_KEY_A: &str = "image_height";
