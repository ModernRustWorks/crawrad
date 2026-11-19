//! The Release report: the executable size of every Release build,
//! what weighs most in it (read from the linker map on Windows, from the symbol
//! table on Linux/macOS — no extra tool installed), and the `[profile.release]`
//! keys that shrink it.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use toml_edit::{DocumentMut, Item, Value};

/// A single recorded Release build with timestamp, executable byte count,
/// and the byte counts of its heaviest crates.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
pub struct Release {
    #[serde(default)]
    pub package: String,
    #[serde(default)]
    pub at: u64,
    #[serde(default)]
    pub bytes: u64,
    #[serde(default)]
    pub crates: BTreeMap<String, u64>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct HistoryDocument {
    #[serde(default)]
    release: Vec<Release>,
}

/// The path to `.crawrad/release-sizes.toml` inside the given project root.
pub fn history_file(root: &Path) -> PathBuf {
    root.join(".crawrad").join("release-sizes.toml")
}

/// Parses the contents of a release history file; a malformed file returns an
/// empty list.
pub fn parse_history(text: &str) -> Vec<Release> {
    toml::from_str::<HistoryDocument>(text)
        .map(|doc| doc.release)
        .unwrap_or_default()
}

/// Formats the releases as TOML text that round-trips with [`parse_history`].
pub fn history_text(releases: &[Release]) -> String {
    let doc = HistoryDocument {
        release: releases.to_vec(),
    };
    toml::to_string(&doc).unwrap_or_default()
}

/// Reads the history of releases from disk; missing or unreadable file yields
/// an empty list.
pub fn read_history(root: &Path) -> Vec<Release> {
    let path = history_file(root);
    match std::fs::read_to_string(path) {
        Ok(text) => parse_history(&text),
        Err(_) => Vec::new(),
    }
}

/// Appends a release record to history on disk, preserving only the last 20
/// entries for that package, keeping only the 40 heaviest crates for that
/// record, and creating the `.crawrad` directory if needed.
pub fn record(root: &Path, release: Release) -> std::io::Result<()> {
    let mut crate_entries: Vec<(String, u64)> = release.crates.into_iter().collect();
    crate_entries.sort_by(|(name_a, bytes_a), (name_b, bytes_b)| {
        bytes_b.cmp(bytes_a).then_with(|| name_a.cmp(name_b))
    });
    crate_entries.truncate(40);
    let trimmed_crates: BTreeMap<String, u64> = crate_entries.into_iter().collect();
    let trimmed_release = Release {
        crates: trimmed_crates,
        ..release
    };

    let target_package = trimmed_release.package.clone();
    let mut all = read_history(root);
    all.push(trimmed_release);

    let pkg_count = all.iter().filter(|r| r.package == target_package).count();
    let mut to_skip = pkg_count.saturating_sub(20);
    all.retain(|r| {
        if r.package == target_package && to_skip > 0 {
            to_skip = to_skip.saturating_sub(1);
            false
        } else {
            true
        }
    });

    let path = history_file(root);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, history_text(&all))
}

/// Returns the most recent entries for the given package in oldest-first order,
/// capped at `count`.
pub fn last_of<'a>(releases: &'a [Release], package: &str, count: usize) -> Vec<&'a Release> {
    let matching: Vec<&'a Release> = releases.iter().filter(|r| r.package == package).collect();
    let start = matching.len().saturating_sub(count);
    matching.get(start..).unwrap_or_default().to_vec()
}

/// A symbol with mangled or raw name, byte size, and originating library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub size: u64,
    pub library: String,
}

/// A symbol before its size is known: where it starts inside which section.
#[derive(Debug)]
struct Placed {
    section: u64,
    offset: u64,
    name: String,
    library: String,
}

/// Sizes symbols by the gap to the next one in the same section; the last one runs to the
/// section's end. Two symbols at the same place: the first gets 0 and is dropped.
fn sized_by_gaps(mut placed: Vec<Placed>, section_ends: &BTreeMap<u64, u64>) -> Vec<Symbol> {
    placed.sort_by_key(|p| (p.section, p.offset));
    let mut symbols = Vec::new();
    let mut iter = placed.into_iter().peekable();
    while let Some(cur) = iter.next() {
        let end = match iter.peek() {
            Some(next) if next.section == cur.section => next.offset,
            _ => section_ends
                .get(&cur.section)
                .copied()
                .unwrap_or(cur.offset),
        };
        let size = end.saturating_sub(cur.offset);
        if size > 0 {
            symbols.push(Symbol {
                name: cur.name,
                size,
                library: cur.library,
            });
        }
    }
    symbols
}

