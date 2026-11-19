//! The Build dialog's target list.

use super::TargetPlatform;

#[test]
fn the_check_list_has_every_target_with_its_triple_and_state() {
    let (rows, triples, on) = TargetPlatform::check_list(&[TargetPlatform::Linux64]);
    assert_eq!(rows.len(), TargetPlatform::ALL.len());
    assert_eq!(rows[2], "🐧  Linux 64-bit (x64)");
    assert_eq!(triples[2], "x86_64-unknown-linux-gnu");
    assert_eq!(on.iter().filter(|&&b| b).count(), 1);
    assert!(on[2]);
}

#[test]
fn ticking_never_duplicates_and_unticking_removes() {
    let mut sel = vec![TargetPlatform::Win64];
    TargetPlatform::set_in(&mut sel, 1, true);
    assert_eq!(sel, vec![TargetPlatform::Win64]);
    TargetPlatform::set_in(&mut sel, 2, true);
    assert_eq!(sel, vec![TargetPlatform::Win64, TargetPlatform::Linux64]);
    TargetPlatform::set_in(&mut sel, 1, false);
    assert_eq!(sel, vec![TargetPlatform::Linux64]);
    TargetPlatform::set_in(&mut sel, 99, true);
    assert_eq!(sel, vec![TargetPlatform::Linux64]);
}

#[test]
fn test_build_page_default_and_page_title() {
    assert_eq!(super::BuildPage::default(), super::BuildPage::Options);
    assert_eq!(super::page_title(super::BuildMode::Build), "Build");
    assert_eq!(super::page_title(super::BuildMode::Run), "Run");
    assert_eq!(super::page_title(super::BuildMode::Check), "Check");
    assert_eq!(super::page_title(super::BuildMode::Clean), "Clean");
}

#[test]
fn test_build_state_runs_and_start_run() {
    let mut state = super::BuildState::default();
    assert_eq!(state.page, super::BuildPage::Options);
    assert!(state.runs.is_empty());
    assert!(state.run(super::BuildMode::Build).is_none());

    let p1 = super::BuildProgress::new("Building", "my_app", "cargo build");
    state.start_run(super::BuildMode::Build, p1);
    assert_eq!(state.runs.len(), 1);
    assert!(state.run(super::BuildMode::Build).is_some());
    assert_eq!(
        state.run(super::BuildMode::Build).unwrap().verb(),
        "Building"
    );

    // Overwriting the run for the same mode
    let p2 = super::BuildProgress::new("Building", "my_app", "cargo build --release");
    state.start_run(super::BuildMode::Build, p2);
    assert_eq!(state.runs.len(), 1);
    assert_eq!(
        state.run(super::BuildMode::Build).unwrap().log()[0],
        "> cargo build --release"
    );

    // Adding another mode
    let p3 = super::BuildProgress::new("Checking", "my_app", "cargo check");
    state.start_run(super::BuildMode::Check, p3);
    assert_eq!(state.runs.len(), 2);
    assert!(state.run(super::BuildMode::Check).is_some());

    // run_mut works
    if let Some(r) = state.run_mut(super::BuildMode::Build) {
        r.append_log_line("added line");
    }
    assert_eq!(state.run(super::BuildMode::Build).unwrap().log().len(), 2);

    // switch_page records timestamp only on change
    assert_eq!(state.page_switched_at, None);
    state.switch_page(super::BuildPage::Options);
    assert_eq!(state.page_switched_at, None); // was already Options
    state.switch_page(super::BuildPage::Command(super::BuildMode::Build));
    assert!(state.page_switched_at.is_some());
    assert_eq!(
        state.page,
        super::BuildPage::Command(super::BuildMode::Build)
    );

    // record_finish records mode and instant
    assert_eq!(state.finished_at, None);
    let now = std::time::Instant::now();
    state.record_finish(super::BuildMode::Build, now);
    assert_eq!(state.finished_at, Some((super::BuildMode::Build, now)));

    // needs_tick returns true while running or lingering
    assert!(state.needs_tick()); // because finished_at was just set and < CARD_LINGER
    state.finished_at = None;
    state.page_switched_at = None;
    assert!(!state.needs_tick());
}

