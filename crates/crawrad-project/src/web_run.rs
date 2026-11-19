//! Web application running and packaging for WebAssembly browser targets.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{JoinHandle, spawn};

use crawrad_note::note;

/// `cargo metadata` of the workspace at `root` (`call` says whether with dependencies).
fn metadata(call: crawrad_os::process::CargoCall) -> Result<crate::workspace::Metadata, String> {
    let out = crawrad_os::process::wait(&mut call.command())?;
    if !out.success {
        return Err(out
            .report()
            .unwrap_or_else(|| "cargo metadata failed".into()));
    }
    serde_json::from_str(&out.stdout).map_err(|e| format!("parsing cargo metadata failed: {e}"))
}

#[note("web-run-is-web-project")]
pub fn is_web_project(root: &Path, package: &str) -> bool {
    // A workspace root has no `[package]`: the member's own manifest says it, and cargo knows which.
    metadata(crawrad_os::process::CargoCall::metadata(root)).is_ok_and(|m| {
        m.packages.iter().any(|p| {
            p.name == package && p.metadata.pointer("/crawrad/platform") == Some(&"web".into())
        })
    })
}

/// Whether the member at `member` is a web project, from its own `Cargo.toml`
/// (no `cargo metadata`: the Toolbox and the Designer ask while they draw).
#[note("web-run-is-web-member")]
pub fn is_web_member(member: &Path) -> bool {
    std::fs::read_to_string(member.join("Cargo.toml"))
        .ok()
        .and_then(|t| t.parse::<toml::Table>().ok())
        .and_then(|t| {
            let platform = t
                .get("package")?
                .get("metadata")?
                .get("crawrad")?
                .get("platform")?;
            Some(platform.as_str()? == "web")
        })
        .unwrap_or(false)
}