fn parse_map_section_offset(s: &str) -> Option<(u64, u64)> {
    let (sec_str, off_str) = s.split_once(':')?;
    let sec = u64::from_str_radix(sec_str, 16).ok()?;
    let off = u64::from_str_radix(off_str, 16).ok()?;
    Some((sec, off))
}

#[derive(PartialEq, Eq)]
enum MapParserState {
    Header,
    Sections,
    PublicsOrStatics,
}

fn extract_map_library(tokens: &[&str]) -> String {
    let lib_token = if tokens.get(3) == Some(&"f") {
        tokens.get(4)
    } else {
        tokens.get(3)
    };
    match lib_token {
        Some(token) => match token.split_once(':') {
            Some((lib, _)) => lib.to_string(),
            None => String::new(),
        },
        None => String::new(),
    }
}

/// Parses an MSVC linker map into symbols with computed sizes.
pub fn parse_msvc_map(text: &str) -> Vec<Symbol> {
    let mut section_ends: BTreeMap<u64, u64> = BTreeMap::new();
    let mut raw_symbols = Vec::new();
    let mut state = MapParserState::Header;

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.contains("Start") && trimmed.contains("Length") && trimmed.contains("Class") {
            state = MapParserState::Sections;
            continue;
        }
        if trimmed.contains("Publics by Value") || trimmed.contains("Static symbols") {
            state = MapParserState::PublicsOrStatics;
            continue;
        }

        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }

        match state {
            MapParserState::Sections => {
                if let (Some(sec_token), Some(len_token)) = (tokens.first(), tokens.get(1))
                    && let Some((sec, off)) = parse_map_section_offset(sec_token)
                    && let Ok(len) = u64::from_str_radix(len_token.trim_end_matches(['H', 'h']), 16)
                {
                    let end = section_ends.entry(sec).or_insert(0);
                    *end = (*end).max(off.saturating_add(len));
                }
            }
            MapParserState::PublicsOrStatics => {
                if tokens.len() < 3 {
                    continue;
                }
                let Some(sec_token) = tokens.first() else {
                    continue;
                };
                let Some((sec, off)) = parse_map_section_offset(sec_token) else {
                    continue;
                };
                if sec == 0 {
                    continue;
                }
                let Some(&sec_end) = section_ends.get(&sec) else {
                    continue;
                };
                if off >= sec_end {
                    continue;
                }
                let Some(name_token) = tokens.get(1) else {
                    continue;
                };

                let library = extract_map_library(&tokens);
                raw_symbols.push(Placed {
                    section: sec,
                    offset: off,
                    name: (*name_token).to_string(),
                    library,
                });
            }
            MapParserState::Header => {}
        }
    }

    sized_by_gaps(raw_symbols, &section_ends)
}

/// Parses defined text and data symbols from an executable binary via the
/// `object` crate. Windows PE files return an error instructing callers to read
/// the linker map instead.
pub fn symbols_from_binary(bytes: &[u8]) -> Result<Vec<Symbol>, String> {
    use object::{BinaryFormat, Object, ObjectSection, ObjectSymbol, SymbolSection};

    let file = object::File::parse(bytes).map_err(|e| e.to_string())?;
    if file.format() == BinaryFormat::Pe {
        return Err("no symbol table — the Windows build reads the linker map".to_string());
    }
    let mut placed = Vec::new();
    let mut sized = Vec::new();
    for sym in file.symbols() {
        // A definition is code or data placed in the file (`object` leaves out sections,
        // file names and imports), so every one of them weighs in the executable.
        if !sym.is_definition() {
            continue;
        }
        let (Ok(name), SymbolSection::Section(section)) = (sym.name(), sym.section()) else {
            continue;
        };
        if sym.size() > 0 {
            sized.push(Symbol {
                name: name.to_string(),
                size: sym.size(),
                library: String::new(),
            });
        }
        placed.push(Placed {
            section: u64::try_from(section.0).unwrap_or(u64::MAX),
            offset: sym.address(),
            name: name.to_string(),
            library: String::new(),
        });
    }
    if !sized.is_empty() {
        return Ok(sized);
    }
    // Mach-O symbols carry no size: measure them by the gaps, like the map.
    let section_ends = file
        .sections()
        .map(|sec| {
            let key = u64::try_from(sec.index().0).unwrap_or(u64::MAX);
            (key, sec.address().saturating_add(sec.size()))
        })
        .collect();
    Ok(sized_by_gaps(placed, &section_ends))
}

