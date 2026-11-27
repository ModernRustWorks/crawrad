use super::*;

#[test]
fn an_ellipsis_action_is_refused_only_while_a_dialog_is_open() {
    assert!(modal_refusal(true, "Options...").is_some());
    assert!(modal_refusal(true, "New Project\u{2026}").is_some());
    assert!(modal_refusal(true, "Undo").is_none());
    assert!(modal_refusal(false, "Options...").is_none());
}

#[test]
fn esc_presses_cancel_or_else_close() {
    assert_eq!(OPTIONS.cancel_control(), Some("opt_cancel"));
    assert_eq!(BUILD.cancel_control(), Some("bld_close"));
    assert_eq!(NEW_PROJECT.cancel_control(), Some("np_cancel"));
}

/// A dialog absent from `veil_roots` is not modal: `modal_open` answers "no
/// dialog", key dispatch hands the keystroke to the window behind, and typing
/// in the dialog lands in the user's source file. That happened to the Note
/// dialog, so every dialog that owns a veil is named here.
#[test]
fn a_dialog_with_a_veil_is_modal() {
    let roots: Vec<_> = veil_roots().collect();
    for veil in ["note_dialog", "shrink_dialog", "environment_dialog"] {
        assert!(roots.contains(&veil), "{veil} owns a veil but is not modal");
    }
}

#[test]
fn every_veil_is_named_once() {
    let roots: Vec<_> = veil_roots().collect();
    assert!(roots.contains(&"shrink_dialog") && roots.contains(&"goto_dialog"));
    let unique: std::collections::BTreeSet<_> = roots.iter().collect();
    assert_eq!(unique.len(), roots.len());
}

fn find(name: &str) -> Option<Control> {
    NEW_PROJECT.expanded().into_iter().find(|c| c.name == name)
}

#[test]
fn a_repeated_row_expands_with_its_label() {
    assert_eq!(
        find("np_cat_list").map(|c| c.kind),
        Some(ControlKind::Label)
    );
    assert_eq!(
        find("np_card_7").and_then(|c| c.text_of).as_deref(),
        Some("np_c7_title")
    );
    assert_eq!(find("np_c3_fav").map(|c| c.kind), Some(ControlKind::Button));
    assert_eq!(
        find("np_escolher").map(|c| c.kind),
        Some(ControlKind::Button)
    );
    let all = NEW_PROJECT.expanded();
    assert_eq!(
        all.iter().filter(|c| c.name.starts_with("np_cat_")).count(),
        1
    );
    assert_eq!(
        all.iter()
            .filter(|c| c.name.starts_with("np_card_"))
            .count(),
        8
    );
    assert!(
        !all.iter().any(|c| c.name.contains("{i}")),
        "every {{i}} is expanded"
    );
}

/// Every control the table names must exist in the dialog's DSL: a misspelt
/// name would read as an empty control, silently.
#[test]
fn every_new_project_control_exists_in_the_dialog_dsl() {
    let dsl = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../crawrad-ide/src/ide_app/dialogs/dsl/project_dialogs.rs"
    ))
    .expect("the New Project DSL");
    for c in NEW_PROJECT.expanded() {
        assert!(
            dsl.contains(&format!("{} = <", c.name)),
            "{} is not in the New Project DSL",
            c.name
        );
        if let Some(label) = c.text_of {
            assert!(
                dsl.contains(&format!("{label} = <")),
                "{label} is not in the New Project DSL"
            );
        }
    }
}

#[test]
fn every_build_control_exists_in_the_dialog_dsl() {
    let dsl = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../crawrad-ide/src/ide_app/dialogs/dsl/tools_dialogs.rs"
    ))
    .expect("the Build DSL");
    for c in BUILD.expanded().into_iter() {
        assert!(
            dsl.contains(&format!("{} = <", c.name)),
            "{} is not in the Build DSL",
            c.name
        );
        if let Some(label) = c.text_of {
            assert!(
                dsl.contains(&format!("{label} = <")),
                "{label} is not in the Build DSL"
            );
        }
    }
}

