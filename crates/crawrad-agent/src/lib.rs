#![forbid(unsafe_code)]

pub mod acp;
pub mod acp_diagnostic;
pub mod acp_live;
pub mod acp_registry;
pub mod acp_status;
pub mod acp_turn;
pub mod acp_update;
pub mod agent_profile;
pub mod chat_attachment;
pub mod chat_config;
pub mod chat_history;
pub mod chat_layout;
pub mod client;
pub mod diff_review;
pub mod edit_ask;
pub mod edit_review;
pub mod file_write;
pub mod markdown;
pub mod mcp;
pub mod memory;
pub mod permission;
pub mod prompt_assembler;
pub mod session;
pub mod slash_commands;
pub mod teacher;

pub use client::{AcpClient, AcpStreamEvent, ide_mcp_server};
pub use diff_review::{
    DiffHunk, DiffReview, DiffReviewManager, DiffReviewMode, HunkStatus, ReviewStatus,
    compute_diff_hunks,
};
pub use edit_ask::{AgentEditAsks, ProposedEdit};
pub use file_write::{AgentWrites, FileWriteRequest};
pub use markdown::{ChatContentBlock, ToolCallRecord, ToolCallStatus, parse_chat_markdown};

pub use permission::{
    OperationKind, PermissionDecision, PermissionManager, PermissionOption, PermissionOptionKind,
    PermissionQueue, PermissionRequest, SessionPermissions, classify_tool,
    global as permission_manager, is_read_only_tool,
};