fn canonical_crate(name: &str) -> String {
    match name {
        "std" | "core" | "alloc" | "panic_unwind" | "panic_abort" | "std_detect"
        | "compiler_builtins" | "__rustc" | "___rustc" => "std".to_string(),
        other => other.to_string(),
    }
}

fn strip_type_modifiers(mut s: &str) -> &str {
    loop {
        let prev = s;
        s = s.trim_start_matches(['<', '&', '[', ' ']);
        if let Some(rest) = s.strip_prefix("mut ") {
            s = rest.trim_start();
        }
        if let Some(rest) = s.strip_prefix("dyn ") {
            s = rest.trim_start();
        }
        if s == prev {
            break;
        }
    }
    s
}

/// The crate a demangled path starts with. For a trait impl (`<&mut a::B as c::D>::f`)
/// that is the crate of the TYPE `a::B`, after the `<`, `&`, `[`, `mut ` and `dyn ` in front.
fn extract_crate_from_demangled(text: &str) -> Option<String> {
    let first = strip_type_modifiers(text)
        .split(['<', '>', '[', ']', ' ', ':'])
        .next()?
        .trim();
    (!first.is_empty()).then(|| canonical_crate(first))
}

fn crate_from_library(lib: &str) -> Option<String> {
    let rest = lib.strip_prefix("lib")?;
    let (name, hash) = rest.rsplit_once('-')?;
    if hash.len() == 16 && hash.chars().all(|c| c.is_ascii_hexdigit()) {
        Some(canonical_crate(name))
    } else {
        None
    }
}

/// The demangled `{:#}` form (no hash) of a Rust symbol, also with the extra leading `_` of macOS.
fn demangled(name: &str) -> Option<String> {
    rustc_demangle::try_demangle(name)
        .or_else(|_| rustc_demangle::try_demangle(name.strip_prefix('_').unwrap_or(name)))
        .ok()
        .map(|d| format!("{d:#}"))
}

/// Identifies the owning crate for a given symbol.
pub fn crate_of(symbol: &Symbol) -> String {
    demangled(&symbol.name)
        .and_then(|text| extract_crate_from_demangled(&text))
        .or_else(|| {
            (symbol.name.starts_with('<') || symbol.name.contains("::"))
                .then(|| extract_crate_from_demangled(&symbol.name))
                .flatten()
        })
        .or_else(|| crate_from_library(&symbol.library))
        .unwrap_or_else(|| "[system]".to_string())
}

/// Returns the demangled `{:#}` function signature, or the raw name if it
/// does not demangle.
pub fn function_name(symbol: &Symbol) -> String {
    demangled(&symbol.name).unwrap_or_else(|| symbol.name.clone())
}

/// Sums symbol byte sizes grouped by crate.
pub fn by_crate(symbols: &[Symbol]) -> BTreeMap<String, u64> {
    let mut map = BTreeMap::new();
    for symbol in symbols {
        let k = crate_of(symbol);
        *map.entry(k).or_insert(0) += symbol.size;
    }
    map
}

/// A row on the Size report representing a crate's byte footprint and growth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrateRow {
    pub name: String,
    pub bytes: u64,
    pub share: u8,
    pub change: Option<i64>,
    pub own: bool,
    pub grew_most: bool,
}

/// Whole percent of `total`, rounded half up.
fn share_of(bytes: u64, total: u64) -> u8 {
    if total == 0 {
        return 0;
    }
    let percent = bytes.saturating_mul(100).saturating_add(total / 2) / total;
    u8::try_from(percent).unwrap_or(100)
}

/// Bytes now minus bytes before (a crate missing before counts from 0).
fn change_of(bytes: u64, before: u64) -> i64 {
    let now = i128::from(bytes);
    i64::try_from(now - i128::from(before)).unwrap_or(i64::MAX)
}

