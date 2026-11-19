use super::*;
use crawrad_ir::{Document, Node, Rect, Value};
use crawrad_registry::Registry;

fn make_form() -> Document {
    let mut doc = Document::new("form_main", "Form");
    doc.nodes
        .get_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
    doc
}

fn add_control(doc: &mut Document, parent: &str, id: &str, class: &str) {
    let n = Node::new(id, class);
    doc.add_child(parent, n).unwrap();
}

#[test]
fn source_texts_extracts_translatable_properties_in_paint_order() {
    let mut doc = make_form();
    doc.nodes
        .get_mut("form_main")
        .unwrap()
        .props
        .insert("text".into(), Value::Str("Main Window".into()));

    add_control(&mut doc, "form_main", "btn_save", "Button");
    let btn_save = doc.nodes.get_mut("btn_save").unwrap();
    btn_save
        .props
        .insert("text".into(), Value::Str("Save".into()));
    btn_save
        .props
        .insert("hint".into(), Value::Str("Save changes".into()));

    // Empty text should be skipped
    add_control(&mut doc, "form_main", "btn_empty", "Button");
    let btn_empty = doc.nodes.get_mut("btn_empty").unwrap();
    btn_empty.props.insert("text".into(), Value::Str("".into()));

    // Class not in registry should be skipped
    add_control(
        &mut doc,
        "form_main",
        "unregistered_widget",
        "UnknownWidget",
    );
    let unregistered = doc.nodes.get_mut("unregistered_widget").unwrap();
    unregistered
        .props
        .insert("text".into(), Value::Str("Ignored".into()));

    // Added in paint order after btn_save and btn_empty
    add_control(&mut doc, "form_main", "btn_cancel", "Button");
    let btn_cancel = doc.nodes.get_mut("btn_cancel").unwrap();
    btn_cancel
        .props
        .insert("text".into(), Value::Str("Cancel".into()));

    let reg = Registry::builtin();
    let sources = source_texts(&doc, "form_main", &reg);

    assert_eq!(sources.len(), 4);

    // Root caption comes first
    assert_eq!(sources[0].key, "form_main.form_main.text");
    assert_eq!(sources[0].label, "form_main.text");
    assert_eq!(sources[0].node, "form_main");
    assert_eq!(sources[0].prop, "text");
    assert_eq!(sources[0].text, "Main Window");

    // Button text
    assert_eq!(sources[1].key, "form_main.btn_save.text");
    assert_eq!(sources[1].label, "btn_save.text");
    assert_eq!(sources[1].node, "btn_save");
    assert_eq!(sources[1].prop, "text");
    assert_eq!(sources[1].text, "Save");

    // Button hint
    assert_eq!(sources[2].key, "form_main.btn_save.hint");
    assert_eq!(sources[2].label, "btn_save.hint");
    assert_eq!(sources[2].node, "btn_save");
    assert_eq!(sources[2].prop, "hint");
    assert_eq!(sources[2].text, "Save changes");

    // Paint order: btn_cancel text
    assert_eq!(sources[3].key, "form_main.btn_cancel.text");
    assert_eq!(sources[3].label, "btn_cancel.text");
    assert_eq!(sources[3].node, "btn_cancel");
    assert_eq!(sources[3].prop, "text");
    assert_eq!(sources[3].text, "Cancel");
}

#[test]
fn text_key_uses_node_id_when_name_changes() {
    let mut doc = make_form();
    add_control(&mut doc, "form_main", "btn_1", "Button");
    let btn = doc.nodes.get_mut("btn_1").unwrap();
    btn.props.insert("text".into(), Value::Str("Click".into()));
    btn.set_name("btn_renamed");

    let reg = Registry::builtin();
    let sources = source_texts(&doc, "form_main", &reg);
    let btn_source = sources.iter().find(|s| s.node == "btn_1").unwrap();

    assert_eq!(btn_source.key, "form_main.btn_1.text");
    assert_eq!(btn_source.label, "btn_renamed.text");
    assert_eq!(
        text_key("form_main", &"btn_1".to_string(), "text"),
        "form_main.btn_1.text"
    );
}

