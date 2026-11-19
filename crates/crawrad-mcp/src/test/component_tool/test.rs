//! Unit tests for component_tool.

use std::path::{Path, PathBuf};

use serde_json::json;

use crate::component_tool::answer;
use crate::protocol::ToolContent;

fn absolute() -> &'static str {
    if cfg!(windows) {
        "C:/packages/gauge"
    } else {
        "/packages/gauge"
    }
}

#[test]
fn a_missing_or_relative_path_is_refused_before_installing() {
    for args in [
        json!({}),
        json!({ "path": 3 }),
        json!({ "path": "relative/gauge" }),
    ] {
        let (refusal, status) = answer(&args, |_| panic!("install must not run"));
        assert_eq!(status, "error");
        assert_eq!(refusal.data.unwrap()["code"], json!("invalid-arguments"));
    }
}

#[test]
fn an_install_refusal_is_install_failed_with_its_reason() {
    let args = json!({ "path": absolute() });
    let (refusal, status) = answer(&args, |path| {
        assert_eq!(path, Path::new(absolute()));
        Err("'gauge-crawrad' is already installed".into())
    });
    assert_eq!(status, "error");
    let data = refusal.data.unwrap();
    assert_eq!(data["code"], json!("install-failed"));
    assert_eq!(
        data["message"],
        json!("'gauge-crawrad' is already installed")
    );
}

#[test]
fn a_success_reports_the_package_its_components_and_folder() {
    let args = json!({ "path": absolute() });
    let (result, status) = answer(&args, |_| {
        Ok((
            "gauge-crawrad".into(),
            vec!["Gauge".into()],
            PathBuf::from("C:/ide/components/gauge-crawrad"),
        ))
    });
    assert_eq!(status, "applied");
    assert!(!result.is_error);
    let data = result.data.unwrap();
    assert_eq!(data["package"], json!("gauge-crawrad"));
    assert_eq!(data["components"], json!(["Gauge"]));
    assert_eq!(data["folder"], json!("C:/ide/components/gauge-crawrad"));
    assert_eq!(
        result.content[0],
        ToolContent::Text {
            text: "Installed 'gauge-crawrad': 1 component(s) in the palette.".into()
        }
    );
}
