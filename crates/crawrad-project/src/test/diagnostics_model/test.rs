//! Diagnostics model tests.

use super::*;

const ERRORS: &str = include_str!("../../../testdata/cargo-erros.json");
const WARNINGS: &str = include_str!("../../../testdata/cargo-avisos.json");
const IN_GENERATED: &str = include_str!("../../../testdata/cargo-erro-no-gerado.json");
const EXTERNAL: &str = include_str!("../../../testdata/cargo-erro-externo.json");
const WITHOUT_SPAN: &str = include_str!("../../../testdata/cargo-erro-sem-span.json");

#[cfg(windows)]
fn workspace_group() -> PathBuf {
    PathBuf::from(r"C:\projetos\grupo")
}

#[cfg(not(windows))]
fn workspace_group() -> PathBuf {
    PathBuf::from("/projetos/grupo")
}

/// The fixture as this system's `cargo` would write it: with `/` outside Windows.
fn fx(fixture: &str) -> String {
    if cfg!(windows) {
        fixture.to_string()
    } else {
        fixture.replace("\\\\", "/").replace("\"C:/", "\"/")
    }
}

/// A path written with `/`, in this system's own spelling.
fn p(path: &str) -> PathBuf {
    if cfg!(windows) {
        PathBuf::from(path.replace('/', "\\"))
    } else {
        PathBuf::from(path)
    }
}

/// The separator that this system's `cargo` writes.
const SEP: &str = if cfg!(windows) { "\\" } else { "/" };

fn member() -> PathBuf {
    workspace_group().join("meu_app")
}

fn form_pair() -> (PathBuf, PathBuf) {
    let src = member().join("src");
    (src.join("form_main.rs"), src.join("form_main.designer.rs"))
}

fn location_of(output: &str, file_name: &str) -> DiagnosticLocation {
    parse_diagnostics(output)
        .into_iter()
        .find_map(|d| d.location.filter(|l| l.file.ends_with(file_name)))
        .unwrap_or_else(|| panic!("no diagnostic points to {file_name}"))
}

// ------------------------------------------------------------------- parser

#[test]
fn error_with_span_becomes_line_pointing_to_location() {
    let d = parse_diagnostics(&fx(ERRORS));
    assert_eq!(d.len(), 2, "two E0308 expected: {d:#?}");

    let first = &d[0];
    assert_eq!(first.severity, Severity::Error);
    assert_eq!(first.message, "mismatched types");
    let l = first
        .location
        .as_ref()
        .expect("error with span has location");
    assert_eq!(l.file, p("src/outro.rs"));
    assert_eq!((l.line, l.column), (2, 21));
}

#[test]
fn primary_span_takes_precedence_over_secondary() {
    assert!(
        ERRORS.contains(r#""column_start":12"#),
        "secondary span missing from fixture"
    );
    let l = location_of(&fx(ERRORS), "outro.rs");
    assert_eq!(l.column, 21);
}

#[test]
fn warning_is_read_with_file_and_line() {
    let d = parse_diagnostics(&fx(WARNINGS));
    assert_eq!(d.len(), 2, "two unused variables expected: {d:#?}");
    assert!(d.iter().all(|x| x.severity == Severity::Warning));
    let l = d[0]
        .location
        .as_ref()
        .expect("warning with span has location");
    assert_eq!(l.file, p("meu_app/src/form_main.rs"));
    assert_eq!((l.line, l.column), (2, 9));
}

#[test]
fn error_comes_before_warning() {
    let inverted = fx(IN_GENERATED)
        .lines()
        .rev()
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        first_severity(&inverted) == Some("warning"),
        "inversion did not put warning first"
    );

    for entry in [&fx(IN_GENERATED), &inverted] {
        let d = parse_diagnostics(entry);
        assert_eq!(d.len(), 2, "{d:#?}");
        assert_eq!(d[0].severity, Severity::Error);
        assert_eq!(d[1].severity, Severity::Warning);
    }
}

fn first_severity(output: &str) -> Option<&'static str> {
    output.lines().find_map(|l| {
        [r#""level":"error""#, r#""level":"warning""#]
            .iter()
            .zip(["error", "warning"])
            .find(|(tag, _)| l.contains(**tag))
            .map(|(_, name)| name)
    })
}

