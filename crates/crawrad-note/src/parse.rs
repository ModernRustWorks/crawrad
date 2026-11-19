use crawrad_note_format::{NoteFile, NoteKind};
use std::path::{Path, PathBuf};

pub(crate) use crawrad_note_format::validate_id;

/// Resolves the file path where a note should be located.
pub(crate) fn note_path(manifest_dir: impl AsRef<Path>, id: &str) -> PathBuf {
    crawrad_note_format::note_path(manifest_dir.as_ref(), id)
}

/// Formats the compile error when a note file is missing (Rule 2).
pub(crate) fn missing_file_error(id: &str, path: &Path) -> String {
    format!("note \"{id}\" not found: expected {}", path.display())
}

/// Parses the contents of a note markdown file, validates its front matter
/// against rules 3, 4, 5, and 6, and returns it (the body trimmed).
pub(crate) fn parse_note(id: &str, text: &str) -> Result<NoteFile, String> {
    let note = NoteFile::parse(text).map_err(|err| format!("note \"{id}\" {err}"))?;

    // Rule 4: kind must be "doc"
    if note.kind != NoteKind::Doc {
        let (article, kind_name) = match note.kind {
            NoteKind::Internal => ("an", "internal"),
            NoteKind::Comment => ("a", "comment"),
            NoteKind::Doc => ("a", "doc"),
        };
        let kind_str = if note.kind_declared {
            note.kind.as_str()
        } else {
            "missing"
        };
        return Err(format!(
            "note \"{id}\" is {article} {kind_name} note (kind: {kind_str}); only kind: doc notes become documentation"
        ));
    }

    // Rule 5: front matter id must equal the attribute id
    if note.id != id {
        return Err(format!(
            "note \"{id}\" front matter id does not match: expected \"{id}\", found \"{}\"",
            note.id
        ));
    }

    // Rule 6: body must not be empty
    if note.body.is_empty() {
        return Err(format!("note \"{id}\" has empty body"));
    }

    Ok(note)
}

#[cfg(test)]
#[path = "test/parse/test.rs"]
mod test;
