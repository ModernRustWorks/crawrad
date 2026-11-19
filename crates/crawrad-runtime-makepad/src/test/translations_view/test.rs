use super::*;

#[test]
fn chip_label_formatting() {
    let chip1 = LangChip {
        name: "Português (Brasil)".into(),
        done: 10,
        total: 14,
    };
    assert_eq!(chip_label(&chip1), "Português (Brasil) 10/14");

    let chip2 = LangChip {
        name: "Deutsch".into(),
        done: 14,
        total: 14,
    };
    assert_eq!(chip_label(&chip2), "Deutsch 14/14");
}

#[test]
fn cell_text_done_stale_and_missing() {
    let done = CellView::Done("Salvar".into());
    assert_eq!(cell_text(&done), "Salvar");

    let stale = CellView::Stale("Gravar".into());
    assert_eq!(cell_text(&stale), "Gravar");

    let missing = CellView::Missing;
    assert_eq!(cell_text(&missing), "");
}