/// Builds the crate breakdown rows: the `top` heaviest crates, and an optional
/// summary row for all remaining crates.
pub fn crate_rows(
    current: &BTreeMap<String, u64>,
    previous: Option<&BTreeMap<String, u64>>,
    own: &str,
    top: usize,
) -> (Vec<CrateRow>, Option<CrateRow>) {
    let total: u64 = current.values().sum();
    let own_normalized = own.replace('-', "_");
    let mut rows: Vec<CrateRow> = current
        .iter()
        .map(|(name, &bytes)| CrateRow {
            name: name.clone(),
            bytes,
            share: share_of(bytes, total),
            change: previous.map(|prev| change_of(bytes, prev.get(name).copied().unwrap_or(0))),
            own: name.replace('-', "_") == own_normalized,
            grew_most: false,
        })
        .collect();
    // The one crate that grew most, decided over ALL crates, flagged only if it is shown.
    let champion = rows
        .iter()
        .filter(|r| r.change.is_some_and(|c| c > 0))
        .max_by_key(|r| (r.change, std::cmp::Reverse(r.name.clone())))
        .map(|r| r.name.clone());
    rows.sort_by(|a, b| b.bytes.cmp(&a.bytes).then_with(|| a.name.cmp(&b.name)));

    let rest = rows.split_off(top.min(rows.len()));
    for row in &mut rows {
        row.grew_most = champion.as_ref() == Some(&row.name);
    }
    if rest.is_empty() {
        return (rows, None);
    }
    let rest_bytes: u64 = rest.iter().map(|r| r.bytes).sum();
    let summary = CrateRow {
        name: format!("{} more crates", rest.len()),
        bytes: rest_bytes,
        share: share_of(rest_bytes, total),
        change: None,
        own: false,
        grew_most: false,
    };
    (rows, Some(summary))
}

/// Returns the heaviest functions by demangled name, summing sizes across
/// occurrences and sorting by size descending.
pub fn function_rows(symbols: &[Symbol], top: usize) -> Vec<(String, u64)> {
    let mut map: BTreeMap<String, u64> = BTreeMap::new();
    for sym in symbols {
        let name = function_name(sym);
        *map.entry(name).or_insert(0) += sym.size;
    }
    let mut rows: Vec<(String, u64)> = map.into_iter().collect();
    rows.sort_by(|(name_a, size_a), (name_b, size_b)| {
        size_b.cmp(size_a).then_with(|| name_a.cmp(name_b))
    });
    rows.truncate(top);
    rows
}

/// Formats a byte quantity into a human-readable base-1024 string.
pub fn human_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1_048_576 {
        let kb = (bytes.saturating_add(512)) / 1024;
        format!("{kb} KB")
    } else {
        let hundredths = (bytes.saturating_mul(100).saturating_add(524_288)) / 1_048_576;
        let mb = hundredths / 100;
        let frac = hundredths % 100;
        format!("{mb}.{frac:02} MB")
    }
}

/// Formats a signed byte difference into a human-readable delta string.
pub fn human_delta(delta: i64) -> String {
    let sign = match delta.cmp(&0) {
        std::cmp::Ordering::Equal => return "no change".to_string(),
        std::cmp::Ordering::Less => '-',
        std::cmp::Ordering::Greater => '+',
    };
    format!("{sign}{}", human_size(delta.unsigned_abs()))
}

/// The shrink tip options that adjust `[profile.release]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShrinkKey {
    Strip,
    Lto,
    CodegenUnits,
    OptLevel,
    Panic,
}

impl ShrinkKey {
    pub const ALL: [Self; 5] = [
        Self::Strip,
        Self::Lto,
        Self::CodegenUnits,
        Self::OptLevel,
        Self::Panic,
    ];

    pub fn key(self) -> &'static str {
        match self {
            Self::Strip => "strip",
            Self::Lto => "lto",
            Self::CodegenUnits => "codegen-units",
            Self::OptLevel => "opt-level",
            Self::Panic => "panic",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Strip => "Leave debug symbols out of the executable",
            Self::Lto => "Optimize across crates at link time",
            Self::CodegenUnits => "Compile each crate as one unit",
            Self::OptLevel => "Optimize for size instead of speed",
            Self::Panic => "End the app at a panic, without unwinding",
        }
    }

    pub fn cost(self) -> &'static str {
        match self {
            Self::Strip => "Stack traces of a crash lose function names.",
            Self::Lto => "Release builds take longer.",
            Self::CodegenUnits => "Smaller and faster code; slower Release builds.",
            Self::OptLevel => "The app may run slower. Only when size matters most.",
            Self::Panic => "A panic can no longer be caught.",
        }
    }

    pub fn proposed(self) -> &'static str {
        match self {
            Self::Strip => "\"symbols\"",
            Self::Lto => "\"thin\"",
            Self::CodegenUnits => "1",
            Self::OptLevel => "\"s\"",
            Self::Panic => "\"abort\"",
        }
    }

    pub fn cargo_default(self) -> &'static str {
        match self {
            Self::Strip => "false",
            Self::Lto => "false",
            Self::CodegenUnits => "16",
            Self::OptLevel => "3",
            Self::Panic => "\"unwind\"",
        }
    }

    pub fn checked_by_default(self) -> bool {
        match self {
            Self::Strip | Self::Lto => true,
            Self::CodegenUnits | Self::OptLevel | Self::Panic => false,
        }
    }
}

