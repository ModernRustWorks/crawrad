//! Launching a process is testable without a screen, and this file is the
//! proof: it starts real children, on all three platforms, and measures what
//! came back.
//!
//! The command used is `cargo` itself. It is not a convenient coincidence — a
//! test of this crate only ever runs under `cargo test`, so `cargo` is on the
//! `PATH` by definition, on Windows, Linux and macOS alike. Writing a shell
//! script fixture, or reaching for `echo` (a built-in on Windows, a binary
//! elsewhere), would trade that guarantee for a platform `cfg`.

use super::*;

/// A directory that exists, for the calls that do not care where they run.
fn anywhere() -> std::path::PathBuf {
    std::env::temp_dir()
}

/// What cargo will actually receive, read back from the `Command` itself.
///
/// Read from the `Command` and not from a getter on [`CargoCall`]: an accessor
/// that only tests call is dead code in the product, and the thing worth
/// measuring is what leaves the house, not what was stored on the way.
fn arguments_of(call: &CargoCall) -> Vec<String> {
    call.command()
        .get_args()
        .map(|a| a.to_string_lossy().into_owned())
        .collect()
}

/// The arguments of a build are written in exactly one place.
///
/// This is the shape of three hand-written `Command::new("cargo")`
/// grew beside the shared builder and drifted apart. The constructor is now the
/// single author of this list, and this test is what notices if a second author
/// appears.
#[test]
fn a_build_spells_its_arguments_in_one_place() {
    let call = CargoCall::build("meu_app", anywhere());
    assert_eq!(
        arguments_of(&call),
        ["build", "-p", "meu_app", "--message-format=json"],
        "the build lost its shape. `--message-format=json` is what turns one \
         error line in the status bar into a clickable list; `-p` at the \
         workspace root is what keeps a single `target/`"
    );
    assert_eq!(call.command().get_current_dir(), Some(anywhere().as_path()));
}

/// `run` never carries `--message-format=json`: nobody is reading that output.
#[test]
fn a_run_is_not_a_build_wearing_a_hat() {
    let args = arguments_of(&CargoCall::run("meu_app", anywhere()));
    assert_eq!(args, ["run", "-p", "meu_app"]);
    assert!(
        !args.iter().any(|a| a.contains("message-format")),
        "a detached `cargo run` has no reader for its output, and json on the \
         user's terminal would be noise: {args:?}",
    );
}

/// The three environment lines, each of which has already cost a defect.
#[test]
fn the_house_calls_cargo_without_colour_and_without_our_own_build_vars() {
    let cmd = CargoCall::build("meu_app", anywhere()).command();
    let env: Vec<_> = cmd.get_envs().collect();

    assert!(
        env.contains(&(
            std::ffi::OsStr::new("CARGO_TERM_COLOR"),
            Some(std::ffi::OsStr::new("never"))
        )),
        "cargo's output here is READ, and without this it arrives painted for \
         anyone who exported CARGO_TERM_COLOR=always — CI is one of those: \
         {env:?}"
    );
    for ours in ["CARGO_TARGET_DIR", "RUSTFLAGS"] {
        assert!(
            env.contains(&(std::ffi::OsStr::new(ours), None)),
            "`{ours}` of the IDE would leak into the user's build"
        );
    }
}

/// It runs where it was told, and it runs `cargo`.
#[test]
fn the_call_carries_its_own_working_directory() {
    let dir = anywhere().join("crawrad-launcher-cwd");
    let cmd = CargoCall::metadata(&dir).command();
    assert_eq!(cmd.get_program(), std::ffi::OsStr::new("cargo"));
    assert_eq!(cmd.get_current_dir(), Some(dir.as_path()));
}

/// `cargo metadata` is asked for by two callers, and this is the one place
/// the question is written.
///
/// `--no-deps` is load-bearing: the question is always about the user's own
/// members, and resolving the whole dependency graph turns an answer the
/// status bar waits for into one it waits a long time for.
#[test]
fn metadata_is_asked_the_same_way_by_everyone_who_asks() {
    assert_eq!(
        arguments_of(&CargoCall::metadata(anywhere())),
        ["metadata", "--format-version", "1", "--no-deps"],
        "this list used to be hand-written in workspace.rs and again in \
         package_manager.rs, which is the shape of issue #67"
    );
}

