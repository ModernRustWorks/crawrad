//! The hover card: which answer counts, where it goes, how much it shows.

use super::{Anchor, GAP, HoverCard, clip, markdown_height, place};

fn at(x: f64, y: f64) -> Anchor {
    Anchor {
        x,
        y,
        line_height: 18.0,
    }
}

/// **Only the newest request's answer opens the card, once.**
#[test]
fn only_the_newest_answer_counts() {
    let mut c = HoverCard::default();
    let old = c.ask(at(1.0, 1.0), false);
    let new = c.ask(at(9.0, 9.0), true);
    assert_eq!(c.answered(old), None);
    assert_eq!(c.answered(new), Some((at(9.0, 9.0), true)));
    assert_eq!(c.answered(new), None);
}

/// **Closing drops the awaited request and says whether a card was up.**
#[test]
fn closing_forgets_the_request() {
    let mut c = HoverCard::default();
    let t = c.ask(at(1.0, 1.0), false);
    assert!(!c.close(), "nothing was up");
    assert_eq!(c.answered(t), None, "the mouse left: the answer is late");
    c.shown_at(at(3.0, 4.0));
    assert_eq!(c.shown(), Some(at(3.0, 4.0)));
    assert!(c.close());
    assert_eq!(c.shown(), None);
}

/// **Above the line when it fits; below when not; never out of the window.**
#[test]
fn above_when_it_fits_else_below_and_inside() {
    let window = (0.0, 0.0, 1000.0, 800.0);
    let card = (500.0, 120.0);
    // room above
    assert_eq!(
        place(at(100.0, 400.0), card, window),
        (100.0, 400.0 - GAP - 120.0)
    );
    // no room above: below the line
    assert_eq!(
        place(at(100.0, 50.0), card, window),
        (100.0, 50.0 + 18.0 + GAP)
    );
    // near the right edge: pulled left to stay inside
    assert_eq!(place(at(900.0, 400.0), card, window).0, 500.0);
    // a card taller than the room below and above is kept inside the window
    let (_, y) = place(at(10.0, 20.0), (500.0, 790.0), window);
    assert!((0.0..=10.0).contains(&y), "{y}");
}

/// **A long card is cut, and says it.**
#[test]
fn long_text_is_clipped() {
    let text = (1..=40)
        .map(|i| format!("line {i}"))
        .collect::<Vec<_>>()
        .join("\n");
    let got = clip(&text, 30);
    assert_eq!(got.lines().count(), 31);
    assert!(got.ends_with("line 30\n\u{2026}"));
    assert_eq!(clip("a\nb", 30), "a\nb");
}

#[test]
fn an_open_card_sees_the_caret_move_once_and_a_closed_one_never() {
    let anchor = Anchor {
        x: 1.0,
        y: 2.0,
        line_height: 16.0,
    };
    let mut card = HoverCard::default();
    assert!(!card.caret_moved((3, 4)), "closed: never");
    card.shown_at(anchor);
    assert!(!card.caret_moved((3, 4)), "same place: no move");
    assert!(card.caret_moved((3, 5)), "a ) stepped over moves the caret");
    assert!(!card.caret_moved((3, 5)), "once per move");
}

#[test]
fn the_note_belongs_to_the_card_up_and_goes_with_it() {
    let mut card = HoverCard::default();
    let at = Anchor {
        x: 1.0,
        y: 2.0,
        line_height: 18.0,
    };
    card.set_note(Some(("add-two".into(), "src/lib.rs".into(), 3)));
    assert_eq!(card.note(), None, "no card up, no note");
    card.shown_at(at);
    assert_eq!(card.note().map(|n| n.0.as_str()), Some("add-two"));
    assert!(card.close());
    card.shown_at(at);
    assert_eq!(card.note(), None, "closing forgot it");
}

#[test]
fn the_rich_card_grows_with_its_markdown_and_stops_at_the_max() {
    assert_eq!(markdown_height("One line.", 10.0, 60, 300.0), 10.0);
    assert_eq!(
        markdown_height("```rust\nfn f()\n```", 10.0, 60, 300.0),
        50.0
    );
    assert_eq!(markdown_height(&"x".repeat(130), 10.0, 60, 300.0), 30.0);
    assert_eq!(
        markdown_height(&"line\n".repeat(100), 10.0, 60, 300.0),
        300.0
    );
}
