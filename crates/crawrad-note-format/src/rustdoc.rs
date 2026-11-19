//! The rustdoc rule for documentation notes.
//!
//! Follows official conventions:
//! - The rustdoc book: "How to write documentation"
//!   <https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html>
//! - Rust API Guidelines: C-EXAMPLE and C-FAILURE (Errors, Panics, Safety)
//!   <https://rust-lang.github.io/api-guidelines/documentation.html>

/// Standard documentation sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Summary,
    Examples,
    Errors,
    Panics,
    Safety,
}

impl Section {
    /// Returns the label of the section.
    pub fn label(self) -> &'static str {
        match self {
            Self::Summary => "Summary",
            Self::Examples => "# Examples",
            Self::Errors => "# Errors",
            Self::Panics => "# Panics",
            Self::Safety => "# Safety",
        }
    }

    /// Returns the markdown heading for this section, or `None` for summary.
    pub fn heading(self) -> Option<&'static str> {
        match self {
            Self::Summary => None,
            Self::Examples => Some("# Examples"),
            Self::Errors => Some("# Errors"),
            Self::Panics => Some("# Panics"),
            Self::Safety => Some("# Safety"),
        }
    }
}

/// A documentation section suggested based on item signature or body analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    pub section: Section,
    pub required: bool,
    pub reason: String,
}

/// What the item itself asks its docs to carry, read from the signature and body.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ItemFacts {
    /// The item is part of the public API (`pub`); only then are the sections required.
    pub is_public: bool,
    pub returns_result: bool,
    pub is_unsafe: bool,
    pub is_trait: bool,
    pub first_panic: Option<String>,
}

/// Returns the suggested sections for an item based on its facts.
pub fn suggestions_for(facts: &ItemFacts) -> Vec<Suggestion> {
    let mut suggestions = Vec::new();

    if facts.returns_result {
        suggestions.push(Suggestion {
            section: Section::Errors,
            required: false,
            reason: "returns Result".to_string(),
        });
    }

    if let Some(ref panic_reason) = facts.first_panic {
        suggestions.push(Suggestion {
            section: Section::Panics,
            required: false,
            reason: panic_reason.clone(),
        });
    }

    if facts.is_unsafe {
        let reason = if facts.is_trait {
            "unsafe trait".to_string()
        } else {
            "unsafe fn".to_string()
        };
        suggestions.push(Suggestion {
            section: Section::Safety,
            required: true,
            reason,
        });
    }

    suggestions
}

/// Scans a line for the earliest panic-inducing construct.
pub fn earliest_panic_construct(line: &str) -> Option<(usize, &'static str)> {
    let mut earliest: Option<(usize, &'static str)> = None;

    let update =
        |earliest: &mut Option<(usize, &'static str)>, pos: usize, construct: &'static str| {
            match earliest {
                Some((prev_pos, _)) if pos < *prev_pos => *earliest = Some((pos, construct)),
                None => *earliest = Some((pos, construct)),
                _ => {}
            }
        };

    // Method calls
    if let Some(pos) = line.find(".unwrap()") {
        update(&mut earliest, pos, "unwrap()");
    }
    if let Some(pos) = line.find(".expect(") {
        update(&mut earliest, pos, "expect()");
    }

    // Macro invocations
    let macros = [
        ("panic!", "panic!"),
        ("unreachable!", "unreachable!"),
        ("todo!", "todo!"),
        ("unimplemented!", "unimplemented!"),
        ("assert_eq!", "assert_eq!"),
        ("assert_ne!", "assert_ne!"),
        ("assert!", "assert!"),
    ];

    for (needle, name) in macros {
        let mut start_idx = 0;
        while let Some(rel_pos) = line[start_idx..].find(needle) {
            let pos = start_idx + rel_pos;
            start_idx = pos + needle.len();

            let bytes = line.as_bytes();
            let prev_ok = if pos == 0 {
                true
            } else {
                let prev_char = bytes[pos - 1];
                !prev_char.is_ascii_alphanumeric() && prev_char != b'_'
            };

            let next_ok = if needle == "assert!" {
                !line[pos..].starts_with("assert_eq!") && !line[pos..].starts_with("assert_ne!")
            } else {
                true
            };

            let not_debug = if pos >= 6 {
                &line[pos - 6..pos] != "debug_"
            } else {
                true
            };

            if prev_ok && next_ok && not_debug {
                update(&mut earliest, pos, name);
                break;
            }
        }
    }

    earliest
}

/// Returns the first panic construct on a line, if any.
pub fn first_panic_construct(line: &str) -> Option<&'static str> {
    earliest_panic_construct(line).map(|(_, construct)| construct)
}

/// Sections the signature asks for that the body markdown does not have yet.
pub fn missing_sections(markdown: &str, suggestions: &[Suggestion]) -> Vec<Suggestion> {
    let mut missing = Vec::new();
    for suggestion in suggestions {
        let heading = match suggestion.section.heading() {
            Some(h) => h,
            None => continue,
        };
        let h_lower = heading.to_ascii_lowercase();
        let mut present = false;
        let mut in_fence = false;

        for line in markdown.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("```") {
                in_fence = !in_fence;
                continue;
            }
            if in_fence {
                continue;
            }
            let l_lower = trimmed.to_ascii_lowercase();
            if l_lower == h_lower
                || (l_lower.starts_with(&h_lower)
                    && l_lower[h_lower.len()..].starts_with(char::is_whitespace))
            {
                present = true;
                break;
            }
        }

        if !present {
            missing.push(suggestion.clone());
        }
    }
    missing
}

