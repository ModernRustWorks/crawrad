//! Unit tests for editor autocomplete and suggestions (Layer 4).

use crate::ide_app::editor_complete::EditorComplete;

#[test]
fn coletar_sugestoes_filtra_keywords_e_tipos() {
    let doc = "fn calcular_salario() { let mut valor = 100; }";

    // Prefix "st" should return "struct", "static", "str", "String"
    let sugs = EditorComplete::coletar_sugestoes("st", doc);
    assert!(sugs.contains(&"struct".to_string()));
    assert!(sugs.contains(&"String".to_string()));
    assert!(sugs.contains(&"static".to_string()));

    // Prefix "calc" should return the function from the document itself
    let sugs_calc = EditorComplete::coletar_sugestoes("calc", doc);
    assert!(sugs_calc.contains(&"calcular_salario".to_string()));

    // Prefix "But" should return CRAWRAD component "Button"
    let sugs_btn = EditorComplete::coletar_sugestoes("But", doc);
    assert!(sugs_btn.contains(&"Button".to_string()));
}

#[test]
fn coletar_sugestoes_com_prefixo_vazio_traz_itens_principais() {
    let doc = "let a = 1;";
    let sugs = EditorComplete::coletar_sugestoes("", doc);
    assert!(!sugs.is_empty());
    assert!(sugs.len() <= 16);
}

#[test]
fn collect_items_with_self_returns_form_fields_and_methods() {
    use crate::ide_app::editor_complete::ItemKind;

    let doc = r#"
impl FormMain {
    fn btn_ok_click(&mut self) {
        self.btn_ok.enabled = false;
        self.btn_cancel.text = "Desfazer".into();
    }
    fn btn_cancel_click(&mut self) {
        self.edt_nome_.text.clear();
    }
}
"#;

    let itens = EditorComplete::coletar_itens("", doc, Some("self"));
    let rotulos: Vec<&str> = itens.iter().map(|it| it.rotulo.as_str()).collect();

    // Should contain form components as in classic Code Insight
    assert!(rotulos.contains(&"edt_nome_"));
    assert!(rotulos.contains(&"btn_ok"));
    assert!(rotulos.contains(&"btn_cancel"));
    assert!(rotulos.contains(&"btn_ok_click"));
    assert!(rotulos.contains(&"btn_cancel_click"));

    // The kind must be Field or Method
    let edt = itens.iter().find(|it| it.rotulo == "edt_nome_").unwrap();
    assert_eq!(edt.kind, ItemKind::Field);

    let m_ok = itens.iter().find(|it| it.rotulo == "btn_ok_click").unwrap();
    assert_eq!(m_ok.kind, ItemKind::Method);
}
