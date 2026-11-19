use super::*;

#[test]
fn the_file_round_trips() {
    let entries = vec![(
        "evlog".to_string(),
        "Event with log line".to_string(),
        "pub fn ${1:btn_ok}_click(&mut self) {\n    log::info!(\"x\");\n    $0\n}".to_string(),
    )];
    assert_eq!(parse(&to_text(&entries)), entries);
}

#[test]
fn a_bad_file_reads_as_none() {
    assert!(parse("[[template]\ntrigger = ").is_empty());
}

#[test]
fn an_entry_without_trigger_or_body_is_skipped() {
    let text = "[[template]]\ntrigger = \"\"\nbody = \"x\"\n\n[[template]]\ntrigger = \"a\"\n\n[[template]]\ntrigger = \" b \"\nbody = \"y$0\"\n";
    assert_eq!(
        parse(text),
        vec![("b".to_string(), String::new(), "y$0".to_string())]
    );
}

#[test]
fn write_and_read_file_round_trips() {
    let tmp = std::env::temp_dir().join(format!("crawrad_templates_test_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    let path = tmp.join("sub").join("imported.toml");
    let entries = vec![(
        "custom".to_string(),
        "A custom template".to_string(),
        "body $0".to_string(),
    )];
    write_file(&path, &entries).expect("write");
    let read = read_file(&path).expect("read");
    assert_eq!(read, entries);
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn read_nonexistent_file_returns_error() {
    let tmp = std::env::temp_dir().join(format!(
        "crawrad_templates_nonexistent_{}",
        std::process::id()
    ));
    assert!(read_file(tmp.join("does_not_exist.toml")).is_err());
}
