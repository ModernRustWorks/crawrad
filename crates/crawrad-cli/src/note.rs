//! Scans notes and source code anchors without compiling anything.
//!
//! Reconciles `.crawrad/notes/*.md` with the `.rs` files under `src/`, `tests/`, `examples/` and `benches/`, identifying orphan notes,
//! dangling anchors, kind mismatches, and stale signatures.

use crawrad_note::note;
use crawrad_note_format::{NoteFile, NoteKind};
use quote::ToTokens;
use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use syn::parse::Parser;
use syn::visit::Visit;

#[note("note-missingdoc")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MissingDoc {
    /// Source file, relative to the crate folder, with forward slashes.
    pub file: PathBuf,
    /// 1-based line of the item's first attribute or keyword.
    pub line: usize,
    /// `crate`, `mod`, `fn`, `struct`, `field`, `variant`, `method`, ...
    pub kind: &'static str,
    /// The item's name; the package name for `crate`.
    pub name: String,
}

#[note("note-noteentry")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteEntry {
    pub id: String,
    pub kind: NoteKind,
    pub anchor: Option<String>,
}

#[note("note-danglinganchor")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DanglingAnchor {
    pub id: String,
    pub file: String,
    pub line: usize,
}

#[note("note-kindmismatch")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindMismatch {
    pub id: String,
    pub file: String,
    pub line: usize,
    pub kind: NoteKind,
}

#[note("note-stalesignature")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaleSignature {
    pub id: String,
    pub file: String,
    pub line: usize,
    pub message: String,
}

#[note("note-docproblem")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocProblem {
    pub id: String,
    pub file: String,
    pub line: usize,
    pub message: String,
}

#[note("note-brokenlink")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrokenLink {
    /// Source file path relative to the crate root, with forward slashes.
    pub file: String,
    /// 1-based line of the primary span.
    pub line: usize,
    /// Id of the doc note whose anchor is on that line, or `None` for a link in plain `///`.
    pub note: Option<String>,
    /// The diagnostic message from rustdoc.
    pub message: String,
}

#[note("note-notereport")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteReport {
    pub notes: Vec<NoteEntry>,
    pub orphans: Vec<NoteEntry>,
    pub dangling: Vec<DanglingAnchor>,
    pub mismatches: Vec<KindMismatch>,
    pub stale: Vec<StaleSignature>,
    pub doc_problems: Vec<DocProblem>,
}

impl NoteReport {
    #[note("note-notereport-is-clean")]
    pub fn is_clean(&self) -> bool {
        self.orphans.is_empty()
            && self.dangling.is_empty()
            && self.mismatches.is_empty()
            && self.stale.is_empty()
            && self.doc_problems.is_empty()
    }

    #[note("note-notereport-has-problems")]
    pub fn has_problems(&self) -> bool {
        !self.is_clean()
    }
}

impl fmt::Display for NoteReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "notes:")?;
        if self.notes.is_empty() {
            writeln!(f, "  (none)")?;
        } else {
            for n in &self.notes {
                let loc = n.anchor.as_deref().unwrap_or("—");
                writeln!(f, "  {} ({}): {}", n.id, n.kind, loc)?;
            }
        }

        if !self.orphans.is_empty() {
            writeln!(f, "\norphan notes:")?;
            for o in &self.orphans {
                writeln!(f, "  {}", o.id)?;
            }
        }

        if !self.dangling.is_empty() {
            writeln!(f, "\ndangling anchors:")?;
            for d in &self.dangling {
                writeln!(f, "  {} at {}:{}", d.id, d.file, d.line)?;
            }
        }

        if !self.mismatches.is_empty() {
            writeln!(f, "\nkind mismatch:")?;
            for m in &self.mismatches {
                writeln!(
                    f,
                    "  {} at {}:{}: note is {}; only kind: doc notes become documentation",
                    m.id,
                    m.file,
                    m.line,
                    m.kind.as_str()
                )?;
            }
        }

        if !self.stale.is_empty() {
            writeln!(f, "\nstale:")?;
            for s in &self.stale {
                writeln!(f, "  {}", s.message)?;
            }
        }

        if !self.doc_problems.is_empty() {
            writeln!(f, "\nrustdoc:")?;
            for p in &self.doc_problems {
                writeln!(f, "  {} at {}:{}: {}", p.id, p.file, p.line, p.message)?;
            }
        }

        Ok(())
    }
}

#[note("note-inlinednote")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlinedNote {
    /// Note identifier.
    pub id: String,
    /// Relative path of the source file.
    pub file: String,
    /// 1-based line of the original anchor attribute.
    pub line: usize,
}

#[note("note-skippednote")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedNote {
    /// Note identifier.
    pub id: String,
    /// Relative path of the source file.
    pub file: String,
    /// 1-based line of the anchor attribute.
    pub line: usize,
    /// Reason explaining why the anchor was skipped.
    pub reason: String,
}

#[note("note-inlinereport")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineReport {
    /// Notes successfully inlined into source files.
    pub inlined: Vec<InlinedNote>,
    /// Anchors skipped with a reason.
    pub skipped: Vec<SkippedNote>,
}

#[note("note-extractednote")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractedNote {
    /// Note identifier.
    pub id: String,
    /// Relative path of the source file.
    pub file: String,
    /// 1-based line of the new `#[note]` anchor.
    pub line: usize,
    /// Name of the extracted item.
    pub item: String,
}

#[note("note-skippeddoc")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedDoc {
    /// Relative path of the source file.
    pub file: String,
    /// 1-based line of the item or its first doc comment.
    pub line: usize,
    /// Name of the item.
    pub item: String,
    /// Reason explaining why the doc was kept as `///`.
    pub reason: String,
}

#[note("note-extractreport")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractReport {
    /// Extracted notes.
    pub extracted: Vec<ExtractedNote>,
    /// Items whose docs were skipped and kept as `///`.
    pub skipped: Vec<SkippedDoc>,
    /// True if Cargo.toml was modified (or would be in dry-run) to add `crawrad-note`.
    pub cargo_toml_changed: bool,
}

#[note("note-foundanchor")]
#[derive(Debug, Clone)]
struct FoundAnchor {
    id: String,
    file: String,
    line: usize,
    signature: String,
    facts: crawrad_note_format::rustdoc::ItemFacts,
}

