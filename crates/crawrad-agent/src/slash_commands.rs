use crawrad_note::note;
use serde::{Deserialize, Serialize};

#[note("slash-commands-slashcategory")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlashCategory {
    Tools,
    Assistants,
    System,
    Workflows,
}

impl SlashCategory {
    pub fn label(self) -> &'static str {
        match self {
            Self::Tools => "Tools & Utilities",
            Self::Assistants => "Code Assistants",
            Self::System => "System & Config",
            Self::Workflows => "Project Workflows",
        }
    }
}

#[note("slash-commands-slashcommand")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlashCommand {
    /// Command identifier (without slash, e.g. `"memory"`, `"mcp"`, `"explain"`).
    pub id: &'static str,
    /// Invocation trigger (e.g. `"/memory"`, `"/mcp"`).
    pub trigger: &'static str,
    /// Short summary of what the command does.
    pub description: &'static str,
    /// Group category for the command.
    pub category: SlashCategory,
}

impl SlashCommand {
    pub const ALL: &'static [Self] = &[
        Self {
            id: "memory",
            trigger: "/memory",
            description: "Open and edit global assistant instructions & memory",
            category: SlashCategory::System,
        },
        Self {
            id: "mcp",
            trigger: "/mcp",
            description: "Manage connected Model Context Protocol (MCP) servers",
            category: SlashCategory::Tools,
        },
        Self {
            id: "new",
            trigger: "/new",
            description: "Scaffold a new CrawRAD project from templates",
            category: SlashCategory::Workflows,
        },
        Self {
            id: "settings",
            trigger: "/settings",
            description: "Open the IDE options",
            category: SlashCategory::System,
        },
        Self {
            id: "acp",
            trigger: "/acp",
            description: "Show ACP status, test connections, or query agent",
            category: SlashCategory::System,
        },
        Self {
            id: "use",
            trigger: "/use",
            description: "Switch the chat to another connected ACP agent (no name: list them)",
            category: SlashCategory::System,
        },
        Self {
            id: "clear",
            trigger: "/clear",
            description: "Start a new conversation",
            category: SlashCategory::System,
        },
        Self {
            id: "actions",
            trigger: "/actions",
            description: "Open the ActionList editor of the active form",
            category: SlashCategory::Tools,
        },
        Self {
            id: "plan",
            trigger: "/plan",
            description: "Show agent implementation plan card and approval controls",
            category: SlashCategory::Workflows,
        },
        Self {
            id: "queue",
            trigger: "/queue",
            description: "Queue a prompt to send when the agent is free",
            category: SlashCategory::Workflows,
        },
        Self {
            id: "agent",
            trigger: "/agent",
            description: "Send the prompt in agent mode (the agent may use tools)",
            category: SlashCategory::Workflows,
        },
        Self {
            id: "find",
            trigger: "/find",
            description: "Semantic search across components, actions, and code",
            category: SlashCategory::Tools,
        },
        Self {
            id: "help",
            trigger: "/help",
            description: "List every slash command",
            category: SlashCategory::System,
        },
        Self {
            id: "explain",
            trigger: "/explain",
            description: "Explain the selected code or visual component",
            category: SlashCategory::Assistants,
        },
        Self {
            id: "refactor",
            trigger: "/refactor",
            description: "Refactor selected code or optimize component structure",
            category: SlashCategory::Assistants,
        },
        Self {
            id: "review",
            trigger: "/review",
            description: "Review the selected code: problems, risks and missing tests (read-only)",
            category: SlashCategory::Assistants,
        },
        Self {
            id: "test",
            trigger: "/test",
            description: "Generate #[test] unit tests for the active code (run with cargo test)",
            category: SlashCategory::Assistants,
        },
        Self {
            id: "docs",
            trigger: "/docs",
            description: "Generate documentation comments for active units",
            category: SlashCategory::Assistants,
        },
        Self {
            id: "teach",
            trigger: "/teach",
            description: "Teach the selected code line by line (ownership, borrows, types)",
            category: SlashCategory::Assistants,
        },
        Self {
            id: "why",
            trigger: "/why",
            description: "Explain why the compiler rejects the code on the caret's line",
            category: SlashCategory::Assistants,
        },
        Self {
            id: "fix-explain",
            trigger: "/fix-explain",
            description: "Fix the diagnostic on the caret's line and explain why",
            category: SlashCategory::Assistants,
        },
        Self {
            id: "compare",
            trigger: "/compare",
            description: "Compare two Rust choices, e.g. /compare Rc Arc",
            category: SlashCategory::Assistants,
        },
    ];

    #[note("slash-commands-slashcommand-filter")]
    pub fn filter(query: &str) -> Vec<Self> {
        let q = query.trim_start_matches('/').to_lowercase();
        if q.is_empty() {
            return Self::ALL.to_vec();
        }
        Self::ALL
            .iter()
            .filter(|cmd| {
                cmd.id.to_lowercase().contains(&q) || cmd.description.to_lowercase().contains(&q)
            })
            .copied()
            .collect()
    }

    #[note("slash-commands-slashcommand-match-exact")]
    pub fn match_exact(text: &str) -> Option<Self> {
        let first_word = text.split_whitespace().next()?;
        Self::ALL
            .iter()
            .find(|cmd| cmd.trigger.eq_ignore_ascii_case(first_word))
            .copied()
    }
}

