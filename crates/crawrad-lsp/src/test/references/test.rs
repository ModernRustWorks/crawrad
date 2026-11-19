//! A references request: the places as they came, nothing, or no answer.

use super::{Outcome, ask};
use crate::definition::CONTENT_MODIFIED;
use crate::protocol::{Location, Position, Range};
use std::time::Duration;

fn loc(line: u32) -> Location {
    let p = Position { line, character: 3 };
    Location {
        uri: "file:///C:/p/src/lib.rs".into(),
        range: Range { start: p, end: p },
    }
}

/// **The places come back as the server sent them, in its order.**
#[test]
fn the_places_come_back_in_the_servers_order() {
    let places = vec![loc(9), loc(2), loc(5)];
    let sent = places.clone();
    let mut request = |_| Ok(sent.clone());
    let got = ask(&mut request, Duration::from_secs(1), Duration::ZERO);
    assert_eq!(got, Outcome::Found(places));
}

/// **An empty list is "nothing", not an empty find.**
#[test]
fn an_empty_list_is_nothing() {
    let mut request = |_| Ok(Vec::new());
    let got = ask(&mut request, Duration::from_secs(1), Duration::ZERO);
    assert_eq!(got, Outcome::Nothing);
}

/// **`ContentModified` is asked again; a real error is said.**
#[test]
fn content_modified_is_asked_again_and_an_error_is_said() {
    let mut calls = 0;
    let mut request = |_| {
        calls += 1;
        match calls {
            1 => Err(crate::LspError::RpcError {
                code: CONTENT_MODIFIED,
                message: "content modified".into(),
            }),
            _ => Ok(vec![loc(1)]),
        }
    };
    let got = ask(&mut request, Duration::from_secs(1), Duration::ZERO);
    assert_eq!(got, Outcome::Found(vec![loc(1)]));
    assert_eq!(calls, 2);

    let mut failing = |_| Err(crate::LspError::ChannelClosed);
    let got = ask(&mut failing, Duration::from_secs(1), Duration::ZERO);
    assert!(
        matches!(got, Outcome::NoAnswer(ref s) if s.contains("closed")),
        "{got:?}"
    );
}

fn range(uri: &str, line: u32, from: u32, to: (u32, u32)) -> Location {
    Location {
        uri: uri.into(),
        range: Range {
            start: Position {
                line,
                character: from,
            },
            end: Position {
                line: to.0,
                character: to.1,
            },
        },
    }
}

/// **The symbol's bytes in its own line, whatever the encoding** — after
/// `"é🦀"` the UTF-16 range 19..24 and the UTF-8 range 22..27 are the same
/// bytes, and they spell the name.
#[test]
fn the_range_becomes_the_symbols_bytes_in_its_line() {
    use crate::encoding::PositionEncoding::{Utf8, Utf16};
    use std::path::Path;
    let text = "fn a() {}\nlet s = \"é🦀\"; let total = 1;\n";
    let uri = crate::path_to_uri(Path::new("C:/p/src/lib.rs"));
    for (enc, from, to) in [(Utf16, 19, 24), (Utf8, 22, 27)] {
        let mut read = |_: &Path| Some(text.to_string());
        let got = super::spans(vec![range(&uri, 1, from, (1, to))], enc, &mut read);
        assert_eq!(got.len(), 1, "{enc:?}");
        let s = &got[0];
        assert_eq!(&s.text[s.start..s.end], "total", "{enc:?}");
        assert_eq!(
            (s.line, s.text.as_str()),
            (1, "let s = \"é🦀\"; let total = 1;")
        );
    }
}

/// **Each file is read once; a range past its line ends at the line's end;
/// an unreadable file keeps the line, empty; the order is the server's.**
#[test]
fn one_read_per_file_and_the_edges_hold() {
    use crate::encoding::PositionEncoding::Utf8;
    use std::path::Path;
    let a = crate::path_to_uri(Path::new("C:/p/a.rs"));
    let b = crate::path_to_uri(Path::new("C:/p/b.rs"));
    let mut reads = Vec::new();
    let mut read = |f: &Path| {
        reads.push(f.to_path_buf());
        f.to_string_lossy()
            .ends_with("a.rs")
            .then(|| "x\nfn long(\n".to_string())
    };
    let got = super::spans(
        vec![
            range(&a, 1, 3, (2, 0)),
            range(&b, 4, 2, (4, 5)),
            range(&a, 0, 0, (0, 1)),
        ],
        Utf8,
        &mut read,
    );
    assert_eq!(reads.len(), 2, "{reads:?}");
    let seen: Vec<_> = got
        .iter()
        .map(|s| (s.line, s.start, s.end, s.text.as_str()))
        .collect();
    assert_eq!(
        seen,
        vec![(1, 3, 8, "fn long("), (4, 0, 0, ""), (0, 0, 1, "x")]
    );
}
