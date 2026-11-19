//! A relative SQLite url never depends on the working directory.

use super::*;

fn temp(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("crawrad-dburl-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

#[test]
fn the_project_file_is_found_when_the_exe_folder_has_none() {
    let (exe, proj) = (temp("exe1"), temp("proj1"));
    std::fs::write(proj.join("contacts.db"), b"x").unwrap();
    let got = resolve_with("sqlite:contacts.db", Some(&exe), &proj);
    assert_eq!(
        got,
        format!(
            "sqlite://{}",
            proj.join("contacts.db")
                .to_string_lossy()
                .replace('\\', "/")
        )
    );
}

#[test]
fn a_file_next_to_the_exe_wins() {
    let (exe, proj) = (temp("exe2"), temp("proj2"));
    std::fs::write(exe.join("contacts.db"), b"x").unwrap();
    std::fs::write(proj.join("contacts.db"), b"x").unwrap();
    let got = resolve_with("sqlite://contacts.db", Some(&exe), &proj);
    assert!(got.contains("crawrad-dburl-exe2"), "{got}");
}

#[test]
fn a_missing_file_names_a_full_path_and_others_pass_through() {
    let exe = temp("exe3");
    let gone = exe.join("no-such-project");
    let got = resolve_with("sqlite:contacts.db", Some(&exe), &gone);
    assert!(
        got.ends_with("/contacts.db") && got.contains("crawrad-dburl-exe3"),
        "{got}"
    );
    for same in ["sqlite::memory:", "postgres://h/db", "file:x?mode=memory"] {
        assert_eq!(resolve_with(same, Some(&exe), &exe), same);
    }
    let abs = format!(
        "sqlite://{}",
        exe.join("a.db").to_string_lossy().replace('\\', "/")
    );
    assert_eq!(resolve_with(&abs, Some(&exe), &exe), abs);
}