#[test]
fn test_meta_line_formatting() {
    let mut state = super::BuildState {
        profile: super::BuildProfile::Debug,
        selected_targets: vec![TargetPlatform::Win64],
        env_vars: vec![("RUST_LOG".into(), "info".into())],
        ..Default::default()
    };

    let meta = state.meta_line();
    assert_eq!(meta, "Debug · Windows 64-bit (x64) · RUST_LOG=info");

    // Without env vars
    state.env_vars.clear();
    assert_eq!(state.meta_line(), "Debug · Windows 64-bit (x64)");

    // With empty targets, host default is used
    state.selected_targets.clear();
    let host_meta = state.meta_line();
    assert!(host_meta.starts_with("Debug · "));
}

#[test]
fn test_cargo_call_and_target_helpers() {
    use std::path::Path;

    let mut state = super::BuildState::default();
    state.ensure_default_targets();
    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    assert!(!state.selected_targets.is_empty());

    state.selected_targets = vec![TargetPlatform::Linux64, TargetPlatform::Win32];
    state.sort_targets();
    assert_eq!(
        state.selected_targets,
        vec![TargetPlatform::Win32, TargetPlatform::Linux64]
    );

    let (call, cmd_line) = state.cargo_call(super::CargoCommand::Build, "my_pkg", Path::new("."));
    assert!(cmd_line.contains("cargo build -p my_pkg"));
    assert_eq!(call.command().get_program(), "cargo");
}

#[test]
fn test_build_state_new_decision_methods() {
    use std::time::{Duration, Instant};

    let mut state = super::BuildState {
        selected_targets: vec![TargetPlatform::Win64, TargetPlatform::Wasm32],
        ..Default::default()
    };

    // 1. target_chip_labels
    assert_eq!(
        state.target_chip_labels(),
        vec![
            "Windows 64-bit (x64)".to_string(),
            "WebAssembly (wasm32)".to_string()
        ]
    );

    // 2. rail_label & is_mode_enabled
    assert_eq!(state.rail_label(super::BuildMode::Build), "🔨 Build");
    assert_eq!(state.rail_label(super::BuildMode::Run), "▶ Run");
    assert_eq!(state.rail_label(super::BuildMode::Check), "📝 Check");
    assert_eq!(state.rail_label(super::BuildMode::Clean), "🧹 Clean");
    assert!(state.is_mode_enabled(super::BuildMode::Build));
    assert!(state.is_mode_enabled(super::BuildMode::Run));

    // When running with start instant long ago (past busy delay of 150 ms)
    let past = Instant::now() - Duration::from_millis(200);
    state.running = Some((super::BuildMode::Build, crawrad_os::tasks::TaskId(1), past));
    assert!(state.is_mode_enabled(super::BuildMode::Build));
    assert!(!state.is_mode_enabled(super::BuildMode::Run));
    let busy_txt = state.rail_label(super::BuildMode::Build);
    assert!(busy_txt.ends_with("Building"));
    assert_eq!(state.rail_label(super::BuildMode::Run), "▶ Run");

    // 3. active_or_last_mode
    assert_eq!(state.active_or_last_mode(), super::BuildMode::Build);

    // 4. page_offset
    state.page_switched_at = Some(Instant::now());
    assert!(state.page_offset() > 0.0);
    state.page_switched_at = Some(Instant::now() - Duration::from_millis(200));
    assert_eq!(state.page_offset(), 0.0);

    // 5. take_running & finish_failed_run
    let (m, el) = state.take_running();
    assert_eq!(m, super::BuildMode::Build);
    assert!(el >= Duration::from_millis(200));
    assert!(state.running.is_none());

    let prog = super::BuildProgress::new("Building", "pkg", "cargo build");
    state.start_run(super::BuildMode::Build, prog);
    state.finish_failed_run(
        super::BuildMode::Build,
        Duration::from_secs(1),
        "syntax error",
    );
    let run = state.run(super::BuildMode::Build).unwrap();
    assert_eq!(run.state(), crate::build_progress::ProgressState::Failed);
    assert!(run.log().iter().any(|l| l.contains("syntax error")));

    // 6. after_finish: failed and dialog closed -> opens window and switches to Errors
    state.dialog = false;
    let should_open = state.after_finish(super::BuildMode::Build, Instant::now());
    assert!(should_open);
    assert_eq!(
        state.page,
        super::BuildPage::Command(super::BuildMode::Build)
    );
    assert_eq!(state.tab, super::BuildTab::Errors);

    // If dialog was open, after_finish returns false
    state.dialog = true;
    let should_open_again = state.after_finish(super::BuildMode::Build, Instant::now());
    assert!(!should_open_again);

    // BuildMode::Run never opens on finish even when failed and dialog closed
    state.dialog = false;
    let prog_run = super::BuildProgress::new("Running", "pkg", "cargo build");
    state.start_run(super::BuildMode::Run, prog_run);
    state.finish_failed_run(
        super::BuildMode::Run,
        Duration::from_secs(1),
        "syntax error",
    );
    let run_opens = state.after_finish(super::BuildMode::Run, Instant::now());
    assert!(!run_opens);

    // 7. card_now (lingering after finish)
    let card = state.card_now();
    assert!(card.is_some());
    let c = card.unwrap();
    assert_eq!(c.icon, "✕");
    assert!(!c.running);

    // 8. command_view_info
    let (headline, _counter, clock, frac) = state.command_view_info(super::BuildMode::Build);
    assert!(!headline.is_empty());
    assert_eq!(clock, "00:01");
    assert_eq!(frac, None);

    // 9. update_diagnostics
    let diags = vec![
        crate::diagnostics_model::Diagnostic {
            message: "error found".into(),
            severity: crate::diagnostics_model::Severity::Error,
            location: None,
            rendered: String::new(),
        },
        crate::diagnostics_model::Diagnostic {
            message: "warn found".into(),
            severity: crate::diagnostics_model::Severity::Warning,
            location: None,
            rendered: String::new(),
        },
    ];
    state.update_diagnostics(&diags);
    assert_eq!(state.errors_count, 1);
    assert_eq!(state.warnings_count, 1);
}