#[note("slash-commands-use-targets")]
pub fn use_targets() -> Vec<(&'static str, &'static str)> {
    crate::agent_profile::PROFILES
        .iter()
        .map(|p| (p.id, p.display_name))
        .collect()
}

#[note("slash-commands-resolve-use")]
pub fn resolve_use(name: &str) -> Option<&'static str> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return None;
    }
    crate::agent_profile::find_profile_by_id(trimmed)
        .or_else(|| crate::agent_profile::find_profile_by_display_name(trimmed))
        .or_else(|| crate::agent_profile::find_profile_by_binary(trimmed))
        .map(|p| p.display_name)
}

#[note("slash-commands-review-prompt")]
const REVIEW_PROMPT: &str = "Review the attached code (or the selected code / active file). Point out correctness problems, risks and missing tests, citing line numbers. This is a read-only review: do NOT edit, create or delete any file; only report.";

#[note("slash-commands-build-command-prompt")]
pub fn build_command_prompt(cmd: &str, argument: &str) -> Option<String> {
    let rest = argument.trim();
    match cmd {
        "/explain" => Some(if rest.is_empty() {
            "Explain the selected code, active file, or component structure.".to_string()
        } else {
            format!("Explain the following code/context with focus on: {rest}")
        }),
        "/refactor" => Some(if rest.is_empty() {
            "Suggest refactoring improvements and optimizations for the selected code or active component.".to_string()
        } else {
            format!("Refactor and optimize the selected code or context according to: {rest}")
        }),
        "/review" => Some(if rest.is_empty() {
            REVIEW_PROMPT.to_string()
        } else {
            format!("{REVIEW_PROMPT} Focus on: {rest}")
        }),
        "/test" => Some(if rest.is_empty() {
            "Generate Rust #[test] unit tests for the active code, in a #[cfg(test)] mod tests, runnable with cargo test.".to_string()
        } else {
            format!(
                "Generate Rust #[test] unit tests (a #[cfg(test)] mod tests, runnable with cargo test) for: {rest}"
            )
        }),
        "/docs" => Some(if rest.is_empty() {
            "Generate documentation comments and explanations for the active code.".to_string()
        } else {
            format!("Generate documentation for: {rest}")
        }),
        _ => None,
    }
}

