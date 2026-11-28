//! The open folder is the workspace, and projects join it as **members**.
//!
//! This is [ADR 0017] with Amendment 1. Previously there was **one** document,
//! at a relative path hardcoded in code (the sample app's `form_main.crawform`), and it was
//! that exact path which silently swapped the user's document the day the IDE was
//! launched from another directory — `current_dir` was different, the relative path
//! pointed nowhere, and the IDE opened the embedded example as if nothing happened.
//! Since then, **everything here is absolute from the opened root**.
//!
//! The layout follows the classic RAD IDE `.groupproj`: the group holds the executable project
//! and each library project.
//!
//! ```text
//! MyGroup/                      the folder being opened: the group
//!   Cargo.toml                    [workspace] members / default-members
//!   app/
//!     Cargo.toml                  the executable
//!     src/
//!       form_main.crawform        the UI design
//!       form_main.designer.rs     ours, rewritten
//!       form_main.rs              YOURS
//! ```
//!
//! **The root is virtual** — only `[workspace]`, without `[package]` and without
//! its own code. This is the Amendment 1 decision, based on a single argument:
//! with a package root, adding a second application eventually forces choosing
//! between permanent asymmetry and **restructuring the user's directory** — and
//! restructuring someone's project is something this IDE must never do.
//!
//! **`default-members` is the active project**, not an IDE-specific field:
//! `Cargo.toml` already tracks which it is, and does so for `cargo` too. A separate
//! field alongside it would be the second source of truth that this entire ADR
//! exists to prevent.
//!
//! ## What this module is, and what it is not
//!
//! Everything here is **a pure function or a thin shell around one**: locating
//! documents under a root, deciding which to open, maintaining the list of recent
//! folders, building `cargo` invocations. None of them know `egui` or GUI frameworks,
//! and that is why they all have unit tests.
//!
//! ## Recent folders are GLOBAL state
//!
//! They cannot live inside a project directory: a folder list stored inside one of
//! them disappears with it, and does not exist on first launch — which is precisely
//! when the IDE needs to know where the user was. Therefore they go to the user
//! configuration directory (`%APPDATA%\CrawRAD\` on Windows), just like a modern editor.
//!

pub const QUIT_PERGUNTA: &str = "Save changes before closing?";
pub const QUIT_SAVE: &str = "Save";

use crawrad_note::note;
use std::path::{Path, PathBuf};

#[note("workspace-extension")]
pub const EXTENSION: &str = "crawform";

#[note("workspace-recent-limit")]
pub const RECENT_LIMIT: usize = 10;

/// Default number of recent projects retained.
pub const DEFAULT_RECENT_LIMIT: u32 = RECENT_LIMIT as u32;
/// The range of recent project limits the IDE accepts.
pub const RECENT_LIMIT_RANGE: std::ops::RangeInclusive<u32> = 1..=50;

#[note("workspace-manifest")]
pub const MANIFEST: &str = "Cargo.toml";

// --------------------------------------------------------------------- members

#[note("workspace-workspace-manifest")]
pub fn workspace_manifest(first_member: &str) -> String {
    format!(
        "[workspace]\nresolver = \"2\"\nmembers = [\"{first_member}\"]\ndefault-members = [\"{first_member}\"]\n"
    )
}

#[note("workspace-workspace-list")]
pub fn workspace_list(cargo_toml: &str, key: &str) -> Vec<String> {
    let mut in_workspace = false;
    let mut collecting = false;
    let mut raw = String::new();
    for line in cargo_toml.lines() {
        let line = line.trim();
        if !collecting && line.starts_with('[') {
            in_workspace = line.starts_with("[workspace]");
            continue;
        }
        if collecting {
            raw.push_str(line);
            if line.contains(']') {
                break;
            }
            continue;
        }
        if !in_workspace {
            continue;
        }
        let Some(rest) = line.strip_prefix(key) else {
            continue;
        };
        // `members` cannot match inside `default-members`, and checking the
        // NEXT character guarantees this — `strip_prefix` alone would accept
        // any key starting with ours.
        if !rest.trim_start().starts_with('=') {
            continue;
        }
        raw.push_str(rest);
        if rest.contains(']') {
            break;
        }
        collecting = true;
    }
    raw.split('"')
        .skip(1)
        .step_by(2)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

#[note("workspace-has-package")]
pub fn has_package(cargo_toml: &str) -> bool {
    cargo_toml
        .lines()
        .any(|l| l.trim_start().starts_with("[package]"))
}

#[note("workspace-members")]
pub fn members(root: &Path) -> Vec<PathBuf> {
    if let Ok(text) = std::fs::read_to_string(root.join(MANIFEST)) {
        let active = workspace_list(&text, "default-members");
        let mut names = active.clone();
        for m in workspace_list(&text, "members") {
            if !names.contains(&m) {
                names.push(m);
            }
        }
        if names.is_empty() {
            // Without `members`, the group is the package itself — or not a group at all.
            return if has_package(&text) {
                vec![root.to_path_buf()]
            } else {
                Vec::new()
            };
        }
        names
            .into_iter()
            // `members` accepts globs (`crates/*`), and this parser does not resolve
            // globs. A member that does not exist on disk cannot enter the list:
            // it would become a ghost folder in the Explorer.
            .map(|n| root.join(n))
            .filter(|p| p.join(MANIFEST).is_file())
            .collect()
    } else {
        Vec::new()
    }
}

#[note("workspace-document-member")]
pub fn document_member(document: &Path) -> Option<PathBuf> {
    let src = document.parent()?;
    let name = src.file_name()?;
    if name != std::ffi::OsStr::new("src") {
        return None;
    }
    Some(src.parent()?.to_path_buf())
}

// ------------------------------------------------------------------ documents

#[note("workspace-documents-in")]
pub fn documents_in(root: &Path) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = Vec::new();
    for member in members(root) {
        for doc in member_documents(&member) {
            if !found.contains(&doc) {
                found.push(doc);
            }
        }
    }
    found
}

#[note("workspace-member-documents")]
pub fn member_documents(member: &Path) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = Vec::new();
    let Ok(entries) = std::fs::read_dir(member.join("src")) else {
        return Vec::new();
    };
    for entry in entries.flatten().map(|e| e.path()).filter(|c| c.is_file()) {
        // `eq_ignore_ascii_case`: on Windows the disk does not distinguish
        // `.CRAWFORM` from `.crawform`, and a document saved by another
        // tool cannot disappear from the list because of casing.
        if entry
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case(EXTENSION))
            && !found.contains(&entry)
        {
            found.push(entry);
        }
    }
    in_order(found)
}

#[note("workspace-in-order")]
fn in_order(mut documents: Vec<PathBuf>) -> Vec<PathBuf> {
    documents.sort();
    documents
}

#[note("workspace-onopen")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnOpen {
    /// No `.crawform` in the folder. Not an error: a folder without a form yet,
    /// which can be a console project or a library.
    NoForm,
    /// Exactly one. Opens directly, which is what the user expects.
    One(PathBuf),
    /// More than one. **The IDE does not choose for the user.** Guessing which to open
    /// belongs to the same bug family as the relative path: the IDE decides on its own
    /// and the user finds out later looking at the wrong form.
    Choose(Vec<PathBuf>),
}

#[note("workspace-decide")]
pub fn decide(documents: Vec<PathBuf>) -> OnOpen {
    let mut documents = documents;
    match documents.len() {
        0 => OnOpen::NoForm,
        1 => OnOpen::One(documents.remove(0)),
        _ => OnOpen::Choose(documents),
    }
}

// ------------------------------------------------------------ recent folders

#[note("workspace-config-folder")]
pub fn config_folder() -> Option<PathBuf> {
    let base = if cfg!(windows) {
        std::env::var_os("APPDATA").map(PathBuf::from)
    } else {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
    }?;
    let ours = base.join(if cfg!(windows) { "CrawRAD" } else { "crawrad" });
    std::fs::create_dir_all(&ours).ok()?;
    Some(ours)
}

/// The recent projects as a menu wants them: a label and the folder it opens.
///
/// It is built from `welcome_rows` on purpose. The project the Welcome page calls
/// `crawrad` must be called `crawrad` in the File menu too, and a second notion
/// of a project's name is how two places start disagreeing about one thing.
///
/// It lives here, beside the list it reads, rather than in the shell: the shell
/// is under a downward line ratchet and this is data work, not drawing.
#[note("workspace-recent-menu-items")]
pub fn recent_menu_items() -> Vec<(String, PathBuf)> {
    welcome_rows()
        .into_iter()
        .map(|r| (format!("{}  \u{2014}  {}", r.name, r.place), r.folder))
        .collect()
}

#[note("workspace-recent-folders-file")]
fn recent_folders_file() -> Option<PathBuf> {
    Some(config_folder()?.join("recent-folders.txt"))
}

#[note("workspace-read-recent")]
pub fn read_recent() -> Vec<PathBuf> {
    let limit = read_preferences().recent_limit as usize;
    let list = with_settings(|s| s.recent_folders(limit)).unwrap_or_default();
    prune(&list, |p| p.is_dir())
}

#[note("workspace-save-recent")]
pub fn save_recent(list: &[PathBuf]) {
    let limit = read_preferences().recent_limit as usize;
    with_settings(|s| {
        for p in list.iter().take(limit).rev() {
            let _ = s.push_recent_folder(p);
        }
        let all = s.recent_folders(usize::MAX);
        if all.len() > limit {
            for extra in &all[limit..] {
                let _ = s.forget_recent_folder(extra);
            }
        }
    });
}

#[note("workspace-from-text")]
pub fn from_text(text: &str) -> Vec<PathBuf> {
    let mut list: Vec<PathBuf> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let path = PathBuf::from(line);
        if !list.contains(&path) {
            list.push(path);
        }
    }
    list
}

#[note("workspace-to-text")]
pub fn to_text(list: &[PathBuf]) -> String {
    let mut text = String::new();
    for p in list {
        text.push_str(&p.to_string_lossy());
        text.push('\n');
    }
    text
}

