use super::*;
use std::fs;
use std::path::PathBuf;

fn make_temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

#[test]
fn test_is_workspace() {
    let ws = make_temp_dir("test_act_proj_ws");
    fs::write(
        ws.join(MANIFEST),
        "[workspace]\nmembers = [\"app\"]\ndefault-members = [\"app\"]\n",
    )
    .unwrap();
    assert!(is_workspace(&ws));

    let single = make_temp_dir("test_act_proj_single");
    fs::write(
        single.join(MANIFEST),
        "[package]\nname = \"single_app\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    assert!(!is_workspace(&single));

    let empty = make_temp_dir("test_act_proj_empty");
    assert!(!is_workspace(&empty));

    let _ = fs::remove_dir_all(&ws);
    let _ = fs::remove_dir_all(&single);
    let _ = fs::remove_dir_all(&empty);
}

#[test]
fn test_active_member_folder() {
    let ws = make_temp_dir("test_act_member_folder");
    fs::write(
        ws.join(MANIFEST),
        "[workspace]\nmembers = [\"app1\", \"app2\"]\ndefault-members = [\"app2\"]\n",
    )
    .unwrap();
    fs::create_dir_all(ws.join("app1")).unwrap();
    fs::write(
        ws.join("app1").join(MANIFEST),
        "[package]\nname = \"app1\"\n",
    )
    .unwrap();
    fs::create_dir_all(ws.join("app2")).unwrap();
    fs::write(
        ws.join("app2").join(MANIFEST),
        "[package]\nname = \"app2\"\n",
    )
    .unwrap();

    let active = active_member_folder(&ws).expect("active member found");
    assert_eq!(active, ws.join("app2"));

    let single = make_temp_dir("test_act_single_folder");
    fs::write(single.join(MANIFEST), "[package]\nname = \"solo\"\n").unwrap();
    let solo = active_member_folder(&single).expect("single project active");
    assert_eq!(solo, single);

    let _ = fs::remove_dir_all(&ws);
    let _ = fs::remove_dir_all(&single);
}

#[test]
fn test_project_name() {
    let dir = make_temp_dir("test_act_project_name");
    fs::write(dir.join(MANIFEST), "[package]\nname = \"hello_world\"\n").unwrap();
    assert_eq!(project_name(&dir), "hello_world");

    let no_manifest = dir.join("custom_folder");
    fs::create_dir_all(&no_manifest).unwrap();
    assert_eq!(project_name(&no_manifest), "custom_folder");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_active_project_name() {
    let ws = make_temp_dir("test_act_proj_name_ws");
    fs::write(
        ws.join(MANIFEST),
        "[workspace]\nmembers = [\"gui\"]\ndefault-members = [\"gui\"]\n",
    )
    .unwrap();
    fs::create_dir_all(ws.join("gui")).unwrap();
    fs::write(
        ws.join("gui").join(MANIFEST),
        "[package]\nname = \"my_cool_gui\"\n",
    )
    .unwrap();

    assert_eq!(active_project_name(&ws), Some("my_cool_gui".to_string()));
    let _ = fs::remove_dir_all(&ws);
}

#[test]
fn test_window_title() {
    assert_eq!(
        window_title(Some("app"), Some("form_main.crawform")),
        "CrawRAD — app · form_main.crawform"
    );
    assert_eq!(window_title(Some("app"), None), "CrawRAD — app · no form");
    assert_eq!(
        window_title(None, Some("form_main.crawform")),
        "CrawRAD — form_main.crawform"
    );
    assert_eq!(window_title(None, None), "CrawRAD — no form");
}

#[test]
fn test_status_project_text() {
    assert_eq!(status_project_text(Some("app")), "Project: app");
    assert_eq!(status_project_text(None), "");
}
