//! **Adding a project to the workspace, and choosing the active one**.
//!
//! A RAD IDE works inside a group of projects; in Rust the group is the Cargo
//! workspace, whose root `Cargo.toml` lists its `members` and, in
//! `default-members`, the project `cargo run` and the IDE start (the *active*
//! one, as [`crate::workspace::members`] reads it).
//!
//! The edits touch ONLY those two keys of the `[workspace]` section and keep
//! every other byte of the file — the user's comments, order and layout — the
//! way `cargo new` does when it adds a member. Both shapes cargo and people
//! write are handled: the list on one line and the list across lines.

use std::path::{Path, PathBuf};

use crawrad_actions::WorkspaceRole;

use crate::workspace::{MANIFEST, workspace_list};
use crawrad_note::note;

#[note("workspace-edit-keyspan")]
struct KeySpan {
    first: usize,
    last: usize,
}

#[note("workspace-edit-locate")]
fn locate(lines: &[&str], key: &str) -> (Option<usize>, Option<KeySpan>) {
    let mut header = None;
    let mut in_section = false;
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        if line.starts_with('[') && !line.starts_with("[[") && !line.contains('=') {
            in_section = line.starts_with("[workspace]");
            if in_section {
                header = Some(i);
            }
        } else if in_section {
            let is_key = line
                .strip_prefix(key)
                .is_some_and(|rest| rest.trim_start().starts_with('='));
            if is_key {
                let mut last = i;
                while !lines[last].contains(']') && last + 1 < lines.len() {
                    last += 1;
                }
                return (header, Some(KeySpan { first: i, last }));
            }
        }
        i += 1;
    }
    (header, None)
}

#[note("workspace-edit-with-member")]
pub fn with_member(manifest: &str, member: &str) -> Result<String, String> {
    if workspace_list(manifest, "members")
        .iter()
        .any(|m| m == member)
    {
        return Ok(manifest.to_string());
    }
    let lines: Vec<&str> = manifest.lines().collect();
    let (header, span) = locate(&lines, "members");
    let Some(header) = header else {
        return Err("this Cargo.toml has no [workspace] section".to_string());
    };
    let mut out: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    match span {
        None => out.insert(header + 1, format!("members = [\"{member}\"]")),
        Some(KeySpan { first, last }) if first == last => {
            let line = &out[first];
            let close = line.rfind(']').unwrap_or(line.len());
            let inside = line[line.find('[').map_or(close, |o| o + 1)..close].trim();
            let sep = if inside.is_empty() || inside.ends_with(',') {
                ""
            } else {
                ", "
            };
            out[first] = format!(
                "{}{sep}\"{member}\"{}",
                line[..close].trim_end(),
                &line[close..]
            );
        }
        Some(KeySpan { last, .. }) => {
            let previous = out[last - 1].clone();
            let indent: String = previous.chars().take_while(|c| c.is_whitespace()).collect();
            if !previous.trim_end().ends_with(',') && !previous.trim_end().ends_with('[') {
                out[last - 1] = format!("{},", previous.trim_end());
            }
            out.insert(last, format!("{indent}\"{member}\","));
        }
    }
    Ok(out.join("\n") + "\n")
}

#[note("workspace-edit-with-active")]
pub fn with_active(manifest: &str, member: &str) -> Result<String, String> {
    let lines: Vec<&str> = manifest.lines().collect();
    let (header, span) = locate(&lines, "default-members");
    let Some(header) = header else {
        return Err("this Cargo.toml has no [workspace] section".to_string());
    };
    let mut out: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    let entry = format!("default-members = [\"{member}\"]");
    match span {
        Some(KeySpan { first, last }) => {
            out.splice(first..=last, std::iter::once(entry));
        }
        None => out.insert(header + 1, entry),
    }
    Ok(out.join("\n") + "\n")
}

#[note("workspace-edit-member-name")]
pub fn member_name(root: &Path, member: &Path) -> Result<String, String> {
    let rel = member.strip_prefix(root).map_err(|_| {
        format!(
            "'{}' is not inside the workspace folder '{}'",
            member.display(),
            root.display()
        )
    })?;
    let name = rel.to_string_lossy().replace('\\', "/");
    if name.is_empty() {
        return Err("the workspace root cannot be its own member".to_string());
    }
    Ok(name)
}