pub fn register_recent_with_limit(list: &[PathBuf], new_path: &Path, limit: usize) -> Vec<PathBuf> {
    let mut out = Vec::with_capacity(list.len() + 1);
    if limit == 0 {
        return out;
    }
    out.push(new_path.to_path_buf());
    for p in list {
        if p != new_path && out.len() < limit {
            out.push(p.clone());
        }
    }
    out
}

#[note("workspace-register-recent")]
pub fn register_recent(list: &[PathBuf], new_path: &Path) -> Vec<PathBuf> {
    let limit = read_preferences().recent_limit as usize;
    register_recent_with_limit(list, new_path, limit)
}

#[note("workspace-remember-recent")]
pub fn remember_recent(folder: &Path) {
    let limit = read_preferences().recent_limit as usize;
    with_settings(|s| {
        let _ = s.push_recent_folder(folder);
        let all = s.recent_folders(usize::MAX);
        if all.len() > limit {
            for extra in &all[limit..] {
                let _ = s.forget_recent_folder(extra);
            }
        }
    });
}

#[note("workspace-last-project-form")]
pub fn last_project_form(
    p: &Preferences,
    recent: &[PathBuf],
    documents: impl Fn(&Path) -> Vec<PathBuf>,
) -> Option<PathBuf> {
    if p.welcome_on_startup {
        return None;
    }
    let root = recent.first()?;
    // The form that WAS open, when it is still there and still belongs to that
    // project. Taking the first document the folder lists meant reopening whichever
    // form sorted first — a newly added, empty one wins over the one being worked on.
    if let Some(f) = p.last_form.as_ref()
        && f.starts_with(root)
        && f.is_file()
    {
        return Some(f.clone());
    }
    documents(root).into_iter().next()
}

#[note("workspace-forget-recent")]
pub fn forget_recent(folder: &Path) {
    with_settings(|s| s.forget_recent_folder(folder));
}

#[note("workspace-welcomerow")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WelcomeRow {
    /// The folder `project.open_folder` gets when the row is clicked.
    pub folder: PathBuf,
    /// The folder's own name, in bold.
    pub name: String,
    /// Where it sits: the folder above it.
    pub place: String,
    /// `no form`, `1 form`, `3 forms`.
    pub forms: String,
    /// The age of its newest form (`today`, `5 days ago`), or empty.
    pub age: String,
}

/// Longest folder path a Welcome row shows on one line.
pub const PLACE_MAX_CHARS: usize = 56;

/// `text` cut to `max` chars, keeping both ends with `…` in the middle.
#[note("workspace-shorten-end")]
pub fn shorten_end(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_string();
    }
    let head: String = text.chars().take(max.saturating_sub(1)).collect();
    format!("{head}…")
}

#[note("workspace-shorten-middle")]
pub fn shorten_middle(text: &str, max: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= max || max < 3 {
        return text.to_string();
    }
    let keep = max - 1;
    let head = keep / 2;
    let tail = keep - head;
    let start: String = chars[..head].iter().collect();
    let end: String = chars[chars.len() - tail..].iter().collect();
    format!("{start}…{end}")
}

impl WelcomeRow {
    #[note("workspace-welcomerow-of")]
    pub fn of(recent: &Recent, now: std::time::SystemTime) -> Self {
        let name = recent.path.file_name().map_or_else(
            || recent.path.display().to_string(),
            |n| n.to_string_lossy().into_owned(),
        );
        let place = recent
            .path
            .parent()
            .map(|p| shorten_middle(&p.display().to_string(), PLACE_MAX_CHARS))
            .unwrap_or_default();
        let forms = match recent.forms {
            0 => "no form".to_string(),
            1 => "1 form".to_string(),
            n => format!("{n} forms"),
        };
        Self {
            folder: recent.path.clone(),
            name,
            place,
            forms,
            age: recent
                .modified
                .map(|t| time_ago(now, t))
                .unwrap_or_default(),
        }
    }
}

#[note("workspace-welcome-rows")]
pub fn welcome_rows() -> Vec<WelcomeRow> {
    let now = std::time::SystemTime::now();
    read_recent()
        .iter()
        .map(|f| WelcomeRow::of(&describe_recent(f), now))
        .collect()
}

#[note("workspace-prune")]
pub fn prune(list: &[PathBuf], exists: impl Fn(&Path) -> bool) -> Vec<PathBuf> {
    let limit = read_preferences().recent_limit as usize;
    list.iter()
        .filter(|p| exists(p))
        .take(limit)
        .cloned()
        .collect()
}

// ------------------------------------------------ what is known of a recent

#[note("workspace-recent")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recent {
    pub path: PathBuf,
    /// How many `.crawform` files the folder has, via the SAME scan that opening uses.
    /// A separate counter here could say 3 while opening finds 2.
    pub forms: usize,
    /// When the folder's most recent design changed.
    ///
    /// **It is the MODIFICATION date, not the last opened date**, and the label on
    /// screen says so explicitly. When the folder was opened is recorded by no one:
    /// `recent-folders.txt` stores the ORDER, and the timestamp of each opening is
    /// nowhere. Writing "opened 2 days ago" on top of an `mtime` would be the screen
    /// precisely asserting something it does not know.
    ///
    /// `None` when the folder has no drawings and its directory date also does not
    /// respond — not an error, the row simply omits what it does not know.
    pub modified: Option<std::time::SystemTime>,
}

#[note("workspace-describe-recent")]
pub fn describe_recent(folder: &Path) -> Recent {
    let docs = documents_in(folder);
    // The most recent among the designs, and the folder itself as a last resort:
    // a console project has no `.crawform` files and still has an age.
    let when = docs
        .iter()
        .filter_map(|c| std::fs::metadata(c).ok()?.modified().ok())
        .max()
        .or_else(|| std::fs::metadata(folder).ok()?.modified().ok());
    Recent {
        forms: docs.len(),
        modified: when,
        path: folder.to_path_buf(),
    }
}

#[note("workspace-time-ago")]
pub fn time_ago(now: std::time::SystemTime, then: std::time::SystemTime) -> String {
    // Clocks that move backward exist — timezone changes, NTP, file copied from another
    // machine. `duration_since` returns an error in that case, and the honest answer
    // is today's, not a negative number masquerading as the future.
    let Ok(d) = now.duration_since(then) else {
        return "today".to_string();
    };
    let days = d.as_secs() / 86_400;
    match days {
        0 => "today".to_string(),
        1 => "yesterday".to_string(),
        2..=13 => format!("{days} days ago"),
        14..=60 => format!("{} weeks ago", days / 7),
        61..=364 => format!("{} months ago", days / 30),
        _ => {
            let years = days / 365;
            if years == 1 {
                "a year ago".to_string()
            } else {
                format!("{years} years ago")
            }
        }
    }
}
// -------------------------------------------------- user preferences

#[note("workspace-autosave")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AutoSave {
    /// Files are written only when asked — the default.
    #[default]
    Off,
    /// [`AutoSave::DELAY_SECONDS`] after the last keystroke.
    AfterDelay,
    /// When the editor gives the keyboard away.
    EditorFocusLost,
    /// When the IDE's window does.
    WindowFocusLost,
}

impl AutoSave {
    pub const ALL: [Self; 4] = [
        Self::Off,
        Self::AfterDelay,
        Self::EditorFocusLost,
        Self::WindowFocusLost,
    ];

    #[note("workspace-autosave-delay-seconds")]
    pub const DELAY_SECONDS: f64 = 1.0;

    #[note("workspace-autosave-label")]
    pub fn label(self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::AfterDelay => "After a delay (1000 ms)",
            Self::EditorFocusLost => "When the editor loses focus",
            Self::WindowFocusLost => "When the window loses focus",
        }
    }

    #[note("workspace-autosave-labels")]
    pub fn labels() -> Vec<String> {
        Self::ALL.iter().map(|a| a.label().to_string()).collect()
    }

    #[note("workspace-autosave-position")]
    pub fn position(self) -> usize {
        Self::ALL.iter().position(|a| *a == self).unwrap_or(0)
    }

    #[note("workspace-autosave-key")]
    pub fn key(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::AfterDelay => "after-delay",
            Self::EditorFocusLost => "editor-focus-lost",
            Self::WindowFocusLost => "window-focus-lost",
        }
    }

    #[note("workspace-autosave-from-key")]
    pub fn from_key(s: &str) -> Self {
        Self::ALL
            .into_iter()
            .find(|a| a.key() == s.trim())
            .unwrap_or_default()
    }
}

#[note("workspace-editorcolorscheme")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditorColorScheme {
    #[default]
    ModernDark,
    ClassicDark,
    Monokai,
    Custom,
}

