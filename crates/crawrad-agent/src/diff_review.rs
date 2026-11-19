//! Management and the unified diff-review gate for code edits by ACP/MCP agents.
//!
//! Every tool that writes code into the project goes through the same gate:
//! - Creating a pending review with the added and removed lines.
//! - Blocking new concurrent edits on the same file while one is pending (`pending-review-busy`).
//! - Strictly blocking saves while a review is pending (never accepts silently).
//! - Accepting produces exactly ONE undo transaction (undone by a single Ctrl+Z).
//! - Rejecting restores the document byte for byte, leaving no trace.
//! - Configurable mode: `WaitForApproval` (default), `ShowAndApply`, `ApplyDirectly`.

use crawrad_note::note;
use serde::{Deserialize, Serialize};
use similar::{ChangeTag, DiffTag, TextDiff};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

#[note("diff-review-diffreviewmode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DiffReviewMode {
    /// Waits for the developer's explicit approval before applying to the buffer/disk (the safe default).
    #[default]
    WaitForApproval,
    /// Applies the change with undo and shows the diff inline for inspection.
    ShowAndApply,
    /// Applies directly to the buffer with undo, keeping no pending state.
    ApplyDirectly,
}

impl DiffReviewMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WaitForApproval => "wait_for_approval",
            Self::ShowAndApply => "show_and_apply",
            Self::ApplyDirectly => "apply_directly",
        }
    }

    pub fn display_label(&self) -> &'static str {
        match self {
            Self::WaitForApproval => "Wait for approval (Default)",
            Self::ShowAndApply => "Show and apply",
            Self::ApplyDirectly => "Apply directly",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "show_and_apply" | "show" => Self::ShowAndApply,
            "apply_directly" | "direct" => Self::ApplyDirectly,
            _ => Self::WaitForApproval,
        }
    }
}

#[note("diff-review-hunkstatus")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum HunkStatus {
    #[default]
    Pending,
    Accepted,
    Rejected,
}

#[note("diff-review-diffhunk")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffHunk {
    pub id: usize,
    pub start_line_old: usize,
    pub end_line_old: usize,
    pub start_line_new: usize,
    pub end_line_new: usize,
    pub old_lines: Vec<String>,
    pub new_lines: Vec<String>,
    pub status: HunkStatus,
}

impl DiffHunk {
    pub fn added_count(&self) -> usize {
        self.new_lines.len()
    }

    pub fn removed_count(&self) -> usize {
        self.old_lines.len()
    }
}

#[note("diff-review-reviewstatus")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ReviewStatus {
    #[default]
    Pending,
    Accepted,
    Rejected,
}

#[note("diff-review-diffreview")]
#[derive(Clone)]
pub struct DiffReview {
    pub id: u64,
    pub file_path: PathBuf,
    pub original_content: String,
    /// The file as it was when the review began: per-change decisions move
    /// `original_content`, never this — undo goes back here.
    pub base_content: String,
    pub proposed_content: String,
    pub predicted_hash: String,
    pub author_tool: String,
    pub hunks: Vec<DiffHunk>,
    pub status: ReviewStatus,
    pub created_at: Instant,
}

impl std::fmt::Debug for DiffReview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DiffReview")
            .field("id", &self.id)
            .field("file_path", &self.file_path)
            .field("predicted_hash", &self.predicted_hash)
            .field("author_tool", &self.author_tool)
            .field("status", &self.status)
            .field("hunks_count", &self.hunks.len())
            .finish()
    }
}

impl DiffReview {
    pub fn new(
        id: u64,
        file_path: impl Into<PathBuf>,
        original_content: impl Into<String>,
        proposed_content: impl Into<String>,
        author_tool: impl Into<String>,
    ) -> Self {
        let path = file_path.into();
        let orig = original_content.into();
        let prop = proposed_content.into();
        let predicted_hash = hex_sha256(prop.as_bytes());
        let hunks = compute_diff_hunks(&orig, &prop);

        Self {
            id,
            file_path: path,
            base_content: orig.clone(),
            original_content: orig,
            proposed_content: prop,
            predicted_hash,
            author_tool: author_tool.into(),
            hunks,
            status: ReviewStatus::Pending,
            created_at: Instant::now(),
        }
    }

    pub fn total_additions(&self) -> usize {
        self.hunks.iter().map(|h| h.added_count()).sum()
    }

    pub fn total_deletions(&self) -> usize {
        self.hunks.iter().map(|h| h.removed_count()).sum()
    }

