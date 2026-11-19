//! Integration tests for the ACP Continuous Client (ADR 0029).
#![forbid(unsafe_code)]

use std::path::PathBuf;
use std::time::Duration;

use crate::client::{AcpClient, AcpStreamEvent};

pub(crate) fn get_mock_acp_agent_binary() -> PathBuf {
    // The test runs from `<target>/debug/deps/<test>.exe`, and cargo puts the
    // binaries one level up, in `<target>/debug/`. Deriving the folder from the
    // test's own executable is what makes this work with any `CARGO_TARGET_DIR`
    // (a fixed `target/debug` only worked where a stale binary happened to be).
    let exe = std::env::current_exe().unwrap();
    let debug_dir = exe
        .parent()
        .and_then(|deps| deps.parent())
        .expect("the test runs from <target>/debug/deps")
        .to_path_buf();
    let binary = debug_dir.join(if cfg!(windows) {
        "mock_acp_agent.exe"
    } else {
        "mock_acp_agent"
    });
    // A binary OLDER than its source is a mock from before the last edit: it
    // once kept speaking a dead dialect while twelve tests failed for nothing.
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/bin/mock_acp_agent.rs");
    let modified = |p: &std::path::Path| std::fs::metadata(p).and_then(|m| m.modified()).ok();
    if binary.exists() && modified(&binary) >= modified(&source) {
        return binary;
    }

    let status = std::process::Command::new("cargo")
        .args(["build", "-p", "crawrad-agent", "--bin", "mock_acp_agent"])
        .status()
        .expect("cargo build mock_acp_agent");
    assert!(status.success(), "building mock_acp_agent failed");
    assert!(
        binary.exists(),
        "mock_acp_agent was built but is not at {}",
        binary.display()
    );
    binary
}

#[test]
fn test_acp_client_handshake_and_session_new() {
    let bin = get_mock_acp_agent_binary();
    let mut client = AcpClient::spawn(&bin, &[], None, true).expect("spawn mock agent");

    // 1. Handshake `initialize` - verifies protocolVersion and clientCapabilities with fs
    let init_res = client.initialize().expect("initialize handshake");
    assert_eq!(
        init_res.get("protocolVersion").and_then(|v| v.as_i64()),
        Some(1)
    );
    let agent_name = init_res
        .get("agentInfo")
        .and_then(|a| a.get("name"))
        .and_then(|n| n.as_str());
    assert_eq!(agent_name, Some("MockAcpAgent"));

    // 2. `session/new` with absolute cwd and McpServerStdio (env: [] and absolute command)
    let cwd = std::env::current_dir().expect("cwd");
    let bridge_path = cwd.join("target").join("debug").join(if cfg!(windows) {
        "crawrad-mcp-bridge.exe"
    } else {
        "crawrad-mcp-bridge"
    });
    let mcp_servers = vec![crate::client::ide_mcp_server(&bridge_path, "test-instance")];

    let session_id = client
        .new_session(Some(&cwd), mcp_servers)
        .expect("session/new");
    assert!(session_id.starts_with("mock-session-"));
    assert_eq!(client.active_session_id(), Some(session_id));

    client.shutdown();
}