// ---------------------------------------------------------------------------
// Real children. No screen, no framework, no fixture on disk.
// ---------------------------------------------------------------------------

/// A child that succeeds: standard output arrives, and it arrives whole.
#[test]
fn a_child_that_succeeds_hands_back_its_stdout() {
    let out = wait(&mut CargoCall::new(["--version"], anywhere()).command())
        .expect("cargo is on the PATH of anything running `cargo test`");

    assert!(out.success, "`cargo --version` failed: {out:?}");
    assert!(
        out.stdout.contains("cargo"),
        "stdout was dropped on the way back: {out:?}"
    );
}

/// **The regression of the earlier defect, measured against a real process.**
///
/// A child that fails writes on `stderr`, and every byte of it has to survive
/// the trip. The old code kept one line and binned the rest, which is why the
/// Build panel could say `nothing to show` about a cargo that had just
/// explained itself in four lines.
#[test]
fn a_child_that_fails_hands_back_its_whole_stderr() {
    let out = wait(&mut CargoCall::new(["--definitely-not-a-flag"], anywhere()).command())
        .expect("the process started; it is its exit status that is bad");

    assert!(!out.success, "cargo accepted a nonsense flag: {out:?}");
    assert!(
        !out.stderr.trim().is_empty(),
        "cargo explained the refusal and the explanation was thrown away — \
         this is issue #67: {out:?}"
    );
    // Whole, not one line. The count is the cheapest proof that nothing was
    // filtered on the way.
    assert!(
        out.stderr.lines().count() > 1,
        "only one line of stderr survived, which is exactly the old defect: \
         {:?}",
        out.stderr
    );
    assert!(
        out.report().is_some_and(|r| r.contains("error")),
        "the status bar would have had nothing to say: {out:?}"
    );
}

/// Letting go of a child is not the same as ignoring it: the handover still
/// has to say whether it started.
///
/// This is the path of `F5` after a green build, and the sentence on the status
/// bar depends on it — `built — meu_app running` or `built, but could not
/// launch`. `cargo --version` stands in for the user's application: it starts,
/// prints, and dies on its own, so the test leaves nothing behind.
#[test]
fn a_detached_child_is_let_go_but_still_reports_whether_it_started() {
    let mut cmd = CargoCall::new(["--version"], anywhere()).command();
    // Silenced here and not inside `detach`: the user's application is meant to
    // print on the terminal it was given. It is this test that has no business
    // spraying a cargo version banner across the suite's output.
    cmd.stdout(std::process::Stdio::null());
    detach(&mut cmd).expect("cargo starts, and `detach` has to say so");
}

/// And a detached child that never starts is an `Err` with a name in it.
///
/// Measured before this existed: `spawn().is_ok()` swallowed the reason, and
/// the editor launcher fell through to the next candidate with nothing to say
/// when both failed.
#[test]
fn a_detached_child_that_never_started_says_what_it_was() {
    let error = detach(&mut no_console("crawrad-no-such-program-ever-9c42"))
        .expect_err("that program does not exist anywhere");
    assert!(
        error.contains("crawrad-no-such-program-ever-9c42"),
        "the message does not name what failed to launch: {error}"
    );
}

/// The child that outlives the call.
///
/// `#[ignore]` because it is not a test: it is the *subject* of the one below,
/// started as a child of this very binary. Three seconds is long enough that a
/// [`detach`] which secretly waited could not hide inside timing noise, and
/// short enough that the child lets go of this binary before the next `cargo
/// test` needs to relink it -- measured, and `link.exe` says 1104 when it does
/// not.
#[test]
#[ignore = "started as a child by `detach_hands_back_before_the_child_is_done`"]
fn a_child_that_takes_its_time() {
    std::thread::sleep(std::time::Duration::from_secs(3));
}

