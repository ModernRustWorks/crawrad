use super::*;
use std::io::{Read, Write};
use std::net::TcpStream;

fn send_request(url: &str, path: &str) -> String {
    let addr = url.trim_start_matches("http://").trim_end_matches('/');
    let mut stream = TcpStream::connect(addr).expect("connect to test server");
    let req = format!("GET {path} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n");
    stream.write_all(req.as_bytes()).expect("write request");
    let mut response = String::new();
    stream.read_to_string(&mut response).expect("read response");
    response
}

#[test]
fn is_web_project_true_and_false() {
    let dir = std::env::temp_dir().join(format!("crawrad-test-web-project-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let member = dir.join("demo");
    std::fs::create_dir_all(member.join("src")).unwrap();
    std::fs::write(
        member.join("src").join("main.rs"),
        "fn main() {}
",
    )
    .unwrap();
    // The IDE asks from the workspace root, which has no `[package]` of its own.
    std::fs::write(
        dir.join("Cargo.toml"),
        "[workspace]
members = [\"demo\"]
resolver = \"3\"
",
    )
    .unwrap();

    let non_web_cargo = "[package]
name = \"demo\"
version = \"0.1.0\"
edition = \"2024\"
";
    std::fs::write(member.join("Cargo.toml"), non_web_cargo).unwrap();
    assert!(!is_web_project(&dir, "demo"));

    let web_cargo = format!(
        "{non_web_cargo}
[package.metadata.crawrad]
platform = \"web\"
"
    );
    std::fs::write(member.join("Cargo.toml"), web_cargo).unwrap();
    assert!(is_web_project(&dir, "demo"));
    assert!(!is_web_project(&dir, "other"));

    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn index_html_names_wasm_target() {
    let html = index_html("my_app");
    assert!(html.contains("<title>my_app</title>"));
    assert!(html.contains("fetch_and_instantiate_wasm('./my_app.wasm')"));
}

#[test]
fn server_responses_and_security_headers() {
    let dir = std::env::temp_dir().join(format!("crawrad-test-server-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    std::fs::write(dir.join("index.html"), "<h1>Hello</h1>").unwrap();
    std::fs::write(dir.join("x.wasm"), [0x00, 0x61, 0x73, 0x6d]).unwrap();

    let server = WebServer::start(dir.clone()).expect("server starts");
    let url = server.url();

    // 1. GET /index.html
    let res_html = send_request(&url, "/index.html");
    assert!(res_html.starts_with("HTTP/1.1 200 OK"));
    assert!(res_html.contains("Content-Type: text/html"));
    assert!(res_html.contains("Cross-Origin-Embedder-Policy: require-corp"));
    assert!(res_html.contains("Cross-Origin-Opener-Policy: same-origin"));
    assert!(res_html.contains("<h1>Hello</h1>"));

    // 2. GET /x.wasm
    let res_wasm = send_request(&url, "/x.wasm");
    assert!(res_wasm.starts_with("HTTP/1.1 200 OK"));
    assert!(res_wasm.contains("Content-Type: application/wasm"));

    // 3. GET /missing
    let res_missing = send_request(&url, "/missing");
    assert!(res_missing.starts_with("HTTP/1.1 404 Not Found"));

    // 4. GET /../secret refused
    let res_parent = send_request(&url, "/../secret");
    assert!(res_parent.starts_with("HTTP/1.1 403 Forbidden"));

    // 5. An absolute path would make `join` drop the served folder.
    for escape in ["/C:/Windows/win.ini", "/a/../../secret", "/a\\..\\secret"] {
        let res = send_request(&url, escape);
        assert!(res.starts_with("HTTP/1.1 403 Forbidden"), "{escape}: {res}");
    }

    drop(server);
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn web_run_reports_a_failed_package_and_has_nothing_to_stop() {
    let mut run = WebRun::default();
    let dir = std::env::temp_dir().join(format!(
        "crawrad-test-web-run-missing-{}",
        std::process::id()
    ));
    assert_eq!(run.start(&dir, "demo"), "building demo for the browser…");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
    let answer = loop {
        if let Some(answer) = run.pump() {
            break answer;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "the packaging never ended"
        );
        std::thread::sleep(std::time::Duration::from_millis(50));
    };
    assert!(answer.0.starts_with("web build failed: "), "{answer:?}");
    assert_eq!(answer.1, None);
    assert!(!run.stop());
}

#[test]
fn is_web_member_reads_the_member_manifest() {
    let dir = std::env::temp_dir().join(format!("crawrad-web-member-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let manifest = dir.join("Cargo.toml");
    std::fs::write(
        &manifest,
        "[package]\nname = \"w\"\n\n[package.metadata.crawrad]\nplatform = \"web\"\n",
    )
    .unwrap();
    assert!(super::is_web_member(&dir));
    std::fs::write(&manifest, "[package]\nname = \"d\"\n").unwrap();
    assert!(!super::is_web_member(&dir));
    let _ = std::fs::remove_dir_all(&dir);
}