#[note("note-scan")]
pub fn scan(root: &Path) -> Result<NoteReport, String> {
    if !root.join("Cargo.toml").is_file() {
        return Err(format!("{}: no Cargo.toml found", root.display()));
    }

    let notes_by_id = read_notes(root)?;
    let anchors = collect_anchors(root)?;

    let mut anchors_by_id: BTreeMap<String, Vec<FoundAnchor>> = BTreeMap::new();
    for a in anchors {
        anchors_by_id.entry(a.id.clone()).or_default().push(a);
    }

    let mut notes = Vec::new();
    let mut orphans = Vec::new();
    let mut mismatches = Vec::new();
    let mut stale = Vec::new();
    let mut doc_problems = Vec::new();

    for (id, note) in &notes_by_id {
        if let Some(matching) = anchors_by_id.get(id) {
            let anchor_str = matching
                .iter()
                .map(|a| format!("{}:{}", a.file, a.line))
                .collect::<Vec<_>>()
                .join(", ");
            notes.push(NoteEntry {
                id: id.clone(),
                kind: note.kind,
                anchor: Some(anchor_str),
            });

            for a in matching {
                if note.kind != NoteKind::Doc {
                    mismatches.push(KindMismatch {
                        id: id.clone(),
                        file: a.file.clone(),
                        line: a.line,
                        kind: note.kind,
                    });
                }
                if let Some(msg) =
                    crawrad_note_format::stale_signature(id, &note.signature, &a.signature)
                {
                    stale.push(StaleSignature {
                        id: id.clone(),
                        file: a.file.clone(),
                        line: a.line,
                        message: msg,
                    });
                }
                if note.kind == NoteKind::Doc {
                    let problems = crawrad_note_format::rustdoc::doc_problems(&note.body, &a.facts);
                    for problem in problems {
                        doc_problems.push(DocProblem {
                            id: id.clone(),
                            file: a.file.clone(),
                            line: a.line,
                            message: problem,
                        });
                    }
                }
            }
        } else {
            let entry = NoteEntry {
                id: id.clone(),
                kind: note.kind,
                anchor: None,
            };
            notes.push(entry.clone());
            orphans.push(entry);
        }
    }

    let mut dangling = Vec::new();
    for (id, found_list) in &anchors_by_id {
        if !notes_by_id.contains_key(id) {
            for a in found_list {
                dangling.push(DanglingAnchor {
                    id: id.clone(),
                    file: a.file.clone(),
                    line: a.line,
                });
            }
        }
    }

    Ok(NoteReport {
        notes,
        orphans,
        dangling,
        mismatches,
        stale,
        doc_problems,
    })
}

#[note("note-inline-notes")]
pub fn inline_notes(root: &Path, write: bool) -> Result<InlineReport, String> {
    if !root.join("Cargo.toml").is_file() {
        return Err(format!("{}: no Cargo.toml found", root.display()));
    }

    let notes_by_id = read_notes(root)?;
    let anchors = collect_anchors(root)?;

    let mut inlined = Vec::new();
    let mut skipped = Vec::new();

    let mut anchors_by_file: BTreeMap<String, Vec<FoundAnchor>> = BTreeMap::new();
    for a in anchors {
        anchors_by_file.entry(a.file.clone()).or_default().push(a);
    }

    let mut files_to_write: Vec<(PathBuf, String)> = Vec::new();

    for (rel_path, file_anchors) in anchors_by_file {
        let file_path = root.join(&rel_path);
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| format!("cannot read {}: {e}", file_path.display()))?;
        let newline = if content.contains("\r\n") {
            "\r\n"
        } else {
            "\n"
        };
        let has_final_newline = content.ends_with("\r\n") || content.ends_with('\n');
        let mut lines: Vec<String> = content.lines().map(String::from).collect();

        let mut replacements: Vec<(usize, Vec<String>)> = Vec::new();

        for a in file_anchors {
            let line = a.line.checked_sub(1).and_then(|i| lines.get(i));
            match rustdoc_lines(notes_by_id.get(&a.id), line.map(String::as_str), &a.id) {
                Ok(replacement) => {
                    replacements.push((a.line - 1, replacement));
                    inlined.push(InlinedNote {
                        id: a.id,
                        file: rel_path.clone(),
                        line: a.line,
                    });
                }
                Err(reason) => skipped.push(SkippedNote {
                    id: a.id,
                    file: rel_path.clone(),
                    line: a.line,
                    reason,
                }),
            }
        }

        if !replacements.is_empty() {
            replacements.sort_by_key(|r| std::cmp::Reverse(r.0));
            for (idx, rep) in replacements {
                lines.splice(idx..=idx, rep);
            }

            if !has_any_note_attr(&lines) {
                lines.retain(|l| l.trim() != "use crawrad_note::note;");
            }

            let mut new_content = lines.join(newline);
            if has_final_newline && !new_content.is_empty() {
                new_content.push_str(newline);
            }

            files_to_write.push((file_path, new_content));
        }
    }

    if write {
        for (file_path, new_content) in files_to_write {
            std::fs::write(&file_path, new_content)
                .map_err(|e| format!("cannot write {}: {e}", file_path.display()))?;
        }

        let skipped_ids: std::collections::BTreeSet<&str> =
            skipped.iter().map(|s| s.id.as_str()).collect();
        let inlined_ids: std::collections::BTreeSet<&str> =
            inlined.iter().map(|i| i.id.as_str()).collect();

        for id in inlined_ids {
            if !skipped_ids.contains(id) {
                let note_file = root.join(".crawrad").join("notes").join(format!("{id}.md"));
                if note_file.is_file() {
                    std::fs::remove_file(&note_file)
                        .map_err(|e| format!("cannot remove {}: {e}", note_file.display()))?;
                }
            }
        }
    }

    inlined.sort_by(|a, b| a.file.cmp(&b.file).then_with(|| a.line.cmp(&b.line)));
    skipped.sort_by(|a, b| a.file.cmp(&b.file).then_with(|| a.line.cmp(&b.line)));

    Ok(InlineReport { inlined, skipped })
}

#[note("note-crawrad-note-version")]
const CRAWRAD_NOTE_VERSION: &str = "0.1.0";