/// Checks the first paragraph for rustdoc summary quality.
pub fn summary_warning(markdown: &str) -> Option<String> {
    let summary = extract_summary_paragraph(markdown);
    if summary.is_empty() {
        return Some(
            "Missing summary: the first paragraph is what rustdoc shows in lists.".to_string(),
        );
    }
    let count = summary.chars().count();
    if count > 80 {
        return Some(format!(
            "Summary is {count} chars; rustdoc shows it in lists — keep it under 80."
        ));
    }
    if !summary.ends_with('.') {
        return Some("Summary should be one sentence ending with a period.".to_string());
    }
    None
}

/// Extracts the summary paragraph from markdown text, ignoring fences and headings.
pub fn extract_summary_paragraph(markdown: &str) -> String {
    let mut in_fence = false;
    let mut current_para: Vec<&str> = Vec::new();

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            if !current_para.is_empty() {
                let joined = current_para.join(" ");
                let clean = joined.trim();
                if !clean.starts_with('#') && !clean.is_empty() {
                    return clean.to_string();
                }
                current_para.clear();
            }
            continue;
        }
        if in_fence {
            continue;
        }
        if trimmed.is_empty() || trimmed.starts_with('#') {
            if !current_para.is_empty() {
                let joined = current_para.join(" ");
                let clean = joined.trim();
                if !clean.starts_with('#') && !clean.is_empty() {
                    return clean.to_string();
                }
                current_para.clear();
            }
        } else {
            current_para.push(trimmed);
        }
    }

    if !current_para.is_empty() {
        let joined = current_para.join(" ");
        let clean = joined.trim();
        if !clean.starts_with('#') && !clean.is_empty() {
            return clean.to_string();
        }
    }

    String::new()
}

/// Appends (or for Summary, prepends a placeholder line) the section template.
pub fn insert_section(markdown: &str, section: Section) -> String {
    match section {
        Section::Summary => {
            let summary = extract_summary_paragraph(markdown);
            if summary.is_empty() {
                format!("One-line summary.\n\n{markdown}")
            } else {
                markdown.to_string()
            }
        }
        Section::Examples | Section::Errors | Section::Panics | Section::Safety => {
            let dummy_suggestion = Suggestion {
                section,
                required: false,
                reason: String::new(),
            };
            if missing_sections(markdown, &[dummy_suggestion]).is_empty() {
                return markdown.to_string();
            }

            let template = match section {
                Section::Examples => "# Examples\n\n```\n\n```",
                Section::Errors => "# Errors\n\n",
                Section::Panics => "# Panics\n\n",
                Section::Safety => "# Safety\n\n",
                Section::Summary => unreachable!(),
            };

            if markdown.trim().is_empty() {
                template.to_string()
            } else {
                format!("{}\n\n{template}", markdown.trim_end())
            }
        }
    }
}

/// Whether rustdoc reads a fence with this info string as Rust: no tag, or
/// any rustdoc attribute (`rust`, `no_run`, `ignore`, `should_panic`, ...).
fn is_rust_code_fence(info: &str) -> bool {
    let info = info.trim();
    info.is_empty()
        || info.split([',', ' ']).map(str::trim).any(|tag| {
            matches!(
                tag,
                "rust" | "no_run" | "ignore" | "should_panic" | "compile_fail"
            ) || tag.starts_with("edition")
        })
}

