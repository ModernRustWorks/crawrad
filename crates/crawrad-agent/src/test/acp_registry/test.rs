use super::*;
use std::path::Path;

fn fixture_json() -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/test/fixtures/registry.json"
    );
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("Cannot read fixture {path}: {e}"))
}

#[test]
fn fixture_parses_all_41_agents() {
    let agents = parse_registry_json(&fixture_json()).unwrap();
    assert!(
        agents.len() >= 41,
        "Expected >= 41 agents, got {}",
        agents.len()
    );
}

#[test]
fn fixture_contains_known_official_agents() {
    let agents = parse_registry_json(&fixture_json()).unwrap();
    assert!(
        agents.iter().any(|a| a.id == "claude-acp"),
        "claude-acp missing"
    );
    assert!(agents.iter().any(|a| a.id == "gemini"), "gemini missing");
    assert!(
        agents.iter().any(|a| a.id == "codex-acp"),
        "codex-acp missing"
    );
}

#[test]
fn fixture_agents_have_required_fields() {
    let agents = parse_registry_json(&fixture_json()).unwrap();
    for a in &agents {
        assert!(!a.id.is_empty(), "Agent has empty id");
        assert!(!a.name.is_empty(), "Agent {} has empty name", a.id);
        assert!(
            !a.provider_key.is_empty(),
            "Agent {} has empty provider_key",
            a.id
        );
    }
}

#[test]
fn search_filters_by_name_id_author_description_tags() {
    let agents = parse_registry_json(&fixture_json()).unwrap();

    let filter = |q: &str| -> Vec<AcpAgentEntry> {
        let q = q.to_lowercase();
        agents
            .iter()
            .filter(|a| {
                a.name.to_lowercase().contains(&q)
                    || a.id.to_lowercase().contains(&q)
                    || a.author.to_lowercase().contains(&q)
                    || a.description.to_lowercase().contains(&q)
                    || a.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .cloned()
            .collect()
    };

    let claude = filter("claude");
    assert!(!claude.is_empty(), "claude search should return results");

    let nada = filter("zzz_nonexistent_zzz");
    assert!(nada.is_empty(), "nonsense search should return nothing");
}

#[test]
fn link_text_changes_with_query() {
    let empty = registry_link_text("");
    assert!(empty.contains("Browse all"), "Empty query: {empty}");

    let with_q = registry_link_text("claude");
    assert!(with_q.contains("claude"), "With query: {with_q}");
    assert!(with_q.contains("Search"), "With query: {with_q}");
}

#[test]
fn browse_url_encodes_query() {
    let url = registry_browse_url("");
    assert_eq!(url, ACP_PORTAL_URL);

    let url_q = registry_browse_url("hello world");
    assert!(url_q.contains("hello+world"), "URL: {url_q}");
}

#[test]
fn registry_cdn_payload_parses_sample() {
    let json = r#"{
        "version": "1.0.0",
        "agents": [
            {
                "id": "sample-agent",
                "name": "Sample Agent",
                "description": "Test description",
                "authors": ["Test Author"],
                "distribution": { "npx": { "package": "sample-pkg@1.0" } }
            }
        ]
    }"#;
    let agents = parse_registry_json(json).unwrap();
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].id, "sample-agent");
    assert_eq!(agents[0].name, "Sample Agent");
    assert_eq!(agents[0].package_cmd, "sample-pkg@1.0");
}

#[test]
fn launch_for_gemini_npx() {
    let agents = parse_registry_json(&fixture_json()).unwrap();
    let gemini = agents.iter().find(|a| a.id == "gemini").unwrap();
    let launch = launch_for(gemini, "windows-x86_64", Path::new("C:/fake/agents")).unwrap();
    assert_eq!(launch.program, "npx");
    assert_eq!(
        launch.args,
        vec![
            "-y".to_string(),
            "@google/gemini-cli@0.61.0".to_string(),
            "--acp".to_string()
        ]
    );
}

#[test]
fn launch_for_antigravity_windows() {
    let agents = parse_registry_json(&fixture_json()).unwrap();
    let agy = agents.iter().find(|a| a.id == "antigravity-acp").unwrap();
    let temp_agents = Path::new("C:/fake/agents");
    let launch = launch_for(agy, "windows-x86_64", temp_agents).unwrap();
    let expected = temp_agents
        .join("antigravity-acp")
        .join("1.2.1")
        .join("agy_acp_server.exe");
    assert_eq!(launch.program, expected.to_string_lossy());
    assert!(launch.args.is_empty());
}

#[test]
fn launch_for_antigravity_linux() {
    let agents = parse_registry_json(&fixture_json()).unwrap();
    let agy = agents.iter().find(|a| a.id == "antigravity-acp").unwrap();
    let temp_agents = Path::new("/fake/agents");
    let launch = launch_for(agy, "linux-x86_64", temp_agents).unwrap();
    let expected = temp_agents
        .join("antigravity-acp")
        .join("1.2.1")
        .join("agy_acp_server.par");
    assert_eq!(launch.program, expected.to_string_lossy());
    assert_eq!(launch.args, vec!["--uid=".to_string()]);
}

#[test]
fn launch_for_agent_without_platform_build() {
    let agents = parse_registry_json(&fixture_json()).unwrap();
    let amp = agents.iter().find(|a| a.id == "amp-acp").unwrap();
    let res = launch_for(amp, "windows-aarch64", Path::new("C:/fake/agents"));
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(err.contains("has no build for windows-aarch64"));
}

