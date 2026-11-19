use super::*;

fn root() -> PathBuf {
    PathBuf::from(r"C:\work\calc")
}

#[test]
fn the_current_shape_gives_the_spot_and_the_message() {
    let err = "thread 'main' panicked at src\\form_main.rs:42:9:\n\
               attempt to divide by zero\n\
               note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n";
    let p = find_panics(err, &root());
    assert_eq!(p.len(), 1);
    assert_eq!(p[0].thread, "main");
    assert_eq!(p[0].message, "attempt to divide by zero");
    assert_eq!(p[0].spot.label(), "src\\form_main.rs:42:9");
    assert!(p[0].frames.is_empty());
}

#[test]
fn a_message_over_several_lines_is_kept_whole() {
    let err = "thread 'worker' panicked at calc/src/main.rs:7:5:\n\
               assertion `left == right` failed\n  left: 1\n right: 2\n\
               note: run with `RUST_BACKTRACE=1`\n";
    let p = &find_panics(err, &root())[0];
    assert_eq!(p.thread, "worker");
    assert_eq!(
        p.message,
        "assertion `left == right` failed\n  left: 1\n right: 2"
    );
    assert_eq!((p.spot.line, p.spot.column), (7, 5));
}

#[test]
fn the_older_shape_is_read_too() {
    let err = "thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 9', src/grid.rs:88:17\n";
    let p = &find_panics(err, &root())[0];
    assert_eq!(
        p.message,
        "index out of bounds: the len is 3 but the index is 9"
    );
    assert_eq!(p.spot.file, "src/grid.rs");
    assert_eq!(p.spot.line, 88);
}

#[test]
fn only_the_projects_frames_come_from_the_backtrace() {
    let err = "thread 'main' panicked at src/form_main.rs:42:9:\n\
attempt to divide by zero\n\
stack backtrace:\n\
   0: std::panicking::begin_panic_handler\n\
             at /rustc/abc123/library/std/src/panicking.rs:665:5\n\
   1: core::panicking::panic_const::panic_const_div_by_zero\n\
             at /rustc/abc123/library/core/src/panicking.rs:178:9\n\
   2: calc::form_main::FormMain::divide\n\
             at .\\src\\form_main.rs:42:9\n\
   3: makepad_widgets::button::Button::handle_event\n\
             at C:\\Users\\a\\.cargo\\registry\\src\\index.crates.io-1\\makepad-widgets-1.0.0\\src\\button.rs:300:1\n\
   4: calc::form_main::FormMain::on_click\n\
             at C:\\work\\calc\\src\\form_main.rs:30:13\n\
   5: main\n\
note: Some details are omitted.\n";
    let p = &find_panics(err, &root())[0];
    let names: Vec<&str> = p.frames.iter().map(|f| f.function.as_str()).collect();
    assert_eq!(
        names,
        [
            "calc::form_main::FormMain::divide",
            "calc::form_main::FormMain::on_click"
        ]
    );
    assert_eq!(p.frames[1].spot.line, 30);
}

#[test]
fn two_panics_are_two_sites_and_noise_is_ignored() {
    let err = "starting\nthread 'a' panicked at src/a.rs:1:1:\nboom\n\nthread 'b' panicked at src/b.rs:2:2:\nbang\nbye\n";
    let p = find_panics(err, &root());
    assert_eq!(p.len(), 2);
    assert_eq!(
        (p[0].message.as_str(), p[1].message.as_str()),
        ("boom", "bang\nbye")
    );
    assert!(find_panics("all good\nerror: not a panic at all\n", &root()).is_empty());
}

#[test]
fn a_spot_resolves_under_the_root_or_the_member() {
    let dir = std::env::temp_dir().join(format!("crawrad-panic-{}", std::process::id()));
    let member = dir.join("calc");
    std::fs::create_dir_all(member.join("src")).unwrap();
    std::fs::write(member.join("src").join("main.rs"), "fn main() {}").unwrap();
    let spot = |file: &str| SourceSpot {
        file: file.into(),
        line: 1,
        column: 1,
    };
    let in_member = member.join("src").join("main.rs");
    assert_eq!(
        spot("calc/src/main.rs").resolve(&dir, &member),
        Some(in_member.clone())
    );
    assert_eq!(
        spot("src/main.rs").resolve(&dir, &member),
        Some(in_member.clone())
    );
    assert_eq!(
        spot(&in_member.display().to_string()).resolve(&dir, &member),
        Some(in_member)
    );
    assert_eq!(spot("src/gone.rs").resolve(&dir, &member), None);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_headline_carries_the_first_line_and_the_spot() {
    let err = "thread 'main' panicked at src/a.rs:4:2:\nfirst\nsecond\n";
    assert_eq!(
        find_panics(err, &root())[0].headline("calc"),
        "calc panicked: first — src/a.rs:4:2"
    );
}

/// What a Makepad app really wrote (11/11/2026): the thread id after the name,
/// frames with no column, the designer's generated frames, and the echo panic
/// of an `extern` callback that cannot unwind.
const REAL: &str = include_str!("real_abort.txt");

#[test]
fn a_real_abort_gives_one_panic_and_only_the_users_frames() {
    let p = find_panics(REAL, &root());
    assert_eq!(p.len(), 1, "{p:#?}");
    assert_eq!(p[0].message, "attempt to divide by zero");
    assert_eq!((p[0].spot.line, p[0].spot.column), (13, 21));
    let names: Vec<&str> = p[0].frames.iter().map(|f| f.function.as_str()).collect();
    assert_eq!(
        names,
        ["meu_app::form_main_designer::FormMain::btn_ok_click"]
    );
    assert_eq!(
        (p[0].frames[0].spot.line, p[0].frames[0].spot.column),
        (13, 1)
    );
}
