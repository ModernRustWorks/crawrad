//! Rust teacher module for conversational code explanation and diagnostics grounding.

#![forbid(unsafe_code)]

use crawrad_note::note;
#[note("teacher-error-code")]
pub fn error_code(code: Option<&str>) -> Option<String> {
    let s = code?.trim();
    if s.len() == 5 {
        let bytes = s.as_bytes();
        if (bytes[0] == b'E' || bytes[0] == b'e') && bytes[1..].iter().all(u8::is_ascii_digit) {
            return Some(format!("E{}", &s[1..]));
        }
    }
    None
}

#[note("teacher-book-chapter")]
pub fn book_chapter(code: &str) -> Option<(&'static str, &'static str)> {
    let normalized = error_code(Some(code))?;
    match normalized.as_str() {
        "E0382" | "E0505" | "E0507" => Some((
            "Understanding Ownership",
            "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html",
        )),
        "E0499" | "E0502" | "E0596" | "E0716" => Some((
            "References and Borrowing",
            "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html",
        )),
        "E0106" | "E0597" | "E0515" => Some((
            "Validating References with Lifetimes",
            "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html",
        )),
        "E0308" => Some((
            "Data Types",
            "https://doc.rust-lang.org/book/ch03-02-data-types.html",
        )),
        "E0277" => Some((
            "Traits: Defining Shared Behavior",
            "https://doc.rust-lang.org/book/ch10-02-traits.html",
        )),
        "E0599" => Some((
            "Method Syntax",
            "https://doc.rust-lang.org/book/ch05-03-method-syntax.html",
        )),
        "E0425" | "E0433" => Some((
            "Paths for Referring to an Item in the Module Tree",
            "https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html",
        )),
        "E0384" => Some((
            "Variables and Mutability",
            "https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html",
        )),
        _ => None,
    }
}

#[note("teacher-rustc-explain")]
pub fn rustc_explain(code: &str) -> Option<String> {
    // Only a well-formed code reaches the process, never user text.
    if error_code(Some(code)).as_deref() != Some(code) {
        return None;
    }
    // no console window next to the IDE (crawrad_os's rule for every child)
    let mut cmd = crawrad_os::process::no_console("rustc");
    cmd.arg("--explain").arg(code);
    let output = cmd.output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    let truncated = if trimmed.len() > 4000 {
        let mut boundary = 4000;
        while !trimmed.is_char_boundary(boundary) {
            boundary -= 1;
        }
        &trimmed[..boundary]
    } else {
        trimmed
    };
    Some(truncated.to_string())
}

#[note("teacher-pick-on-line")]
pub fn pick_on_line<T>(
    items: &[T],
    line0: u32,
    span: impl Fn(&T) -> (u32, u32, bool),
) -> Option<&T> {
    let on: Vec<&T> = items
        .iter()
        .filter(|d| {
            let (first, last, _) = span(d);
            first <= line0 && line0 <= last
        })
        .collect();
    on.iter().find(|d| span(d).2).or(on.first()).copied()
}

#[note("teacher-snippet-around")]
pub fn snippet_around(text: &str, line0: usize, around: usize) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let first = line0.saturating_sub(around).min(lines.len());
    let last = (line0 + around + 1).min(lines.len());
    lines[first..last].join("\n")
}

#[note("teacher-grounding")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grounding {
    pub code: Option<String>,
    pub message: String,
    pub file: String,
    pub line: u32, /* 1-based */
    pub snippet: String,
    pub explain: Option<String>,
}

#[note("teacher-teacher-rules")]
pub const TEACHER_RULES: &str = "\
You are a patient Rust teacher for people learning the language.
Ground every claim in the compiler's own words given below, never invent a rule.
Explain WHY Rust demands it (ownership, borrowing, lifetimes, traits, types) in plain words.
When a choice exists, say why this one and not the neighbour (e.g. borrow vs clone, Rc vs Arc vs Box, &str vs String, ? vs unwrap).
Finish with the book chapter when one is given.";

#[note("teacher-fix-explain-prompt")]
pub fn fix_explain_prompt(g: &Grounding, lang: &str) -> String {
    let mut p = String::new();
    p.push_str("[crawrad-teacher]\n");
    p.push_str(TEACHER_RULES);
    p.push_str("\n\n## Task\nFix this diagnostic with the smallest change and show it as a diff in your answer; do NOT edit any file, the learner applies it. Then teach: 1) what was wrong, 2) which Rust rule it broke, 3) why the language demands it, 4) why this fix and not the alternatives.\n\n");
    p.push_str("## Diagnostic\n");
    if let Some(ref code) = g.code {
        p.push_str(&format!("Code: {code}\n"));
    }
    p.push_str(&format!("Message: {}\n", g.message));
    p.push_str(&format!("Location: {}:{}\n\n", g.file, g.line));
    p.push_str("## Code\n```rust\n");
    p.push_str(&g.snippet);
    if !g.snippet.ends_with('\n') {
        p.push('\n');
    }
    p.push_str("```\n");
    if let Some(ref explain) = g.explain {
        let code_str = g.code.as_deref().unwrap_or("");
        p.push_str(&format!(
            "\n## Compiler explanation (rustc --explain {code_str})\n{explain}\n"
        ));
    }
    if let Some(ref code) = g.code
        && let Some((title, url)) = book_chapter(code)
    {
        p.push_str(&format!("\n## Book\n{title}: {url}\n"));
    }
    p.push_str(&format!(
        "\nAnswer in {lang}, but keep code and identifiers in English."
    ));
    p
}

