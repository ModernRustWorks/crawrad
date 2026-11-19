use crate::{Adicao, crescido, migrar_handlers_renomeados};

#[test]
fn renaming_component_renames_handler_preserving_body_and_adds_no_second_fn() {
    let initial = r#"impl FormMain {
    /// `btn_ok.on_click`
    pub fn btn_ok_click(&mut self) {
        self.total += 1;
    }
}
"#;

    let additions = vec![Adicao {
        nome_da_fn: "btn_save_click".to_string(),
        texto: "\nimpl FormMain {\n    /// `btn_save.on_click`\n    pub fn btn_save_click(&mut self) {\n        \n    }\n}\n".to_string(),
        ancora: None,
    }];
    let neighbours: Vec<String> = Vec::new();

    let migrated = migrar_handlers_renomeados(initial, &additions, &neighbours, "");
    let final_text = crescido(&migrated, &additions);

    assert!(final_text.contains("/// `btn_save.on_click`"));
    assert!(final_text.contains("pub fn btn_save_click(&mut self)"));
    assert!(final_text.contains("self.total += 1;"));
    assert!(!final_text.contains("btn_ok"));

    let handlers = crate::elo::handlers_do_arquivo(&final_text);
    assert_eq!(handlers.len(), 1);
    assert_eq!(handlers[0].0, "btn_save_click");
}

#[test]
fn two_orphan_candidates_results_in_no_rename_and_new_stub_added() {
    let initial = r#"impl FormMain {
    /// `btn_a.on_click`
    pub fn btn_a_click(&mut self) {
        self.total += 1;
    }

    /// `btn_b.on_click`
    pub fn btn_b_click(&mut self) {
        self.total += 2;
    }
}
"#;

    let additions = vec![Adicao {
        nome_da_fn: "btn_save_click".to_string(),
        texto: "\nimpl FormMain {\n    /// `btn_save.on_click`\n    pub fn btn_save_click(&mut self) {\n        \n    }\n}\n".to_string(),
        ancora: None,
    }];
    let neighbours: Vec<String> = Vec::new();

    let migrated = migrar_handlers_renomeados(initial, &additions, &neighbours, "");
    assert_eq!(migrated, initial);

    let final_text = crescido(&migrated, &additions);
    assert!(final_text.contains("pub fn btn_a_click"));
    assert!(final_text.contains("pub fn btn_b_click"));
    assert!(final_text.contains("pub fn btn_save_click"));

    let handlers = crate::elo::handlers_do_arquivo(&final_text);
    assert_eq!(handlers.len(), 3);
}

#[test]
fn candidate_cited_in_neighbour_is_not_renamed() {
    let initial = r#"impl FormMain {
    /// `btn_ok.on_click`
    pub fn btn_ok_click(&mut self) {
        self.total += 1;
    }
}
"#;

    let additions = vec![Adicao {
        nome_da_fn: "btn_save_click".to_string(),
        texto: "\nimpl FormMain {\n    /// `btn_save.on_click`\n    pub fn btn_save_click(&mut self) {\n        \n    }\n}\n".to_string(),
        ancora: None,
    }];
    let neighbours = vec!["pub fn helper(&mut self) { self.btn_ok_click(); }".to_string()];

    let migrated = migrar_handlers_renomeados(initial, &additions, &neighbours, "");
    assert_eq!(migrated, initial);

    let final_text = crescido(&migrated, &additions);
    assert!(final_text.contains("pub fn btn_ok_click"));
    assert!(final_text.contains("pub fn btn_save_click"));

    let handlers = crate::elo::handlers_do_arquivo(&final_text);
    assert_eq!(handlers.len(), 2);
}