/// **The page resolves the agent the way the launch does**: the
/// saved `agent_id: "codex"` is the registry's `codex-acp` through its
/// profile — the lookup by command missed it, and the page said "not
/// found" while the chat answered through Codex.
#[test]
fn the_active_agent_is_found_by_its_profile_id() {
    let agents = parse_registry_json(&fixture_json()).expect("fixture");
    let cfg = crate::AcpConfig {
        agent_id: "codex".into(),
        endpoint: "stdio:codex".into(),
        provider: "OpenAI / Codex ACP Bridge".into(),
        ..Default::default()
    };
    let entry = active_entry(&cfg, &agents).expect("codex resolves");
    assert_eq!(entry.id, "codex-acp");
}

/// A custom agent is judged by the command it runs, and its link is never
/// another vendor's documentation.
#[test]
fn a_custom_agent_is_installed_when_its_command_is() {
    let cfg = crate::AcpConfig {
        agent_id: "custom".into(),
        endpoint: "stdio:cargo".into(),
        provider: "My Agent".into(),
        ..Default::default()
    };
    let op = get_operational_status(&cfg, None);
    assert_eq!(op.status_title, "My Agent is installed and ready.");
    assert!(!op.target_url.contains("anthropic"), "{}", op.target_url);
    let missing = crate::AcpConfig {
        endpoint: "stdio:crawrad-no-such-agent-xyz".into(),
        ..cfg
    };
    let op = get_operational_status(&missing, None);
    assert!(
        op.status_title.contains("is not installed"),
        "{}",
        op.status_title
    );
}

#[test]
fn agent_state_resolution_rules() {
    // connected-but-not-found => Ready
    let state = agent_state(false, true, None);
    assert_eq!(state, AgentState::Ready);
    assert_eq!(state.label(), "Ready");
    assert_eq!(state.fix(), None);

    // auth error => SignedOut
    let auth_err = r#"{"code": -32000, "message": "Auth required"}"#;
    let state = agent_state(false, false, Some(auth_err));
    assert_eq!(state, AgentState::SignedOut);
    assert_eq!(state.label(), "Signed out");
    assert_eq!(state.fix(), Some("Sign in"));

    // other error => NotInstalled
    let other_err = "failed to start process";
    let state = agent_state(false, false, Some(other_err));
    assert_eq!(state, AgentState::NotInstalled);

    // nothing => NotInstalled
    let state = agent_state(false, false, None);
    assert_eq!(state, AgentState::NotInstalled);
    assert_eq!(state.label(), "Not installed");
    assert_eq!(state.fix(), Some("Install"));

    // installed => Ready
    let state = agent_state(true, false, None);
    assert_eq!(state, AgentState::Ready);
    assert_eq!(state.label(), "Ready");
    assert_eq!(state.fix(), None);
}

#[test]
fn agent_state_hints() {
    assert_eq!(
        AgentState::Ready.hint(true, "claude-agent"),
        "Connected and answering in the chat."
    );
    assert_eq!(
        AgentState::Ready.hint(false, "claude-agent"),
        "Installed and ready."
    );
    assert_eq!(
        AgentState::NotInstalled.hint(false, "my-agent"),
        "The command my-agent was not found."
    );
    assert_eq!(
        AgentState::SignedOut.hint(false, "my-agent"),
        "The agent asks you to sign in."
    );
}

#[test]
fn operational_status_bare_command_and_installed_flag() {
    let cfg = crate::AcpConfig {
        agent_id: "custom".into(),
        endpoint: "stdio:cargo".into(),
        provider: "My Agent".into(),
        ..Default::default()
    };
    let op = get_operational_status(&cfg, None);
    assert_eq!(op.command, "stdio:cargo");
    assert!(op.is_installed);

    let missing = crate::AcpConfig {
        endpoint: "stdio:crawrad-no-such-agent-xyz".into(),
        ..cfg
    };
    let op_missing = get_operational_status(&missing, None);
    assert_eq!(op_missing.command, "stdio:crawrad-no-such-agent-xyz");
    assert!(!op_missing.is_installed);
}

#[test]
fn chat_status_line_every_state() {
    assert_eq!(
        chat_status_line(AgentState::Ready, true),
        "🟢 Connected · ready"
    );
    assert_eq!(
        chat_status_line(AgentState::Ready, false),
        "🟢 Ready · connects on your first message"
    );
    assert_eq!(
        chat_status_line(AgentState::SignedOut, false),
        "🟡 Signed out · sign in at Tools › Options › AI"
    );
    assert_eq!(
        chat_status_line(AgentState::SignedOut, true),
        "🟡 Signed out · sign in at Tools › Options › AI"
    );
    assert_eq!(
        chat_status_line(AgentState::NotInstalled, false),
        "⚪ No AI agent installed · set one up at Tools › Options › AI"
    );
    assert_eq!(
        chat_status_line(AgentState::NotInstalled, true),
        "⚪ No AI agent installed · set one up at Tools › Options › AI"
    );
}

#[test]
fn hero_labels_carry_the_real_agent_state() {
    use crate::acp_registry::AgentState;
    let labels = crate::chat_layout::hero_labels(&[], AgentState::NotInstalled, false);
    assert_eq!(labels[0], ("lbl_session_count", "0 messages".to_string()));
    assert_eq!(labels[1].0, "lbl_session_status");
    assert_eq!(
        labels[2],
        (
            "lbl_ready_status",
            "⚪ No AI agent installed · set one up at Tools › Options › AI".to_string()
        )
    );
    let ready = crate::chat_layout::hero_labels(&[], AgentState::Ready, true);
    assert_eq!(ready[2].1, "🟢 Connected · ready");
}
