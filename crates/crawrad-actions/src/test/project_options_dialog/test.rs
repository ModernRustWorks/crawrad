use super::*;
use crate::project_log::{LogFolder, LogLevel, LogOptions};
use std::path::PathBuf;

#[test]
fn default_initial_state() {
    let state = ProjectOptionsState::new("my_project", false, "my_project");
    assert_eq!(state.pages, vec![Page::Application, Page::Log]);
    assert_eq!(state.current_page, Page::Application);
    assert_eq!(state.title, "my_project");
    assert_eq!(state.pending_icon, None);
    assert_eq!(state.caption, DEFAULT_CAPTION);
    assert_eq!(state.error, None);
    assert_eq!(state.heading, "Project Options \u{2014} my_project");
}

#[test]
fn custom_initial_state() {
    let state = ProjectOptionsState::new("calc", true, "calc");
    assert_eq!(state.caption, "icon.ico");
}

#[test]
fn page_selection() {
    let mut state = ProjectOptionsState::new("calc", false, "calc");
    state.select_page(Page::Log);
    assert_eq!(state.current_page, Page::Log);
    state.select_page(Page::Application);
    assert_eq!(state.current_page, Page::Application);
}

#[test]
fn title_editing() {
    let mut state = ProjectOptionsState::new("old_title", false, "calc");
    state.set_title("new_title");
    assert_eq!(state.title, "new_title");
}

#[test]
fn loaded_path_sets_pending_and_caption() {
    let mut state = ProjectOptionsState::new("calc", false, "calc");
    let path = PathBuf::from("assets/custom_icon.png");
    state.loaded(path.clone());
    assert_eq!(state.caption, "custom_icon.png");
    assert_eq!(state.pending_icon, Some(IconSource::File(path)));
    assert_eq!(state.error, None);
}

#[test]
fn default_icon_restores_crab_caption_and_pending() {
    let mut state = ProjectOptionsState::new("calc", true, "calc");
    state.default_icon();
    assert_eq!(state.caption, DEFAULT_CAPTION);
    assert_eq!(state.pending_icon, Some(IconSource::Default));
    assert_eq!(state.error, None);
}

#[test]
fn cancel_discards_pending_and_error() {
    let mut state = ProjectOptionsState::new("calc", false, "calc");
    state.loaded(PathBuf::from("icon.ico"));
    state.set_error("some error");
    state.cancel();
    assert_eq!(state.pending_icon, None);
    assert_eq!(state.error, None);
}

#[test]
fn ok_returns_title_and_pending_icon() {
    let mut state = ProjectOptionsState::new("calc", false, "calc");
    state.set_title("Calculator");
    let path = PathBuf::from("app.ico");
    state.loaded(path.clone());
    let (applied_title, applied_icon) = state.ok();
    assert_eq!(applied_title, "Calculator");
    assert_eq!(applied_icon, Some(IconSource::File(path)));
    assert_eq!(state.pending_icon, None);
}

#[test]
fn error_message_management() {
    let mut state = ProjectOptionsState::new("calc", false, "calc");
    assert_eq!(state.error, None);
    state.set_error("Failed to write Cargo.toml");
    assert_eq!(state.error.as_deref(), Some("Failed to write Cargo.toml"));
    state.clear_error();
    assert_eq!(state.error, None);
}

#[test]
fn a_read_file_becomes_pending_and_a_bad_one_shows_why() {
    let mut st = ProjectOptionsState::new("app", false, "app");
    let ico = st.load_result(PathBuf::from("c:/x/logo.png"), Ok(vec![1, 2]));
    assert_eq!(ico, Some(vec![1, 2]));
    assert_eq!(st.caption, "logo.png");
    assert_eq!(
        st.pending_icon,
        Some(IconSource::File(PathBuf::from("c:/x/logo.png")))
    );
    let mut st = ProjectOptionsState::new("app", false, "app");
    let bad = st.load_result(PathBuf::from("c:/x/a.bmp"), Err("only .ico or .png".into()));
    assert_eq!(bad, None);
    assert_eq!(st.error.as_deref(), Some("only .ico or .png"));
    assert_eq!(st.pending_icon, None);
}

#[test]
fn markers_go_before_the_extension() {
    let mut st = ProjectOptionsState::new("app", false, "app");
    st.set_file_name("{app}.log");
    st.insert_marker("{date}");
    assert_eq!(st.log.file_name, "{app}_{date}.log");
    st.set_file_name("run-");
    st.insert_marker("{pid}");
    assert_eq!(st.log.file_name, "run-{pid}");
    st.set_file_name("");
    st.insert_marker("{app}");
    assert_eq!(st.log.file_name, "{app}");
}

