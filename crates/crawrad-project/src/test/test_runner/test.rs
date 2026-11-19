use super::*;
use std::fs;

const LIB_RUN: &str = include_str!("lib_run.txt");
const DOC_RUN: &str = include_str!("doc_run.txt");

fn root() -> PathBuf {
    PathBuf::from(r"C:\work\calc")
}

fn lib() -> TestTarget {
    TestTarget {
        kind: TargetKind::Lib,
        source: "src/lib.rs".into(),
    }
}

fn doc() -> TestTarget {
    TestTarget {
        kind: TargetKind::Doc,
        source: "src/lib.rs".into(),
    }
}

#[test]
fn each_target_has_its_own_cargo_flag_and_one_test_is_exact() {
    let bin = TestTarget {
        kind: TargetKind::Bin("calc".into()),
        source: "src/main.rs".into(),
    };
    assert_eq!(
        lib().cargo_args("calc", &[]),
        ["test", "-p", "calc", "--lib", "--"]
    );
    assert_eq!(
        bin.cargo_args("calc", &["a::b".into(), "c".into()]),
        [
            "test", "-p", "calc", "--bin", "calc", "--", "a::b", "c", "--exact"
        ]
    );
    assert_eq!(doc().cargo_args("calc", &[])[3], "--doc");
}

#[test]
fn metadata_gives_lib_doc_bin_and_tests_relative_to_the_package() {
    let meta = r#"{"packages":[
        {"name":"other","manifest_path":"C:\\o\\Cargo.toml","targets":[{"kind":["lib"],"name":"other","src_path":"C:\\o\\src\\lib.rs"}]},
        {"name":"calc","manifest_path":"C:\\w\\calc\\Cargo.toml","targets":[
            {"kind":["test"],"name":"integration","src_path":"C:\\w\\calc\\tests\\integration.rs"},
            {"kind":["bin"],"name":"calc","src_path":"C:\\w\\calc\\src\\main.rs"},
            {"kind":["lib"],"name":"calc","src_path":"C:\\w\\calc\\src\\lib.rs","doctest":true},
            {"kind":["example"],"name":"demo","src_path":"C:\\w\\calc\\examples\\demo.rs"}]}]}"#;
    let t = targets(meta, "calc");
    let kinds: Vec<_> = t
        .iter()
        .map(|t| (t.kind.clone(), t.source.as_str()))
        .collect();
    assert_eq!(
        kinds,
        [
            (TargetKind::Lib, "src/lib.rs"),
            (TargetKind::Bin("calc".into()), "src/main.rs"),
            (
                TargetKind::Test("integration".into()),
                "tests/integration.rs"
            ),
            (TargetKind::Doc, "src/lib.rs"),
        ]
    );
    assert!(targets(meta, "nope").is_empty());
    assert!(targets("not json", "calc").is_empty());
}

#[test]
fn a_lib_without_doctests_has_no_doc_target() {
    let meta = r#"{"packages":[{"name":"c","manifest_path":"C:\\c\\Cargo.toml","targets":[
        {"kind":["rlib"],"name":"c","src_path":"C:\\c\\src\\lib.rs","doctest":false}]}]}"#;
    assert_eq!(targets(meta, "c"), [lib_of("src/lib.rs")]);
}

fn lib_of(source: &str) -> TestTarget {
    TestTarget {
        kind: TargetKind::Lib,
        source: source.into(),
    }
}

#[test]
fn the_list_is_the_lines_ending_in_test() {
    let out = "math::tests::adds_two_numbers: test\nsrc\\lib.rs - double (line 4): test\nbench_x: benchmark\n\n2 tests, 0 benchmarks\n";
    assert_eq!(
        parse_list(out),
        [
            "math::tests::adds_two_numbers",
            "src\\lib.rs - double (line 4)"
        ]
    );
}

#[test]
fn a_real_run_gives_outcomes_the_failure_spot_left_right_and_time() {
    let run = parse_run(LIB_RUN, &root());
    assert_eq!(
        run.outcomes,
        [
            (
                "math::tests::slow_overflow_check".to_string(),
                Outcome::Ignored
            ),
            ("math::tests::adds_two_numbers".to_string(), Outcome::Passed),
            ("math::tests::divides_by_ten".to_string(), Outcome::Failed),
        ]
    );
    let f = &run.failures["math::tests::divides_by_ten"];
    assert_eq!(f.summary, "left 3 \u{00b7} right 4");
    let spot = f.spot.as_ref().unwrap();
    assert_eq!(
        (spot.file.as_str(), spot.line, spot.column),
        ("src\\math.rs", 10, 9)
    );
    assert!(f.output.starts_with("thread 'math::tests::divides_by_ten'"));
    assert!(f.output.contains("right: 4") && !f.output.contains("failures:"));
    assert_eq!(run.seconds, Some(0.40));
}