#[note("teacher-why-prompt")]
pub fn why_prompt(g: &Grounding, lang: &str) -> String {
    let mut p = String::new();
    p.push_str("[crawrad-teacher]\n");
    p.push_str(TEACHER_RULES);
    p.push_str("\n\n## Task\nDo not change code. Explain why the compiler rejects this and what rule a fix must satisfy.\n\n");
    p.push_str("## Diagnostic\n");
    if let Some(ref code) = g.code {
        p.push_str(&format!("Code: {code}\n"));
    }
    p.push_str(&format!("Message: {}\n", g.message));
    p.push_str(&format!("Location: {}:{}\n\n", g.file, g.line));
    p.push_str("## Code\n```rust\n");
    p.push_str(&g.snippet);
    if !g.snippet.ends_with('\n') {
        p.push('\n');
    }
    p.push_str("```\n");
    if let Some(ref explain) = g.explain {
        let code_str = g.code.as_deref().unwrap_or("");
        p.push_str(&format!(
            "\n## Compiler explanation (rustc --explain {code_str})\n{explain}\n"
        ));
    }
    if let Some(ref code) = g.code
        && let Some((title, url)) = book_chapter(code)
    {
        p.push_str(&format!("\n## Book\n{title}: {url}\n"));
    }
    p.push_str(&format!(
        "\nAnswer in {lang}, but keep code and identifiers in English."
    ));
    p
}

#[note("teacher-teach-prompt")]
pub fn teach_prompt(focus: &str, lang: &str) -> String {
    let mut p = String::new();
    p.push_str("[crawrad-teacher]\n");
    p.push_str(TEACHER_RULES);
    p.push_str("\n\nExplain the selected code (it is in the context) line by line for a learner: ownership, borrows, lifetimes, traits and the types involved.\n");
    let trimmed = focus.trim();
    if !trimmed.is_empty() {
        p.push_str(&format!("Focus: {trimmed}\n"));
    }
    p.push_str(&format!(
        "\nAnswer in {lang}, but keep code and identifiers in English."
    ));
    p
}

#[note("teacher-compare-prompt")]
pub fn compare_prompt(a: &str, b: &str, lang: &str) -> String {
    let mut p = String::new();
    p.push_str("[crawrad-teacher]\n");
    p.push_str(TEACHER_RULES);
    p.push_str(&format!(
        "\n\nCompare `{a}` and `{b}` for a learner: what each is, when to use which, a tiny example of each, and the cost of each.\n"
    ));
    p.push_str(&format!(
        "\nAnswer in {lang}, but keep code and identifiers in English."
    ));
    p
}

#[note("teacher-no-diagnostic-message")]
pub fn no_diagnostic_message() -> &'static str {
    "No compiler diagnostic on the caret's line. Put the caret on a line with an error, or use /teach for the selected code."
}

#[note("teacher-reply")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reply {
    Prompt(String),
    Say(String),
}

#[note("teacher-command")]
pub fn command(
    id: &str,
    rest: &str,
    lang: &str,
    selected: bool,
    grounding: Option<Grounding>,
) -> Reply {
    let say = |s: &str| Reply::Say(s.to_string());
    match id {
        "teach" if !selected => {
            say("⚠️ Select the code to teach first, or put the caret on an error and use /why.")
        }
        "teach" => Reply::Prompt(teach_prompt(rest, lang)),
        "compare" => match rest.split_whitespace().collect::<Vec<_>>()[..] {
            [a, b] => Reply::Prompt(compare_prompt(a, b, lang)),
            _ => say(
                "💡 Usage: `/compare <a> <b>`, e.g. `/compare Rc Arc` or `/compare &str String`",
            ),
        },
        "why" | "fix-explain" => match grounding {
            None => say(no_diagnostic_message()),
            Some(g) if id == "why" => Reply::Prompt(why_prompt(&g, lang)),
            Some(g) => Reply::Prompt(fix_explain_prompt(&g, lang)),
        },
        _ => say("Unknown teacher command."),
    }
}

#[cfg(test)]
#[path = "test/teacher/test.rs"]
mod tests;
