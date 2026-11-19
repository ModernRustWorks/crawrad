//! The manual page "Project templates", generated from the template catalogue.

use crate::project_template::{DEFAULT_PROJECT_NAME, TEMPLATES, structure_preview};
use crawrad_note::note;

#[cfg(test)]
#[path = "test/reference/test.rs"]
mod tests;

#[note("templates-reference-markdown")]
pub fn templates_reference_markdown() -> String {
    let mut out = String::from("# Project templates\n\n");
    out.push_str(
        "This page is generated from the project template catalogue: every \
         template File > New Project offers.\n",
    );
    for t in TEMPLATES {
        out.push_str(&format!("\n## {}\n\n{}\n\n", t.name, t.description));
        out.push_str(&format!(
            "Platforms: {}.\n\n",
            platforms(t.platforms_detail)
        ));
        out.push_str(&format!(
            "Files created for a project named `{DEFAULT_PROJECT_NAME}` (a Cargo workspace with the project inside it):\n\n```text\n{}\n```\n",
            structure_preview(t, DEFAULT_PROJECT_NAME, None)
        ));
    }
    out
}

/// The platform line without its icons: `⊞ Windows    🐧 Linux` becomes `Windows, Linux`.
fn platforms(detail: &str) -> String {
    detail
        .split("    ")
        .map(|p| {
            p.trim()
                .trim_start_matches(|c: char| !c.is_ascii_alphanumeric())
                .trim()
        })
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join(", ")
}