#[note("note-extract-notes")]
pub fn extract_notes(root: &Path, write: bool, all: bool) -> Result<ExtractReport, String> {
    if !root.join("Cargo.toml").is_file() {
        return Err(format!("{}: no Cargo.toml found", root.display()));
    }

    let mut taken_ids = std::collections::BTreeSet::new();
    let notes_dir = root.join(".crawrad").join("notes");
    if notes_dir.is_dir() {
        let entries = std::fs::read_dir(&notes_dir)
            .map_err(|e| format!("cannot read {}: {e}", notes_dir.display()))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("{}: {e}", notes_dir.display()))?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md")
                && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
            {
                taken_ids.insert(stem.to_string());
            }
        }
    }

    let src_dir = root.join("src");
    let mut roots = Vec::new();
    let lib_rs = src_dir.join("lib.rs");
    if lib_rs.is_file() {
        roots.push((lib_rs, src_dir.clone(), Vec::new()));
    }
    let main_rs = src_dir.join("main.rs");
    if main_rs.is_file() {
        roots.push((main_rs, src_dir.clone(), Vec::new()));
    }
    let bin_dir = src_dir.join("bin");
    if bin_dir.is_dir() {
        let mut bin_files = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&bin_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().is_some_and(|ext| ext == "rs") {
                    bin_files.push(path);
                }
            }
        }
        bin_files.sort();
        for bin_path in bin_files {
            if let Some(stem) = bin_path.file_stem().and_then(|s| s.to_str()) {
                let segments = vec!["bin".to_string(), stem.to_string()];
                roots.push((bin_path, bin_dir.clone(), segments));
            }
        }
    }

    let mut visited_paths = std::collections::BTreeSet::new();
    let mut module_files = Vec::new();

    for (file_path, dir, mod_segments) in roots {
        collect_module_files(
            root,
            &file_path,
            &dir,
            &mod_segments,
            &mut visited_paths,
            &mut module_files,
        )?;
    }

    let mut all_extracted = Vec::new();
    let mut all_skipped = Vec::new();
    let mut files_to_write: Vec<(PathBuf, String)> = Vec::new();
    let mut notes_to_write: Vec<(PathBuf, String)> = Vec::new();

    for mod_file in module_files {
        let file_path = mod_file.file_path;
        let rel_path = mod_file.rel_path;
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| format!("cannot read {}: {e}", file_path.display()))?;
        let newline = if content.contains("\r\n") {
            "\r\n"
        } else {
            "\n"
        };
        let has_final_newline = content.ends_with("\r\n") || content.ends_with('\n');
        let mut lines: Vec<String> = content.lines().map(String::from).collect();

        let syn_file =
            syn::parse_file(&content).map_err(|e| format!("{}: {e}", file_path.display()))?;

        let mut walker = ExtractWalk {
            rel_path: &rel_path,
            lines: &lines,
            file_segments: &mod_file.mod_segments,
            all,
            taken_ids: &mut taken_ids,
            candidates: Vec::new(),
            skipped: Vec::new(),
        };
        walker.visit_items(&syn_file.items, &[], false);
        let ExtractWalk {
            candidates,
            skipped,
            ..
        } = walker;

        all_skipped.extend(skipped);

        if candidates.is_empty() {
            continue;
        }

        let mut replacements = Vec::new();
        let mut scope_file_needed = false;
        let mut scope_mods_needed: std::collections::BTreeSet<Vec<String>> =
            std::collections::BTreeSet::new();

        for cand in &candidates {
            replacements.push((
                cand.first_doc_idx,
                cand.last_doc_idx,
                format!("{}#[note(\"{}\")]", cand.indent, cand.id),
            ));
            if cand.inline_mods.is_empty() {
                scope_file_needed = true;
            } else {
                scope_mods_needed.insert(cand.inline_mods.clone());
            }
        }

        replacements.sort_by_key(|r| std::cmp::Reverse(r.0));
        for (start, end, rep) in replacements {
            lines.splice(start..=end, [rep]);
        }

        let rewritten_text = lines.join("\n");
        let parsed_rewritten = syn::parse_file(&rewritten_text)
            .map_err(|e| format!("{}: {e}", file_path.display()))?;

        let mut use_insertions: Vec<(usize, String)> = Vec::new();

        if scope_file_needed && !has_use_note_at_items(&parsed_rewritten.items) {
            let last_use = parsed_rewritten
                .items
                .iter()
                .filter_map(|it| match it {
                    syn::Item::Use(u) => Some(u),
                    _ => None,
                })
                .next_back();

            if let Some(u) = last_use {
                let insert_line = u.semi_token.span.end().line;
                use_insertions.push((insert_line, "use crawrad_note::note;".to_string()));
            } else if let Some(first_item) = parsed_rewritten.items.first() {
                let first_l = item_line(item_attrs(first_item).unwrap_or(&[]), first_item);
                let insert_line = first_l.saturating_sub(1);
                use_insertions.push((insert_line, "use crawrad_note::note;".to_string()));
            }
        }

        for mod_path in scope_mods_needed {
            // Two modules may share a path (`#[cfg(windows)] mod window` and its
            // `#[cfg(not(windows))]` twin): each one that carries a note gets the `use`.
            for m in find_inline_mods(&parsed_rewritten.items, &mod_path) {
                if let Some((_, mod_items)) = &m.content
                    && carries_note(mod_items)
                    && !has_use_note_at_items(mod_items)
                {
                    let start_search = m.mod_token.span.start().line.saturating_sub(1);
                    let mut brace_line_0based = start_search;
                    for (idx, line) in lines.iter().enumerate().skip(start_search) {
                        if line.contains('{') {
                            brace_line_0based = idx + 1;
                            break;
                        }
                    }

                    let indent = if let Some(first_item) = mod_items.first() {
                        let l = item_line(item_attrs(first_item).unwrap_or(&[]), first_item);
                        if l > 0 && l <= lines.len() {
                            let s = &lines[l - 1];
                            s[..s.len() - s.trim_start().len()].to_string()
                        } else {
                            "    ".to_string()
                        }
                    } else {
                        let mod_l = m.mod_token.span.start().line;
                        let s = if mod_l > 0 && mod_l <= lines.len() {
                            &lines[mod_l - 1]
                        } else {
                            ""
                        };
                        let parent_indent = &s[..s.len() - s.trim_start().len()];
                        format!("{parent_indent}    ")
                    };
                    use_insertions.push((
                        brace_line_0based,
                        format!("{indent}use crawrad_note::note;"),
                    ));
                }
            }
        }

        use_insertions.sort_by_key(|(idx, _)| std::cmp::Reverse(*idx));
        for (idx, line_str) in use_insertions {
            lines.insert(idx, line_str);
        }

        for cand in &candidates {
            let line_pos = lines
                .iter()
                .position(|l| is_attribute_alone_on_line(l, &cand.id))
                .ok_or_else(|| {
                    format!("failed to find #[note(\"{}\")] in rewritten lines", cand.id)
                })?;
            let line_1based = line_pos + 1;

            all_extracted.push(ExtractedNote {
                id: cand.id.clone(),
                file: rel_path.clone(),
                line: line_1based,
                item: cand.item_display.clone(),
            });

            let note_file = NoteFile {
                id: cand.id.clone(),
                kind: NoteKind::Doc,
                kind_declared: true,
                resolved: false,
                title: cand.item_display.clone(),
                target_file: rel_path.clone(),
                line_hint: line_1based,
                author: String::new(),
                created: String::new(),
                tags: Vec::new(),
                signature: cand.signature.clone(),
                body: cand.body.clone(),
            };

            let md_path = crawrad_note_format::note_path(root, &cand.id);
            notes_to_write.push((md_path, note_file.to_markdown()));
        }

        let mut new_content = lines.join(newline);
        if has_final_newline && !new_content.is_empty() {
            new_content.push_str(newline);
        }
        files_to_write.push((file_path, new_content));
    }

    let mut cargo_toml_changed = false;
    let mut cargo_to_write: Option<(PathBuf, String)> = None;

    if !all_extracted.is_empty() {
        let cargo_path = root.join("Cargo.toml");
        let cargo_text = std::fs::read_to_string(&cargo_path)
            .map_err(|e| format!("cannot read {}: {e}", cargo_path.display()))?;
        let cargo_val: toml::Value = toml::from_str(&cargo_text)
            .map_err(|e| format!("cannot parse {}: {e}", cargo_path.display()))?;
        let has_dep = cargo_val
            .get("dependencies")
            .and_then(|d| d.get("crawrad-note"))
            .is_some();

        if !has_dep {
            cargo_toml_changed = true;
            let dep_line = if has_workspace_dep_crawrad_note(root) {
                "crawrad-note = { workspace = true }".to_string()
            } else {
                format!("crawrad-note = \"{CRAWRAD_NOTE_VERSION}\"")
            };
            let new_cargo_text = insert_dependency(&cargo_text, &dep_line);
            cargo_to_write = Some((cargo_path, new_cargo_text));
        }
    }

    if write {
        for (note_path, note_content) in notes_to_write {
            if let Some(parent) = note_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("cannot create dir {}: {e}", parent.display()))?;
            }
            std::fs::write(&note_path, note_content)
                .map_err(|e| format!("cannot write {}: {e}", note_path.display()))?;
        }
        for (file_path, new_content) in files_to_write {
            std::fs::write(&file_path, new_content)
                .map_err(|e| format!("cannot write {}: {e}", file_path.display()))?;
        }
        if let Some((cargo_path, new_cargo_text)) = cargo_to_write {
            std::fs::write(&cargo_path, new_cargo_text)
                .map_err(|e| format!("cannot write {}: {e}", cargo_path.display()))?;
        }
    }

    all_extracted.sort_by(|a, b| a.file.cmp(&b.file).then_with(|| a.line.cmp(&b.line)));
    all_skipped.sort_by(|a, b| a.file.cmp(&b.file).then_with(|| a.line.cmp(&b.line)));

    Ok(ExtractReport {
        extracted: all_extracted,
        skipped: all_skipped,
        cargo_toml_changed,
    })
}

