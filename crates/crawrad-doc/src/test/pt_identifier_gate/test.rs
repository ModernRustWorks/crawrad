use super::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::process::Command;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

/// Parses git diff output to get a mapping from file path (e.g. "crates/crawrad-actions/src/lib.rs")
/// to a set of added/modified line numbers in the new file.
pub fn parse_git_diff_added_lines(diff_text: &str) -> HashMap<String, HashSet<usize>> {
    let mut map: HashMap<String, HashSet<usize>> = HashMap::new();
    let mut current_file: Option<String> = None;

    for line in diff_text.lines() {
        if let Some(target) = line.strip_prefix("+++ b/") {
            current_file = Some(target.trim().replace('\\', "/"));
        } else if line.starts_with("+++ /dev/null") {
            current_file = None;
        } else if let Some(ref file) = current_file
            && line.starts_with("@@ ")
            && let Some(plus_idx) = line.find('+')
        {
            let after_plus = &line[plus_idx + 1..];
            let chunk = after_plus.split_whitespace().next().unwrap_or("");
            let parts: Vec<&str> = chunk.split(',').collect();
            if let Ok(start) = parts[0].parse::<usize>() {
                let count = if parts.len() > 1 {
                    parts[1].parse::<usize>().unwrap_or(1)
                } else {
                    1
                };
                if count > 0 {
                    let entry = map.entry(file.clone()).or_default();
                    for line_num in start..(start + count) {
                        entry.insert(line_num);
                    }
                }
            }
        }
    }

    map
}

/// Identifies Portuguese definitions added or modified in the current branch compared to main.
pub fn find_added_pt_definitions(root_dir: &Path, pt_defs: &[PtDefinition]) -> Vec<PtDefinition> {
    let targets = ["origin/main", "main", "HEAD~1", "HEAD"];
    let mut diff_output = None;

    for target in targets {
        let mut cmd = Command::new("git");
        cmd.arg("-C")
            .arg(root_dir)
            .arg("diff")
            .arg("-U0")
            .arg(target);
        #[cfg(windows)]
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
        if let Ok(output) = cmd.output()
            && output.status.success()
        {
            let text = String::from_utf8_lossy(&output.stdout).to_string();
            if !text.is_empty() {
                diff_output = Some(text);
                break;
            }
        }
    }

    let Some(diff_text) = diff_output else {
        return Vec::new();
    };

    let added_lines = parse_git_diff_added_lines(&diff_text);

    pt_defs
        .iter()
        .filter(|d| {
            let normalized_file = d.file.replace('\\', "/");
            added_lines
                .get(&normalized_file)
                .is_some_and(|lines| lines.contains(&d.line))
        })
        .cloned()
        .collect()
}

/// Evaluates the Portuguese identifier and path ratchet gate.
pub fn check_pt_identifier_ratchet(
    root_dir: &Path,
    max_identifiers: usize,
    max_paths: usize,
) -> Result<(usize, usize), String> {
    let (pt_defs, pt_paths) = scan_workspace_pt_gate(root_dir);

    let actual_idents = pt_defs.len();
    let actual_paths = pt_paths.len();

    if actual_idents > max_identifiers {
        let added = find_added_pt_definitions(root_dir, &pt_defs);
        let items_to_show = if !added.is_empty() { &added } else { &pt_defs };

        let sample: Vec<String> = items_to_show
            .iter()
            .take(20)
            .map(|d| {
                format!(
                    "{}:{}: {} `{}` (matched `{}`)",
                    d.file, d.line, d.kind, d.identifier, d.matched_pt_word
                )
            })
            .collect();

        let header = if !added.is_empty() {
            format!(
                "Portuguese identifiers ADDED in this PR ({} found):\n{}",
                added.len(),
                sample.join("\n")
            )
        } else {
            format!("First findings in workspace:\n{}", sample.join("\n"))
        };

        return Err(format!(
            "[IDENTIFIER GATE - ADR 0030] Portuguese identifiers violated!\n\
             Current: {actual_idents} (maximum: {max_identifiers})\n\
             {header}"
        ));
    } else if actual_idents < max_identifiers {
        return Err(format!(
            "[IDENTIFIER GATE - ADR 0030] The identifier ratchet must be tightened!\n\
             Current: {actual_idents} (previous ratchet: {max_identifiers})\n\
             Update MAX_PT_IDENTIFIERS from {max_identifiers} to {actual_idents} in pt_identifier_gate.rs in this PR."
        ));
    }

    if actual_paths > max_paths {
        return Err(format!(
            "[PATH GATE - ADR 0030] Portuguese paths violated!\n\
             Current: {actual_paths} (maximum: {max_paths})\n\
             Found paths: {:?}",
            pt_paths
        ));
    } else if actual_paths < max_paths {
        return Err(format!(
            "[PATH GATE - ADR 0030] The path ratchet must be tightened!\n\
             Current: {actual_paths} (previous ratchet: {max_paths})\n\
             Update MAX_PT_PATHS from {max_paths} to {actual_paths} in pt_identifier_gate.rs in this PR."
        ));
    }

    Ok((actual_idents, actual_paths))
}

fn root_dir() -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("cargo workspace root")
        .to_path_buf()
}

#[test]
fn pt_identifier_gate_ratchet() {
    let r = root_dir();
    let res = check_pt_identifier_ratchet(&r, MAX_PT_IDENTIFIERS, MAX_PT_PATHS);
    assert!(
        res.is_ok(),
        "Identifier/path gate failed: {}",
        res.unwrap_err()
    );
}

#[test]
fn mutation_proof_pt_identifier_fails_on_abrir_arquivo() {
    let temp_dir = std::env::temp_dir().join("crawrad_pt_gate_test");
    let crates_dir = temp_dir.join("crates").join("dummy");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&crates_dir).unwrap();

    let dummy_rs = crates_dir.join("lib.rs");
    fs::write(&dummy_rs, "pub fn abrir_arquivo() {}\n").unwrap();

    let res = check_pt_identifier_ratchet(&temp_dir, 0, 0);
    assert!(
        res.is_err(),
        "The gate should have failed in the presence of `fn abrir_arquivo`!"
    );

    let err = res.unwrap_err();
    assert!(
        err.contains("abrir_arquivo"),
        "The gate error must mention `abrir_arquivo`: {err}"
    );

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn pt_words_is_sorted_and_has_no_duplicates() {
    for i in 0..PT_WORDS.len().saturating_sub(1) {
        let a = PT_WORDS[i];
        let b = PT_WORDS[i + 1];
        assert!(
            a < b,
            "PT_WORDS must be strictly sorted and without duplicates: '{a}' >= '{b}' (index {i})"
        );
    }
}

#[test]
fn every_pt_word_is_detected_in_dummy_identifier() {
    for &word in PT_WORDS {
        let dummy = format!("x_{word}");
        let detected = is_pt_identifier(&dummy);
        assert_eq!(
            detected,
            Some(word),
            "Word '{word}' was not detected in dummy identifier '{dummy}'"
        );
    }
}

#[test]
fn the_portadb_product_name_is_not_the_word_porta() {
    assert_eq!(is_pt_identifier("PortaDbBridge"), None);
    assert_eq!(is_pt_identifier("PortaAberta"), Some("porta"));
}
