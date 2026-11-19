//! The splice of the edit tools: ranges, appends, the final newline, refusals.

use super::{
    ActiveFileParams, absolute_path_arg, active_file_json, busy, resolve_path, selection_range,
    splice, wrong_view,
};
use serde_json::json;

const TEXT: &str = "a\nb\nc\n";

#[test]
fn a_selection_range_is_one_based_and_counts_characters() {
    let range = selection_range("fn main() {\n    let ção = 1;\n}\n", [(1, 8), (1, 14)]);
    assert_eq!(range["start"], json!({ "line": 2, "column": 9 }));
    // "ção" is 3 characters in 5 bytes: byte 14 is column 13, not 15
    assert_eq!(range["end"], json!({ "line": 2, "column": 13 }));
    let past = selection_range("a\n", [(0, 0), (5, 3)]);
    assert_eq!(past["end"], json!({ "line": 6, "column": 4 }));
}

#[test]
fn a_range_is_replaced_and_the_final_newline_kept() {
    let args = json!({ "start_line": 2, "end_line": 2, "new_text": "B" });
    assert_eq!(splice(TEXT, &args).unwrap(), "a\nB\nc\n");
    let args = json!({ "start_line": 1, "end_line": 3, "new_text": "x\n" });
    assert_eq!(splice(TEXT, &args).unwrap(), "x\n");
    let args = json!({ "start_line": 2, "end_line": 2, "new_text": "B" });
    assert_eq!(splice("a\nb\nc", &args).unwrap(), "a\nB\nc");
}

#[test]
fn past_the_end_appends_and_a_long_end_is_clamped() {
    let args = json!({ "start_line": 4, "end_line": 4, "new_text": "d" });
    assert_eq!(splice(TEXT, &args).unwrap(), "a\nb\nc\nd");
    let args = json!({ "start_line": 3, "end_line": 99, "new_text": "C" });
    assert_eq!(splice(TEXT, &args).unwrap(), "a\nb\nC");
}

#[test]
fn content_replaces_everything_and_bad_arguments_are_refused() {
    assert_eq!(splice(TEXT, &json!({ "content": "z" })).unwrap(), "z");
    let bad = json!({ "start_line": 0, "end_line": 1, "new_text": "" });
    assert_eq!(splice(TEXT, &bad).unwrap_err().0, "invalid-line-range");
    let bad = json!({ "start_line": 3, "end_line": 2, "new_text": "" });
    assert_eq!(splice(TEXT, &bad).unwrap_err().0, "invalid-line-range");
    assert_eq!(splice(TEXT, &json!({})).unwrap_err().0, "invalid-params");
    assert!(busy("src/a.rs").contains("'src/a.rs' has a pending diff review"));
}

#[test]
fn resolve_path_prefers_args_then_active_then_fails() {
    let active = std::path::Path::new("src/main.rs");
    let with_path = json!({ "path": "src/lib.rs" });
    assert_eq!(
        resolve_path(&with_path, Some(active)).unwrap(),
        "src/lib.rs"
    );

    let empty = json!({});
    assert_eq!(resolve_path(&empty, Some(active)).unwrap(), "src/main.rs");

    let (code, msg) = resolve_path(&empty, None).unwrap_err();
    assert_eq!(code, "no-file");
    assert_eq!(
        msg,
        "No file specified and no active file currently in editor."
    );
}