/// The build clock rewrote the bar every 0.1 s and the refusal of a second
/// Run was gone before anyone could read it.
#[test]
fn a_refusal_holds_the_bar_then_the_build_line_comes_back() {
    use super::{ALREADY_BUILDING, BuildMode, BuildState, REFUSAL_HOLD};
    use crate::build_progress::BuildProgress;
    let mut b = BuildState::default();
    b.start_run(
        BuildMode::Run,
        BuildProgress::new("Running", "meu_app", "cargo build"),
    );
    b.running = Some((
        BuildMode::Run,
        crawrad_os::tasks::TaskId(1),
        std::time::Instant::now(),
    ));
    assert_eq!(b.refuse_busy(), ALREADY_BUILDING);
    assert_eq!(b.status_line_now().as_deref(), Some(ALREADY_BUILDING));
    b.refused_at = std::time::Instant::now().checked_sub(REFUSAL_HOLD);
    assert!(b.status_line_now().unwrap().starts_with("Building meu_app"));
}

#[test]
fn wants_map_decision_matrix() {
    let state = super::BuildState {
        profile: super::BuildProfile::Release,
        ..Default::default()
    };

    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    {
        let mut state = state;
        // Release + Build + host -> true
        assert!(state.wants_map(super::CargoCommand::Build));

        // Debug -> false
        state.profile = super::BuildProfile::Debug;
        assert!(!state.wants_map(super::CargoCommand::Build));
        state.profile = super::BuildProfile::Release;

        // Check -> false
        assert!(!state.wants_map(super::CargoCommand::Check));

        // Cross target -> false
        state.selected_targets = vec![super::TargetPlatform::Linux64];
        assert!(!state.wants_map(super::CargoCommand::Build));
        state.selected_targets.clear();

        // extra_args containing "--" -> false
        state.extra_args = "-- -C opt-level=3".into();
        assert!(!state.wants_map(super::CargoCommand::Build));
        state.extra_args = "--".into();
        assert!(!state.wants_map(super::CargoCommand::Build));
    }

    #[cfg(not(all(target_os = "windows", target_env = "msvc")))]
    {
        assert!(!state.wants_map(super::CargoCommand::Build));
    }
}

