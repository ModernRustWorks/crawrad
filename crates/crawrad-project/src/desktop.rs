//! The project desktop: how the IDE looked when a project was left —
//! the window, the docks, the open tabs and the caret of each — kept in
//! `<project>/.crawrad/desktop.txt` and given back when the project opens again.
//!
//! The file is plain lines, so a hand edit or a merge conflict reads back as
//! well as it can: an unknown line is skipped, a tab whose file is gone is
//! dropped, and only a missing or foreign header throws the file away.

use crawrad_note::note;
use std::path::{Path, PathBuf};

#[note("desktop-header")]
pub const HEADER: &str = "crawrad-desktop 1";

pub const DOCK_LAYOUT_FILE: &str = "makepad-dock-layout.ron";
pub const DISCARDED: &str =
    "the saved panel layout could not be read whole and was discarded — showing the default";
pub const DOCK_WIDTH: f64 = 300.0;
pub const CHAT_WIDTH: f64 = 390.0;

pub fn dock_layout_file_path() -> Option<PathBuf> {
    Some(crate::workspace::config_folder()?.join(DOCK_LAYOUT_FILE))
}

#[note("desktop-windowplace")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowPlace {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub maximized: bool,
}

impl WindowPlace {
    #[note("desktop-windowplace-new")]
    pub fn new([x, y, width, height]: [f64; 4], maximized: bool) -> Self {
        Self {
            x,
            y,
            width,
            height,
            maximized,
        }
    }
}

#[note("desktop-tabplace")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabPlace {
    pub file: PathBuf,
    pub line: usize,
    pub byte: usize,
}

#[note("desktop-desktop")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Desktop {
    pub window: Option<WindowPlace>,
    pub tabs: Vec<TabPlace>,
    /// The tab in front, an index into `tabs`; `None` is the designer.
    pub active: Option<usize>,
    /// The dock arrangement, in the dock's own text format, last in the file.
    pub dock: String,
}

impl Desktop {
    #[note("desktop-desktop-new")]
    pub fn new(window: WindowPlace, dock: String) -> Self {
        let window = Some(window);
        Self {
            window,
            dock,
            ..Self::default()
        }
    }

    #[note("desktop-desktop-file-in")]
    pub fn file_in(root: &Path) -> PathBuf {
        root.join(".crawrad").join("desktop.txt")
    }

    #[note("desktop-desktop-with-tabs")]
    pub fn with_tabs(
        mut self,
        root: &Path,
        files: &[PathBuf],
        active: Option<usize>,
        caret: impl Fn(&Path) -> Option<(usize, usize)>,
    ) -> Self {
        self.tabs.clear();
        self.active = None;
        for (i, f) in files.iter().enumerate() {
            let Ok(rel) = f.strip_prefix(root) else {
                continue;
            };
            if active == Some(i) {
                self.active = Some(self.tabs.len());
            }
            let (line, byte) = caret(f).unwrap_or_default();
            let file = rel.to_path_buf();
            self.tabs.push(TabPlace { file, line, byte });
        }
        self
    }

    #[note("desktop-desktop-encode")]
    pub fn encode(&self) -> String {
        let mut s = format!("{HEADER}\n");
        if let Some(w) = self.window {
            let max = u8::from(w.maximized);
            s += &format!("window {} {} {} {} {max}\n", w.x, w.y, w.width, w.height);
        }
        for t in &self.tabs {
            let file = t.file.to_string_lossy().replace('\\', "/");
            s += &format!("tab {} {} {file}\n", t.line, t.byte);
        }
        if let Some(a) = self.active {
            s += &format!("active {a}\n");
        }
        s + "dock\n" + &self.dock
    }

    #[note("desktop-desktop-decode")]
    pub fn decode(text: &str) -> Option<Self> {
        let mut lines = text.lines();
        if lines.next()?.trim() != HEADER {
            return None;
        }
        let mut d = Self::default();
        while let Some(line) = lines.next() {
            let (word, rest) = line.split_once(' ').unwrap_or((line, ""));
            match word {
                "window" => d.window = window_of(rest),
                "tab" => d.tabs.extend(tab_of(rest)),
                "active" => d.active = rest.trim().parse().ok(),
                "dock" => {
                    d.dock = lines.collect::<Vec<_>>().join("\n");
                    break;
                }
                _ => {}
            }
        }
        d.active = d.active.filter(|a| *a < d.tabs.len());
        Some(d)
    }

    #[note("desktop-desktop-read")]
    pub fn read(root: &Path) -> Option<Self> {
        let mut d = Self::decode(&std::fs::read_to_string(Self::file_in(root)).ok()?)?;
        let active = d.active.map(|a| d.tabs[a].clone());
        d.tabs.retain(|t| root.join(&t.file).is_file());
        d.active = active.and_then(|t| d.tabs.iter().position(|x| *x == t));
        Some(d)
    }

    #[note("desktop-desktop-write")]
    pub fn write(&self, root: &Path) -> std::io::Result<()> {
        let file = Self::file_in(root);
        if let Some(dir) = file.parent() {
            std::fs::create_dir_all(dir)?;
        }
        std::fs::write(file, self.encode())
    }
}

pub(crate) fn window_of(rest: &str) -> Option<WindowPlace> {
    let n: Vec<f64> = rest
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .ok()?;
    match n[..] {
        [x, y, width, height, max] if width > 0.0 && height > 0.0 => Some(WindowPlace {
            x,
            y,
            width,
            height,
            maximized: max != 0.0,
        }),
        _ => None,
    }
}

fn tab_of(rest: &str) -> Option<TabPlace> {
    let mut parts = rest.splitn(3, ' ');
    let line = parts.next()?.parse().ok()?;
    let byte = parts.next()?.parse().ok()?;
    let file: PathBuf = parts.next()?.trim().split('/').collect();
    (!file.as_os_str().is_empty()).then_some(TabPlace { file, line, byte })
}

#[cfg(test)]
#[path = "test/desktop/test.rs"]
mod tests;