impl EditorColorScheme {
    pub const ALL: &'static [Self] = &[
        Self::ModernDark,
        Self::ClassicDark,
        Self::Monokai,
        Self::Custom,
    ];

    #[note("workspace-editorcolorscheme-short-name")]
    pub fn short_name(self) -> &'static str {
        match self {
            Self::ModernDark => "Modern Dark",
            Self::ClassicDark => "Classic Dark",
            Self::Monokai => "Vivid Dark",
            Self::Custom => "Custom",
        }
    }

    #[note("workspace-editorcolorscheme-preset-labels")]
    pub fn preset_labels(base: EditorColorScheme) -> Vec<String> {
        vec![
            "Modern Dark".to_string(),
            "Classic Dark".to_string(),
            "Vivid Dark".to_string(),
            format!("Custom (based on {})", base.short_name()),
        ]
    }

    #[note("workspace-editorcolorscheme-position")]
    pub fn position(self) -> usize {
        Self::ALL.iter().position(|s| *s == self).unwrap_or(0)
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::ModernDark => "Modern Dark (Default)",
            Self::ClassicDark => "Classic Dark",
            Self::Monokai => "Vivid Dark",
            Self::Custom => "Custom...",
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            Self::ModernDark => "modern-dark",
            Self::ClassicDark => "classic-dark",
            Self::Monokai => "monokai",
            Self::Custom => "custom",
        }
    }

    pub fn from_key(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "classic-dark" | "classic" => Self::ClassicDark,
            "monokai" => Self::Monokai,
            "custom" => Self::Custom,
            _ => Self::ModernDark,
        }
    }

    pub fn colors(self, custom: &EditorCustomColors) -> EditorTokenColors {
        match self {
            Self::ModernDark => EditorTokenColors {
                branch_keyword: [1.0, 0.482, 0.447, 1.0], // #ff7b72
                loop_keyword: [1.0, 0.482, 0.447, 1.0],
                other_keyword: [1.0, 0.482, 0.447, 1.0],
                typename: [0.306, 0.788, 0.690, 1.0], // #4ec9b0
                function: [0.475, 0.753, 1.0, 1.0],   // #79c0ff
                identifier: [0.831, 0.831, 0.831, 1.0], // #d4d4d4
                comment: [0.545, 0.580, 0.620, 1.0],  // #8b949e
                number: [0.710, 0.808, 0.659, 1.0],   // #b5cea8
                string: [0.808, 0.569, 0.471, 1.0],   // #ce9178
                constant: [0.808, 0.569, 0.471, 1.0],
                delimiter: [1.0, 0.843, 0.0, 1.0], // #ffd700
                delimiter_highlight: [1.0, 1.0, 1.0, 1.0],
                punctuator: [0.831, 0.831, 0.831, 1.0],
                bg: [0.118, 0.118, 0.118, 1.0], // #1e1e1e
            },
            Self::ClassicDark => EditorTokenColors {
                branch_keyword: [0.769, 0.522, 0.745, 1.0], // #c485be
                loop_keyword: [1.0, 0.549, 0.0, 1.0],       // #ff8c00
                other_keyword: [0.357, 0.608, 0.827, 1.0],  // #5b9bd3
                typename: [0.337, 0.788, 0.694, 1.0],       // #56c9b1
                function: [1.0, 0.988, 0.788, 1.0],         // #fffcc9
                identifier: [0.831, 0.831, 0.831, 1.0],     // #d4d4d4
                comment: [0.388, 0.553, 0.329, 1.0],        // #638d54
                number: [0.714, 0.808, 0.667, 1.0],         // #b6ceaa
                string: [0.8, 0.569, 0.482, 1.0],           // #cc917b
                constant: [0.8, 0.569, 0.482, 1.0],
                delimiter: [0.667, 0.667, 0.667, 1.0], // #aaaaaa
                delimiter_highlight: [1.0, 1.0, 1.0, 1.0],
                punctuator: [0.831, 0.831, 0.831, 1.0],
                bg: [0.122, 0.137, 0.165, 1.0], // #1f232a
            },
            Self::Monokai => EditorTokenColors {
                branch_keyword: [0.976, 0.149, 0.447, 1.0], // #f92672
                loop_keyword: [0.976, 0.149, 0.447, 1.0],
                other_keyword: [0.976, 0.149, 0.447, 1.0],
                typename: [0.400, 0.851, 0.937, 1.0], // #66d9ef
                function: [0.651, 0.886, 0.180, 1.0], // #a6e22e
                identifier: [0.973, 0.973, 0.949, 1.0], // #f8f8f2
                comment: [0.459, 0.443, 0.369, 1.0],  // #75715e
                number: [0.682, 0.506, 1.0, 1.0],     // #ae81ff
                string: [0.902, 0.859, 0.455, 1.0],   // #e6db74
                constant: [0.682, 0.506, 1.0, 1.0],
                delimiter: [0.973, 0.973, 0.949, 1.0],
                delimiter_highlight: [1.0, 1.0, 1.0, 1.0],
                punctuator: [0.973, 0.973, 0.949, 1.0],
                bg: [0.153, 0.157, 0.133, 1.0], // #272822
            },
            Self::Custom => {
                let def = Self::ModernDark.colors(custom);
                let kw = hex_to_rgba(&custom.keyword).unwrap_or(def.branch_keyword);
                let ty = hex_to_rgba(&custom.typename).unwrap_or(def.typename);
                let func = hex_to_rgba(&custom.function).unwrap_or(def.function);
                let id = hex_to_rgba(&custom.identifier).unwrap_or(def.identifier);
                let comm = hex_to_rgba(&custom.comment).unwrap_or(def.comment);
                let num = hex_to_rgba(&custom.number).unwrap_or(def.number);
                let str_col = hex_to_rgba(&custom.string).unwrap_or(def.string);
                let delim = hex_to_rgba(&custom.delimiter).unwrap_or(def.delimiter);
                let bg = hex_to_rgba(&custom.bg).unwrap_or(def.bg);
                EditorTokenColors {
                    branch_keyword: kw,
                    loop_keyword: kw,
                    other_keyword: kw,
                    typename: ty,
                    function: func,
                    identifier: id,
                    comment: comm,
                    number: num,
                    string: str_col,
                    constant: str_col,
                    delimiter: delim,
                    delimiter_highlight: [1.0, 1.0, 1.0, 1.0],
                    punctuator: id,
                    bg,
                }
            }
        }
    }

    pub fn cores(self, custom: &EditorCustomColors) -> EditorTokenColors {
        self.colors(custom)
    }
}

#[note("workspace-editorcustomcolors")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorCustomColors {
    pub keyword: String,
    pub typename: String,
    pub function: String,
    pub identifier: String,
    pub comment: String,
    pub number: String,
    pub string: String,
    pub delimiter: String,
    pub bg: String,
}

impl Default for EditorCustomColors {
    fn default() -> Self {
        Self {
            keyword: "#ff7b72".to_string(),
            typename: "#4ec9b0".to_string(),
            function: "#79c0ff".to_string(),
            identifier: "#d4d4d4".to_string(),
            comment: "#8b949e".to_string(),
            number: "#b5cea8".to_string(),
            string: "#ce9178".to_string(),
            delimiter: "#ffd700".to_string(),
            bg: "#1e1e1e".to_string(),
        }
    }
}

impl EditorCustomColors {
    #[note("workspace-editorcustomcolors-fields")]
    pub fn fields(&self) -> [&str; 9] {
        [
            &self.keyword,
            &self.typename,
            &self.function,
            &self.identifier,
            &self.number,
            &self.string,
            &self.comment,
            &self.delimiter,
            &self.bg,
        ]
    }

    #[note("workspace-editorcustomcolors-update-from-fields")]
    pub fn update_from_fields(&mut self, inputs: &[String; 9]) -> bool {
        let mut changed = false;
        let targets = [
            &mut self.keyword,
            &mut self.typename,
            &mut self.function,
            &mut self.identifier,
            &mut self.number,
            &mut self.string,
            &mut self.comment,
            &mut self.delimiter,
            &mut self.bg,
        ];
        for (target, input) in targets.into_iter().zip(inputs) {
            let value = input.trim();
            if hex_to_rgba(value).is_some() && target != value {
                *target = value.to_string();
                changed = true;
            }
        }
        changed
    }
}

#[note("workspace-editordiagnosticcolors")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorDiagnosticColors {
    pub error: String,
    pub warning: String,
    pub info: String,
    pub error_band: String,
    pub warning_band: String,
    pub info_band: String,
}

impl Default for EditorDiagnosticColors {
    fn default() -> Self {
        Self {
            error: "#f38ba8".to_string(),
            warning: "#fab387".to_string(),
            info: "#89b4fa".to_string(),
            error_band: "#3a2230".to_string(),
            warning_band: "#3a2c22".to_string(),
            info_band: "#1d2236".to_string(),
        }
    }
}

impl EditorDiagnosticColors {
    #[note("workspace-editordiagnosticcolors-keys")]
    pub const KEYS: [&'static str; 6] = [
        "editor-diagnostic-error",
        "editor-diagnostic-warning",
        "editor-diagnostic-info",
        "editor-diagnostic-error-band",
        "editor-diagnostic-warning-band",
        "editor-diagnostic-info-band",
    ];

    #[note("workspace-editordiagnosticcolors-fields")]
    pub fn fields(&self) -> [&str; 6] {
        [
            &self.error,
            &self.warning,
            &self.info,
            &self.error_band,
            &self.warning_band,
            &self.info_band,
        ]
    }

    fn targets(&mut self) -> [&mut String; 6] {
        [
            &mut self.error,
            &mut self.warning,
            &mut self.info,
            &mut self.error_band,
            &mut self.warning_band,
            &mut self.info_band,
        ]
    }

    #[note("workspace-editordiagnosticcolors-update-from-fields")]
    pub fn update_from_fields(&mut self, inputs: &[String; 6]) -> bool {
        let mut changed = false;
        for (target, input) in self.targets().into_iter().zip(inputs) {
            let value = input.trim();
            if hex_to_rgba(value).is_some() && target != value {
                *target = value.to_string();
                changed = true;
            }
        }
        changed
    }

    #[note("workspace-editordiagnosticcolors-set-by-key")]
    fn set_by_key(&mut self, key: &str, value: &str) -> bool {
        match Self::KEYS.iter().position(|k| *k == key) {
            Some(i) => {
                if !value.is_empty() {
                    *self.targets()[i] = value.to_string();
                }
                true
            }
            None => false,
        }
    }

    #[note("workspace-editordiagnosticcolors-rgba")]
    pub fn rgba(&self) -> [[f32; 4]; 6] {
        let def = Self::default();
        let mut out = [[0.0; 4]; 6];
        for (i, (value, fallback)) in self.fields().into_iter().zip(def.fields()).enumerate() {
            out[i] = hex_to_rgba(value)
                .or_else(|| hex_to_rgba(fallback))
                .unwrap_or([1.0; 4]);
        }
        out
    }
}