#[test]
fn every_options_control_exists_in_the_dialog_dsl() {
    let dsl = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../crawrad-ide/src/ide_app/dialogs/dsl/options_dialog.rs"
    ))
    .expect("the Options DSL");
    let all = OPTIONS.expanded();
    assert!(
        all.len() > 50,
        "the Options table lists its controls ({})",
        all.len()
    );
    for c in all {
        assert!(
            dsl.contains(&format!("{} = <", c.name)),
            "{} is not in the Options DSL",
            c.name
        );
    }
}

/// The kind a widget type is read as — the same mapping as `tools/gen_dialog_table.py`.
fn kind_of(ty: &str) -> Option<ControlKind> {
    let text = [
        "CrawradCampoDenso",
        "CrawradPropInput",
        "CrawradTextInput",
        "CrawradLinesText",
        "TextInput",
    ];
    let label = [
        "CrawradDialogTitle",
        "CrawradLabel",
        "CrawradLabelMuted",
        "CrawradMonoLabel",
        "CrawradMonoLabelMuted",
        "Label",
    ];
    match ty {
        "CrawradItemsRow" => Some(ControlKind::Row),
        t if t.contains("Button") || t.contains("Radio") => Some(ControlKind::Button),
        t if t.contains("Check") => Some(ControlKind::Check),
        t if t.contains("Drop") => Some(ControlKind::Drop),
        t if text.contains(&t) || t.contains("Input") => Some(ControlKind::Text),
        t if label.contains(&t) => Some(ControlKind::Label),
        _ => None,
    }
}

/// `name = <Type` pairs in a piece of DSL, in order.
fn declared(block: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in block.lines() {
        let mut rest = line;
        while let Some(at) = rest.find(" = <") {
            let name: String = rest[..at]
                .chars()
                .rev()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            let after = &rest[at + 4..];
            let ty: String = after
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect();
            if name.starts_with(|c: char| c.is_ascii_lowercase()) {
                out.push((name, ty));
            }
            rest = after;
        }
    }
    out
}

/// The generated table and the DSL agree in BOTH directions: every control the
/// table names is in its dialog, and every control the dialog declares is in the
/// table. A drift means `python tools/gen_dialog_table.py` was not run.
#[test]
fn every_generated_dialog_matches_its_dsl() {
    let shell = concat!(env!("CARGO_MANIFEST_DIR"), "/../crawrad-ide/src/ide_app");
    let main = std::fs::read_to_string(format!("{shell}/dsl_main.rs")).expect("dsl_main.rs");
    let dir = format!("{shell}/dialogs/dsl");
    let sources: Vec<String> = std::fs::read_dir(&dir)
        .expect("the dialogs' DSL")
        .filter_map(|e| std::fs::read_to_string(e.ok()?.path()).ok())
        .collect();
    for spec in crate::dialogs_table::GENERATED {
        let instance = format!("{} = <", spec.root[0]);
        let at = main
            .find(&instance)
            .unwrap_or_else(|| panic!("{} is not in dsl_main.rs", spec.root[0]));
        let template: String = main[at + instance.len()..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric())
            .collect();
        let head = format!("pub {template} = ");
        let text = sources
            .iter()
            .find(|s| s.contains(&head))
            .unwrap_or_else(|| panic!("{template} is not in the dialogs' DSL"));
        let start = text.find(&head).expect("found above");
        let end = text[start + 1..]
            .find("pub Crawrad")
            .map_or(text.len(), |e| start + 1 + e);
        let mut want: Vec<(String, ControlKind)> = Vec::new();
        for (name, ty) in declared(&text[start..end]) {
            if let Some(k) = kind_of(&ty).filter(|_| name != "dlg")
                && !want.iter().any(|(n, _)| *n == name)
            {
                want.push((name, k));
            }
        }
        let have: Vec<(String, ControlKind)> = spec
            .expanded()
            .into_iter()
            .map(|c| (c.name, c.kind))
            .collect();
        assert_eq!(
            have, want,
            "{}: the generated table and its DSL differ — run tools/gen_dialog_table.py",
            spec.name
        );
    }
}

#[test]
fn every_dialog_has_a_distinct_root() {
    let roots: Vec<_> = all().iter().map(|d| d.root[0]).collect();
    let mut unique = roots.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(
        roots.len(),
        unique.len(),
        "two dialogs share a root: {roots:?}"
    );
    assert_eq!(roots.len(), 26);
}
