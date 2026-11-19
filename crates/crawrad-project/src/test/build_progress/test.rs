use super::*;
use crawrad_os::process::Pipe;
use std::time::Duration;

#[test]
fn new_initializes_with_command_line_and_defaults() {
    let bp = BuildProgress::new("Building", "meu_app", "cargo build -p meu_app");
    assert_eq!(bp.verb(), "Building");
    assert_eq!(bp.package(), "meu_app");
    assert_eq!(bp.current(), None);
    assert_eq!(bp.done(), 0);
    assert_eq!(bp.total(), None);
    assert_eq!(bp.log(), &["> cargo build -p meu_app"]);
    assert_eq!(bp.log_text(), "> cargo build -p meu_app\n");
    assert!(bp.is_running());
    assert_eq!(bp.state(), ProgressState::Running);
    assert_eq!(bp.fraction(), None);
    assert_eq!(bp.counter(), None);
    assert_eq!(bp.headline(), "Building meu_app…");
}

#[test]
fn feed_stderr_progress_bar_parses_done_and_total_without_logging() {
    let mut bp = BuildProgress::new("Building", "meu_app", "cargo build");
    bp.feed(
        Pipe::Stderr,
        "    Building [=====>    ] 38/214: serde_json, itoa",
    );
    assert_eq!(bp.done(), 38);
    assert_eq!(bp.total(), Some(214));
    assert_eq!(bp.counter(), Some("38 / 214 crates".to_string()));
    assert_eq!(bp.fraction(), Some(38.0 / 214.0));
    // Progress bar must NOT be logged
    assert_eq!(bp.log().len(), 1);

    // Malformed progress bar is ignored and not logged
    bp.feed(Pipe::Stderr, "    Building [=== malformed");
    assert_eq!(bp.done(), 38);
    assert_eq!(bp.total(), Some(214));
    assert_eq!(bp.log().len(), 1);
}

#[test]
fn feed_stderr_compiling_checking_and_fresh() {
    let mut bp = BuildProgress::new("Building", "meu_app", "cargo build");

    // Compiling with path
    bp.feed(Pipe::Stderr, "   Compiling serde v1.0.197 (/path/to/serde)");
    assert_eq!(bp.current(), Some("serde v1.0.197"));
    assert_eq!(bp.headline(), "Building meu_app — Compiling serde v1.0.197");
    assert_eq!(bp.log()[1], "   Compiling serde v1.0.197 (/path/to/serde)");

    // Checking without path
    bp.feed(Pipe::Stderr, "   Checking tokio v1.36.0");
    assert_eq!(bp.current(), Some("tokio v1.36.0"));
    assert_eq!(bp.headline(), "Building meu_app — Checking tokio v1.36.0");
    assert_eq!(bp.log()[2], "   Checking tokio v1.36.0");

    // Fresh updates current but is NOT logged
    let log_len = bp.log().len();
    bp.feed(Pipe::Stderr, "   Fresh syn v2.0.50");
    assert_eq!(bp.current(), Some("syn v2.0.50"));
    assert_eq!(bp.log().len(), log_len);
}

#[test]
fn feed_other_stderr_lines_are_logged() {
    let mut bp = BuildProgress::new("Building", "meu_app", "cargo build");
    bp.feed(Pipe::Stderr, "warning: unused variable: `x`   \r\n");
    assert_eq!(bp.log()[1], "warning: unused variable: `x`");
}

#[test]
fn feed_stdout_json_and_non_json() {
    let mut bp = BuildProgress::new("Building", "meu_app", "cargo build");

    // JSON compiler-artifact increments done when total is None
    bp.feed(
        Pipe::Stdout,
        r#"{"reason":"compiler-artifact","package_id":"foo 0.1.0"}"#,
    );
    assert_eq!(bp.done(), 1);
    assert_eq!(bp.log().len(), 1); // not logged

    bp.feed(
        Pipe::Stdout,
        r#"{"reason":"compiler-artifact","package_id":"bar 0.1.0"}"#,
    );
    assert_eq!(bp.done(), 2);
    assert_eq!(bp.log().len(), 1); // not logged

    // Other stdout JSON is ignored and not logged
    bp.feed(
        Pipe::Stdout,
        r#"{"reason":"compiler-message","message":{"rendered":"err"}}"#,
    );
    assert_eq!(bp.done(), 2);
    assert_eq!(bp.log().len(), 1);

    // When total is known, compiler-artifact does NOT increment done
    bp.feed(Pipe::Stderr, "    Building [==>       ] 10/50: baz");
    assert_eq!(bp.done(), 10);
    assert_eq!(bp.total(), Some(50));
    bp.feed(
        Pipe::Stdout,
        r#"{"reason":"compiler-artifact","package_id":"qux 0.1.0"}"#,
    );
    assert_eq!(bp.done(), 10); // remains 10

    // Non-JSON stdout is logged
    bp.feed(Pipe::Stdout, "custom build script output");
    assert_eq!(bp.log()[1], "custom build script output");
}