    pub fn is_pending(&self) -> bool {
        self.status == ReviewStatus::Pending
    }

    /// Asked by an agent, which writes the file itself once allowed.
    pub fn from_agent(&self) -> bool {
        self.author_tool.starts_with("agent.")
    }
}

#[note("diff-review-canonical-path-key")]
pub fn canonical_path_key(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "/")
        .trim_start_matches("./")
        .to_lowercase()
}

#[note("diff-review-diffreviewmanager")]
#[derive(Debug, Default)]
pub struct DiffReviewManager {
    next_id: u64,
    pending: HashMap<String, DiffReview>,
}

impl DiffReviewManager {
    pub fn new() -> Self {
        Self::default()
    }

    #[note("diff-review-diffreviewmanager-has-pending-review")]
    pub fn has_pending_review(&self, path: &Path) -> bool {
        let key = canonical_path_key(path);
        self.pending.get(&key).is_some_and(|r| r.is_pending())
    }

    #[note("diff-review-diffreviewmanager-get-pending-review")]
    pub fn get_pending_review(&self, path: &Path) -> Option<&DiffReview> {
        let key = canonical_path_key(path);
        self.pending.get(&key).filter(|r| r.is_pending())
    }

    #[note("diff-review-diffreviewmanager-any-pending")]
    pub fn any_pending(&self) -> Option<&DiffReview> {
        self.pending.values().find(|r| r.is_pending())
    }

    #[note("diff-review-diffreviewmanager-propose-edit")]
    pub fn propose_edit(
        &mut self,
        path: PathBuf,
        original_content: String,
        proposed_content: String,
        author_tool: &str,
    ) -> Result<DiffReview, String> {
        let key = canonical_path_key(&path);
        if let Some(existing) = self.pending.get(&key)
            && existing.is_pending()
        {
            return Err(format!(
                "File '{}' has a pending diff review awaiting user decision. Please wait for the user to accept or reject before submitting further edits.",
                path.display()
            ));
        }

        self.next_id += 1;
        let review = DiffReview::new(
            self.next_id,
            path,
            original_content,
            proposed_content,
            author_tool,
        );
        self.pending.insert(key, review.clone());
        Ok(review)
    }

    #[note("diff-review-diffreviewmanager-accept")]
    pub fn accept(&mut self, path: &Path) -> Result<String, String> {
        let key = canonical_path_key(path);
        let review = self
            .pending
            .get_mut(&key)
            .ok_or_else(|| format!("No pending diff review for '{}'", path.display()))?;

        if !review.is_pending() {
            return Err("Diff review is not pending".to_string());
        }

        review.status = ReviewStatus::Accepted;
        for hunk in &mut review.hunks {
            hunk.status = HunkStatus::Accepted;
        }

        let proposed = review.proposed_content.clone();
        self.pending.remove(&key);
        Ok(proposed)
    }

    #[note("diff-review-diffreviewmanager-reject")]
    pub fn reject(&mut self, path: &Path) -> Result<String, String> {
        let key = canonical_path_key(path);
        let review = self
            .pending
            .get_mut(&key)
            .ok_or_else(|| format!("No pending diff review for '{}'", path.display()))?;

        if !review.is_pending() {
            return Err("Diff review is not pending".to_string());
        }

        review.status = ReviewStatus::Rejected;
        for hunk in &mut review.hunks {
            hunk.status = HunkStatus::Rejected;
        }

        let original = review.original_content.clone();
        self.pending.remove(&key);
        Ok(original)
    }

    #[note("diff-review-diffreviewmanager-decide-change")]
    pub fn decide_change(&mut self, path: &Path, k: usize, accept: bool) -> Result<bool, String> {
        let review = self
            .pending
            .get_mut(&canonical_path_key(path))
            .filter(|r| r.is_pending())
            .ok_or_else(|| format!("No pending diff review for '{}'", path.display()))?;
        let (original, proposed) = (&review.original_content, &review.proposed_content);
        if accept {
            review.original_content = accept_change(original, proposed, k);
        } else {
            review.proposed_content = reject_change(original, proposed, k);
        }
        review.hunks = compute_diff_hunks(&review.original_content, &review.proposed_content);
        Ok(review.original_content == review.proposed_content)
    }

    #[note("diff-review-diffreviewmanager-clear")]
    pub fn clear(&mut self) {
        self.pending.clear();
    }
}

/// Applies change `k` of the diff between `original` and `proposed` to `original`.
pub fn accept_change(original: &str, proposed: &str, k: usize) -> String {
    apply_change(original, proposed, k, true)
}

