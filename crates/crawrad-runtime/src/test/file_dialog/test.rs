//! Tests for the native file dialog helpers.

use super::*;

#[test]
fn parse_filter_standard_pairs() {
    let filters = parse_filter("Text files (*.txt)|*.txt;*.text|All files (*.*)|*.*");
    assert_eq!(filters.len(), 2);
    assert_eq!(filters[0].name, "Text files (*.txt)");
    assert_eq!(filters[0].patterns, vec!["txt", "text"]);
    assert_eq!(filters[1].name, "All files (*.*)");
    assert_eq!(filters[1].patterns, vec!["*"]);
}

#[test]
fn parse_filter_empty_input() {
    let filters = parse_filter("");
    assert!(filters.is_empty());

    let whitespace = parse_filter("   ");
    assert!(whitespace.is_empty());
}

#[test]
fn parse_filter_odd_count_drops_dangling_name() {
    let filters = parse_filter("Text files|*.txt|Dangling");
    assert_eq!(filters.len(), 1);
    assert_eq!(filters[0].name, "Text files");
    assert_eq!(filters[0].patterns, vec!["txt"]);
}

#[test]
fn parse_filter_patterns_with_dots_and_spaces() {
    let filters = parse_filter("Source| .rs ; *.toml |Images|*.png;*.jpg");
    assert_eq!(filters.len(), 2);
    assert_eq!(filters[0].patterns, vec!["rs", "toml"]);
    assert_eq!(filters[1].patterns, vec!["png", "jpg"]);
}

#[test]
fn with_default_ext_adds_extension_when_missing() {
    let p = with_default_ext(PathBuf::from("document"), "txt");
    assert_eq!(p, PathBuf::from("document.txt"));

    let p2 = with_default_ext(PathBuf::from("report"), ".pdf");
    assert_eq!(p2, PathBuf::from("report.pdf"));
}

#[test]
fn with_default_ext_preserves_existing_extension() {
    let p = with_default_ext(PathBuf::from("document.docx"), "txt");
    assert_eq!(p, PathBuf::from("document.docx"));
}

#[test]
fn with_default_ext_empty_default_does_nothing() {
    let p = with_default_ext(PathBuf::from("archive"), "");
    assert_eq!(p, PathBuf::from("archive"));

    let p2 = with_default_ext(PathBuf::from("archive"), ".");
    assert_eq!(p2, PathBuf::from("archive"));
}

#[test]
fn env_hook_pick_open_and_pick_save() {
    let test_answer = "first.txt|second.txt";

    let req_multi = FileDialogRequest {
        multi: true,
        ..Default::default()
    };
    let open_res = open_answer(test_answer, &req_multi);
    assert_eq!(
        open_res,
        Some(vec![
            PathBuf::from("first.txt"),
            PathBuf::from("second.txt")
        ])
    );

    let req_single = FileDialogRequest {
        multi: false,
        ..Default::default()
    };
    let single_res = open_answer(test_answer, &req_single);
    assert_eq!(single_res, Some(vec![PathBuf::from("first.txt")]));

    let req_save = FileDialogRequest {
        default_ext: "json".to_string(),
        ..Default::default()
    };
    let save_res = save_answer("output", &req_save);
    assert_eq!(save_res, Some(PathBuf::from("output.json")));

    // Test cancel via empty string
    assert_eq!(open_answer("", &req_multi), None);
    assert_eq!(save_answer("", &req_save), None);
}

/// A real PNG goes into the picture, with its name as the source.
#[test]
fn a_png_file_becomes_a_picture_named_after_it() {
    let dir = std::env::temp_dir().join(format!("picture-553-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let png = dir.join("logo.png");
    std::fs::write(&png, b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR").unwrap();
    let picture = picture_from_file(&png).expect("a PNG is a picture");
    assert_eq!(picture.format(), crawrad_ir::ImageFormat::Png);
    assert_eq!(picture.source(), Some("logo.png"));
    assert!(!picture.is_empty());

    // The bytes say what it is, not the extension: text named `.png` is refused.
    let fake = dir.join("fake.png");
    std::fs::write(&fake, b"just some text").unwrap();
    let why = picture_from_file(&fake).expect_err("text is not a picture");
    assert!(
        why.contains("fake.png"),
        "the refusal does not name the file: {why}"
    );

    let missing = picture_from_file(&dir.join("missing.png")).expect_err("no file, no picture");
    assert!(missing.contains("missing.png"), "{missing}");
    std::fs::remove_dir_all(&dir).ok();
}

/// An agent's `file` is read without a dialog opening.
#[test]
fn pick_picture_with_a_file_reads_it() {
    let dir = std::env::temp_dir().join(format!("pick-553-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let png = dir.join("a.png");
    std::fs::write(&png, b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR").unwrap();
    let got = pick_picture(Some(png.to_str().unwrap()));
    assert!(
        matches!(got, Some(Ok(ref p)) if p.source() == Some("a.png")),
        "{got:?}"
    );
    std::fs::remove_dir_all(&dir).ok();
}
