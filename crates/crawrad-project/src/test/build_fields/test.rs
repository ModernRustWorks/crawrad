use super::*;

#[test]
fn every_check_box_shows_and_takes_its_option() {
    let mut st = BuildState::default();
    for (name, kind) in st.fields() {
        if *kind != FieldKind::Check {
            continue;
        }
        let before = st.value(name);
        let Some(FieldValue::On(on)) = before else {
            panic!("{name}: {before:?}");
        };
        st.input(name, FieldValue::On(!on));
        assert_eq!(st.value(name), Some(FieldValue::On(!on)), "{name}");
    }
    assert!(!st.clean_before && !st.show_messages && st.verbose);
}

#[test]
fn presets_select_their_targets_and_the_empty_line_follows() {
    let mut st = BuildState::default();
    assert_eq!(
        st.value("bld_chip_empty"),
        Some(FieldValue::Text("No targets selected".into()))
    );
    st.input("bld_btn_preset_web", FieldValue::On(true));
    assert_eq!(st.selected_targets, vec![TargetPlatform::Wasm32]);
    assert_eq!(
        st.value("bld_chip_empty"),
        Some(FieldValue::Text(String::new()))
    );
    st.input("bld_btn_deselect_all", FieldValue::On(true));
    assert!(st.selected_targets.is_empty());
}

#[test]
fn the_environment_row_adds_edits_and_removes() {
    let mut st = BuildState::default();
    assert_eq!(st.value("bld_env_k_0"), None, "no row, nothing to paint");
    st.input("bld_env_add", FieldValue::On(true));
    assert_eq!(st.env_vars, vec![("RUST_LOG".into(), "info".into())]);
    st.input("bld_env_v_0", FieldValue::Text("debug".into()));
    assert_eq!(
        st.value("bld_env_v_0"),
        Some(FieldValue::Text("debug".into()))
    );
    st.input("bld_env_del_btn_0", FieldValue::On(true));
    assert!(st.env_vars.is_empty());
}

#[test]
fn the_tab_in_front_carries_the_dot_and_counts_show() {
    let mut st = BuildState {
        warnings_count: 3,
        ..BuildState::default()
    };
    st.input("bld_tab_warnings", FieldValue::On(true));
    assert_eq!(st.tab, BuildTab::Warnings);
    assert_eq!(
        st.value("bld_tab_warnings"),
        Some(FieldValue::Text("\u{26a0} Warnings (3) \u{25cf}".into()))
    );
    assert_eq!(
        st.value("bld_tab_output"),
        Some(FieldValue::Text(">_ Output".into()))
    );
}

#[test]
fn the_command_lines_show_only_on_a_command_page() {
    let mut st = BuildState::default();
    assert_eq!(st.value("bld_cmd_title"), None);
    st.page = BuildPage::Command(BuildMode::Check);
    assert_eq!(
        st.value("bld_cmd_title"),
        Some(FieldValue::Text(page_title(BuildMode::Check).into()))
    );
    assert!(matches!(
        st.value("bld_run_clock"),
        Some(FieldValue::Text(_))
    ));
}

#[test]
fn the_profile_button_toggles_and_shows_its_profile() {
    let mut st = BuildState::default();
    let before = st.profile;
    st.input("bld_profile_btn", FieldValue::On(true));
    assert_eq!(st.profile, before.toggle());
    assert_eq!(
        st.value("bld_profile_btn"),
        Some(FieldValue::Text(format!("{} \u{25be}", st.profile.label())))
    );
}