#[test]
fn cargo_call_release_uses_rustc_with_map_link_args_and_debug_stays_unchanged() {
    use std::path::Path;

    let debug_state = super::BuildState {
        profile: super::BuildProfile::Debug,
        ..Default::default()
    };
    let root = Path::new("my_project");
    let (debug_call, debug_cmd) =
        debug_state.cargo_call(super::CargoCommand::Build, "my_pkg", root);
    assert_eq!(debug_cmd, "cargo build -p my_pkg");
    let debug_args: Vec<_> = debug_call
        .command()
        .get_args()
        .map(|s| s.to_string_lossy().to_string())
        .collect();
    assert_eq!(
        debug_args,
        vec!["build", "-p", "my_pkg", "--message-format=json"]
    );

    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    {
        let release_state = super::BuildState {
            profile: super::BuildProfile::Release,
            ..Default::default()
        };
        // A folder with a space: the map path must stay ONE argument.
        let spaced = Path::new("Ana Lima").join("my_project");
        let (release_call, release_cmd) =
            release_state.cargo_call(super::CargoCommand::Build, "my_pkg", &spaced);
        let release_args: Vec<_> = release_call
            .command()
            .get_args()
            .map(|s| s.to_string_lossy().to_string())
            .collect();

        let map = crate::release_size::map_file(&spaced.join("target"), "my_pkg");
        assert_eq!(
            release_args,
            vec![
                "rustc".to_string(),
                "-p".into(),
                "my_pkg".into(),
                "--message-format=json".into(),
                "--release".into(),
                "--bin".into(),
                "my_pkg".into(),
                "--".into(),
                "-C".into(),
                format!("link-arg=/MAP:{}", map.display()),
            ]
        );
        assert!(release_args.contains(&"rustc".to_string()));
        assert!(release_args.contains(&"--bin".to_string()));
        assert!(release_args.contains(&"my_pkg".to_string()));
        assert!(release_args.contains(&"--".to_string()));
        let map_arg = release_args
            .iter()
            .find(|a| a.starts_with("link-arg=/MAP:"))
            .expect("map link arg");
        assert!(map_arg.ends_with("crawrad-size-my_pkg.map"));

        assert!(release_cmd.contains("cargo rustc -p my_pkg --release"));
        assert!(release_cmd.contains("--bin my_pkg"));
        assert!(release_cmd.contains("-- -C link-arg=/MAP:"));
        assert!(release_cmd.ends_with("crawrad-size-my_pkg.map"));
    }
}

#[test]
fn test_build_page_size_and_unseen_flag() {
    let mut state = super::BuildState::default();
    assert!(!state.size_unseen);
    assert!(state.last_release_symbols.is_empty());

    state.size_unseen = true;
    state.switch_page(super::BuildPage::Size);
    assert_eq!(state.page, super::BuildPage::Size);
    assert!(!state.size_unseen); // Cleared upon opening Size page
}