/// **`detach` must come back while the child is still running.**
///
/// This is the law the status bar broke once and the one nothing was measuring.
/// With `cargo run` on the build thread, the call only returned when the user's
/// application *closed* — so the IDE said `building…` with the app already on
/// screen, for as long as the person kept it open. Swapping `spawn` for
/// `output` in here would bring that back, compile clean, and pass every other
/// test in this file: they all use children that finish in milliseconds.
///
/// The child is this same test binary, asked for the ignored test above. It
/// needs no fixture on disk and no platform `cfg`.
#[test]
fn detach_hands_back_before_the_child_is_done() {
    let exe = std::env::current_exe().expect("the test binary is running");
    let mut cmd = no_console(&exe);
    cmd.args([
        "launcher::tests::a_child_that_takes_its_time",
        "--exact",
        "--ignored",
    ])
    .stdout(std::process::Stdio::null())
    .stderr(std::process::Stdio::null());

    let clock = std::time::Instant::now();
    detach(&mut cmd).expect("the binary that is running can be run again");
    let elapsed = clock.elapsed();

    assert!(
        elapsed < std::time::Duration::from_millis(1500),
        "`detach` took {elapsed:?} for a child that sleeps three seconds, so it \
         waited. That is the `building…` that stayed on screen while the app \
         was already in front of the user"
    );
}

/// A program that does not exist is an `Err`, not a failed [`Outcome`].
///
/// The distinction is not pedantry: "cargo is not installed" and "your code
/// does not compile" need different sentences on screen, and collapsing them
/// would make the first one look like the second.
#[test]
fn a_program_that_never_started_is_an_error_and_not_an_outcome() {
    let mut cmd = no_console("crawrad-no-such-program-ever-38f1");
    let error = wait(&mut cmd).expect_err("that program does not exist anywhere");
    assert!(
        error.contains("crawrad-no-such-program-ever-38f1"),
        "the message does not name what failed to start: {error}"
    );
}

// ---------------------------------------------------------------------------
// Streaming execution (`stream`)
// ---------------------------------------------------------------------------

#[cfg(windows)]
fn portable_shell(script: &str) -> Command {
    let mut cmd = no_console("cmd");
    cmd.args(["/C", script]);
    cmd
}

#[cfg(not(windows))]
fn portable_shell(script: &str) -> Command {
    let mut cmd = no_console("sh");
    cmd.args(["-c", script]);
    cmd
}

/// The test binary itself, running one of the `#[ignore]`d children below: the
/// exact bytes on each pipe, without a shell's quoting in between. libtest's own
/// chatter goes to stdout, so stdout checks use `contains`.
fn this_test_binary_as_child(test: &str) -> Command {
    let exe = std::env::current_exe().expect("the test binary is running");
    let mut cmd = no_console(&exe);
    cmd.args([test, "--ignored"]);
    cmd
}

#[test]
#[ignore = "started as a child by `stream_lines_arrive_with_pipe_and_are_kept_in_outcome`"]
fn a_child_that_prints_on_both_pipes() {
    use std::io::Write;
    let _ = std::io::stdout().write_all(b"out_line\n");
    let _ = std::io::stderr().write_all(b"err_line\n");
}

#[test]
#[ignore = "started as a child by `stream_progress_bar_reaches_online_but_is_omitted_from_outcome_stderr`"]
fn a_child_that_draws_a_progress_bar() {
    use std::io::Write;
    let _ = std::io::stderr().write_all(b"    Building [==> ] 3/10: x\r");
}

/// Lines arrive through `on_line` with the right `Pipe`, and `Outcome.stdout` / `stderr` hold them.
#[test]
fn stream_lines_arrive_with_pipe_and_are_kept_in_outcome() {
    let mut cmd = this_test_binary_as_child("a_child_that_prints_on_both_pipes");

    let mut lines = Vec::new();
    let outcome = stream(
        &mut cmd,
        |pipe, line| lines.push((pipe, line.to_string())),
        || false,
    )
    .expect("streaming run succeeds");

    assert!(lines.contains(&(Pipe::Stdout, "out_line".to_string())));
    assert!(lines.contains(&(Pipe::Stderr, "err_line".to_string())));
    assert!(outcome.stdout.contains("out_line\n"));
    assert!(outcome.stderr.contains("err_line\n"));
    assert!(outcome.success);
    assert!(!outcome.cancelled);
}

#[test]
#[ignore = "started as a child by `stream_splits_stderr_on_cr_and_lf`"]
fn a_child_that_prints_cr_on_stderr() {
    use std::io::Write;
    let _ = std::io::stderr().write_all(b"a\rb\n");
}