impl EditorColorScheme {
    #[note("workspace-editorcolorscheme-preset")]
    pub fn preset(self) -> Option<EditorCustomColors> {
        let palette = |c: [&str; 9]| EditorCustomColors {
            keyword: c[0].into(),
            typename: c[1].into(),
            function: c[2].into(),
            identifier: c[3].into(),
            comment: c[4].into(),
            number: c[5].into(),
            string: c[6].into(),
            delimiter: c[7].into(),
            bg: c[8].into(),
        };
        match self {
            Self::ModernDark => Some(EditorCustomColors::default()),
            Self::ClassicDark => Some(palette([
                "#569cd6", "#4ec9b0", "#dcdcaa", "#d4d4d4", "#6a9955", "#b5cea8", "#ce9178",
                "#d4d4d4", "#1e1e1e",
            ])),
            Self::Monokai => Some(palette([
                "#f92672", "#66d9ef", "#a6e22e", "#f8f8f2", "#75715e", "#ae81ff", "#e6db74",
                "#f8f8f2", "#272822",
            ])),
            Self::Custom => None,
        }
    }
}

#[note("workspace-editortokencolors")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EditorTokenColors {
    pub branch_keyword: [f32; 4],
    pub loop_keyword: [f32; 4],
    pub other_keyword: [f32; 4],
    pub typename: [f32; 4],
    pub function: [f32; 4],
    pub identifier: [f32; 4],
    pub comment: [f32; 4],
    pub number: [f32; 4],
    pub string: [f32; 4],
    pub constant: [f32; 4],
    pub delimiter: [f32; 4],
    pub delimiter_highlight: [f32; 4],
    pub punctuator: [f32; 4],
    pub bg: [f32; 4],
}

#[note("workspace-hex-to-rgba")]
pub fn hex_to_rgba(hex: &str) -> Option<[f32; 4]> {
    let s = hex.trim().strip_prefix('#').unwrap_or(hex.trim());
    if s.len() == 6 {
        let r = u8::from_str_radix(&s[0..2], 16).ok()? as f32 / 255.0;
        let g = u8::from_str_radix(&s[2..4], 16).ok()? as f32 / 255.0;
        let b = u8::from_str_radix(&s[4..6], 16).ok()? as f32 / 255.0;
        Some([r, g, b, 1.0])
    } else if s.len() == 8 {
        let r = u8::from_str_radix(&s[0..2], 16).ok()? as f32 / 255.0;
        let g = u8::from_str_radix(&s[2..4], 16).ok()? as f32 / 255.0;
        let b = u8::from_str_radix(&s[4..6], 16).ok()? as f32 / 255.0;
        let a = u8::from_str_radix(&s[6..8], 16).ok()? as f32 / 255.0;
        Some([r, g, b, a])
    } else if s.len() == 3 {
        let r = u8::from_str_radix(&s[0..1].repeat(2), 16).ok()? as f32 / 255.0;
        let g = u8::from_str_radix(&s[1..2].repeat(2), 16).ok()? as f32 / 255.0;
        let b = u8::from_str_radix(&s[2..3].repeat(2), 16).ok()? as f32 / 255.0;
        Some([r, g, b, 1.0])
    } else {
        None
    }
}

#[note("workspace-not-a-colour")]
pub const NOT_A_COLOUR: &str = "Not a colour: use #RRGGBB";

#[note("workspace-color-fields")]
pub const COLOR_FIELDS: [(&str, &str); 15] = [
    ("kw", "Keywords"),
    ("type", "Types"),
    ("fn", "Functions"),
    ("id", "Identifiers"),
    ("num", "Numbers"),
    ("str", "Strings"),
    ("com", "Comments"),
    ("delim", "Delimiters"),
    ("bg", "Background"),
    ("diag_err", "Error"),
    ("diag_warn", "Warning"),
    ("diag_info", "Info"),
    ("diag_err_band", "Error band"),
    ("diag_warn_band", "Warning band"),
    ("diag_info_band", "Info band"),
];

#[note("workspace-preferences")]
#[derive(Debug, Clone, PartialEq)]
pub struct Preferences {
    /// Open welcome screen instead of reopening last folder.
    pub welcome_on_startup: bool,
    /// Active keymap scheme in the IDE (default `Modern`, or classic RAD IDE).
    pub keymap_scheme: crawrad_actions::KeymapScheme,
    /// Code editor font size in points (default: 11).
    pub editor_font_size: u32,
    /// Code editor tab size in spaces (default: 4).
    pub editor_tab_size: u8,
    /// Show line numbers in editor gutter (default: true).
    pub editor_line_numbers: bool,
    /// The types rust-analyzer infers, in grey inside the lines (default: on).
    pub editor_inlay_hints: bool,
    /// The caret's line gets a background band and a bold number (default: on).
    pub editor_current_line: bool,
    /// Whether breadcrumb segments use chevron styling (default: true).
    pub editor_breadcrumb_chevron: bool,
    /// Color scheme and syntax highlighting for editor (default: ModernDark).
    pub editor_color_scheme: EditorColorScheme,
    /// Base preset scheme for custom colors (default: ModernDark).
    pub editor_color_base: EditorColorScheme,
    /// Custom editor colors when scheme is `Custom`.
    pub editor_custom_colors: EditorCustomColors,
    /// The colours of a diagnostic's line: message, number and band per
    /// severity. Apart from the scheme: picking a preset keeps them.
    pub editor_diagnostic_colors: EditorDiagnosticColors,
    /// Recent colors of the Color dialog, at most 8 hex strings.
    pub color_recent: Vec<String>,
    /// Default directory for opening and creating projects.
    pub default_projects_dir: Option<PathBuf>,
    /// The form that was open, so reopening brings back THAT one.
    pub last_form: Option<PathBuf>,
    /// Whether local file history snapshots are recorded (default: true).
    pub history_enabled: bool,
    /// Maximum number of local history revisions kept per file (default: 10, range: 1..=100).
    pub history_limit: u32,
    /// Maximum number of recent projects retained in the recent list (default: 10, range: 1..=50).
    pub recent_limit: u32,
    /// Window in seconds within which subsequent saves replace the previous snapshot (default: 10, range: 0..=3600).
    pub history_merge_seconds: u32,
    /// Maximum file size in kilobytes eligible for local history recording (default: 5120 KB = 5 MB, range: 1..=1048576).
    pub history_max_file_kb: u32,
    /// Every open editor file is saved before Build or Run, not only the one
    /// in view (default: off).
    pub save_before_build: bool,
    /// When the editor files save by themselves (default: off).
    pub auto_save: AutoSave,
    /// Keep the project desktop when it is closed (default: on).
    pub save_desktop: bool,
    /// The desktop profile last applied or saved (default: `Default`).
    pub desktop_profile: String,
    /// Keep recovery copies of the unsaved editor files (default: on).
    pub recover_copies: bool,
    /// Minutes between recovery copies (default: 10).
    pub recover_minutes: u32,
    /// The Form Designer's grid size, in [`DESIGNER_GRID_RANGE`] (default: 8).
    pub designer_grid: u32,
    /// Snap to grid (default: on).
    pub designer_snap: bool,
    /// Display grid (default: on).
    pub designer_show_grid: bool,
    /// The Tab-order badges on the form (default: on); the owner turns them off when they
    /// get in the way of the design.
    pub designer_show_tab_order: bool,
    /// Form Designer zoom factor (default: `crawrad_runtime::ZOOM_ONE`, range: 0.25..=4.0).
    pub designer_scale: f64,
    /// The whole IDE's interface scale (View › Zoom), an IDE setting kept
    /// across restarts (owner, 13/11). Default 1.0.
    pub ide_scale: f64,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            // The recent list, not the last project. Reopening was the default and
            // nobody asked for it: the IDE decided, on a machine it had never run on,
            // that a person wanted to be dropped straight into a file. Options still
            // offers the other behaviour to whoever wants it.
            welcome_on_startup: true,
            keymap_scheme: crawrad_actions::KeymapScheme::Modern,
            editor_font_size: 11,
            editor_tab_size: 4,
            editor_line_numbers: true,
            editor_inlay_hints: true,
            editor_current_line: true,
            editor_breadcrumb_chevron: true,
            editor_color_scheme: EditorColorScheme::ModernDark,
            editor_color_base: EditorColorScheme::ModernDark,
            editor_custom_colors: EditorCustomColors::default(),
            editor_diagnostic_colors: EditorDiagnosticColors::default(),
            color_recent: Vec::new(),
            default_projects_dir: None,
            last_form: None,
            history_enabled: true,
            history_limit: 10,
            recent_limit: DEFAULT_RECENT_LIMIT,
            history_merge_seconds: 10,
            history_max_file_kb: 5120,
            save_before_build: false,
            auto_save: AutoSave::Off,
            save_desktop: true,
            desktop_profile: crate::profile::DEFAULT.to_string(),
            recover_copies: true,
            recover_minutes: 10,
            designer_grid: 8,
            designer_snap: true,
            designer_show_grid: true,
            designer_show_tab_order: true,
            designer_scale: crawrad_runtime::ZOOM_ONE as f64,
            ide_scale: 1.0,
        }
    }
}

impl Preferences {
    #[note("workspace-preferences-editor-colors")]
    pub fn editor_colors(&self) -> EditorTokenColors {
        self.editor_color_scheme.colors(&self.editor_custom_colors)
    }

    /// Returns true if the active editor theme background is dark.
    pub fn is_dark(&self) -> bool {
        let [r, g, b, _] = self.editor_colors().bg;
        (0.299 * r + 0.587 * g + 0.114 * b) < 0.5
    }

    #[note("workspace-preferences-color-fields")]
    pub fn color_fields(&self) -> [&str; 15] {
        let (syntax, diagnostic) = (
            self.editor_custom_colors.fields(),
            self.editor_diagnostic_colors.fields(),
        );
        std::array::from_fn(|i| if i < 9 { syntax[i] } else { diagnostic[i - 9] })
    }

