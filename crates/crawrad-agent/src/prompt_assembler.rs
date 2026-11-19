use crawrad_note::note;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub const DEFAULT_MAX_BYTES_PER_FILE: usize = 12 * 1024;
pub const DEFAULT_MAX_TOTAL_PROMPT_BYTES: usize = 28 * 1024;

#[note("prompt-assembler-promptmode")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptMode {
    /// Interactive chat assistant mode.
    Chat,
    /// Autonomous coding agent mode (instructs to use IDE tools first and act directly).
    Agent,
}

#[note("prompt-assembler-editorselectioncontext")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorSelectionContext {
    /// 1-based start line number.
    pub start_line: usize,
    /// 1-based end line number.
    pub end_line: usize,
    /// Selected text content.
    pub selected_text: String,
}

#[note("prompt-assembler-projectcontext")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectContext {
    /// No project context attached.
    None,
    /// Code editor is active with an open file buffer.
    CodeEditor {
        file_path: String,
        language: String,
        live_content: String,
        selection: Option<EditorSelectionContext>,
    },
    /// Visual form designer is active with a live document.
    Designer {
        form_name: String,
        form_path: Option<String>,
        component_count: usize,
        selected_components: Vec<String>,
        live_summary: String,
    },
}

impl ProjectContext {
    #[note("prompt-assembler-projectcontext-filtered")]
    pub fn filtered(self, form: bool, selection: bool) -> Self {
        match self {
            _ if !form => Self::None,
            Self::CodeEditor {
                file_path,
                language,
                live_content,
                ..
            } if !selection => Self::CodeEditor {
                file_path,
                language,
                live_content,
                selection: None,
            },
            Self::Designer {
                form_name,
                form_path,
                component_count,
                live_summary,
                ..
            } if !selection => Self::Designer {
                form_name,
                form_path,
                component_count,
                selected_components: Vec::new(),
                live_summary,
            },
            other => other,
        }
    }

    #[note("prompt-assembler-projectcontext-language-of")]
    pub fn language_of(path: &Path) -> String {
        match path.extension().and_then(|s| s.to_str()) {
            Some("rs") => "rust",
            Some("md") => "markdown",
            Some(ext) => ext,
            None => "text",
        }
        .to_string()
    }

    #[note("prompt-assembler-projectcontext-for-code-editor")]
    pub fn for_code_editor(
        path: &Path,
        root: Option<&Path>,
        live_content: String,
        selection: Option<EditorSelectionContext>,
    ) -> Self {
        let rel = match root {
            Some(r) => path.strip_prefix(r).unwrap_or(path),
            None => path,
        };
        let file_path = rel.to_string_lossy().replace('\\', "/");
        let language = Self::language_of(path);
        Self::CodeEditor {
            file_path,
            language,
            live_content,
            selection,
        }
    }

    #[note("prompt-assembler-projectcontext-for-designer")]
    pub fn for_designer(
        form_name: String,
        form_path: Option<&Path>,
        root: Option<&Path>,
        component_count: usize,
        selected_components: Vec<String>,
    ) -> Self {
        let form_path_str = form_path.map(|p| {
            let rel = match root {
                Some(r) => p.strip_prefix(r).unwrap_or(p),
                None => p,
            };
            rel.to_string_lossy().replace('\\', "/")
        });
        let live_summary = format!(
            "Form: {form_name}, Total Components: {component_count}, Selection: {selected_components:?}"
        );
        Self::Designer {
            form_name,
            form_path: form_path_str,
            component_count,
            selected_components,
            live_summary,
        }
    }
}

#[note("prompt-assembler-mentionedfile")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MentionedFile {
    /// Relative or display file path.
    pub path: String,
    /// Content of the file.
    pub content: String,
    /// Whether content was truncated due to budget constraints.
    pub is_truncated: bool,
    /// Original line count before truncation.
    pub total_lines: usize,
}

#[note("prompt-assembler-assembleparams")]
#[derive(Debug, Clone)]
pub struct AssembleParams<'a> {
    pub mode: PromptMode,
    pub global_memory: Option<&'a str>,
    pub project_context: &'a ProjectContext,
    pub user_query: &'a str,
    pub session_history: &'a [crate::ChatMessage],
    pub workspace_root: Option<&'a Path>,
    pub live_buffers: Option<&'a BTreeMap<PathBuf, String>>,
    pub max_bytes_per_file: usize,
    pub max_total_prompt_bytes: usize,
    pub attachments: &'a [crate::chat_attachment::SelectionAttachment],
}