#[test]
fn a_doctest_failure_points_at_its_own_line_not_the_temp_bundle() {
    let run = parse_run(DOC_RUN, &root());
    let f = &run.failures["src\\lib.rs - double (line 4)"];
    let spot = f.spot.as_ref().unwrap();
    assert_eq!((spot.file.as_str(), spot.line), ("src\\lib.rs", 4));
    assert_eq!(f.summary, "left 4 \u{00b7} right 5");
    assert_eq!(run.seconds, Some(0.02));
}

#[test]
fn a_panic_without_left_right_summarises_with_its_first_line() {
    let out = "test t ... FAILED\n\nfailures:\n\n---- t stdout ----\nthread 't' panicked at src\\a.rs:3:5:\nboom\nsecond\n\nfailures:\n    t\n";
    let run = parse_run(out, &root());
    assert_eq!(run.failures["t"].summary, "boom");
}

#[test]
fn a_unit_test_lives_in_its_deepest_module_file() {
    let dir = crate::fixture::folder("test-runner");
    fs::create_dir_all(dir.join("src/net")).unwrap();
    fs::write(dir.join("src/math.rs"), "").unwrap();
    fs::write(dir.join("src/net/mod.rs"), "").unwrap();
    fs::write(dir.join("src/net/http.rs"), "").unwrap();
    assert_eq!(
        test_file(&lib(), "math::tests::divides_by_ten", &dir),
        "src/math.rs"
    );
    assert_eq!(test_file(&lib(), "net::tests::x", &dir), "src/net/mod.rs");
    assert_eq!(
        test_file(&lib(), "net::http::tests::x", &dir),
        "src/net/http.rs"
    );
    assert_eq!(test_file(&lib(), "tests::at_root", &dir), "src/lib.rs");
    assert_eq!(
        test_file(&doc(), "src\\lib.rs - double (line 4)", &dir),
        "src/lib.rs"
    );
    let integ = TestTarget {
        kind: TargetKind::Test("integration".into()),
        source: "tests/integration.rs".into(),
    };
    assert_eq!(test_file(&integ, "adds", &dir), "tests/integration.rs");
    let _ = fs::remove_dir_all(&dir);
}

fn tree() -> TestTree {
    let mut t = TestTree {
        package: "calc".into(),
        root: root(),
        package_dir: root(),
        targets: vec![lib(), doc()],
        ..Default::default()
    };
    t.listed(
        0,
        &[
            "math::tests::adds_two_numbers".into(),
            "math::tests::divides_by_ten".into(),
            "math::tests::slow_overflow_check".into(),
        ],
    );
    t.listed(1, &["src\\lib.rs - double (line 4)".into()]);
    t
}

#[test]
fn a_listed_tree_is_not_run_and_a_run_fills_it() {
    let mut t = tree();
    assert_eq!(t.count(Outcome::NotRun), 4);
    assert_eq!(t.summary(), "4 tests");
    t.ran(0, parse_run(LIB_RUN, &root()));
    assert_eq!(
        (
            t.count(Outcome::Passed),
            t.count(Outcome::Failed),
            t.count(Outcome::Ignored)
        ),
        (1, 1, 1)
    );
    assert_eq!(t.chip(Outcome::Failed), ("Failed 1".to_string(), true));
    assert_eq!(t.summary(), "4 tests \u{00b7} 0.4 s");
    assert_eq!(
        t.failed(),
        [(0, vec!["math::tests::divides_by_ten".to_string()])]
    );
}

#[test]
fn a_rerun_forgets_the_old_failure() {
    let mut t = tree();
    t.ran(0, parse_run(LIB_RUN, &root()));
    let fixed =
        "test math::tests::divides_by_ten ... ok\n\ntest result: ok. 1 passed; finished in 0.10s\n";
    t.ran(0, parse_run(fixed, &root()));
    assert!(t.failed().is_empty());
    assert!(t.failure(1).is_none());
}