/// Checks that `# Examples` is present and contains at least one rust code fence.
fn check_examples(markdown: &str) -> Option<String> {
    let mut in_fence = false;
    let mut in_examples = false;
    let mut found_examples = false;
    let mut found_rust_fence = false;

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            if in_examples && !in_fence {
                let info = trimmed.trim_start_matches('`').trim();
                if is_rust_code_fence(info) {
                    found_rust_fence = true;
                }
            }
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }

        let l_lower = trimmed.to_ascii_lowercase();
        if l_lower == "# examples"
            || (l_lower.starts_with("# examples")
                && l_lower["# examples".len()..].starts_with(char::is_whitespace))
        {
            found_examples = true;
            in_examples = true;
        } else if in_examples && trimmed.starts_with("# ") {
            in_examples = false;
        }
    }

    if !found_examples {
        Some("missing `# Examples` section".to_string())
    } else if !found_rust_fence {
        Some("# Examples has no rust code block".to_string())
    } else {
        None
    }
}

/// Every way a `kind: doc` body breaks the rustdoc conventions, empty when it follows them.
pub fn doc_problems(markdown: &str, facts: &ItemFacts) -> Vec<String> {
    let mut problems = Vec::new();

    if let Some(warning) = summary_warning(markdown) {
        problems.push(warning);
    }

    if facts.is_public {
        if let Some(examples_problem) = check_examples(markdown) {
            problems.push(examples_problem);
        }

        let suggestions = suggestions_for(facts);
        for missing in missing_sections(markdown, &suggestions) {
            problems.push(format!(
                "missing `{}`: {}",
                missing.section.label(),
                missing.reason
            ));
        }
    }

    problems
}

#[cfg(feature = "syn")]
pub use syn_support::item_facts;

#[cfg(feature = "syn")]
mod syn_support {
    use super::ItemFacts;
    use proc_macro2::{Delimiter, TokenStream, TokenTree};
    use quote::ToTokens;

    /// Builds `ItemFacts` from Rust item tokens.
    pub fn item_facts(tokens: &TokenStream) -> ItemFacts {
        // A bodiless `fn f(&self);` parses as `Item::Verbatim`: it is a trait item.
        if let Ok(item) = syn::parse2::<syn::Item>(tokens.clone())
            && !matches!(item, syn::Item::Verbatim(_))
        {
            return item_facts_item(&item);
        }
        if let Ok(item) = syn::parse2::<syn::TraitItem>(tokens.clone()) {
            return item_facts_trait_item(&item);
        }
        if let Ok(item) = syn::parse2::<syn::ImplItem>(tokens.clone()) {
            return item_facts_impl_item(&item);
        }
        ItemFacts::default()
    }

    fn is_public_vis(vis: &syn::Visibility) -> bool {
        matches!(vis, syn::Visibility::Public(_))
    }

    fn item_visibility(item: &syn::Item) -> Option<&syn::Visibility> {
        match item {
            syn::Item::Const(i) => Some(&i.vis),
            syn::Item::Enum(i) => Some(&i.vis),
            syn::Item::Fn(i) => Some(&i.vis),
            syn::Item::ForeignMod(_) => None,
            syn::Item::Impl(_) => None,
            syn::Item::Macro(_) => None,
            syn::Item::Mod(i) => Some(&i.vis),
            syn::Item::Static(i) => Some(&i.vis),
            syn::Item::Struct(i) => Some(&i.vis),
            syn::Item::Trait(i) => Some(&i.vis),
            syn::Item::TraitAlias(i) => Some(&i.vis),
            syn::Item::Type(i) => Some(&i.vis),
            syn::Item::Union(i) => Some(&i.vis),
            syn::Item::Use(i) => Some(&i.vis),
            _ => None,
        }
    }

    fn impl_item_visibility(item: &syn::ImplItem) -> Option<&syn::Visibility> {
        match item {
            syn::ImplItem::Const(c) => Some(&c.vis),
            syn::ImplItem::Fn(f) => Some(&f.vis),
            syn::ImplItem::Type(t) => Some(&t.vis),
            _ => None,
        }
    }