#[note("slash-commands-format-acp-status")]
pub fn format_acp_status(acp: &crate::acp::AcpConfig) -> String {
    let provider = &acp.provider;
    let model = &acp.model;
    let endpoint = &acp.endpoint;
    let think = if acp.enable_thinking {
        "Enabled"
    } else {
        "Disabled"
    };
    let auto = if acp.auto_approve_reads {
        "Enabled"
    } else {
        "Disabled"
    };
    format!(
        "🤖 **Agent Client Protocol (ACP) Status**\n\n\
        • **Provider**: {provider}\n\
        • **Default Model**: {model}\n\
        • **Endpoint**: `{endpoint}`\n\
        • **Extended Thinking**: {think}\n\
        • **Auto-Approve Reads**: {auto}\n\
        • **Connection**: 🟢 Connected & Ready\n\n\
        💡 Commands:\n\
        • `/acp test` — Test live probe to configured agent\n\
        • `/acp ask <prompt>` — Execute query with active ACP agent\n\
        • `/use <agent>` — Switch the active ACP agent\n\
        • `/settings` — Type `/settings` or click ⚙ to configure in Tools -> Options."
    )
}

#[note("slash-commands-acp-probe")]
pub fn acp_probe(target: &str, configured: &crate::acp::AcpConfig) -> Result<String, String> {
    if target.is_empty() {
        configured.test_connection()
    } else if let Some(provider_name) = resolve_use(target) {
        crate::acp::AcpConfig::test_connection_for(provider_name, "")
    } else {
        configured.test_connection()
    }
}

#[note("slash-commands-auto-approve-labels")]
pub fn auto_approve_labels(on: bool) -> (&'static str, &'static str) {
    if on {
        ("ENABLED (ON)", "Auto-approve: ON")
    } else {
        ("DISABLED (OFF)", "Auto-approve: OFF")
    }
}

#[note("slash-commands-plan-status-label")]
pub fn plan_status_label(auto_approve: bool) -> &'static str {
    if auto_approve {
        "🟢 Auto-Approved"
    } else {
        "Pending Review"
    }
}

#[note("slash-commands-plan-prompt-for")]
pub fn plan_prompt_for(objective: &str) -> String {
    format!("Create a step-by-step implementation plan for: {objective}")
}

#[note("slash-commands-plan-usage")]
pub fn plan_usage() -> &'static str {
    "💡 Usage: `/plan <objective>` to plan a task, or `/plan auto` to toggle auto-approval."
}

#[note("slash-commands-plan-auto-toggled-message")]
pub fn plan_auto_toggled_message(state: &str) -> String {
    format!("⚡ ACP Auto-Approve Plans is now {state}.")
}

#[note("slash-commands-acp-probing-message")]
pub fn acp_probing_message() -> &'static str {
    "⏳ Probing ACP agent connection in background..."
}

#[note("slash-commands-acp-ask-usage")]
pub fn acp_ask_usage() -> &'static str {
    "⚠️ Usage: `/acp ask <your prompt>`"
}

#[note("slash-commands-acp-commands-help")]
pub fn acp_commands_help() -> &'static str {
    "💡 ACP commands: `/acp`, `/acp test`, `/acp ask <prompt>`"
}

#[note("slash-commands-help-text")]
pub fn help_text() -> String {
    let mut help = String::from("📋 Available Slash Commands:\n\n");
    for item in SlashCommand::ALL {
        help.push_str(&format!("• `{}` — {}\n", item.trigger, item.description));
    }
    help
}

#[note("slash-commands-use-reply")]
pub fn use_reply(rest: &str, provider: &str, agent_id: &str) -> (Option<&'static str>, String) {
    if rest.trim().is_empty() {
        let mut out = String::from("🤖 **Available ACP agents:**\n\n");
        for (id, display) in use_targets() {
            let active =
                provider.eq_ignore_ascii_case(display) || agent_id.eq_ignore_ascii_case(id);
            let mark = if active { " (active)" } else { "" };
            out.push_str(&format!("• `{id}` — {display}{mark}\n"));
        }
        out.push_str("\n💡 `/use <agent>` switches.");
        return (None, out);
    }
    match resolve_use(rest) {
        Some(p) => (Some(p), format!("💡 Switched the chat to {p}.")),
        None => (
            None,
            format!(
                "⚠️ Unknown agent `{}`. Try `/use` to list them.",
                rest.trim()
            ),
        ),
    }
}

#[cfg(test)]
#[path = "test/slash_commands/test.rs"]
mod tests;
