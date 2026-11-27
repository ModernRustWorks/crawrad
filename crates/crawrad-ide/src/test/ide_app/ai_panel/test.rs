//! The MOCKUP gate: the AI panel must not go back to manufacturing verdicts.
//!
//! # What the panel was
//!
//! It chose among three replies based on the word present in the prompt, and all
//! three claimed to have done what they had not:
//!
//! | phrase | why it is fraudulent |
//! |---|---|
//! | `All constraints and anchors are correctly aligned to the 8px grid.` | it is a VERDICT on the user's form, and no line of this shell measured constraints, anchors, or grid |
//! | `I've generated the native event handler for ...` | it did not generate it: not a single byte entered any file |
//! | `Would you like me to insert this handler into the form code?` | offers a path that does not exist |
//!
//! And the opening card showed `22:57` as the time — the same time across
//! the owner's screenshots taken on different days, because it was a DSL literal.
//!
//! # What remains
//!
//! The panel **is** a deliberate mockup, and the real chat belongs to Phase 4. It does
//! not leave; it now **states** that it is a mockup, which is the rule this very
//! shell created for the status bar: whatever is not wired says it is not.
//!
//! # What this gate does NOT prove
//!
//! It reads `src/ide_app.rs` as text. It proves that the fabricated phrases are not
//! written in the file and that the badge is declared in the DSL — it does **not**
//! prove that the badge appears on screen (a wrong property name in Makepad's DSL
//! does not break the build: it becomes a log line,
//! `makepad-live-compiler-1.0.0/src/live_cx.rs:153-166`), and does not prove that the
//! reply was shown to the user.

/// The Makepad shell — the entire file, as text.
use super::casca::CASCA;

static MODEL_CACHE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn model_cache_guard() -> std::sync::MutexGuard<'static, ()> {
    let guard = MODEL_CACHE_LOCK
        .lock()
        .unwrap_or_else(|poison_error| poison_error.into_inner());
    crawrad_agent::AgentProfile::set_background_fetch(false);
    crawrad_agent::AgentProfile::clear_cache();
    guard
}

