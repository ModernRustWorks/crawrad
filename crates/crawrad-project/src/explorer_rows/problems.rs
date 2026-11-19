//! **The problems behind a row**: the errors and warnings the language
//! server published for a file — or, for a member, for every file under its
//! folder. The worst one gives the row its tone (an error red, a warning
//! amber) and the count goes before the badge, as the reference does
//! (`main.rs  2, M`).

use std::collections::BTreeMap;
use std::path::Path;

use super::icon::{ERROR, RowIcon, UNSAVED, WARNING};
use crawrad_note::note;

#[note("explorer-rows-problems-problems")]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Problems {
    pub errors: usize,
    pub warnings: usize,
}

#[note("explorer-rows-problems-problemmap")]
pub type ProblemMap = BTreeMap<String, Problems>;

#[note("explorer-rows-problems-key")]
pub fn key(path: &Path) -> String {
    let key = path.to_string_lossy().replace('\\', "/");
    if cfg!(windows) {
        key.to_lowercase()
    } else {
        key
    }
}

#[note("explorer-rows-problems-map")]
pub fn map(counts: Vec<(String, usize, usize)>) -> ProblemMap {
    let one = |(file, errors, warnings)| (file, Problems { errors, warnings });
    counts.into_iter().map(one).collect()
}

impl Problems {
    #[note("explorer-rows-problems-problems-of")]
    pub fn of(map: &ProblemMap, file: &Path) -> Self {
        map.get(&key(file)).copied().unwrap_or_default()
    }

    #[note("explorer-rows-problems-problems-under")]
    pub fn under(map: &ProblemMap, folder: &Path) -> Self {
        let prefix = format!("{}/", key(folder).trim_end_matches('/'));
        let inside = map.iter().filter(|(k, _)| k.starts_with(&prefix));
        inside.fold(Self::default(), |a, (_, p)| Self {
            errors: a.errors + p.errors,
            warnings: a.warnings + p.warnings,
        })
    }

    #[note("explorer-rows-problems-problems-tone")]
    pub fn tone(&self) -> Option<&'static str> {
        match (self.errors, self.warnings) {
            (0, 0) => None,
            (0, _) => Some("Warn"),
            _ => Some("Error"),
        }
    }

    #[note("explorer-rows-problems-problems-badge")]
    pub fn badge(&self, word: &str) -> String {
        match (self.errors + self.warnings, word.is_empty()) {
            (0, _) => word.to_string(),
            (n, true) => n.to_string(),
            (n, false) => format!("{n} \u{b7} {word}"),
        }
    }
}

#[note("explorer-rows-problems-state-icon")]
pub fn state_icon(problems: Problems, unsaved: bool) -> Option<RowIcon> {
    if problems.errors > 0 {
        Some(ERROR)
    } else if problems.warnings > 0 {
        Some(WARNING)
    } else if unsaved {
        Some(UNSAVED)
    } else {
        None
    }
}

#[cfg(test)]
#[path = "../test/explorer_problems/test.rs"]
mod tests;
