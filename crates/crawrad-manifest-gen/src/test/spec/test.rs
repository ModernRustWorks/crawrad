use super::infer_kind;

/// Pieces that are not a property's type, each with why.
const NOT_A_PROPERTY: &[(&str, &str)] = &[
    // the layout keys are the designer's, never a field a component declares
    ("Anchors", "layout: the form owns it"),
    // the parts of other pieces, and a timing helper
    ("Key", "a part of `Shortcut`"),
    ("BusyTiming", "a timing helper, not a value"),
];

/// **Every piece `crawrad-component` exports has a kind here**.
///
/// This crate reads source text and has no dependency (ADR 0033 §3), so a
/// piece cannot declare its own kind to it: the closed `match` in
/// `infer_kind` stays, and this gate is what stops a new piece from being
/// forgotten there — a component declaring a field of that type would be
/// refused and told to write `kind = "…"` by hand.
#[test]
fn every_exported_piece_has_a_kind() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../crawrad-component/src/pieces/mod.rs"
    );
    let source = std::fs::read_to_string(path).expect("the pieces module");
    let mut pieces = Vec::new();
    for line in source.lines().filter(|l| l.starts_with("pub use ")) {
        let names = line
            .split_once('{')
            .map(|(_, rest)| rest.trim_end_matches("};").to_string())
            .unwrap_or_else(|| {
                line.rsplit("::")
                    .next()
                    .unwrap_or("")
                    .trim_end_matches(';')
                    .to_string()
            });
        for name in names.split(',') {
            let name = name.split(" as ").next().unwrap_or("").trim();
            if name.starts_with(|c: char| c.is_ascii_uppercase()) {
                pieces.push(name.to_string());
            }
        }
    }
    assert!(pieces.len() >= 10, "the pieces were not read: {pieces:?}");
    let missing: Vec<&String> = pieces
        .iter()
        .filter(|p| !NOT_A_PROPERTY.iter().any(|(n, _)| n == p))
        .filter(|p| infer_kind(&syn::parse_str(p).unwrap(), false).is_none())
        .collect();
    assert!(
        missing.is_empty(),
        "pieces with no kind in `infer_kind` (add the arm, or say why in NOT_A_PROPERTY): {missing:?}"
    );
}

/// **A margin field needs no `kind` and no `default`**: the
/// type says `edges`, and nothing said is zero on every side.
#[test]
fn an_edges_field_is_inferred_with_a_zero_default() {
    let source = "#[derive(Component)]\n\
                  #[crawrad(palette = \"P\", prefix = \"p\", size = (10.0, 10.0))]\n\
                  pub struct Framed {\n    \
                  #[prop(category = \"layout\")]\n    pub inset: Edges,\n}\n";
    let found = crate::from_source(source).expect("the file must read");
    let prop = found[0]
        .props
        .iter()
        .find(|p| p.key == "inset")
        .expect("the field is a property");
    assert_eq!(prop.kind, "edges");
    assert_eq!(
        prop.default_json,
        r#"{"left":0.0,"top":0.0,"right":0.0,"bottom":0.0}"#
    );
}