struct ExtractCandidate {
    id: String,
    item_display: String,
    signature: String,
    body: String,
    first_doc_idx: usize,
    last_doc_idx: usize,
    indent: String,
    inline_mods: Vec<String>,
}

struct ExtractWalk<'a> {
    rel_path: &'a str,
    lines: &'a [String],
    file_segments: &'a [String],
    all: bool,
    taken_ids: &'a mut std::collections::BTreeSet<String>,
    candidates: Vec<ExtractCandidate>,
    skipped: Vec<SkippedDoc>,
}

impl ExtractWalk<'_> {
    fn visit_items(&mut self, items: &[syn::Item], inline_mods: &[String], in_cfg_test: bool) {
        for item in items {
            match item {
                syn::Item::Mod(m) => {
                    let mod_test = in_cfg_test || is_cfg_test(&m.attrs);
                    if let Some((_, inner_items)) = &m.content {
                        let mut next_inline = inline_mods.to_vec();
                        next_inline.push(m.ident.to_string());
                        self.visit_items(inner_items, &next_inline, mod_test);
                    }
                }
                syn::Item::Impl(imp) => {
                    let imp_test = in_cfg_test || is_cfg_test(&imp.attrs);
                    let type_name = type_ident(&imp.self_ty);
                    for impl_item in &imp.items {
                        match impl_item {
                            syn::ImplItem::Fn(f) => {
                                self.check_candidate(
                                    &f.attrs,
                                    &f.sig.ident,
                                    f,
                                    Some(&type_name),
                                    inline_mods,
                                    imp_test,
                                );
                            }
                            syn::ImplItem::Const(c) => {
                                self.check_candidate(
                                    &c.attrs,
                                    &c.ident,
                                    c,
                                    Some(&type_name),
                                    inline_mods,
                                    imp_test,
                                );
                            }
                            syn::ImplItem::Type(t) => {
                                self.check_candidate(
                                    &t.attrs,
                                    &t.ident,
                                    t,
                                    Some(&type_name),
                                    inline_mods,
                                    imp_test,
                                );
                            }
                            _ => {}
                        }
                    }
                }
                syn::Item::Trait(tr) => {
                    let tr_test = in_cfg_test || is_cfg_test(&tr.attrs);
                    let trait_name = tr.ident.to_string();
                    self.check_candidate(&tr.attrs, &tr.ident, tr, None, inline_mods, tr_test);
                    for trait_item in &tr.items {
                        match trait_item {
                            syn::TraitItem::Fn(f) => {
                                self.check_candidate(
                                    &f.attrs,
                                    &f.sig.ident,
                                    f,
                                    Some(&trait_name),
                                    inline_mods,
                                    tr_test,
                                );
                            }
                            syn::TraitItem::Const(c) => {
                                self.check_candidate(
                                    &c.attrs,
                                    &c.ident,
                                    c,
                                    Some(&trait_name),
                                    inline_mods,
                                    tr_test,
                                );
                            }
                            syn::TraitItem::Type(t) => {
                                self.check_candidate(
                                    &t.attrs,
                                    &t.ident,
                                    t,
                                    Some(&trait_name),
                                    inline_mods,
                                    tr_test,
                                );
                            }
                            _ => {}
                        }
                    }
                }
                syn::Item::Fn(f) => {
                    self.check_candidate(
                        &f.attrs,
                        &f.sig.ident,
                        f,
                        None,
                        inline_mods,
                        in_cfg_test || is_cfg_test(&f.attrs),
                    );
                }
                syn::Item::Struct(s) => {
                    self.check_candidate(
                        &s.attrs,
                        &s.ident,
                        s,
                        None,
                        inline_mods,
                        in_cfg_test || is_cfg_test(&s.attrs),
                    );
                }
                syn::Item::Enum(e) => {
                    self.check_candidate(
                        &e.attrs,
                        &e.ident,
                        e,
                        None,
                        inline_mods,
                        in_cfg_test || is_cfg_test(&e.attrs),
                    );
                }
                syn::Item::Union(u) => {
                    self.check_candidate(
                        &u.attrs,
                        &u.ident,
                        u,
                        None,
                        inline_mods,
                        in_cfg_test || is_cfg_test(&u.attrs),
                    );
                }
                syn::Item::Type(t) => {
                    self.check_candidate(
                        &t.attrs,
                        &t.ident,
                        t,
                        None,
                        inline_mods,
                        in_cfg_test || is_cfg_test(&t.attrs),
                    );
                }
                syn::Item::Const(c) => {
                    self.check_candidate(
                        &c.attrs,
                        &c.ident,
                        c,
                        None,
                        inline_mods,
                        in_cfg_test || is_cfg_test(&c.attrs),
                    );
                }
                syn::Item::Static(s) => {
                    self.check_candidate(
                        &s.attrs,
                        &s.ident,
                        s,
                        None,
                        inline_mods,
                        in_cfg_test || is_cfg_test(&s.attrs),
                    );
                }
                _ => {}
            }
        }
    }

    fn check_candidate(
        &mut self,
        attrs: &[syn::Attribute],
        ident: &syn::Ident,
        item: &impl ToTokens,
        parent: Option<&str>,
        inline_mods: &[String],
        in_test: bool,
    ) {
        let has_note = attrs.iter().any(|a| parse_note_arg(a).is_some());
        let doc_attrs: Vec<&syn::Attribute> = attrs
            .iter()
            .filter(|a| a.style == syn::AttrStyle::Outer && a.path().is_ident("doc"))
            .collect();

        if doc_attrs.is_empty() && !has_note {
            return;
        }

        let item_display = match parent {
            Some(p) => format!("{p}::{ident}"),
            None => ident.to_string(),
        };

        let first_line = doc_attrs
            .first()
            .map(|a| a.pound_token.span.start().line)
            .unwrap_or_else(|| item_line(attrs, item));

        if has_note {
            self.skipped.push(SkippedDoc {
                file: self.rel_path.to_string(),
                line: first_line,
                item: item_display,
                reason: "already has #[note]".to_string(),
            });
            return;
        }

        if in_test {
            self.skipped.push(SkippedDoc {
                file: self.rel_path.to_string(),
                line: first_line,
                item: item_display,
                reason: "inside #[cfg(test)] module".to_string(),
            });
            return;
        }

        let mut is_plain_triple_slash = true;
        let mut doc_line_indices = Vec::new();
        for attr in &doc_attrs {
            let start_line = attr.pound_token.span.start().line;
            let end_line = attr.pound_token.span.end().line;
            if start_line == 0 || start_line > self.lines.len() || start_line != end_line {
                is_plain_triple_slash = false;
                break;
            }
            let src_line = &self.lines[start_line - 1];
            let trimmed = src_line.trim_start();
            if !trimmed.starts_with("///") || trimmed.starts_with("////") {
                is_plain_triple_slash = false;
                break;
            }
            doc_line_indices.push(start_line - 1);
        }

        if is_plain_triple_slash && doc_line_indices.windows(2).any(|w| w[1] != w[0] + 1) {
            self.skipped.push(SkippedDoc {
                file: self.rel_path.to_string(),
                line: first_line,
                item: item_display,
                reason: "the /// lines are not contiguous".to_string(),
            });
            return;
        }

        if !is_plain_triple_slash {
            self.skipped.push(SkippedDoc {
                file: self.rel_path.to_string(),
                line: first_line,
                item: item_display,
                reason: "doc attribute is not a /// line comment".to_string(),
            });
            return;
        }

        let mut body_lines = Vec::new();
        for &idx in &doc_line_indices {
            let src_line = &self.lines[idx];
            let trimmed = src_line.trim_start();
            let s = trimmed.strip_prefix("///").unwrap_or(trimmed);
            let line_text = if let Some(rest) = s.strip_prefix(' ') {
                rest
            } else {
                s
            };
            body_lines.push(line_text);
        }
        let body = body_lines.join("\n");
        let body = body.trim().to_string();

        let tokens = item.to_token_stream();
        let facts = crawrad_note_format::rustdoc::item_facts(&tokens);
        let problems = crawrad_note_format::rustdoc::doc_problems(&body, &facts);
        if !problems.is_empty() && !self.all {
            self.skipped.push(SkippedDoc {
                file: self.rel_path.to_string(),
                line: first_line,
                item: item_display,
                reason: problems.join("; "),
            });
            return;
        }

        let id = build_note_id(
            self.file_segments,
            inline_mods,
            parent,
            &ident.to_string(),
            self.taken_ids,
        );

        let Some(&first_idx) = doc_line_indices.first() else {
            return;
        };
        let Some(&last_idx) = doc_line_indices.last() else {
            return;
        };
        let first_src_line = &self.lines[first_idx];
        let indent =
            first_src_line[..first_src_line.len() - first_src_line.trim_start().len()].to_string();
        let signature = crawrad_note_format::item_signature(&tokens);

        self.candidates.push(ExtractCandidate {
            id,
            item_display,
            signature,
            body,
            first_doc_idx: first_idx,
            last_doc_idx: last_idx,
            indent,
            inline_mods: inline_mods.to_vec(),
        });
    }
}