/// Reverts change `k` of the diff between `original` and `proposed` in `proposed` back to `original`.
pub fn reject_change(original: &str, proposed: &str, k: usize) -> String {
    apply_change(original, proposed, k, false)
}

fn apply_change(original: &str, proposed: &str, k: usize, accept: bool) -> String {
    let diff = TextDiff::from_lines(original, proposed);
    let mut count = 0;
    let mut in_change = false;
    for op in diff.ops() {
        if op.tag() == DiffTag::Equal {
            in_change = false;
        } else if !in_change {
            count += 1;
            in_change = true;
        }
    }
    if k >= count {
        return if accept {
            original.to_string()
        } else {
            proposed.to_string()
        };
    }

    let mut out = String::new();
    let mut current_change = 0;
    let mut in_change = false;

    for op in diff.ops() {
        if op.tag() == DiffTag::Equal {
            in_change = false;
            for change in diff.iter_changes(op) {
                out.push_str(change.value());
            }
        } else {
            let c = if in_change {
                current_change - 1
            } else {
                let c = current_change;
                current_change += 1;
                in_change = true;
                c
            };
            let keep_proposed = if accept { c == k } else { c != k };
            let desired_tag = if keep_proposed {
                ChangeTag::Insert
            } else {
                ChangeTag::Delete
            };
            for change in diff.iter_changes(op) {
                if change.tag() == desired_tag {
                    out.push_str(change.value());
                }
            }
        }
    }

    out
}

#[note("diff-review-hex-sha256")]
pub fn hex_sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