#[note("workspace-edit-rewrite")]
fn rewrite(root: &Path, edit: impl FnOnce(&str) -> Result<String, String>) -> Result<(), String> {
    let path = root.join(MANIFEST);
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    std::fs::write(&path, edit(&text)?).map_err(|e| format!("{}: {e}", path.display()))
}

#[note("workspace-edit-add-member")]
pub fn add_member(root: &Path, member: &Path) -> Result<(), String> {
    if !member.join(MANIFEST).exists() {
        return Err(format!(
            "'{}' has no Cargo.toml: it is not a project",
            member.display()
        ));
    }
    let name = member_name(root, member)?;
    rewrite(root, |text| with_member(text, &name))
}

#[note("workspace-edit-set-active")]
pub fn set_active(root: &Path, member: &Path) -> Result<(), String> {
    let name = member_name(root, member)?;
    rewrite(root, |text| with_active(text, &name))
}

#[note("workspace-edit-role-of-item")]
pub fn role_of_item(path: &Path, is_dir: bool) -> WorkspaceRole {
    if is_dir {
        role_of(path)
    } else {
        WorkspaceRole::Other
    }
}

#[note("workspace-edit-role-of")]
pub fn role_of(path: &Path) -> WorkspaceRole {
    let read = |p: &Path| std::fs::read_to_string(p.join(MANIFEST)).unwrap_or_default();
    if read(path).contains("[workspace]") {
        return WorkspaceRole::Root;
    }
    let Some(root) = path.parent().map(PathBuf::from) else {
        return WorkspaceRole::Other;
    };
    let manifest = read(&root);
    let Ok(name) = member_name(&root, path) else {
        return WorkspaceRole::Other;
    };
    if !workspace_list(&manifest, "members").contains(&name) {
        return WorkspaceRole::Other;
    }
    let active = workspace_list(&manifest, "default-members").contains(&name);
    WorkspaceRole::Member { active }
}

#[note("workspace-edit-workspacestep")]
#[derive(Debug, PartialEq, Eq)]
pub enum WorkspaceStep {
    OpenProject(PathBuf),
    AskInDialog(PathBuf),
    Say(String),
}

#[note("workspace-edit-run")]
pub fn run(
    root: Option<&Path>,
    command: crawrad_actions::FileEffect,
    pick: impl FnOnce() -> Option<PathBuf>,
) -> WorkspaceStep {
    use crawrad_actions::FileEffect;
    let Some(root) = root.filter(|r| role_of(r) == WorkspaceRole::Root) else {
        return WorkspaceStep::Say(
            "this needs an open workspace: File > New Project creates one".into(),
        );
    };
    let say = |done: Result<(), String>, what: &Path, how: &str| {
        WorkspaceStep::Say(done.map_or_else(|e| e, |_| format!("{} {how}", what.display())))
    };
    match command {
        FileEffect::AddProject { name: None, .. } => WorkspaceStep::AskInDialog(root.to_path_buf()),
        FileEffect::AddProject {
            name: Some(name),
            template,
        } => {
            let template = template.as_deref().unwrap_or("console_app");
            match crate::project_template::add_project(
                root,
                &name,
                template,
                crawrad_codegen::AlvoGerador::Makepad,
            ) {
                Ok(created) => WorkspaceStep::OpenProject(created),
                Err(e) => WorkspaceStep::Say(e),
            }
        }
        FileEffect::AddExistingProject(folder) => match folder.or_else(pick) {
            Some(f) => say(add_member(root, &f), &f, "joined the workspace"),
            None => WorkspaceStep::Say(String::new()),
        },
        FileEffect::SetActiveProject(f) => {
            say(set_active(root, &f), &f, "is now the active project")
        }
        _ => WorkspaceStep::Say(String::new()),
    }
}

#[cfg(test)]
#[path = "test/workspace_edit/test.rs"]
mod tests;