#[test]
fn test_acp_client_streaming_and_wire_continuity() {
    let bin = get_mock_acp_agent_binary();
    let mut client = AcpClient::spawn(&bin, &[], None, true).expect("spawn mock agent");

    client.initialize().expect("initialize");

    let cwd = std::env::current_dir().expect("cwd");
    let bridge_path = cwd.join("target").join("debug").join(if cfg!(windows) {
        "crawrad-mcp-bridge.exe"
    } else {
        "crawrad-mcp-bridge"
    });
    let mcp_servers = vec![crate::client::ide_mcp_server(&bridge_path, "test-inst")];

    // Turn 1
    let rx1 = client
        .prompt("First turn prompt", &[], None, mcp_servers.clone())
        .expect("turn 1 prompt");
    let session_id_1 = client
        .active_session_id()
        .expect("active session id on turn 1");

    let mut turn1_text = String::new();
    let mut got_thought = false;
    while let Ok(ev) = rx1.recv_timeout(Duration::from_secs(5)) {
        match ev {
            AcpStreamEvent::ThoughtDelta(t) => {
                assert_eq!(t, "Thinking...");
                got_thought = true;
            }
            AcpStreamEvent::TextDelta(t) => {
                turn1_text.push_str(&t);
            }
            AcpStreamEvent::Done { stop_reason } => {
                assert_eq!(stop_reason, "end_turn");
                break;
            }
            AcpStreamEvent::Error(err) => panic!("Unexpected error in turn 1: {err}"),
            _ => {}
        }
    }
    assert!(got_thought, "Should have received streaming thought");
    assert!(turn1_text.contains(&format!(
        "Echo on session {session_id_1}: First turn prompt"
    )));

    // Turn 2 (CONTINUITY MEASURED ON THE WIRE): the 2nd prompt goes on the SAME sessionId
    let rx2 = client
        .prompt("Second turn prompt", &[], None, mcp_servers)
        .expect("turn 2 prompt");
    let session_id_2 = client
        .active_session_id()
        .expect("active session id on turn 2");
    assert_eq!(
        session_id_1, session_id_2,
        "Continuity on the wire: the second prompt MUST use exactly the same sessionId as the first"
    );

    let mut turn2_text = String::new();
    while let Ok(ev) = rx2.recv_timeout(Duration::from_secs(5)) {
        match ev {
            AcpStreamEvent::TextDelta(t) => turn2_text.push_str(&t),
            AcpStreamEvent::Done { stop_reason } => {
                assert_eq!(stop_reason, "end_turn");
                break;
            }
            AcpStreamEvent::Error(err) => panic!("Unexpected error in turn 2: {err}"),
            _ => {}
        }
    }
    assert!(turn2_text.contains(&format!(
        "Echo on session {session_id_1}: Second turn prompt"
    )));

    client.shutdown();
}

#[test]
fn test_acp_client_fs_read_text_file_returns_live_buffer() {
    let bin = get_mock_acp_agent_binary();
    let mut client = AcpClient::spawn(&bin, &[], None, true).expect("spawn mock agent");

    client.initialize().expect("initialize");

    // Register a live buffer changed in the IDE (not saved to disk)
    let dirty_path = "crates/crawrad-ide/src/dirty_file.rs";
    let dirty_content = "// MODIFICACAO VIVA NO EDITOR: versao em memoria";
    client.set_live_buffer(dirty_path, dirty_content);

    // The prompt tells mock_acp_agent to call `fs/read_text_file` against the IDE
    let response = client
        .prompt_sync(&format!("TEST_READ_FILE {dirty_path}"), None, vec![])
        .expect("prompt_sync");

    assert!(
        response.contains(&format!("FileContent: {dirty_content}")),
        "fs/read_text_file MUST return the live in-memory buffer, not the disk: answer received: '{response}'"
    );

    client.shutdown();
}

#[test]
fn test_acp_client_cancel_timeout_resilience() {
    let bin = get_mock_acp_agent_binary();
    let mut client = AcpClient::spawn(&bin, &[], None, true).expect("spawn mock agent");

    client.initialize().expect("initialize");

    // Start a task that hangs/sleeps forever
    let rx = client
        .prompt("HANG_ON_CANCEL", &[], None, vec![])
        .expect("prompt hang");

    // Wait for the streaming to start
    let first = rx
        .recv_timeout(Duration::from_secs(3))
        .expect("first event");
    assert_eq!(
        first,
        AcpStreamEvent::TextDelta("Starting long task...".into())
    );

    // Cancel: sends a notification with NO id; does not fake Done locally.
    // The mock agent receives session/cancel and answers the pending prompt with stopReason: "cancelled".
    let start_cancel = std::time::Instant::now();
    let cancel_res = client.cancel();
    assert!(cancel_res.is_ok(), "cancel sent");
    let cancel_elapsed = start_cancel.elapsed();

    assert!(
        cancel_elapsed < Duration::from_millis(3500),
        "cancellation took {:?}, it must respect the timeout and free the thread",
        cancel_elapsed
    );

    let end_event = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("end event after cancel");
    assert_eq!(
        end_event,
        AcpStreamEvent::Done {
            stop_reason: "cancelled".into()
        },
        "The agent answers the pending prompt with stopReason: 'cancelled'"
    );

    client.shutdown();
}

