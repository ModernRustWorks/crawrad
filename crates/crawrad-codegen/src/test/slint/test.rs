use crate::Gerador;
use crate::targets::slint::GeradorSlint;
use crate::targets::slint::layout::{position, text};
use crate::targets::slint::main_app::main_rs;
use crawrad_ir::{Document, Node, Rect};

#[test]
fn slint_generator_emits_four_files() {
    let mut doc = Document::new("Form1", "Form");
    {
        let form = doc.node_mut("Form1").unwrap();
        form.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        form.set_name("Form1");
        form.set_text("My Slint Form");
    }

    let mut btn = Node::new("btn_ok", "Button");
    btn.set_name("btn_ok");
    btn.set_text("Confirm");
    btn.events
        .insert("on_click".to_string(), "btn_ok_click".to_string());
    let _ = doc.add_child("Form1", btn);

    let generator = GeradorSlint;
    let files = generator
        .gerar(&doc, "my_app", &crawrad_registry::Registry::builtin())
        .expect("must generate successfully");

    assert_eq!(files.len(), 4);
    let paths: Vec<_> = files.iter().map(|a| a.caminho.to_str().unwrap()).collect();
    assert!(paths.contains(&"src/main.rs"));
    assert!(paths.contains(&"src/Form1.designer.rs"));
    assert!(paths.contains(&"src/Form1.slint"));
    assert!(paths.contains(&"src/Form1.rs"));

    let main_rs_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/main.rs")
        .unwrap();
    assert!(
        main_rs_file
            .conteudo
            .contains("Form1_designer::app_main();")
    );

    let slint_file = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.slint")
        .unwrap();
    assert!(
        slint_file
            .conteudo
            .contains("export component Form1Window inherits Window")
    );
    assert!(slint_file.conteudo.contains("title: \"My Slint Form\";"));
    assert!(slint_file.conteudo.contains("width: 640.0px;"));
    assert!(slint_file.conteudo.contains("height: 480.0px;"));
    assert!(slint_file.conteudo.contains("btn_ok := Button"));
    assert!(slint_file.conteudo.contains("callback btn_ok_clicked();"));

    let designer_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.designer.rs")
        .unwrap();
    assert!(designer_rs.conteudo.contains("pub struct Form1"));
    assert!(designer_rs.conteudo.contains("slint::slint!"));
    assert!(designer_rs.conteudo.contains("pub fn app_main()"));
    assert!(designer_rs.conteudo.contains("window.on_btn_ok_clicked"));
    assert!(designer_rs.conteudo.contains("f.form.btn_ok_click();"));

    let user_rs = files
        .iter()
        .find(|a| a.caminho.to_str().unwrap() == "src/Form1.rs")
        .unwrap();
    assert!(user_rs.conteudo.contains("impl Form1"));
    assert!(user_rs.conteudo.contains("pub fn btn_ok_click(&mut self)"));
}

#[test]
fn test_slint_main_rs_formatting() {
    let code = main_rs("FormMain", "FormMain");
    assert!(code.contains("mod FormMain;"));
    assert!(code.contains("FormMain_designer::app_main();"));
}

#[test]
fn test_slint_layout_position_and_text() {
    assert_eq!(position(10.0, 25.5), "x: 10.0px; y: 25.5px;");

    let mut node = Node::new("btn", "Button");
    node.set_text("&Save File");
    assert_eq!(text(&node), "Save File");
}
