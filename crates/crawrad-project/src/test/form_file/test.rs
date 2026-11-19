use super::{path_for, save};
use crawrad_ir::Document;
use std::path::Path;

fn form_named(name: &str) -> Document {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main").expect("the root").set_name(name);
    d
}

fn folder(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("crawrad-form-file-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp folder");
    dir
}

#[test]
fn a_renamed_form_is_saved_under_its_new_name_and_the_old_file_goes() {
    let dir = folder("rename");
    let old = dir.join("form_main.crawform");
    form_named("form_main").save(&old).expect("first save");

    let written = save(&form_named("frmCustomer"), &old).expect("save");

    assert_eq!(written, dir.join("frmCustomer.crawform"));
    assert!(written.exists());
    assert!(!old.exists(), "the old name stayed on disk");
    let back = Document::load(&written).expect("reload");
    assert_eq!(back.node(&back.root).expect("root").name(), "frmCustomer");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_form_already_named_after_its_file_stays_where_it_is() {
    let dir = folder("same");
    let here = dir.join("form_main.crawform");
    form_named("form_main").save(&here).expect("first save");

    assert_eq!(save(&form_named("form_main"), &here).expect("save"), here);
    assert!(here.exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_name_another_form_already_holds_does_not_overwrite_it() {
    let dir = folder("taken");
    let mine = dir.join("form_main.crawform");
    let theirs = dir.join("frmOther.crawform");
    form_named("form_main").save(&mine).expect("mine");
    form_named("frmOther").save(&theirs).expect("theirs");

    assert_eq!(path_for(&mine, &form_named("frmOther")), mine);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_change_of_case_alone_moves_the_file() {
    let dir = folder("case");
    let old = dir.join("form_main.crawform");
    form_named("form_main").save(&old).expect("first save");

    let written = save(&form_named("Form_Main"), &old).expect("save");

    let names: Vec<String> = std::fs::read_dir(&dir)
        .expect("list")
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(names, vec!["Form_Main.crawform".to_string()], "{names:?}");
    assert_eq!(
        written.file_name(),
        Path::new("Form_Main.crawform").file_name()
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_invalid_name_keeps_the_file() {
    let here = Path::new("C:/nowhere/src/form_main.crawform");
    assert_eq!(path_for(here, &form_named("not a name")), here);
}

#[test]
fn a_derived_form_opens_with_the_components_of_its_bases() {
    use crawrad_ir::Node;
    let dir = std::env::temp_dir().join(format!("crawrad-design-doc-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut a = Document::new("form_a", "Form");
    a.add_child("form_a", Node::new("btn_a", "Button")).unwrap();
    let mut b = Document::new_inherited("form_b", "Form", "form_a");
    b.add_child("form_b", Node::new("btn_b", "Button")).unwrap();
    a.save(dir.join("form_a.crawform")).unwrap();
    b.save(dir.join("form_b.crawform")).unwrap();
    let c = Document::new_inherited("form_c", "Form", "form_b");
    let doc = super::design_doc(&dir.join("form_c.crawform"), c);
    let _ = std::fs::remove_dir_all(&dir);
    assert!(doc.is_inherited("btn_a") && doc.is_inherited("btn_b"));
    assert_eq!(doc.ir().root, "form_c");
    assert_eq!(doc.own_ir().nodes.len(), 1);
}