#[test]
fn test_acp_client_fault_isolation_on_crash() {
    let bin = get_mock_acp_agent_binary();
    let mut client = AcpClient::spawn(&bin, &[], None, true).expect("spawn mock agent");

    client.initialize().expect("initialize");

    // Send a command that brings the agent process down with exit code 101
    let rx = client
        .prompt("CRASH", &[], None, vec![])
        .expect("prompt crash");

    let event = rx
        .recv_timeout(Duration::from_secs(3))
        .expect("event on crash");
    match event {
        AcpStreamEvent::Error(err) => {
            assert!(
                err.contains("101") || err.contains("terminated") || err.contains("closed"),
                "The error must be explicit and visible to the user, neither empty nor a mock-up: '{err}'"
            );
        }
        other => panic!("expected AcpStreamEvent::Error when the process crashes, got: {other:?}"),
    }

    assert!(
        !client.is_alive(),
        "The agent process must be dead after the crash"
    );
    client.shutdown();
}

#[test]
fn test_acp_mcp_servers_passed_to_agent() {
    let bin = get_mock_acp_agent_binary();
    let mut client = AcpClient::spawn(&bin, &[], None, true).expect("spawn mock agent");

    client.initialize().expect("initialize");

    let cwd = std::env::current_dir().expect("cwd");
    let bridge = cwd.join("target").join("debug").join(if cfg!(windows) {
        "crawrad-mcp-bridge.exe"
    } else {
        "crawrad-mcp-bridge"
    });
    let fs_bridge = cwd.join("target").join("debug").join(if cfg!(windows) {
        "crawrad-mcp-fs.exe"
    } else {
        "crawrad-mcp-fs"
    });

    let mcp_servers = vec![
        crate::client::ide_mcp_server(&bridge, "instance-alpha"),
        serde_json::json!({
            "name": "filesystem",
            "command": fs_bridge.to_string_lossy().to_string(),
            "args": [],
            "env": []
        }),
    ];

    let resp = client
        .prompt_sync("liste as ferramentas do crawrad-ide", None, mcp_servers)
        .expect("prompt mcp tools");

    assert!(
        resp.contains("crawrad-ide"),
        "The agent must report a connection to the MCP servers passed in the session: '{resp}'"
    );
    assert!(
        resp.contains("Connected MCP servers (2)"),
        "The number of connected MCP servers must match the list: '{resp}'"
    );

    client.shutdown();
}

#[test]
fn test_fake_agent_asks_permission_and_gets_the_right_option_id() {
    let bin = get_mock_acp_agent_binary();
    let mut client = AcpClient::spawn(&bin, &[], None, false).expect("spawn mock agent");

    client.initialize().expect("initialize");

    let rx = client
        .prompt("TEST_REQUEST_PERMISSION write_to_file", &[], None, vec![])
        .expect("prompt");

    let mut permission_found = false;
    let mut text_reply = String::new();

    while let Ok(ev) = rx.recv_timeout(Duration::from_secs(5)) {
        match ev {
            AcpStreamEvent::PermissionRequest(req) => {
                assert_eq!(req.tool_name, "write_to_file");
                assert_eq!(
                    req.target.as_deref(),
                    Some("crates/crawrad-ide/src/main.rs")
                );
                assert!(!req.options.is_empty());
                let responded =
                    req.respond(crate::PermissionDecision::Selected("allow_once".into()));
                assert!(responded);
                permission_found = true;
            }
            AcpStreamEvent::TextDelta(delta) => {
                text_reply.push_str(&delta);
            }
            AcpStreamEvent::Done { .. } => break,
            AcpStreamEvent::Error(err) => panic!("Unexpected error: {err}"),
            _ => {}
        }
    }

    assert!(
        permission_found,
        "The agent must have asked for permission on the wire"
    );
    assert!(
        text_reply.contains("allow_once"),
        "The mock agent must have received the right optionId: '{text_reply}'"
    );

    client.shutdown();
}