    fn item_facts_item(item: &syn::Item) -> ItemFacts {
        let is_public = item_visibility(item).is_some_and(is_public_vis);
        match item {
            syn::Item::Fn(f) => ItemFacts {
                is_public,
                returns_result: is_result_return(&f.sig.output),
                is_unsafe: f.sig.unsafety.is_some(),
                is_trait: false,
                first_panic: find_first_panic(&f.block.to_token_stream())
                    .map(|c| format!("calls {c}")),
            },
            syn::Item::Trait(t) => ItemFacts {
                is_public,
                returns_result: false,
                is_unsafe: t.unsafety.is_some(),
                is_trait: true,
                first_panic: None,
            },
            _ => ItemFacts {
                is_public,
                ..ItemFacts::default()
            },
        }
    }

    fn item_facts_trait_item(item: &syn::TraitItem) -> ItemFacts {
        match item {
            syn::TraitItem::Fn(f) => ItemFacts {
                is_public: true,
                returns_result: is_result_return(&f.sig.output),
                is_unsafe: f.sig.unsafety.is_some(),
                is_trait: false,
                first_panic: f
                    .default
                    .as_ref()
                    .and_then(|b| find_first_panic(&b.to_token_stream()))
                    .map(|c| format!("calls {c}")),
            },
            _ => ItemFacts {
                is_public: true,
                ..ItemFacts::default()
            },
        }
    }

    fn item_facts_impl_item(item: &syn::ImplItem) -> ItemFacts {
        let is_public = impl_item_visibility(item).is_some_and(is_public_vis);
        match item {
            syn::ImplItem::Fn(f) => ItemFacts {
                is_public,
                returns_result: is_result_return(&f.sig.output),
                is_unsafe: f.sig.unsafety.is_some(),
                is_trait: false,
                first_panic: find_first_panic(&f.block.to_token_stream())
                    .map(|c| format!("calls {c}")),
            },
            _ => ItemFacts {
                is_public,
                ..ItemFacts::default()
            },
        }
    }

    fn is_result_return(output: &syn::ReturnType) -> bool {
        match output {
            syn::ReturnType::Default => false,
            syn::ReturnType::Type(_, ty) => is_result_type(ty),
        }
    }

    fn is_result_type(ty: &syn::Type) -> bool {
        match ty {
            syn::Type::Path(type_path) => type_path
                .path
                .segments
                .last()
                .is_some_and(|seg| seg.ident == "Result"),
            syn::Type::Group(g) => is_result_type(&g.elem),
            _ => false,
        }
    }

    fn find_first_panic(stream: &TokenStream) -> Option<&'static str> {
        let mut iter = stream.clone().into_iter().peekable();
        while let Some(tt) = iter.next() {
            match tt {
                TokenTree::Punct(p) if p.as_char() == '.' => {
                    if let Some(TokenTree::Ident(ident)) = iter.peek() {
                        let name = ident.to_string();
                        if name == "unwrap" {
                            iter.next();
                            if let Some(TokenTree::Group(g)) = iter.peek()
                                && g.delimiter() == Delimiter::Parenthesis
                            {
                                return Some("unwrap()");
                            }
                        } else if name == "expect" {
                            iter.next();
                            if let Some(TokenTree::Group(g)) = iter.peek()
                                && g.delimiter() == Delimiter::Parenthesis
                            {
                                return Some("expect()");
                            }
                        }
                    }
                }
                TokenTree::Ident(ident) => {
                    let name = ident.to_string();
                    if matches!(
                        name.as_str(),
                        "panic"
                            | "unreachable"
                            | "todo"
                            | "unimplemented"
                            | "assert"
                            | "assert_eq"
                            | "assert_ne"
                    ) && let Some(TokenTree::Punct(p)) = iter.peek()
                        && p.as_char() == '!'
                    {
                        iter.next();
                        let construct = match name.as_str() {
                            "panic" => "panic!",
                            "unreachable" => "unreachable!",
                            "todo" => "todo!",
                            "unimplemented" => "unimplemented!",
                            "assert" => "assert!",
                            "assert_eq" => "assert_eq!",
                            "assert_ne" => "assert_ne!",
                            _ => unreachable!(),
                        };
                        return Some(construct);
                    }
                }
                TokenTree::Group(g) => {
                    if let Some(found) = find_first_panic(&g.stream()) {
                        return Some(found);
                    }
                }
                _ => {}
            }
        }
        None
    }
}

#[cfg(test)]
#[path = "test/rustdoc/test.rs"]
mod test;