impl Default for AssembleParams<'_> {
    fn default() -> Self {
        Self {
            mode: PromptMode::Agent,
            global_memory: None,
            project_context: &ProjectContext::None,
            user_query: "",
            session_history: &[],
            workspace_root: None,
            live_buffers: None,
            max_bytes_per_file: DEFAULT_MAX_BYTES_PER_FILE,
            max_total_prompt_bytes: DEFAULT_MAX_TOTAL_PROMPT_BYTES,
            attachments: &[],
        }
    }
}

#[note("prompt-assembler-truncate-text-on-line-boundary")]
pub fn truncate_text_on_line_boundary(text: &str, max_bytes: usize) -> (String, bool, usize) {
    let total_lines = text.lines().count().max(1);
    if text.len() <= max_bytes {
        return (text.to_string(), false, total_lines);
    }

    let mut byte_accum = 0;
    let mut kept_lines = Vec::new();

    for line in text.lines() {
        let line_len = line.len() + 1; // + 1 for newline
        if byte_accum + line_len > max_bytes {
            // If the first line alone already exceeds max_bytes, cut the line
            // at a UTF-8 boundary so the content is never empty.
            if kept_lines.is_empty() {
                let budget = max_bytes.min(line.len());
                let safe_slice: String = line.chars().take(budget).collect();
                kept_lines.push(&line[..safe_slice.len()]);
            }
            break;
        }
        kept_lines.push(line);
        byte_accum += line_len;
    }

    let truncated_content = kept_lines.join("\n");
    let notice = format!(
        "\n\n[... Truncated: showing first {} bytes of {} total lines ...]",
        truncated_content.len(),
        total_lines
    );
    let mut result = truncated_content;
    result.push_str(&notice);
    (result, true, total_lines)
}

#[note("prompt-assembler-extract-mention-paths")]
pub fn extract_mention_paths(text: &str) -> Vec<String> {
    let mut mentions = Vec::new();
    for token in text.split_whitespace() {
        if let Some(rest) = token.strip_prefix('@') {
            // Trim trailing punctuation like comma, semicolon, quotes or colon
            let clean = rest.trim_matches(|c| {
                c == ',' || c == ';' || c == ':' || c == '"' || c == '\'' || c == ')' || c == ']'
            });
            if !clean.is_empty()
                && (clean.contains('/') || clean.contains('\\') || clean.contains('.'))
                && !mentions.contains(&clean.to_string())
            {
                mentions.push(clean.to_string());
            }
        }
    }
    mentions
}

#[note("prompt-assembler-resolve-mentions")]
pub fn resolve_mentions(
    mention_paths: &[String],
    workspace_root: Option<&Path>,
    live_buffers: Option<&BTreeMap<PathBuf, String>>,
    max_bytes_per_file: usize,
) -> Vec<MentionedFile> {
    let mut result = Vec::new();

    for mention in mention_paths {
        // Security validation: prevents path traversal and access outside workspace.
        // Component-level check allows legitimate names like `@notes..v2.md` while rejecting `..`.
        let mention_path = Path::new(mention);
        if mention_path.is_absolute()
            || mention.starts_with('/')
            || mention.starts_with('\\')
            || mention_path
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            // Reject traversal attempts like @../../file or absolute paths
            continue;
        }

        // 1. Check live in-memory buffers first (buffer takes precedence over disk)
        let mut found_content: Option<String> = None;

        if let Some(buffers) = live_buffers {
            let mention_comps: Vec<String> = mention_path
                .components()
                .map(|c| {
                    c.as_os_str()
                        .to_string_lossy()
                        .replace('\\', "/")
                        .to_lowercase()
                })
                .collect();

            for (buf_path, content) in buffers {
                let buf_comps: Vec<String> = buf_path
                    .components()
                    .map(|c| {
                        c.as_os_str()
                            .to_string_lossy()
                            .replace('\\', "/")
                            .to_lowercase()
                    })
                    .collect();

                // Strict path components matching: e.g. `data.rs` does NOT match `a.rs`,
                // but `src/a.rs` matches `a.rs`.
                if buf_comps.len() >= mention_comps.len()
                    && buf_comps[(buf_comps.len() - mention_comps.len())..] == mention_comps[..]
                {
                    found_content = Some(content.clone());
                    break;
                }
            }
        }

        // 2. If not found in live memory buffers, read from disk relative to workspace
        if found_content.is_none()
            && let Some(root) = workspace_root
        {
            let candidate = root.join(mention_path);
            // Ensure the candidate exists and canonically resides inside workspace root
            if let (Ok(canon_root), Ok(canon_candidate)) =
                (root.canonicalize(), candidate.canonicalize())
                && canon_candidate.starts_with(&canon_root)
                && canon_candidate.is_file()
                && let Ok(content) = std::fs::read_to_string(&canon_candidate)
            {
                found_content = Some(content);
            }
        }

        if let Some(raw_content) = found_content {
            let (content, is_truncated, total_lines) =
                truncate_text_on_line_boundary(&raw_content, max_bytes_per_file);
            result.push(MentionedFile {
                path: mention.clone(),
                content,
                is_truncated,
                total_lines,
            });
        }
    }

    result
}