pub use acp::{
    AcpConfig, AcpModelInfo, CliInvocation, PromptDelivery, SessionSupport,
    build_cli_args_for_session, build_cli_invocation_with_options, execute_invocation,
    format_model_label_pure,
};
pub use acp_diagnostic::{
    CheckStatus, DiagnosticAction, DiagnosticCheck, DiagnosticHooks, DiagnosticReport,
    DiagnosticStep, run_diagnostic,
};
pub use acp_live::{LiveAgent, LiveStatus};
pub use acp_registry::{
    AcpAgentEntry, AcpLaunch, AgentState, agent_card, agent_state, agents_dir, chat_status_line,
    current_platform, install_binary_agent, launch_for, registry_agent, search_registry,
    spawn_background_refresh,
};
pub use agent_profile::{
    AgentProfile, CLAUDE_MODELS, GEMINI_AGY_MODELS, ModelSource, ModelSpec, OPENAI_MODELS,
    OPENCODE_MODELS, PROFILES, StaticModelSpec, build_agent_args, build_agent_invocation_args,
    find_profile_by_display_name, find_profile_by_id, query_agent_models, resolve_binary_path,
    resolve_profile,
};
pub use chat_attachment::{
    ImageAttachment, MAX_ATTACHMENTS, SelectionAttachment, images_dir, paste_image, prompt_blocks,
    remove_chip,
};
pub use chat_config::{ConfigMenu, MenuItem, button_label, menu_for};
pub use mcp::{McpRegistry, McpServerConfig, McpSource, McpTransport};
pub use memory::AefosMemory;
pub use prompt_assembler::{
    AssembleParams, EditorSelectionContext, MentionedFile, ProjectContext, PromptAssembler,
    PromptMode, extract_mention_paths, truncate_text_on_line_boundary,
};
pub use session::{
    ChatMessage, ChatSession, MessageSender, MessageStatus, PlanStatus, QueuedMessage, TurnOutcome,
};
pub use slash_commands::{SlashCategory, SlashCommand};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slash_commands_filtering_and_matching() {
        let all = SlashCommand::filter("");
        assert!(all.len() >= 10);

        let mem_match = SlashCommand::filter("mem");
        assert_eq!(mem_match.len(), 1);
        assert_eq!(mem_match[0].id, "memory");

        let exact = SlashCommand::match_exact("/mcp");
        assert!(exact.is_some());
        assert_eq!(exact.unwrap().id, "mcp");

        assert!(SlashCommand::match_exact("/inexistente").is_none());
    }

    /// Each box of the AI page writes the field it reads.
    #[test]
    fn an_acp_flag_reads_what_it_writes() {
        for flag in acp::AcpFlag::ALL {
            for on in [false, true] {
                let mut c = AcpConfig::default();
                *flag.field(&mut c) = on;
                assert_eq!(flag.get(&c), on, "{flag:?}");
            }
        }
    }

    #[test]
    fn memory_character_and_token_heuristics() {
        let mem = AefosMemory::new("Use idiomatic Rust and strict types.");
        assert_eq!(mem.character_count(), 36);
        assert_eq!(mem.estimated_tokens(), 9);
    }

    #[test]
    fn mcp_registry_defaults_and_custom_addition() {
        let mut reg = McpRegistry::new();
        assert_eq!(reg.servers.len(), 2);

        reg.add_server(McpServerConfig {
            name: "postgres-db".into(),
            transport: McpTransport::Stdio,
            command: "pg-mcp".into(),
            args: vec![],
            env: std::collections::HashMap::new(),
            source: McpSource::Custom,
            enabled: true,
        });

        assert_eq!(reg.servers.len(), 3);
        reg.remove_server("postgres-db");
        assert_eq!(reg.servers.len(), 2);
    }

    #[test]
    fn chat_session_lifecycle() {
        let mut sess = ChatSession::default();
        assert!(sess.is_empty());

        sess.add_message(ChatMessage::user("msg-1", "Explain this form"));
        assert_eq!(sess.messages.len(), 1);
        assert_eq!(sess.messages[0].sender, MessageSender::User);

        sess.add_message(ChatMessage::thinking("msg-2", "Analyzing components..."));
        assert_eq!(sess.messages.len(), 2);
        assert_eq!(sess.messages[1].status, MessageStatus::Thinking);

        sess.clear();
        assert!(sess.is_empty());
    }

    #[test]
    fn acp_config_defaults_and_serialization() {
        let mut acp = AcpConfig::default();
        assert_eq!(acp.provider, "Claude Agent");
        assert_eq!(acp.model, "auto");
        assert!(acp.auto_approve_reads);
        assert!(!acp.auto_approve_plans);
        assert!(acp.enable_thinking);

        acp.provider = "ACP Stdio Agent".into();
        acp.model = "claude-sonnet-4-6".into();
        let json = serde_json::to_string(&acp).unwrap();
        let decoded: AcpConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.provider, "ACP Stdio Agent");
        assert_eq!(decoded.model, "claude-sonnet-4-6");
    }

    #[test]
    fn acp_catalogs_and_connection_probes() {
        assert!(AcpConfig::PROVIDERS.contains(&"Claude Agent"));
        assert!(AcpConfig::PROVIDERS.contains(&"Gemini (Antigravity)"));
        assert!(AcpConfig::MODELS.contains(&"gemini-3.8-flash-high"));
        assert!(AcpConfig::MODELS.contains(&"sonnet"));

        // An imaginary or non-existent binary returns Err (honest probe, no false positives)
        let nonexistent_res =
            AcpConfig::test_connection_for("Custom ACP Server", "stdio:custom_nonexistent_binary");
        assert!(nonexistent_res.is_err());
        assert!(
            nonexistent_res
                .unwrap_err()
                .contains("not found or not in PATH")
        );

        let empty_res = AcpConfig::test_connection_for("Custom ACP Server", "   ");
        assert!(empty_res.is_err());

        // A valid installed binary returns Ok with probe output
        let cargo_res = AcpConfig::test_connection_for("Custom ACP Server", "cargo");
        assert!(cargo_res.is_ok());
        assert!(
            cargo_res
                .unwrap()
                .contains("🟢 Connected to `Custom ACP Server`")
        );
    }

    #[test]
    fn acp_new_slash_commands_present() {
        assert!(SlashCommand::match_exact("/acp").is_some());
        assert!(SlashCommand::match_exact("/use").is_some());
        assert!(SlashCommand::match_exact("/teach").is_some());
    }
}

#[cfg(test)]
#[path = "test/acp_client_test.rs"]
mod acp_client_test;

#[cfg(test)]
#[path = "test/permission_dialog_test.rs"]
mod permission_dialog_test;

#[cfg(test)]
extern crate self as crawrad_agent;

#[cfg(test)]
#[path = "bin/acp_check.rs"]
pub mod acp_check;
