use super::*;
use std::time::{Duration, Instant};

fn wait_end(run: &mut AppRun) -> AppEnd {
    let until = Instant::now() + Duration::from_secs(20);
    loop {
        if let Some(end) = run.pump().pop() {
            return end;
        }
        assert!(Instant::now() < until, "the watched app never ended");
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn shell(script: &str) -> Command {
    if cfg!(windows) {
        let mut c = Command::new("cmd");
        c.args(["/C", script]);
        c
    } else {
        let mut c = Command::new("sh");
        c.args(["-c", script]);
        c
    }
}

#[test]
fn a_panic_on_stderr_comes_back_when_the_app_ends() {
    let mut run = AppRun::default();
    let root = std::env::temp_dir();
    run.start(
        shell("echo thread 'main' panicked at src/main.rs:3:5: 1>&2 && echo boom 1>&2"),
        "calc",
        &root,
    )
    .unwrap();
    let end = wait_end(&mut run);
    assert_eq!(end.package, "calc");
    assert_eq!(end.panics.len(), 1);
    assert_eq!(end.panics[0].message, "boom");
    assert_eq!(end.panics[0].spot.line, 3);
}

#[test]
fn a_clean_end_has_no_panic_and_a_bad_program_fails_at_once() {
    let mut run = AppRun::default();
    run.start(shell("echo fine"), "calc", &std::env::temp_dir())
        .unwrap();
    assert!(wait_end(&mut run).panics.is_empty());
    let err = run
        .start(
            Command::new("crawrad-no-such-program-926"),
            "calc",
            &std::env::temp_dir(),
        )
        .unwrap_err();
    assert!(err.contains("could not launch crawrad-no-such-program-926"));
}

#[test]
fn a_panic_is_told_while_the_dead_app_is_still_held() {
    let mut run = AppRun::default();
    let script = if cfg!(windows) {
        "echo thread 'main' panicked at src/main.rs:3:5: 1>&2 && echo boom 1>&2 && ping -n 30 127.0.0.1 >nul"
    } else {
        "echo \"thread 'main' panicked at src/main.rs:3:5:\" 1>&2; echo boom 1>&2; sleep 30"
    };
    let started = Instant::now();
    run.start(shell(script), "calc", &std::env::temp_dir())
        .unwrap();
    let end = wait_end(&mut run);
    assert_eq!(end.panics[0].message, "boom");
    assert!(
        started.elapsed() < Duration::from_secs(10),
        "it waited for the process"
    );
}