/// The whole Windows chain on a REAL project in a folder with a space: the command the IDE
/// builds runs, the linker writes the map, and `measure` reads crates out of it.
#[test]
#[ignore = "builds a real Cargo project (~20 s); runs in the gate's #[ignore] step"]
#[cfg(all(target_os = "windows", target_env = "msvc"))]
fn release_build_writes_a_map_that_measure_reads() {
    let base = std::env::temp_dir().join(format!("crawrad size {}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).expect("temp folder");
    let made = std::process::Command::new("cargo")
        .args(["new", "-q", "--vcs", "none", "size_demo"])
        .current_dir(&base)
        .status()
        .expect("cargo new");
    assert!(made.success());
    let root = base.join("size_demo");
    let state = super::BuildState {
        profile: super::BuildProfile::Release,
        ..Default::default()
    };
    let (call, _) = state.cargo_call(super::CargoCommand::Build, "size_demo", &root);
    let built = call.command().output().expect("cargo rustc");
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );

    let map = crate::release_size::map_file(&root.join("target"), "size_demo");
    let map_text = std::fs::read_to_string(&map).expect("the linker wrote the map");
    let exe = root.join("target").join("release").join("size_demo.exe");
    let release =
        crate::release_size::measure("size_demo", &exe, Some(&map_text), 1).expect("measure");
    assert_eq!(release.bytes, std::fs::metadata(&exe).expect("exe").len());
    assert!(
        release.crates.get("size_demo").is_some_and(|b| *b > 0),
        "{:?}",
        release.crates
    );
    assert!(
        release.crates.get("std").is_some_and(|b| *b > 0),
        "{:?}",
        release.crates
    );
    let _ = std::fs::remove_dir_all(&base);
}

use super::{BuildMode, BuildProfile, BuildState};

fn release_report(
    dir: &std::path::Path,
    mode: BuildMode,
    success: bool,
) -> crate::build_report::Report {
    let exe = dir.join("demo.exe");
    std::fs::write(&exe, vec![0u8; 2048]).unwrap();
    crate::build_report::Report {
        manifest: "Cargo.toml".into(),
        platform: "Windows 64-bit".into(),
        configuration: "Release".into(),
        executables: vec![(exe, Some(2048))],
        mode,
        package: "demo".into(),
        success,
        errors: 0,
        warnings: 0,
        error_lines: Vec::new(),
    }
}

#[test]
fn closing_summary_adds_and_records_the_size_only_for_a_finished_release_build() {
    let dir_path = std::env::temp_dir().join(format!(
        "crawrad_test_closing_summary_{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir_path);
    std::fs::create_dir_all(&dir_path).unwrap();
    let dir = &dir_path;
    let root = Some(dir.as_path());
    let plain = |r: &crate::build_report::Report| crate::build_report::summary(r);

    let mut s = BuildState::default();
    let debug = release_report(dir, BuildMode::Build, true);
    assert_eq!(s.closing_summary(&debug, false, root), plain(&debug));

    s.profile = BuildProfile::Release;
    for (mode, success, cancelled) in [
        (BuildMode::Run, true, false),
        (BuildMode::Build, false, false),
        (BuildMode::Build, true, true),
    ] {
        let r = release_report(dir, mode, success);
        assert_eq!(s.closing_summary(&r, cancelled, root), plain(&r));
    }
    assert!(crate::release_size::read_history(dir).is_empty());

    let r = release_report(dir, BuildMode::Build, true);
    let lines = s.closing_summary(&r, false, root);
    assert_eq!(lines.len(), plain(&r).len() + 1);
    assert!(
        lines.last().unwrap().starts_with("Release size: 2 KB"),
        "{lines:?}"
    );
    assert_eq!(crate::release_size::read_history(dir).len(), 1);
    assert!(
        s.size_unseen,
        "a recorded Release marks the Size page as new"
    );

    let no_root = s.closing_summary(&r, false, None);
    assert_eq!(
        no_root.last().unwrap(),
        "Release size not recorded: no project root"
    );
}

#[test]
fn release_card_recorded_and_cleared_on_next_build_start() {
    let dir_path =
        std::env::temp_dir().join(format!("crawrad_test_release_card_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir_path);
    std::fs::create_dir_all(&dir_path).unwrap();
    let dir = &dir_path;
    let root = Some(dir.as_path());

    let mut s = BuildState {
        profile: BuildProfile::Release,
        ..Default::default()
    };

    let r = release_report(dir, BuildMode::Build, true);
    s.closing_summary(&r, false, root);

    assert!(
        s.last_release.is_some(),
        "last_release is set on successful measured release"
    );
    let card = s.last_release.as_ref().unwrap();
    assert_eq!(card.size, "2 KB");
    assert_eq!(s.status_text(), Some("Release 2 KB".to_string()));
    assert!(!s.release_card_dismissed);

    // Next build of any kind starts -> cleared
    let cmd_line = "cargo check -p demo";
    s.start_run(
        BuildMode::Check,
        crate::build_progress::BuildProgress::new("Checking", "demo", cmd_line),
    );
    assert!(s.last_release.is_none(), "cleared on next build start");
    assert!(!s.release_card_dismissed);
    assert_eq!(s.status_text(), None);

    let _ = std::fs::remove_dir_all(&dir_path);
}

#[test]
fn release_card_dismissed_when_dialog_is_open() {
    let dir_path = std::env::temp_dir().join(format!(
        "crawrad_test_release_card_dialog_{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir_path);
    std::fs::create_dir_all(&dir_path).unwrap();
    let dir = &dir_path;
    let root = Some(dir.as_path());

    let mut s = BuildState {
        profile: BuildProfile::Release,
        dialog: true, // window open
        ..Default::default()
    };

    let r = release_report(dir, BuildMode::Build, true);
    s.closing_summary(&r, false, root);

    assert!(s.last_release.is_some());
    assert!(
        s.release_card_dismissed,
        "card is dismissed when finished with build window open"
    );
    // Status text is still available
    assert_eq!(s.status_text(), Some("Release 2 KB".to_string()));

    let _ = std::fs::remove_dir_all(&dir_path);
}