#[note("web-run-index-html")]
pub fn index_html(wasm: &str) -> String {
    format!(
        "<!DOCTYPE html>\n\
<html>\n\
<head>\n\
<meta charset='utf-8'>\n\
<meta name='viewport' content='width=device-width, initial-scale=1.0, user-scalable=no'>\n\
<title>{wasm}</title>\n\
<script type='module'>\n\
import {{WasmWebGL}} from './makepad_platform/web_gl.js'\n\
const wasm = await WasmWebGL.fetch_and_instantiate_wasm('./{wasm}.wasm');\n\
class MyWasmApp {{\n    \
constructor(wasm) {{\n        \
let canvas = document.getElementsByClassName('full_canvas')[0];\n        \
this.webgl = new WasmWebGL(wasm, this, canvas);\n    \
}}\n\
}}\n\
let app = new MyWasmApp(wasm);\n\
</script>\n\
<link rel='stylesheet' type='text/css' href='./makepad_platform/full_canvas.css'>\n\
</head>\n\
<body>\n\
<canvas class='full_canvas'></canvas>\n\
<div class='canvas_loader'><div>Loading..</div></div>\n\
</body>\n\
</html>\n"
    )
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

#[note("web-run-build-package")]
pub fn build_package(root: &Path, package: &str) -> Result<PathBuf, String> {
    // With dependencies: their `resources/` folders go into the page.
    let call = crawrad_os::process::CargoCall::new(["metadata", "--format-version", "1"], root);
    let metadata = metadata(call)?;
    let target_dir = &metadata.target_directory;
    let args = [
        "build",
        "--target",
        "wasm32-unknown-unknown",
        "--release",
        "-p",
        package,
    ];
    let outcome =
        crawrad_os::process::wait(&mut crawrad_os::process::CargoCall::new(args, root).command())?;
    if !outcome.success {
        return Err(outcome
            .report()
            .unwrap_or_else(|| "wasm build failed".to_string()));
    }

    let package_dir = target_dir.join("crawrad-web").join(package);
    std::fs::create_dir_all(&package_dir).map_err(|e| format!("cannot create package dir: {e}"))?;

    let wasm_file_name = format!("{}.wasm", package.replace('-', "_"));
    let wasm_src = target_dir
        .join("wasm32-unknown-unknown")
        .join("release")
        .join(&wasm_file_name);
    let wasm_dst = package_dir.join(format!("{package}.wasm"));
    if !wasm_src.exists() {
        return Err(format!("wasm artifact not found at {}", wasm_src.display()));
    }
    std::fs::copy(&wasm_src, &wasm_dst).map_err(|e| format!("cannot copy wasm artifact: {e}"))?;

    for p in &metadata.packages {
        let Some(pkg_dir) = p.manifest_path.parent() else {
            continue;
        };

        if p.name == "makepad-wasm-bridge" {
            let bridge_src = pkg_dir.join("src").join("wasm_bridge.js");
            if bridge_src.exists() {
                let bridge_dst_dir = package_dir.join("makepad_wasm_bridge");
                std::fs::create_dir_all(&bridge_dst_dir)
                    .map_err(|e| format!("cannot create makepad_wasm_bridge dir: {e}"))?;
                std::fs::copy(&bridge_src, bridge_dst_dir.join("wasm_bridge.js"))
                    .map_err(|e| format!("cannot copy wasm_bridge.js: {e}"))?;
            }
        }

        if p.name == "makepad-platform" {
            let web_src_dir = pkg_dir.join("src").join("os").join("web");
            if web_src_dir.is_dir() {
                let platform_dst_dir = package_dir.join("makepad_platform");
                std::fs::create_dir_all(&platform_dst_dir)
                    .map_err(|e| format!("cannot create makepad_platform dir: {e}"))?;
                let files = [
                    "web_gl.js",
                    "web.js",
                    "web_worker.js",
                    "audio_worklet.js",
                    "auto_reload.js",
                    "full_canvas.css",
                ];
                for f in files {
                    let src_file = web_src_dir.join(f);
                    if src_file.exists() {
                        std::fs::copy(&src_file, platform_dst_dir.join(f))
                            .map_err(|e| format!("cannot copy {f}: {e}"))?;
                    }
                }
            }
        }

        let res_dir = pkg_dir.join("resources");
        if res_dir.is_dir() {
            let pkg_ident = p.name.replace('-', "_");
            let res_dst = package_dir.join(&pkg_ident).join("resources");
            copy_dir_all(&res_dir, &res_dst)
                .map_err(|e| format!("cannot copy resources for {}: {e}", p.name))?;
        }
    }

    let html = index_html(package);
    std::fs::write(package_dir.join("index.html"), html)
        .map_err(|e| format!("cannot write index.html: {e}"))?;

    Ok(package_dir)
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()).unwrap_or("") {
        "html" | "htm" => "text/html",
        "wasm" => "application/wasm",
        "js" | "mjs" => "text/javascript",
        "css" => "text/css",
        "ttf" => "application/ttf",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml",
        _ => "application/octet-stream",
    }
}

fn handle_connection(mut stream: TcpStream, dir: &Path) {
    let mut buf = [0u8; 4096];
    let Ok(n) = stream.read(&mut buf) else {
        return;
    };
    if n == 0 {
        return;
    }
    let req_str = String::from_utf8_lossy(&buf[..n]);
    let Some(first_line) = req_str.lines().next() else {
        return;
    };
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let raw_path = parts.next().unwrap_or("");

    let clean_path = raw_path
        .split('?')
        .next()
        .unwrap_or("")
        .split('#')
        .next()
        .unwrap_or("");

    let rel_path = clean_path.trim_start_matches('/');
    let rel_path = if rel_path.is_empty() {
        "index.html"
    } else {
        rel_path
    };
    // Only plain names below the folder: `..`, `C:/…` and `\\server` would make `join` leave it.
    // `:` is refused everywhere, so the answer does not depend on the host OS.
    let inside = !rel_path.contains(['\\', ':'])
        && Path::new(rel_path)
            .components()
            .all(|c| matches!(c, std::path::Component::Normal(_)));
    if !inside {
        let resp = "HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        let _ = stream.write_all(resp.as_bytes());
        return;
    }

    let target_file = dir.join(rel_path);
    if !target_file.is_file() {
        let resp = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        let _ = stream.write_all(resp.as_bytes());
        return;
    }

    let Ok(bytes) = std::fs::read(&target_file) else {
        let resp =
            "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        let _ = stream.write_all(resp.as_bytes());
        return;
    };

    let mime = content_type(&target_file);
    let header = format!(
        "HTTP/1.1 200 OK\r\n\
Content-Type: {mime}\r\n\
Content-Length: {}\r\n\
Cross-Origin-Embedder-Policy: require-corp\r\n\
Cross-Origin-Opener-Policy: same-origin\r\n\
Connection: close\r\n\
\r\n",
        bytes.len()
    );
    let _ = stream.write_all(header.as_bytes());
    if method == "GET" {
        let _ = stream.write_all(&bytes);
    }
}

