//! The whole path against a REAL `lldb-dap`: build a program, launch it under
//! [`DebugSession`], stop at a breakpoint, read the locals through
//! [`RustPresenter`], step, continue, and see it end.
//!
//! # When it runs
//!
//! Only when `CRAWRAD_LLDB_DAP` names an `lldb-dap` executable — the installer
//! does not carry one yet. Without it the test says so on
//! stderr and passes; that pass proves nothing, and the message says exactly
//! that. `CRAWRAD_DAP_TOOLCHAIN` (e.g. `stable-x86_64-pc-windows-gnu`) picks the
//! toolchain the program is built with: `-gnu` (layouts from DWARF) and
//! `-msvc` (layouts from the PDB) are both measured, with the same values.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use crawrad_dap::{DebugSession, LaunchConfig, SessionState};
use crawrad_dap_rust::RustPresenter;

const PROGRAM: &str = r#"#[allow(dead_code)]
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rect(u32, u32),
}

#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let greeting = String::from("Hello, CrawRAD");
    let numbers: Vec<i32> = vec![10, 20, 30];
    let shape = Shape::Circle { radius: 2.0 };
    let rect = Shape::Rect(3, 4);
    let some: Option<i32> = Some(7);
    let names: Vec<String> = vec!["rad".into(), "makepad".into()];
    let people = vec![Person { name: "Ana".into(), age: 30 }, Person { name: "Rui".into(), age: 41 }];
    let shapes = vec![Shape::Rect(1, 2), Shape::Circle { radius: 0.5 }];
    let maybe: Vec<Option<String>> = vec![None, Some("x".into())];
    let ages: std::collections::HashMap<String, u32> = [("Ana".to_string(), 30), ("Rui".to_string(), 41)].into();
    let ids: std::collections::HashSet<i32> = [7, -3].into();
    let pairs: Vec<(u32, i32)> = vec![(1, 2)];
    let small: Vec<i8> = vec![-1, 2];
    let (ch, b8, pair, none, chars, words): (char, i8, (u32, i32), Option<String>, Vec<char>, Vec<&str>) = ('z', -1, (1, -2), None, vec!['a'], vec!["a"]);
    println!("{greeting} {numbers:?} {shape:?} {names:?} {people:?} {shapes:?} {maybe:?}"); // BREAKPOINT (line 30)
    println!("{rect:?} {some:?} {ages:?} {ids:?} {pairs:?} {small:?} {ch:?} {b8} {pair:?} {none:?} {chars:?} {words:?}");
}
"#;
const BREAKPOINT_LINE: i64 = 30;
const T: Duration = Duration::from_secs(60);

fn build(dir: &Path, toolchain: Option<&str>) -> PathBuf {
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"dap_probe\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )
    .unwrap();
    std::fs::write(dir.join("src").join("main.rs"), PROGRAM).unwrap();
    let mut cargo = Command::new("cargo");
    if let Some(tc) = toolchain {
        cargo.arg(format!("+{tc}"));
    }
    let status = cargo
        .arg("build")
        .current_dir(dir)
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("RUSTFLAGS")
        .status()
        .expect("cargo runs");
    assert!(status.success(), "the probe program must build");
    dir.join("target").join("debug").join(if cfg!(windows) {
        "dap_probe.exe"
    } else {
        "dap_probe"
    })
}