/// Current state and application status of a shrink tip.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShrinkTip {
    pub key: ShrinkKey,
    pub current: String,
    pub applied: bool,
}

fn get_release_value<'a>(doc: &'a DocumentMut, key: &str) -> Option<&'a Value> {
    doc.get("profile")?
        .as_table_like()?
        .get("release")?
        .as_table_like()?
        .get(key)?
        .as_value()
}

/// Whether `val` is the key's proposed value (compared as TOML text: `"thin"`, `1`).
fn is_applied(key: ShrinkKey, val: &Value) -> bool {
    value_to_toml_text(val) == key.proposed()
}

fn value_to_toml_text(val: &Value) -> String {
    if let Some(b) = val.as_bool() {
        b.to_string()
    } else if let Some(i) = val.as_integer() {
        i.to_string()
    } else if let Some(s) = val.as_str() {
        format!("\"{s}\"")
    } else {
        val.to_string().trim().to_string()
    }
}

/// Evaluates all five shrink tips against the provided manifest text.
pub fn shrink_tips(manifest: &str) -> Vec<ShrinkTip> {
    let doc: Option<DocumentMut> = manifest.parse().ok();
    ShrinkKey::ALL
        .iter()
        .map(|&key| {
            let value = doc.as_ref().and_then(|d| get_release_value(d, key.key()));
            ShrinkTip {
                key,
                current: value.map_or_else(|| key.cargo_default().to_string(), value_to_toml_text),
                applied: value.is_some_and(|val| is_applied(key, val)),
            }
        })
        .collect()
}

/// Returns the keys that should be checked initially in the shrink dialog:
/// keys already applied, plus defaults for keys not present in the manifest.
pub fn initially_checked(manifest: &str) -> Vec<ShrinkKey> {
    let doc: Option<DocumentMut> = manifest.parse().ok();
    let mut checked = Vec::new();
    for key in ShrinkKey::ALL {
        let value = doc.as_ref().and_then(|d| get_release_value(d, key.key()));
        if value.map_or_else(|| key.checked_by_default(), |val| is_applied(key, val)) {
            checked.push(key);
        }
    }
    checked
}

/// Generates the diff preview lines (`+ key = val` / `- key = val`) for the
/// dialog preview.
pub fn shrink_preview(manifest: &str, checked: &[ShrinkKey]) -> Vec<String> {
    let doc: Option<DocumentMut> = manifest.parse().ok();
    let mut lines = Vec::new();
    for key in ShrinkKey::ALL {
        let is_chk = checked.contains(&key);
        let applied = doc
            .as_ref()
            .and_then(|d| get_release_value(d, key.key()))
            .is_some_and(|v| is_applied(key, v));

        if is_chk && !applied {
            lines.push(format!("+ {} = {}", key.key(), key.proposed()));
        } else if !is_chk && applied {
            lines.push(format!("- {} = {}", key.key(), key.proposed()));
        }
    }
    lines
}

/// Applies the selected shrink tips to `[profile.release]`.
///
/// NOTE: This is meant for the WORKSPACE ROOT `Cargo.toml` (Cargo ignores
/// profiles in member manifests).
pub fn apply_shrink(manifest: &str, checked: &[ShrinkKey]) -> Result<String, String> {
    let mut doc: DocumentMut = manifest
        .parse()
        .map_err(|e: toml_edit::TomlError| e.to_string())?;

    let profile = doc.entry("profile").or_insert_with(|| {
        let mut table = toml_edit::Table::new();
        table.set_implicit(true);
        Item::Table(table)
    });
    let release = profile
        .as_table_like_mut()
        .ok_or_else(|| "[profile] is not a table".to_string())?
        .entry("release")
        .or_insert(Item::Table(toml_edit::Table::new()))
        .as_table_like_mut()
        .ok_or_else(|| "[profile.release] is not a table".to_string())?;
    for key in ShrinkKey::ALL {
        if checked.contains(&key) {
            let value: Value = key
                .proposed()
                .parse()
                .map_err(|e: toml_edit::TomlError| e.to_string())?;
            release.insert(key.key(), Item::Value(value));
        } else if release
            .get(key.key())
            .and_then(Item::as_value)
            .is_some_and(|v| is_applied(key, v))
        {
            release.remove(key.key());
        }
    }
    Ok(doc.to_string())
}

/// State for the shrink profile.release dialog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShrinkDialog {
    pub manifest_path: PathBuf,
    pub manifest: String,
    pub tips: Vec<ShrinkTip>,
    pub checked: Vec<ShrinkKey>,
}

