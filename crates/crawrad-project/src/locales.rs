//! Project locales persistence in `locales/<code>.toml`.

use std::path::Path;

/// Directory name under project directory where locale files are stored.
pub const LOCALES_DIR: &str = "locales";

/// Returns the sorted language codes of existing `locales/<code>.toml` files.
pub fn list_locales(project_dir: &Path) -> Vec<String> {
    let dir = project_dir.join(LOCALES_DIR);
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut codes = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file()
            && path.extension().is_some_and(|ext| ext == "toml")
            && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
        {
            codes.push(stem.to_string());
        }
    }
    codes.sort();
    codes
}

/// Reads a locale file `locales/<code>.toml` into `(key, text, from)` tuples.
///
/// Missing file returns an empty list. Malformed entries are skipped.
pub fn read_locale(project_dir: &Path, code: &str) -> Vec<(String, String, String)> {
    let path = project_dir.join(LOCALES_DIR).join(format!("{code}.toml"));
    let Ok(content) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    let Ok(table) = toml::from_str::<toml::Table>(&content) else {
        return Vec::new();
    };
    let mut entries = Vec::new();
    for (key, val) in &table {
        let Some(item) = val.as_table() else {
            continue;
        };
        let Some(text) = item.get("text").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some(from) = item.get("from").and_then(|v| v.as_str()) else {
            continue;
        };
        entries.push((key.clone(), text.to_string(), from.to_string()));
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries
}

/// A TOML basic string escaping backslashes and double quotes.
fn quoted(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Writes locale entries to `locales/<code>.toml`, creating the directory if missing.
///
/// Entries are written sorted by key, one entry per line.
pub fn write_locale(
    project_dir: &Path,
    code: &str,
    entries: &[(String, String, String)],
) -> Result<(), String> {
    let dir = project_dir.join(LOCALES_DIR);
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("cannot create directory {}: {e}", dir.display()))?;
    let path = dir.join(format!("{code}.toml"));

    let mut sorted_entries = entries.to_vec();
    sorted_entries.sort_by(|a, b| a.0.cmp(&b.0));
    sorted_entries.dedup_by(|a, b| a.0 == b.0);

    let mut content = String::new();
    for (key, text, from) in &sorted_entries {
        content.push_str(&format!(
            "{} = {{ text = {}, from = {} }}\n",
            quoted(key),
            quoted(text),
            quoted(from),
        ));
    }
    std::fs::write(&path, content)
        .map_err(|e| format!("cannot write file {}: {e}", path.display()))?;
    Ok(())
}

/// Removes `locales/<code>.toml` if it exists.
pub fn remove_locale(project_dir: &Path, code: &str) -> Result<(), String> {
    let path = project_dir.join(LOCALES_DIR).join(format!("{code}.toml"));
    if path.exists() {
        std::fs::remove_file(&path)
            .map_err(|e| format!("cannot remove file {}: {e}", path.display()))?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/locales/test.rs"]
mod tests;

/// A locale file: its code and its `(key, text, from)` entries.
pub type LocaleEntries = (String, Vec<(String, String, String)>);

/// Every locale of the project, in [`list_locales`] order, with its entries.
pub fn read_all(project_dir: &Path) -> Vec<LocaleEntries> {
    list_locales(project_dir)
        .into_iter()
        .map(|code| {
            let entries = read_locale(project_dir, &code);
            (code, entries)
        })
        .collect()
}

/// Sets one translation (`from` = the English it translates) and rewrites the file.
pub fn set_entry(
    project_dir: &Path,
    code: &str,
    key: &str,
    text: &str,
    from: &str,
) -> Result<(), String> {
    let mut entries = read_locale(project_dir, code);
    entries.retain(|(k, _, _)| k != key);
    entries.push((key.to_string(), text.to_string(), from.to_string()));
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    write_locale(project_dir, code, &entries)
}
