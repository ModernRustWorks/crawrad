//! What can be measured of the PALETTE **without a window**.
//!
//! The owner rejected this canvas looking at it next to the RXP, and the
//! diagnosis was *"material contrast"*: in egui every kind of control has a
//! value and a border of its own, and here *"everything is the same dark
//! rectangle, with no border"*. What these tests demand is not aesthetics — it
//! is the **relationship between the tokens**, which is exactly what flattens
//! out when somebody touches one colour on its own.
//!
//! They mirror the RXP's (`crawrad-runtime-egui/src/tests.rs:2500-2546`), and the
//! ruler is the same: 4.5:1 for text and 3:1 for the outline that identifies a
//! control (the WCAG floor). They do not mirror the VALUE from there, and they
//! cannot — the RXP's paper is light and this one's is dark. They mirror the
//! ROLE each colour plays, which is what ADR 0013 demands.

use super::*;

/// The WCAG contrast ratio between two opaque colours.
///
/// The same computation as the RXP's (`crawrad-runtime-egui/src/tests.rs:2502`),
/// with the input in `[f32; 4]` because that is how a token lives here.
fn contrast(a: [f32; 4], b: [f32; 4]) -> f32 {
    fn lum(c: [f32; 4]) -> f32 {
        fn channel(v: f32) -> f32 {
            if v <= 0.04045 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        }
        0.2126 * channel(c[0]) + 0.7152 * channel(c[1]) + 0.0722 * channel(c[2])
    }
    let (x, y) = (lum(a), lum(b));
    let (light, dark) = if x > y { (x, y) } else { (y, x) };
    (light + 0.05) / (dark + 0.05)
}

/// A token's relative luminance — to compare who is lighter.
fn luminance(c: [f32; 4]) -> f32 {
    // Reuses the contrast computation against absolute black: monotonic, and
    // therefore enough to sort by.
    contrast(c, [0.0, 0.0, 0.0, 1.0])
}

// ---------------------------------------------------------------------------
// The value hierarchy — item 3 on the owner's list
// ---------------------------------------------------------------------------

#[test]
fn the_button_rises_and_the_field_sinks_against_the_paper() {
    // *"In egui the button is lighter than the background (it rises) and the
    // text field is darker (it sinks). In Makepad the two are the same."* It is
    // an interface convention since Windows 95 and a classic RAD IDE uses it — and it is
    // what makes a form readable WITHOUT reading the text.
    //
    // A bare `>` would pass with a one-bit step. The minimum step is what the
    // eye reads: 10% of contrast to each side.
    assert!(
        luminance(MAKEPAD_WIDGET) >= luminance(MAKEPAD_PAPER) * 1.10,
        "the button face ({:.4}) has to RISE above the paper ({:.4})",
        luminance(MAKEPAD_WIDGET),
        luminance(MAKEPAD_PAPER)
    );
    assert!(
        luminance(MAKEPAD_WIDGET_INPUT) <= luminance(MAKEPAD_PAPER) * 0.90,
        "the field ({:.4}) has to SINK below the paper ({:.4})",
        luminance(MAKEPAD_WIDGET_INPUT),
        luminance(MAKEPAD_PAPER)
    );
}

#[test]
fn the_container_is_not_the_button_face() {
    // The owner circled three containers in the print and wrote "Scroll???": he
    // could not tell a `ScrollBox` from a `Panel` from a `GroupBox`. Part of the
    // cause was this — the container surface and the button face were the SAME
    // token, so a panel and a button came out identical.
    assert_ne!(MAKEPAD_PANEL, MAKEPAD_WIDGET);
    assert_ne!(MAKEPAD_PANEL, MAKEPAD_PAPER);
    // And the container has to be visible against the paper — it is the reason
    // the RXP has a `PAINEL` of its own
    // (`crawrad-runtime-egui/src/lib.rs:172-177`).
    assert!(
        contrast(MAKEPAD_PANEL, MAKEPAD_PAPER) > 1.0,
        "the container vanished into the paper"
    );
}

// ---------------------------------------------------------------------------
// The border — item 1, which on its own solves half of it
// ---------------------------------------------------------------------------

#[test]
fn the_outline_says_where_the_control_is() {
    // The WCAG asks for 3:1 on the outline that identifies a control. This is
    // the test that rejects the old `#45475a`: it measured 1.5:1 against the
    // paper, that is, the form read as *"a set of holes"*.
    for (label, background) in [
        ("paper", MAKEPAD_PAPER),
        ("field", MAKEPAD_WIDGET_INPUT),
        ("face", MAKEPAD_WIDGET),
    ] {
        let r = contrast(MAKEPAD_BORDER, background);
        assert!(r >= 3.0, "control border over {label}: {r:.2}:1");
    }
    // The container frame is more discreet on purpose — it may not compete with
    // the outline of whatever is INSIDE it — and the floor is the same one the
    // RXP adopts for its own pair (`crawrad-runtime-egui/src/tests.rs:2545`).
    assert!(contrast(MAKEPAD_PANEL_BORDER, MAKEPAD_PANEL) >= 2.0);
    assert!(contrast(MAKEPAD_PANEL_BORDER, MAKEPAD_PAPER) >= 2.0);
}

