//! What `project.build` runs: `cargo check`, `build` or `test` in the
//! project folder, and its output as JSON. Pure of the window, so it lives
//! here; the shell only finds the folder.

use std::path::Path;

use crawrad_note::note;
use serde_json::{Value, json};

#[note("cargo-tool-subcommand")]
pub fn subcommand(args: &Value) -> &'static str {
    match args.get("command").and_then(Value::as_str) {
        Some("build") => "build",
        Some("test") => "test",
        _ => "check",
    }
}

#[note("cargo-tool-run")]
pub fn run(dir: &Path, args: &Value) -> Result<(Value, String, bool), String> {
    let command = subcommand(args);
    let mut cmd = crawrad_os::process::no_console("cargo");
    cmd.arg(command);
    if args
        .get("release")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        cmd.arg("--release");
    }
    cmd.current_dir(dir);
    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run cargo: {e}"))?;
    let success = output.status.success();
    let code = output.status.code().unwrap_or(-1);
    let answer = json!({
        "success": success,
        "exit_code": code,
        "stdout": String::from_utf8_lossy(&output.stdout),
        "stderr": String::from_utf8_lossy(&output.stderr),
        "directory": dir.display().to_string()
    });
    let said = format!("cargo {command} finished with exit code {code}");
    Ok((answer, said, success))
}

#[note("cargo-tool-required")]
pub fn required<'a>(args: &'a Value, tool: &str, keys: &[&str]) -> Result<Vec<&'a str>, String> {
    keys.iter()
        .map(|k| {
            args.get(*k)
                .and_then(Value::as_str)
                .ok_or_else(|| format!("Missing required parameter '{k}' for '{tool}'"))
        })
        .collect()
}

#[note("cargo-tool-nonempty")]
pub fn nonempty<'a>(args: &'a Value, key: &str) -> Option<&'a str> {
    let v = args.get(key)?.as_str()?.trim();
    (!v.is_empty()).then_some(v)
}

#[note("cargo-tool-ran")]
pub type Ran = Result<(Value, String, bool), String>;

fn cargo(dir: &Path, args: &[&str]) -> std::io::Result<std::process::Output> {
    let mut cmd = crawrad_os::process::no_console("cargo");
    cmd.args(args).current_dir(dir);
    cmd.output()
}

#[note("cargo-tool-compiler-messages")]
pub fn compiler_messages(stdout: &[u8]) -> Vec<Value> {
    String::from_utf8_lossy(stdout)
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .filter(|v| v.get("reason").and_then(Value::as_str) == Some("compiler-message"))
        .filter_map(|v| v.get("message").cloned())
        .collect()
}

#[note("cargo-tool-summary")]
pub fn summary(msg: &Value) -> Value {
    let span = msg.pointer("/spans/0");
    let at = |k: &str| span.and_then(|s| s.get(k)).cloned();
    json!({
        "level": msg.get("level").and_then(Value::as_str).unwrap_or("info"),
        "message": msg.get("message").and_then(Value::as_str).unwrap_or(""),
        "file": at("file_name"),
        "line": at("line_start"),
        "column": at("column_start"),
        "rendered": msg.get("rendered"),
    })
}

#[note("cargo-tool-check")]
pub fn check(dir: &Path) -> Ran {
    let out = cargo(dir, &["check", "--message-format=json"])
        .map_err(|e| format!("Failed to run cargo check: {e}"))?;
    let diagnostics = compiler_messages(&out.stdout);
    let (success, n) = (out.status.success(), diagnostics.len());
    let answer = json!({
        "success": success,
        "exit_code": out.status.code().unwrap_or(-1),
        "diagnostics": diagnostics,
        "diagnostics_count": n,
        "raw_stderr": String::from_utf8_lossy(&out.stderr),
    });
    let said = format!("cargo check finished (diagnostics: {n}, success: {success}).");
    Ok((answer, said, success))
}

#[note("cargo-tool-build")]
pub fn build(dir: &Path, args: &Value) -> Ran {
    let release = args.get("release").and_then(Value::as_bool) == Some(true);
    let argv: &[&str] = if release {
        &["build", "--release"]
    } else {
        &["build"]
    };
    let out = cargo(dir, argv).map_err(|e| format!("Failed to run cargo build: {e}"))?;
    let (success, code) = (out.status.success(), out.status.code().unwrap_or(-1));
    let answer = json!({
        "success": success,
        "exit_code": code,
        "stdout": String::from_utf8_lossy(&out.stdout),
        "stderr": String::from_utf8_lossy(&out.stderr),
    });
    Ok((
        answer,
        format!("cargo build finished with exit code {code}."),
        success,
    ))
}

#[note("cargo-tool-diagnostics")]
pub fn diagnostics(dir: &Path) -> Ran {
    let out = cargo(dir, &["check", "--message-format=json"])
        .map_err(|e| format!("Failed to collect diagnostics: {e}"))?;
    let diagnostics: Vec<Value> = compiler_messages(&out.stdout).iter().map(summary).collect();
    let n = diagnostics.len();
    let answer = json!({ "diagnostics": diagnostics, "count": n });
    Ok((answer, format!("Collected {n} compiler diagnostics."), true))
}

#[note("cargo-tool-dependencies")]
pub fn dependencies(toml: &str) -> std::collections::HashMap<String, String> {
    let mut deps = std::collections::HashMap::new();
    let mut inside = false;
    for line in toml.lines().map(str::trim) {
        if line.starts_with('[') {
            inside = line == "[dependencies]";
        } else if inside
            && !line.is_empty()
            && !line.starts_with('#')
            && let Some((k, v)) = line.split_once('=')
        {
            deps.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    deps
}

#[note("cargo-tool-add-dependency")]
pub fn add_dependency(dir: &Path, crate_name: &str, args: &Value) -> Ran {
    let mut argv = vec!["add", crate_name];
    for f in args
        .get("features")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        if let Some(s) = f.as_str() {
            argv.extend(["--features", s]);
        }
    }
    if args.get("dev").and_then(Value::as_bool) == Some(true) {
        argv.push("--dev");
    }
    let out = cargo(dir, &argv).map_err(|e| format!("Failed to run cargo add: {e}"))?;
    let success = out.status.success();
    let answer = json!({
        "success": success,
        "crate": crate_name,
        "stdout": String::from_utf8_lossy(&out.stdout),
        "stderr": String::from_utf8_lossy(&out.stderr),
    });
    let said = format!("cargo add {crate_name} finished (success: {success}).");
    Ok((answer, said, success))
}

#[note("cargo-tool-run-command")]
pub fn run_command(dir: &Path, args: &Value) -> std::process::Command {
    let mut cmd = crawrad_os::process::no_console("cargo");
    cmd.arg("run");
    if let Some(extra) = args.get("args").and_then(Value::as_array) {
        cmd.arg("--");
        cmd.args(extra.iter().filter_map(Value::as_str));
    }
    cmd.current_dir(dir);
    cmd
}

#[note("cargo-tool-form-unit")]
pub fn form_unit(name: &str) -> String {
    format!(
        "//! Form unit for {name}\nuse crawrad_runtime::prelude::*;\n\npub struct {name} {{\n// Form components and handlers\n}}\n\nimpl {name} {{\npub fn new() -> Self {{\nSelf {{}}\n}}\n}}\n"
    )
}

#[cfg(test)]
#[path = "test/cargo_tool/test.rs"]
mod tests;