#[test]
fn headlines_for_finished_states() {
    let mut bp = BuildProgress::new("Building", "meu_app", "cargo build");
    bp.finish(Finish {
        success: true,
        cancelled: false,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(5),
    });
    assert_eq!(bp.headline(), "Built meu_app");
    assert_eq!(bp.state(), ProgressState::Succeeded);
    assert_eq!(bp.fraction(), Some(1.0));

    let mut bp_check = BuildProgress::new("Checking", "meu_app", "cargo check");
    bp_check.finish(Finish {
        success: true,
        cancelled: false,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(3),
    });
    assert_eq!(bp_check.headline(), "Checked meu_app");

    let mut bp_clean = BuildProgress::new("Cleaning", "meu_app", "cargo clean");
    bp_clean.finish(Finish {
        success: true,
        cancelled: false,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(1),
    });
    assert_eq!(bp_clean.headline(), "Cleaned meu_app");

    let mut bp_run = BuildProgress::new("Running", "meu_app", "cargo run");
    bp_run.finish(Finish {
        success: true,
        cancelled: false,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(2),
    });
    assert_eq!(bp_run.headline(), "Ran meu_app");

    // Failure
    let mut bp_fail = BuildProgress::new("Building", "meu_app", "cargo build");
    bp_fail.finish(Finish {
        success: false,
        cancelled: false,
        errors: 2,
        warnings: 1,
        elapsed: Duration::from_secs(4),
    });
    assert_eq!(bp_fail.headline(), "Building meu_app failed");
    assert_eq!(bp_fail.state(), ProgressState::Failed);

    // Cancelled
    let mut bp_cancel = BuildProgress::new("Building", "meu_app", "cargo build");
    bp_cancel.finish(Finish {
        success: false,
        cancelled: true,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(2),
    });
    assert_eq!(bp_cancel.headline(), "Building meu_app cancelled");
    assert_eq!(bp_cancel.state(), ProgressState::Cancelled);
}

#[test]
fn counter_singular_and_plural_rules() {
    let mut bp = BuildProgress::new("Building", "meu_app", "cargo build");
    bp.feed(Pipe::Stderr, "    Building [==>       ] 5/20: test");

    // Finished with 0 errors, 0 warnings
    let mut bp0 = bp.clone();
    bp0.finish(Finish {
        success: true,
        cancelled: false,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(1),
    });
    assert_eq!(
        bp0.counter(),
        Some("20 crates · 0 errors · 0 warnings".to_string())
    );

    // Finished with 1 error, 1 warning
    let mut bp1 = bp.clone();
    bp1.finish(Finish {
        success: false,
        cancelled: false,
        errors: 1,
        warnings: 1,
        elapsed: Duration::from_secs(1),
    });
    assert_eq!(
        bp1.counter(),
        Some("20 crates · 1 error · 1 warning".to_string())
    );

    // Finished with 2 errors, 3 warnings
    let mut bp2 = bp.clone();
    bp2.finish(Finish {
        success: false,
        cancelled: false,
        errors: 2,
        warnings: 3,
        elapsed: Duration::from_secs(1),
    });
    assert_eq!(
        bp2.counter(),
        Some("20 crates · 2 errors · 3 warnings".to_string())
    );

    // Finished when total was unknown falls back to done
    let mut bp_no_total = BuildProgress::new("Building", "meu_app", "cargo build");
    bp_no_total.feed(
        Pipe::Stdout,
        r#"{"reason":"compiler-artifact","package_id":"p1"}"#,
    );
    bp_no_total.feed(
        Pipe::Stdout,
        r#"{"reason":"compiler-artifact","package_id":"p2"}"#,
    );
    bp_no_total.finish(Finish {
        success: true,
        cancelled: false,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(2),
    });
    assert_eq!(
        bp_no_total.counter(),
        Some("2 crates · 0 errors · 0 warnings".to_string())
    );
}

#[test]
fn clock_edge_cases() {
    assert_eq!(BuildProgress::clock(Duration::from_secs(7)), "00:07");
    assert_eq!(BuildProgress::clock(Duration::from_secs(59)), "00:59");
    assert_eq!(BuildProgress::clock(Duration::from_secs(60)), "01:00");
    assert_eq!(BuildProgress::clock(Duration::from_secs(65)), "01:05");
    assert_eq!(BuildProgress::clock(Duration::from_secs(3599)), "59:59");
    assert_eq!(BuildProgress::clock(Duration::from_secs(3600)), "1:00:00");
    assert_eq!(BuildProgress::clock(Duration::from_secs(3723)), "1:02:03");
    assert_eq!(BuildProgress::clock(Duration::from_secs(7200)), "2:00:00");
}