#[test]
fn message_without_span_appears_without_location() {
    let d = parse_diagnostics(&fx(WITHOUT_SPAN));
    assert_eq!(d.len(), 1, "{d:#?}");
    assert_eq!(d[0].severity, Severity::Error);
    assert!(d[0].message.starts_with("linking with"), "{}", d[0].message);
    assert_eq!(d[0].location, None);
}

#[test]
fn top_level_notes_and_help_are_excluded() {
    assert!(&fx(ERRORS).contains(r#""level":"failure-note""#));
    assert!(
        !parse_diagnostics(&fx(ERRORS))
            .iter()
            .any(|d| d.message.starts_with("For more information"))
    );
}

#[test]
fn only_compiler_message_line_is_included() {
    assert!(&fx(WARNINGS).contains(r#""reason":"compiler-artifact""#));
    assert!(&fx(WARNINGS).contains(r#""reason":"build-finished""#));
    assert_eq!(parse_diagnostics(&fx(WARNINGS)).len(), 2);

    let disguised = fx(WARNINGS).replace(
        r#""reason":"compiler-message""#,
        r#""reason":"build-script-executed""#,
    );
    assert!(
        parse_diagnostics(&disguised).is_empty(),
        "reason is ignored"
    );
}

#[test]
fn non_json_line_does_not_break_the_rest() {
    let dirty = format!(
        "   Compiling meu_app v0.1.0\n{}\nerror: could not compile\n",
        fx(WARNINGS)
    );
    assert_eq!(parse_diagnostics(&dirty).len(), 2);
}

#[test]
fn multiline_rendered_arrives_intact() {
    let d = parse_diagnostics(&fx(ERRORS));
    let r = &d[0].rendered;
    assert!(r.lines().count() > 4, "single line rendered: {r:?}");
    assert!(r.contains(&format!(" --> src{SEP}outro.rs:2:21")), "{r:?}");
    assert!(r.contains("^^"), "caret accent missing: {r:?}");
    assert!(
        r.contains("let s: String = 42;"),
        "code line missing: {r:?}"
    );
}

#[test]
fn summary_counts_existing_severities() {
    assert_eq!(
        summary(&parse_diagnostics(&fx(IN_GENERATED))),
        "1 error  ·  1 warning"
    );
    assert_eq!(summary(&parse_diagnostics(&fx(WARNINGS))), "2 warnings");
    assert_eq!(summary(&parse_diagnostics(&fx(WITHOUT_SPAN))), "1 error");
    assert_eq!(summary(&[]), "");
}

// ------------------------------------------------------------------ destination

#[test]
fn error_in_user_file_points_to_code_tab() {
    let (unit, designer) = form_pair();
    let l = location_of(&fx(IN_GENERATED), "form_main.rs");
    assert_eq!(
        destination(
            &l,
            &workspace_group(),
            &member(),
            Some(&unit),
            Some(&designer)
        ),
        Destination::Unit { line: 2, column: 9 }
    );
}

#[test]
fn error_in_generated_file_points_to_form_source() {
    let (unit, designer) = form_pair();
    let l = location_of(&fx(IN_GENERATED), "form_main.designer.rs");
    assert_eq!(
        destination(
            &l,
            &workspace_group(),
            &member(),
            Some(&unit),
            Some(&designer)
        ),
        Destination::FormSource {
            line: 3,
            column: 24
        }
    );
}

#[test]
fn other_member_file_is_relative_to_member() {
    let (unit, designer) = form_pair();
    let l = location_of(&fx(WARNINGS), "main.rs");
    assert_eq!(l.file, p("meu_app/src/main.rs"));
    assert_eq!(
        destination(
            &l,
            &workspace_group(),
            &member(),
            Some(&unit),
            Some(&designer)
        ),
        Destination::InMember {
            relative: p("src/main.rs"),
            line: 4,
        }
    );
}

#[test]
fn absolute_path_is_not_joined_with_root() {
    let l = location_of(&fx(EXTERNAL), "lib.rs");
    assert!(l.file.is_absolute(), "{:?}", l.file);
    let (unit, designer) = form_pair();
    assert_eq!(
        destination(
            &l,
            &workspace_group(),
            &member(),
            Some(&unit),
            Some(&designer)
        ),
        Destination::Outside {
            path: l.file.clone()
        }
    );
}

#[test]
fn without_open_form_no_file_is_paired() {
    let l = location_of(&fx(IN_GENERATED), "form_main.rs");
    assert_eq!(
        destination(&l, &workspace_group(), &member(), None, None),
        Destination::InMember {
            relative: p("src/form_main.rs"),
            line: 2,
        }
    );
}

#[test]
#[cfg(windows)]
fn different_casing_and_slashes_match_same_file() {
    let l = location_of(&fx(IN_GENERATED), "form_main.rs");
    let group_path = PathBuf::from(r"c:/PROJETOS/grupo");
    let member_path = PathBuf::from(r"C:\projetos\GRUPO\meu_app");
    let unit = PathBuf::from(r"c:\Projetos\grupo\meu_app/src/form_main.rs");
    let designer = PathBuf::from(r"C:\projetos\grupo\meu_app\src\form_main.designer.rs");
    assert_eq!(
        destination(&l, &group_path, &member_path, Some(&unit), Some(&designer)),
        Destination::Unit { line: 2, column: 9 }
    );
}

#[test]
fn extract_artifacts_from_real_cargo_fixture() {
    let arts = extract_artifacts(&fx(WARNINGS));
    assert!(
        !arts.is_empty(),
        "expected at least one artifact in WARNINGS"
    );
    let art = &arts[0];
    assert_eq!(art.package_name, "meu_app");
    assert!(art.executable.is_some() || !art.filenames.is_empty());
}

#[test]
fn format_file_size_formats_units_correctly() {
    assert_eq!(format_file_size(500), "500 B");
    assert_eq!(format_file_size(1024), "1.0 KB");
    assert_eq!(format_file_size(1024 * 1024 * 2 + 512 * 1024), "2.50 MB");
    assert_eq!(format_file_size(1024 * 1024 * 1024 * 3), "3.00 GB");
}

/// Without a server the build's diagnostics mark the open file only, from
/// their place, 0-based, errors told apart (moved this out of the shell).
#[test]
fn the_build_marks_only_the_named_file() {
    use super::{Diagnostic, DiagnosticLocation, Severity, marks_in};
    let d = |file: &str, line, column, severity| Diagnostic {
        severity,
        message: String::new(),
        location: Some(DiagnosticLocation {
            file: file.into(),
            line,
            column,
        }),
        rendered: String::new(),
    };
    let diags = [
        d("src/a.rs", 3, 5, Severity::Error),
        d("src/b.rs", 1, 1, Severity::Warning),
        d("src/a.rs", 1, 1, Severity::Warning),
    ];
    assert_eq!(marks_in(&diags, "a.rs"), vec![(2, 4, true), (0, 0, false)]);
    assert!(marks_in(&diags, "c.rs").is_empty());
}

#[test]
fn count_problems_or_diagnostics_lsp_and_fallback() {
    use super::count_problems_or_diagnostics;
    assert_eq!(
        count_problems_or_diagnostics(Some((3, 5)), None, "—"),
        ("3".to_string(), "5".to_string())
    );
    assert_eq!(
        count_problems_or_diagnostics(None, None, "—"),
        ("—".to_string(), "—".to_string())
    );
}

#[test]
fn note_marker_warnings_read_plainly() {
    let raw = |msg: &str| {
        format!(
            r#"{{"reason":"compiler-message","message":{{"message":{},"level":"warning","spans":[],"rendered":""}}}}"#,
            serde_json::to_string(msg).expect("json string")
        )
    };
    let style = raw("use of deprecated constant `f::NOTE_NOT_RUSTDOC_STYLE`: missing # Errors");
    let stale = raw("use of deprecated constant `f::NOTE_SIGNATURE_CHANGED`: signature changed");
    let other = raw("unused variable: `x`");
    let got: Vec<String> = [style, stale, other]
        .iter()
        .flat_map(|l| parse_diagnostics(l))
        .map(|d| d.message)
        .collect();
    assert_eq!(
        got,
        [
            "Doc note is not in rustdoc style: missing # Errors",
            "Note is out of date: signature changed",
            "unused variable: `x`",
        ]
    );
}
