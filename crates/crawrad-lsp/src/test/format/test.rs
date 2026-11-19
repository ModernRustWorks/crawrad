//! Format's edits on the text: changed, or "already formatted".

use super::{Outcome, Verdict, formatted, refusal, verdict};
use crate::edit::TextEdit;
use crate::encoding::PositionEncoding::Utf8;
use crate::protocol::{Position, Range};

fn edit(l0: u32, c0: u32, l1: u32, c1: u32, new_text: &str) -> TextEdit {
    TextEdit {
        range: Range {
            start: Position {
                line: l0,
                character: c0,
            },
            end: Position {
                line: l1,
                character: c1,
            },
        },
        new_text: new_text.into(),
    }
}

/// **The edits make the formatted text**, several on one line included.
#[test]
fn the_edits_make_the_formatted_text() {
    let ugly = "fn main(){\nlet x=1;\n}\n";
    let edits = [
        edit(0, 9, 0, 9, " "),
        edit(1, 0, 1, 0, "    "),
        edit(1, 5, 1, 6, " = "),
    ];
    assert_eq!(
        formatted(ugly, &edits, Utf8).as_deref(),
        Some("fn main() {\n    let x = 1;\n}\n")
    );
}

/// **No edits, or edits that change nothing, are "already formatted"** —
/// the file must not turn dirty.
#[test]
fn nothing_to_change_is_none() {
    let text = "fn main() {}\n";
    assert_eq!(formatted(text, &[], Utf8), None);
    assert_eq!(formatted(text, &[edit(0, 3, 0, 7, "main")], Utf8), None);
}

/// **The verdict:** a changed file drops the answer; no answer is said;
/// nothing to change is "already formatted"; otherwise the new text.
#[test]
fn the_verdict_on_an_answer() {
    let ugly = "fn main(){}\n";
    let fix = || Outcome::Edits(vec![edit(0, 9, 0, 9, " ")]);
    assert_eq!(
        verdict(fix(), false, ugly, Utf8),
        Verdict::Say("The file changed while formatting; format again")
    );
    assert_eq!(
        verdict(Outcome::NoAnswer("t".into()), true, ugly, Utf8),
        Verdict::Say("Language server did not answer")
    );
    assert_eq!(
        verdict(Outcome::Edits(vec![]), true, ugly, Utf8),
        Verdict::Say("Already formatted")
    );
    assert_eq!(
        verdict(fix(), true, ugly, Utf8),
        Verdict::Apply("fn main() {}\n".into())
    );
}

/// **The generated form source is refused with its own reason.**
#[test]
fn the_refusal_names_the_generated_source() {
    let p = std::path::Path::new("C:/p/src/form_main.designer.rs");
    assert!(refusal(Some(p), ".designer.rs").contains("generator"));
    assert!(
        refusal(Some(std::path::Path::new("C:/x/string.rs")), ".designer.rs")
            .contains("takes typing")
    );
    assert!(refusal(None, ".designer.rs").contains("takes typing"));
}