#[test]
fn cell_states_done_stale_and_missing() {
    let source = SourceText {
        key: "form_main.btn_ok.text".into(),
        label: "btn_ok.text".into(),
        node: "btn_ok".into(),
        prop: "text".into(),
        text: "OK".into(),
    };

    let mut locale = Locale::new("es");

    // Missing when key is absent
    assert_eq!(cell(&source, &locale), CellState::Missing);

    // Missing when translation text is empty
    locale.set("form_main.btn_ok.text", "", "OK");
    assert_eq!(cell(&source, &locale), CellState::Missing);

    // Done when text is non-empty and from matches source text
    locale.set("form_main.btn_ok.text", "Aceptar", "OK");
    assert_eq!(cell(&source, &locale), CellState::Done("Aceptar".into()));

    // Stale when from differs from source text
    locale.set("form_main.btn_ok.text", "Aceptar", "Previous Text");
    assert_eq!(cell(&source, &locale), CellState::Stale("Aceptar".into()));
}

#[test]
fn progress_counts_done_only() {
    let s1 = SourceText {
        key: "form_main.btn_save.text".into(),
        label: "btn_save.text".into(),
        node: "btn_save".into(),
        prop: "text".into(),
        text: "Save".into(),
    };
    let s2 = SourceText {
        key: "form_main.btn_cancel.text".into(),
        label: "btn_cancel.text".into(),
        node: "btn_cancel".into(),
        prop: "text".into(),
        text: "Cancel".into(),
    };
    let s3 = SourceText {
        key: "form_main.btn_help.text".into(),
        label: "btn_help.text".into(),
        node: "btn_help".into(),
        prop: "text".into(),
        text: "Help".into(),
    };

    let sources = vec![s1, s2, s3];
    let mut locale = Locale::new("fr");

    // s1 is Done
    locale.set("form_main.btn_save.text", "Enregistrer", "Save");
    // s2 is Stale
    locale.set("form_main.btn_cancel.text", "Annuler", "Old Cancel");
    // s3 is Missing (not in locale)

    let (done, total) = progress(&sources, &locale);
    assert_eq!(done, 1);
    assert_eq!(total, 3);
}

#[test]
fn localized_replaces_done_and_stale_keeps_missing_and_leaves_input_untouched() {
    let mut doc = make_form();
    doc.nodes
        .get_mut("form_main")
        .unwrap()
        .props
        .insert("text".into(), Value::Str("Main Window".into()));

    add_control(&mut doc, "form_main", "btn_done", "Button");
    doc.nodes
        .get_mut("btn_done")
        .unwrap()
        .props
        .insert("text".into(), Value::Str("Save".into()));

    add_control(&mut doc, "form_main", "btn_stale", "Button");
    doc.nodes
        .get_mut("btn_stale")
        .unwrap()
        .props
        .insert("text".into(), Value::Str("Edit".into()));

    add_control(&mut doc, "form_main", "btn_missing", "Button");
    doc.nodes
        .get_mut("btn_missing")
        .unwrap()
        .props
        .insert("text".into(), Value::Str("Delete".into()));

    let mut locale = Locale::new("es");
    locale.set(
        "form_main.form_main.text",
        "Ventana Principal",
        "Main Window",
    );
    locale.set("form_main.btn_done.text", "Guardar", "Save");
    locale.set("form_main.btn_stale.text", "Editar", "Old Edit");
    // btn_missing is not translated in locale

    let reg = Registry::builtin();
    let localized_doc = localized(&doc, "form_main", &reg, &locale);

    // Translated document has replacements
    assert_eq!(
        localized_doc.node("form_main").unwrap().props.get("text"),
        Some(&Value::Str("Ventana Principal".into()))
    );
    assert_eq!(
        localized_doc.node("btn_done").unwrap().props.get("text"),
        Some(&Value::Str("Guardar".into()))
    );
    assert_eq!(
        localized_doc.node("btn_stale").unwrap().props.get("text"),
        Some(&Value::Str("Editar".into()))
    );
    assert_eq!(
        localized_doc.node("btn_missing").unwrap().props.get("text"),
        Some(&Value::Str("Delete".into()))
    );

    // Original input document remains completely untouched
    assert_eq!(
        doc.node("form_main").unwrap().props.get("text"),
        Some(&Value::Str("Main Window".into()))
    );
    assert_eq!(
        doc.node("btn_done").unwrap().props.get("text"),
        Some(&Value::Str("Save".into()))
    );
    assert_eq!(
        doc.node("btn_stale").unwrap().props.get("text"),
        Some(&Value::Str("Edit".into()))
    );
    assert_eq!(
        doc.node("btn_missing").unwrap().props.get("text"),
        Some(&Value::Str("Delete".into()))
    );
}

