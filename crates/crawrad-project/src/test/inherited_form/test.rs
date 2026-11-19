use super::*;

fn dir(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!(
        "crawrad-inherited-form-{tag}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn write(d: &Path, doc: &Document) {
    let path = d.join(format!("{}.crawform", doc.root));
    std::fs::write(path, doc.to_json().unwrap()).unwrap();
}

fn project(tag: &str) -> PathBuf {
    let d = dir(tag);
    let mut parent = Document::new("form_parent", "Form");
    let mut btn = crawrad_ir::Node::new("btn_save", "Button");
    btn.events
        .insert("on_click".into(), "btn_save_click".into());
    parent.nodes.insert("btn_save".into(), btn);
    parent.nodes.insert(
        "lbl_status".into(),
        crawrad_ir::Node::new("lbl_status", "Label"),
    );
    if let Some(root) = parent.nodes.get_mut("form_parent") {
        root.children = vec!["btn_save".into(), "lbl_status".into()];
    }
    write(&d, &parent);
    write(
        &d,
        &Document::new_inherited("form_child", "Form", "form_parent"),
    );
    write(&d, &Document::new("form_about", "Form"));
    d
}

#[test]
fn a_derived_form_is_listed_under_its_base() {
    let d = project("list");
    let rows: Vec<(String, usize)> = forms_in(&d)
        .into_iter()
        .map(|f| (f.name, f.depth))
        .collect();
    assert_eq!(
        rows,
        [
            ("form_about".to_string(), 0),
            ("form_parent".to_string(), 0),
            ("form_child".to_string(), 1),
        ]
    );
}

#[test]
fn the_name_must_be_snake_case_and_free() {
    let d = project("name");
    let mut m = NewInheritedForm::new(&d, Some("form_parent"));
    assert_eq!(m.name, "form_1");
    assert!(m.can_create());
    m.name = "FormCustomer".into();
    assert!(m.name_problem().is_some_and(|w| w.contains("snake_case")));
    m.name = "form_child".into();
    assert!(m.name_problem().is_some_and(|w| w.contains("already")));
    assert!(!m.can_create());
}

#[test]
fn create_writes_a_form_that_inherits_the_picked_base() {
    let d = project("create");
    let mut m = NewInheritedForm::new(&d, Some("form_parent"));
    m.name = "form_customer".into();
    let path = m.create().unwrap();
    let doc = Document::load(&path).unwrap();
    assert_eq!(doc.inherits(), Some("form_parent"));
    assert_eq!(
        doc.nodes.len(),
        1,
        "only the root: the rest comes from the base"
    );
    let derived = crate::form_file::design_doc(&path, doc);
    assert!(derived.is_inherited("btn_save"));
}

#[test]
fn the_summary_counts_what_the_base_brings() {
    let d = project("summary");
    let m = NewInheritedForm::new(&d, Some("form_child"));
    let (line, names) = m.summary().unwrap();
    assert_eq!(line, "form_child \u{b7} 2 components \u{b7} 1 handler");
    assert_eq!(names, "btn_save, lbl_status");
    let (files, ty) = NewInheritedForm::new(&d, Some("form_parent")).files();
    assert!(files.contains("form_1.crawform") && files.ends_with("form_1.rs"));
    assert_eq!(ty, "struct Form1 { base: FormParent, \u{2026} }");
}

#[test]
fn the_filter_keeps_the_forms_whose_name_has_it() {
    let d = project("filter");
    let mut m = NewInheritedForm::new(&d, None);
    m.filter = "PAR".into();
    let names: Vec<&str> = m.rows().iter().map(|&i| m.forms[i].name.as_str()).collect();
    assert_eq!(names, ["form_parent"]);
}