/// Stderr is split on BOTH `\r` and `\n`.
#[test]
fn stream_splits_stderr_on_cr_and_lf() {
    let mut cmd = this_test_binary_as_child("a_child_that_prints_cr_on_stderr");

    let mut lines = Vec::new();
    let outcome = stream(
        &mut cmd,
        |pipe, line| {
            if pipe == Pipe::Stderr {
                lines.push(line.to_string());
            }
        },
        || false,
    )
    .expect("streaming run succeeds");

    assert_eq!(lines, ["a", "b"]);
    assert_eq!(outcome.stderr, "a\nb\n");
}

/// A `Building [==> ] 3/10: x` fragment reaches `on_line` but is NOT in `Outcome.stderr`.
#[test]
fn stream_progress_bar_reaches_online_but_is_omitted_from_outcome_stderr() {
    let mut cmd = this_test_binary_as_child("a_child_that_draws_a_progress_bar");

    let mut seen = Vec::new();
    let outcome = stream(
        &mut cmd,
        |pipe, line| {
            if pipe == Pipe::Stderr {
                seen.push(line.to_string());
            }
        },
        || false,
    )
    .expect("streaming run succeeds");

    assert_eq!(seen, ["    Building [==> ] 3/10: x"]);
    assert!(!outcome.stderr.contains("Building ["));
    assert_eq!(outcome.stderr, "");
}

/// Cancel kills the child and returns `cancelled: true`, `success: false` quickly.
#[test]
fn stream_cancels_and_kills_child_process() {
    #[cfg(windows)]
    let mut cmd = portable_shell("ping -n 11 127.0.0.1 >NUL");
    #[cfg(not(windows))]
    let mut cmd = portable_shell("sleep 10");

    let start = std::time::Instant::now();
    let polls = std::cell::Cell::new(0);
    let outcome = stream(
        &mut cmd,
        |_pipe, _line| {},
        || {
            polls.set(polls.get() + 1);
            polls.get() > 1
        },
    )
    .expect("stream cancellation yields an Outcome");

    let elapsed = start.elapsed();
    assert!(
        elapsed < std::time::Duration::from_secs(5),
        "cancellation took {elapsed:?}, expected well under 5s"
    );
    assert!(outcome.cancelled);
    assert!(!outcome.success);
}

/// A program that does not exist returns an `Err` starting with "could not run".
#[test]
fn stream_program_that_does_not_exist_returns_err() {
    let mut cmd = no_console("crawrad-no-such-program-ever-48b2");
    let error = stream(&mut cmd, |_pipe, _line| {}, || false)
        .expect_err("that program does not exist anywhere");
    assert!(
        error.starts_with("could not run"),
        "expected error starting with 'could not run', got: {error}"
    );
}

/// `progress_command()` carries the two env vars and still carries `CARGO_TERM_COLOR=never`.
#[test]
fn progress_command_carries_term_progress_env_and_color_never() {
    let call = CargoCall::build("meu_app", anywhere());
    let cmd = call.progress_command();
    let env: Vec<_> = cmd.get_envs().collect();

    assert!(
        env.contains(&(
            std::ffi::OsStr::new("CARGO_TERM_PROGRESS_WHEN"),
            Some(std::ffi::OsStr::new("always"))
        )),
        "missing CARGO_TERM_PROGRESS_WHEN=always: {env:?}"
    );
    assert!(
        env.contains(&(
            std::ffi::OsStr::new("CARGO_TERM_PROGRESS_WIDTH"),
            Some(std::ffi::OsStr::new("200"))
        )),
        "missing CARGO_TERM_PROGRESS_WIDTH=200: {env:?}"
    );
    assert!(
        env.contains(&(
            std::ffi::OsStr::new("CARGO_TERM_COLOR"),
            Some(std::ffi::OsStr::new("never"))
        )),
        "CARGO_TERM_COLOR=never was lost: {env:?}"
    );
}

// ---------------------------------------------------------------------------
// The report. Pure, because the cases that matter are the ones cargo is least
// likely to produce on demand.
// ---------------------------------------------------------------------------