    #[note("workspace-preferences-update-color-fields")]
    pub fn update_color_fields(&mut self, inputs: &[String; 15]) -> bool {
        let syntax: [String; 9] = std::array::from_fn(|i| inputs[i].clone());
        let diagnostic: [String; 6] = std::array::from_fn(|i| inputs[i + 9].clone());
        let syntax_changed = self.editor_custom_colors.update_from_fields(&syntax);
        if syntax_changed {
            self.editor_color_scheme = EditorColorScheme::Custom;
        }
        self.editor_diagnostic_colors
            .update_from_fields(&diagnostic)
            | syntax_changed
    }

    #[note("workspace-preferences-pick-scheme")]
    pub fn pick_scheme(&mut self, scheme: EditorColorScheme) {
        self.editor_color_scheme = scheme;
        if scheme != EditorColorScheme::Custom {
            self.editor_color_base = scheme;
            if let Some(preset) = scheme.preset() {
                self.editor_custom_colors = preset;
            }
        }
    }

    #[note("workspace-preferences-reset-colors")]
    pub fn reset_colors(&mut self) {
        self.editor_color_scheme = EditorColorScheme::ModernDark;
        self.editor_color_base = EditorColorScheme::ModernDark;
        self.editor_custom_colors = EditorCustomColors::default();
        self.editor_diagnostic_colors = EditorDiagnosticColors::default();
    }

    #[note("workspace-preferences-color-field-errors")]
    pub fn color_field_errors(inputs: &[String; 15]) -> [Option<&'static str>; 15] {
        std::array::from_fn(|i| {
            let val = inputs[i].trim();
            if hex_to_rgba(val).is_some() {
                None
            } else {
                Some(NOT_A_COLOUR)
            }
        })
    }
}

#[note("workspace-key-welcome")]
pub const KEY_WELCOME: &str = "welcome-on-startup";
pub const KEY_KEYMAP_SCHEME: &str = "keymap-scheme";
pub const KEY_EDITOR_FONT_SIZE: &str = "editor-font-size";
pub const KEY_EDITOR_TAB_SIZE: &str = "editor-tab-size";
pub const KEY_EDITOR_LINE_NUMBERS: &str = "editor-line-numbers";
pub const KEY_EDITOR_INLAY_HINTS: &str = "editor-inlay-hints";
pub const KEY_EDITOR_CURRENT_LINE: &str = "editor-current-line";
pub const KEY_EDITOR_BREADCRUMB_CHEVRON: &str = "editor-breadcrumb-chevron";
pub const KEY_EDITOR_COLOR_SCHEME: &str = "editor-color-scheme";
pub const KEY_EDITOR_COLOR_BASE: &str = "editor-color-base";
pub const KEY_COLOR_RECENT: &str = "color-recent";
pub const KEY_DEFAULT_PROJECTS_DIR: &str = "default-projects-dir";
pub const KEY_LAST_FORM: &str = "last-form";
pub const KEY_HISTORY_ENABLED: &str = "history-enabled";
pub const KEY_HISTORY_LIMIT: &str = "history-limit";
pub const KEY_RECENT_LIMIT: &str = "recent-limit";
pub const KEY_HISTORY_MERGE_SECONDS: &str = "history-merge-seconds";
pub const KEY_HISTORY_MAX_FILE_KB: &str = "history-max-file-kb";
pub const KEY_SAVE_BEFORE_BUILD: &str = "save-before-build";
pub const KEY_AUTO_SAVE: &str = "auto-save";
pub const KEY_SAVE_DESKTOP: &str = "save-desktop";
pub const KEY_DESKTOP_PROFILE: &str = "desktop-profile";
pub const KEY_RECOVER_COPIES: &str = "recover-copies";
pub const KEY_RECOVER_MINUTES: &str = "recover-minutes";
pub const KEY_DESIGNER_GRID: &str = "designer-grid";
pub const KEY_DESIGNER_SNAP: &str = "designer-snap";
pub const KEY_DESIGNER_SHOW_GRID: &str = "designer-show-grid";
pub const KEY_DESIGNER_SHOW_TAB_ORDER: &str = "designer-show-tab-order";
pub const KEY_DESIGNER_SCALE: &str = "designer-zoom";
pub const KEY_IDE_SCALE: &str = "ide-zoom";
/// The interface scales the IDE accepts (View › Zoom).
pub const IDE_SCALE_RANGE: std::ops::RangeInclusive<f64> = 0.5..=2.0;
#[note("workspace-designer-grid-range")]
pub const DESIGNER_GRID_RANGE: std::ops::RangeInclusive<u32> = 2..=64;

fn preferences_file() -> Option<PathBuf> {
    Some(config_folder()?.join("preferences.txt"))
}

/// The IDE's one settings connection, opened on first use and kept for the
/// whole process: some readers run on every sync, and opening SQLite each
/// time would cost more than the text file it replaced.
static SETTINGS: std::sync::OnceLock<Option<std::sync::Mutex<crawrad_settings::SettingsStore>>> =
    std::sync::OnceLock::new();

fn with_settings<T>(f: impl FnOnce(&mut crawrad_settings::SettingsStore) -> T) -> Option<T> {
    let cell = SETTINGS.get_or_init(|| {
        let config_dir = config_folder()?;
        let mut store = crawrad_settings::SettingsStore::open(&config_dir.join("ide.db")).ok()?;
        migrate_text_files_if_needed(&mut store, &config_dir);
        Some(std::sync::Mutex::new(store))
    });
    let mut store = cell.as_ref()?.lock().ok()?;
    Some(f(&mut store))
}

fn migrate_text_files_if_needed(store: &mut crawrad_settings::SettingsStore, config_dir: &Path) {
    if !store.all().is_empty() {
        return;
    }
    let pref_path = preferences_file().unwrap_or_else(|| config_dir.join("preferences.txt"));
    if pref_path.is_file() {
        if let Ok(text) = std::fs::read_to_string(&pref_path) {
            let p = preferences_from_text(&text);
            let serialized = preferences_to_text(p);
            let mut pairs = Vec::new();
            for line in serialized.lines() {
                if let Some((k, v)) = line.split_once('=') {
                    pairs.push((k.trim(), v.trim()));
                }
            }
            if !pairs.is_empty() {
                let _ = store.set_many(&pairs);
            }
        }
        let migrated = config_dir.join("preferences.txt.migrated");
        let _ = std::fs::rename(&pref_path, migrated);
    }

    let recent_path =
        recent_folders_file().unwrap_or_else(|| config_dir.join("recent-folders.txt"));
    if recent_path.is_file() {
        if let Ok(text) = std::fs::read_to_string(&recent_path) {
            let folders = from_text(&text);
            for f in folders.iter().rev() {
                let _ = store.push_recent_folder(f);
            }
        }
        let migrated = config_dir.join("recent-folders.txt.migrated");
        let _ = std::fs::rename(&recent_path, migrated);
    }
}

#[note("workspace-read-preferences")]
pub fn read_preferences() -> Preferences {
    let rows = with_settings(|s| s.all()).unwrap_or_default();
    if rows.is_empty() {
        return Preferences::default();
    }
    let mut text = String::new();
    for (k, v) in rows {
        text.push_str(&k);
        text.push('=');
        text.push_str(&v);
        text.push('\n');
    }
    preferences_from_text(&text)
}

#[note("workspace-save-preferences")]
pub fn save_preferences(p: Preferences) {
    let text = preferences_to_text(p);
    let mut pairs = Vec::new();
    for line in text.lines() {
        if let Some((k, v)) = line.split_once('=') {
            pairs.push((k.trim(), v.trim()));
        }
    }
    with_settings(|s| s.set_many(&pairs));
}

#[note("workspace-update-preferences")]
pub fn update_preferences(change: impl FnOnce(&mut Preferences)) {
    let mut p = read_preferences();
    change(&mut p);
    save_preferences(p);
}