#[test]
fn legacy_named_candidate_is_renamed() {
    let initial = r#"impl FormMain {
    /// `btn_ok.on_click`
    pub fn btn_okClick(&mut self) {
        self.total += 1;
    }
}
"#;

    let additions = vec![Adicao {
        nome_da_fn: "btn_saveClick".to_string(),
        texto: "\nimpl FormMain {\n    /// `btn_save.on_click`\n    pub fn btn_saveClick(&mut self) {\n        \n    }\n}\n".to_string(),
        ancora: None,
    }];
    let neighbours: Vec<String> = Vec::new();

    let migrated = migrar_handlers_renomeados(initial, &additions, &neighbours, "");
    let final_text = crescido(&migrated, &additions);

    assert!(final_text.contains("/// `btn_save.on_click`"));
    assert!(final_text.contains("pub fn btn_saveClick(&mut self)"));
    assert!(final_text.contains("self.total += 1;"));
    assert!(!final_text.contains("btn_ok"));

    let handlers = crate::elo::handlers_do_arquivo(&final_text);
    assert_eq!(handlers.len(), 1);
    assert_eq!(handlers[0].0, "btn_saveClick");
}

#[test]
fn renaming_handler_keeps_body_under_new_name() {
    let initial = r#"impl FormMain {
    /// `btn_ok.on_click`
    pub fn btn_ok_click(&mut self) {
        self.total += 1;
    }

    pub fn helper(&mut self) {
        self.btn_ok_click();
    }
}
"#;

    let additions = vec![Adicao {
        nome_da_fn: "save_click".to_string(),
        texto: "\nimpl FormMain {\n    /// `btn_ok.on_click`\n    pub fn save_click(&mut self) {\n        \n    }\n}\n".to_string(),
        ancora: None,
    }];
    let neighbours: Vec<String> = Vec::new();

    let migrated = migrar_handlers_renomeados(initial, &additions, &neighbours, DESIGNER);
    let final_text = crescido(&migrated, &additions);

    assert!(final_text.contains("/// `btn_ok.on_click`"));
    assert!(final_text.contains("pub fn save_click(&mut self)"));
    assert!(final_text.contains("self.total += 1;"));
    assert!(final_text.contains("self.save_click();"));
    assert!(!final_text.contains("btn_ok_click"));

    let handlers = crate::elo::handlers_do_arquivo(&final_text);
    assert!(handlers.iter().any(|(f, _)| f == "save_click"));
}

#[test]
fn renamed_handler_cited_by_neighbour_is_not_moved() {
    let initial = r#"impl FormMain {
    /// `btn_ok.on_click`
    pub fn btn_ok_click(&mut self) {
        self.total += 1;
    }

    pub fn helper(&mut self) {
        self.btn_ok_click();
    }
}
"#;

    let additions = vec![Adicao {
        nome_da_fn: "save_click".to_string(),
        texto: "\nimpl FormMain {\n    /// `btn_ok.on_click`\n    pub fn save_click(&mut self) {\n        \n    }\n}\n".to_string(),
        ancora: None,
    }];
    let neighbours = vec!["pub fn helper(&mut self) { self.btn_ok_click(); }".to_string()];

    let migrated = migrar_handlers_renomeados(initial, &additions, &neighbours, DESIGNER);
    assert_eq!(migrated, initial);
}

/// The generated code as it was before `event.rename`: it called the old handler.
const DESIGNER: &str = "if clicked { self.form.btn_ok_click(); }";

/// **A handler the user renamed by hand is not taken back**: the generated
/// code never called `save_order`, so nothing says it was the bound one.
#[test]
fn handler_renamed_by_hand_is_not_moved() {
    let initial = r#"impl FormMain {
    /// `btn_ok.on_click`
    pub fn save_order(&mut self) {
        self.total += 1;
    }
}
"#;
    let additions = vec![Adicao {
        nome_da_fn: "btn_ok_click".to_string(),
        texto: "\nimpl FormMain {\n    /// `btn_ok.on_click`\n    pub fn btn_ok_click(&mut self) {\n        \n    }\n}\n".to_string(),
        ancora: None,
    }];
    let migrated = migrar_handlers_renomeados(initial, &additions, &[], DESIGNER);
    assert_eq!(migrated, initial);
}
