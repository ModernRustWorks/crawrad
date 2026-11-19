//! Slash command suite tests.

use super::*;
use std::collections::HashSet;

#[test]
fn all_commands_integrity_and_forbidden_words() {
    let mut seen_ids = HashSet::new();
    let forbidden: Vec<String> = vec![
        ["cla", "ude"].concat(),
        ["gem", "ini"].concat(),
        ["anti", "gravity"].concat(),
        ["co", "pilot"].concat(),
        ["open", "ai"].concat(),
        ["co", "dex"].concat(),
        ["ju", "nit"].concat(),
        ["py", "test"].concat(),
        ["je", "st"].concat(),
        ["nu", "nit"].concat(),
        ["xu", "nit"].concat(),
        ["unit", "test"].concat(),
        ["mo", "cha"].concat(),
        ["rs", "pec"].concat(),
    ];

    for cmd in SlashCommand::ALL {
        assert_eq!(cmd.trigger, format!("/{}", cmd.id));
        assert!(
            seen_ids.insert(cmd.id),
            "Duplicate slash command id: {}",
            cmd.id
        );

        let trigger_lower = cmd.trigger.to_lowercase();
        let desc_lower = cmd.description.to_lowercase();
        for word in &forbidden {
            assert!(
                !trigger_lower.contains(word),
                "Trigger {} contains forbidden word",
                cmd.trigger
            );
            assert!(
                !desc_lower.contains(word),
                "Description of {} contains forbidden word",
                cmd.trigger
            );
        }
    }
}

#[test]
fn match_exact_first_word_and_removed_aliases() {
    let matched = SlashCommand::match_exact("/compare Rc Arc");
    assert!(matched.is_some());
    assert_eq!(matched.unwrap().id, "compare");

    let removed = [
        "/acoes",
        &["/", "cla", "ude"].concat(),
        &["/", "gem", "ini"].concat(),
        "/project",
        "/options",
        "/optimize",
        "/commands",
        "/run",
    ];
    for r in removed {
        assert!(
            SlashCommand::match_exact(r).is_none(),
            "Expected {r} to be None"
        );
    }
}

#[test]
fn resolve_use_targets_and_lookup() {
    let targets = use_targets();
    assert!(!targets.is_empty());

    let resolved_by_id = resolve_use("opencode");
    assert_eq!(resolved_by_id, Some("OpenCode / Local (Ollama)"));

    let resolved_by_display = resolve_use("OpenCode / Local (Ollama)");
    assert_eq!(resolved_by_display, Some("OpenCode / Local (Ollama)"));

    assert_eq!(resolve_use("nope"), None);
}

#[test]
fn help_lists_every_command_in_all() {
    let help = help_text();
    for item in SlashCommand::ALL {
        assert!(help.contains(item.trigger), "{}", item.trigger);
    }
}

#[test]
fn plan_auto_approve_labels() {
    assert_eq!(
        auto_approve_labels(true),
        ("ENABLED (ON)", "Auto-approve: ON")
    );
    assert_eq!(auto_approve_labels(false).1, "Auto-approve: OFF");
}

/// **`/use`:** no name lists the agents with the active one marked; a known
/// name switches; an unknown one says so and switches nothing.
#[test]
fn use_reply_lists_switches_or_refuses() {
    let (id, display) = use_targets()[0];
    let (none, list) = use_reply("", display, id);
    assert!(none.is_none() && list.contains("(active)") && list.contains(id));
    assert_eq!(use_reply(id, "", "").0, Some(display));
    let (none, said) = use_reply("nope", "", "");
    assert!(none.is_none() && said.contains("nope"));
}

#[test]
fn plan_and_acp_helper_strings() {
    assert_eq!(plan_status_label(true), "🟢 Auto-Approved");
    assert_eq!(plan_status_label(false), "Pending Review");
    assert_eq!(
        plan_prompt_for("test task"),
        "Create a step-by-step implementation plan for: test task"
    );
    assert!(plan_usage().contains("/plan <objective>"));
    assert_eq!(
        plan_auto_toggled_message("ENABLED (ON)"),
        "⚡ ACP Auto-Approve Plans is now ENABLED (ON)."
    );

    assert!(acp_probing_message().contains("Probing ACP agent connection"));
    assert!(acp_ask_usage().contains("/acp ask <your prompt>"));
    assert!(acp_commands_help().contains("/acp test"));
}

#[test]
fn review_command_lookup_and_prompt() {
    let filtered = SlashCommand::filter("/rev");
    assert!(filtered.iter().any(|c| c.id == "review"));

    let matched = SlashCommand::match_exact("/review");
    assert_eq!(matched.map(|c| c.id), Some("review"));

    let prompt_empty = build_command_prompt("/review", "").expect("review prompt");
    assert!(prompt_empty.contains("do NOT edit"));
    assert!(prompt_empty.contains("line"));

    let prompt_unsafe =
        build_command_prompt("/review", "unsafe").expect("review prompt with focus");
    assert!(prompt_unsafe.contains("Focus on: unsafe"));
}