#[note("workspace-preferences-from-text")]
pub fn preferences_from_text(text: &str) -> Preferences {
    let mut p = Preferences::default();
    for line in text.lines() {
        let Some((key, val)) = line.split_once('=') else {
            continue;
        };
        let c = key.trim();
        let v = val.trim();
        if c == KEY_WELCOME {
            p.welcome_on_startup = v == "true";
        } else if c == KEY_KEYMAP_SCHEME {
            p.keymap_scheme = if v == crawrad_actions::KeymapScheme::ALL[1].id() {
                crawrad_actions::KeymapScheme::ALL[1]
            } else {
                crawrad_actions::KeymapScheme::Modern
            };
        } else if c == KEY_EDITOR_FONT_SIZE {
            if let Ok(tam) = v.parse::<u32>()
                && (8..=32).contains(&tam)
            {
                p.editor_font_size = tam;
            }
        } else if c == KEY_EDITOR_TAB_SIZE {
            if let Ok(tab) = v.parse::<u8>()
                && (1..=8).contains(&tab)
            {
                p.editor_tab_size = tab;
            }
        } else if c == KEY_EDITOR_LINE_NUMBERS {
            p.editor_line_numbers = v == "true";
        } else if c == KEY_EDITOR_INLAY_HINTS {
            p.editor_inlay_hints = v == "true";
        } else if c == KEY_EDITOR_CURRENT_LINE {
            p.editor_current_line = v == "true";
        } else if c == KEY_EDITOR_BREADCRUMB_CHEVRON {
            p.editor_breadcrumb_chevron = v == "true";
        } else if c == KEY_EDITOR_COLOR_SCHEME {
            p.editor_color_scheme = EditorColorScheme::from_key(v);
        } else if c == KEY_EDITOR_COLOR_BASE {
            p.editor_color_base = EditorColorScheme::from_key(v);
        } else if c == KEY_COLOR_RECENT {
            p.color_recent = v
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty() && hex_to_rgba(s).is_some())
                .take(8)
                .map(str::to_string)
                .collect();
        } else if c == "editor-color-keyword" && !v.is_empty() {
            p.editor_custom_colors.keyword = v.to_string();
        } else if c == "editor-color-typename" && !v.is_empty() {
            p.editor_custom_colors.typename = v.to_string();
        } else if c == "editor-color-function" && !v.is_empty() {
            p.editor_custom_colors.function = v.to_string();
        } else if c == "editor-color-identifier" && !v.is_empty() {
            p.editor_custom_colors.identifier = v.to_string();
        } else if c == "editor-color-comment" && !v.is_empty() {
            p.editor_custom_colors.comment = v.to_string();
        } else if c == "editor-color-number" && !v.is_empty() {
            p.editor_custom_colors.number = v.to_string();
        } else if c == "editor-color-string" && !v.is_empty() {
            p.editor_custom_colors.string = v.to_string();
        } else if c == "editor-color-delimiter" && !v.is_empty() {
            p.editor_custom_colors.delimiter = v.to_string();
        } else if c == "editor-color-bg" && !v.is_empty() {
            p.editor_custom_colors.bg = v.to_string();
        } else if p.editor_diagnostic_colors.set_by_key(c, v) {
        } else if c == KEY_LAST_FORM && !v.is_empty() {
            p.last_form = Some(PathBuf::from(v));
        } else if c == KEY_DEFAULT_PROJECTS_DIR && !v.is_empty() {
            p.default_projects_dir = Some(PathBuf::from(v));
        } else if c == KEY_HISTORY_ENABLED {
            p.history_enabled = v == "true";
        } else if c == KEY_SAVE_BEFORE_BUILD {
            p.save_before_build = v == "true";
        } else if c == KEY_AUTO_SAVE {
            p.auto_save = AutoSave::from_key(v);
        } else if c == KEY_SAVE_DESKTOP {
            p.save_desktop = v != "false";
        } else if c == KEY_DESKTOP_PROFILE && !v.trim().is_empty() {
            p.desktop_profile = v.trim().to_string();
        } else if c == KEY_DESIGNER_SNAP {
            p.designer_snap = v != "false";
        } else if c == KEY_DESIGNER_SHOW_GRID {
            p.designer_show_grid = v != "false";
        } else if c == KEY_DESIGNER_SHOW_TAB_ORDER {
            p.designer_show_tab_order = v != "false";
        } else if c == KEY_DESIGNER_GRID {
            let grid = v.parse().ok().filter(|g| DESIGNER_GRID_RANGE.contains(g));
            p.designer_grid = grid.unwrap_or(p.designer_grid);
        } else if c == KEY_RECOVER_COPIES {
            p.recover_copies = v != "false";
        } else if c == KEY_RECOVER_MINUTES {
            if let Some(m) = v
                .parse()
                .ok()
                .filter(|m| crate::recover::MINUTES.contains(m))
            {
                p.recover_minutes = m;
            }
        } else if c == KEY_HISTORY_LIMIT {
            if let Ok(limit) = v.parse::<u32>()
                && (1..=100).contains(&limit)
            {
                p.history_limit = limit;
            }
        } else if c == KEY_HISTORY_MERGE_SECONDS {
            if let Ok(merge) = v.parse::<u32>()
                && (0..=3600).contains(&merge)
            {
                p.history_merge_seconds = merge;
            }
        } else if c == KEY_HISTORY_MAX_FILE_KB
            && let Ok(max_kb) = v.parse::<u32>()
            && (1..=1_048_576).contains(&max_kb)
        {
            p.history_max_file_kb = max_kb;
        } else if (c == KEY_RECENT_LIMIT || c == "recent-projects-limit")
            && let Ok(limit) = v.parse::<u32>()
            && RECENT_LIMIT_RANGE.contains(&limit)
        {
            p.recent_limit = limit;
        } else if c == KEY_DESIGNER_SCALE
            && let Ok(zoom) = v.parse::<f64>()
            && ((crawrad_runtime::ZOOMS[0] as f64)
                ..=(crawrad_runtime::ZOOMS[crawrad_runtime::ZOOMS.len() - 1] as f64))
                .contains(&zoom)
        {
            p.designer_scale = zoom;
        } else if c == KEY_IDE_SCALE
            && let Ok(scale) = v.parse::<f64>()
            && IDE_SCALE_RANGE.contains(&scale)
        {
            p.ide_scale = scale;
        }
    }
    p
}

#[note("workspace-preferences-to-text")]
pub fn preferences_to_text(p: Preferences) -> String {
    let dir = p
        .default_projects_dir
        .as_ref()
        .map(|d| d.to_string_lossy().to_string())
        .unwrap_or_default();
    let recent = p
        .color_recent
        .iter()
        .map(|s| s.trim())
        .filter(|s| hex_to_rgba(s).is_some())
        .take(8)
        .collect::<Vec<_>>()
        .join(",");
    let mut text = format!(
        "{KEY_WELCOME}={}\n{KEY_KEYMAP_SCHEME}={}\n{KEY_EDITOR_FONT_SIZE}={}\n{KEY_EDITOR_TAB_SIZE}={}\n{KEY_EDITOR_LINE_NUMBERS}={}\n{KEY_EDITOR_INLAY_HINTS}={}\n{KEY_EDITOR_COLOR_SCHEME}={}\n{KEY_EDITOR_COLOR_BASE}={}\neditor-color-keyword={}\neditor-color-typename={}\neditor-color-function={}\neditor-color-identifier={}\neditor-color-comment={}\neditor-color-number={}\neditor-color-string={}\neditor-color-delimiter={}\neditor-color-bg={}\n{KEY_DEFAULT_PROJECTS_DIR}={dir}\n{KEY_HISTORY_ENABLED}={}\n{KEY_HISTORY_LIMIT}={}\n{KEY_HISTORY_MERGE_SECONDS}={}\n{KEY_HISTORY_MAX_FILE_KB}={}\n{KEY_RECENT_LIMIT}={}\n{KEY_EDITOR_CURRENT_LINE}={}\n{KEY_EDITOR_BREADCRUMB_CHEVRON}={}\n{KEY_SAVE_BEFORE_BUILD}={}\n{KEY_AUTO_SAVE}={}\n{KEY_SAVE_DESKTOP}={}\n{KEY_DESKTOP_PROFILE}={}\n{KEY_RECOVER_COPIES}={}\n{KEY_RECOVER_MINUTES}={}\n{KEY_DESIGNER_GRID}={}\n{KEY_DESIGNER_SNAP}={}\n{KEY_DESIGNER_SHOW_GRID}={}\n{KEY_DESIGNER_SHOW_TAB_ORDER}={}\n{KEY_DESIGNER_SCALE}={}\n{KEY_IDE_SCALE}={}\n{KEY_COLOR_RECENT}={recent}\n",
        p.welcome_on_startup,
        p.keymap_scheme.id(),
        p.editor_font_size,
        p.editor_tab_size,
        p.editor_line_numbers,
        p.editor_inlay_hints,
        p.editor_color_scheme.key(),
        p.editor_color_base.key(),
        p.editor_custom_colors.keyword,
        p.editor_custom_colors.typename,
        p.editor_custom_colors.function,
        p.editor_custom_colors.identifier,
        p.editor_custom_colors.comment,
        p.editor_custom_colors.number,
        p.editor_custom_colors.string,
        p.editor_custom_colors.delimiter,
        p.editor_custom_colors.bg,
        p.history_enabled,
        p.history_limit,
        p.history_merge_seconds,
        p.history_max_file_kb,
        p.recent_limit,
        p.editor_current_line,
        p.editor_breadcrumb_chevron,
        p.save_before_build,
        p.auto_save.key(),
        p.save_desktop,
        p.desktop_profile,
        p.recover_copies,
        p.recover_minutes,
        p.designer_grid,
        p.designer_snap,
        p.designer_show_grid,
        p.designer_show_tab_order,
        p.designer_scale,
        p.ide_scale,
    );
    for (key, value) in EditorDiagnosticColors::KEYS
        .iter()
        .zip(p.editor_diagnostic_colors.fields())
    {
        text.push_str(&format!(
            "{key}={value}
"
        ));
    }
    // The form that was open, written only when there is one: an empty value
    // would be read back as a path and reopened as a missing file.
    if let Some(f) = p
        .last_form
        .as_ref()
        .map(|f| f.to_string_lossy().to_string())
        && !f.is_empty()
    {
        text.push_str(&format!(
            "{KEY_LAST_FORM}={f}
"
        ));
    }
    text
}

// ------------------------------------------------------------- new project

#[note("workspace-projecttype")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
    /// `cargo new --bin` + `cargo add eframe` + an initial `.crawform`. This is
    /// `File → New → Application` in the classic RAD IDE, and the idea copied from it
    /// is this: two seconds later there is a form on screen.
    Gui,
    /// `cargo new --bin`. Without a form — the command-line tool living next to the screen.
    Console,
    /// `cargo new --lib`. Without a form — shared code, which in the classic RAD IDE
    /// would be a `.bpl` in the same project group.
    Library,
}

impl ProjectType {
    #[note("workspace-projecttype-all")]
    pub const ALL: &'static [Self] = &[Self::Gui, Self::Console, Self::Library];

    #[note("workspace-projecttype-label")]
    pub fn label(self) -> &'static str {
        match self {
            Self::Gui => "GUI Application",
            Self::Console => "Console Application",
            Self::Library => "Library",
        }
    }

    #[note("workspace-projecttype-description")]
    pub fn description(self) -> &'static str {
        match self {
            Self::Gui => "A window with a form, ready to design.",
            Self::Console => "A command-line program. No form.",
            Self::Library => "A crate to share code with other projects. No form.",
        }
    }

    #[note("workspace-projecttype-flag")]
    pub fn flag(self) -> &'static str {
        match self {
            Self::Gui | Self::Console => "--bin",
            Self::Library => "--lib",
        }
    }

    #[note("workspace-projecttype-dependency-for")]
    pub fn dependency_for(self, target: crawrad_codegen::AlvoGerador) -> Option<&'static str> {
        match (self, target) {
            (Self::Gui, crawrad_codegen::AlvoGerador::Makepad) => Some("makepad-widgets@1.0"),
            (Self::Gui, crawrad_codegen::AlvoGerador::Egui) => Some("eframe@0.36"),
            (Self::Console | Self::Library, _) => None,
        }
    }

    #[note("workspace-projecttype-dependency")]
    pub fn dependency(self) -> Option<&'static str> {
        self.dependency_for(crawrad_codegen::AlvoGerador::Egui)
    }

    #[note("workspace-projecttype-with-form")]
    pub fn with_form(self) -> bool {
        matches!(self, Self::Gui)
    }
}