#[test]
fn locale_tuples_round_trip_sorted() {
    let input = vec![
        (
            "key_z".to_string(),
            "Text Z".to_string(),
            "From Z".to_string(),
        ),
        (
            "key_a".to_string(),
            "Text A".to_string(),
            "From A".to_string(),
        ),
        (
            "key_m".to_string(),
            "Text M".to_string(),
            "From M".to_string(),
        ),
    ];

    let locale = Locale::from_tuples("de", input);
    let output = locale.to_tuples();

    assert_eq!(
        output,
        vec![
            (
                "key_a".to_string(),
                "Text A".to_string(),
                "From A".to_string()
            ),
            (
                "key_m".to_string(),
                "Text M".to_string(),
                "From M".to_string()
            ),
            (
                "key_z".to_string(),
                "Text Z".to_string(),
                "From Z".to_string()
            ),
        ]
    );
}

#[test]
fn language_name_known_and_unknown() {
    assert_eq!(language_name("en"), "English");
    assert_eq!(language_name("pt-BR"), "Português (Brasil)");
    assert_eq!(language_name("pt-PT"), "Português (Portugal)");
    assert_eq!(language_name("es"), "Español");
    assert_eq!(language_name("fr"), "Français");
    assert_eq!(language_name("de"), "Deutsch");
    assert_eq!(language_name("it"), "Italiano");
    assert_eq!(language_name("ja"), "日本語");
    assert_eq!(language_name("zh-CN"), "简体中文");
    assert_eq!(language_name("ru"), "Русский");
    assert_eq!(language_name("nl"), "Nederlands");
    assert_eq!(language_name("pl"), "Polski");
    assert_eq!(language_name("tr"), "Türkçe");
    assert_eq!(language_name("unknown-code"), "unknown-code");
}

#[test]
fn valid_code_bcp47() {
    assert!(valid_code("pt-BR"));
    assert!(valid_code("de"));
    assert!(valid_code("zh-Hans"));
    assert!(!valid_code("en"));
    assert!(!valid_code("PT"));
    assert!(!valid_code("pt_BR"));
    assert!(!valid_code(""));
}

#[test]
fn table_grouping_order_and_cells_order() {
    let s_main = SourceText {
        key: "form_main.btn_ok.text".into(),
        label: "btn_ok.text".into(),
        node: "btn_ok".into(),
        prop: "text".into(),
        text: "OK".into(),
    };
    let s_settings = SourceText {
        key: "form_settings.btn_save.text".into(),
        label: "btn_save.text".into(),
        node: "btn_save".into(),
        prop: "text".into(),
        text: "Save".into(),
    };

    let forms = vec![
        ("form_main".to_string(), vec![s_main]),
        ("form_settings".to_string(), vec![s_settings]),
    ];

    let mut loc_es = Locale::new("es");
    loc_es.set("form_main.btn_ok.text", "Aceptar", "OK");
    loc_es.set("form_settings.btn_save.text", "Guardar", "Save");

    let mut loc_de = Locale::new("de");
    loc_de.set("form_main.btn_ok.text", "OK", "OK");

    let locales = vec![loc_es, loc_de];
    let groups = table(&forms, &locales, false);

    assert_eq!(groups.len(), 2);
    assert_eq!(groups[0].form, "form_main");
    assert_eq!(groups[0].rows.len(), 1);
    assert_eq!(groups[0].rows[0].key, "form_main.btn_ok.text");
    assert_eq!(groups[0].rows[0].label, "btn_ok.text");
    assert_eq!(groups[0].rows[0].source, "OK");
    assert_eq!(
        groups[0].rows[0].cells,
        vec![
            CellState::Done("Aceptar".into()),
            CellState::Done("OK".into()),
        ]
    );

    assert_eq!(groups[1].form, "form_settings");
    assert_eq!(groups[1].rows.len(), 1);
    assert_eq!(groups[1].rows[0].key, "form_settings.btn_save.text");
    assert_eq!(groups[1].rows[0].label, "btn_save.text");
    assert_eq!(groups[1].rows[0].source, "Save");
    assert_eq!(
        groups[1].rows[0].cells,
        vec![CellState::Done("Guardar".into()), CellState::Missing]
    );
}