#[test]
fn status_line_formats() {
    let mut bp = BuildProgress::new("Building", "meu_app", "cargo build");
    // Without counter
    assert_eq!(
        bp.status_line(Duration::from_secs(12)),
        "Building meu_app · 00:12"
    );

    // With counter
    bp.feed(
        Pipe::Stderr,
        "    Building [=====>    ] 38/214: serde_json, itoa",
    );
    assert_eq!(
        bp.status_line(Duration::from_secs(12)),
        "Building meu_app · 38 / 214 · 00:12"
    );

    // Finished
    bp.finish(Finish {
        success: true,
        cancelled: false,
        errors: 0,
        warnings: 0,
        elapsed: Duration::from_secs(45),
    });
    assert_eq!(
        bp.status_line(Duration::from_secs(100)),
        "Built meu_app · 00:45"
    );
}

#[test]
fn realistic_transcript_and_verdict() {
    let mut bp = BuildProgress::new("Building", "meu_app", "cargo build -p meu_app");

    let transcript = [
        (Pipe::Stderr, "   Compiling proc-macro2 v1.0.78"),
        (Pipe::Stderr, "   Compiling unicode-ident v1.0.12"),
        (
            Pipe::Stderr,
            "    Building [=>                  ] 2/20: quote",
        ),
        (
            Pipe::Stdout,
            r#"{"reason":"compiler-artifact","package_id":"proc-macro2 1.0.78"}"#,
        ),
        (Pipe::Stderr, "   Compiling quote v1.0.35"),
        (Pipe::Stderr, "   Compiling syn v2.0.48"),
        (
            Pipe::Stderr,
            "    Building [====>               ] 5/20: serde",
        ),
        (Pipe::Stderr, "   Fresh itoa v1.0.10"),
        (Pipe::Stderr, "   Compiling serde v1.0.196"),
        (
            Pipe::Stderr,
            "warning: variable does not need to be mutable",
        ),
        (
            Pipe::Stderr,
            "    Building [==========>         ] 11/20: meu_app",
        ),
        (
            Pipe::Stdout,
            r#"{"reason":"compiler-message","message":{"rendered":"warning"}}"#,
        ),
        (
            Pipe::Stderr,
            "   Compiling meu_app v0.1.0 (/project/meu_app)",
        ),
        (
            Pipe::Stderr,
            "    Building [===================>] 20/20: meu_app",
        ),
        (
            Pipe::Stderr,
            "    Finished `dev` profile [unoptimized + debuginfo] in 3.42s",
        ),
    ];

    for (pipe, line) in transcript {
        bp.feed(pipe, line);
    }

    assert_eq!(bp.done(), 20);
    assert_eq!(bp.total(), Some(20));
    assert_eq!(bp.fraction(), Some(1.0));
    assert_eq!(bp.current(), Some("meu_app v0.1.0"));
    assert_eq!(bp.headline(), "Building meu_app — Compiling meu_app v0.1.0");

    bp.finish(Finish {
        success: true,
        cancelled: false,
        errors: 0,
        warnings: 1,
        elapsed: Duration::from_millis(3420),
    });

    assert_eq!(bp.headline(), "Built meu_app");
    assert_eq!(
        bp.counter(),
        Some("20 crates · 0 errors · 1 warning".to_string())
    );
    assert_eq!(bp.fraction(), Some(1.0));
    assert_eq!(bp.state(), ProgressState::Succeeded);

    // Verify log text
    let log_text = bp.log_text();
    assert!(log_text.starts_with("> cargo build -p meu_app\n"));
    assert!(log_text.contains("Compiling proc-macro2 v1.0.78\n"));
    assert!(log_text.contains("warning: variable does not need to be mutable\n"));
    assert!(log_text.contains("Finished `dev` profile"));
    // Verify progress bar and Fresh are omitted from the log
    assert!(!log_text.contains("Building ["));
    assert!(!log_text.contains("Fresh itoa"));
    assert!(!log_text.contains("compiler-artifact"));
}

#[test]
fn test_encode_and_decode_line_round_trip() {
    let out_line = "compiling crate v0.1.0";
    let encoded_out = encode_line(Pipe::Stdout, out_line);
    assert_eq!(encoded_out, "o compiling crate v0.1.0");
    assert_eq!(decode_line(&encoded_out), Some((Pipe::Stdout, out_line)));

    let err_line = "error[E0425]: cannot find value";
    let encoded_err = encode_line(Pipe::Stderr, err_line);
    assert_eq!(encoded_err, "e error[E0425]: cannot find value");
    assert_eq!(decode_line(&encoded_err), Some((Pipe::Stderr, err_line)));

    // Unknown or invalid prefixes
    assert_eq!(decode_line("x unknown prefix"), None);
    assert_eq!(decode_line("o"), None);
    assert_eq!(decode_line("e"), None);
    assert_eq!(decode_line(""), None);
    assert_eq!(decode_line("plain text"), None);
}

#[test]
fn a_run_says_building_while_cargo_works() {
    let bp = BuildProgress::new("Running", "meu_app", "cargo build -p meu_app");
    assert_eq!(
        bp.status_line(Duration::from_secs(3)),
        "Building meu_app · 00:03"
    );
}
