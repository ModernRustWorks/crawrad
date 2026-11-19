//! The local list's order and the signatures it knows.

use super::{ItemKind, ItemSugestao, RowColours, known_detail, prefix_first};

/// **What starts with the typed text comes first; ties go by label.**
#[test]
fn prefix_first_then_by_label() {
    let mut items: Vec<ItemSugestao> = ["xs_len", "len", "b_len", "length"]
        .into_iter()
        .map(|l| ItemSugestao::novo(l, ItemKind::Method))
        .collect();
    prefix_first(&mut items, "len");
    let order: Vec<&str> = items.iter().map(|i| i.rotulo.as_str()).collect();
    assert_eq!(order, ["len", "length", "b_len", "xs_len"]);
}

#[test]
fn known_detail_only_for_known_names() {
    assert_eq!(known_detail("len"), Some("fn(&self) -> usize"));
    assert_eq!(known_detail("nonexistent"), None);
}

/// **The documentation panel:** the doc sent with the list shows at
/// once; an item without one is resolved under a new ticket; only the newest
/// ticket's answer is taken; the same item asks nothing; closing clears it.
#[test]
fn the_doc_panel_follows_the_selection() {
    use super::{DocPanel, DocStep};
    let mut with_doc = ItemSugestao::novo("len", ItemKind::Method);
    with_doc.doc = Some("Returns the length".into());
    let mut a = ItemSugestao::novo("push", ItemKind::Method);
    a.resolve = Some(serde_json::json!({ "hash": 1 }));
    let mut b = ItemSugestao::novo("pop", ItemKind::Method);
    b.resolve = Some(serde_json::json!({ "hash": 2 }));

    let mut panel = DocPanel::default();
    assert_eq!(panel.select(Some(&with_doc)), DocStep::Redraw);
    assert_eq!(panel.text.as_deref(), Some("Returns the length"));
    assert_eq!(panel.select(Some(&with_doc)), DocStep::Same);

    let DocStep::Ask(first, label, _) = panel.select(Some(&a)) else {
        panic!("push is resolved")
    };
    assert_eq!(label, "push");
    assert_eq!(panel.text, None, "nothing old meanwhile");
    let DocStep::Ask(second, _, data) = panel.select(Some(&b)) else {
        panic!("pop is resolved")
    };
    assert_eq!(data, serde_json::json!({ "hash": 2 }));
    assert!(
        !panel.answered(first, Some("push doc".into())),
        "an old ticket is dropped"
    );
    assert_eq!(panel.text, None);
    assert!(panel.answered(second, Some("pop doc".into())));
    assert_eq!(panel.text.as_deref(), Some("pop doc"));

    assert_eq!(panel.select(None), DocStep::Redraw);
    assert_eq!(panel.text, None);
}

/// **Typing is matched against `filterText`**: with snippets on,
/// rust-analyzer labels `len()` and `le(…)` but filters them as `len` and
/// `le`, so `le` puts `le` first and `len` before `leak()`, as before.
#[test]
fn typing_matches_the_filter_text_not_the_label() {
    let item = |label: &str, filter: &str| {
        let mut i = ItemSugestao::novo(label, ItemKind::Method);
        i.filter = Some(filter.to_string());
        i
    };
    let all = [
        item("leak()", "leak"),
        item("len()", "len"),
        item("le(…)", "le"),
    ];
    let shown: Vec<_> = super::filtered(&all, "le")
        .into_iter()
        .map(|i| i.rotulo)
        .collect();
    assert_eq!(shown[0], "le(…)", "exactly what was typed first");
    assert_eq!(
        super::filtered(&all, "len()").len(),
        0,
        "the label's `()` is not typed"
    );
}

/// **Each kind has its own icon colour; the pending row is a snippet's grey.**
#[test]
fn kinds_have_five_distinct_colours_and_pending_matches_snippet() {
    use ItemKind::*;
    let mut hexes: Vec<String> = [Field, Method, Type, Snippet, Keyword]
        .iter()
        .map(ItemKind::hex_colour)
        .collect();
    hexes.sort();
    hexes.dedup();
    assert_eq!(hexes.len(), 5);
    assert_eq!(Pending.colour(), Snippet.colour());
    assert_eq!(Method.hex_colour(), "#c39ac9");
    assert_eq!(
        Method.colour(),
        [195.0 / 255.0, 154.0 / 255.0, 201.0 / 255.0, 1.0]
    );
}