#[note("web-run-webserver")]
pub struct WebServer {
    port: u16,
    running: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl WebServer {
    #[note("web-run-webserver-start")]
    pub fn start(dir: PathBuf) -> std::io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = Arc::clone(&running);

        let thread = spawn(move || {
            while running_clone.load(Ordering::SeqCst) {
                let Ok((stream, _)) = listener.accept() else {
                    continue;
                };
                if !running_clone.load(Ordering::SeqCst) {
                    break;
                }
                let dir_clone = dir.clone();
                spawn(move || {
                    handle_connection(stream, &dir_clone);
                });
            }
        });

        Ok(Self {
            port,
            running,
            thread: Some(thread),
        })
    }

    #[note("web-run-webserver-url")]
    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}/", self.port)
    }
}

impl Drop for WebServer {
    fn drop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        let _ = TcpStream::connect(format!("127.0.0.1:{}", self.port));
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

/// The packaging task's answer: the served folder and the package, or cargo's error.
type Packaged = Result<(PathBuf, String), String>;

#[note("web-run-webrun")]
#[derive(Default)]
pub struct WebRun {
    tasks: crawrad_os::tasks::TaskManager<Packaged>,
    server: Option<WebServer>,
}

impl WebRun {
    #[note("web-run-webrun-start")]
    pub fn start(&mut self, root: &Path, package: &str) -> String {
        let (root, owned) = (root.to_path_buf(), package.to_string());
        self.tasks.spawn("web-run", move |reporter| {
            reporter.done(build_package(&root, &owned).map(|dir| (dir, owned)));
        });
        format!("building {package} for the browser…")
    }

    #[note("web-run-webrun-pump")]
    pub fn pump(&mut self) -> Option<(String, Option<String>)> {
        let mut answer = None;
        for (_, event) in self.tasks.drain() {
            let served = match event {
                crawrad_os::tasks::TaskEvent::Done(Ok((dir, package))) => WebServer::start(dir)
                    .map(|s| (s, package))
                    .map_err(|e| e.to_string()),
                crawrad_os::tasks::TaskEvent::Done(Err(e))
                | crawrad_os::tasks::TaskEvent::Failed(e) => Err(e),
                crawrad_os::tasks::TaskEvent::Progress(_) => continue,
            };
            answer = Some(match served {
                Ok((server, package)) => {
                    let url = server.url();
                    // Replacing the server stops the one a previous Run left up.
                    self.server = Some(server);
                    (
                        format!("built — {package} running in the browser at {url}"),
                        Some(url),
                    )
                }
                Err(e) => (format!("web build failed: {e}"), None),
            });
        }
        answer
    }

    #[note("web-run-webrun-serving")]
    pub fn serving(&self) -> bool {
        self.server.is_some()
    }

    #[note("web-run-webrun-stop")]
    pub fn stop(&mut self) -> bool {
        self.server.take().is_some()
    }
}

#[cfg(test)]
#[path = "test/web_run/test.rs"]
mod test;
