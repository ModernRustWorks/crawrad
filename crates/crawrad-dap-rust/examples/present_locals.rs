//! The Rust presenter against a real adapter, end to end.
//!
//! ```text
//! cargo run -p crawrad-dap-rust --example present_locals -- <lldb-dap> <program> <source file> <line>
//! ```
//!
//! Launches `program` under `lldb-dap`, stops at `source:line`, and prints
//! every local twice: as the adapter sent it, and as the presenter shows it.
//! This is how ADR 0048's measurement is repeated with the product code.

use std::path::Path;
use std::time::Duration;

use crawrad_dap::{DapClient, FrameScoped, ValuePresenter, ValueSource, Variable};
use crawrad_dap_rust::RustPresenter;
use serde_json::{Value, json};

const T: Duration = Duration::from_secs(60);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [adapter, program, source, line] = args.as_slice() else {
        eprintln!("usage: present_locals <lldb-dap> <program> <source file> <line>");
        std::process::exit(2);
    };
    let line: i64 = line.parse()?;
    let presenter = RustPresenter::for_program(Path::new(program));
    println!(
        "type layouts (DWARF or PDB): {}",
        if presenter.has_layouts() {
            "read"
        } else {
            "none"
        }
    );
    let client = DapClient::spawn(Path::new(adapter), &[])?;
    let c = client.connection();
    c.request(
        "initialize",
        json!({ "clientID": "crawrad", "adapterID": "lldb-dap",
        "linesStartAt1": true, "columnsStartAt1": true, "pathFormat": "path" }),
        T,
    )?;
    let launch = c.send_request("launch", json!({ "program": program }))?;
    c.wait_event("initialized", T)?;
    c.request(
        "setBreakpoints",
        json!({ "source": { "path": source }, "breakpoints": [{ "line": line }] }),
        T,
    )?;
    c.request("configurationDone", json!({}), T)?;
    launch.wait(T)?;
    let stopped = c.wait_event("stopped", T)?;
    let thread = stopped.body["threadId"].as_i64().unwrap_or(1);
    let frames = c.request("stackTrace", json!({ "threadId": thread, "levels": 1 }), T)?;
    let frame = &frames["stackFrames"][0];
    let frame_id = frame["id"].as_i64().unwrap_or(0);
    println!(
        "stopped in {} @ line {}",
        frame["name"].as_str().unwrap_or("?"),
        frame["line"]
    );
    let scopes = c.request("scopes", json!({ "frameId": frame_id }), T)?;
    let locals_ref = scopes["scopes"][0]["variablesReference"]
        .as_i64()
        .unwrap_or(0);
    let scoped = FrameScoped {
        source: c,
        frame_id,
    };
    for var in scoped.children(locals_ref)? {
        report(&var, &scoped, &presenter);
        // CRAWRAD_TREE=name,name: the adapter's raw tree of those locals.
        if std::env::var("CRAWRAD_TREE").is_ok_and(|t| t.split(',').any(|n| n == var.name)) {
            tree(&var, &scoped, 1);
        }
    }
    // CRAWRAD_EVAL=expr;expr: what the adapter's evaluator answers for each.
    for expr in std::env::var("CRAWRAD_EVAL").unwrap_or_default().split(';') {
        if !expr.is_empty() {
            match scoped.evaluate(expr, Some(frame_id)) {
                Ok(v) => println!(
                    "eval {expr} = {} ({})",
                    v.value,
                    v.type_name.as_deref().unwrap_or("?")
                ),
                Err(e) => println!("eval {expr} failed: {e}"),
            }
        }
    }
    let _ = c.send_request("disconnect", json!({ "terminateDebuggee": true }));
    Ok(())
}

fn tree(var: &Variable, source: &dyn ValueSource, depth: usize) {
    if var.variables_reference == 0 || depth > 4 {
        return;
    }
    for child in source.children(var.variables_reference).unwrap_or_default() {
        println!(
            "{:indent$}{} = {} ({})",
            "",
            child.name,
            child.value,
            child.type_name.as_deref().unwrap_or("?"),
            indent = depth * 2
        );
        tree(&child, source, depth + 1);
    }
}

fn report(var: &Variable, source: &dyn ValueSource, presenter: &RustPresenter) {
    let shown = presenter.present(var, source);
    println!(
        "{:<10} adapter: {:<45} presenter: {}",
        var.name,
        truncate(&var.value, 45),
        shown.as_deref().unwrap_or("(adapter's text)")
    );
    println!(
        "{:<10} type:    {}",
        "",
        var.type_name.as_deref().unwrap_or("?")
    );
}

fn truncate(s: &str, n: usize) -> String {
    let v: Value = Value::String(s.to_string());
    let s = v.as_str().unwrap_or_default();
    if s.chars().count() > n {
        format!("{}…", s.chars().take(n - 1).collect::<String>())
    } else {
        s.to_string()
    }
}