impl ShrinkDialog {
    pub fn new(manifest_path: PathBuf, manifest: String) -> Self {
        let tips = shrink_tips(&manifest);
        let checked = initially_checked(&manifest);
        Self {
            manifest_path,
            manifest,
            tips,
            checked,
        }
    }

    /// Formats the five recommendation rows as plain `(key, label, cost, current → proposed)`.
    pub fn tip_rows(&self) -> Vec<(String, String, String, String)> {
        ShrinkKey::ALL
            .iter()
            .map(|key| {
                let tip = self.tips.iter().find(|t| t.key == *key);
                let current_str = tip.map_or("-", |t| t.current.as_str());
                let proposed_str = key.proposed();
                let diff = format!("{current_str} \u{2192} {proposed_str}");
                (
                    key.key().to_string(),
                    key.label().to_string(),
                    key.cost().to_string(),
                    diff,
                )
            })
            .collect()
    }

    /// Indices into `ShrinkKey::ALL` for currently checked tips.
    pub fn checked_indices(&self) -> Vec<usize> {
        ShrinkKey::ALL
            .iter()
            .enumerate()
            .filter_map(|(i, k)| self.checked.contains(k).then_some(i))
            .collect()
    }

    /// Sets the checked keys from indices into `ShrinkKey::ALL`.
    pub fn set_checked_from_indices(&mut self, indices: &[usize]) {
        self.checked.clear();
        for &i in indices {
            if let Some(&k) = ShrinkKey::ALL.get(i) {
                self.checked.push(k);
            }
        }
    }
}

/// Formats the status line reporting the applied Cargo.toml shrink changes from preview diff lines.
///
/// Example outputs:
/// - `Cargo.toml updated: strip, lto`
/// - `Cargo.toml updated: removed: opt-level`
/// - `Cargo.toml updated: strip; removed: codegen-units`
/// - `Cargo.toml updated`
pub fn shrink_status<S: AsRef<str>>(preview_lines: &[S]) -> String {
    let mut added = Vec::new();
    let mut removed = Vec::new();
    for line in preview_lines {
        let text = line.as_ref().trim();
        if let Some(rest) = text.strip_prefix('+') {
            if let Some(key) = rest.trim().split('=').next() {
                let k = key.trim();
                if !k.is_empty() {
                    added.push(k);
                }
            }
        } else if let Some(rest) = text.strip_prefix('-')
            && let Some(key) = rest.trim().split('=').next()
        {
            let k = key.trim();
            if !k.is_empty() {
                removed.push(k);
            }
        }
    }

    match (added.is_empty(), removed.is_empty()) {
        (true, true) => "Cargo.toml updated".to_string(),
        (false, true) => format!("Cargo.toml updated: {}", added.join(", ")),
        (true, false) => format!("Cargo.toml updated: removed: {}", removed.join(", ")),
        (false, false) => format!(
            "Cargo.toml updated: {}; removed: {}",
            added.join(", "),
            removed.join(", ")
        ),
    }
}

/// The linker map path for a release build of a given package.
pub fn map_file(target_dir: &Path, package: &str) -> PathBuf {
    target_dir
        .join("release")
        .join(format!("crawrad-size-{package}.map"))
}

/// MSVC link.exe arguments requesting a map file.
pub fn map_link_args(map: &Path) -> Vec<String> {
    vec!["-C".to_string(), format!("link-arg=/MAP:{}", map.display())]
}

/// Measures a release build's executable byte count, per-crate breakdown, and parsed symbols.
///
/// An unreadable `exe` returns an error naming the path. When `map` is provided,
/// symbols are parsed from the MSVC linker map; otherwise symbols are parsed
/// from the binary, falling back to empty symbols if the binary contains no symbol table.
pub fn measure_with_symbols(
    package: &str,
    exe: &Path,
    map: Option<&str>,
    at: u64,
) -> Result<(Release, Vec<Symbol>), String> {
    let exe_bytes = std::fs::read(exe).map_err(|e| format!("{}: {e}", exe.display()))?;
    let bytes = u64::try_from(exe_bytes.len()).unwrap_or(u64::MAX);
    let symbols = match map {
        Some(text) => parse_msvc_map(text),
        None => symbols_from_binary(&exe_bytes).unwrap_or_default(),
    };
    let crates = by_crate(&symbols);
    let release = Release {
        package: package.to_string(),
        at,
        bytes,
        crates,
    };
    Ok((release, symbols))
}