#[note("diff-review-compute-diff-hunks")]
pub fn compute_diff_hunks(original: &str, proposed: &str) -> Vec<DiffHunk> {
    if original == proposed {
        return Vec::new();
    }

    let orig_lines: Vec<&str> = original.lines().collect();
    let prop_lines: Vec<&str> = proposed.lines().collect();

    // Find the common prefix
    let mut prefix_len = 0;
    while prefix_len < orig_lines.len()
        && prefix_len < prop_lines.len()
        && orig_lines[prefix_len] == prop_lines[prefix_len]
    {
        prefix_len += 1;
    }

    // Find the common suffix
    let mut orig_suffix = orig_lines.len();
    let mut prop_suffix = prop_lines.len();
    while orig_suffix > prefix_len
        && prop_suffix > prefix_len
        && orig_lines[orig_suffix - 1] == prop_lines[prop_suffix - 1]
    {
        orig_suffix -= 1;
        prop_suffix -= 1;
    }

    let old_slice = &orig_lines[prefix_len..orig_suffix];
    let new_slice = &prop_lines[prefix_len..prop_suffix];

    vec![DiffHunk {
        id: 1,
        start_line_old: prefix_len + 1,
        end_line_old: orig_suffix,
        start_line_new: prefix_len + 1,
        end_line_new: prop_suffix,
        old_lines: old_slice.iter().map(|s| s.to_string()).collect(),
        new_lines: new_slice.iter().map(|s| s.to_string()).collect(),
        status: HunkStatus::Pending,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_diff_hunks_finds_insertion_and_removal() {
        let orig = "fn main() {\n    println!(\"hello\");\n}\n";
        let prop = "fn main() {\n    println!(\"world\");\n    println!(\"crawrad\");\n}\n";
        let hunks = compute_diff_hunks(orig, prop);

        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].old_lines, vec!["    println!(\"hello\");"]);
        assert_eq!(
            hunks[0].new_lines,
            vec!["    println!(\"world\");", "    println!(\"crawrad\");"]
        );
        assert_eq!(hunks[0].added_count(), 2);
        assert_eq!(hunks[0].removed_count(), 1);
    }

    #[test]
    fn test_diff_review_lifecycle_accept_and_reject() {
        let mut mgr = DiffReviewManager::new();
        let path = PathBuf::from("src/main.rs");
        let orig = "line 1\nline 2\n";
        let prop = "line 1\nline 2 edited\n";

        let review = mgr
            .propose_edit(
                path.clone(),
                orig.to_string(),
                prop.to_string(),
                "editor.apply_edit",
            )
            .expect("proposal accepted");

        assert_eq!(review.author_tool, "editor.apply_edit");
        assert!(mgr.has_pending_review(&path));

        // A new edit on the same file must be REFUSED while one is pending
        let err = mgr.propose_edit(path.clone(), orig.to_string(), "line 3".into(), "code.edit");
        assert!(err.is_err());
        assert!(
            err.unwrap_err()
                .contains("pending diff review awaiting user decision")
        );

        // Rejecting restores the original byte for byte
        let rejected = mgr.reject(&path).expect("rejection succeeded");
        assert_eq!(rejected, orig);
        assert!(!mgr.has_pending_review(&path));

        // A new proposal after a rejection can be accepted
        mgr.propose_edit(
            path.clone(),
            orig.to_string(),
            prop.to_string(),
            "code.edit",
        )
        .expect("new proposal");
        assert!(mgr.has_pending_review(&path));

        let accepted = mgr.accept(&path).expect("acceptance succeeded");
        assert_eq!(accepted, prop);
        assert!(!mgr.has_pending_review(&path));
    }

    fn two_changes(orig: &str, prop: &str) -> (DiffReviewManager, PathBuf) {
        let mut mgr = DiffReviewManager::new();
        let path = PathBuf::from("src/main.rs");
        let edit = (orig.to_string(), prop.to_string());
        mgr.propose_edit(path.clone(), edit.0, edit.1, "editor.apply_edit")
            .expect("proposal accepted");
        (mgr, path)
    }

    #[test]
    fn test_decide_change_accept_and_reject_lifecycle() {
        let (orig, prop) = (
            "a
b
c
d
e
",
            "a
B
c
D
e
",
        );
        let (mut mgr, path) = two_changes(orig, prop);

        // Accepting change 0 modifies original_content so only change 1 remains
        let res0 = mgr.decide_change(&path, 0, true).expect("decide change 0");
        assert!(!res0);
        assert!(mgr.has_pending_review(&path));
        let review = mgr.get_pending_review(&path).expect("still pending");
        assert_eq!(review.original_content, "a\nB\nc\nd\ne\n");

        // Accepting the last change leaves nothing: the review is accepted whole
        assert!(mgr.decide_change(&path, 0, true).expect("decide remaining"));
        let review = mgr
            .get_pending_review(&path)
            .expect("pending until accepted");
        assert_eq!(
            review.base_content, orig,
            "undo still goes back to the start"
        );
        assert_eq!(mgr.accept(&path).expect("accept"), prop);
        assert!(!mgr.has_pending_review(&path));
    }

    #[test]
    fn test_decide_change_reject_to_finish() {
        let (orig, prop) = (
            "a
b
c
d
e
",
            "a
B
c
D
e
",
        );
        let (mut mgr, path) = two_changes(orig, prop);

        // Rejecting change 1 reverts it in proposed_content
        assert!(!mgr.decide_change(&path, 1, false).expect("reject change 1"));
        let review = mgr.get_pending_review(&path).expect("still pending");
        assert_eq!(review.proposed_content, "a\nB\nc\nd\ne\n");

        // Rejecting remaining change (which is change 0) reverts proposed_content back to orig
        assert!(mgr.decide_change(&path, 0, false).expect("reject change 0"));
        assert_eq!(mgr.accept(&path).expect("accept"), orig);
    }

    #[test]
    fn accept_or_reject_one_of_two_changes() {
        let (orig, prop) = (
            "a
b
c
d
e
",
            "a
B
c
D
e
",
        );
        let accepted = accept_change(orig, prop, 0);
        assert_eq!(
            accepted,
            "a
B
c
d
e
"
        );
        assert_eq!(accept_change(&accepted, prop, 0), prop);
        let rejected = reject_change(orig, prop, 1);
        assert_eq!(
            rejected,
            "a
B
c
d
e
"
        );
        assert_eq!(reject_change(orig, &rejected, 0), orig);
    }

    #[test]
    fn accept_and_reject_insert_delete_replace() {
        for (orig, prop) in [
            (
                "first
third
",
                "first
second
third
",
            ),
            (
                "first
second
third
",
                "first
third
",
            ),
            (
                "first
old
third
",
                "first
new
third
",
            ),
        ] {
            assert_eq!(accept_change(orig, prop, 0), prop);
            assert_eq!(reject_change(orig, prop, 0), orig);
            assert_eq!(
                accept_change(orig, prop, 9),
                orig,
                "out of range keeps the input"
            );
        }
    }

    #[test]
    fn a_missing_final_newline_stays_missing() {
        assert_eq!(
            accept_change(
                "a
b", "a
B", 0
            ),
            "a
B"
        );
        assert_eq!(
            reject_change(
                "a
b", "a
B", 0
            ),
            "a
b"
        );
        assert_eq!(
            accept_change(
                "a
b
", "a
B
", 0
            ),
            "a
B
"
        );
    }
}