#[note("prompt-assembler-promptassembler")]
pub struct PromptAssembler;

impl PromptAssembler {
    #[note("prompt-assembler-promptassembler-assemble")]
    pub fn assemble(params: AssembleParams) -> String {
        let max_total_bytes = if params.max_total_prompt_bytes == 0 {
            DEFAULT_MAX_TOTAL_PROMPT_BYTES
        } else {
            params.max_total_prompt_bytes
        };
        let initial_max_per_file = if params.max_bytes_per_file == 0 {
            DEFAULT_MAX_BYTES_PER_FILE
        } else {
            params.max_bytes_per_file
        };

        // 1. Mandatory immutable parts: Preamble and User Request
        let preamble = match params.mode {
            PromptMode::Agent => {
                "### Autonomous Agent Guidelines\n\
                 You are an autonomous engineering agent working inside the CrawRAD IDE.\n\
                 Priority rules:\n\
                 - Use available IDE tools and actions first before falling back to generic shell commands.\n\
                 - Act directly and produce concrete code edits rather than passive long deliberations.\n\
                 - Respect existing architecture, conventions, and zero-warning build policies.\n\
                 - Directly execute or respond to the user request above. Never output mode confirmations."
                    .to_string()
            }
            PromptMode::Chat => {
                "### Context & Guidelines\n\
                 You are the CrawRAD AI Assistant helping the developer with code, design, and architecture.\n\
                 Always respond directly and helpfully to the user request above in the same language. Never echo system headers or mode confirmations."
                    .to_string()
            }
        };

        // An attachment is the selection the user asked about: the
        // live one, read at send time, stays out.
        let detached;
        let project_context = match params.project_context {
            ProjectContext::CodeEditor {
                file_path,
                language,
                live_content,
                selection: Some(_),
            } if !params.attachments.is_empty() => {
                detached = ProjectContext::CodeEditor {
                    file_path: file_path.clone(),
                    language: language.clone(),
                    live_content: live_content.clone(),
                    selection: None,
                };
                &detached
            }
            other => other,
        };
        let attached = crate::chat_attachment::render(params.attachments);
        let query = params.user_query.trim();
        let user_request = match (attached.is_empty(), query.is_empty()) {
            (true, _) => format!("### User Request\n{query}"),
            (false, true) => format!("### User Request\n{attached}"),
            (false, false) => format!("### User Request\n{attached}\n\n{query}"),
        };

        // 2. Global Permanent Instructions (Memory)
        let memory = params.global_memory.and_then(|mem| {
            let trimmed = mem.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(format!(
                    "### Global Permanent Instructions (CrawRAD Memory)\n{trimmed}"
                ))
            }
        });

        // 3. Count context blocks
        let has_project_context = match project_context {
            ProjectContext::None => false,
            ProjectContext::CodeEditor {
                live_content,
                selection,
                ..
            } => !live_content.trim().is_empty() || selection.is_some(),
            ProjectContext::Designer { live_summary, .. } => !live_summary.trim().is_empty(),
        };

        let mention_paths = extract_mention_paths(params.user_query);
        let num_context_blocks = (if has_project_context { 1 } else { 0 }) + mention_paths.len();

        // 4. Calculate budget for context blocks
        // Separator "\n\n---\n\n" is 9 bytes
        let fixed_overhead =
            preamble.len() + memory.as_ref().map_or(0, |m| m.len() + 9) + user_request.len() + 18;

