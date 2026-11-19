use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::*;
use crate::acp::AcpConfig;
use crate::acp_client_test::get_mock_acp_agent_binary;
use crate::permission::SessionPermissions;

fn mock_config() -> AcpConfig {
    AcpConfig {
        agent_id: "custom".to_string(),
        provider: "Custom".to_string(),
        endpoint: get_mock_acp_agent_binary().to_string_lossy().to_string(),
        endpoint_args: vec![],
        auto_approve_reads: true,
        ..AcpConfig::default()
    }
}

/// Polls until the warm-up answers, for at most ten seconds.
fn settle(live: &mut LiveAgent) {
    for _ in 0..200 {
        if live.poll() {
            return;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    panic!("the warm-up never answered");
}

#[test]
fn ensure_warm_starts_once_per_agent_and_folder() {
    let cfg = mock_config();
    let perms = Arc::new(Mutex::new(SessionPermissions::new()));
    let mut live = LiveAgent::new();
    assert!(live.ensure_warm(&cfg, None, perms.clone()));
    assert_eq!(live.status(), &LiveStatus::Connecting);
    assert!(
        !live.ensure_warm(&cfg, None, perms.clone()),
        "a warm-up in flight is not restarted"
    );
    settle(&mut live);
    assert!(
        matches!(live.status(), LiveStatus::Ready { .. }),
        "{:?}",
        live.status()
    );
    assert!(
        !live.ensure_warm(&cfg, None, perms.clone()),
        "a live agent is kept"
    );
    let folder = std::env::temp_dir();
    assert!(
        live.ensure_warm(&cfg, Some(folder), perms.clone()),
        "another folder replaces it"
    );
    assert_eq!(live.status(), &LiveStatus::Connecting);
}

#[test]
fn a_failed_agent_waits_for_an_explicit_retry() {
    let cfg = AcpConfig {
        endpoint: "crawrad-agent-that-does-not-exist-429".to_string(),
        ..mock_config()
    };
    let perms = Arc::new(Mutex::new(SessionPermissions::new()));
    let mut live = LiveAgent::new();
    assert!(live.ensure_warm(&cfg, None, perms.clone()));
    settle(&mut live);
    assert!(
        matches!(live.status(), LiveStatus::Failed(_)),
        "{:?}",
        live.status()
    );
    // Layout passes call ensure_warm all the time: a Failed agent must not respawn.
    assert!(!live.ensure_warm(&cfg, None, perms.clone()));
    live.reset();
    assert!(live.ensure_warm(&cfg, None, perms), "reset is the retry");
}

#[test]
fn key_of_changes_with_agent_endpoint_and_args() {
    let c1 = AcpConfig {
        agent_id: "claude".to_string(),
        endpoint: "stdio:claude-agent".to_string(),
        endpoint_args: vec!["--stdio".to_string()],
        ..AcpConfig::default()
    };

    let mut c2 = c1.clone();
    assert_eq!(LiveAgent::key_of(&c1), LiveAgent::key_of(&c2));

    // Change agent_id
    c2.agent_id = "gemini".to_string();
    assert_ne!(LiveAgent::key_of(&c1), LiveAgent::key_of(&c2));

    // Change endpoint
    let mut c3 = c1.clone();
    c3.endpoint = "stdio:gemini-agent".to_string();
    assert_ne!(LiveAgent::key_of(&c1), LiveAgent::key_of(&c3));

    // Change endpoint_args
    let mut c4 = c1.clone();
    c4.endpoint_args = vec!["--verbose".to_string()];
    assert_ne!(LiveAgent::key_of(&c1), LiveAgent::key_of(&c4));
}

#[test]
fn status_transitions_and_accessors() {
    let mut live = LiveAgent::new();
    assert_eq!(live.status(), &LiveStatus::Idle);
    assert!(live.client().is_none());
    assert!(live.agent_key().is_empty());

    live.set_connecting("agent-1".to_string());
    assert_eq!(live.status(), &LiveStatus::Connecting);
    assert_eq!(live.agent_key(), "agent-1");
    assert!(live.client().is_none());

    live.set_failed("Connection refused".to_string());
    assert_eq!(
        live.status(),
        &LiveStatus::Failed("Connection refused".to_string())
    );
    assert!(live.client().is_none());

    live.reset();
    assert_eq!(live.status(), &LiveStatus::Idle);
    assert!(live.agent_key().is_empty());
    assert!(live.client().is_none());
}

#[test]
fn mock_agent_warm_and_session_continuity_across_prompts() {
    let cfg = mock_config();

    let perms = Arc::new(Mutex::new(SessionPermissions::new()));
    let rx = LiveAgent::warm(&cfg, None, vec![], perms);

    let res = rx
        .recv_timeout(Duration::from_secs(10))
        .expect("warm timed out");
    let (client, session_id) = res.expect("warm failed");
    assert!(!session_id.is_empty(), "session id must not be empty");

    let mut live = LiveAgent::new();
    live.set_ready(Arc::clone(&client), session_id.clone());
    assert_eq!(
        live.status(),
        &LiveStatus::Ready {
            session: session_id.clone()
        }
    );
    assert!(live.client().is_some());

    // 1st prompt on the live client
    let (status_tx, _status_rx) = std::sync::mpsc::channel();
    let mut feed = crate::acp_status::StatusFeed::start("Mock", false, status_tx);
    let live_c = live.client().expect("client is ready");
    let turn1 = execute_client_turn(
        &live_c,
        "Hello Turn 1",
        &[],
        None,
        vec![],
        &mut feed,
        || false,
        |_| {},
        |_| {},
    )
    .expect("turn 1 succeeded");
    assert_eq!(
        turn1.2,
        Via::Acp {
            session: Some(session_id.clone())
        }
    );
    assert!(
        turn1.0.contains(&format!("Echo on session {session_id}")),
        "turn 1 response should echo session {session_id}, got: {}",
        turn1.0
    );

    // 2nd prompt on the SAME live client carries the SAME session
    let turn2 = execute_client_turn(
        &live_c,
        "Hello Turn 2",
        &[],
        None,
        vec![],
        &mut feed,
        || false,
        |_| {},
        |_| {},
    )
    .expect("turn 2 succeeded");
    assert_eq!(
        turn2.2,
        Via::Acp {
            session: Some(session_id.clone())
        }
    );
    assert!(
        turn2.0.contains(&format!("Echo on session {session_id}")),
        "turn 2 response should echo session {session_id}, got: {}",
        turn2.0
    );

    // Verify active_session_id matches on the client
    assert_eq!(
        live_c.active_session_id().as_deref(),
        Some(session_id.as_str())
    );
}

#[test]
fn live_agent_config_options_choose_and_session_choices_persistence() {
    let mut cfg = mock_config();
    let perms = Arc::new(Mutex::new(SessionPermissions::new()));
    let mut live = LiveAgent::new();

    assert!(live.ensure_warm(&cfg, None, perms.clone()));
    settle(&mut live);
    assert!(matches!(live.status(), LiveStatus::Ready { .. }));

    let opts = live.config_options();
    assert!(!opts.is_empty(), "config_options should not be empty");

    let model_menu = crate::chat_config::menu_for(&opts, "model").expect("model menu");
    assert_eq!(model_menu.items[0].value, "mock-fast");
    assert!(model_menu.items[0].selected);

    let effort_menu = crate::chat_config::menu_for(&opts, "thought_level").expect("effort menu");
    assert_eq!(effort_menu.items[1].value, "medium");
    assert!(effort_menu.items[1].selected);

    // Apply choose("model", "mock-smart")
    live.choose("model", "mock-smart");
    // Optimistic pending choice immediately reflected:
    let opt_opts = live.config_options();
    let opt_model_menu = crate::chat_config::menu_for(&opt_opts, "model").expect("model menu");
    let smart_item = opt_model_menu
        .items
        .iter()
        .find(|i| i.value == "mock-smart")
        .unwrap();
    assert!(
        smart_item.selected,
        "optimistic update must reflect mock-smart"
    );

    // Settle choose
    settle(&mut live);
    assert!(live.last_error().is_none());

    let settled_opts = live.config_options();
    let settled_model_menu =
        crate::chat_config::menu_for(&settled_opts, "model").expect("model menu");
    let smart_item_settled = settled_model_menu
        .items
        .iter()
        .find(|i| i.value == "mock-smart")
        .unwrap();
    assert!(smart_item_settled.selected);

    // Record session choices and perform second warm
    cfg.record_session_choice("model", "mock-smart");
    let mut live2 = LiveAgent::new();
    assert!(live2.ensure_warm(&cfg, None, perms));
    settle(&mut live2);
    assert!(matches!(live2.status(), LiveStatus::Ready { .. }));

    let opts2 = live2.config_options();
    let model_menu2 = crate::chat_config::menu_for(&opts2, "model").expect("model menu");
    let smart_item2 = model_menu2
        .items
        .iter()
        .find(|i| i.value == "mock-smart")
        .unwrap();
    assert!(
        smart_item2.selected,
        "second warm with session_choices must come up with mock-smart selected"
    );
}

#[test]
fn a_turn_that_uses_a_tool_streams_its_status_line_and_returns_the_last() {
    let cfg = mock_config();
    let perms = Arc::new(Mutex::new(SessionPermissions::new()));
    let (status_tx, status_rx) = std::sync::mpsc::channel();
    let req = TurnRequest {
        config: cfg,
        live: None,
        prompt: "please use a tool".to_string(),
        images: vec![],
        cwd: None,
        buffers: BTreeMap::new(),
        perms,
        cli_session: None,
        agent: "Mock".to_string(),
        status: status_tx,
    };
    let (text, _tools, via, last) =
        run_turn(req, || false, |_| {}, |_| {}).expect("the turn answers");
    assert!(text.contains("use a tool"), "{text}");
    assert!(matches!(via, Via::Acp { .. }), "{via:?}");
    assert!(last.done);
    assert!(last.line.ends_with(" · 1 tool"), "{}", last.line);

    let lines: Vec<String> = status_rx.try_iter().map(|s| s.line).collect();
    let expected = [
        "ACP session started. Sending the prompt to Mock\u{2026}",
        "Thinking: Looking for the tests\u{2026}",
        "Running tool: cargo test -p app",
        "Tool done: cargo test -p app (ok)",
    ];
    assert_eq!(&lines[..expected.len()], &expected[..], "{lines:?}");
    assert_eq!(lines.last(), Some(&last.line));
}

/// Is process `pid` still running? (`tasklist` answers with its row, or with "INFO: No tasks".)
#[cfg(windows)]
fn alive(pid: &str) -> bool {
    let out = std::process::Command::new("tasklist")
        .args(["/FI", &format!("PID eq {pid}"), "/NH"])
        .output()
        .expect("tasklist");
    String::from_utf8_lossy(&out.stdout).contains(pid)
}

/// An `npx` agent is `npx` → `cmd` → `node`: killing the process the client
/// spawned left its children orphaned, one pair per agent switch or test run.
/// Here the client spawns `cmd /c <mock>`; the mock's pid comes back in its
/// session id (`mock-session-<pid>-<n>`), and dropping the client must end it.
#[cfg(windows)]
#[test]
fn dropping_the_client_ends_the_agents_children_too() {
    let mock = get_mock_acp_agent_binary();
    // `--linger`: the mock outlives the end of its stdin, as `node` does
    let args = vec![
        "/c".to_string(),
        mock.to_string_lossy().to_string(),
        "--linger".to_string(),
    ];
    let cmd =
        std::path::PathBuf::from(std::env::var("ComSpec").unwrap_or_else(|_| "cmd.exe".into()));
    let client = AcpClient::spawn(&cmd, &args, None, true).expect("spawn cmd /c mock");
    client.initialize().expect("initialize");
    let session = client.new_session(None, vec![]).expect("session/new");
    let pid = session
        .split('-')
        .nth(2)
        .expect("mock-session-<pid>-<n>")
        .to_string();
    assert!(
        alive(&pid),
        "the mock (a child of cmd) runs while the client lives"
    );
    // without the tree the drop BLOCKED until the orphan let go of the pipe (the
    // reader waits for EOF): on the UI thread, a frozen IDE at every agent switch
    let started = std::time::Instant::now();
    drop(client);
    let took = started.elapsed();
    assert!(
        took < Duration::from_secs(5),
        "dropping the client blocked {took:?} on the agent's orphan"
    );
    let gone = (0..20).any(|_| {
        std::thread::sleep(Duration::from_millis(100));
        !alive(&pid)
    });
    assert!(
        gone,
        "the agent's child {pid} outlived the client: an orphan"
    );
}

#[test]
fn a_reopened_conversation_resumes_its_acp_session_and_drops_the_replay() {
    // The first conversation's session, reopened from the history
    let cfg = mock_config();
    let perms = Arc::new(Mutex::new(SessionPermissions::new()));
    let mut first = LiveAgent::new();
    assert!(first.ensure_warm(&cfg, None, perms.clone()));
    settle(&mut first);
    let old = first.session_id().expect("first session").to_string();
    assert!(first.can_resume(), "the mock declares loadSession");

    let mut reopened = LiveAgent::new();
    reopened.reset();
    reopened.resume_next(Some(old.clone()));
    reopened.reset(); // the shell resets before warming: the session to resume survives it
    assert!(reopened.ensure_warm(&cfg, None, perms.clone()));
    settle(&mut reopened);
    assert_eq!(
        reopened.session_id(),
        Some(old.as_str()),
        "{:?}",
        reopened.status()
    );
    assert!(
        !reopened.config_options().is_empty(),
        "the load's configOptions feed the menus"
    );

    let (status_tx, _status_rx) = std::sync::mpsc::channel();
    let mut feed = crate::acp_status::StatusFeed::start("Mock", false, status_tx);
    let client = reopened.client().expect("ready");
    let turn = execute_client_turn(
        &client,
        "after reopening",
        &[],
        None,
        vec![],
        &mut feed,
        || false,
        |_| {},
        |_| {},
    )
    .expect("turn succeeded");
    assert!(
        turn.0.contains(&format!("Echo on session {old}")),
        "{}",
        turn.0
    );
    assert!(
        !turn.0.contains("REPLAYED"),
        "the load's replay leaked into the turn: {}",
        turn.0
    );

    // without a session to resume, a new one, as before
    let mut fresh = LiveAgent::new();
    assert!(fresh.ensure_warm(&cfg, None, perms));
    settle(&mut fresh);
    assert_ne!(fresh.session_id(), Some(old.as_str()));
}

#[test]
fn live_agent_sends_images_to_agent_accepting_images() {
    let cfg = mock_config();
    let perms = Arc::new(Mutex::new(SessionPermissions::new()));
    let mut live = LiveAgent::new();
    assert!(live.ensure_warm(&cfg, None, perms));
    settle(&mut live);
    let client = live.client().expect("live client");
    assert!(client.accepts_images());

    let temp_dir =
        std::env::temp_dir().join(format!("crawrad-test-live-img-{}", uuid::Uuid::new_v4()));
    let mut buf = Vec::new();
    let mut encoder = png::Encoder::new(&mut buf, 1, 1);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().expect("png header");
    writer
        .write_image_data(&[255, 0, 0, 255])
        .expect("png data");
    drop(writer);

    let att =
        crate::chat_attachment::ImageAttachment::save(&temp_dir, &buf).expect("save test image");
    let (status_tx, _status_rx) = std::sync::mpsc::channel();
    let mut feed = crate::acp_status::StatusFeed::start("Mock", false, status_tx);

    let turn = execute_client_turn(
        &client,
        "check image",
        &[att],
        None,
        vec![],
        &mut feed,
        || false,
        |_| {},
        |_| {},
    )
    .expect("turn with image succeeded");

    assert!(
        turn.0.contains("[images: 1]"),
        "expected [images: 1] in response: {}",
        turn.0
    );
    let _ = std::fs::remove_dir_all(&temp_dir);
}