#[test]
fn test_two_near_simultaneous_requests_get_independent_answers() {
    let bin = get_mock_acp_agent_binary();
    let mut client = AcpClient::spawn(&bin, &[], None, false).expect("spawn mock agent");

    client.initialize().expect("initialize");

    let mut queue = crate::PermissionQueue::new();

    let rx = client
        .prompt("TEST_TWO_PERMISSIONS", &[], None, vec![])
        .expect("prompt");

    let mut text_reply = String::new();

    while let Ok(ev) = rx.recv_timeout(Duration::from_secs(5)) {
        match ev {
            AcpStreamEvent::PermissionRequest(req) => {
                let tool = req.tool_name.clone();
                queue.push(req);
                let top = queue.pop().expect("top request");
                if tool == "write_file_alpha" {
                    top.respond(crate::PermissionDecision::Selected("allow_once".into()));
                } else if tool == "write_file_beta" {
                    top.respond(crate::PermissionDecision::Selected("reject_once".into()));
                }
            }
            AcpStreamEvent::TextDelta(delta) => {
                text_reply.push_str(&delta);
            }
            AcpStreamEvent::Done { .. } => break,
            AcpStreamEvent::Error(err) => panic!("Unexpected error: {err}"),
            _ => {}
        }
    }

    assert!(
        text_reply.contains("TwoPermissionsOutcomes:"),
        "Both requests must be answered: '{text_reply}'"
    );
    assert!(
        text_reply.contains("allow_once") && text_reply.contains("reject_once"),
        "Each request must receive its own independent answer: '{text_reply}'"
    );

    client.shutdown();
}

#[test]
fn test_timeout_returns_cancelled_not_a_refusal() {
    let bin = get_mock_acp_agent_binary();
    let mut client = AcpClient::spawn(&bin, &[], None, false).expect("spawn mock agent");

    client.initialize().expect("initialize");

    let rx = client
        .prompt("TEST_REQUEST_PERMISSION custom_calc", &[], None, vec![])
        .expect("prompt");

    let mut text_reply = String::new();

    while let Ok(ev) = rx.recv_timeout(Duration::from_secs(5)) {
        match ev {
            AcpStreamEvent::PermissionRequest(req) => {
                req.respond(crate::PermissionDecision::Cancelled);
            }
            AcpStreamEvent::TextDelta(delta) => {
                text_reply.push_str(&delta);
            }
            AcpStreamEvent::Done { .. } => break,
            AcpStreamEvent::Error(err) => panic!("Unexpected error: {err}"),
            _ => {}
        }
    }

    assert!(
        text_reply.contains("cancelled"),
        "A timeout must return 'cancelled' and never 'rejected': '{text_reply}'"
    );

    client.shutdown();
}