struct ModuleFile {
    file_path: PathBuf,
    rel_path: String,
    mod_segments: Vec<String>,
}

fn path_attr(attrs: &[syn::Attribute]) -> Option<String> {
    for a in attrs {
        if a.path().is_ident("path")
            && let syn::Meta::NameValue(nv) = &a.meta
            && let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) = &nv.value
        {
            return Some(s.value());
        }
    }
    None
}

fn resolve_mod_file(dir: &Path, m: &syn::ItemMod) -> Option<PathBuf> {
    if let Some(path_val) = path_attr(&m.attrs) {
        let p = dir.join(path_val);
        if p.is_file() {
            return Some(p);
        }
    }
    let child_dir = dir.join(m.ident.to_string());
    [
        dir.join(format!("{}.rs", m.ident)),
        child_dir.join("mod.rs"),
    ]
    .into_iter()
    .find(|p| p.is_file())
}

fn collect_module_files(
    root: &Path,
    file_path: &Path,
    dir: &Path,
    mod_segments: &[String],
    visited: &mut std::collections::BTreeSet<PathBuf>,
    out: &mut Vec<ModuleFile>,
) -> Result<(), String> {
    let norm = file_path
        .canonicalize()
        .unwrap_or_else(|_| file_path.to_path_buf());
    if !visited.insert(norm) {
        return Ok(());
    }

    let content = std::fs::read_to_string(file_path)
        .map_err(|e| format!("cannot read {}: {e}", file_path.display()))?;
    let syn_file =
        syn::parse_file(&content).map_err(|e| format!("{}: {e}", file_path.display()))?;

    let rel_path = file_path
        .strip_prefix(root)
        .unwrap_or(file_path)
        .to_string_lossy()
        .replace('\\', "/");

    out.push(ModuleFile {
        file_path: file_path.to_path_buf(),
        rel_path,
        mod_segments: mod_segments.to_vec(),
    });

    collect_submodules_from_items(root, &syn_file.items, dir, mod_segments, visited, out)
}

fn collect_submodules_from_items(
    root: &Path,
    items: &[syn::Item],
    dir: &Path,
    mod_segments: &[String],
    visited: &mut std::collections::BTreeSet<PathBuf>,
    out: &mut Vec<ModuleFile>,
) -> Result<(), String> {
    for item in items {
        if let syn::Item::Mod(m) = item {
            if is_cfg_test(&m.attrs) {
                continue;
            }
            if let Some((_, inner_items)) = &m.content {
                let child_dir = dir.join(m.ident.to_string());
                let mut child_segments = mod_segments.to_vec();
                child_segments.push(m.ident.to_string());
                collect_submodules_from_items(
                    root,
                    inner_items,
                    &child_dir,
                    &child_segments,
                    visited,
                    out,
                )?;
            } else if let Some(child_file) = resolve_mod_file(dir, m) {
                let child_dir = if child_file.file_name().is_some_and(|f| f == "mod.rs") {
                    child_file.parent().unwrap_or(dir).to_path_buf()
                } else {
                    child_file.parent().unwrap_or(dir).join(m.ident.to_string())
                };
                let mut child_segments = mod_segments.to_vec();
                child_segments.push(m.ident.to_string());
                collect_module_files(root, &child_file, &child_dir, &child_segments, visited, out)?;
            }
        }
    }
    Ok(())
}

fn build_note_id(
    file_segments: &[String],
    inline_mods: &[String],
    parent_type: Option<&str>,
    item_name: &str,
    taken_ids: &mut std::collections::BTreeSet<String>,
) -> String {
    let mut parts: Vec<String> = Vec::new();
    for seg in file_segments {
        parts.push(seg.clone());
    }
    for m in inline_mods {
        parts.push(m.clone());
    }
    if let Some(parent) = parent_type {
        parts.push(parent.to_string());
    }
    parts.push(item_name.to_string());

    let raw = parts.join("-");
    // A type such as `[u8; 4]` brings brackets and `;` along: anything outside
    // `validate_id`'s alphabet becomes a separator, and separators never repeat.
    let base = raw
        .to_ascii_lowercase()
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    if !taken_ids.contains(&base) {
        taken_ids.insert(base.clone());
        base
    } else {
        let mut counter = 2;
        loop {
            let candidate = format!("{base}-{counter}");
            if !taken_ids.contains(&candidate) {
                taken_ids.insert(candidate.clone());
                return candidate;
            }
            counter += 1;
        }
    }
}

fn type_ident(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(tp) => tp
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default(),
        syn::Type::Group(g) => type_ident(&g.elem),
        syn::Type::Reference(r) => type_ident(&r.elem),
        _ => quote::quote!(#ty).to_string().replace(' ', ""),
    }
}

fn has_use_note_at_items(items: &[syn::Item]) -> bool {
    items.iter().any(|item| {
        if let syn::Item::Use(u) = item {
            is_use_crawrad_note(u)
        } else {
            false
        }
    })
}