/// Measures a release build's executable byte count and per-crate breakdown.
///
/// An unreadable `exe` returns an error naming the path. When `map` is provided,
/// crate sizes are parsed from the MSVC linker map; otherwise symbols are parsed
/// from the binary, falling back to empty crates if the binary contains no symbols.
pub fn measure(package: &str, exe: &Path, map: Option<&str>, at: u64) -> Result<Release, String> {
    measure_with_symbols(package, exe, map, at).map(|(r, _)| r)
}

/// Converts a `u64` byte count to `f64` for bar height calculations. The 53-bit
/// mantissa of `f64` preserves exact integer precision up to 9 PB, which is far
/// larger than any executable size.
#[inline]
fn bytes_to_f64(bytes: u64) -> f64 {
    bytes as f64
}

/// Converts days since UNIX epoch (1970-01-01) into (year, month, day) in the Gregorian calendar.
fn civil_from_days(days: i64) -> (i32, u32, u32) {
    let z = days.saturating_add(719_468);
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let mut y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    if m <= 2 {
        y += 1;
    }
    (
        i32::try_from(y).unwrap_or(0),
        u32::try_from(m).unwrap_or(1),
        u32::try_from(d).unwrap_or(1),
    )
}

/// Formats a UNIX timestamp as `dd/mm HH:MM`, in UTC (local time needs an external crate).
pub fn format_when(at: u64) -> String {
    let days = i64::try_from(at / 86_400).unwrap_or(0);
    let (_, m, d) = civil_from_days(days);
    let rem_secs = at % 86_400;
    let hour = rem_secs / 3600;
    let min = (rem_secs % 3600) / 60;
    format!("{d:02}/{m:02} {hour:02}:{min:02}")
}

/// Formats a UNIX timestamp as `dd/mm`, in UTC (local time needs an external crate).
pub fn format_day_month(at: u64) -> String {
    let days = i64::try_from(at / 86_400).unwrap_or(0);
    let (_, m, d) = civil_from_days(days);
    format!("{d:02}/{m:02}")
}

/// The direction of size change between releases.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trend {
    Grew,
    Shrank,
    Same,
}

/// Plain standard types representing the Size page for presentation widgets.
pub type SizePageRawParts = (
    String,
    String,
    Option<(String, u8)>,
    Option<String>,
    Vec<f64>,
    Vec<(String, String, String, String, bool)>,
    Vec<(String, String)>,
    String,
);

/// The structured data model for the Size page in the Build window.
#[derive(Debug, Clone, PartialEq)]
pub struct SizePage {
    pub exe_label: String,
    pub when: String,
    pub size: String,
    pub change: Option<(String, Trend)>,
    pub since: Option<String>,
    pub bars: Vec<f64>,
    pub crates: Vec<CrateRow>,
    pub more: Option<CrateRow>,
    pub functions: Vec<(String, u64)>,
}

impl SizePage {
    /// Turns the Size page model into plain standard parts for presentation widgets.
    pub fn parts(&self) -> SizePageRawParts {
        let header = format!("{}, {}", self.exe_label, self.when);
        let change = self.change.as_ref().map(|(c, t)| {
            let code = match t {
                Trend::Grew => 0,
                Trend::Shrank => 1,
                Trend::Same => 2,
            };
            (c.clone(), code)
        });
        let crates = self
            .crates
            .iter()
            .chain(self.more.as_ref())
            .map(|row| {
                let change_text = match row.change {
                    Some(c) => {
                        let delta = human_delta(c);
                        if row.grew_most {
                            format!("▲ {delta}")
                        } else {
                            delta
                        }
                    }
                    None => "—".to_string(),
                };
                (
                    row.name.clone(),
                    human_size(row.bytes),
                    format!("{}%", row.share),
                    change_text,
                    row.own,
                )
            })
            .collect();
        let functions = self
            .functions
            .iter()
            .map(|(name, size)| (name.clone(), human_size(*size)))
            .collect();
        let footnote = "Read from the linker's map of this build — no extra tool installed. Your own crate in purple.".to_string();
        (
            header,
            self.size.clone(),
            change,
            self.since.clone(),
            self.bars.clone(),
            crates,
            functions,
            footnote,
        )
    }
}

