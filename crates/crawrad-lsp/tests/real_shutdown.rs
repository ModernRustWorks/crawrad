//! Stopping the REAL rust-analyzer leaves nothing running.
//!
//! Runs only with `CRAWRAD_RUST_ANALYZER=<path to rust-analyzer>` on Windows;
//! without it the test passes and proves NOTHING.
//!
//! The project has a `build.rs` that sleeps 30 s, so the `cargo check`
//! rust-analyzer starts on save (flycheck) is surely running when the
//! client stops. The processes are counted from OUTSIDE (the system's process
//! list, by command line and parent), not through the client's own job.
#![cfg(windows)]

use std::path::PathBuf;
use std::time::{Duration, Instant};

use crawrad_lsp::LspClient;

const TOKEN: &str = "ra-shutdown-367";

/// `name pid` of every live process descending from rust-analyzer(s)
/// started by this test, at any depth, plus the `pids` already known (once
/// the server is dead its orphans have no living ancestor to walk from).
fn ours(known: &[u32]) -> Vec<String> {
    let me = std::process::id();
    let known: Vec<String> = known.iter().map(u32::to_string).collect();
    let script = format!(
        "$all = Get-CimInstance Win32_Process;          $set = @{{}}; foreach ($p in $all) {{ if ($p.Name -like 'rust-analyzer*' -and $p.ParentProcessId -eq {me}) {{ $set[$p.ProcessId] = 1 }} }};          foreach ($k in @({known})) {{ if ($k -gt 0) {{ $set[[uint32]$k] = 1 }} }};          do {{ $n = $set.Count; foreach ($p in $all) {{ if ($set.ContainsKey($p.ParentProcessId) -and $p.ProcessId -ne $p.ParentProcessId -and $p.ProcessId -gt 4) {{ $set[$p.ProcessId] = 1 }} }} }} while ($set.Count -ne $n);          foreach ($p in $all) {{ if ($set.ContainsKey($p.ProcessId)) {{ \"$($p.Name) $($p.ProcessId)\" }} }}",
        known = if known.is_empty() {
            "-1".to_string()
        } else {
            known.join(",")
        }
    );
    let out = std::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect()
}

fn pids(list: &[String]) -> Vec<u32> {
    list.iter()
        .filter_map(|l| l.rsplit(' ').next()?.parse().ok())
        .collect()
}

fn cleanup(left: &[String]) {
    for pid in left.iter().filter_map(|l| l.split(' ').nth(1)) {
        let _ = std::process::Command::new("taskkill")
            .args(["/F", "/PID", pid])
            .output();
    }
}

#[test]
fn stopping_the_server_leaves_no_cargo_behind() {
    let Ok(server) = std::env::var("CRAWRAD_RUST_ANALYZER") else {
        eprintln!("CRAWRAD_RUST_ANALYZER not set: this test proves NOTHING");
        return;
    };
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(TOKEN);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"ra_shutdown\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("build.rs"),
        "fn main() { std::thread::sleep(std::time::Duration::from_secs(30)); }\n",
    )
    .unwrap();
    let file = dir.join("src").join("main.rs");
    let main = "fn main() {}\n";
    std::fs::write(&file, main).unwrap();

    let client = LspClient::spawn(Some(PathBuf::from(server)), &dir).expect("spawn");
    client.did_open(&file, "rust", 1, main).unwrap();

    // flycheck runs on save; wait until the build script — a grandchild of
    // rust-analyzer, under cargo — is running its 30 s sleep
    let started = Instant::now();
    let mut seen = Vec::new();
    while !seen.iter().any(|l: &String| l.starts_with("build-script")) {
        assert!(
            started.elapsed() < Duration::from_secs(90),
            "cargo check never started: {seen:?}"
        );
        let _ = client.did_save(&file);
        std::thread::sleep(Duration::from_secs(1));
        seen = ours(&[]);
        eprintln!("{:>3}s {seen:?}", started.elapsed().as_secs());
    }
    eprintln!("before shutdown: {seen:?}");

    let stopping = Instant::now();
    client.shutdown();
    let took = stopping.elapsed();
    std::thread::sleep(Duration::from_millis(500));
    let left = ours(&pids(&seen));
    eprintln!("shutdown took {took:?}; left: {left:?}");
    cleanup(&left);
    assert!(left.is_empty(), "orphans after shutdown: {left:?}");
    assert!(took < Duration::from_secs(4), "shutdown took {took:?}");
}