/// A failure that never says `error` still has something to show.
///
/// This is the heart of the earlier defect and the reason [`Outcome::report`] is not a
/// `find(starts_with("error"))`. A broken manifest, a missing linker and a
/// registry that will not resolve all fail while saying something else.
#[test]
fn the_report_survives_a_failure_that_never_says_the_word_error() {
    let out = Outcome {
        success: false,
        cancelled: false,
        stdout: String::new(),
        stderr: "    Updating crates.io index\n\
                 warning: spurious network error\n\
                 caused by: failed to resolve\n"
            .into(),
    };
    assert_eq!(
        out.report().as_deref(),
        Some("caused by: failed to resolve"),
        "this is the `build failed, with nothing to show` of issue #67: the \
         reason was in the pipe and the shell printed a shrug"
    );
}

/// The last line and not the first: cargo's first lines are progress.
#[test]
fn the_report_takes_the_verdict_at_the_bottom_not_the_progress_at_the_top() {
    let out = Outcome {
        success: false,
        cancelled: false,
        stdout: String::new(),
        stderr: "   Compiling meu_app v0.1.0\n    Finished in 3s\nlink failed\n".into(),
    };
    assert_eq!(out.report().as_deref(), Some("link failed"));
}

/// What cargo marked wins over position.
#[test]
fn the_report_prefers_the_line_cargo_itself_marked() {
    let out = Outcome {
        success: false,
        cancelled: false,
        stdout: String::new(),
        stderr: "   Compiling meu_app\nerror: could not compile `meu_app`\nnote: see above\n"
            .into(),
    };
    assert_eq!(
        out.report().as_deref(),
        Some("error: could not compile `meu_app`"),
        "the `note:` is the last line, but the `error:` is the one written to \
         be read"
    );
}

/// With nothing on `stderr`, `stdout` is what is left — and it is used.
#[test]
fn the_report_falls_back_to_stdout_for_a_child_that_ignores_stderr() {
    let out = Outcome {
        success: false,
        cancelled: false,
        stdout: "something went sideways\n".into(),
        stderr: "   \n\n".into(),
    };
    assert_eq!(out.report().as_deref(), Some("something went sideways"));
}

/// `None` is reserved for the child that really said nothing.
#[test]
fn only_a_silent_child_reports_nothing() {
    let silent = Outcome {
        success: false,
        cancelled: false,
        stdout: "  \n".into(),
        stderr: String::new(),
    };
    assert_eq!(
        silent.report(),
        None,
        "`None` is the caller's cue to write its own sentence, and it has to \
         mean *nothing was said*"
    );
}

/// `stderr` first, because that is where cargo narrates.
#[test]
fn merged_puts_stderr_first_because_that_is_where_cargo_talks() {
    let out = Outcome {
        success: true,
        cancelled: false,
        stdout: "OUT".into(),
        stderr: "ERR".into(),
    };
    assert_eq!(out.merged(), "ERROUT");
}

/// A child that said nothing merges to nothing, rather than to a stray newline
/// that would look like output to `is_empty` callers.
#[test]
fn merged_of_a_silent_child_is_empty() {
    assert_eq!(Outcome::default().merged(), "");
}

// --------------------------------------------------------------- after a build

/// **A green build launches.** This is the fact an earlier change was missing.
///
/// The Makepad shell had every piece — the button, the action, the effect, this
/// very launcher — and no line anywhere that turned *"it compiled"* into *"then
/// start it"*. Measured here, without a window, because the alternative is
/// pressing `F5` and believing what one sees.
#[test]
fn a_green_build_is_answered_with_a_launch() {
    let green = Outcome {
        success: true,
        cancelled: false,
        stdout: String::new(),
        stderr: "    Finished `dev` profile in 3.10s\n".into(),
    };
    assert_eq!(
        after_build(Ok(green)),
        AfterBuild::Launch,
        "a build that succeeded has to end in a launch. Compiling and not \
         running is half of `Run`, and it is the half the owner sees."
    );
}

/// **A red build never launches**, and it says what cargo said.
///
/// Launching the binary of the previous build while the code on screen does not
/// compile is worse than not launching: the window that opens is a lie about
/// what is on disk.
#[test]
fn a_red_build_reports_what_cargo_said_and_starts_nothing() {
    let red = Outcome {
        success: false,
        cancelled: false,
        stdout: String::new(),
        stderr: "   Compiling meu_app v0.1.0\nerror: could not compile `meu_app`\n".into(),
    };
    assert_eq!(
        after_build(Ok(red)),
        AfterBuild::Report("error: could not compile `meu_app`".into()),
        "the line cargo marked `error` is the one written to be read"
    );
}

