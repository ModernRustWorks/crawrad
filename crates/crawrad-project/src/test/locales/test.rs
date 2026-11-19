use super::*;

#[test]
fn round_trip_with_quotes_equals_and_non_ascii_text() {
    let tmp = crate::fixture::folder("locales-round-trip");
    let entries = vec![
        (
            "form.label.text".to_string(),
            "Non-ASCII: Olá mundo, çãé!".to_string(),
            "Non-ASCII: Hello world!".to_string(),
        ),
        (
            "form.special.text".to_string(),
            "With \"quotes\" and key=value pairs".to_string(),
            "With \"quotes\" and a=b".to_string(),
        ),
    ];
    write_locale(&tmp, "pt-BR", &entries).unwrap();
    let read_back = read_locale(&tmp, "pt-BR");
    assert_eq!(read_back, entries);
}

#[test]
fn bad_entry_is_skipped_and_missing_file_is_empty() {
    let tmp = crate::fixture::folder("locales-bad-entry");

    let missing = read_locale(&tmp, "nonexistent");
    assert!(missing.is_empty());

    let locales_dir = tmp.join(LOCALES_DIR);
    std::fs::create_dir_all(&locales_dir).unwrap();
    let toml_content = r#"
"good.one" = { text = "Valid Text", from = "English" }
"bad.scalar" = 42
"bad.missing_from" = { text = "Only text" }
"bad.missing_text" = { from = "Only from" }
"bad.non_string" = { text = 123, from = "From" }
"good.two" = { text = "Second Valid", from = "Second English" }
"#;
    std::fs::write(locales_dir.join("es.toml"), toml_content).unwrap();

    let read_back = read_locale(&tmp, "es");
    assert_eq!(
        read_back,
        vec![
            (
                "good.one".to_string(),
                "Valid Text".to_string(),
                "English".to_string()
            ),
            (
                "good.two".to_string(),
                "Second Valid".to_string(),
                "Second English".to_string()
            ),
        ]
    );
}

#[test]
fn list_locales_returns_sorted_codes() {
    let tmp = crate::fixture::folder("locales-list");
    let dummy_entries = vec![("key".to_string(), "text".to_string(), "from".to_string())];
    write_locale(&tmp, "pt-BR", &dummy_entries).unwrap();
    write_locale(&tmp, "de", &dummy_entries).unwrap();
    write_locale(&tmp, "fr", &dummy_entries).unwrap();
    write_locale(&tmp, "es", &dummy_entries).unwrap();

    std::fs::write(tmp.join(LOCALES_DIR).join("notes.txt"), "ignore me").unwrap();

    let codes = list_locales(&tmp);
    assert_eq!(codes, vec!["de", "es", "fr", "pt-BR"]);
}

#[test]
fn remove_locale_deletes_file_and_tolerates_missing() {
    let tmp = crate::fixture::folder("locales-remove");
    let dummy_entries = vec![("key".to_string(), "text".to_string(), "from".to_string())];
    write_locale(&tmp, "de", &dummy_entries).unwrap();
    assert_eq!(list_locales(&tmp), vec!["de"]);

    remove_locale(&tmp, "de").unwrap();
    assert!(list_locales(&tmp).is_empty());
    assert!(read_locale(&tmp, "de").is_empty());

    assert!(remove_locale(&tmp, "de").is_ok());
}

#[test]
fn set_entry_replaces_the_key_and_read_all_lists_every_locale() {
    let dir = std::env::temp_dir().join(format!("crawrad-locales-set-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    write_locale(&dir, "de", &[]).expect("write de");
    set_entry(&dir, "pt-BR", "f.b.text", "Ok", "OK").expect("set");
    set_entry(&dir, "pt-BR", "f.b.text", "Certo", "OK").expect("set again");
    set_entry(&dir, "pt-BR", "f.a.text", "Nome", "Name").expect("set other");
    let all = read_all(&dir);
    assert_eq!(all.len(), 2);
    assert_eq!(all[0].0, "de");
    assert!(all[0].1.is_empty());
    assert_eq!(all[1].0, "pt-BR");
    assert_eq!(
        all[1].1,
        vec![
            (
                "f.a.text".to_string(),
                "Nome".to_string(),
                "Name".to_string()
            ),
            (
                "f.b.text".to_string(),
                "Certo".to_string(),
                "OK".to_string()
            ),
        ]
    );
    let _ = std::fs::remove_dir_all(&dir);
}