// ------------------------------------------------- creation steps

#[note("workspace-step")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// `cargo new`, creating the project folder with the chosen name.
    Package,
    /// `cargo add` for the type's dependency. **This is the expensive step.**
    Dependency,
    /// `.crawform` and initial `Generate`, without button.
    Code,
}

impl Step {
    #[note("workspace-step-label")]
    pub fn label(self, project_type: ProjectType) -> String {
        match self {
            Self::Package => "Creating the project folder (cargo new)".to_string(),
            // `None` does not happen: [`steps_for`] only adds this step to the plan
            // when there is a dependency. If it happens, a generic phrase is better
            // than an empty label mid-list.
            Self::Dependency => match project_type.dependency() {
                Some(d) => format!("Fetching {d} from crates.io (cargo add)"),
                None => "Fetching the dependency (cargo add)".to_string(),
            },
            Self::Code => "Generating the form code".to_string(),
        }
    }
}

#[note("workspace-steps-for")]
pub fn steps_for(project_type: ProjectType, with_form: bool) -> Vec<Step> {
    let mut plan = Vec::with_capacity(3);
    plan.push(Step::Package);
    if project_type.dependency().is_some() {
        plan.push(Step::Dependency);
    }
    if with_form {
        plan.push(Step::Code);
    }
    plan
}

// ------------------------------------------------- runnable targets

#[note("workspace-metadata")]
#[derive(serde::Deserialize)]
pub(crate) struct Metadata {
    pub(crate) packages: Vec<CargoPackage>,
    #[serde(default)]
    pub(crate) target_directory: PathBuf,
}

#[derive(serde::Deserialize)]
pub(crate) struct CargoPackage {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) targets: Vec<CargoTarget>,
    #[serde(default)]
    pub(crate) manifest_path: PathBuf,
    /// `[package.metadata]`; `crawrad.platform = "web"` marks a browser app.
    #[serde(default)]
    pub(crate) metadata: serde_json::Value,
}

#[derive(serde::Deserialize)]
pub(crate) struct CargoTarget {
    /// `["bin"]`, `["lib"]`, `["cdylib"]`, `["test"]`, `["custom-build"]`…
    ///
    /// **List**, not text: a target can have more than one type — such as
    /// `crate-type = ["rlib", "cdylib"]`, which ADR 0018 already names. `kind`
    /// and `crate_types` travel together and say almost the same; only `kind`
    /// distinguishes `bin` from `lib`, which is the question here.
    kind: Vec<String>,
}

#[note("workspace-executables")]
pub fn executables(json: &str) -> Result<std::collections::BTreeMap<String, bool>, String> {
    let m: Metadata = serde_json::from_str(json).map_err(|e| format!("cargo metadata: {e}"))?;
    Ok(m.packages
        .into_iter()
        .map(|p| {
            let runs = p.targets.iter().any(|t| t.kind.iter().any(|k| k == "bin"));
            (p.name, runs)
        })
        .collect())
}

#[note("workspace-group-executables")]
pub fn group_executables(root: &Path) -> Result<std::collections::BTreeMap<String, bool>, String> {
    // Arguments come from [`crawrad_os::process::CargoCall::metadata`], not a list written here:
    // the same query is made by the package manager, and two hand-written lists are
    // two lists that diverge. The environment variables `Run` strips are removed there
    // for the same reason — user projects live outside ours, and `CARGO_TARGET_DIR`
    // of the cargo that launched us would redirect output into our `target/`.
    let output =
        crawrad_os::process::wait(&mut crawrad_os::process::CargoCall::metadata(root).command())?;
    if !output.success {
        // Cargo's phrase, not ours: broken manifests are normal here, and cargo
        // already knows which line. And when it fails without saying `error`,
        // `report` returns the last line with content instead of swallowing everything.
        return Err(output
            .report()
            .unwrap_or_else(|| "cargo metadata failed".to_string()));
    }
    executables(&output.stdout)
}

#[note("workspace-targets-timestamp")]
pub fn targets_timestamp(root: &Path, member: Option<&Path>) -> Option<std::time::SystemTime> {
    let when = |p: PathBuf| std::fs::metadata(p).ok().and_then(|m| m.modified().ok());
    [
        Some(root.join(MANIFEST)),
        member.map(|m| m.join(MANIFEST)),
        member.map(|m| m.join("src")),
    ]
    .into_iter()
    .flatten()
    .filter_map(when)
    .max()
}

#[note("workspace-new-args")]
pub fn new_args(name: &str, project_type: ProjectType) -> Vec<String> {
    vec![
        "new".to_string(),
        name.to_string(),
        "--name".to_string(),
        name.to_string(),
        project_type.flag().to_string(),
    ]
}

#[note("workspace-refusal-typed-name")]
pub fn refusal_typed_name(name: &str) -> Option<String> {
    if name.is_empty() {
        // Without an error phrase: empty field is not an error, it is the initial state.
        // What the dialog does with this is disable the button, not accuse someone who
        // has not typed yet.
        return Some("type a name for the project".to_string());
    }
    if name.starts_with(|c: char| c.is_ascii_digit()) {
        return Some(format!(
            "`{name}` cannot start with a digit — a crate name is a Rust identifier. \
             Try `{}`",
            suggested_name(name)
        ));
    }
    // **`-` at the start has its own phrase, not falling into general form.**
    //
    // The previous comment here said it fell through, and review measured that it did not:
    // `-app` is ASCII letters and hyphen, so it passes the form rule below entirely.
    // What happens afterwards is worse than a refusal: the name travels as a POSITIONAL
    // argument to `cargo new`, clap reads `-` as a flag start, and the user pays the
    // folder dialog to receive `error: unexpected argument '-a' found` — a message
    // about a command-line flag they never wrote.
    //
    // It is form, not cargo's word list: the rule "first character cannot be `-`"
    // stems from how arguments are passed, and is ours.
    if name.starts_with('-') {
        // Suggestion comes from [`suggested_name`] like other phrases, not a local
        // `trim_start_matches`: it already knows to return `app` when nothing is
        // left (`-`, `--`), and a second sanitizing rule here would be the same
        // rule written in two places.
        return Some(format!(
            "`{name}` cannot start with `-` — cargo would read it as a flag. \
             Try `{}`",
            suggested_name(name)
        ));
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        let cleaned = suggested_name(name);
        // **The suggestion is the useful half of the phrase.** `My App` is a
        // perfectly normal name for a person, and was the case `suggested_name`
        // resolved automatically when the name came from a folder. Saying only
        // "that is invalid", the IDE hands back a problem; saying `my_app`, it
        // hands back the answer.
        if cleaned == name {
            return Some(format!(
                "`{name}` is not a crate name — use letters, digits, `_` and `-`"
            ));
        }
        return Some(format!(
            "`{name}` is not a crate name — use letters, digits, `_` and `-`. \
             Try `{cleaned}`"
        ));
    }
    None
}

/// The `Cargo.toml` of a group — the **virtual** root of a workspace.
///
/// **Creation no longer writes this** ([ADR 0017], Amendment 3): the owner decided that
/// `New Project` produces a single-level package, like `cargo new`, and that the
/// group is born the day there is a second project. Along with it went
/// `root_files`, which was the list of what creation wrote at the root.
///
/// The text remains, and is **`#[cfg(test)]`**, because the other half of the amendment
/// continues to hold completely: **opening** must honor groups. Every project created by the IDE
/// up to 08/09/2026 is group+member, and the project the owner currently has open as well
/// — so the test suites that measure opening need a real group to open,
/// and this is the manifest they build. A literal written inside each test would be
/// the same shape copied in three places.
///
/// `resolver = "2"` explicitly written: in a virtual root cargo assumes resolver 1 and
/// warns on every compilation. A warning that appears constantly is a warning people
/// learn to ignore, and then the next warning slips past unnoticed.
///
#[cfg(test)]
pub fn group_cargo_toml(member: &str) -> String {
    format!(
        "# THE GROUP. This folder is a Cargo workspace, and each project enters as\n\
         # a member — it is the classic RAD IDE's `.groupproj`, holding the executable project\n\
         # and each library project.\n\
         #\n\
         # `default-members` is the ACTIVE PROJECT: `cargo run` in this folder, without\n\
         # `-p`, runs it. From now on what maintains `members` is cargo itself —\n\
         # `cargo new` in here adds the member on its own.\n\
         [workspace]\n\
         resolver = \"2\"\n\
         members = [\"{member}\"]\n\
         default-members = [\"{member}\"]\n"
    )
}

#[note("workspace-refusal-new")]
pub fn refusal_new(name: &str, cargo_error: &str) -> Refusal {
    // The needle is cargo's: `error: destination `<path>` already exists`.
    // Measured with real `cargo new`, and is the first line starting with
    // `error` that `create_project` extracts.
    //
    // Comparison is by CONTENT and not equality: cargo interpolates the path
    // in the middle, and its phrasing varies across toolchains. Searching for
    // both words is the minimum that does not rot — and if one day it fails to match,
    // the fallback is the generic phrase, which is correct though less helpful.
    if cargo_error.contains("destination") && cargo_error.contains("already exists") {
        return Refusal {
            phrase: format!(
                "a folder named `{name}` already exists here, with a project in it — \
                 open it instead of creating one, or use another name. \
                 Nothing was written. ({cargo_error})"
            ),
            open_instead: true,
        };
    }
    Refusal {
        phrase: format!(
            "{cargo_error} — `{name}` is the name you typed; pick another one and try \
             again. Nothing was written."
        ),
        open_instead: false,
    }
}