fn is_use_crawrad_note(u: &syn::ItemUse) -> bool {
    let s = quote::quote!(#u).to_string();
    let compact: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    compact == "userrad_note::note;" || compact == "use::crawrad_note::note;"
}

/// Every inline module at `path`: cfg twins share one.
fn find_inline_mods<'a>(items: &'a [syn::Item], path: &[String]) -> Vec<&'a syn::ItemMod> {
    let Some((target, rest)) = path.split_first() else {
        return Vec::new();
    };
    let mut found = Vec::new();
    for item in items {
        if let syn::Item::Mod(m) = item
            && m.ident == target
        {
            if rest.is_empty() {
                found.push(m);
            } else if let Some((_, inner)) = &m.content {
                found.extend(find_inline_mods(inner, rest));
            }
        }
    }
    found
}

/// Whether a module's own items (not its nested modules) carry a `#[note]`.
fn carries_note(items: &[syn::Item]) -> bool {
    struct NoteFinder(bool);
    impl<'ast> Visit<'ast> for NoteFinder {
        fn visit_attribute(&mut self, attr: &'ast syn::Attribute) {
            self.0 |= parse_note_arg(attr).is_some();
        }
        fn visit_item_mod(&mut self, _: &'ast syn::ItemMod) {}
    }
    let mut finder = NoteFinder(false);
    for item in items {
        finder.visit_item(item);
    }
    finder.0
}

fn insert_dependency(cargo_text: &str, dep_line: &str) -> String {
    let newline = if cargo_text.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let has_final_newline = cargo_text.ends_with("\r\n") || cargo_text.ends_with('\n');
    let mut lines: Vec<String> = cargo_text.lines().map(String::from).collect();

    let dep_idx = lines.iter().position(|l| l.trim() == "[dependencies]");
    if let Some(idx) = dep_idx {
        lines.insert(idx + 1, dep_line.to_string());
    } else {
        if lines.last().is_some_and(|l| !l.is_empty()) {
            lines.push(String::new());
        }
        lines.push("[dependencies]".to_string());
        lines.push(dep_line.to_string());
    }

    let mut new_text = lines.join(newline);
    if has_final_newline && !new_text.is_empty() {
        new_text.push_str(newline);
    }
    new_text
}

fn has_workspace_dep_crawrad_note(root: &Path) -> bool {
    let mut curr = if root.is_absolute() {
        root.to_path_buf()
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(root),
            Err(_) => root.to_path_buf(),
        }
    };
    if let Ok(canon) = curr.canonicalize() {
        curr = canon;
    }

    loop {
        let manifest = curr.join("Cargo.toml");
        if manifest.is_file()
            && let Ok(text) = std::fs::read_to_string(&manifest)
            && let Ok(val) = toml::from_str::<toml::Value>(&text)
            && let Some(ws) = val.get("workspace")
        {
            let has_crawrad_note = ws
                .get("dependencies")
                .and_then(|d| d.get("crawrad-note"))
                .is_some();
            return has_crawrad_note;
        }
        if !curr.pop() {
            break;
        }
    }
    false
}

#[note("note-broken-links-from-cargo-json")]
pub fn broken_links_from_cargo_json(stdout: &str, root: &Path) -> Result<Vec<BrokenLink>, String> {
    let anchors = collect_anchors(root)?;
    let mut links = Vec::new();

    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Ok(val) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if !val.is_object() {
            continue;
        }
        if val.get("reason").and_then(|r| r.as_str()) != Some("compiler-message") {
            continue;
        }
        let Some(msg_obj) = val.get("message").and_then(|m| m.as_object()) else {
            continue;
        };
        let code = msg_obj
            .get("code")
            .and_then(|c| c.get("code"))
            .and_then(|c| c.as_str());
        if !matches!(
            code,
            Some("rustdoc::broken_intra_doc_links" | "rustdoc::private_intra_doc_links")
        ) {
            continue;
        }

        let message = msg_obj
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or_default()
            .to_string();

        let spans = msg_obj.get("spans").and_then(|s| s.as_array());
        let primary_span = spans.and_then(|arr| {
            arr.iter()
                .find(|s| s.get("is_primary").and_then(|b| b.as_bool()) == Some(true))
                .or_else(|| arr.first())
        });
        let Some(span) = primary_span else {
            continue;
        };
        let Some(raw_file) = span.get("file_name").and_then(|f| f.as_str()) else {
            continue;
        };
        let line_num = span.get("line_start").and_then(|l| l.as_u64()).unwrap_or(0) as usize;

        let file = normalize_file_path(raw_file, root);
        let note = anchors
            .iter()
            .find(|a| a.file == file && a.line == line_num)
            .map(|a| a.id.clone());

        links.push(BrokenLink {
            file,
            line: line_num,
            note,
            message,
        });
    }

    links.sort_by(|a, b| {
        a.file
            .cmp(&b.file)
            .then_with(|| a.line.cmp(&b.line))
            .then_with(|| a.note.cmp(&b.note))
            .then_with(|| a.message.cmp(&b.message))
    });
    links.dedup();

    Ok(links)
}

#[note("note-broken-links")]
pub fn broken_links(root: &Path) -> Result<Vec<BrokenLink>, String> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(cargo);
    // The IDE may run this command: without `CREATE_NO_WINDOW` the cargo
    // child opens a console window next to it. This crate does not depend on
    // `crawrad-os` (its `no_console` needs the adapter's full feature), so it
    // carries the flag itself, and the crawrad-os process gate holds it to that.
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000);
    }
    let output = command
        .args(["doc", "--no-deps", "--lib", "--message-format=json"])
        .current_dir(root)
        .stderr(Stdio::null())
        .output()
        .map_err(|e| format!("cannot run cargo doc: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let links = broken_links_from_cargo_json(&stdout, root)?;

    if !output.status.success() && links.is_empty() {
        let last_rendered = last_compiler_message_rendered(&stdout);
        return Err(last_rendered.unwrap_or_else(|| "cargo doc failed".to_string()));
    }

    Ok(links)
}

fn last_compiler_message_rendered(stdout: &str) -> Option<String> {
    let mut last = None;
    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(line)
            && val.get("reason").and_then(|r| r.as_str()) == Some("compiler-message")
            && let Some(rendered) = val
                .get("message")
                .and_then(|m| m.get("rendered"))
                .and_then(|r| r.as_str())
        {
            last = Some(rendered.trim_end().to_string());
        }
    }
    last
}

fn normalize_file_path(file_name: &str, root: &Path) -> String {
    let p = Path::new(file_name);
    let rel = if p.is_absolute() {
        if let Ok(r) = p.strip_prefix(root) {
            r
        } else if let Ok(canonical_root) = root.canonicalize()
            && let Ok(r) = p.strip_prefix(&canonical_root)
        {
            r
        } else {
            p
        }
    } else {
        p
    };
    let rel = rel.strip_prefix(".").unwrap_or(rel);
    rel.to_string_lossy().replace('\\', "/")
}

fn read_notes(root: &Path) -> Result<BTreeMap<String, NoteFile>, String> {
    let notes_dir = root.join(".crawrad").join("notes");
    let mut notes_by_id = BTreeMap::new();
    if notes_dir.is_dir() {
        let mut note_files = Vec::new();
        let entries = std::fs::read_dir(&notes_dir)
            .map_err(|e| format!("cannot read {}: {e}", notes_dir.display()))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("{}: {e}", notes_dir.display()))?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md") {
                note_files.push(path);
            }
        }
        note_files.sort();

        for path in note_files {
            let text = std::fs::read_to_string(&path)
                .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
            let note = NoteFile::parse(&text)
                .map_err(|e| format!("cannot parse {}: {e}", path.display()))?;
            notes_by_id.insert(note.id.clone(), note);
        }
    }
    Ok(notes_by_id)
}

