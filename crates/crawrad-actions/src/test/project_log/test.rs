use super::*;

fn values() -> MarkerValues<'static> {
    MarkerValues {
        app: "meu_app",
        date: "2026-11-09",
        time: "143005",
        pid: 4242,
        user: "ana",
    }
}

#[test]
fn the_default_name_is_one_file_a_day_in_the_users_folder() {
    let o = LogOptions::default();
    assert_eq!(
        expand_file_name(&o.file_name, &values()),
        "meu_app_2026-11-09.log"
    );
    assert_eq!(o.folder, LogFolder::UserData);
    assert_eq!(
        (o.debug_level, o.release_level),
        (LogLevel::Info, LogLevel::Warning)
    );
    assert_eq!(check(&o), Ok(()));
}

#[test]
fn every_marker_is_filled() {
    let name = expand_file_name("{app}-{user}-{pid}-{date}_{time}.log", &values());
    assert_eq!(name, "meu_app-ana-4242-2026-11-09_143005.log");
}

#[test]
fn levels_round_trip_and_warning_is_read_as_warn() {
    for l in LogLevel::ALL {
        assert_eq!(LogLevel::from_key(l.key()), Some(l));
    }
    assert_eq!(LogLevel::from_key(" Warning "), Some(LogLevel::Warning));
    assert_eq!(LogLevel::from_key("loud"), None);
}

#[test]
fn the_support_names_follow_the_app() {
    assert_eq!(env_variable("meu-app"), "MEU_APP_LOG");
    assert_eq!(override_file("meu_app"), "meu_app.log.toml");
    assert_eq!(user_data_folder("meu_app"), "%APPDATA%\\meu_app\\logs");
}

#[test]
fn a_name_that_cannot_rotate_or_holds_a_folder_is_refused() {
    let base = LogOptions::default();
    let bad = |f: fn(&mut LogOptions)| {
        let mut o = base.clone();
        f(&mut o);
        check(&o).unwrap_err()
    };
    assert!(bad(|o| o.file_name = "  ".into()).contains("empty"));
    assert!(bad(|o| o.file_name = "logs/{date}.log".into()).contains("folder"));
    assert!(bad(|o| o.file_name = "{app}.log".into()).contains("{date}"));
    assert!(
        bad(|o| {
            o.new_file = NewFile::EveryRun;
            o.file_name = "{app}_{date}.log".into();
        })
        .contains("{time}")
    );
    assert!(bad(|o| o.folder = LogFolder::Custom(" ".into())).contains("custom"));
    assert!(bad(|o| o.keep = 0).contains("keep"));
}

#[test]
fn the_sample_override_carries_the_release_level_and_the_folder() {
    let o = LogOptions {
        folder: LogFolder::Custom("D:\\logs\\meu_app".into()),
        ..LogOptions::default()
    };
    let s = sample_override("meu_app", &o);
    assert!(s.contains("level = \"warn\"\n"), "{s}");
    assert!(s.contains("folder = \"D:\\\\logs\\\\meu_app\"\n"), "{s}");
    assert!(s.contains("file_name = \"{app}_{date}.log\"\n"), "{s}");
    assert!(s.contains("MEU_APP_LOG"), "{s}");
}