        let per_block_budget = max_total_bytes
            .saturating_sub(fixed_overhead)
            .checked_div(num_context_blocks)
            .map(|b| b.max(128).min(initial_max_per_file))
            .unwrap_or(initial_max_per_file);

        // 5. Build context sections using per_block_budget
        let mut context_sections = Vec::new();

        match project_context {
            ProjectContext::None => {}
            ProjectContext::CodeEditor {
                file_path,
                language,
                live_content,
                selection,
            } => {
                let mut ctx_block = format!(
                    "### Active File Context\n- Path: {file_path}\n- Language: {language}\n"
                );
                if let Some(sel) = selection {
                    ctx_block.push_str(&format!(
                        "\n#### Selected Code (lines {}-{}):\n```{}\n{}\n```\n",
                        sel.start_line, sel.end_line, language, sel.selected_text
                    ));
                }
                if !live_content.trim().is_empty() {
                    let (truncated_code, _, _) =
                        truncate_text_on_line_boundary(live_content, per_block_budget);
                    ctx_block.push_str(&format!(
                        "\n#### Live Editor Buffer:\n```{}\n{}\n```",
                        language, truncated_code
                    ));
                }
                context_sections.push(ctx_block);
            }
            ProjectContext::Designer {
                form_name,
                form_path,
                component_count,
                selected_components,
                live_summary,
            } => {
                let mut dsg_block = format!(
                    "### Active Form Designer Context\n- Form Name: {form_name}\n- Component Count: {component_count}\n"
                );
                if let Some(path) = form_path {
                    dsg_block.push_str(&format!("- Form File: {path}\n"));
                }
                if !selected_components.is_empty() {
                    dsg_block.push_str(&format!(
                        "- Selected Components: {}\n",
                        selected_components.join(", ")
                    ));
                }
                if !live_summary.trim().is_empty() {
                    dsg_block.push_str(&format!("\n#### Form Layout Summary:\n{live_summary}"));
                }
                context_sections.push(dsg_block);
            }
        }

        let mentions = resolve_mentions(
            &mention_paths,
            params.workspace_root,
            params.live_buffers,
            per_block_budget,
        );

        if !mentions.is_empty() {
            let mut mention_block = "### Referenced Files (@mentions)\n".to_string();
            for file in mentions {
                mention_block.push_str(&format!(
                    "\n#### File: {}\n```\n{}\n```\n",
                    file.path, file.content
                ));
            }
            context_sections.push(mention_block.trim_end().to_string());
        }

        // 6. Assemble context string and apply safe strict byte truncation if necessary
        let context_str = context_sections.join("\n\n---\n\n");

        let mut fixed_sections = Vec::new();
        fixed_sections.push(preamble);
        if let Some(mem) = memory {
            fixed_sections.push(mem);
        }

        let pre_context_len: usize = fixed_sections.iter().map(|s| s.len() + 9).sum();
        let total_fixed_len = pre_context_len + user_request.len();

        let final_context = if context_str.is_empty() {
            String::new()
        } else {
            let max_context_bytes = max_total_bytes.saturating_sub(total_fixed_len + 9);
            if context_str.len() <= max_context_bytes {
                context_str
            } else {
                let notice = "\n\n[... Context truncated to fit prompt budget ...]";
                let target_cut = max_context_bytes.saturating_sub(notice.len());
                let mut cut = target_cut.min(context_str.len());
                while cut > 0 && !context_str.is_char_boundary(cut) {
                    cut -= 1;
                }
                format!("{}{notice}", &context_str[..cut])
            }
        };

        let mut all_sections = Vec::new();
        all_sections.push(user_request);
        all_sections.extend(fixed_sections);
        if !final_context.is_empty() {
            all_sections.push(final_context);
        }

        all_sections.join("\n\n---\n\n")
    }
}