fn collect_anchors(root: &Path) -> Result<Vec<FoundAnchor>, String> {
    let mut src_files = Vec::new();
    for dir in ["src", "tests", "examples", "benches"] {
        let dir = root.join(dir);
        if dir.is_dir() {
            collect_rs_files(&dir, &mut src_files)?;
        }
    }
    src_files.sort();
    let mut anchors = Vec::new();
    for path in src_files {
        let text = std::fs::read_to_string(&path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let syn_file = syn::parse_file(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        let rel = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");

        let mut visitor = AnchorVisitor {
            file_rel_path: &rel,
            anchors: Vec::new(),
        };
        visitor.visit_file(&syn_file);
        anchors.extend(visitor.anchors);
    }
    Ok(anchors)
}

#[note("note-rustdoc-lines")]
fn rustdoc_lines(
    note: Option<&NoteFile>,
    line: Option<&str>,
    id: &str,
) -> Result<Vec<String>, String> {
    let note = note.ok_or("no note file")?;
    if note.kind != NoteKind::Doc {
        return Err(format!(
            "note is {}; only doc notes inline",
            note.kind.as_str()
        ));
    }
    let line = line
        .filter(|l| is_attribute_alone_on_line(l, id))
        .ok_or("attribute shares its line with code")?;
    let indent = &line[..line.len() - line.trim_start().len()];
    let body = note.body.trim_end();
    Ok(body
        .lines()
        .map(|l| {
            if l.is_empty() {
                format!("{indent}///")
            } else {
                format!("{indent}/// {l}")
            }
        })
        .collect())
}

fn is_attribute_alone_on_line(line: &str, expected_id: &str) -> bool {
    syn::Attribute::parse_outer
        .parse_str(line.trim())
        .is_ok_and(|attrs| {
            matches!(attrs.as_slice(), [attr] if parse_note_arg(attr).as_deref() == Some(expected_id))
        })
}

#[note("note-has-any-note-attr")]
fn has_any_note_attr(lines: &[String]) -> bool {
    struct NoteFinder(bool);
    impl<'ast> Visit<'ast> for NoteFinder {
        fn visit_attribute(&mut self, attr: &'ast syn::Attribute) {
            self.0 |= parse_note_arg(attr).is_some();
        }
    }
    syn::parse_file(&lines.join("\n")).map_or(true, |file| {
        let mut finder = NoteFinder(false);
        finder.visit_file(&file);
        finder.0
    })
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries =
        std::fs::read_dir(dir).map_err(|e| format!("cannot read {}: {e}", dir.display()))?;
    for e in entries {
        let e = e.map_err(|e| format!("{}: {e}", dir.display()))?;
        let p = e.path();
        if p.is_dir() {
            collect_rs_files(&p, out)?;
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
    Ok(())
}

struct AnchorVisitor<'a> {
    file_rel_path: &'a str,
    anchors: Vec<FoundAnchor>,
}

impl AnchorVisitor<'_> {
    #[note("note-anchorvisitor-record")]
    fn record(&mut self, attrs: &[syn::Attribute], item: &impl ToTokens) {
        let tokens = item.to_token_stream();
        for attr in attrs {
            if let Some(id) = parse_note_arg(attr) {
                self.anchors.push(FoundAnchor {
                    id,
                    file: self.file_rel_path.to_string(),
                    line: attr.pound_token.span.start().line,
                    signature: crawrad_note_format::item_signature(&tokens),
                    facts: crawrad_note_format::rustdoc::item_facts(&tokens),
                });
            }
        }
    }
}

impl<'ast> Visit<'ast> for AnchorVisitor<'_> {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if let Some(attrs) = item_attrs(item) {
            self.record(attrs, item);
        }
        syn::visit::visit_item(self, item);
    }

    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        self.record(impl_item_attrs(item), item);
        syn::visit::visit_impl_item(self, item);
    }

    fn visit_trait_item(&mut self, item: &'ast syn::TraitItem) {
        self.record(trait_item_attrs(item), item);
        syn::visit::visit_trait_item(self, item);
    }
}

fn parse_note_arg(attr: &syn::Attribute) -> Option<String> {
    if attr.style != syn::AttrStyle::Outer {
        return None;
    }
    let path = attr.path();
    let is_note = if let Some(ident) = path.get_ident() {
        ident == "note"
    } else if let Some(last) = path.segments.last() {
        last.ident == "note"
    } else {
        false
    };
    if !is_note {
        return None;
    }

    match &attr.meta {
        syn::Meta::List(list) => {
            let parsed: Result<syn::LitStr, _> = syn::parse::Parser::parse2(
                |input: syn::parse::ParseStream| {
                    let lit: syn::LitStr = input.parse()?;
                    if !input.is_empty() {
                        return Err(input.error("expected one string literal"));
                    }
                    Ok(lit)
                },
                list.tokens.clone(),
            );
            parsed.ok().map(|lit| lit.value())
        }
        _ => None,
    }
}

fn item_attrs(item: &syn::Item) -> Option<&[syn::Attribute]> {
    match item {
        syn::Item::Const(i) => Some(&i.attrs),
        syn::Item::Enum(i) => Some(&i.attrs),
        syn::Item::Fn(i) => Some(&i.attrs),
        syn::Item::ForeignMod(i) => Some(&i.attrs),
        syn::Item::Impl(i) => Some(&i.attrs),
        syn::Item::Macro(i) => Some(&i.attrs),
        syn::Item::Mod(i) => Some(&i.attrs),
        syn::Item::Static(i) => Some(&i.attrs),
        syn::Item::Struct(i) => Some(&i.attrs),
        syn::Item::Trait(i) => Some(&i.attrs),
        syn::Item::TraitAlias(i) => Some(&i.attrs),
        syn::Item::Type(i) => Some(&i.attrs),
        syn::Item::Union(i) => Some(&i.attrs),
        syn::Item::Use(i) => Some(&i.attrs),
        _ => None,
    }
}

fn impl_item_attrs(item: &syn::ImplItem) -> &[syn::Attribute] {
    match item {
        syn::ImplItem::Const(i) => &i.attrs,
        syn::ImplItem::Fn(i) => &i.attrs,
        syn::ImplItem::Type(i) => &i.attrs,
        syn::ImplItem::Macro(i) => &i.attrs,
        _ => &[],
    }
}

fn trait_item_attrs(item: &syn::TraitItem) -> &[syn::Attribute] {
    match item {
        syn::TraitItem::Const(i) => &i.attrs,
        syn::TraitItem::Fn(i) => &i.attrs,
        syn::TraitItem::Type(i) => &i.attrs,
        syn::TraitItem::Macro(i) => &i.attrs,
        _ => &[],
    }
}

