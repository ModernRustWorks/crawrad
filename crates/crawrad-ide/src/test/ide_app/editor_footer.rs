//! The gate over the editor footer.

use super::{SLOTS, SOMENTE_LEITURA, file_encoding};

/// **The mode cell says `Read only`, and that is the truth about this view.**
///
/// The egui shell writes `Insert` there. Copying that word would be the bar
/// answering with the other shell's answer: the `CodeView` of this one is the
/// framework's editor with `read_only: true` hard-wired
/// (`makepad-code-editor-1.0.0/src/code_view.rs:22`), so every key is refused.
#[test]
fn the_mode_cell_tells_the_truth_about_a_read_only_view() {
    assert_eq!(SOMENTE_LEITURA, "Read only");
    assert_ne!(
        SOMENTE_LEITURA, "Insert",
        "the view rejects every key; saying `Insert` would be the bar lying"
    );
}

/// **The three cells write to three different places.**
///
/// The mistake this catches is the one made by copying the line above: two
/// values into one label means the second wins and the first is gone from the
/// screen, with nothing failing.
#[test]
fn the_three_cells_do_not_share_a_slot() {
    assert_eq!(SLOTS.len(), 6, "the editor footer has six cells");
    let fonte = include_str!("../../ide_app/editor/footer.rs");
    for nome in [
        "lbl_caret",
        "lbl_mode",
        "lbl_modified",
        "lbl_spaces",
        "lbl_encoding",
        "lbl_language",
    ] {
        assert_eq!(
            fonte.matches(nome).count(),
            1,
            "`{nome}` appears more than once in the module — two cells on the same \
             label, and the second erases the first"
        );
    }
}

/// **A file that is not on disk answers the generator's own encoding.**
///
/// It is the common case and it is not a fallback: `Alt+F12` shows the form
/// source PLANNED this instant, and the skeleton of a unit that was never
/// generated. Both are UTF-8 with `\n`, because that is what the generator
/// writes — so saying it is telling the truth about what is on screen.
#[test]
fn a_view_with_no_file_answers_what_the_generator_writes() {
    let fantasma = std::path::Path::new("E:/nao/existe/form_main.rs");
    assert_eq!(file_encoding(Some(fantasma)).label(), "UTF-8  LF");
    assert_eq!(file_encoding(None).label(), "UTF-8  LF");
}

/// **A real file on disk is read from its BYTES.**
#[test]
fn a_real_file_is_read_from_its_bytes() {
    let p = std::env::temp_dir().join(format!(
        "crawrad-rodape-{}-{:?}.rs",
        std::process::id(),
        std::thread::current().id()
    ));
    std::fs::write(&p, b"\xEF\xBB\xBFuma linha\r\noutra\r\n").expect("write fixture");
    let e = file_encoding(Some(&p));
    let _ = std::fs::remove_file(&p);
    assert_eq!(e.label(), "UTF-8 BOM  CRLF");
}