#[test]
fn table_filter_and_empty_group_dropped() {
    let s1 = SourceText {
        key: "form_main.btn_done.text".into(),
        label: "btn_done.text".into(),
        node: "btn_done".into(),
        prop: "text".into(),
        text: "Done".into(),
    };
    let s2 = SourceText {
        key: "form_main.btn_missing.text".into(),
        label: "btn_missing.text".into(),
        node: "btn_missing".into(),
        prop: "text".into(),
        text: "Pending".into(),
    };
    let s_clean = SourceText {
        key: "form_clean.lbl.text".into(),
        label: "lbl.text".into(),
        node: "lbl".into(),
        prop: "text".into(),
        text: "Clean".into(),
    };

    let forms = vec![
        ("form_main".to_string(), vec![s1, s2]),
        ("form_clean".to_string(), vec![s_clean]),
    ];

    let mut loc = Locale::new("fr");
    loc.set("form_main.btn_done.text", "Fait", "Done");
    // form_main.btn_missing.text is missing
    loc.set("form_clean.lbl.text", "Propre", "Clean");

    let locales = vec![loc];

    // Without missing_only: all groups kept
    let all_groups = table(&forms, &locales, false);
    assert_eq!(all_groups.len(), 2);
    assert_eq!(missing_count(&all_groups), 1);

    // With missing_only: form_clean has no missing/stale cells so its group is dropped
    let filtered = table(&forms, &locales, true);
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].form, "form_main");
    assert_eq!(filtered[0].rows.len(), 1);
    assert_eq!(filtered[0].rows[0].key, "form_main.btn_missing.text");
    assert_eq!(missing_count(&filtered), 1);
}

#[test]
fn missing_count_counts_stale_and_missing() {
    let group = TableGroup {
        form: "form_1".into(),
        rows: vec![
            TableRow {
                key: "k1".into(),
                label: "l1".into(),
                source: "s1".into(),
                cells: vec![CellState::Done("ok".into())],
            },
            TableRow {
                key: "k2".into(),
                label: "l2".into(),
                source: "s2".into(),
                cells: vec![CellState::Stale("old".into())],
            },
            TableRow {
                key: "k3".into(),
                label: "l3".into(),
                source: "s3".into(),
                cells: vec![CellState::Missing],
            },
        ],
    };
    assert_eq!(missing_count(&[group]), 2);
}

#[test]
fn test_language_rows() {
    let codes = vec!["de".to_string(), "pt-BR".to_string()];

    // Default or en selected
    let rows_en = language_rows(&codes, None);
    assert_eq!(rows_en.len(), 3);
    assert_eq!(rows_en[0], ("en".into(), "English (source)".into(), true));
    assert_eq!(rows_en[1], ("de".into(), "Deutsch".into(), false));
    assert_eq!(
        rows_en[2],
        ("pt-BR".into(), "Português (Brasil)".into(), false)
    );

    // Non-en selected
    let rows_de = language_rows(&codes, Some("de"));
    assert_eq!(rows_de[0], ("en".into(), "English (source)".into(), false));
    assert_eq!(rows_de[1], ("de".into(), "Deutsch".into(), true));
    assert_eq!(
        rows_de[2],
        ("pt-BR".into(), "Português (Brasil)".into(), false)
    );
}

#[test]
fn test_run_language_rows() {
    let codes = vec!["de".to_string(), "pt-BR".to_string()];

    // System language default
    let rows_sys = run_language_rows(&codes, None);
    assert_eq!(rows_sys.len(), 4);
    assert_eq!(rows_sys[0], (None, "System language".into(), true));
    assert_eq!(rows_sys[1], (Some("en".into()), "English".into(), false));
    assert_eq!(rows_sys[2], (Some("de".into()), "Deutsch".into(), false));
    assert_eq!(
        rows_sys[3],
        (Some("pt-BR".into()), "Português (Brasil)".into(), false)
    );

    // Specific language
    let rows_de = run_language_rows(&codes, Some("de"));
    assert_eq!(rows_de[0], (None, "System language".into(), false));
    assert_eq!(rows_de[1], (Some("en".into()), "English".into(), false));
    assert_eq!(rows_de[2], (Some("de".into()), "Deutsch".into(), true));
    assert_eq!(
        rows_de[3],
        (Some("pt-BR".into()), "Português (Brasil)".into(), false)
    );
}