#[note("note-missing-docs")]
pub fn missing_docs(root: &Path) -> Result<Vec<MissingDoc>, String> {
    let manifest = root.join("Cargo.toml");
    let text = std::fs::read_to_string(&manifest)
        .map_err(|e| format!("cannot read {}: {e}", manifest.display()))?;
    let crate_name = toml::from_str::<toml::Value>(&text)
        .ok()
        .and_then(|v| v.get("package")?.get("name")?.as_str().map(str::to_string))
        .unwrap_or_else(|| {
            root.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        });
    let crate_root = ["src/lib.rs", "src/main.rs"]
        .into_iter()
        .map(|f| root.join(f))
        .find(|p| p.is_file())
        .ok_or_else(|| format!("{}: no src/lib.rs or src/main.rs", root.display()))?;

    let file = parse_source(&crate_root)?;
    let mut walk = MissingDocsWalk {
        root,
        file: relative_to(root, &crate_root),
        out: Vec::new(),
    };
    if !is_documented(&file.attrs) {
        walk.out.push(MissingDoc {
            file: walk.file.clone(),
            line: 1,
            kind: "crate",
            name: crate_name,
        });
    }
    walk.items(&file.items, &root.join("src"))?;
    walk.out.sort();
    Ok(walk.out)
}

#[note("note-missingdocswalk")]
struct MissingDocsWalk<'a> {
    root: &'a Path,
    file: PathBuf,
    out: Vec<MissingDoc>,
}

impl MissingDocsWalk<'_> {
    #[note("note-missingdocswalk-check")]
    fn check(
        &mut self,
        attrs: &[syn::Attribute],
        node: &impl ToTokens,
        kind: &'static str,
        name: &syn::Ident,
    ) {
        if !is_documented(attrs) && !is_cfg_test(attrs) {
            self.out.push(MissingDoc {
                file: self.file.clone(),
                line: item_line(attrs, node),
                kind,
                name: name.to_string(),
            });
        }
    }

    #[note("note-missingdocswalk-items")]
    fn items(&mut self, items: &[syn::Item], dir: &Path) -> Result<(), String> {
        for item in items {
            match item {
                syn::Item::Fn(i) if is_public(&i.vis) => {
                    self.check(&i.attrs, i, "fn", &i.sig.ident)
                }
                syn::Item::Struct(i) if is_public(&i.vis) && !is_cfg_test(&i.attrs) => {
                    self.check(&i.attrs, i, "struct", &i.ident);
                    self.fields(&i.fields);
                }
                syn::Item::Union(i) if is_public(&i.vis) && !is_cfg_test(&i.attrs) => {
                    self.check(&i.attrs, i, "union", &i.ident);
                    self.fields(&i.fields.named);
                }
                syn::Item::Enum(i) if is_public(&i.vis) && !is_cfg_test(&i.attrs) => {
                    self.check(&i.attrs, i, "enum", &i.ident);
                    for v in &i.variants {
                        self.check(&v.attrs, v, "variant", &v.ident);
                    }
                }
                syn::Item::Trait(i) if is_public(&i.vis) && !is_cfg_test(&i.attrs) => {
                    self.check(&i.attrs, i, "trait", &i.ident);
                    for t in &i.items {
                        match t {
                            syn::TraitItem::Fn(m) => {
                                self.check(&m.attrs, m, "method", &m.sig.ident)
                            }
                            syn::TraitItem::Const(c) => {
                                self.check(&c.attrs, c, "assoc const", &c.ident)
                            }
                            syn::TraitItem::Type(t) => {
                                self.check(&t.attrs, t, "assoc type", &t.ident)
                            }
                            _ => {}
                        }
                    }
                }
                syn::Item::Impl(i) if i.trait_.is_none() && !is_cfg_test(&i.attrs) => {
                    for m in &i.items {
                        match m {
                            syn::ImplItem::Fn(m) if is_public(&m.vis) => {
                                self.check(&m.attrs, m, "method", &m.sig.ident)
                            }
                            syn::ImplItem::Const(c) if is_public(&c.vis) => {
                                self.check(&c.attrs, c, "assoc const", &c.ident)
                            }
                            syn::ImplItem::Type(t) if is_public(&t.vis) => {
                                self.check(&t.attrs, t, "assoc type", &t.ident)
                            }
                            _ => {}
                        }
                    }
                }
                syn::Item::Type(i) if is_public(&i.vis) => {
                    self.check(&i.attrs, i, "type", &i.ident)
                }
                syn::Item::Const(i) if is_public(&i.vis) => {
                    self.check(&i.attrs, i, "const", &i.ident)
                }
                syn::Item::Static(i) if is_public(&i.vis) => {
                    self.check(&i.attrs, i, "static", &i.ident)
                }
                syn::Item::Macro(i)
                    if i.attrs.iter().any(|a| a.path().is_ident("macro_export"))
                        && let Some(ident) = &i.ident =>
                {
                    self.check(&i.attrs, i, "macro", ident)
                }
                syn::Item::Mod(i) if is_public(&i.vis) && !is_cfg_test(&i.attrs) => {
                    self.module(i, dir)?
                }
                _ => {}
            }
        }
        Ok(())
    }

    #[note("note-missingdocswalk-fields")]
    fn fields<'f>(&mut self, fields: impl IntoIterator<Item = &'f syn::Field>) {
        for f in fields {
            if is_public(&f.vis)
                && let Some(ident) = &f.ident
            {
                self.check(&f.attrs, f, "field", ident);
            }
        }
    }

    #[note("note-missingdocswalk-module")]
    fn module(&mut self, m: &syn::ItemMod, dir: &Path) -> Result<(), String> {
        let child_dir = dir.join(m.ident.to_string());
        if let Some((_, items)) = &m.content {
            self.check(&m.attrs, m, "mod", &m.ident);
            return self.items(items, &child_dir);
        }
        let Some(path) = [
            dir.join(format!("{}.rs", m.ident)),
            child_dir.join("mod.rs"),
        ]
        .into_iter()
        .find(|p| p.is_file()) else {
            self.check(&m.attrs, m, "mod", &m.ident);
            return Ok(());
        };
        let file = parse_source(&path)?;
        if !is_documented(&file.attrs) {
            self.check(&m.attrs, m, "mod", &m.ident);
        }
        let parent = std::mem::replace(&mut self.file, relative_to(self.root, &path));
        let walked = self.items(&file.items, &child_dir);
        self.file = parent;
        walked
    }
}

fn parse_source(path: &Path) -> Result<syn::File, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    syn::parse_file(&text).map_err(|e| format!("{}: {e}", path.display()))
}

fn relative_to(root: &Path, path: &Path) -> PathBuf {
    let rel = path.strip_prefix(root).unwrap_or(path);
    PathBuf::from(rel.to_string_lossy().replace('\\', "/"))
}

#[note("note-is-documented")]
fn is_documented(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if parse_note_arg(attr).is_some() {
            return true;
        }
        if !attr.path().is_ident("doc") {
            return false;
        }
        match &attr.meta {
            syn::Meta::NameValue(nv) => !matches!(
                &nv.value,
                syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. })
                    if s.value().trim().is_empty()
            ),
            syn::Meta::List(_) => attr.parse_args::<syn::Ident>().is_ok_and(|i| i == "hidden"),
            syn::Meta::Path(_) => false,
        }
    })
}

fn is_cfg_test(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|a| {
        a.path().is_ident("cfg") && a.parse_args::<syn::Ident>().is_ok_and(|i| i == "test")
    })
}

fn is_public(vis: &syn::Visibility) -> bool {
    matches!(vis, syn::Visibility::Public(_))
}

#[note("note-item-line")]
fn item_line(attrs: &[syn::Attribute], item: &impl ToTokens) -> usize {
    match attrs.first() {
        Some(attr) => attr.pound_token.span.start().line,
        None => item
            .to_token_stream()
            .into_iter()
            .next()
            .map_or(1, |t| t.span().start().line),
    }
}
