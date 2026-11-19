//! Unit tests for the Minimap (panoramic code overview).

use makepad_code_editor::minimap_model::{MinimapLine, TokenKind};

#[test]
fn minimap_rust_line_tokenization() {
    let line = "    pub struct Debt {";
    let parsed = MinimapLine::parse(line);
    assert_eq!(parsed.indent, 4);
    assert_eq!(parsed.tokens.len(), 3);
    assert_eq!(parsed.tokens[0].kind, TokenKind::Keyword); // pub
    assert_eq!(parsed.tokens[1].kind, TokenKind::Keyword); // struct
    assert_eq!(parsed.tokens[2].kind, TokenKind::Type); // Debt
}

#[test]
fn minimap_comment_tokenization() {
    let line = "    /// This is doc comment";
    let parsed = MinimapLine::parse(line);
    assert_eq!(parsed.indent, 4);
    assert_eq!(parsed.tokens.len(), 1);
    assert_eq!(parsed.tokens[0].kind, TokenKind::Comment);
}

#[test]
fn minimap_string_tokenization() {
    let line = "let msg = \"Hello CrawRAD\";";
    let parsed = MinimapLine::parse(line);
    assert_eq!(parsed.indent, 0);
    let tem_string = parsed.tokens.iter().any(|t| t.kind == TokenKind::String);
    assert!(
        tem_string,
        "string literal should be identified in the minimap"
    );
}

#[test]
fn minimap_function_tokenization() {
    let line = "    fn walk(index: &Index) {";
    let parsed = MinimapLine::parse(line);
    assert_eq!(parsed.indent, 4);
    assert_eq!(parsed.tokens[0].kind, TokenKind::Keyword); // fn
    assert_eq!(parsed.tokens[1].kind, TokenKind::Function); // walk
}

#[test]
fn minimap_scroll_and_click_sync() {
    use makepad_code_editor::minimap_model::MinimapMetrics;

    // Simulates an editor with 100 total lines, 20 visible (visible_h 400px / cell_h 20px), line_s 2.0px
    let metrics = MinimapMetrics::new(
        2.0,   // line_s
        0.0,   // minimap_scroll_y
        400.0, // visible_h
        20.0,  // cell_h
        100,   // total_lines
    );

    // Simulates a click at the middle of the minimap (rel_y = 50.0 with line_s = 2.0 -> line 25)
    // For 20 visible lines, centering should position the top line at 25 - 10 = 15 -> 15 * 20px = 300px
    let (target_y, clicked_line) = metrics.calcular_scroll_para_y(50.0);
    assert_eq!(target_y, 300.0);
    assert_eq!(clicked_line, 25);

    // Tests viewport slider position and height calculation
    let (slider_y, slider_h) = metrics.calcular_slider_y_e_altura(200.0, 600.0);
    // Line 10 (scroll 200px / 20px) * line_s 2.0 = 20px
    assert_eq!(slider_y, 20.0);
    // 20 visible lines * line_s 2.0 = 40px
    assert_eq!(slider_h, 40.0);
}

#[test]
fn minimap_presence_in_dsl_and_shell() {
    let casca = super::casca::CASCA;
    let paineis = include_str!("../../ide_app/panels/design.rs");
    assert!(
        paineis.contains("editor_minimap = <CrawradMinimap> {"),
        "editor_minimap must be declared in the panels DSL"
    );
    assert!(
        casca.contains("sincronizar_minimap"),
        "the sincronizar_minimap routine must be present in the IDE loop"
    );
}

#[test]
fn minimap_cargo_toml_utf8_tokenization() {
    let line1 = "# Este arquivo é SEU. O CrawRAD não o escreve nem o reescreve — quem o criou";
    let parsed1 = MinimapLine::parse(line1);
    assert_eq!(parsed1.indent, 0);
    assert_eq!(parsed1.tokens.len(), 1);
    assert_eq!(parsed1.tokens[0].kind, TokenKind::Comment);

    let line2 = "nome = \"configuração_açúcar_café\" # comentário utf-8";
    let parsed2 = MinimapLine::parse(line2);
    assert!(!parsed2.tokens.is_empty());
}