#[test]
fn test_banner_text() {
    assert_eq!(banner("en"), None);
    assert_eq!(banner(""), None);
    assert_eq!(
        banner("de"),
        Some(
            "Showing Deutsch — read-only texts. Switch back to English to edit them here, or edit them in Translations."
                .to_string()
        )
    );
    assert_eq!(
        banner("pt-BR"),
        Some(
            "Showing Português (Brasil) — read-only texts. Switch back to English to edit them here, or edit them in Translations."
                .to_string()
        )
    );
}

#[test]
fn test_language_button_text() {
    assert_eq!(language_button_text(None), "Language: English (source)");
    assert_eq!(language_button_text(Some("")), "Language: English (source)");
    assert_eq!(
        language_button_text(Some("en")),
        "Language: English (source)"
    );
    assert_eq!(language_button_text(Some("de")), "Language: Deutsch");
    assert_eq!(
        language_button_text(Some("pt-BR")),
        "Language: Português (Brasil)"
    );
}

#[test]
fn test_text_fits_calculation() {
    assert!(text_fits("Save", 12.0, 40.0));
    assert!(!text_fits("Salvar alterações", 12.0, 40.0));
    // Multi-line uses the widest line
    assert!(text_fits("Save\nOK", 12.0, 40.0));
    assert!(!text_fits("Save\nSalvar alterações", 12.0, 40.0));
}

#[test]
fn test_banner_with_fit_logic() {
    assert_eq!(banner_with_fit("en", &[]), None);
    assert_eq!(banner_with_fit("en", &["btn_save".to_string()]), None);

    let base_pt = "Showing Português (Brasil) — read-only texts. Switch back to English to edit them here, or edit them in Translations.";
    assert_eq!(
        banner_with_fit("pt-BR", &[]),
        Some((format!("{base_pt} All texts fit."), true))
    );

    let one = vec!["btn_save".to_string()];
    assert_eq!(
        banner_with_fit("pt-BR", &one),
        Some((format!("{base_pt} 1 text does not fit: btn_save"), false))
    );

    let four = vec![
        "btn_save".to_string(),
        "btn_cancel".to_string(),
        "btn_help".to_string(),
        "btn_apply".to_string(),
    ];
    assert_eq!(
        banner_with_fit("pt-BR", &four),
        Some((
            format!("{base_pt} 4 texts do not fit: btn_save, btn_cancel, btn_help…"),
            false
        ))
    );
}

#[test]
fn test_overflowing_detects_non_fitting_controls() {
    let mut doc = make_form();
    add_control(&mut doc, "form_main", "btn_small", "Button");
    let btn_small = doc.nodes.get_mut("btn_small").unwrap();
    btn_small.set_rect(Rect::new(10.0, 10.0, 40.0, 24.0));
    btn_small
        .props
        .insert("text".into(), Value::Str("Save".into()));

    add_control(&mut doc, "form_main", "btn_wide", "Button");
    let btn_wide = doc.nodes.get_mut("btn_wide").unwrap();
    btn_wide.set_rect(Rect::new(10.0, 40.0, 200.0, 24.0));
    btn_wide
        .props
        .insert("text".into(), Value::Str("Cancel".into()));

    let mut locale = Locale::new("pt-BR");
    locale.set("form_main.btn_small.text", "Salvar alterações", "Save");
    locale.set("form_main.btn_wide.text", "Cancelar", "Cancel");

    let reg = Registry::builtin();
    let ov = overflowing(&doc, "pt-BR", &reg, &locale);
    assert_eq!(ov, vec!["btn_small".to_string()]);

    let ov_en = overflowing(&doc, "en", &reg, &locale);
    assert!(ov_en.is_empty());
}