/// **Columns formats an item with and without origin and detail.**
#[test]
fn columns_with_and_without_origin_and_detail() {
    let mut plain = ItemSugestao::novo("len", ItemKind::Method);
    let cols = plain.columns();
    assert_eq!(cols.icon, ItemKind::Method.badge());
    assert_eq!(cols.label, "len");
    assert_eq!(cols.origin, "");
    assert_eq!(cols.signature, "");

    plain.origin = Some("(as ExactSizeIterator)".into());
    plain.detalhe = Some("fn(&self) -> usize".into());
    let cols_with = plain.columns();
    assert_eq!(cols_with.icon, ItemKind::Method.badge());
    assert_eq!(cols_with.label, "len");
    assert_eq!(cols_with.origin, "(as ExactSizeIterator)");
    assert_eq!(cols_with.signature, "fn(&self) -> usize");
}

/// **Fit keeps a short text, cuts a long one within room ending in `…`, and returns empty if nothing fits.**
#[test]
fn fit_keeps_short_cuts_long_within_room_or_empties() {
    let width = |s: &str| s.chars().count() as f64 * 7.0;
    assert_eq!(super::fit("short", 35.0, width), "short");
    assert_eq!(super::fit("hello world", 35.0, width), "hell…");
    assert_eq!(super::fit("anything", 5.0, width), "");
}

/// **The LSP mapping puts `labelDetails.detail` in origin and description in detalhe.**
#[test]
fn from_lsp_maps_origin_and_description() {
    let raw: crawrad_lsp::CompletionItem = serde_json::from_value(serde_json::json!({
        "label": "clone",
        "kind": 2,
        "labelDetails": {
            "detail": " (as Clone)",
            "description": "fn(&self) -> Self"
        }
    }))
    .unwrap();
    let mapped = super::from_lsp(vec![raw]);
    assert_eq!(mapped[0].origin.as_deref(), Some("(as Clone)"), "trimmed");
    assert_eq!(mapped[0].detalhe.as_deref(), Some("fn(&self) -> Self"));
}

/// **The signature gets the room the other columns leave, and the strike
/// covers the label only.**
#[test]
fn a_row_is_laid_out_in_its_width() {
    let width = |s: &str| s.chars().count() as f64 * 7.0;
    let mut it = ItemSugestao::novo("len", ItemKind::Method).com_detalhe("fn(&self) -> usize");
    it.origin = Some("(as X)".into());
    // 16 padding + 24 gaps + icon 7 + label 21 + origin 42 = 110; 35 left
    let laid = it.columns().layout(145.0, width);
    assert_eq!(laid.signature, "fn(&…");
    assert_eq!(laid.strike, (0.0, 0.0));
    it.deprecated = true;
    assert_eq!(it.columns().layout(400.0, width).strike, (23.0, 44.0));
    assert_eq!(
        it.columns().layout(400.0, width).signature,
        "fn(&self) -> usize"
    );
}

/// **The thumb shows the share of the list in view, and where: at the top,
/// at the bottom, none when everything fits.**
#[test]
fn the_thumb_follows_the_list() {
    assert_eq!(super::thumb(10, 10, 0, 22.0), (0.0, 0.0));
    assert_eq!(super::thumb(40, 10, 0, 22.0), (55.0, 0.0));
    assert_eq!(super::thumb(40, 10, 30, 22.0), (55.0, 165.0));
    assert_eq!(
        super::thumb(1000, 10, 0, 22.0).0,
        12.0,
        "never thinner than 12"
    );
    // the thicker thumb still ends at the track's end, never past it
    assert_eq!(super::thumb(1000, 10, 990, 22.0), (12.0, 208.0));
}

/// **On the selected row the origin and the signature turn white with the
/// label; elsewhere they are the soft grey.**
#[test]
fn the_selected_row_whitens_its_grey_columns() {
    let (on, off) = (RowColours::of(true), RowColours::of(false));
    assert_eq!(on.soft, on.text);
    assert_ne!(off.soft, off.text);
}

/// **An item whose only label detail is its origin has no signature: the
/// origin is not shown twice.**
#[test]
fn the_origin_alone_is_no_signature() {
    let raw: crawrad_lsp::CompletionItem = serde_json::from_value(serde_json::json!({
        "label": "into", "kind": 2, "labelDetails": { "detail": "(as Into)" }
    }))
    .unwrap();
    let mapped = &super::from_lsp(vec![raw])[0];
    assert_eq!(
        (mapped.origin.as_deref(), mapped.detalhe.as_deref()),
        (Some("(as Into)"), None)
    );
}

/// **The MCP reads each item's origin and icon colour.**
#[test]
fn the_list_json_carries_origin_and_colour() {
    let mut it = ItemSugestao::novo("clone", ItemKind::Method);
    it.origin = Some("(as Clone)".into());
    let list = crate::list::EditorComplete {
        sugestoes: vec![it],
        ..Default::default()
    };
    let item = &list.to_json()["items"][0];
    assert_eq!(
        (item["origin"].as_str(), item["colour"].as_str()),
        (Some("(as Clone)"), Some("#c39ac9"))
    );
}
