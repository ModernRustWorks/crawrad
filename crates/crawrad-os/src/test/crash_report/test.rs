//! The crash report, measured without a screen. Ported from the egui shell's
//! `falha_inicial` tests.

use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

/// A temporary folder for this proof only, removed at the end.
///
/// The name carries the process id and a counter: the suite runs tests in
/// parallel, and a fixed name would let two of them delete each other's folder.
struct Folder(PathBuf);

impl Folder {
    fn new(name: &str) -> Self {
        static N: AtomicU64 = AtomicU64::new(0);
        let p = std::env::temp_dir().join(format!(
            "crawrad-crash-report-{name}-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = std::fs::remove_dir_all(&p);
        std::fs::create_dir_all(&p).expect("create the test folder");
        Self(p)
    }
}

impl Drop for Folder {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A fabricated error: a panic cannot be caused on demand here, but the
/// reporter only asks for its text, and text can be fabricated.
const ERROR: &str = "NoGlutinConfigs(no available configs)";
const DETAILS: &str = "stack backtrace:\n   0: fabricated::frame";

#[test]
fn the_message_carries_the_error_text() {
    let folder = Folder::new("error-in-message");
    let r = prepare(ERROR, DETAILS, Some(&folder.0));
    assert!(
        r.message.contains(ERROR),
        "the message lost the error:\n{}",
        r.message
    );
}

#[test]
fn the_report_goes_to_disk_with_the_error_and_the_details() {
    // The box may not show (on Linux it is zenity, which may be missing). The
    // file is the path that does not depend on a screen.
    let folder = Folder::new("report-on-disk");
    let r = prepare(ERROR, DETAILS, Some(&folder.0));
    let path = r.file.as_ref().expect("the report had to have a path");
    assert_eq!(path, &folder.0.join(FILE_NAME), "the report went elsewhere");
    let text = std::fs::read_to_string(path).expect("read the report");
    assert!(text.contains(ERROR), "the report lacks the error:\n{text}");
    assert!(
        text.contains(DETAILS),
        "the report lacks the backtrace:\n{text}"
    );
    assert!(
        text.contains(TITLE),
        "the report does not say what it is:\n{text}"
    );
}

#[test]
fn the_box_points_at_the_file() {
    let folder = Folder::new("box-points");
    let r = prepare(ERROR, DETAILS, Some(&folder.0));
    let path = r.file.as_ref().expect("the report had to have a path");
    assert!(
        r.message.contains(&path.display().to_string()),
        "the message does not say where the report is:\n{}",
        r.message
    );
}

#[test]
fn with_no_writable_folder_the_message_is_still_whole() {
    // The reporter must not panic nor return an empty message when writing
    // fails: a crash here is the silence coming back.
    let folder = Folder::new("no-write");
    // A file where a folder should be: `create_dir_all` and `write` refuse, on
    // any system.
    let blocked = folder.0.join("this-is-a-file");
    std::fs::write(&blocked, b"x").expect("create the blocker");
    let r = prepare(ERROR, DETAILS, Some(&blocked));
    assert_eq!(
        r.file, None,
        "it wrote where it should not have been able to"
    );
    assert!(r.message.contains(ERROR), "the message lost the error");
    assert!(
        r.message.contains("could not be written"),
        "the message does not warn there was no file:\n{}",
        r.message
    );
    assert_ne!(r.code, 0, "it failed and exited as a success");
}

#[test]
fn with_no_folder_at_all_there_is_still_a_message() {
    let r = prepare(ERROR, DETAILS, None);
    assert_eq!(r.file, None);
    assert!(r.message.contains(ERROR));
    assert_ne!(r.code, 0);
}

#[test]
fn the_message_says_what_to_do_without_apologising() {
    let r = prepare(ERROR, DETAILS, None);
    assert!(
        r.message.contains("What to do:"),
        "the message describes the problem and does not say what to do:\n{}",
        r.message
    );
    for apology in ["sorry", "Sorry", "unfortunately", "Unfortunately"] {
        assert!(
            !r.message.contains(apology),
            "the message became an apology ({apology})"
        );
    }
}

#[test]
fn the_report_has_what_support_asks_for() {
    let folder = Folder::new("report-complete");
    let r = prepare(ERROR, DETAILS, Some(&folder.0));
    let text = std::fs::read_to_string(r.file.unwrap()).expect("read the report");
    for piece in ["Unix time:", "Version:", "Target:", "Executable:"] {
        assert!(text.contains(piece), "`{piece}` is missing from the report");
    }
}

#[test]
fn the_second_failure_rewrites_the_report_instead_of_stacking() {
    // The file has to be exactly what the box just described.
    let folder = Folder::new("rewrites");
    prepare("FIRST FAILURE", DETAILS, Some(&folder.0));
    let r = prepare("SECOND FAILURE", DETAILS, Some(&folder.0));
    let text = std::fs::read_to_string(r.file.unwrap()).expect("read the report");
    assert!(text.contains("SECOND FAILURE"));
    assert!(
        !text.contains("FIRST FAILURE"),
        "the report stacked, and the failure of now is not the first one read"
    );
}

#[test]
fn the_configuration_folder_is_the_first_choice() {
    let chosen = PathBuf::from("/a/configuration/folder");
    assert_eq!(
        choose_folder(Some(chosen.clone())),
        Some(chosen),
        "the report stopped going to the user's configuration folder"
    );
}

#[test]
fn without_a_configuration_folder_the_report_falls_back_to_temp() {
    assert_eq!(
        choose_folder(None),
        Some(std::env::temp_dir()),
        "without a configuration folder the report would have nowhere to go"
    );
}

#[test]
fn there_is_a_folder_for_the_report_on_this_machine() {
    // Without a configuration folder the chain still answers (the temp dir).
    assert!(
        report_folder(None).is_some(),
        "no folder accepted the report, and the failure would be mute again"
    );
}