#[test]
fn the_tree_is_package_then_files_marked_by_their_worst_test() {
    let mut t = tree();
    t.ran(0, parse_run(LIB_RUN, &root()));
    let rows = t.rows();
    assert_eq!(
        rows[0],
        TreeRow::Package {
            label: "calc".into(),
            mark: Outcome::Failed,
            note: "1 failed".into()
        }
    );
    match &rows[1] {
        TreeRow::File {
            label,
            mark,
            note,
            open,
            ..
        } => {
            assert_eq!(
                (label.as_str(), *mark, note.as_str(), *open),
                ("src/lib.rs", Outcome::Failed, "0.4 s", true)
            );
        }
        other => panic!("{other:?}"),
    }
    let doc = rows.iter().find_map(|r| match r {
        TreeRow::File {
            target: 1,
            label,
            note,
            ..
        } => Some((label.clone(), note.clone())),
        _ => None,
    });
    assert_eq!(
        doc,
        Some(("Doc-tests (src/lib.rs)".to_string(), "1".to_string()))
    );
    let names: Vec<_> = rows
        .iter()
        .filter_map(|r| match r {
            TreeRow::Case { name, .. } => Some(name.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(
        names,
        [
            "adds_two_numbers",
            "divides_by_ten",
            "slow_overflow_check",
            "double (line 4)"
        ]
    );
}

#[test]
fn folding_and_chips_and_the_filter_hide_rows() {
    let mut t = tree();
    t.ran(0, parse_run(LIB_RUN, &root()));
    let file = t.rows()[1].clone();
    t.fold(&file);
    assert!(matches!(t.rows()[2], TreeRow::File { target: 1, .. }));
    t.fold(&file);
    t.toggle(Outcome::Passed);
    t.toggle(Outcome::NotRun);
    t.toggle(Outcome::Ignored);
    let rows = t.rows();
    assert_eq!(
        rows.len(),
        3,
        "only the failed test, its file and the package: {rows:?}"
    );
    assert!(!t.chip(Outcome::Passed).1);
    t.hidden.clear();
    t.filter = "DIVIDES".into();
    assert_eq!(t.rows().len(), 3);
    t.filter = "nothing".into();
    assert!(t.rows().is_empty());
}

#[test]
fn a_click_goes_to_the_assert_or_else_the_test_file_and_the_editor_is_marked() {
    let mut t = tree();
    t.ran(0, parse_run(LIB_RUN, &root()));
    t.ran(1, parse_run(DOC_RUN, &root()));
    assert_eq!(t.place(1), Some((root().join("src\\math.rs"), 10, 9)));
    assert_eq!(t.place(0), Some((root().join("src/lib.rs"), 1, 1)));
    assert_eq!(t.place(3), Some((root().join("src\\lib.rs"), 4, 1)));
    assert_eq!(
        t.marks(&root().join("src\\math.rs")),
        [(9, 8, "failed: left 3 \u{00b7} right 4".to_string())]
    );
    assert_eq!(t.marks(&root().join("src\\lib.rs")).len(), 1);
}

#[test]
fn the_view_rows_clicks_and_output_follow_the_tree() {
    let mut t = tree();
    t.ran(0, parse_run(LIB_RUN, &root()));
    let rows = t.view_rows();
    assert_eq!(
        rows[0],
        (
            0,
            0,
            "calc".to_string(),
            "1 failed".to_string(),
            true,
            false
        )
    );
    assert_eq!(
        rows[4],
        (
            2,
            2,
            "slow_overflow_check".to_string(),
            "ignored".to_string(),
            false,
            false
        )
    );
    let (chips, summary) = t.view_head(Some("Running src/lib.rs"));
    assert_eq!(chips[1], ("Failed 1".to_string(), true));
    assert_eq!(summary, "Running src/lib.rs\u{2026}");
    assert_eq!(t.click(3), Some((root().join(r"src\math.rs"), 10, 9)));
    assert!(t.view_rows()[3].5, "the picked test is highlighted");
    let (mark, name, place, lines, spot) = t.output().unwrap();
    assert_eq!(
        (mark, name.as_str(), place.as_str()),
        (
            0,
            "calc::math::tests::divides_by_ten",
            "in src/lib.rs \u{00b7} 0.4 s"
        )
    );
    assert!(lines[0].contains(&spot) && spot == r"src\math.rs:10:9");
    assert_eq!(
        t.run_for(3),
        Some(vec![(0, vec!["math::tests::divides_by_ten".to_string()])])
    );
    assert_eq!(t.run_for(1).unwrap()[0].1.len(), 3);
    assert_eq!(t.run_for(0), None);
    assert_eq!(t.click(1), None);
    assert!(!t.view_rows()[1].4, "a click folds the file");
}
