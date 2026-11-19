//! The server's places, turned into the editor's file, line and byte.

use super::{CONTENT_MODIFIED, Outcome, Target, ask, targets};
use crate::encoding::PositionEncoding::{Utf8, Utf16};
use crate::protocol::{Location, Position, Range};
use std::path::Path;
use std::time::Duration;

fn loc(uri: &str, line: u32, character: u32) -> Location {
    let p = Position { line, character };
    Location {
        uri: uri.into(),
        range: Range { start: p, end: p },
    }
}

/// **The column becomes a byte in the target file's own line** — after
/// `"é🦀"` the UTF-16 column 19 and the UTF-8 column 22 are the same byte.
#[test]
fn the_column_becomes_a_byte_of_the_target_line() {
    let text = "fn a() {}\nlet s = \"é🦀\"; let total = 1;\n";
    let byte = text.lines().nth(1).unwrap().find("total").unwrap();
    let uri = crate::path_to_uri(Path::new("C:/p/src/lib.rs"));
    let mut read = |_: &Path| Some(text.to_string());
    for (enc, col) in [(Utf16, 19), (Utf8, 22)] {
        let got = targets(vec![loc(&uri, 1, col)], enc, &mut read);
        assert_eq!(got.len(), 1, "{enc:?}");
        assert_eq!((got[0].line, got[0].byte), (1, byte), "{enc:?}");
        assert_eq!(got[0].file, crate::uri_to_path(&uri).unwrap());
    }
}

/// **An unreadable file keeps the line, at its start; order is kept.**
#[test]
fn unreadable_file_lands_on_the_line_start_and_order_is_kept() {
    let a = crate::path_to_uri(Path::new("C:/p/a.rs"));
    let b = crate::path_to_uri(Path::new("C:/p/b.rs"));
    let mut read = |_: &Path| None;
    let got = targets(vec![loc(&a, 7, 9), loc(&b, 2, 4)], Utf8, &mut read);
    assert_eq!(
        got,
        vec![
            Target {
                file: crate::uri_to_path(&a).unwrap(),
                line: 7,
                byte: 0
            },
            Target {
                file: crate::uri_to_path(&b).unwrap(),
                line: 2,
                byte: 0
            },
        ]
    );
}

fn modified() -> crate::LspError {
    crate::LspError::RpcError {
        code: CONTENT_MODIFIED,
        message: "content modified".into(),
    }
}

/// **`ContentModified` is asked again, not shown** (LSP 3.17): two of them,
/// then the place, gives the place after three requests.
#[test]
fn content_modified_is_asked_again() {
    let uri = crate::path_to_uri(Path::new("C:/p/a.rs"));
    let mut answers = vec![Ok(vec![loc(&uri, 3, 0)]), Err(modified()), Err(modified())];
    let mut asked = 0;
    let mut request = |_| {
        asked += 1;
        answers.pop().unwrap()
    };
    let mut read = |_: &Path| None;
    let tick = Duration::from_millis(1);
    let got = ask(&mut request, Utf8, &mut read, Duration::from_secs(5), tick);
    assert_eq!(asked, 3);
    assert!(
        matches!(got, Outcome::Found(ref t) if t[0].line == 3),
        "{got:?}"
    );
}

/// **Out of time, the last `ContentModified` is the answer; other errors
/// and empty answers are not retried.**
#[test]
fn retries_stop_at_the_timeout_and_other_answers_stop_at_once() {
    let mut read = |_: &Path| None;
    let mut asked = 0;
    let mut always = |_| {
        asked += 1;
        Err(modified())
    };
    let got = ask(
        &mut always,
        Utf8,
        &mut read,
        Duration::from_millis(50),
        Duration::from_millis(20),
    );
    assert!(matches!(got, Outcome::NoAnswer(_)), "{got:?}");
    assert!((2..=3).contains(&asked), "asked {asked}");

    let mut asked = 0;
    let mut empty = |_| {
        asked += 1;
        Ok(vec![])
    };
    let got = ask(
        &mut empty,
        Utf8,
        &mut read,
        Duration::from_secs(5),
        Duration::from_millis(1),
    );
    assert_eq!((got, asked), (Outcome::Nothing, 1));

    let mut asked = 0;
    let mut timeout = |_| {
        asked += 1;
        Err(crate::LspError::Timeout)
    };
    let got = ask(
        &mut timeout,
        Utf8,
        &mut read,
        Duration::from_secs(5),
        Duration::from_millis(1),
    );
    assert!(
        matches!(got, Outcome::NoAnswer(_)) && asked == 1,
        "{got:?} {asked}"
    );
}