#[test]
fn active_file_json_formats_values_and_summary() {
    let params = ActiveFileParams {
        path: Some("src/lib.rs"),
        is_dirty: true,
        is_editable: true,
        center_view: "code",
        tab_file: Some("src/lib.rs"),
        read_only: false,
        caret: Some((10, 4)),
        status: Some("Ready"),
        breadcrumb: vec!["src".into(), "lib.rs".into()],
        disk_changed: false,
        deleted_on_disk: false,
    };
    let (val, said) = active_file_json(params);
    assert_eq!(val["path"], "src/lib.rs");
    assert_eq!(val["is_dirty"], true);
    assert_eq!(val["caret"]["line"], 10);
    assert_eq!(val["caret"]["byte"], 4);
    assert_eq!(said, "Active editor file: 'src/lib.rs' (dirty: true)");

    let none_params = ActiveFileParams {
        path: None,
        is_dirty: false,
        is_editable: false,
        center_view: "welcome",
        tab_file: None,
        read_only: true,
        caret: None,
        status: None,
        breadcrumb: vec![],
        disk_changed: false,
        deleted_on_disk: false,
    };
    let (none_val, none_said) = active_file_json(none_params);
    assert!(none_val["path"].is_null());
    assert_eq!(none_said, "No file is currently active in the editor.");
}

#[test]
fn a_tool_sent_to_the_other_view_is_told_which_view_to_open() {
    use crawrad_actions::ViewAffinity::{Any, Code, Design};
    assert_eq!(wrong_view(Design).0, "wrong-view-code");
    assert!(
        wrong_view(Design)
            .1
            .starts_with("The form designer is not active")
    );
    assert_eq!(wrong_view(Code).0, "wrong-view-design");
    assert!(
        wrong_view(Code)
            .1
            .starts_with("The code editor is not active")
    );
    assert_eq!(wrong_view(Any).0, "wrong-view");
}

#[test]
fn absolute_path_arg_resolves_relative_paths_against_bases() {
    let temp = std::env::temp_dir().join(format!("crawrad_mcp_test_abs_{}", std::process::id()));
    let root = temp.join("root");
    let member = temp.join("member");
    let _ = std::fs::remove_dir_all(&temp);
    std::fs::create_dir_all(root.join("src")).expect("root src");
    std::fs::create_dir_all(member.join("src")).expect("member src");

    let root_file = root.join("src/root_only.rs");
    std::fs::write(&root_file, "// root").expect("write root file");
    let member_file = member.join("src/member_only.rs");
    std::fs::write(&member_file, "// member").expect("write member file");

    let bases = [Some(root.clone()), None, Some(member.clone())];

    // relative path that exists under the root -> root joined
    let args_root = json!({ "path": "src/root_only.rs", "extra": 1 });
    let res_root = absolute_path_arg(&args_root, &bases).expect("root joined");
    assert_eq!(res_root["path"], root_file.display().to_string());
    assert_eq!(res_root["extra"], 1);

    // relative path that exists only under the second base -> second base joined
    let args_member = json!({ "path": "src/member_only.rs" });
    let res_member = absolute_path_arg(&args_member, &bases).expect("second base joined");
    assert_eq!(res_member["path"], member_file.display().to_string());

    // relative path that exists nowhere -> first base joined
    let args_nowhere = json!({ "path": "src/new_file.rs" });
    let res_nowhere = absolute_path_arg(&args_nowhere, &bases).expect("first base joined");
    assert_eq!(
        res_nowhere["path"],
        root.join("src/new_file.rs").display().to_string()
    );

    // absolute path -> None
    let args_abs = json!({ "path": root_file.display().to_string() });
    assert!(absolute_path_arg(&args_abs, &bases).is_none());

    // no path -> None
    let args_no_path = json!({ "other": "value" });
    assert!(absolute_path_arg(&args_no_path, &bases).is_none());

    // empty path -> None
    let args_empty_path = json!({ "path": "" });
    assert!(absolute_path_arg(&args_empty_path, &bases).is_none());

    // non-string path -> None
    let args_non_string = json!({ "path": 42 });
    assert!(absolute_path_arg(&args_non_string, &bases).is_none());

    // empty bases -> None
    let args_rel = json!({ "path": "src/root_only.rs" });
    assert!(absolute_path_arg(&args_rel, &[]).is_none());
    assert!(absolute_path_arg(&args_rel, &[None, None]).is_none());

    let _ = std::fs::remove_dir_all(&temp);
}