/// A red build that never said the word `error` still gets a sentence.
///
/// This is the earlier defect arriving at the other shell: a broken manifest or a
/// missing linker fails without that word, and the old code printed `nothing to
/// show` while the reason sat in the pipe.
#[test]
fn a_red_build_without_the_word_error_still_says_something() {
    let red = Outcome {
        success: false,
        cancelled: false,
        stdout: String::new(),
        stderr: "failed to parse manifest at `Cargo.toml`\n".into(),
    };
    assert_eq!(
        after_build(Ok(red)),
        AfterBuild::Report("failed to parse manifest at `Cargo.toml`".into())
    );
}

/// And a child that said **nothing at all** gets the one sentence written once.
#[test]
fn a_silent_red_build_falls_back_to_the_single_sentence() {
    assert_eq!(
        after_build(Ok(Outcome::default())),
        AfterBuild::Report(NOTHING_TO_SHOW.into()),
        "the fallback sentence has one author, because two literals is how a \
         sentence starts drifting between the shells"
    );
}

/// The process that never started is not code that does not compile.
///
/// Cargo missing from the `PATH` takes cargo's own words, not a build verdict —
/// the two need different sentences on screen.
#[test]
fn a_build_that_never_started_carries_its_own_reason() {
    assert_eq!(
        after_build(Err("could not run cargo: not found".into())),
        AfterBuild::Report("could not run cargo: not found".into())
    );
}

/// **Every child process of the product goes through [`no_console`].**
///
/// An earlier fix took the console away from the release build, and on Windows a
/// console program started without `CREATE_NO_WINDOW` then gets a console of
/// its own: `rust-analyzer` opened a black window next to the IDE the day the
/// IDE's own console went away. Before that, every spawn borrowed the IDE's
/// console and nobody saw the difference.
///
/// So this reads the production source of every crate and refuses a
/// `Command::new(` outside this module. Test code, `tests/` and `src/bin/`
/// tools are exempt: they never run inside the IDE. An inline
/// `#[cfg(test)] mod` is cut off at its first line, which is the conservative
/// side: it can only let a production line through if it sits BELOW a test
/// module, never report a test line as production.
#[test]
fn every_child_process_goes_through_no_console() {
    let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..");
    let mut files = Vec::new();
    for top in ["crates", "components"] {
        collect_rs(&workspace.join(top), &mut files);
    }
    // A gate that scans a folder and finds nothing stays green forever.
    assert!(
        files.len() > 100,
        "the scan found only {} files under crates/ and components/: the layout changed",
        files.len()
    );

    let mut offenders = Vec::new();
    for file in &files {
        let path = file.to_string_lossy().replace('\\', "/");
        let exempt = path.contains("/test/")
            || path.contains("/tests/")
            || path.contains("/bin/")
            || path.contains("/target/")
            || path.ends_with("/tests.rs")
            || path.ends_with("crawrad-os/src/process.rs");
        if exempt {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(file) else {
            continue;
        };
        let text = text.replace("\r\n", "\n");
        // The link opener ships INSIDE the generated app (the egui target
        // inlines its source), which cannot depend on this crate: it carries
        // `CREATE_NO_WINDOW` itself, and is held to it here.
        // `crawrad note --links` runs cargo doc from a CLI crate that stays
        // off the adapter: held to the same flag.
        if path.ends_with("crawrad-runtime/src/shell.rs")
            || path.ends_with("crawrad-cli/src/note.rs")
        {
            assert!(
                text.contains(".creation_flags(0x0800_0000)"),
                "{path} starts processes without `CREATE_NO_WINDOW`"
            );
            continue;
        }
        let production = match text.find("#[cfg(test)]\nmod ") {
            Some(i) => &text[..i],
            None => &text[..],
        };
        for (n, line) in production.lines().enumerate() {
            let code = line.trim_start();
            if code.starts_with("//") {
                continue;
            }
            if code.contains("Command::new(") {
                offenders.push(format!("{path}:{}: {}", n + 1, code));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "child processes started outside `crawrad_os::process::no_console`; on Windows \
         each one opens a console window next to the IDE:\n{}",
        offenders.join("\n")
    );
}

fn collect_rs(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            if p.file_name().is_some_and(|n| n == "target") {
                continue;
            }
            collect_rs(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}
