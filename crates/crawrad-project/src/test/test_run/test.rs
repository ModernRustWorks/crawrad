use super::*;
use crate::test_runner::Outcome as Verdict;

const META: &str = r#"{"packages":[{"name":"calc","manifest_path":"C:\\w\\Cargo.toml","targets":[
    {"kind":["lib"],"name":"calc","src_path":"C:\\w\\src\\lib.rs","doctest":false},
    {"kind":["test"],"name":"api","src_path":"C:\\w\\tests\\api.rs"}]}]}"#;

const LIB_RUN: &str = include_str!("../test_runner/lib_run.txt");

fn ok(stdout: &str) -> Result<Outcome, String> {
    Ok(Outcome {
        success: true,
        stdout: stdout.into(),
        ..Default::default()
    })
}

/// A cargo that answers metadata, lists and runs, and records what it was asked.
fn fake(asked: &mut Vec<String>) -> impl FnMut(&[String]) -> Result<Outcome, String> + '_ {
    move |args: &[String]| {
        asked.push(args.join(" "));
        match args[0].as_str() {
            "metadata" => ok(META),
            _ if args.last().is_some_and(|a| a == "--list") => ok("a: test\nb: test\n"),
            _ if args.contains(&"--lib".to_string()) => Ok(Outcome {
                success: false,
                stdout: LIB_RUN.into(),
                ..Default::default()
            }),
            _ => Ok(Outcome {
                success: false,
                stderr: "error[E0425]: cannot find value `x`".into(),
                ..Default::default()
            }),
        }
    }
}

#[test]
fn list_asks_every_target_for_its_tests_and_runs_none() {
    let mut asked = Vec::new();
    let r = perform(
        &TestJob::List,
        "calc",
        Path::new(r"C:\w"),
        fake(&mut asked),
        |_| {},
    );
    assert_eq!(
        r.lists,
        [
            (0, vec!["a".to_string(), "b".into()]),
            (1, vec!["a".to_string(), "b".into()])
        ]
    );
    assert!(r.runs.is_empty());
    assert_eq!(asked[1], "test -p calc --lib -- --list");
}

#[test]
fn run_all_runs_each_target_and_a_build_error_is_told_not_hidden() {
    let mut asked = Vec::new();
    let mut said = Vec::new();
    let r = perform(
        &TestJob::RunAll,
        "calc",
        Path::new(r"C:\w"),
        fake(&mut asked),
        |t| said.push(t),
    );
    assert!(r.full);
    assert_eq!(said, ["Running src/lib.rs", "Running tests/api.rs"]);
    assert_eq!(r.runs[0].1.outcomes.len(), 3);
    assert_eq!(
        r.runs[1].1.error.as_deref(),
        Some("error[E0425]: cannot find value `x`")
    );
    let mut tree = TestTree::default();
    assert_eq!(
        tree.apply(r).as_deref(),
        Some("error[E0425]: cannot find value `x`")
    );
    assert_eq!(tree.count(Verdict::Failed), 1);
}

#[test]
fn run_failed_names_the_tests_and_only_their_targets() {
    let mut asked = Vec::new();
    let job = TestJob::Run(vec![(0, vec!["math::tests::divides_by_ten".into()])]);
    let r = perform(&job, "calc", Path::new(r"C:\w"), fake(&mut asked), |_| {});
    assert_eq!(asked.len(), 2);
    assert_eq!(
        asked[1],
        "test -p calc --lib -- math::tests::divides_by_ten --exact"
    );
    assert!(!r.full);
}

#[test]
fn an_unknown_package_or_broken_metadata_is_an_error() {
    let r = perform(
        &TestJob::RunAll,
        "nope",
        Path::new(r"C:\w"),
        |_| ok(META),
        |_| {},
    );
    assert_eq!(r.error.as_deref(), Some("nope has no test targets"));
    let r = perform(
        &TestJob::RunAll,
        "calc",
        Path::new(r"C:\w"),
        |_| Err("no cargo".into()),
        |_| {},
    );
    assert_eq!(r.error.as_deref(), Some("no cargo"));
}

#[test]
fn a_full_run_forgets_tests_that_are_gone_and_new_targets_start_over() {
    let mut asked = Vec::new();
    let mut tree = TestTree::default();
    tree.apply(perform(
        &TestJob::List,
        "calc",
        Path::new(r"C:\w"),
        fake(&mut asked),
        |_| {},
    ));
    assert_eq!(tree.cases.len(), 4);
    tree.apply(perform(
        &TestJob::RunAll,
        "calc",
        Path::new(r"C:\w"),
        fake(&mut asked),
        |_| {},
    ));
    let names: Vec<_> = tree.cases.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "a",
            "b",
            "math::tests::slow_overflow_check",
            "math::tests::adds_two_numbers",
            "math::tests::divides_by_ten"
        ]
    );
    let mut other = TestReport {
        targets: vec![tree.targets[0].clone()],
        ..Default::default()
    };
    other.lists.push((0, vec!["z".into()]));
    tree.apply(other);
    assert_eq!(tree.cases.len(), 1);
}

#[test]
fn the_panel_without_a_project_says_so_and_a_report_leaves_a_status() {
    let mut panel = TestPanel::default();
    let none = PanelEffect::Status("Tests: open a project first".into());
    assert_eq!(panel.open(None), none);
    assert_eq!(panel.input(PanelInput::RunAll, None), none);
    let mut asked = Vec::new();
    let report = perform(
        &TestJob::RunAll,
        "calc",
        Path::new(r"C:\w"),
        fake(&mut asked),
        |_| {},
    );
    panel.tree.package = "calc".into();
    let status = panel.take(report);
    assert_eq!(
        status,
        "Tests did not run: error[E0425]: cannot find value `x`"
    );
    let fine = TestReport {
        targets: panel.tree.targets.clone(),
        ..Default::default()
    };
    assert_eq!(panel.take(fine), "Failed 1 \u{00b7} 3 tests \u{00b7} 0.4 s");
}

#[test]
fn panel_inputs_toggle_chips_filter_and_go_to_a_test() {
    let mut panel = TestPanel::default();
    let mut asked = Vec::new();
    panel.tree.package = "calc".into();
    panel.take(perform(
        &TestJob::RunAll,
        "calc",
        Path::new(r"C:\w"),
        fake(&mut asked),
        |_| {},
    ));
    assert_eq!(panel.input(PanelInput::Chip(0), None), PanelEffect::Paint);
    assert!(!panel.view().0.0[0].1, "Passed hidden");
    panel.input(PanelInput::Chip(0), None);
    panel.input(PanelInput::Filter("divides".into()), None);
    assert_eq!(panel.view().1.len(), 3);
    match panel.input(PanelInput::Row(2), None) {
        PanelEffect::Go { place, output } => {
            assert_eq!(place.1, 10);
            assert!(output);
        }
        other => panic!("{other:?}"),
    }
    assert!(matches!(
        panel.input(PanelInput::Spot, None),
        PanelEffect::Go { output: false, .. }
    ));
    assert_eq!(
        panel.view().2.unwrap().1,
        "calc::math::tests::divides_by_ten"
    );
}