#[note("workspace-cargo-version")]
pub fn cargo_version(output: &str) -> Option<(u32, u32)> {
    let line = output
        .lines()
        .map(str::trim)
        .find(|l| l.starts_with("cargo "))?;
    let version_word = line.split_whitespace().nth(1)?;
    let mut parts = version_word.split('.');
    let major = parts.next()?.parse::<u32>().ok()?;
    let minor = parts.next()?.parse::<u32>().ok()?;
    Some((major, minor))
}

#[note("workspace-refusal-old-toolchain")]
pub fn refusal_old_toolchain(found: (u32, u32), needed: (u32, u32)) -> Option<Refusal> {
    if found >= needed {
        return None;
    }
    Some(Refusal::simple(format!(
        "this project needs Rust {}.{} or newer, and this machine has {}.{} — run `rustup update stable` and try again. Nothing was written.",
        needed.0, needed.1, found.0, found.1,
    )))
}

#[note("workspace-refusal")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    /// The phrase, in English, for the dialog and for the status bar.
    pub phrase: String,
    /// A project already exists in the chosen folder, and **opening** is the way out.
    pub open_instead: bool,
}

impl Refusal {
    #[note("workspace-refusal-simple")]
    pub fn simple(phrase: String) -> Self {
        Self {
            phrase,
            open_instead: false,
        }
    }
}

impl From<String> for Refusal {
    fn from(phrase: String) -> Self {
        Self::simple(phrase)
    }
}

#[note("workspace-refusal-occupied-folder")]
pub fn refusal_occupied_folder(parent: &Path, name: &str) -> Option<Refusal> {
    let dest = parent.join(name);
    if !dest.exists() {
        return None;
    }
    let has_project = dest.join(MANIFEST).is_file();
    Some(Refusal {
        phrase: if has_project {
            format!(
                "a folder named `{name}` already exists here, with a project in it — \
                 open it instead of creating one, or use another name. Nothing was written."
            )
        } else {
            format!(
                "a folder named `{name}` already exists here — use another name, or \
                 another folder. Nothing was written."
            )
        },
        open_instead: has_project,
    })
}

// Where the New Project dialog should point when it opens.
//
// It must point somewhere a project can actually be written. The old default was
// the process working directory, which is fine when the IDE was installed by an
// ordinary installer and WRONG when it came from the Microsoft Store: there the
// working directory IS `...\WindowsApps\<package>`, the package's own install
// folder, which Windows keeps read-only with locked ACLs. The dialog opened
// pointing at the one place a project can never live.
//
// The order is the useful one: the folder chosen in Options first, since a person
// who set it said where their projects go; then beside the project last opened, since a person
// keeps their projects together; then a `CrawRAD-Projects` folder under the home
// folder; then the working directory, but only when it is not inside the install
// folder. Every input is a parameter so the choice can be tested without touching
// the real environment.
#[note("workspace-default-project-parent")]
pub fn default_project_parent(
    configured: Option<&Path>,
    recent: Option<&Path>,
    home: Option<&Path>,
    cwd: Option<&Path>,
    app_dir: Option<&Path>,
) -> Option<PathBuf> {
    // What the person chose in Options wins: they said where their projects go,
    // and nothing here knows better. It was being written and shown there, and
    // never read by this dialog.
    if let Some(c) = configured
        && !is_inside(c, app_dir)
    {
        return Some(c.to_path_buf());
    }
    if let Some(r) = recent
        && let Some(parent) = r.parent()
        && parent.is_dir()
        && !is_inside(parent, app_dir)
    {
        return Some(parent.to_path_buf());
    }
    if let Some(h) = home {
        return Some(h.join("CrawRAD-Projects"));
    }
    if let Some(c) = cwd
        && c.is_dir()
        && !is_inside(c, app_dir)
    {
        return Some(c.to_path_buf());
    }
    None
}

// True when `path` is the install folder or sits under it.
#[note("workspace-is-inside")]
fn is_inside(path: &Path, outer: Option<&Path>) -> bool {
    match outer {
        Some(o) => path.starts_with(o),
        None => false,
    }
}

// Makes the folder a project is about to be written into, or says why it cannot.
//
// `Create` is a promise to create, so a folder that does not exist yet is created
// here rather than answered with a file picker. The picker used to be the silent
// fallback for a path that was not a directory, which is how editing the drive
// letter of the default location — making the path point nowhere — turned the
// Create button into a folder chooser, with nothing said about why.
//
// `app_dir` is CrawRAD's own install folder, when it is known: a path inside it
// gets its own phrase, because no amount of retrying will make that folder
// writable and the person deserves to be told that instead of an OS error code.
#[note("workspace-make-project-parent")]
pub fn make_project_parent(folder: &Path, app_dir: Option<&Path>) -> Result<(), Refusal> {
    if folder.is_dir() {
        return Ok(());
    }
    if folder.exists() {
        return Err(Refusal::simple(format!(
            "`{}` is a file, not a folder. Nothing was written.",
            folder.display()
        )));
    }
    if is_inside(folder, app_dir) {
        return Err(Refusal::simple(format!(
            "`{}` is inside CrawRAD's own installation folder, which Windows keeps              read-only — a project cannot be created there. Choose a folder under your              user folder instead. Nothing was written.",
            folder.display()
        )));
    }
    match std::fs::create_dir_all(folder) {
        Ok(()) => Ok(()),
        Err(e) => Err(Refusal::simple(format!(
            "`{}` could not be created: {e}. Nothing was written.",
            folder.display()
        ))),
    }
}

#[note("workspace-without-color")]
pub fn without_color(raw: &str) -> String {
    let mut clean = String::with_capacity(raw.len());
    let mut chars = raw.chars();
    while let Some(c) = chars.next() {
        if c != '\u{1b}' {
            clean.push(c);
            continue;
        }
        match chars.next() {
            // `ESC [ parameters letter` — what cargo emits. Consumes up to the final byte,
            // which is anything from `@` to `~`.
            Some('[') => {
                for f in chars.by_ref() {
                    if ('\u{40}'..='\u{7e}').contains(&f) {
                        break;
                    }
                }
            }
            // `ESC ] … BEL` (OSC, used for terminal links): goes up to `\u{7}`
            // or `ESC \`. Cargo does not use it today; if one day it does, it disappears
            // here instead of turning into garbage in the middle of the phrase.
            Some(']') => {
                let mut prev = '\0';
                for f in chars.by_ref() {
                    if f == '\u{7}' || (prev == '\u{1b}' && f == '\\') {
                        break;
                    }
                    prev = f;
                }
            }
            // Two-character sequence. The `ESC` and what followed leave.
            Some(_) | None => {}
        }
    }
    clean
}

#[note("workspace-first-error")]
pub fn first_error(cargo_output: &str) -> String {
    without_color(cargo_output)
        .lines()
        .find(|l| l.starts_with("error"))
        .unwrap_or("cargo failed")
        .to_string()
}

#[note("workspace-adopting-workspace")]
pub fn adopting_workspace(cargo_output: &str) -> Option<String> {
    for line in without_color(cargo_output).lines() {
        let l = line.trim();
        if !l.starts_with("Adding ") || !l.contains("as member of workspace at") {
            continue;
        }
        // The path arrives between backticks at line end. Grabbing the LAST pair
        // separates it from the package name, which also arrives between backticks.
        let mut parts = l.rsplit('`');
        // `rsplit` on `…at `path`` gives: "", path, "…at ", …
        let _after = parts.next();
        if let Some(path) = parts.next()
            && !path.is_empty()
        {
            return Some(path.to_string());
        }
    }
    None
}

#[note("workspace-suggested-name")]
pub fn suggested_name(raw: &str) -> String {
    let mut name = String::with_capacity(raw.len());
    for c in raw.chars() {
        if c.is_ascii_alphanumeric() {
            name.push(c.to_ascii_lowercase());
        } else if c.is_alphanumeric() {
            // Non-ASCII letter — accents, and the rest of the world. It DISAPPEARS
            // instead of becoming `_`: `Ação` would become `a__o`, and a crate name
            // full of underscores inside a word is worse than the word without accents.
        } else if !name.ends_with('_') {
            // Space, hyphen, dot: text separator becomes crate name separator.
            // `My App` is perfectly normal, and without this line it would become `myapp`.
            name.push('_');
        }
    }
    let name = name.trim_matches('_').to_string();
    if name.is_empty() {
        // Text where nothing remained (`...`, `日本`). A fixed name is better than
        // an empty suggestion that suggests nothing.
        return "app".into();
    }
    if name.starts_with(|c: char| c.is_ascii_digit()) {
        return format!("_{name}");
    }
    name
}

#[note("workspace-package-name-from-manifest")]
pub fn package_name_from_manifest(cargo_toml: &str) -> Option<String> {
    let mut in_package = false;
    for line in cargo_toml.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            in_package = line.starts_with("[package]");
            continue;
        }
        if !in_package {
            continue;
        }
        let Some(val) = line.strip_prefix("name") else {
            continue;
        };
        let val = val.trim_start();
        let Some(val) = val.strip_prefix('=') else {
            continue;
        };
        let val = val.trim().trim_matches('"').trim_matches('\'');
        if !val.is_empty() {
            return Some(val.to_string());
        }
    }
    None
}

#[cfg(test)]
#[path = "test/workspace/test.rs"]
mod tests;

#[note("workspace-has-binary")]
pub fn has_binary(root: &std::path::Path, package: &str) -> bool {
    group_executables(root)
        .ok()
        .and_then(|packages| packages.get(package).copied())
        .unwrap_or(true)
}