#[test]
fn test_allow_always_holds_for_the_session_keyed_by_tool_and_target() {
    let bin = get_mock_acp_agent_binary();
    let mut client = AcpClient::spawn(&bin, &[], None, false).expect("spawn mock agent");

    client.initialize().expect("initialize");

    client.allow_always(
        "write_to_file",
        Some("crates/crawrad-ide/src/main.rs".to_string()),
    );
    assert!(client.is_permission_allowed("write_to_file", Some("crates/crawrad-ide/src/main.rs")));
    assert!(!client.is_permission_allowed("write_to_file", Some("other_file.rs")));

    let rx1 = client
        .prompt("TEST_REQUEST_PERMISSION write_to_file", &[], None, vec![])
        .expect("prompt");

    let mut saw_permission_req = false;
    let mut text1 = String::new();
    while let Ok(ev) = rx1.recv_timeout(Duration::from_secs(5)) {
        match ev {
            AcpStreamEvent::PermissionRequest(_) => {
                saw_permission_req = true;
            }
            AcpStreamEvent::TextDelta(delta) => text1.push_str(&delta),
            AcpStreamEvent::Done { .. } => break,
            _ => {}
        }
    }

    assert!(
        !saw_permission_req,
        "A request allowed for the session must go straight through"
    );
    assert!(
        text1.contains("allow_once") || text1.contains("allow_always"),
        "The agent must receive continuous approval: '{text1}'"
    );

    client.clear_session_permissions();
    assert!(!client.is_permission_allowed("write_to_file", Some("crates/crawrad-ide/src/main.rs")));

    let rx2 = client
        .prompt("TEST_REQUEST_PERMISSION write_to_file", &[], None, vec![])
        .expect("prompt 2");

    let mut saw_permission_req_after_clear = false;
    while let Ok(ev) = rx2.recv_timeout(Duration::from_secs(5)) {
        match ev {
            AcpStreamEvent::PermissionRequest(req) => {
                saw_permission_req_after_clear = true;
                req.respond(crate::PermissionDecision::Selected("allow_once".into()));
            }
            AcpStreamEvent::Done { .. } => break,
            _ => {}
        }
    }
    assert!(
        saw_permission_req_after_clear,
        "After clearing the session, permission MUST be asked again"
    );

    client.shutdown();
}

#[test]
fn test_removing_the_queue_turns_a_test_red_mutation_proof() {
    let mut queue = crate::PermissionQueue::new();
    let (tx1, rx1) = std::sync::mpsc::channel();
    let (tx2, rx2) = std::sync::mpsc::channel();

    let req1 = crate::PermissionRequest::new(
        10,
        "s1",
        "tool_a",
        None,
        "",
        vec![],
        tx1,
        Duration::from_secs(30),
    );
    let req2 = crate::PermissionRequest::new(
        20,
        "s1",
        "tool_b",
        None,
        "",
        vec![],
        tx2,
        Duration::from_secs(30),
    );

    queue.push(req1);
    queue.push(req2);

    assert_eq!(queue.len(), 2);
    let first = queue.pop().unwrap();
    assert_eq!(first.id, 10);
    first.respond(crate::PermissionDecision::Selected("opt_a".into()));

    let second = queue.pop().unwrap();
    assert_eq!(second.id, 20);
    second.respond(crate::PermissionDecision::Selected("opt_b".into()));

    assert_eq!(
        rx1.recv().unwrap(),
        crate::PermissionDecision::Selected("opt_a".into())
    );
    assert_eq!(
        rx2.recv().unwrap(),
        crate::PermissionDecision::Selected("opt_b".into())
    );
}

#[test]
fn test_auto_approve_reads_goes_straight_without_asking() {
    let bin = get_mock_acp_agent_binary();
    let mut client = AcpClient::spawn(&bin, &[], None, true).expect("spawn mock agent");
    client.initialize().expect("initialize");

    let rx = client
        .prompt("TEST_REQUEST_PERMISSION read_file", &[], None, vec![])
        .expect("prompt");

    let mut saw_permission = false;
    let mut reply = String::new();
    while let Ok(ev) = rx.recv_timeout(Duration::from_secs(5)) {
        match ev {
            AcpStreamEvent::PermissionRequest(_) => saw_permission = true,
            AcpStreamEvent::TextDelta(d) => reply.push_str(&d),
            AcpStreamEvent::Done { .. } => break,
            _ => {}
        }
    }

    assert!(
        !saw_permission,
        "A read tool must be auto-approved when auto_approve_reads=true"
    );
    assert!(
        reply.contains("allow_once") || reply.contains("FileContent"),
        "The mock agent must have received auto-approval: '{reply}'"
    );

    client.shutdown();
}