#[test]
fn the_form_text_reads_over_any_surface() {
    for (label, background) in [
        ("paper", MAKEPAD_PAPER),
        ("field", MAKEPAD_WIDGET_INPUT),
        ("face", MAKEPAD_WIDGET),
        ("container", MAKEPAD_PANEL),
        ("selection", MAKEPAD_SELECTION),
    ] {
        let r = contrast(MAKEPAD_TEXT, background);
        assert!(r >= 4.5, "form text over {label}: {r:.2}:1");
    }
    // Disabled is weaker on purpose, but it still has to be readable.
    let r = contrast(MAKEPAD_TEXT_DIM, MAKEPAD_PAPER);
    assert!(r >= 4.5, "disabled text over paper: {r:.2}:1");
}

#[test]
fn the_grid_reads_like_a_drawing_board() {
    // *"The grid is far too weak. In egui the dots read like a drawing board; in
    // Makepad they almost disappear."* The old `#585b70` measured 2.1:1 against
    // the paper.
    let r = contrast(MAKEPAD_GRID, MAKEPAD_PAPER);
    assert!(r >= 3.0, "grid dot over the paper: {r:.2}:1");
    // And it may not compete with the outline of the controls: what pulls the
    // eye is the drawing, not its frame.
    assert!(luminance(MAKEPAD_GRID) < luminance(MAKEPAD_BORDER));
}

// ---------------------------------------------------------------------------
// The accelerator — the classic RAD `&`
// ---------------------------------------------------------------------------

#[test]
fn the_ampersand_marks_the_key_and_is_not_drawn() {
    // The owner saw **`&OK`** written on a button. The Win32 contract is: `&`
    // marks the access key and does not reach the screen; `&&` is a literal `&`.
    assert_eq!(sem_acelerador("&OK"), "OK");
    assert_eq!(sem_acelerador("S&ave"), "Save");
    assert_eq!(sem_acelerador("Ma&&Pa"), "Ma&Pa");
    assert_eq!(sem_acelerador("no mark"), "no mark");
    // A `&` at the end has nothing to mark, and disappears all the same.
    assert_eq!(sem_acelerador("end&"), "end");
}

#[test]
fn the_underline_lands_on_the_character_the_ampersand_marked() {
    // The index is in BYTES of the ALREADY CLEANED text — that is how whoever
    // draws slices the `&str` to measure the prefix.
    assert_eq!(indice_do_acelerador("&OK"), Some(0));
    assert_eq!(indice_do_acelerador("S&ave"), Some(1));
    // `&&` marks nothing, and the literal `&` it leaves behind counts one byte.
    assert_eq!(indice_do_acelerador("Ma&&Pa"), None);
    assert_eq!(indice_do_acelerador("Ma&&P&a"), Some(4));
    assert_eq!(indice_do_acelerador("no mark"), None);
    assert_eq!(indice_do_acelerador("end&"), None);
    // An accented character before the mark: the index has to walk in bytes, and
    // `ç` is two of them. An index in characters would put the underline in the
    // wrong place — and slicing a `&str` by it would panic in mid-frame.
    let t = "Endereço &Novo";
    let i = indice_do_acelerador(t).expect("there is a mark");
    let clean = sem_acelerador(t);
    assert_eq!(&clean[i..i + 1], "N");
}

#[test]
fn the_light_page_turns_the_form_roles_and_leaves_the_designer_marks() {
    // The web page is white, its text dark, and what the designer
    // draws over it (accent, handles, the ruler) keeps its colour.
    assert_eq!(light_page_ink(MAKEPAD_PAPER), rgb(0xFF, 0xFF, 0xFF));
    let paper = light_page_ink(MAKEPAD_PAPER);
    assert!(contrast(light_page_ink(MAKEPAD_TEXT), paper) >= 7.0);
    assert!(contrast(light_page_ink(MAKEPAD_TEXT), light_page_ink(MAKEPAD_WIDGET)) >= 7.0);
    assert!(luminance(light_page_ink(MAKEPAD_WIDGET)) < luminance(paper));
    for mark in [
        MAKEPAD_ACCENT,
        MAKEPAD_HANDLE,
        MAKEPAD_REGUA_FUNDO,
        MAKEPAD_BG,
    ] {
        assert_eq!(light_page_ink(mark), mark);
    }
}