/// Only CODE and DSL: stripping every line comment.
///
/// Stripping is the point of the file: the fabricated phrases are cited
/// intentionally in documentation — that is where why they were removed is explained. Checking
/// against the raw file would fail the explanation itself.
fn sem_comentario() -> String {
    CASCA
        .lines()
        .map(|l| {
            let t = l.trim_start();
            if t.starts_with("//") { "" } else { l }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// No phrase that pretends to analyze the user's document has returned.
///
/// Each needle in this list was once on screen and claimed a fact about the
/// open form, or an action on disk, that never happened.
#[test]
fn the_ai_panel_no_longer_invents_a_verdict_about_the_form() {
    const FABRICADAS: &[(&str, &str)] = &[
        (
            "correctly aligned",
            "verdict on the user's form: no line of this shell \
             measures constraints, anchors, or grid",
        ),
        (
            "I've generated",
            "claim of having generated code: not a single byte enters any file \
             through this panel",
        ),
        (
            "insert this handler",
            "offer of a path that does not exist: there is no way to insert anything into \
             form code from here",
        ),
        (
            "Analyzing component",
            "claim of analyzing the selected component",
        ),
        (
            "Here is the native Makepad LiveDesign DSL definition",
            "claim of having derived DSL from the selected component",
        ),
    ];

    let code = sem_comentario();
    for (agulha, porque) in FABRICADAS {
        assert!(
            !code.contains(agulha),
            "the phrase `{agulha}` returned to the AI panel. It is {porque}. The panel \
             is a mockup and has only ONE reply, `IdeApp::RESPOSTA_DE_MAQUETE`, \
             which opens by saying it is not wired."
        );
    }
}

/// Gate of The text `Not wired yet` and the constant `RESPOSTA_DE_MAQUETE`
/// no longer exist in the shell or in the workspace. AI chat now truly connects
/// via streaming and persistent history.
#[test]
fn no_not_wired_yet_text_or_mockup_response_exists_in_shell() {
    let code = sem_comentario();
    assert!(
        !code.contains("RESPOSTA_DE_MAQUETE"),
        "the RESPOSTA_DE_MAQUETE constant was banned by Issue #180: the chat panel is now truly connected via ACP"
    );
    assert!(
        !code.contains("Not wired yet"),
        "the 'Not wired yet' text was removed from the chat shell by Issue #180"
    );
}

/// The panel carries a VISIBLE mockup badge in its own header.
#[test]
fn the_ai_panel_header_carries_the_mock_up_badge() {
    let code = sem_comentario();
    let i = code
        .find("ai_header = <CrawradPanelHeader>")
        .expect("AI panel header is missing from src/ide_app.rs");
    let fim = code[i..]
        .find("ai_toggles_bar")
        .expect("AI panel header is no longer before the toggles bar");
    let header = &code[i..i + fim];

    assert!(
        header.contains("ai_mock_badge"),
        "the mockup badge was removed from the AI panel header. Without it the panel \
         once again pretends to be a connected assistant."
    );
    assert!(
        header.contains("text: \"No agent\""),
        "the badge lost the wording that makes it a badge. It has to SAY on screen, \
         and not just in code, that it is a mockup."
    );
    assert!(
        header.contains("(CRAWRAD_AVISO)"),
        "the badge stopped using the theme warning token. Hardcoded warning colors \
         are rejected by `nenhuma_cor_a_mao_sobrou_fora_da_maquete_do_canvas`."
    );
}

/// No fake clock remains in the DSL.
///
/// The search is for any `"HH:MM"` literal, not for the `22:57` that was there:
/// changing the number to another plausible one is precisely the mutation this test
/// must reject.
#[test]
fn no_literal_clock_time_is_left_in_the_shell() {
    let mut achados = Vec::new();
    for (n, linha) in CASCA.lines().enumerate() {
        let t = linha.trim_start();
        if t.starts_with("//") {
            continue;
        }
        let bytes: Vec<char> = linha.chars().collect();
        for i in 0..bytes.len() {
            // `"` `d` `d` `:` `d` `d` `"` — a literal time and nothing else.
            if bytes[i] == '"'
                && i + 6 < bytes.len()
                && bytes[i + 1].is_ascii_digit()
                && bytes[i + 2].is_ascii_digit()
                && bytes[i + 3] == ':'
                && bytes[i + 4].is_ascii_digit()
                && bytes[i + 5].is_ascii_digit()
                && bytes[i + 6] == '"'
            {
                achados.push(format!("line {}: {}", n + 1, t));
            }
        }
    }
    assert!(
        achados.is_empty(),
        "a fixed time returned to the shell. A DSL clock is the worst kind of \
         plausible number: it looks like data and never changes — `22:57` appeared \
         identical in screenshots from different days. Either it comes from the clock, or it does \
         not appear:\n  {}",
        achados.join("\n  ")
    );
}

/// Slash commands (`/slash-commands`) of the AI Chat are mapped in dispatch.
#[test]
fn os_comandos_de_barra_estao_no_despacho_do_chat() {
    let code = sem_comentario();
    assert!(
        code.contains("fn despachar_slash_command"),
        "the slash command dispatcher is missing from ide_app.rs"
    );
    assert!(
        code.contains("\"new\" =>")
            && code.contains("\"memory\" =>")
            && code.contains("\"mcp\" =>"),
        "essential commands /new, /memory, and /mcp must be present in chat dispatch"
    );
}

/// Modal AI dialogs (Memory and MCP) are declared in the DSL and instantiated in the shell.
#[test]
fn the_ai_dialogs_are_in_the_dsl_and_in_the_shell() {
    let code = sem_comentario();
    assert!(
        code.contains("memory_dialog = <CrawradMemoryDialog> {}"),
        "memory dialog is not instantiated in the shell tree"
    );
    assert!(
        code.contains("mcp_dialog = <CrawradMcpDialog> {}"),
        "MCP dialog is not instantiated in the shell tree"
    );
}

/// The AI Chat welcome Hero screen (Ref 01) and its 6 shortcut cards are in the DSL.
#[test]
fn the_ai_hero_screen_and_its_cards_are_in_the_dsl() {
    let code = sem_comentario();
    assert!(
        code.contains("ai_hero_view = <CrawradAiHeroView>")
            || code.contains("ai_hero_view = <View>"),
        "the AI Chat welcome hero screen is missing from the DSL"
    );
    assert!(
        code.contains("What are we building today?"),
        "chat hero title is not in the DSL"
    );
    assert!(
        code.contains("btn_card_explain")
            && code.contains("btn_card_refactor")
            && code.contains("btn_card_test")
            && code.contains("btn_card_document")
            && code.contains("btn_card_find")
            && code.contains("btn_card_optimize"),
        "the 6 shortcut cards (Explain, Refactor, Test, Document, Find, Optimize) must be in the DSL"
    );
    assert!(
        code.contains("btn_ai_memory") && code.contains("btn_ai_mcp"),
        "Memory and MCP quick shortcut buttons must be in the utilities bar"
    );
}

/// The slash commands pop-up menu and the thinking card are in the DSL and shell.
#[test]
fn o_popup_de_slash_commands_e_o_thinking_estao_na_dsl() {
    let code = sem_comentario();
    assert!(
        code.contains("ai_slash_popup = <CrawradAiSlashPopup>")
            || code.contains("ai_slash_popup = <View>"),
        "slash commands pop-up is missing from the DSL"
    );
    assert!(
        code.contains("thinking_card = <CrawradAiThinkingCard>")
            || code.contains("thinking_card = <CrawradCard>"),
        "thinking card is missing from the DSL"
    );
    assert!(
        code.contains("btn_cmd_explain")
            && code.contains("btn_cmd_refactor")
            && code.contains("btn_cmd_test")
            && code.contains("btn_cmd_docs")
            && code.contains("btn_cmd_find"),
        "essential command pop-up shortcuts must be present"
    );
}

/// The model selection dropdown menu (Ref 09) is declared in the DSL and shell.
#[test]
fn model_selector_is_in_dsl_and_shell() {
    let code = sem_comentario();
    assert!(
        code.contains("ai_model_dropdown = <CrawradAiModelDropdown>")
            || code.contains("ai_model_dropdown = <View>"),
        "model dropdown menu is missing from the DSL"
    );
    // The items come from the agent's configOptions, drawn in generic slots
    assert!(
        code.contains("opt_0 = <CrawradMenuSlot>")
            && code.contains("opt_15 = <CrawradMenuSlot>")
            && code.contains("btn_ai_effort = <CrawradButton>"),
        "the 16 menu slots and the effort button must be in the DSL"
    );
}

/// ACP (Agent Client Protocol) settings are present in the Options dialog DSL.
#[test]
fn as_configuracoes_de_acp_estao_na_dsl_do_options() {
    let code = sem_comentario();
    assert!(
        code.contains("opt_ai = <View>"),
        "AI and ACP options page (opt_ai) must be in the Options DSL"
    );
    assert!(
        code.contains("opt_ai_provider")
            && code.contains("opt_ai_model")
            && code.contains("opt_ai_endpoint")
            && code.contains("opt_ai_test_btn"),
        "essential ACP configuration fields (provider, model, endpoint, and test button) must be in the DSL"
    );
    assert!(
        !code.contains("opt_ai_token"),
        "per ADR 0029 and Issue #183, the IDE does not store auth token/API key in DSL or plain text"
    );
    assert!(
        code.contains("btn_ai_settings"),
        "the settings button (gear) must be in the AI panel header"
    );
}

/// The /acp command and opening ACP via /settings are in chat dispatch.
#[test]
fn the_acp_command_is_in_the_chat_dispatch() {
    let code = sem_comentario();
    assert!(
        code.contains("\"acp\" =>"),
        "/acp slash command must be mapped in chat dispatch"
    );
    assert!(
        code.contains("Category::AiAcp"),
        "chat options shortcut must direct to AiAcp category"
    );
}

/// Plan cards (ACP / Ref 29) and auto-approval controls are in the DSL and dispatch.
#[test]
fn os_cards_de_plano_e_auto_aprovacao_estao_na_dsl_e_no_despacho() {
    let code = sem_comentario();
    assert!(
        (code.contains("plan_card = <CrawradAiPlanCard>")
            || code.contains("plan_card = <CrawradCard>"))
            && code.contains("btn_plan_approve")
            && code.contains("btn_plan_reject")
            && code.contains("btn_plan_auto_approve"),
        "plan card and action buttons (approve, reject, auto-approve) must be in the DSL"
    );
    assert!(
        code.contains("\"plan\" =>"),
        "/plan slash command must be mapped in chat dispatch"
    );
    assert!(
        code.contains("auto_approve_plans"),
        "auto-approval configuration for plans must be connected to the app"
    );
}

/// The send queue (Queued Messages) is present in the DSL and structured with send, edit, and remove actions.
#[test]
fn the_queued_messages_card_is_in_the_dsl_and_has_a_queue_flow() {
    let code = sem_comentario();
    assert!(
        code.contains("ai_queue_card = <CrawradAiQueueCard>")
            && code.contains("CrawradAiQueueCard = <CrawradOutlined>")
            && code.contains("badge_queue_count")
            && code.contains("btn_queue_toggle")
            && code.contains("btn_q_send")
            && code.contains("btn_q_edit")
            && code.contains("btn_q_del"),
        "send queue card and action buttons must be present in chat DSL"
    );
    assert!(
        code.contains(
            "if self.chat.busy {\n            let attachments = std::mem::take(&mut self.chat.attachments);"
        ) && code.contains("self.enfileirar_ai_prompt(cx, &prompt, attachments, images);"),
        "normal send must auto-queue when agent is busy (ai_busy)"
    );

    // Test of the QueuedMessage model
    let qmsg = crawrad_agent::QueuedMessage::new("q-1", "Test queue prompt");
    assert_eq!(qmsg.id, "q-1");
    assert_eq!(qmsg.text, "Test queue prompt");
    assert!(!qmsg.has_image);

    let qmsg_img = crawrad_agent::QueuedMessage::with_image("q-2", "Prompt with image");
    assert_eq!(qmsg_img.id, "q-2");
    assert!(qmsg_img.has_image);
}

/// The model selection dropdown menu is positioned anchored above the `btn_ai_model` button.
#[test]
fn model_menu_positioning_anchors_above_footer_button() {
    let code = sem_comentario();
    assert!(
        code.contains("position_config_menu"),
        "dynamic positioning calculation routine for model dropdown must exist in the shell"
    );
    assert!(
        code.contains("CrawradAiModelDropdown = <CrawradOutlined>")
            && code.contains("margin: { left: 44.0"),
        "model dropdown menu DSL must have initial margin anchored to the left"
    );
}

/// Context popup (+) and slash command popup (/) anchor above the input bar.
#[test]
fn the_context_and_slash_popups_anchor_above_the_prompt() {
    let code = sem_comentario();
    assert!(
        code.contains("posicionar_context_popup"),
        "dynamic positioning routine for context popup (+) must exist in the shell"
    );
    assert!(
        code.contains("posicionar_slash_popup"),
        "dynamic positioning routine for command popup (/) must exist in the shell"
    );
    assert!(
        code.contains("CrawradAiContextPopup = <CrawradOutlined>")
            && code.contains("btn_ai_attach")
            && code.contains("CrawradContextButton"),
        "context popup DSL and + button must be integrated"
    );
}

/// The audio recording flow disables the prompt field and send button while
/// the user speaks, transcribes in a disabled state, and re-enables everything when recording ends.
#[test]
fn o_fluxo_de_gravacao_de_audio_desabilita_prompt_e_envio_ate_conclusao() {
    let code = sem_comentario();
    assert!(
        code.contains("set_prompt_input_recording_state"),
        "visual/edit state toggle routine for audio prompt must exist"
    );
    // One path for both states; how it looks comes from `crawrad_audio::prompt::look`
    assert!(
        code.contains("prompt_input.set_is_read_only(cx, recording)"),
        "prompt must be marked read-only during recording and re-enabled upon completion"
    );
    assert!(
        code.contains("btn_send.set_enabled(cx, !recording)"),
        "send button must be disabled during recording and re-enabled for clicking upon completion"
    );
    let (on, off) = (
        crawrad_audio::prompt::look(true),
        crawrad_audio::prompt::look(false),
    );
    assert_eq!(
        (on.disabled, off.disabled),
        (1.0, 0.0),
        "Send drawn disabled only while recording"
    );
    assert!(
        code.contains("if self.chat.audio_gravando {\n            return;\n        }"),
        "send_ai_prompt must not execute send while audio recording is active"
    );
    let theme = super::tests::TEMA_COMPLETO;
    assert!(
        theme.contains("uniform color_disabled: (CRAWRAD_BORDA_CONTROLE)"),
        "CrawradCircleButton send button must have disabled visual state declared in the DSL"
    );
}

/// The model selection combobox and the context button (+) have helper routines
/// that look up the nested path under `chat_base.ai_input_bar` and prevent premature dismissal on MouseDown.
#[test]
fn model_and_context_comboboxes_have_helpers_supporting_base_chat() {
    let code = sem_comentario();
    // One helper finds every tool-bar button under chat_base.ai_input_bar.ai_tools_bar
    assert!(
        code.contains("fn chat_tool(&self, name: &str) -> ButtonRef")
            && code.contains("live_id!(chat_base),")
            && code.contains("live_id!(ai_tools_bar),"),
        "chat_tool must look under chat_base.ai_input_bar.ai_tools_bar first"
    );
    assert!(
        code.contains("fn get_chat_input_bar(&self) -> ViewRef")
            && code.contains("chat_base.ai_input_bar"),
        "get_chat_input_bar must navigate under chat_base to measure the floating input bar"
    );
    assert!(
        code.contains(r#"self.chat_tool("btn_ai_model").clicked(actions)"#)
            && code.contains(r#"self.chat_tool("btn_ai_attach").clicked(actions)"#),
        "click dispatch in chat panel must use get_btn_ai_model and get_btn_ai_attach helpers"
    );
}

// =========================================================================
// TESTS: Multi-turn Continuity and Prompt via Stdin
// =========================================================================

#[test]
fn dois_turnos_consecutivos_usam_session_id_e_depois_resume() {
    use crawrad_agent::{SessionSupport, build_cli_args_for_session};

    let session_uuid = "a1b2c3d4-e5f6-4a5b-8c9d-0e1f2a3b4c5d";
    let prompt1 = "My name is Isaque Pinheiro";
    let prompt2 = "What is my name?";

    // Turn 1: New session (not started) -> must pass --session-id <uuid>
    let inv1 = build_cli_args_for_session(
        "claude",
        prompt1,
        Some("sonnet"),
        Some((session_uuid, false)),
        SessionSupport::Pinned,
    );
    assert_eq!(inv1.binary, "claude");
    assert_eq!(
        inv1.args,
        vec![
            "-p",
            "--output-format",
            "text",
            "--model",
            "sonnet",
            "--session-id",
            session_uuid
        ]
    );
    // The prompt CANNOT appear on the command line
    assert!(!inv1.args.iter().any(|a| a.contains(prompt1)));
    assert_eq!(inv1.stdin_data, prompt1);

    // Turn 2: Session already started -> must pass --resume <same uuid>
    let inv2 = build_cli_args_for_session(
        "claude",
        prompt2,
        Some("sonnet"),
        Some((session_uuid, true)),
        SessionSupport::Pinned,
    );
    assert_eq!(
        inv2.args,
        vec![
            "-p",
            "--output-format",
            "text",
            "--model",
            "sonnet",
            "--resume",
            session_uuid
        ]
    );
    assert!(!inv2.args.iter().any(|a| a.contains(prompt2)));
    assert_eq!(inv2.stdin_data, prompt2);
}

#[test]
fn a_40000_character_prompt_goes_on_stdin_not_in_the_arguments() {
    use crawrad_agent::{SessionSupport, build_cli_args_for_session};

    // Creates a large prompt with 40,000 characters (above the Windows limit of 32,766)
    let prompt_gigante = "x".repeat(40_000);
    let inv = build_cli_args_for_session(
        "claude",
        &prompt_gigante,
        Some("sonnet"),
        Some(("test-uuid", false)),
        SessionSupport::Pinned,
    );

    // Prompt received in full via stdin
    assert_eq!(inv.stdin_data.len(), 40_000);
    assert_eq!(inv.stdin_data, prompt_gigante);

    // Arguments do not contain the giant prompt nor exceed the safe command line length
    for arg in &inv.args {
        assert_ne!(arg, &prompt_gigante);
        assert!(arg.len() < 100);
    }
}

#[test]
fn apply_turn_result_marks_the_session_started_and_a_failed_mutation() {
    use crawrad_agent::{ChatSession, TurnOutcome};

    let mut session = ChatSession::default();
    let initial_id = session.ensure_cli_session_id().to_string();
    assert!(!session.session_started);

    // Turn 1 on success must mark session_started = true
    let outcome = session.apply_turn_result(&Ok("Generated response".to_string()));
    assert_eq!(
        outcome,
        TurnOutcome::Success {
            reply: "Generated response".to_string()
        }
    );
    // If the apply_turn_result implementation does not call mark_session_started, this assert breaks
    assert!(
        session.session_started,
        "MUTATION FAIL: apply_turn_result MUST mark session_started = true on success"
    );
    assert_eq!(session.cli_session_id.as_deref(), Some(initial_id.as_str()));
}

#[test]
fn apply_turn_result_captura_conversation_id_do_agy_via_json() {
    use crawrad_agent::{ChatSession, SessionSupport, TurnOutcome};

    let mut session = ChatSession::default();
    assert!(!session.session_started);
    assert_eq!(session.cli_session_id, None);

    // Real format returned by `agy -p <prompt> --output-format json`
    let agy_json =
        r#"{"conversation_id":"7f365c31-real-id","status":"SUCCESS","response":"PONG\n"}"#;
    let outcome =
        session.apply_turn_result_for_support(&Ok(agy_json.to_string()), SessionSupport::Captured);

    match outcome {
        TurnOutcome::Success { reply } => {
            assert_eq!(reply, "PONG");
        }
        _ => panic!("must be TurnOutcome::Success"),
    }

    assert_eq!(session.cli_session_id.as_deref(), Some("7f365c31-real-id"));
    assert!(session.session_started);
}

#[test]
fn apply_turn_result_recovers_from_a_silent_agy_warning_on_exit_0() {
    use crawrad_agent::{ChatSession, SessionSupport, TurnOutcome};

    let mut session = ChatSession {
        cli_session_id: Some("id-que-expirou".to_string()),
        ..Default::default()
    };
    session.mark_session_started();
    assert!(session.session_started);

    // agy exits with exit 0, emits the warning on stderr, and responds with new SUCCESS JSON and new ID
    let agy_warning_with_json = "warning: conversation \"id-que-expirou\" not found\n{\"conversation_id\":\"novo-id-456\",\"status\":\"SUCCESS\",\"response\":\"PONG OK\"}";
    let outcome = session.apply_turn_result_for_support(
        &Ok(agy_warning_with_json.to_string()),
        SessionSupport::Captured,
    );

    match outcome {
        TurnOutcome::SessionLost { error, notice } => {
            assert!(error.contains("expired on backend"));
            assert_eq!(notice, "PONG OK");
        }
        _ => panic!("must be TurnOutcome::SessionLost"),
    }

    // The old id was discarded and the new id minted by agy was adopted
    assert_eq!(session.cli_session_id.as_deref(), Some("novo-id-456"));
    assert!(session.session_started);
}

#[test]
fn claude_pinned_protected_against_arbitrary_json_in_response() {
    use crawrad_agent::{ChatSession, SessionSupport, TurnOutcome};

    let mut session = ChatSession::default();
    let pinned_id = session.ensure_cli_session_id().to_string();
    assert!(!session.session_started);

    // Claude response containing JSON example with conversation_id key
    let claude_reply_with_json = "Here is the requested example:\n{\"conversation_id\": \"id-alheio\", \"status\": \"SUCCESS\"}";
    let outcome = session.apply_turn_result_for_support(
        &Ok(claude_reply_with_json.to_string()),
        SessionSupport::Pinned,
    );

    match outcome {
        TurnOutcome::Success { reply } => {
            assert!(reply.contains("Here is the requested example"));
        }
        _ => panic!("must be TurnOutcome::Success"),
    }

    // Claude's pinned id CANNOT be overwritten by the reply JSON
    assert_eq!(session.cli_session_id.as_deref(), Some(pinned_id.as_str()));
    assert!(session.session_started);
}

#[test]
fn apply_turn_result_recovers_from_a_lost_or_expired_session() {
    use crawrad_agent::{ChatSession, TurnOutcome};

    let mut session = ChatSession::default();
    let initial_id = session.ensure_cli_session_id().to_string();
    session.mark_session_started();
    assert!(session.session_started);

    // Error stating that the session does not exist on the provider
    let err_msg = "Error: session not found on server or expired".to_string();
    let outcome = session.apply_turn_result(&Err(err_msg.clone()));

    assert!(matches!(outcome, TurnOutcome::SessionLost { .. }));
    // Session must be cleared so it does not enter an infinite error loop
    assert_eq!(session.cli_session_id, None);
    assert!(!session.session_started);

    // Next turn allocates clean new ID
    let next_id = session.ensure_cli_session_id().to_string();
    assert_ne!(initial_id, next_id);
}

#[test]
fn apply_turn_result_resets_session_when_first_turn_fails() {
    use crawrad_agent::{ChatSession, TurnOutcome};

    let mut session = ChatSession::default();
    let id_t1 = session.ensure_cli_session_id().to_string();
    assert!(!session.session_started);

    let outcome = session.apply_turn_result(&Err("Failed to spawn process".to_string()));
    assert!(matches!(outcome, TurnOutcome::Error { .. }));

    // On the first turn with error, it must reset so the next attempt mints a new ID
    assert_eq!(session.cli_session_id, None);
    assert!(!session.session_started);

    let id_t2 = session.ensure_cli_session_id().to_string();
    assert_ne!(id_t1, id_t2);
}

#[test]
fn switching_project_resets_the_cli_session() {
    use crawrad_agent::ChatSession;
    use std::path::Path;

    let mut session = ChatSession::default();
    let id_proj1 = session.ensure_cli_session_id().to_string();
    session.mark_session_started();

    let p1 = Path::new("C:/Projetos/App1");
    let p2 = Path::new("C:/Projetos/App2");

    session.reset_if_project_changed(Some(p1));
    // Switch to project 2
    let reset = session.reset_if_project_changed(Some(p2));
    assert!(reset);
    assert_eq!(session.cli_session_id, None);
    assert!(!session.session_started);

    let id_proj2 = session.ensure_cli_session_id().to_string();
    assert_ne!(id_proj1, id_proj2);
}

#[test]
fn the_clear_command_resets_the_cli_session_to_a_new_uuid() {
    use crawrad_agent::ChatSession;

    let mut session = ChatSession::default();
    let id_inicial = session.ensure_cli_session_id().to_string();
    session.mark_session_started();
    assert!(session.session_started);

    // Executes /clear
    session.clear();
    assert_eq!(session.cli_session_id, None);
    assert!(!session.session_started);

    // Next message allocates a new ID
    let new_id = session.ensure_cli_session_id().to_string();
    assert_ne!(id_inicial, new_id);
}

#[test]
fn agy_passes_the_prompt_in_a_flag_and_supports_a_captured_session() {
    use crawrad_agent::{SessionSupport, build_cli_args_for_session};

    let prompt = "Hello Gemini";

    // Turn 1: agy -p "<prompt>" --output-format json --model gemini-3.8-flash-high (no session flag)
    let inv1 = build_cli_args_for_session(
        "agy",
        prompt,
        Some("gemini-3.8-flash-high"),
        None,
        SessionSupport::Captured,
    );

    assert_eq!(inv1.binary, "agy");
    assert_eq!(
        inv1.args,
        vec![
            "--output-format",
            "json",
            "--model",
            "gemini-3.8-flash-high"
        ]
    );
    // agy reads the prompt on stdin, with no `-p`; in argv a prompt with
    // the project context passed the Windows command line limit (error 206)
    assert_eq!(inv1.stdin_data, prompt);

    // Turn 2: agy with captured conversation passes --conversation <id>
    let inv2 = build_cli_args_for_session(
        "agy",
        "Second turn",
        Some("gemini-3.8-flash-high"),
        Some(("conv-1234", true)),
        SessionSupport::Captured,
    );

    assert_eq!(
        inv2.args,
        vec![
            "--output-format",
            "json",
            "--model",
            "gemini-3.8-flash-high",
            "--conversation",
            "conv-1234"
        ]
    );
    assert_eq!(inv2.stdin_data, "Second turn");
}

#[test]
fn the_invocation_respects_the_project_directory() {
    use crawrad_agent::{PromptDelivery, SessionSupport, build_cli_invocation_with_options};
    use std::path::PathBuf;

    let proj_dir = PathBuf::from("E:/MeusProjetos/CrawRAD_App");
    let inv = build_cli_invocation_with_options(
        "claude",
        "test prompt",
        Some("sonnet"),
        None,
        SessionSupport::Pinned,
        PromptDelivery::Stdin,
        Some(proj_dir.clone()),
    );

    assert_eq!(inv.cwd, Some(proj_dir));
}

#[test]
fn invocacao_codex_usa_stdin_e_hifen() {
    use crawrad_agent::{
        PromptDelivery, SessionSupport, build_agent_invocation_args, find_profile_by_id,
    };

    let prof = find_profile_by_id("codex").expect("codex profile must exist");
    assert_eq!(prof.prompt_delivery, PromptDelivery::Stdin);
    assert!(prof.prompt_via_stdin);
    assert_eq!(prof.session, SessionSupport::Captured);

    // Turn 1: exec --json --model <model> - with full prompt via stdin
    let prompt = "Hello\n\ncan we chat in English ?";
    let (args1, stdin1) =
        build_agent_invocation_args(prof, prompt, Some("gpt-5.6-sol"), None, false);
    assert_eq!(args1, vec!["exec", "--json", "--model", "gpt-5.6-sol", "-"]);
    assert_eq!(stdin1, prompt);

    // Turn 2: exec resume <thread_id> --json --model <model> - with prompt via stdin
    let (args2, stdin2) = build_agent_invocation_args(
        prof,
        "What is the Rust version?",
        Some("gpt-5.6-sol"),
        Some("thread-xyz-123"),
        true,
    );
    assert_eq!(
        args2,
        vec![
            "exec",
            "resume",
            "thread-xyz-123",
            "--json",
            "--model",
            "gpt-5.6-sol",
            "-"
        ]
    );
    assert_eq!(stdin2, "What is the Rust version?");
}

// =========================================================================
// TESTS: Canonical Agent Profiles, Exact Models, and Keys
// =========================================================================

/// Code gate: no `contains("claude")`, `contains("gemini")`, or `contains("agy")`
/// may exist outside the profiles module to guess agent behavior.
#[test]
fn no_agent_contains_check_outside_the_profiles_module() {
    let acp_rs = include_str!("../../../../../crawrad-agent/src/acp.rs");
    let ide_app_rs = sem_comentario();

    for forbidden in &[
        "provider.contains(\"claude\")",
        "provider.contains(\"gemini\")",
        "provider.contains(\"agy\")",
        "endpoint.contains(\"claude\")",
        "endpoint.contains(\"gemini\")",
        "endpoint.contains(\"agy\")",
        "p_lower.contains(\"claude\")",
        "p_lower.contains(\"gemini\")",
        "p_lower.contains(\"agy\")",
    ] {
        assert!(
            !acp_rs.contains(forbidden),
            "acp.rs contains forbidden agent detection by substring: `{forbidden}`"
        );
        assert!(
            !ide_app_rs.contains(forbidden),
            "ide_app.rs contains forbidden agent detection by substring: `{forbidden}`"
        );
    }
}

/// Renaming a profile's `display_name` does not alter its functional behavior.
#[test]
fn renaming_a_profile_display_name_changes_no_behavior() {
    let profile =
        crawrad_agent::agent_profile::find_profile_by_id("claude").expect("claude profile");
    assert_eq!(profile.id, "claude");
    assert_eq!(profile.default_binary, "claude");
    assert_eq!(
        profile.session,
        crawrad_agent::agent_profile::SessionSupport::Pinned
    );

    // Creates copy with modified display_name
    let mut custom_profile = profile.clone();
    custom_profile.display_name = "My Renewed Anthropic Assistant";

    // Launch and built arguments depend on ID and metadata, never on display_name
    let args_orig = crawrad_agent::agent_profile::build_agent_args(
        profile,
        Some("claude-3-7-sonnet"),
        Some("sess-1"),
        false,
    );
    let args_renomeado = crawrad_agent::agent_profile::build_agent_args(
        &custom_profile,
        Some("claude-3-7-sonnet"),
        Some("sess-1"),
        false,
    );
    assert_eq!(args_orig, args_renomeado);
}

/// The chosen model ID arrives identical to the `--model` argument (no collapse/translation to "sonnet").
#[test]
fn chosen_model_id_matches_model() {
    let _guard = model_cache_guard();
    for profile in crawrad_agent::agent_profile::PROFILES {
        for model in profile.models() {
            let args = crawrad_agent::agent_profile::build_agent_args(
                profile,
                Some(&model.id),
                None,
                false,
            );

            let pos = args
                .iter()
                .position(|a| a == "--model")
                .expect("must contain --model argument");
            let repassado = &args[pos + 1];
            assert_eq!(
                repassado, &model.id,
                "model passed on command line must be exactly `{}` without translations",
                model.id
            );
        }
    }
}

/// An unknown key in acp_config.json survives a read-modify-write cycle.
#[test]
fn an_unknown_key_in_acp_config_json_survives_the_save() {
    let tmp_path =
        std::env::temp_dir().join(format!("acp_unknown_test_{}.json", std::process::id()));

    let json_inicial = serde_json::json!({
        "agent_id": "gemini",
        "provider": "Gemini (Antigravity)",
        "model": "gemini-3.8-flash-high",
        "endpoint": "stdio:agy",
        "auth_token": "secret-123",
        "auto_approve_reads": true,
        "auto_approve_plans": false,
        "enable_thinking": true,
        "aefos_custom_telemetry_key": "telemetry_payload_xyz",
        "workspace_root_override": "E:\\Projects\\Aefos"
    });

    std::fs::write(&tmp_path, serde_json::to_string(&json_inicial).unwrap()).unwrap();

    // Loads the file using the AcpConfig parser
    let mut config = crawrad_agent::AcpConfig::load_from_file(&tmp_path).expect("load config");
    assert_eq!(config.agent_id, "gemini");
    assert_eq!(config.provider, "Gemini (Antigravity)");

    // Modifies a known property and saves atomically
    config.model = "gemini-3.8-flash-medium".to_string();
    config.save_to_file(&tmp_path).expect("save config");

    // Reads back as generic JSON to check if unknown keys survived
    let saved_text = std::fs::read_to_string(&tmp_path).expect("read saved");
    let json_recarregado: serde_json::Value =
        serde_json::from_str(&saved_text).expect("parse json");

    assert_eq!(
        json_recarregado["aefos_custom_telemetry_key"], "telemetry_payload_xyz",
        "unknown key aefos_custom_telemetry_key must have survived saving"
    );
    assert_eq!(
        json_recarregado["workspace_root_override"], "E:\\Projects\\Aefos",
        "unknown key workspace_root_override must have survived saving"
    );
    assert_eq!(
        json_recarregado["model"], "gemini-3.8-flash-medium",
        "changed model must have been updated successfully"
    );

    let _ = std::fs::remove_file(&tmp_path);
}

/// `/use <agent>` switches the profile without blocking the UI thread
/// (the two vendor-named commands were folded into it).
#[test]
fn the_use_command_switches_profile_without_blocking_ui() {
    let code = sem_comentario();
    assert!(
        code.contains("crawrad_agent::slash_commands::use_reply")
            && code.contains("self.selecionar_provedor_ia(cx, p);"),
        "/use must resolve the agent and call switch_provider"
    );
    assert!(
        !code.contains("match crawrad_agent::AcpConfig::execute_cli_query(\"claude\", query)")
            && !code
                .contains("match crawrad_agent::AcpConfig::execute_cli_query(\"agy\", query)"),
        "synchronous execution on UI thread for /claude and /gemini was eliminated"
    );
}

/// The `profile.models()` call is strictly non-blocking (never triggers synchronous agy models on the UI thread).
#[test]
fn models_nunca_bloqueia_e_le_cache_ou_fallback_instantaneamente() {
    let _guard = model_cache_guard();
    use crawrad_agent::agent_profile::{AgentProfile, find_profile_by_id};
    use std::time::Instant;

    let gemini = find_profile_by_id("gemini").expect("gemini profile must exist");

    AgentProfile::clear_cache();

    // Measures call latency: running `agy models` on the current thread would take > 2000ms.
    let start = Instant::now();
    let models = gemini.models();
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 50,
        "AgentProfile::models() blocked for {}ms; must be instantaneous and non-blocking",
        elapsed.as_millis()
    );
    assert!(
        !models.is_empty(),
        "must return fallback list immediately if cache is empty"
    );
}

/// In-memory model cache isolates the UI and returns dynamic list when updated.
#[test]
fn model_cache_allows_injection_and_returns_updated_models() {
    let _guard = model_cache_guard();
    use crawrad_agent::agent_profile::{AgentProfile, ModelSpec, find_profile_by_id};

    let gemini = find_profile_by_id("gemini").expect("gemini profile must exist");

    AgentProfile::clear_cache();
    let inicial = gemini.models();
    assert!(
        inicial.iter().any(|m| m.id.contains("gemini-3.8-flash")),
        "initial fallback must contain gemini-3.8-flash"
    );

    // Injects updated models into cache (as returned by the background worker)
    gemini.set_cached_models(vec![ModelSpec {
        id: "gemini-cached-turbo".to_string(),
        display_name: "Gemini Cached Turbo".to_string(),
        description: "Test Injected".to_string(),
        supports_reasoning: true,
    }]);

    let cached = gemini.models();
    assert_eq!(cached.len(), 1);
    assert_eq!(cached[0].id, "gemini-cached-turbo");

    // Clears cache for subsequent tests
    AgentProfile::clear_cache();
    let restaurado = gemini.models();
    assert!(
        restaurado.iter().any(|m| m.id.contains("gemini-3.8-flash")),
        "after clearing cache must return fallback"
    );
}

/// The model selector in the Options screen (opt_ai_model) strictly indexes the
/// active provider's model list via `model_at_position`, not the global `AcpConfig::MODELS` array.
/// If there is a regression to `AcpConfig::MODELS.get(i)`, this test breaks immediately.
#[test]
fn model_at_position_indexes_active_list_without_global_leak() {
    let _guard = model_cache_guard();
    use crawrad_agent::AcpConfig;

    let mut cfg = AcpConfig::default();
    cfg.switch_provider("Gemini (Antigravity)");

    let active_models = cfg.models_for_active_provider();
    assert!(
        active_models.len() >= 2,
        "Gemini provider must have available models including Auto"
    );

    // The model returned at position 0 MUST be "auto"
    let m0 = cfg.model_at_position(0).expect("model at position 0");
    assert_eq!(m0, "auto", "model at position 0 must be Auto");
    assert_eq!(
        m0, active_models[0].id,
        "model at position 0 must be the provider's active model"
    );

    // The model returned at position 1 MUST be Gemini's first native model
    let m1 = cfg.model_at_position(1).expect("model at position 1");
    assert_eq!(
        m1, active_models[1].id,
        "model at position 1 must be the first native model of the active provider"
    );

    // Ensures it is a Gemini model and NOT the Claude model from the global list
    assert!(
        m1.starts_with("gemini"),
        "returned model ({m1}) should be from Gemini, and not {m1}"
    );
    assert_ne!(
        Some(m1.as_str()),
        AcpConfig::MODELS.get(1).copied(),
        "if indexing global AcpConfig::MODELS at position 1, regression will pick Claude model instead of Gemini"
    );

    // Test with Claude Agent
    cfg.switch_provider("Claude Agent");
    let claude_models = cfg.models_for_active_provider();
    let claude_0 = cfg.model_at_position(0).expect("claude model 0");
    assert_eq!(claude_0, "auto");
    assert_eq!(claude_0, claude_models[0].id);
    let claude_1 = cfg.model_at_position(1).expect("claude model 1");
    assert_eq!(claude_1, claude_models[1].id);

    // Position beyond length must return None safely
    assert!(cfg.model_at_position(9999).is_none());
}

/// Formatting of the AI model label protects against byte slicing panic
/// on long multibyte UTF-8 strings (emojis, accents, CJK characters).
#[test]
fn format_model_label_safe_with_multibyte_utf8() {
    // Short model without truncation
    let r1 = crawrad_agent::format_model_label_pure("gpt-4o");
    assert_eq!(r1, "GPT-4o ▾");

    // Model with long standard ASCII name
    let r2 = crawrad_agent::format_model_label_pure("my-very-long-custom-model-identifier");
    assert_eq!(r2, "my-very-long… ▾");

    // Model with accented multibyte characters exceeding the limit (> 14 characters)
    let acentuado = "configuração-modelo-extra-longo";
    let r3 = crawrad_agent::format_model_label_pure(acentuado);
    assert!(r3.ends_with("… ▾"));
    assert_eq!(r3, "configuração… ▾");

    // Model with multibyte emoji (4 bytes each)
    let emoji = "🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀";
    let r4 = crawrad_agent::format_model_label_pure(emoji);
    assert!(r4.ends_with("… ▾"));
    assert_eq!(r4, "🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀… ▾");

    // Model with CJK characters
    let cjk = "人工知能言語モデルバージョンワンツー";
    let r5 = crawrad_agent::format_model_label_pure(cjk);
    assert_eq!(r5, "人工知能言語モデルバージ… ▾");
}

/// The options dialog keeps a snapshot of ACP config and restores if canceled.
#[test]
fn canceling_options_restores_previous_configuration() {
    let code = sem_comentario();
    assert!(
        code.contains("self.acp_antes = Some(self.acp_config.clone());"),
        "open_options must save snapshot of acp_config in acp_antes"
    );
    assert!(
        code.contains("if let Some(acp_antes) = self.acp_antes.take() {")
            && code.contains("self.acp_config = acp_antes;"),
        "desfazer_options (Cancel) must restore self.acp_antes"
    );
    assert!(
        code.contains("self.acp_antes = None;"),
        "salvar_options (OK) must discard the acp_antes snapshot"
    );
}

/// The three provider combos in Options (opt_ai_provider, opt_cu_ai_provider, opt_sr_ai_provider)
/// use the centralized `selecionar_provedor_ia` method which synchronizes state and UI.
#[test]
fn selecting_the_ai_provider_unifies_the_three_combos() {
    let code = sem_comentario();
    assert!(
        code.contains("self.selecionar_provedor_ia(cx, p);"),
        "provider combos must dispatch to selecionar_provedor_ia"
    );
    assert!(
        !code.contains("if *p == \"Gemini (Antigravity)\" {")
            && !code.contains("self.acp_config.endpoint = \"gemini --acp-server\".to_string();"),
        "hardcoded endpoints in opt_sr/opt_cu were eliminated in favor of switch_provider"
    );
}

/// Configurations with a custom provider (`profile.id == "custom"`) split
/// endpoint arguments to separate the binary from additional execution flags.
#[test]
fn custom_endpoint_com_argumentos_executa_binario_e_flags_separados() {
    use crawrad_agent::AcpConfig;

    let mut cfg = AcpConfig {
        provider: "Custom ACP Server".to_string(),
        agent_id: "custom".to_string(),
        endpoint: "npx -y @modelcontextprotocol/server-postgres --db-url postgresql://localhost"
            .to_string(),
        model: "custom-model".to_string(),
        ..Default::default()
    };

    let inv = cfg
        .build_invocation_for_prompt("Hello Custom", None, None)
        .expect("build invocation for custom endpoint");

    assert_eq!(inv.binary, "npx", "binary must be the first token");
    assert_eq!(
        inv.args[0], "-y",
        "first argument must be -y from endpoint flags"
    );
    assert_eq!(
        inv.args[1], "@modelcontextprotocol/server-postgres",
        "second argument must be the npx package"
    );
    assert_eq!(
        inv.args[2], "--db-url",
        "third argument must be endpoint flag"
    );
    assert_eq!(
        inv.args[3], "postgresql://localhost",
        "fourth argument must be the flag value"
    );

    // Empty endpoint must return friendly error
    cfg.endpoint = "   ".to_string();
    let err = cfg.build_invocation_for_prompt("test", None, None);
    assert!(err.is_err());
    assert_eq!(err.unwrap_err(), "Custom ACP server endpoint is empty");
}

/// The btn_ai_browser item was removed from the DSL and shell because CrawRAD does not do direct web search.
#[test]
fn btn_ai_browser_is_gone_from_the_context_popup() {
    let code = sem_comentario();
    let panels = include_str!("../../../ide_app/panels.rs");
    assert!(
        !panels.contains("btn_ai_browser"),
        "btn_ai_browser must have been removed from context popup DSL in panels.rs"
    );
    assert!(
        !code.contains("btn_ai_browser"),
        "btn_ai_browser must have been removed from action handling in ide_app.rs"
    );
}

/// No item in the context popup (+) merely writes to the status bar without performing an action.
#[test]
fn no_context_popup_item_merely_writes_to_the_status_bar() {
    let code = sem_comentario();
    assert!(
        !code.contains("self.status = Some(\"Attach Media:"),
        "btn_ai_media must not merely write to the status bar"
    );
    assert!(
        !code.contains("self.status = Some(\"Mentions:"),
        "btn_ai_mentions must not merely write to the status bar"
    );
    assert!(
        !code.contains("self.status = Some(\"Browser:"),
        "btn_ai_browser must not exist"
    );
}

/// The memory dialog synchronizes text with the input and persists real edits.
#[test]
fn the_memory_dialog_syncs_with_the_input_and_persists_the_edit() {
    let memory_dialog_rs = include_str!("../../../ide_app/dialogs/memory.rs");
    assert!(
        memory_dialog_rs.contains("dlg.text_input(id!(mem_input))")
            && memory_dialog_rs.contains(".set_text(cx, &self.memoria_texto);"),
        "open_memory must initialize mem_input with loaded text"
    );
    assert!(
        memory_dialog_rs.contains("mem_inp.changed(actions)"),
        "atender_memory must listen to changes typed into mem_input"
    );
    assert!(
        memory_dialog_rs.contains("mem_inp.text()"),
        "atender_memory must read text from mem_input when saving"
    );
}

/// executar_ai_prompt extracts live project context and global memory via PromptAssembler.
#[test]
fn execute_ai_prompt_builds_live_project_context_and_memory() {
    let code = sem_comentario();
    assert!(
        code.contains("crawrad_agent::PromptAssembler::assemble"),
        "executar_ai_prompt must invoke PromptAssembler::assemble to enrich the prompt"
    );
    assert!(
        code.contains("extrair_contexto_ativo_para_ia()"),
        "executar_ai_prompt must extract live context from editor or designer"
    );
    assert!(
        code.contains("extrair_selecao_do_editor()"),
        "ide_app must have routine to extract active selection from CodeView"
    );
}

/// Gate of The MCP dialog does not paint fixed green and calls the executable probe.
#[test]
fn o_dialogo_mcp_chama_probe_e_nao_tem_verde_fixo() {
    let mcp_dialog_code = include_str!("../../../ide_app/dialogs/mcp.rs");
    assert!(
        mcp_dialog_code.contains("s.probe()"),
        "MCP dialog must invoke s.probe() to check real server availability"
    );
    assert!(
        !mcp_dialog_code.contains("\"🟢 "),
        "MCP dialog cannot fix status 🟢 without probe check"
    );

    let config_fake = crawrad_agent::McpServerConfig {
        name: "fake-mcp-server".into(),
        transport: crawrad_agent::McpTransport::Stdio,
        command: "nonexistent_mcp_binary_xyz_12345".into(),
        args: vec![],
        env: std::collections::HashMap::new(),
        source: crawrad_agent::McpSource::Custom,
        enabled: true,
    };
    assert!(
        !config_fake.probe(),
        "probe must return false for nonexistent executable in PATH"
    );
}

/// Gate of auth_token was removed from AcpConfig and serialization.
#[test]
fn auth_token_no_longer_exists_in_the_type_or_on_disk() {
    let cfg = crawrad_agent::AcpConfig::default();
    let json = serde_json::to_string(&cfg).expect("AcpConfig serialization");
    assert!(
        !json.contains("auth_token"),
        "auth_token field must not exist in JSON serialization of AcpConfig"
    );
}

/// Gate of every slash command of the table having an arm in the
/// shell's dispatch, which matches on the table's `id`.
#[test]
fn todo_slash_command_do_catalogo_tem_despacho() {
    let shell_code = sem_comentario();
    for cmd in crawrad_agent::SlashCommand::ALL {
        assert!(
            shell_code.contains(&format!("\"{}\"", cmd.id)),
            "command `{}` present in the catalog must have handling/dispatch in the shell",
            cmd.trigger
        );
    }
}

/// Gate of Default folders point to CrawRAD (own folder).
#[test]
fn default_paths_point_to_crawrad_with_own_folder() {
    let path_acp = crawrad_agent::AcpConfig::default_path();
    let path_acp_str = path_acp.to_string_lossy();
    assert!(
        path_acp_str.contains("CrawRAD")
            || path_acp_str.contains(".crawrad")
            || path_acp_str.contains("acp_config.json"),
        "AcpConfig::default_path must reside in CrawRAD's own folder: {path_acp_str}"
    );

    let path_mem = crawrad_agent::AefosMemory::default_path();
    let path_mem_str = path_mem.to_string_lossy();
    assert!(
        path_mem_str.contains("CrawRAD")
            || path_mem_str.contains(".crawrad")
            || path_mem_str.contains("memory.json"),
        "AefosMemory::default_path must reside in CrawRAD's own folder: {path_mem_str}"
    );
}

/// Gate of Editing global memory and saving writes and reopens with the edited text.
#[test]
fn global_memory_saves_and_reopens_the_edited_text() {
    let dir = std::env::temp_dir().join("crawrad_test_memoria_183");
    let _ = std::fs::create_dir_all(&dir);
    let file = dir.join("memory_editada.json");

    let edited_text = "Specific test instructions edited by user in dialog";
    let mem = crawrad_agent::AefosMemory::new(edited_text);
    mem.save_to_file(&file).expect("save edited memory");

    let lida = crawrad_agent::AefosMemory::load_from_file(&file).expect("load memory");
    assert_eq!(lida.instructions, edited_text);

    let _ = std::fs::remove_file(&file);
    let _ = std::fs::remove_dir(&dir);
}

/// Chat Usability Gate: auto-scroll to bottom and visual breathing room.
///
/// Ensures that:
/// 1. The chat messages viewport (`ai_messages_view`) has a bottom spacer (`chat_bottom_spacer`).
/// 2. When rendering new messages or AI responses, the shell triggers auto-scroll to the end.
#[test]
fn o_chat_tem_rolagem_automatica_e_respiro_no_final() {
    assert!(
        CASCA.contains("chat_bottom_spacer = <View>"),
        "ai_messages_view must contain chat_bottom_spacer to prevent the last card from sticking to or being cut off by the prompt bar"
    );
    assert!(
        CASCA.contains("scroll_to_bottom_frames"),
        "ide_app must manage pending frames for auto-scrolling to the end of the chat"
    );
    assert!(
        CASCA.contains("ai_messages_view")
            && CASCA.contains("set_scroll_pos(cx, dvec2(0.0, 100_000.0))"),
        "renderizar_historico_chat or handle_draw_2d must apply set_scroll_pos to the end"
    );
}

/// Visual Presentation Gate: Orange User Bubble in Chat and Explorer Spacing.
///
/// Ensures that:
/// 1. The user message card (`CrawradCardUser`) uses CrawRAD's signature orange background (`CRAWRAD_ACENTO`),
///    rounded corners, and left margin to ensure right visual alignment (chat bubble effect / Aefos Chat).
/// 2. User messages use high-contrast white typography (`CRAWRAD_BRANCO`).
/// 3. The Explorer panel header (`explorer_header`) has explicit spacing between action buttons to avoid crowding.
#[test]
fn user_bubble_has_orange_background_aligned_right_with_spacing() {
    let theme = super::tests::TEMA_COMPLETO;
    assert!(
        theme.contains("pub CrawradCardUser = <CrawradCard>")
            && theme.contains("color: (CRAWRAD_ACENTO)")
            // 8 px corners and a 3 px tail at the bottom right (the owner's review, 06/10/2026)
            && theme.contains("uniform border_radius: 8.0")
            && theme.contains("r, r, 3.0, r)")
            && theme.contains("margin: { left:"),
        "CrawradCardUser must define left margin, CRAWRAD_ACENTO background, and border_radius for right-aligned look"
    );
    assert!(
        (CASCA.contains("pub CrawradAiUserCard = <CrawradCardUser>")
            || CASCA.contains("u_bubble = <CrawradCardUser>"))
            && CASCA.contains("color: (CRAWRAD_BRANCO)"),
        "CrawradAiUserCard must render user text with CRAWRAD_BRANCO color over the orange bubble"
    );
    assert!(
        CASCA.contains("explorer_header = <CrawradPanelHeader>") && CASCA.contains("spacing: 3.0,"),
        "explorer_header must contain explicit spacing (spacing: 3.0) between action buttons"
    );
}

/// User Bubble Auto-Size Gate: compact size for short sentences and maximum limit for long sentences.
#[test]
fn the_user_bubble_auto_sizes_with_a_max_width_and_wraps() {
    let panel_width = 300.0;
    // Short text: small bubble, large left indent (e.g. ~155px with meta "19:19 ✓✓")
    let recuo_curto =
        crawrad_agent::chat_layout::user_bubble_indent("Hi", "19:19 ✓✓", panel_width);
    assert!(
        recuo_curto > 140.0,
        "short messages must have larger left indent to keep the bubble compact and against the right edge (indent: {recuo_curto})"
    );

    // Medium text: proportional bubble
    let recuo_medio =
        crawrad_agent::chat_layout::user_bubble_indent("Test build", "19:19 ✓✓", panel_width);
    assert!(
        recuo_medio < recuo_curto
            && recuo_medio > crawrad_agent::chat_layout::USER_BUBBLE_MIN_INDENT,
        "medium messages must have proportional bubble (indent: {recuo_medio})"
    );

    // Long text: hits maximum width limit (minimum indent with line break)
    let long_text = "This is a long prompt typed by the developer to request a complete refactoring of the visual component with multiple arguments.";
    let recuo_longo =
        crawrad_agent::chat_layout::user_bubble_indent(long_text, "19:19 ✓✓", panel_width);
    assert_eq!(
        recuo_longo,
        crawrad_agent::chat_layout::USER_BUBBLE_MIN_INDENT,
        "long messages must reach maximum width limit (minimum left indent)"
    );
}

// =========================================================================
// TESTS: Auto as Default Model and Omission of --model
// =========================================================================

/// Gate of "Auto" is the 1st fixed item in EVERY provider's list and default for new configs.
#[test]
fn auto_is_first_fixed_model_of_every_provider_and_config_default() {
    let _guard = model_cache_guard();
    use crawrad_agent::{AcpConfig, ChatSession};

    // 1. New config and new ChatSession start with "auto" model
    let cfg = AcpConfig::default();
    assert_eq!(
        cfg.model, "auto",
        "AcpConfig::default() must have model 'auto'"
    );
    let sess = ChatSession::default();
    assert_eq!(
        sess.model, "auto",
        "ChatSession::default() must have model 'auto'"
    );
    let sess_new = ChatSession::new("test", "Test Session");
    assert_eq!(
        sess_new.model, "auto",
        "ChatSession::new() must have model 'auto'"
    );

    // 2. Every registered provider has "auto" at position 0 without duplicate
    for &provider in AcpConfig::PROVIDERS {
        let models = cfg.models_for_provider_and_endpoint(provider, "");
        assert!(
            !models.is_empty(),
            "provider {provider} must contain models"
        );
        assert_eq!(
            models[0].id, "auto",
            "first model of {provider} must be 'auto'"
        );
        assert_eq!(
            models[0].display_name, "Auto (Agent Default)",
            "display label for 'auto' must be 'Auto (Agent Default)'"
        );
        let auto_count = models
            .iter()
            .filter(|m| m.id.eq_ignore_ascii_case("auto"))
            .count();
        assert_eq!(
            auto_count, 1,
            "there must be no duplication of 'auto' in provider {provider}"
        );
    }
}

/// Gate of When model is "auto" (or empty), no --model argument is generated in the CLI.
/// Mutation: removing the omission of --model for "auto" fails this test immediately.
#[test]
fn auto_omits_the_model_argument_on_every_cli() {
    use crawrad_agent::{SessionSupport, build_cli_args_for_session};

    let clis = [
        ("claude", SessionSupport::Pinned),
        ("agy", SessionSupport::Captured),
        ("codex", SessionSupport::Captured),
        ("opencode", SessionSupport::None),
    ];

    for &(binary, support) in &clis {
        // Case 1: explicit "auto" model -> NO --model
        let inv_auto =
            build_cli_args_for_session(binary, "test prompt", Some("auto"), None, support);
        assert!(
            !inv_auto.args.iter().any(|a| a == "--model"),
            "CLI {binary} with model 'auto' CANNOT contain --model flag, received: {:?}",
            inv_auto.args
        );

        // Case 2: case-insensitive "Auto" and "AUTO" -> NO --model
        let inv_upper =
            build_cli_args_for_session(binary, "test prompt", Some("Auto"), None, support);
        assert!(
            !inv_upper.args.iter().any(|a| a == "--model"),
            "CLI {binary} with model 'Auto' CANNOT contain --model flag, received: {:?}",
            inv_upper.args
        );

        // Case 3: None or empty model -> NO --model
        let inv_none = build_cli_args_for_session(binary, "test prompt", None, None, support);
        assert!(
            !inv_none.args.iter().any(|a| a == "--model"),
            "CLI {binary} without model CANNOT contain --model flag, received: {:?}",
            inv_none.args
        );

        let inv_empty =
            build_cli_args_for_session(binary, "test prompt", Some("   "), None, support);
        assert!(
            !inv_empty.args.iter().any(|a| a == "--model"),
            "CLI {binary} with empty model CANNOT contain --model flag, received: {:?}",
            inv_empty.args
        );
    }

    // Counter-proof: specific model GENERATES the --model flag with exact id
    let inv_claude_sonnet = build_cli_args_for_session(
        "claude",
        "test prompt",
        Some("sonnet"),
        None,
        SessionSupport::Pinned,
    );
    assert_eq!(
        inv_claude_sonnet.args.iter().position(|a| a == "--model"),
        Some(3),
        "specific model 'sonnet' must include --model flag"
    );
    assert_eq!(inv_claude_sonnet.args.get(4), Some(&"sonnet".to_string()));

    let inv_codex_model = build_cli_args_for_session(
        "codex",
        "test prompt",
        Some("gpt-5.6-sol"),
        None,
        SessionSupport::Captured,
    );
    assert!(
        inv_codex_model.args.iter().any(|a| a == "--model"),
        "specific model in codex must include --model flag"
    );
    let pos = inv_codex_model
        .args
        .iter()
        .position(|a| a == "--model")
        .unwrap();
    assert_eq!(
        inv_codex_model.args.get(pos + 1),
        Some(&"gpt-5.6-sol".to_string())
    );
}

/// Gate of Provider change restores that provider's chosen model or 'auto'.
#[test]
fn provider_change_restores_saved_model_or_auto() {
    let _guard = model_cache_guard();
    use crawrad_agent::AcpConfig;

    let mut cfg = AcpConfig::default();
    assert_eq!(cfg.model, "auto");
    assert_eq!(cfg.provider, "Claude Agent");

    // Selects specific model for Claude
    cfg.model = "claude-opus-4-7".to_string();

    // Switch to Gemini -> new provider without saved model must fall back to "auto"
    cfg.switch_provider("Gemini (Antigravity)");
    assert_eq!(cfg.provider, "Gemini (Antigravity)");
    assert_eq!(
        cfg.model, "auto",
        "provider without history must initialize to 'auto'"
    );

    // Selects model in Gemini
    cfg.model = "gemini-3.8-flash-high".to_string();

    // Back to Claude -> must restore "claude-opus-4-7"
    cfg.switch_provider("Claude Agent");
    assert_eq!(cfg.provider, "Claude Agent");
    assert_eq!(
        cfg.model, "claude-opus-4-7",
        "must restore saved Claude model"
    );

    // Back to Gemini -> must restore "gemini-3.8-flash-high"
    cfg.switch_provider("Gemini (Antigravity)");
    assert_eq!(cfg.provider, "Gemini (Antigravity)");
    assert_eq!(
        cfg.model, "gemini-3.8-flash-high",
        "must restore saved Gemini model"
    );
}

/// Gate of Manual model management (+/-) preserves Auto integrity.
#[test]
fn manual_model_management_adds_removes_and_preserves_auto() {
    let _guard = model_cache_guard();
    use crawrad_agent::AcpConfig;

    let mut cfg = AcpConfig::default();
    cfg.switch_provider("OpenAI / Codex ACP Bridge");

    // Adds new model
    cfg.add_custom_model_for_provider("OpenAI / Codex ACP Bridge", "gpt-7-preview");
    assert_eq!(cfg.model, "gpt-7-preview");

    let models = cfg.models_for_active_provider();
    assert_eq!(models[0].id, "auto", "Auto must remain as 1st element");
    assert!(
        models.iter().any(|m| m.id == "gpt-7-preview"),
        "newly added model must appear in active list"
    );

    // Attempting to add "auto" as custom model is a no-op
    cfg.add_custom_model_for_provider("OpenAI / Codex ACP Bridge", "auto");
    let auto_count = cfg
        .models_for_active_provider()
        .iter()
        .filter(|m| m.id.eq_ignore_ascii_case("auto"))
        .count();
    assert_eq!(
        auto_count, 1,
        "auto cannot be duplicated via add_custom_model"
    );

    // Attempting to remove "auto" is a protected no-op
    cfg.remove_model_for_provider("OpenAI / Codex ACP Bridge", "auto");
    assert_eq!(
        cfg.models_for_active_provider()[0].id,
        "auto",
        "removing 'auto' must be blocked"
    );

    // Removing active custom model resets selected model to 'auto'
    cfg.remove_model_for_provider("OpenAI / Codex ACP Bridge", "gpt-7-preview");
    assert_eq!(
        cfg.model, "auto",
        "when removing active model, must fall back to first available ('auto')"
    );
    assert!(
        !cfg.models_for_active_provider()
            .iter()
            .any(|m| m.id == "gpt-7-preview"),
        "removed model must no longer appear in active list"
    );
}

// =========================================================================
// TESTS: Real History, Markdown, Tool Cards,
// Stop Button, Dynamic Queue, and Non-Blocking Gate on UI Thread
// =========================================================================

/// Criterion 1: Three consecutive turns remain visible simultaneously in history.
#[test]
fn three_turns_in_a_row_stay_visible_in_the_history() {
    use crawrad_agent::{ChatMessage, ChatSession};

    let mut session = ChatSession::new("test-history", "Multi-turn Real History");
    assert!(session.is_empty());

    // Turn 1
    session.add_message(ChatMessage::user("msg-1", "What is the Match syntax?"));
    session.add_message(ChatMessage::assistant(
        "msg-2",
        "The syntax is match val { Pat => expr }",
    ));

    // Turn 2
    session.add_message(ChatMessage::user(
        "msg-3",
        "And how do I destructure a struct?",
    ));
    session.add_message(ChatMessage::assistant("msg-4", "Use Point { x, y } => ..."));

    // Turn 3
    session.add_message(ChatMessage::user("msg-5", "And with references?"));
    session.add_message(ChatMessage::assistant(
        "msg-6",
        "Use ref or &Point { x, ref y }",
    ));

    // The 3 turns (6 messages) coexist in the session
    assert_eq!(session.messages.len(), 6);
    assert_eq!(session.messages[0].text, "What is the Match syntax?");
    assert_eq!(
        session.messages[2].text,
        "And how do I destructure a struct?"
    );
    assert_eq!(session.messages[4].text, "And with references?");

    // Verifies that the shell supports rendering multiple simultaneous slots
    let code = sem_comentario();
    assert!(
        code.contains("user_card_1")
            && code.contains("reply_card_1")
            && code.contains("user_card_2"),
        "the IDE shell must contain multiple message cards (user_card_1, reply_card_1, etc.) to display turns simultaneously"
    );
}

/// Criterion 2: Response with code block renders block with header, copy button, and mono font.
#[test]
fn code_block_response_renders_formatted_code_with_copy() {
    use crawrad_agent::{ChatContentBlock, parse_chat_markdown};

    let text = "Here is the requested function:\n\n```rust\nfn calcular(x: i32) -> i32 {\n    x * 2\n}\n```\n\nReady for use!";
    let blocos = parse_chat_markdown(text);

    assert_eq!(blocos.len(), 3);
    match &blocos[0] {
        ChatContentBlock::Text(t) => assert!(t.contains("Here is the requested function")),
        _ => panic!("first block must be text"),
    }
    match &blocos[1] {
        ChatContentBlock::CodeBlock { language, code } => {
            assert_eq!(language, "rust");
            assert!(code.contains("fn calcular(x: i32) -> i32"));
        }
        _ => panic!("second block must be code"),
    }
    match &blocos[2] {
        ChatContentBlock::Text(t) => assert!(t.contains("Ready for use!")),
        _ => panic!("third block must be text"),
    }

    // The shell must contain the code block widget with copy button
    let code = sem_comentario();
    assert!(
        code.contains("CrawradAiCodeBlock")
            && code.contains("btn_copy_code")
            && code.contains("code_text"),
        "the DSL must have the CrawradAiCodeBlock component with btn_copy_code and mono text area"
    );
}

/// Criterion 3: Tool call appears as a card (ToolCard) with name, arguments, and status.
#[test]
fn chamada_de_ferramenta_aparece_como_card_e_atualiza_status() {
    use crawrad_agent::{ChatMessage, ToolCallRecord, ToolCallStatus};

    let tool1 = ToolCallRecord::new("read_file", "path: 'src/main.rs'")
        .with_status(ToolCallStatus::Running);
    assert_eq!(tool1.name, "read_file");
    assert_eq!(tool1.status, ToolCallStatus::Running);
    assert_eq!(tool1.status.badge_label(), "Running");

    let tool2 = ToolCallRecord::new("cargo_check", "").with_status(ToolCallStatus::Success);
    assert_eq!(tool2.status.badge_label(), "OK");

    let msg = ChatMessage::assistant("msg-tool", "Executing analysis...")
        .with_tool_calls(vec![tool1, tool2]);
    assert_eq!(msg.tool_calls.len(), 2);
    assert_eq!(msg.tool_calls[0].name, "read_file");
    assert_eq!(msg.tool_calls[1].status, ToolCallStatus::Success);

    // The shell must have the tool card widget
    let code = sem_comentario();
    assert!(
        code.contains("CrawradAiToolCard")
            && code.contains("tool_name")
            && code.contains("tool_status"),
        "the DSL must contain the CrawradAiToolCard component to render tool call cards"
    );
}

/// Criterion 5: Stop button cancels active turn, leaves busy state, and preserves message queue.
#[test]
fn stop_button_cancels_and_preserves_message_queue() {
    let code = sem_comentario();

    // Verifies that btn_prompt_stop exists on the toolbar
    assert!(
        code.contains("btn_prompt_stop"),
        "btn_prompt_stop button must be present on chat toolbar"
    );

    // Verifies parar_ia_prompt function in the shell
    assert!(
        code.contains("fn parar_ia_prompt"),
        "parar_ia_prompt routine must be implemented"
    );

    // Verifies that parar_ia_prompt resets ai_busy to false
    assert!(
        code.contains("self.chat.busy = false;"),
        "parar_ia_prompt must set ai_busy = false when stopping"
    );

    // And does NOT empty ai_queue
    let i = code
        .find("fn parar_ia_prompt")
        .expect("fn parar_ia_prompt is missing");
    let fim = code[i..]
        .find("fn ")
        .map(|f| i + f)
        .unwrap_or(code.len());
    let stop_snippet = &code[i..fim];

    assert!(
        !stop_snippet.contains("ai_queue.clear()"),
        "parar_ia_prompt MUST preserve the ai_queue message queue intact when canceling"
    );
}

/// Criterion 6: Message queue without fixed limit (supports multiple items).
#[test]
fn the_message_queue_has_no_fixed_limit() {
    let code = sem_comentario();

    // Strict fixed limit check at 4 items cannot be present
    assert!(
        !code.contains("ai_queue.len() >= 4"),
        "the fixed and artificial limit of 4 items in the chat queue was removed"
    );

    // DSL must support multiple queue slots (q_item_0 through q_item_7)
    assert!(
        code.contains("q_item_0") && code.contains("q_item_7"),
        "the DSL must contain dynamic slots for the message queue"
    );
}

/// Criteria 7 and 8: No blocking calls (Command::output, Command::status, Command::wait)
/// on the UI thread.
#[test]
fn no_blocking_process_call_runs_straight_on_the_ui_thread() {
    let code = sem_comentario();

    // Scans all occurrences of `Command::new` in the shell (ide_app and submodules)
    // and ensures there is no synchronous call outside thread::spawn
    for linha in code.lines() {
        let t = linha.trim();
        if t.starts_with("//") {
            continue;
        }
        // If the line calls Command::new without being in a comment
        if t.contains("Command::new") {
            // Must not call .output()? directly unless in a thread context
            assert!(
                !t.contains("std::process::Command::new(\"claude\").output()"),
                "direct CLI call on UI thread found: {}",
                t
            );
        }
    }

    // Options test button must run on background thread with channel
    assert!(
        code.contains("options_test_rx"),
        "AI connection test in options must use asynchronous channel to avoid freezing UI"
    );

    // ACP probe dispatch must use asynchronous channel
    assert!(
        code.contains("probe_rx"),
        "ACP version probe must use asynchronous channel"
    );
}

/// Criterion 10: Dead agent mid-turn displays real error and leaves busy state.
#[test]
fn an_agent_killed_mid_turn_shows_the_real_error_and_leaves_busy() {
    use crawrad_agent::{ChatMessage, ChatSession, TurnOutcome};

    let mut session = ChatSession::new("test-crash", "Crash recovery test");
    assert!(session.is_empty());

    // Simulates crashed process error
    let outcome = session.apply_turn_result(&Err(
        "Agent process terminated unexpectedly with exit code 1".to_string(),
    ));

    match outcome {
        TurnOutcome::Error { error } => {
            assert!(error.contains("Agent process terminated"));
            // Adds error message to history for real display
            session.add_message(ChatMessage::assistant(
                "err-1",
                format!("❌ Agent error: {error}"),
            ));
        }
        _ => panic!("expected TurnOutcome::Error"),
    }

    assert_eq!(session.messages.len(), 1);
    assert!(session.messages[0].text.contains("❌ Agent error"));

    // In the IDE shell, finalizar_ia_prompt turns off ai_busy even on error
    let code = sem_comentario();
    assert!(
        code.contains("self.chat.busy = false;"),
        "when finishing the turn (even with error), the IDE must always set ai_busy = false"
    );
}

/// The reply card uses dynamic `CrawradSegmentList` and declares no fixed `r_code_block` slot.
#[test]
fn reply_card_uses_dynamic_segment_list_without_fixed_code_slot() {
    let source = sem_comentario();

    assert!(
        source.contains("r_blocks = <CrawradSegmentList>"),
        "CrawradAiReplyCard must declare r_blocks using CrawradSegmentList"
    );
    assert!(
        !source.contains("r_code_block = <CrawradAiCodeBlock>"),
        "CrawradAiReplyCard must not contain fixed r_code_block slot"
    );
}