#[test]
fn test_cli_native_tools_policy_is_off_by_default() {
    let bin = get_mock_acp_agent_binary();
    let client = AcpClient::spawn(&bin, &[], None, true).expect("spawn mock agent");
    assert!(
        !client.allow_native_agent_tools(),
        "The CLI's native tools MUST start switched off by default (ADR 0029)"
    );
}

/// The race behind the intermittent `test_acp_client_streaming_and_wire_continuity`,
/// staged in the exact order that lost the text — no timing, no luck.
///
/// Turn 1's waiter sends `Done`; the caller starts turn 2 at once, which
/// installs its sink; only THEN does turn 1's waiter reach the unhook. That
/// unhook used to be unconditional and dropped every `TextDelta` of turn 2.
#[test]
fn the_waiter_of_an_old_turn_does_not_unhook_the_next_turn() {
    use crate::client::release_stream_if_current;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::mpsc::channel;

    let sink: Mutex<Option<std::sync::mpsc::Sender<AcpStreamEvent>>> = Mutex::new(None);
    let active_turn = AtomicU64::new(0);

    // Turn 1 installs its sink.
    let (tx1, _rx1) = channel();
    *sink.lock().unwrap() = Some(tx1);
    active_turn.store(1, Ordering::SeqCst);
    // Turn 2 installs its sink before turn 1's waiter unhooks.
    let (tx2, rx2) = channel();
    *sink.lock().unwrap() = Some(tx2);
    active_turn.store(2, Ordering::SeqCst);

    // The late waiter of turn 1 arrives now.
    release_stream_if_current(&sink, &active_turn, 1);

    // What the reader thread does with a `session/update` of turn 2.
    let guard = sink.lock().unwrap();
    let tx = guard
        .as_ref()
        .expect("turn 2's sink survived turn 1's waiter");
    tx.send(AcpStreamEvent::TextDelta("second answer".into()))
        .expect("send");
    drop(guard);
    match rx2.try_recv() {
        Ok(AcpStreamEvent::TextDelta(t)) => assert_eq!(t, "second answer"),
        other => panic!("turn 2 lost its text: {other:?}"),
    }

    // Turn 2's own waiter does unhook it.
    release_stream_if_current(&sink, &active_turn, 2);
    assert!(sink.lock().unwrap().is_none());
}

