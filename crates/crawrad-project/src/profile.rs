//! Desktop profiles: named looks of the IDE — the window, each dock's
//! size and visibility, the Inspector split, the editor font and the
//! interface zoom — kept in `<config>/profiles/<name>.txt` and applied from
//! Options or by the `layout.*_profile` actions.
//!
//! [`DEFAULT`] is not a file: it is the IDE as it ships. A line a profile
//! lacks, or cannot read, falls back to that default too, so an old or
//! hand-edited profile still applies as well as it can.

use std::path::{Path, PathBuf};

use crawrad_actions::ProfileCommand::{self, Apply, Delete, Save};

use crate::desktop::WindowPlace;
use crawrad_note::note;

#[note("profile-header")]
pub const HEADER: &str = "crawrad-profile 1";

#[note("profile-default")]
pub const DEFAULT: &str = "Default";

#[note("profile-name-max")]
pub const NAME_MAX: usize = 40;

#[note("profile-profile")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Profile {
    pub window: Option<WindowPlace>,
    /// The width of the Inspector's name column.
    pub name_width: Option<f64>,
    pub editor_font: Option<u32>,
    /// The interface zoom, 1.0 being 100%.
    pub zoom: Option<f64>,
    /// The Form Designer's grid size.
    pub grid: Option<u32>,
    /// The dock arrangement, in the dock's own text format, last in the file.
    pub dock: String,
}

#[note("profile-folder")]
pub fn folder() -> Option<PathBuf> {
    Some(crate::workspace::config_folder()?.join("profiles"))
}

#[note("profile-check-name")]
pub fn check_name(name: &str) -> Result<&str, String> {
    let name = name.trim();
    let allowed = |c: char| c.is_alphanumeric() || matches!(c, ' ' | '-' | '_' | '.');
    if name.is_empty() {
        Err("a profile needs a name".into())
    } else if name.eq_ignore_ascii_case(DEFAULT) {
        Err(format!("`{DEFAULT}` is the built-in profile"))
    } else if name.chars().count() > NAME_MAX || !name.chars().all(allowed) {
        Err(format!(
            "`{name}`: up to {NAME_MAX} letters, digits, spaces, `-`, `_` or `.`"
        ))
    } else if name.starts_with('.') {
        Err(format!("`{name}` cannot start with `.`"))
    } else {
        Ok(name)
    }
}

#[note("profile-names")]
pub fn names(dir: &Path) -> Vec<String> {
    let mut saved: Vec<String> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            let stem = p.file_stem()?.to_str()?;
            let ok = p.extension()? == "txt" && check_name(stem).is_ok();
            ok.then(|| stem.to_string())
        })
        .collect();
    saved.sort_by_key(|n| n.to_lowercase());
    std::iter::once(DEFAULT.to_string()).chain(saved).collect()
}

#[note("profile-settle")]
pub fn settle(command: &ProfileCommand, done: Result<(), String>) -> String {
    let (Apply(name) | Save(name) | Delete(name)) = command;
    let name = name.trim();
    if let Err(e) = done {
        return e;
    }
    crate::workspace::update_preferences(|p| match command {
        Apply(_) | Save(_) => p.desktop_profile = name.into(),
        Delete(_) if p.desktop_profile == name => p.desktop_profile = DEFAULT.into(),
        Delete(_) => {}
    });
    let verb = match command {
        Apply(_) => "applied",
        Save(_) => "saved",
        Delete(_) => "deleted",
    };
    format!("profile `{name}` {verb}")
}

#[note("profile-all")]
pub fn all() -> Vec<String> {
    names(&folder().unwrap_or_default())
}

impl Profile {
    fn file(dir: &Path, name: &str) -> Result<PathBuf, String> {
        Ok(dir.join(format!("{}.txt", check_name(name)?)))
    }

    #[note("profile-profile-encode")]
    pub fn encode(&self) -> String {
        let mut s = format!("{HEADER}\n");
        if let Some(w) = self.window {
            let max = u8::from(w.maximized);
            s += &format!("window {} {} {} {} {max}\n", w.x, w.y, w.width, w.height);
        }
        let numbers = [
            ("name-width", self.name_width),
            ("editor-font", self.editor_font.map(f64::from)),
            ("zoom", self.zoom),
            ("grid", self.grid.map(f64::from)),
        ];
        for (key, value) in numbers {
            if let Some(v) = value {
                s += &format!("{key} {v}\n");
            }
        }
        s + "dock\n" + &self.dock
    }

    #[note("profile-profile-decode")]
    pub fn decode(text: &str) -> Option<Self> {
        let mut lines = text.lines();
        if lines.next()?.trim() != HEADER {
            return None;
        }
        let mut p = Self::default();
        let positive = |v: &str| v.trim().parse::<f64>().ok().filter(|n| *n > 0.0);
        while let Some(line) = lines.next() {
            let (word, rest) = line.split_once(' ').unwrap_or((line, ""));
            match word {
                "window" => p.window = crate::desktop::window_of(rest),
                "name-width" => p.name_width = positive(rest),
                "editor-font" => p.editor_font = rest.trim().parse().ok().filter(|n| *n > 0),
                "zoom" => p.zoom = positive(rest),
                "grid" => {
                    let grid = rest.trim().parse().ok();
                    p.grid = grid.filter(|g| crate::workspace::DESIGNER_GRID_RANGE.contains(g));
                }
                "dock" => {
                    p.dock = lines.collect::<Vec<_>>().join("\n");
                    break;
                }
                _ => {}
            }
        }
        Some(p)
    }

    #[note("profile-profile-read")]
    pub fn read(dir: &Path, name: &str) -> Result<Self, String> {
        if name.trim().eq_ignore_ascii_case(DEFAULT) {
            return Ok(Self::default());
        }
        let text = std::fs::read_to_string(Self::file(dir, name)?)
            .map_err(|_| format!("there is no profile `{}`", name.trim()))?;
        Self::decode(&text).ok_or_else(|| format!("`{}` is not a profile", name.trim()))
    }

    #[note("profile-profile-write")]
    pub fn write(&self, dir: &Path, name: &str) -> Result<(), String> {
        let file = Self::file(dir, name)?;
        std::fs::create_dir_all(dir)
            .and_then(|_| std::fs::write(file, self.encode()))
            .map_err(|e| format!("could not save `{}`: {e}", name.trim()))
    }

    #[note("profile-profile-delete")]
    pub fn delete(dir: &Path, name: &str) -> Result<(), String> {
        std::fs::remove_file(Self::file(dir, name)?)
            .map_err(|_| format!("there is no profile `{}`", name.trim()))
    }
}

#[cfg(test)]
#[path = "test/profile/test.rs"]
mod tests;
