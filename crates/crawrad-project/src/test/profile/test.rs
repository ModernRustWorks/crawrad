use super::*;

fn sample() -> Profile {
    Profile {
        window: Some(WindowPlace::new([-8.0, 10.5, 1600.0, 900.0], false)),
        name_width: Some(132.0),
        editor_font: Some(14),
        zoom: Some(1.25),
        grid: Some(5),
        dock: "line one\nline two".into(),
    }
}

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("crawrad-profile-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

#[test]
fn a_profile_reads_back_what_it_wrote() {
    let p = sample();
    assert_eq!(Profile::decode(&p.encode()), Some(p));
}

#[test]
fn a_missing_or_bad_line_falls_back_to_the_default() {
    let text = format!("{HEADER}\nzoom -3\neditor-font big\ngrid 99\nwhat 1\ndock\nd");
    let p = Profile::decode(&text).unwrap();
    assert_eq!(
        (p.zoom, p.editor_font, p.window, p.grid),
        (None, None, None, None)
    );
    assert_eq!(p.dock, "d");
    assert_eq!(Profile::decode("crawrad-desktop 1\ndock\n"), None);
}

#[test]
fn default_is_built_in_and_never_a_file() {
    let dir = scratch("default");
    assert_eq!(Profile::read(&dir, " default "), Ok(Profile::default()));
    assert!(sample().write(&dir, DEFAULT).is_err());
    assert!(Profile::delete(&dir, DEFAULT).is_err());
    assert!(!dir.exists());
}

#[test]
fn profiles_are_saved_listed_and_deleted() {
    let dir = scratch("cycle");
    assert_eq!(names(&dir), [DEFAULT]);
    sample().write(&dir, " wide ").unwrap();
    Profile::default().write(&dir, "Debug").unwrap();
    std::fs::write(dir.join("notes.md"), "").unwrap();
    assert_eq!(names(&dir), [DEFAULT, "Debug", "wide"]);
    assert_eq!(Profile::read(&dir, "wide"), Ok(sample()));
    Profile::delete(&dir, "Debug").unwrap();
    assert_eq!(names(&dir), [DEFAULT, "wide"]);
    assert!(Profile::read(&dir, "Debug").is_err());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_name_cannot_leave_the_folder() {
    for bad in [
        "",
        "  ",
        "../up",
        "a/b",
        "a\\b",
        ".hidden",
        "x:y",
        &"n".repeat(41),
    ] {
        assert!(check_name(bad).is_err(), "{bad:?} was taken as a name");
    }
    assert_eq!(check_name(" Big screen 2.0 "), Ok("Big screen 2.0"));
}

/// A command that failed says why and leaves the active profile alone: `settle`
/// returns before it writes the preferences.
#[test]
fn a_failed_command_only_says_why() {
    let failed = || Err("there is no profile `x`".to_string());
    for c in [Apply("x".into()), Save("x".into()), Delete("x".into())] {
        assert_eq!(settle(&c, failed()), "there is no profile `x`");
    }
}