#[test]
fn test_acp_status_steps_and_tool_call_tracking() {
    let bin = get_mock_acp_agent_binary();
    let mut client = AcpClient::spawn(&bin, &[], None, true).expect("spawn mock agent");

    client.initialize().expect("initialize");

    let rx = client
        .prompt("TEST_STATUS_STEPS", &[], None, vec![])
        .expect("prompt status steps");

    // 1. Thought delta
    let ev1 = rx
        .recv_timeout(Duration::from_secs(6))
        .expect("thought delta event");
    assert_eq!(
        ev1,
        AcpStreamEvent::ThoughtDelta("Looking for the crate that holds the tests".into())
    );

    // 2. Tool call t1
    let ev2 = rx
        .recv_timeout(Duration::from_secs(6))
        .expect("tool call event");
    match ev2 {
        AcpStreamEvent::ToolCall { id, name, input } => {
            assert_eq!(id, "t1");
            assert_eq!(name, "Terminal");
            assert_eq!(input, serde_json::json!({}));
        }
        other => panic!("expected ToolCall, got: {other:?}"),
    }

    // 3. Tool call update t1 with title "cargo test -p app" and status in_progress -> Running
    let ev3 = rx
        .recv_timeout(Duration::from_secs(6))
        .expect("tool call update in progress");
    match ev3 {
        AcpStreamEvent::ToolCallUpdate {
            id,
            name,
            status,
            summary: _,
        } => {
            assert_eq!(id, "t1");
            assert_eq!(name, "cargo test -p app");
            assert_eq!(status, crate::markdown::ToolCallStatus::Running);
        }
        other => panic!("expected ToolCallUpdate running, got: {other:?}"),
    }

    // 4. Tool call update t1 completed -> Success (inherits remembered title)
    let ev4 = rx
        .recv_timeout(Duration::from_secs(6))
        .expect("tool call update completed");
    match ev4 {
        AcpStreamEvent::ToolCallUpdate {
            id,
            name,
            status,
            summary: _,
        } => {
            assert_eq!(id, "t1");
            assert_eq!(name, "cargo test -p app");
            assert_eq!(status, crate::markdown::ToolCallStatus::Success);
        }
        other => panic!("expected ToolCallUpdate success, got: {other:?}"),
    }

    // 5. Message chunk "All 42 tests passed in app."
    let ev5 = rx
        .recv_timeout(Duration::from_secs(6))
        .expect("text delta event");
    assert_eq!(
        ev5,
        AcpStreamEvent::TextDelta("All 42 tests passed in app.".into())
    );

    // 6. Turn completed
    let ev6 = rx.recv_timeout(Duration::from_secs(6)).expect("done event");
    assert_eq!(
        ev6,
        AcpStreamEvent::Done {
            stop_reason: "end_turn".into()
        }
    );

    client.shutdown();
}

#[test]
fn test_acp_client_fs_read_slice_and_write() {
    let bin = get_mock_acp_agent_binary();
    let mut client = AcpClient::spawn(&bin, &[], None, true).expect("spawn mock agent");

    client.initialize().expect("initialize");

    let dirty_path = "crates/crawrad-ide/src/dirty_slice.rs";
    let dirty_content = "line 1\nline 2\nline 3\nline 4\nline 5\n";
    client.set_live_buffer(dirty_path, dirty_content);

    // 1. Read slice: line 2, limit 2 -> "line 2\nline 3\n"
    let response = client
        .prompt_sync(&format!("TEST_READ_FILE {dirty_path} 2 2"), None, vec![])
        .expect("prompt_sync slice");
    assert!(
        response.contains("FileContent: line 2\nline 3"),
        "fs/read_text_file with line=2 limit=2 must slice lines: '{response}'"
    );

    // 2. Write file
    let write_resp = client
        .prompt_sync(
            &format!("TEST_WRITE_FILE {dirty_path} new_live_content"),
            None,
            vec![],
        )
        .expect("prompt_sync write");
    assert!(
        write_resp.contains("WriteFile: OK"),
        "fs/write_text_file must succeed and return null result: '{write_resp}'"
    );

    // 3. Read back written content
    let read_back = client
        .prompt_sync(&format!("TEST_READ_FILE {dirty_path}"), None, vec![])
        .expect("prompt_sync read back");
    assert!(
        read_back.contains("FileContent: new_live_content"),
        "Buffer must contain newly written content: '{read_back}'"
    );

    client.shutdown();
}

#[test]
fn test_ide_mcp_server_helper_shape() {
    // Absolute on the platform running the test: `C:/…` is relative on Linux.
    let absolute = if cfg!(windows) {
        "C:/tools/crawrad-mcp-bridge.exe"
    } else {
        "/tools/crawrad-mcp-bridge"
    };
    let val = crate::client::ide_mcp_server(std::path::Path::new(absolute), "test-instance");

    assert_eq!(
        val.get("name").and_then(|v| v.as_str()),
        Some("crawrad-ide")
    );
    assert_eq!(val.get("command").and_then(|v| v.as_str()), Some(absolute));
    assert_eq!(
        val.get("args"),
        Some(&serde_json::json!(["--instance", "test-instance"]))
    );
    assert_eq!(val.get("env"), Some(&serde_json::json!([])));
}