#[test]
fn a_real_lldb_dap_stops_and_the_presenter_reads_the_locals() {
    let Some(adapter) = std::env::var_os("CRAWRAD_LLDB_DAP").map(PathBuf::from) else {
        eprintln!(
            "SKIPPED: CRAWRAD_LLDB_DAP is not set — this pass proves NOTHING about a real adapter \
             (issue #276, PR 6 bundles one)."
        );
        return;
    };
    let toolchain = std::env::var("CRAWRAD_DAP_TOOLCHAIN").ok();
    let dir = std::env::temp_dir().join(format!("crawrad-dap-real-{}", std::process::id()));
    let program = build(&dir, toolchain.as_deref());
    let source = dir.join("src").join("main.rs");

    let config = LaunchConfig {
        program,
        cwd: Some(dir.clone()),
        ..LaunchConfig::default()
    };
    let breakpoints = BTreeMap::from([(source.clone(), vec![BREAKPOINT_LINE])]);
    let mut session =
        DebugSession::launch(&adapter, &config, &breakpoints).expect("the session starts");
    assert!(
        session.breakpoints()[&source][0].verified,
        "the breakpoint must be verified"
    );

    let SessionState::Stopped { thread_id, reason } = session.wait_for_stop(T).unwrap().clone()
    else {
        panic!("it must stop at the breakpoint, got {:?}", session.state());
    };
    assert_eq!(reason, "breakpoint");
    let frames = session.stack(thread_id, 5).unwrap();
    assert_eq!(frames[0].line, BREAKPOINT_LINE);
    assert!(
        frames[0].name.contains("main"),
        "stopped in {}",
        frames[0].name
    );

    let frame = frames[0].id;
    // The DWARF of the program itself, for the elements the adapter cannot type.
    let presenter = RustPresenter::for_program(&config.program);
    if toolchain
        .as_deref()
        .is_some_and(|t| t.contains("-gnu") || t.contains("-msvc"))
    {
        assert!(
            presenter.has_layouts(),
            "a -gnu build carries DWARF and a -msvc one a PDB: the layouts must be read"
        );
    }
    let shown: BTreeMap<String, String> = session
        .locals(frame)
        .unwrap()
        .iter()
        .map(|v| (v.name.clone(), session.present(v, frame, &presenter)))
        .collect();
    assert_eq!(shown["greeting"], "\"Hello, CrawRAD\"");
    assert_eq!(shown["numbers"], "[10, 20, 30]");
    assert_eq!(shown["shape"], "Circle { radius: 2.0 }");
    assert_eq!(shown["rect"], "Rect(3, 4)");
    assert_eq!(shown["some"], "Some(7)");
    // LLDB spells `i8` as `char`: it must not be read as a Rust `char`.
    assert_eq!(shown["small"], "[-1, 2]");
    assert_eq!(shown["ch"], "'z'");
    assert_eq!(shown["b8"], "-1");
    assert_eq!(shown["pair"], "(1, -2)");
    assert_eq!(shown["none"], "None");
    // A bare `char` is an `i8` from DWARF and Rust's `char` from a PDB.
    assert_eq!(shown["chars"], "['a']");
    // A local the adapter cannot read (the `args` of a `println!`) is its
    // message, never a walk of failed children.
    assert!(
        shown.values().all(|v| !v.contains("parent failed")),
        "{shown:?}"
    );
    if presenter.has_layouts() {
        assert_eq!(shown["names"], r#"["rad", "makepad"]"#);
        assert_eq!(
            shown["people"],
            r#"[Person { name: "Ana", age: 30 }, Person { name: "Rui", age: 41 }]"#
        );
        assert_eq!(shown["shapes"], "[Rect(1, 2), Circle { radius: 0.5 }]");
        assert_eq!(shown["maybe"], r#"[None, Some("x")]"#);
        assert_eq!(shown["pairs"], "[(1, 2)]");
        assert_eq!(shown["words"], r#"["a"]"#);
        // A HashMap's order changes every run (RandomState): each order is
        // accepted here, and the program's own `{:?}` below says which one.
        assert!(
            [r#"{"Ana": 30, "Rui": 41}"#, r#"{"Rui": 41, "Ana": 30}"#]
                .contains(&shown["ages"].as_str()),
            "ages: {}",
            shown["ages"]
        );
        assert!(
            ["{7, -3}", "{-3, 7}"].contains(&shown["ids"].as_str()),
            "ids: {}",
            shown["ids"]
        );
    } else {
        // No layouts (no DWARF, no PDB beside the program): the count, never a guess.
        assert_eq!(shown["names"], "[… 2 items]");
    }

    session.step_over().unwrap();
    let SessionState::Stopped { thread_id, .. } = session.wait_for_stop(T).unwrap().clone() else {
        panic!("a step must stop again, got {:?}", session.state());
    };
    assert_eq!(
        session.stack(thread_id, 1).unwrap()[0].line,
        BREAKPOINT_LINE + 1
    );

    session.continue_().unwrap();
    assert_eq!(session.wait_for_stop(T).unwrap(), &SessionState::Terminated);
    let printed = session.output().concat();
    assert!(
        printed.contains("Olá, CrawRAD"),
        "the program's output must reach the session: {printed:?}"
    );
    if presenter.has_layouts() {
        // Same process, same hashes: the entries in the order `{:?}` printed them.
        let expected = format!("{} {}", shown["ages"], shown["ids"]);
        assert!(
            printed.contains(&expected),
            "HashMap/HashSet order: shown {expected:?}, printed {printed:?}"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// The same program through the IDE's way in: a [`DebugController`] on its own
/// thread, commands in, updates out — and the presenter's text in the stop.
#[test]
fn the_controller_drives_a_real_lldb_dap_without_blocking_the_caller() {
    use crawrad_dap::{Command, DebugController, Update};
    let Some(adapter) = std::env::var_os("CRAWRAD_LLDB_DAP").map(PathBuf::from) else {
        eprintln!(
            "SKIPPED: CRAWRAD_LLDB_DAP is not set — this pass proves NOTHING about a real adapter."
        );
        return;
    };
    let toolchain = std::env::var("CRAWRAD_DAP_TOOLCHAIN").ok();
    let dir = std::env::temp_dir().join(format!("crawrad-dap-ctl-{}", std::process::id()));
    let program = build(&dir, toolchain.as_deref());
    let source = dir.join("src").join("main.rs");
    let config = LaunchConfig {
        program: program.clone(),
        cwd: Some(dir.clone()),
        ..LaunchConfig::default()
    };
    let controller = DebugController::launch_process(
        adapter,
        config,
        BTreeMap::from([(source, vec![BREAKPOINT_LINE])]),
        Box::new(RustPresenter::for_program(&program)),
        || {},
    );
    let mut seen = Vec::new();
    let mut until = |done: &dyn Fn(&Update) -> bool| {
        let deadline = std::time::Instant::now() + T;
        loop {
            for update in controller.take_updates() {
                let hit = done(&update);
                seen.push(update);
                if hit {
                    return seen.last().cloned().unwrap();
                }
            }
            assert!(
                std::time::Instant::now() < deadline,
                "no such update; saw {seen:?}"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
    };
    let Update::Stopped(stop) = until(&|u| matches!(u, Update::Stopped(_) | Update::Failed(_)))
    else {
        panic!("the controller must stop at the breakpoint");
    };
    assert_eq!(stop.frames[0].line, BREAKPOINT_LINE);
    let greeting = stop.locals.iter().find(|v| v.name == "greeting").unwrap();
    assert_eq!(greeting.value, "\"Olá, CrawRAD\"");

    controller.send(Command::StepOver);
    let Update::Stopped(stop) = until(&|u| matches!(u, Update::Stopped(_) | Update::Failed(_)))
    else {
        panic!("a step must stop again");
    };
    assert_eq!(stop.frames[0].line, BREAKPOINT_LINE + 1);

    let asked = std::time::Instant::now();
    controller.send(Command::Stop);
    until(&|u| matches!(u, Update::Terminated { .. } | Update::Failed(_)));
    assert!(
        asked.elapsed() < Duration::from_secs(10),
        "Stop took {:?}",
        asked.elapsed()
    );
    drop(controller);
    let _ = std::fs::remove_dir_all(&dir);
}

/// **A logpoint writes its message and the program does not stop.**
///
/// The values in the braces are written by the IDE's presenter, as `{:?}`
/// would: LLDB's own `logMessage` would print a `String` as its type name.
#[test]
fn a_logpoint_writes_with_the_presenter_and_never_stops() {
    use crawrad_dap::{DebugController, SourceBreakpoint, Update};
    let Some(adapter) = std::env::var_os("CRAWRAD_LLDB_DAP").map(PathBuf::from) else {
        eprintln!(
            "SKIPPED: CRAWRAD_LLDB_DAP is not set — this pass proves NOTHING about a real adapter."
        );
        return;
    };
    let toolchain = std::env::var("CRAWRAD_DAP_TOOLCHAIN").ok();
    let dir = std::env::temp_dir().join(format!("crawrad-dap-log-{}", std::process::id()));
    let program = build(&dir, toolchain.as_deref());
    let source = dir.join("src").join("main.rs");
    let config = LaunchConfig {
        program: program.clone(),
        cwd: Some(dir.clone()),
        ..LaunchConfig::default()
    };
    let logpoint = SourceBreakpoint {
        line: BREAKPOINT_LINE,
        log_message: Some("greeting={greeting} numbers={numbers} some={some}".into()),
    };
    let controller = DebugController::launch_process(
        adapter,
        config,
        BTreeMap::from([(source, vec![logpoint])]),
        Box::new(RustPresenter::for_program(&program)),
        || {},
    );
    let deadline = std::time::Instant::now() + T;
    let mut seen = Vec::new();
    loop {
        let updates = controller.take_updates();
        let over = updates
            .iter()
            .any(|u| matches!(u, Update::Terminated { .. } | Update::Failed(_)));
        seen.extend(updates);
        if over {
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "it never ended; saw {seen:?}"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(
        !seen.iter().any(|u| matches!(u, Update::Stopped(_))),
        "a logpoint must not stop the program: {seen:?}"
    );
    let output: String = seen
        .iter()
        .filter_map(|u| match u {
            Update::Output(text) => Some(text.as_str()),
            _ => None,
        })
        .collect();
    assert!(
        output.contains(r#"greeting="Olá, CrawRAD" numbers=[10, 20, 30] some=Some(7)"#),
        "the logpoint's line, written by the presenter, is missing: {output:?}"
    );
    drop(controller);
    let _ = std::fs::remove_dir_all(&dir);
}