/// Builds the model for the Size page from recorded release history, current symbols, and package names.
///
/// Dates from `at` (UNIX seconds) are formatted as `dd/mm HH:MM UTC` because local time
/// is not available without an external crate; `since` uses `dd/mm`.
pub fn size_page(
    history: &[Release],
    package: &str,
    exe_name: &str,
    symbols: &[Symbol],
    own: &str,
) -> Option<SizePage> {
    let matching: Vec<&Release> = history.iter().filter(|r| r.package == package).collect();
    let current = matching.last().copied()?;
    let previous = matching.iter().rev().nth(1).copied();

    let when = format_when(current.at);
    let size = human_size(current.bytes);
    let exe_label = if exe_name.ends_with(" · Release") {
        exe_name.to_string()
    } else {
        format!("{exe_name} · Release")
    };

    let (change, since) = match previous {
        Some(prev) => {
            let diff = change_of(current.bytes, prev.bytes);
            let trend = match diff.cmp(&0) {
                std::cmp::Ordering::Greater => Trend::Grew,
                std::cmp::Ordering::Less => Trend::Shrank,
                std::cmp::Ordering::Equal => Trend::Same,
            };
            let change_str = human_delta(diff);
            let prev_date = format_day_month(prev.at);
            let prev_size = human_size(prev.bytes);
            let since_str = format!("since the last Release ({prev_size}, {prev_date})");
            (Some((change_str, trend)), Some(since_str))
        }
        None => (None, None),
    };

    let last_6 = last_of(history, package, 6);
    let max_bytes = last_6.iter().map(|r| r.bytes).max().unwrap_or(0);
    let bars = if max_bytes == 0 {
        vec![0.0; last_6.len()]
    } else {
        last_6
            .iter()
            .map(|r| bytes_to_f64(r.bytes) / bytes_to_f64(max_bytes))
            .collect()
    };

    let (crates, more) = crate_rows(&current.crates, previous.map(|p| &p.crates), own, 5);
    let functions = if symbols.is_empty() {
        Vec::new()
    } else {
        function_rows(symbols, 20)
    };

    Some(SizePage {
        exe_label,
        when,
        size,
        change,
        since,
        bars,
        crates,
        more,
        functions,
    })
}

/// The last recorded Release of the same package (`current` is not recorded yet).
fn find_previous<'a>(history: &'a [Release], current: &Release) -> Option<&'a Release> {
    history.iter().rfind(|r| r.package == current.package)
}

/// Formats a one-line summary of the release executable size and its change
/// compared to the previous recorded release of the same package.
pub fn size_line(history: &[Release], current: &Release) -> String {
    let size = human_size(current.bytes);
    let Some(prev) = find_previous(history, current) else {
        return format!("Release size: {size}");
    };
    let delta = human_delta(change_of(current.bytes, prev.bytes));
    format!("Release size: {size} ({delta} since the last Release)")
}

/// The structured model for the Release finished corner card and status bar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseCard {
    pub headline: String,
    pub size: String,
    pub change: Option<(String, bool /* grew */)>,
    pub mostly: Option<String>,
}

impl ReleaseCard {
    /// Formats the summary text for the editor status bar.
    pub fn status_text(&self) -> String {
        match &self.change {
            Some((delta, _)) => format!("Release {} ({delta})", self.size),
            None => format!("Release {}", self.size),
        }
    }
}

/// Formats a build elapsed duration as `M:SS` or `H:MM:SS` for the release card headline.
pub fn format_build_clock(elapsed: Duration) -> String {
    let secs = elapsed.as_secs();
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    let rem_secs = secs % 60;
    if hours > 0 {
        format!("{hours}:{mins:02}:{rem_secs:02}")
    } else {
        format!("{mins}:{rem_secs:02}")
    }
}

/// Builds the release card model from the recorded history, current release measurement, and build duration.
pub fn release_card(history: &[Release], current: &Release, elapsed: Duration) -> ReleaseCard {
    let size = human_size(current.bytes);
    let headline = format!("Release built in {}", format_build_clock(elapsed));

    let previous = find_previous(history, current);

    let (change, mostly) = match previous {
        Some(prev) => {
            let diff = change_of(current.bytes, prev.bytes);
            if diff > 0 {
                let change_text = human_delta(diff);
                let champion = current
                    .crates
                    .iter()
                    .filter_map(|(name, &bytes)| {
                        let prev_bytes = prev.crates.get(name).copied().unwrap_or(0);
                        let crate_diff = change_of(bytes, prev_bytes);
                        (crate_diff > 0).then_some((crate_diff, name))
                    })
                    .max_by_key(|&(crate_diff, name)| (crate_diff, std::cmp::Reverse(name)))
                    .map(|(_, name)| format!("mostly {name}"));
                (Some((change_text, true)), champion)
            } else if diff < 0 {
                (Some((human_delta(diff), false)), None)
            } else {
                (None, None)
            }
        }
        None => (None, None),
    };

    ReleaseCard {
        headline,
        size,
        change,
        mostly,
    }
}

#[cfg(test)]
#[path = "test/release_size/test.rs"]
mod tests;