#[note("prompt-assembler-extract-selected-text")]
pub fn extract_selected_text(
    texto: &str,
    start_line: usize,
    start_byte: usize,
    end_line: usize,
    end_byte: usize,
) -> String {
    if texto.is_empty() || start_line == 0 || end_line < start_line {
        return String::new();
    }
    let lines: Vec<&str> = texto.lines().collect();
    if lines.is_empty() {
        return String::new();
    }

    let start_line_idx = (start_line - 1).min(lines.len() - 1);
    let end_line_idx = (end_line - 1).min(lines.len() - 1);

    if start_line_idx == end_line_idx {
        let line = lines[start_line_idx];
        let sb = start_byte.min(line.len());
        let eb = end_byte.min(line.len());
        if sb <= eb {
            line[sb..eb].to_string()
        } else {
            String::new()
        }
    } else {
        let mut result = Vec::new();
        // First line
        let first_line = lines[start_line_idx];
        let sb = start_byte.min(first_line.len());
        result.push(first_line[sb..].to_string());

        // Intermediate lines
        if end_line_idx > start_line_idx + 1 {
            for line in &lines[(start_line_idx + 1)..end_line_idx] {
                result.push((*line).to_string());
            }
        }

        // Last line
        let last_line = lines[end_line_idx];
        let eb = end_byte.min(last_line.len());
        result.push(last_line[..eb].to_string());

        result.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn the_context_names_files_relative_to_the_root() {
        let root = std::path::Path::new("/w/proj");
        let file = std::path::Path::new("/w/proj/src/main.rs");
        let ctx = ProjectContext::for_code_editor(file, Some(root), "fn main() {}".into(), None);
        let ProjectContext::CodeEditor {
            file_path,
            language,
            ..
        } = ctx
        else {
            panic!("a code editor context");
        };
        assert_eq!(
            (file_path.as_str(), language.as_str()),
            ("src/main.rs", "rust")
        );
        let outside = ProjectContext::for_code_editor(file, None, String::new(), None);
        assert!(
            matches!(outside, ProjectContext::CodeEditor { ref file_path, .. } if file_path == "/w/proj/src/main.rs")
        );
        let form = std::path::Path::new("/w/proj/src/form_main.crawform");
        let d = ProjectContext::for_designer(
            "Main".into(),
            Some(form),
            Some(root),
            3,
            vec!["btn".into()],
        );
        let ProjectContext::Designer {
            form_path,
            live_summary,
            ..
        } = d
        else {
            panic!("a designer context");
        };
        assert_eq!(form_path.as_deref(), Some("src/form_main.crawform"));
        assert_eq!(
            live_summary,
            "Form: Main, Total Components: 3, Selection: [\"btn\"]"
        );
    }

    #[test]
    fn a_file_s_extension_names_its_language() {
        let of = |p: &str| super::ProjectContext::language_of(std::path::Path::new(p));
        assert_eq!(
            [
                of("a.rs"),
                of("b.md"),
                of("Cargo.toml"),
                of("x.json"),
                of("README")
            ],
            ["rust", "markdown", "toml", "json", "text"]
        );
    }

    #[test]
    fn test_fixed_order_and_headers_of_the_assembler() {
        let mem = "Strict Rust conventions: zero clippy warnings.";
        let code_ctx = ProjectContext::CodeEditor {
            file_path: "src/main.rs".to_string(),
            language: "rust".to_string(),
            live_content: "fn main() {\n    println!(\"Hello\");\n}".to_string(),
            selection: Some(EditorSelectionContext {
                start_line: 1,
                end_line: 3,
                selected_text: "fn main() {\n    println!(\"Hello\");\n}".to_string(),
            }),
        };

        let prompt = PromptAssembler::assemble(AssembleParams {
            mode: PromptMode::Agent,
            global_memory: Some(mem),
            project_context: &code_ctx,
            user_query: "Refactor main to print CrawRAD",
            session_history: &[],
            workspace_root: None,
            live_buffers: None,
            max_bytes_per_file: 32 * 1024,
            max_total_prompt_bytes: 32 * 1024,
            attachments: &[],
        });

        // Check the mandatory headers
        assert!(prompt.contains("### User Request"));
        assert!(prompt.contains("### Autonomous Agent Guidelines"));
        assert!(prompt.contains("### Global Permanent Instructions (CrawRAD Memory)"));
        assert!(prompt.contains("### Active File Context"));
        assert!(prompt.contains("#### Selected Code (lines 1-3):"));

        // Check that the history is NOT in the assembled prompt (continuity belongs to the agent's session)
        assert!(!prompt.contains("### Conversation History"));

        // Check that the relative order of the blocks is strictly enforced: User Request at the top!
        let idx_req = prompt.find("### User Request").unwrap();
        let idx_sys = prompt.find("### Autonomous Agent Guidelines").unwrap();
        let idx_mem = prompt.find("### Global Permanent Instructions").unwrap();
        let idx_file = prompt.find("### Active File Context").unwrap();
        let idx_sel = prompt.find("#### Selected Code").unwrap();

        assert!(idx_req < idx_sys);
        assert!(idx_sys < idx_mem);
        assert!(idx_mem < idx_file);
        assert!(idx_file < idx_sel);
    }

    #[test]
    fn test_live_buffer_wins_over_the_disk() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("live_buffer_test.rs");
        let _ = std::fs::write(&file_path, "TEXTO_GRAVADO_NO_DISCO");

        let mut live_buffers = BTreeMap::new();
        live_buffers.insert(
            file_path.clone(),
            "TEXTO_EM_MEMORIA_VIVA_NAO_SALVA".to_string(),
        );

        let mentions = resolve_mentions(
            &["live_buffer_test.rs".to_string()],
            Some(&temp_dir),
            Some(&live_buffers),
            32 * 1024,
        );

        assert_eq!(mentions.len(), 1);
        assert_eq!(mentions[0].content, "TEXTO_EM_MEMORIA_VIVA_NAO_SALVA");
        assert!(!mentions[0].content.contains("TEXTO_GRAVADO_NO_DISCO"));

        let _ = std::fs::remove_file(&file_path);
    }

    #[test]
    fn test_three_line_selection_comes_out_highlighted() {
        let full_text = "line 1: start\nline 2: alpha\nline 3: beta\nline 4: gamma\nline 5: end";
        let extracted = extract_selected_text(full_text, 2, 0, 4, 13);
        assert_eq!(extracted, "line 2: alpha\nline 3: beta\nline 4: gamma");

        let ctx = ProjectContext::CodeEditor {
            file_path: "crates/sample/src/lib.rs".to_string(),
            language: "rust".to_string(),
            live_content: full_text.to_string(),
            selection: Some(EditorSelectionContext {
                start_line: 2,
                end_line: 4,
                selected_text: extracted,
            }),
        };

        let prompt = PromptAssembler::assemble(AssembleParams {
            mode: PromptMode::Agent,
            global_memory: None,
            project_context: &ctx,
            user_query: "Explain this selection",
            session_history: &[],
            workspace_root: None,
            live_buffers: None,
            max_bytes_per_file: 32 * 1024,
            max_total_prompt_bytes: 32 * 1024,
            attachments: &[],
        });

        assert!(prompt.contains("#### Selected Code (lines 2-4):"));
        assert!(prompt.contains("line 2: alpha\nline 3: beta\nline 4: gamma"));
    }

    #[test]
    fn test_mentions_resolved_and_truncation_with_a_warning() {
        let temp_dir = std::env::temp_dir();
        let big_file = temp_dir.join("big_file_test.txt");

        // Build a file with 100 lines of 50 characters (~5 KB)
        {
            let mut f = std::fs::File::create(&big_file).unwrap();
            for i in 1..=100 {
                writeln!(
                    f,
                    "Line {:04}: This is some repetitive test payload line.",
                    i
                )
                .unwrap();
            }
        }

        let query = format!(
            "Analyze @{}",
            big_file.file_name().unwrap().to_str().unwrap()
        );
        let mention_paths = extract_mention_paths(&query);
        assert_eq!(mention_paths.len(), 1);

        // A budget of only 400 bytes
        let mentions = resolve_mentions(&mention_paths, Some(&temp_dir), None, 400);
        assert_eq!(mentions.len(), 1);
        assert!(mentions[0].is_truncated);
        assert!(
            mentions[0]
                .content
                .contains("[... Truncated: showing first")
        );

        let prompt = PromptAssembler::assemble(AssembleParams {
            mode: PromptMode::Agent,
            global_memory: None,
            project_context: &ProjectContext::None,
            user_query: &query,
            session_history: &[],
            workspace_root: Some(&temp_dir),
            live_buffers: None,
            max_bytes_per_file: 400,
            max_total_prompt_bytes: 32 * 1024,
            attachments: &[],
        });

        assert!(prompt.contains("### Referenced Files (@mentions)"));
        assert!(prompt.contains("[... Truncated: showing first"));

        let _ = std::fs::remove_file(&big_file);
    }

    #[test]
    fn test_memory_redaction_and_inclusion_in_the_prompt() {
        let temp_file = std::env::temp_dir().join("aefos_mem_test.json");
        let initial_mem =
            crate::AefosMemory::new("Always use strict typing and classic RAD semantics.");
        initial_mem.save_to_file(&temp_file).expect("save memory");

        let loaded = crate::AefosMemory::load_from_file(&temp_file).expect("load memory");
        assert_eq!(
            loaded.instructions,
            "Always use strict typing and classic RAD semantics."
        );

        let prompt = PromptAssembler::assemble(AssembleParams {
            mode: PromptMode::Agent,
            global_memory: Some(&loaded.instructions),
            project_context: &ProjectContext::None,
            user_query: "Generate Form1",
            session_history: &[],
            workspace_root: None,
            live_buffers: None,
            max_bytes_per_file: 32 * 1024,
            max_total_prompt_bytes: 32 * 1024,
            attachments: &[],
        });

        assert!(prompt.contains("### Global Permanent Instructions (CrawRAD Memory)"));
        assert!(prompt.contains("Always use strict typing and classic RAD semantics."));

        let _ = std::fs::remove_file(&temp_file);
    }

    #[test]
    fn open_40kb_file_respects_the_total_prompt_budget() {
        // Simulate an open 40 KB file
        let mut big_content = String::with_capacity(60 * 1024);
        for i in 1..=1200 {
            big_content.push_str(&format!(
                "Line {i:04}: let mut item_{i} = compute_value({i});\n"
            ));
        }
        assert!(big_content.len() >= 40 * 1024, "must be at least 40 KB");

        let ctx = ProjectContext::CodeEditor {
            file_path: "src/big_file.rs".to_string(),
            language: "rust".to_string(),
            live_content: big_content,
            selection: None,
        };

        let max_total = 28 * 1024;
        let prompt = PromptAssembler::assemble(AssembleParams {
            mode: PromptMode::Agent,
            global_memory: Some("Follow guidelines."),
            project_context: &ctx,
            user_query: "Review this entire file",
            session_history: &[],
            workspace_root: None,
            live_buffers: None,
            max_bytes_per_file: 16 * 1024,
            max_total_prompt_bytes: max_total,
            attachments: &[],
        });

        assert!(
            prompt.len() <= max_total,
            "assembled prompt ({} bytes) must respect the total limit of {max_total} bytes",
            prompt.len()
        );
        assert!(prompt.contains("### Autonomous Agent Guidelines"));
        assert!(prompt.contains("### User Request"));
    }

    #[test]
    fn path_traversal_attempt_in_a_mention_is_rejected() {
        let temp_dir = std::env::temp_dir();
        let sub_dir = temp_dir.join("crawrad_safe_project");
        let _ = std::fs::create_dir_all(&sub_dir);

        let outside_file = temp_dir.join("secret_outside.txt");
        let _ = std::fs::write(&outside_file, "SECRET_KEYS_NEVER_LEAK");

        // 1. Relative path traversal
        let mentions1 = resolve_mentions(
            &["../secret_outside.txt".to_string()],
            Some(&sub_dir),
            None,
            4096,
        );
        assert!(
            mentions1.is_empty(),
            "a relative traversal attempt must be rejected"
        );

        // 2. Absolute path
        let abs_str = outside_file.to_string_lossy().to_string();
        let mentions2 = resolve_mentions(&[abs_str], Some(&sub_dir), None, 4096);
        assert!(
            mentions2.is_empty(),
            "an absolute path outside the workspace must be rejected"
        );

        let _ = std::fs::remove_file(&outside_file);
        let _ = std::fs::remove_dir_all(&sub_dir);
    }

    #[test]
    fn mention_a_rs_does_not_match_data_rs_in_the_live_buffer() {
        let mut live_buffers = BTreeMap::new();
        live_buffers.insert(
            PathBuf::from("crates/crawrad-ide/src/data.rs"),
            "CONTENT_DATA_RS".to_string(),
        );
        live_buffers.insert(
            PathBuf::from("crates/crawrad-ide/src/a.rs"),
            "CONTENT_A_RS".to_string(),
        );

        // Mentioning @a.rs MUST match a.rs and NOT data.rs
        let m_a = resolve_mentions(&["a.rs".to_string()], None, Some(&live_buffers), 4096);
        assert_eq!(m_a.len(), 1);
        assert_eq!(m_a[0].content, "CONTENT_A_RS");

        // If only data.rs exists, @a.rs must NOT match through a false suffix
        let mut only_data = BTreeMap::new();
        only_data.insert(
            PathBuf::from("crates/crawrad-ide/src/data.rs"),
            "CONTENT_DATA_RS".to_string(),
        );
        let m_data_only = resolve_mentions(&["a.rs".to_string()], None, Some(&only_data), 4096);
        assert!(
            m_data_only.is_empty(),
            "the @a.rs mention must not match data.rs"
        );
    }

    #[test]
    fn a_single_giant_line_does_not_return_empty_content() {
        let single_big_line = "A".repeat(40 * 1024);
        let (content, is_trunc, lines) = truncate_text_on_line_boundary(&single_big_line, 400);

        assert!(is_trunc);
        assert_eq!(lines, 1);
        assert!(
            !content.is_empty(),
            "a single giant line must not produce empty content"
        );
        assert!(content.contains("[... Truncated: showing first 400 bytes of 1 total lines ...]"));
    }

    #[test]
    fn conversation_history_is_not_included_in_the_assembled_prompt() {
        let history = vec![
            crate::ChatMessage::user("1", "Turn 1 message"),
            crate::ChatMessage::assistant("2", "Resposta de turno 1"),
            crate::ChatMessage::user("3", "Turn 2 message"),
        ];

        let prompt = PromptAssembler::assemble(AssembleParams {
            mode: PromptMode::Agent,
            global_memory: None,
            project_context: &ProjectContext::None,
            user_query: "Turno 3 query",
            session_history: &history,
            workspace_root: None,
            live_buffers: None,
            max_bytes_per_file: 4096,
            max_total_prompt_bytes: 28 * 1024,
            attachments: &[],
        });

        assert!(
            !prompt.contains("### Conversation History"),
            "the prompt assembler must not resend the history as text"
        );
        assert!(
            !prompt.contains("Turn 1 message"),
            "old messages must not be pasted into the text prompt"
        );
    }

    #[test]
    fn a_mention_with_a_colon_in_the_name_is_allowed() {
        let mut live_buffers = BTreeMap::new();
        live_buffers.insert(
            PathBuf::from("docs/notas..v2.md"),
            "CONTEUDO_NOTAS_V2".to_string(),
        );

        let mentions = resolve_mentions(
            &["notas..v2.md".to_string()],
            None,
            Some(&live_buffers),
            4096,
        );
        assert_eq!(mentions.len(), 1);
        assert_eq!(mentions[0].content, "CONTEUDO_NOTAS_V2");
    }

    #[test]
    fn three_large_mentions_with_accented_text_keep_the_question_and_respect_the_budget() {
        // Build 3 in-memory buffers of ~20 KB of heavily accented text each
        let mut live_buffers = BTreeMap::new();
        for file in ["a.rs", "b.rs", "c.rs"] {
            let mut payload = String::with_capacity(25 * 1024);
            for i in 1..=400 {
                payload.push_str(&format!(
                    "Line {i:04}: multi-byte text → ✓ λ ∑ ∞ ≈ ≠ ← ↑ ↓ ⇒ ⇔ ∀ ∃ ∈ ∉ ⊂ ⊃ ∩ ∪ ⌘ ⌥.\n"
                ));
            }
            assert!(
                payload.len() >= 20 * 1024,
                "must be more than 20 KB of bytes"
            );
            live_buffers.insert(PathBuf::from(file), payload);
        }

        let query = "@a.rs @b.rs @c.rs USER_QUESTION compare the three files → now";
        let max_budget = 28 * 1024; // 28 KB

        let prompt = PromptAssembler::assemble(AssembleParams {
            mode: PromptMode::Agent,
            global_memory: Some("Permanent architecture guidelines."),
            project_context: &ProjectContext::None,
            user_query: query,
            session_history: &[],
            workspace_root: None,
            live_buffers: Some(&live_buffers),
            max_bytes_per_file: 16 * 1024,
            max_total_prompt_bytes: max_budget,
            attachments: &[],
        });

        // 1. The total budget in BYTES must be strictly respected
        assert!(
            prompt.len() <= max_budget,
            "Total prompt ({} bytes) must be <= {} bytes",
            prompt.len(),
            max_budget
        );

        // 2. The user's question and the ### User Request block can NEVER be truncated
        assert!(
            prompt.contains("### User Request"),
            "The ### User Request header must be present"
        );
        assert!(
            prompt.contains("USER_QUESTION compare the three files → now"),
            "The exact user question must be kept intact at the end of the prompt"
        );
        assert!(prompt.contains("### Autonomous Agent Guidelines"));
    }
}
