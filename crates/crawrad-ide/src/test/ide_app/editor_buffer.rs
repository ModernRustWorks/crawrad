//! The gate over what the editor may edit, and over the tab's dirty mark.
//!
//! The half that needs a window — `apply_over`, the poll, the write — has its
//! wire gate in `fios.rs`. What is measured here is the RULE, which is the part
//! that decides whether the user can type at all.

use std::path::{Path, PathBuf};

use super::pode_editar;
use crate::ide_app::source::Center;

fn unidade() -> PathBuf {
    PathBuf::from("E:/projeto/src/form_main.rs")
}

/// **A `.rs` from disk, with its tab in front, IS EDITABLE.**
///
/// This is the case reported by the owner: *"where is the Editor working?"* regarding
/// `form_main.rs`.
#[test]
fn a_file_from_disk_is_editable() {
    let p = unidade();
    assert_eq!(
        pode_editar(Center::Code, Some(&p), true),
        Some(p.clone()),
        "the user's file must accept keystrokes"
    );
}

/// **Form source is NOT editable, and that is not an oversight.**
///
/// `Alt+F12` shows the `.designer.rs` PLANNED at this instant from the
/// document (see [`crate::source`]). Typing in it would mean typing into something
/// that the next drag regenerates — work would disappear without anyone deleting anything.
#[test]
fn a_fonte_do_formulario_nao_e_editavel() {
    let p = unidade();
    assert_eq!(pode_editar(Center::FormSource, Some(&p), true), None);
    // And canvas even less: there is no text in front.
    assert_eq!(pode_editar(Center::Design, Some(&p), true), None);
}

/// **A unit that was never generated is NOT editable.**
///
/// What is on screen is the SKELETON that `source::user_unit` draws, not a
/// file. Allowing save would create the file through a path other than
/// `Generate` — and two creators for the same file is the race condition that ADR 0025
/// exists to prevent.
#[test]
fn uma_unidade_que_nao_existe_nao_e_editavel() {
    let p = unidade();
    assert_eq!(pode_editar(Center::Code, Some(&p), false), None);
}

/// **Without a file tab in front there is nothing to edit.**
#[test]
fn sem_aba_nao_ha_o_que_editar() {
    assert_eq!(pode_editar(Center::Code, None, true), None);
}

/// **The dirty mark lives where the X is**, as in a modern editor: the close button
/// has a `dirty` on its background (the dot) and on its glyph (the X hides
/// unless the mouse is on it). A circle drawn by the shader, not a font
/// glyph: nothing can turn it into an empty box.
#[test]
fn the_dirty_dot_takes_the_place_of_the_x() {
    let dsl = include_str!("../../ide_app/theme/workspace.rs");
    let close = &dsl[dsl
        .find("tab_close = <Button>")
        .expect("the tab's close button")..];
    let close = &close[..close.find("text_style").unwrap()];
    assert!(
        close.contains("instance dirty: 0.0"),
        "the close button lost its dirty state"
    );
    assert!(
        close.contains("uniform color_dot: (CRAWRAD_ACENTO)"),
        "the dot is the IDE's Rust orange"
    );
    assert!(close.contains("sdf.circle("), "the dot is drawn, not typed");
    assert!(
        close.contains("c.a * mix(1.0, self.hover, self.dirty)"),
        "the X hides while dirty and comes back under the mouse"
    );
}

/// **A relative path behaves the same.**
///
/// Explorer provides a path relative to the project folder, and the rule must not
/// depend on it being absolute.
#[test]
fn um_caminho_relativo_responde_igual() {
    let p = Path::new("src/main.rs");
    assert_eq!(
        pode_editar(Center::Code, Some(p), true),
        Some(p.to_path_buf())
    );
}