#[test]
fn the_custom_folder_survives_picking_another() {
    let mut st = ProjectOptionsState::new("app", false, "app");
    assert_eq!(st.folder_choice(), FolderChoice::UserData);
    st.set_custom_folder("D:/logs");
    assert_eq!(st.log.folder, LogFolder::Custom("D:/logs".into()));
    st.choose_folder(FolderChoice::Executable);
    st.choose_folder(FolderChoice::Custom);
    assert_eq!(st.log.folder, LogFolder::Custom("D:/logs".into()));
}

#[test]
fn keep_takes_a_number_or_says_why() {
    let mut st = ProjectOptionsState::new("app", false, "app");
    st.set_keep("12");
    assert_eq!((st.log.keep, st.error.clone()), (12, None));
    st.set_keep("lots");
    assert_eq!(st.log.keep, 0);
    assert!(st.error.as_deref().is_some_and(|e| e.contains("number")));
}

#[test]
fn the_state_opens_on_the_projects_log() {
    let log = LogOptions {
        folder: LogFolder::Custom("C:/x".into()),
        keep: 5,
        ..LogOptions::default()
    };
    let st = ProjectOptionsState::new("app", false, "app").with_log(log.clone());
    assert_eq!(
        (st.log, st.custom_folder.as_str(), st.keep_text.as_str()),
        (log, "C:/x", "5")
    );
}

#[test]
fn each_build_has_its_own_level() {
    let mut st = ProjectOptionsState::new("app", false, "app");
    st.set_level(true, LogLevel::Debug);
    st.set_level(false, LogLevel::Off);
    assert_eq!(
        (st.log.debug_level, st.log.release_level),
        (LogLevel::Debug, LogLevel::Off)
    );
}

#[test]
fn the_log_page_names_follow_the_package() {
    let st = ProjectOptionsState::new("Calc", false, "meu_app");
    let now = crate::project_log::MarkerValues {
        app: "",
        date: "2026-11-09",
        time: "101500",
        pid: 7,
        user: "ana",
    };
    let t = st.log_text(&now);
    assert_eq!(t.today, "Today: meu_app_2026-11-09.log");
    assert_eq!(t.user_folder, "%APPDATA%\\meu_app\\logs");
    assert_eq!(t.command_line, "meu_app.exe --log=info");
    assert_eq!(t.env_variable, "MEU_APP_LOG=warn");
    assert_eq!(t.override_file, "meu_app.log.toml");
    assert_eq!(t.sample, "Write a sample meu_app.log.toml");
}

#[test]
fn every_field_the_dialog_names_shows_or_takes_something() {
    use crate::dialog_fields::{DialogFields, FieldKind, FieldValue};
    let mut st = ProjectOptionsState::new("Calc", false, "calc");
    for (name, kind) in st.fields() {
        let shown = st.value(name);
        match kind {
            FieldKind::Button => {}
            FieldKind::Text | FieldKind::Label => {
                assert!(
                    matches!(shown, Some(FieldValue::Text(_))),
                    "{name}: {shown:?}"
                )
            }
            FieldKind::Check => assert!(matches!(shown, Some(FieldValue::On(_))), "{name}"),
            FieldKind::Drop(_) | FieldKind::Radio(_) => {
                assert!(matches!(shown, Some(FieldValue::Index(_))), "{name}")
            }
        }
    }
    st.input("plog_m2", FieldValue::On(true));
    assert_eq!(st.log.file_name, "{app}_{date}_{time}.log");
    st.input("plog_folder", FieldValue::Index(2));
    assert_eq!(st.folder_choice(), FolderChoice::Custom);
    st.input("plog_release", FieldValue::Index(4));
    assert_eq!(st.log.release_level, LogLevel::Off);
    st.input("plog_new", FieldValue::Index(1));
    assert_eq!(st.value("plog_new"), Some(FieldValue::Index(1)));
    st.input("popt_title_input", FieldValue::Text("New".into()));
    assert_eq!(
        st.value("popt_title_input"),
        Some(FieldValue::Text("New".into()))
    );
}

#[test]
fn the_level_drop_downs_list_every_level_in_order() {
    use crate::dialog_fields::{DialogFields, FieldKind};
    let st = ProjectOptionsState::new("Calc", false, "calc");
    let items = st
        .fields()
        .iter()
        .find_map(|(n, k)| match k {
            FieldKind::Drop(items) if *n == "plog_debug" => Some(*items),
            _ => None,
        })
        .expect("plog_debug");
    let labels: Vec<&str> = LogLevel::ALL.iter().map(|l| l.label()).collect();
    assert_eq!(items, labels.as_slice());
}
