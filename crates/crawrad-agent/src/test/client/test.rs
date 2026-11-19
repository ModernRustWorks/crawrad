//! ACP client tests: why an agent stopped.

use super::*;

#[test]
fn exit_message_carries_what_the_agent_said() {
    assert_eq!(
        agent_exit_message(Some(1), "'claude-agent-acp' is not recognized\n"),
        "Agent process terminated unexpectedly with exit code 1: 'claude-agent-acp' is not recognized"
    );
    assert_eq!(
        agent_exit_message(None, "  "),
        "Agent process closed communication stream"
    );
}

#[test]
fn stderr_tail_keeps_the_end_and_closes() {
    let long = format!("{}\nlast line\n", "x".repeat(STDERR_TAIL_BYTES * 2));
    let tail = keep_stderr_tail(std::io::Cursor::new(long.into_bytes()));
    for _ in 0..100 {
        if tail.lock().map(|kept| kept.1).unwrap_or(false) {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    let kept = tail.lock().expect("tail lock");
    assert!(kept.1, "the stream closed");
    assert!(kept.0.len() <= STDERR_TAIL_BYTES);
    assert!(kept.0.ends_with("last line\n"));
}
