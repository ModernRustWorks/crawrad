//! **The DISK FIXTURE for shell tests** — path and setup.
//!
//! # Why it moved
//!
//! It used to live inside `main.rs` of the egui shell, and `generator.rs` wrote
//! the debt explicitly in its footer: *"a shared module whose tests only compile
//! from one of its two consumers is a module with the fixture in the wrong place.
//! Closing this means moving `fixture` and `mock_project` into their own file that
//! both roots declare"*.
//!
//! ADR 0032 removed the egui shell, the debt came due, and this is the file that
//! that comment requested. `generator.rs` resumed declaring its tests.
//!
//! The path in `%TEMP%` where a test setup lives — and the reason why there is
//! **only one place** that decides it.
//!
//! The path was FIXED — `%TEMP%\crawrad-creation-inside-other` — and the
//! test **deleted the folder at start** to be born clean. Two suites at the same
//! time (and on this machine there were three worktrees in a single day) delete
//! each other's folder mid-run, leaving false failures: `a folder named 'meu_app'
//! already exists here`, which has nothing to do with what the test measures. False
//! failures are the worst failure mode of a gate, training the team to ignore failures.
//!
//! **This was reproduced, and the number is here for anyone who doubts.** Two copies
//! of the test binary running the same project creation simultaneously, ten times:
//! with the fixed name, **10 of 10 red runs**; with the stamp from this module,
//! **0 of 10**. The entire module, five times: 5 of 5 against 0 of 5.
//!
//! **And collision knows how to use more than one voice — this is the misleading part.**
//! In the ten runs four different phrases appeared, and one of them was
//! `cargo edited another project's Cargo.toml and nobody said so`, i.e.,
//! `created.warning == None`. It is the SAME phrase from an earlier report, which has another
//! cause (CI's `CARGO_TERM_COLOR=always` leaving cargo output colored, and
//! `starts_with("Adding ")` matching nothing). Whoever sees this phrase must first ask
//! whether it is deterministic: the other is, this collision's is not.
//!
//! **The obvious patch is not enough, and this was MEASURED.** Putting
//! `std::process::id()` in the name was what neighbors were doing. On 08/09/2026 there
//! were 1894 `crawrad-*` folders in this `%TEMP%`, and among them PID **21008 appeared twice**,
//! from two distinct runs (16:37 and 17:00) — Windows recycles PIDs, and those two only
//! did not collide because the atomic counter happened to yield different numbers. The PID
//! separates processes **alive at the same time**; it does not separate runs in time, and
//! the old folder remains on disk waiting for whoever inherits the number.
//!
//! Therefore the stamp has three parts, each handling one collision:
//!
//! | part | separates |
//! |---|---|
//! | `process::id()` | two processes running at the same time |
//! | run timestamp | the same PID, recycled later |
//! | atomic counter | two setups from the same process — including in parallel threads |
//!
//! The last one is what `cargo test` exercises: it runs tests in parallel
//! **inside the same binary**, where the PID is identical for all of them.
//!
//! **And cleanup disappeared along with it, which was the other half of the fix.** The
//! `let _ = std::fs::remove_dir_all(&p)` swallowed the error of failing to clean; a path
//! that never existed before has nothing to clean, so there is no error to swallow nor
//! old state to inherit.
//!
//! **This suite lives at the END of the file**, for the reason stated in
//! [`window_starts_maximized`]: `the_folder_and_the_name::name_refusal_is_read_in_dialog_and_creation`
//! splits this `main.rs` at the FIRST `#[cfg(test)]` and treats everything before as "the product".

use crate::generator;
use crawrad_ir::{Document, Rect};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

/// The folder of this setup: newly created, empty, and belonging to no one else.
pub(crate) fn folder(prefix: &str) -> PathBuf {
    static SWEPT: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    SWEPT.get_or_init(|| {
        sweep_stale(
            &std::env::temp_dir(),
            std::time::SystemTime::now(),
            STALE_AFTER,
        );
    });
    setup(std::env::temp_dir().join(format!("crawrad-{prefix}-{}", stamp())))
}

/// How old a `crawrad-*` fixture folder must be before the sweep removes it.
///
/// A week keeps the evidence a red run leaves behind — the failing test is the
/// one that never reaches its own cleanup — and is far longer than any suite
/// running next door, so a live fixture is never in reach.
const STALE_AFTER: std::time::Duration = std::time::Duration::from_secs(7 * 24 * 60 * 60);

/// Removes the fixture FOLDERS under `root` last modified more than `max_age`
/// before `now`. Files and every other name are left alone.
///
/// "Fixture" means the exact shape [`folder`] writes — `crawrad-<prefix>-` and
/// then the [`stamp`] — and NOT any `crawrad-*`. The first cut swept every
/// `crawrad-*`, and on its first run it deleted `%TEMP%\crawrad-darwin-shim`, the
/// gate's cross-check shim holding the only `libclang.dll` on the machine:
/// main went red on `clippy aarch64-apple-darwin`. A name nobody stamped is a
/// name somebody else owns.
///
/// `now` is a parameter so the rule can be gated: Windows will not open a
/// directory for `File::set_times`, so a test cannot age a folder — but it can
/// move the clock forward. Errors are ignored on purpose: a folder another
/// suite is still holding, or one already gone, is not this test's failure.
pub(crate) fn sweep_stale(
    root: &std::path::Path,
    now: std::time::SystemTime,
    max_age: std::time::Duration,
) -> usize {
    let Ok(entries) = std::fs::read_dir(root) else {
        return 0;
    };
    let mut removed = 0;
    for entry in entries.flatten() {
        if !is_stamped_fixture(&entry.file_name().to_string_lossy()) {
            continue;
        }
        let Ok(meta) = entry.metadata() else {
            continue;
        };
        let stale = meta.is_dir()
            && meta
                .modified()
                .ok()
                .and_then(|m| now.duration_since(m).ok())
                .is_some_and(|age| age > max_age);
        if stale && std::fs::remove_dir_all(entry.path()).is_ok() {
            removed += 1;
        }
    }
    removed
}

/// `crawrad-<prefix>-<pid>-<run hex>-<counter>`: the name [`folder`] builds.
fn is_stamped_fixture(name: &str) -> bool {
    let Some(rest) = name.strip_prefix("crawrad-") else {
        return false;
    };
    let mut parts = rest.rsplitn(4, '-');
    let (Some(counter), Some(run), Some(pid), Some(prefix)) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return false;
    };
    let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
    !prefix.is_empty()
        && digits(pid)
        && digits(counter)
        && !run.is_empty()
        && run.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Creates the folder — and **refuses to inherit one that was already on disk**.
///
/// The refusal is not pedantry: it is the difference between discovering the defect
/// here and discovering it seven seconds later, in cargo's voice, saying
/// `a folder named 'meu_app' already exists here, with a project in it`.
/// That phrase sends one hunting for a product defect, and the defect was in the fixture.
/// It is the half of the fix that stated *"a swallowed error is what turns a two-second
/// problem into a wild goose chase"*.
///
/// Separated from [`folder`] because this is what
/// [`fixture_refuses_folder_already_on_disk`] calls: the inherited folder can be
/// simulated on any machine, and waiting for Windows to recycle a PID is not a
/// gate, it is luck.
fn setup(p: PathBuf) -> PathBuf {
    assert!(
        !p.exists(),
        "fixture was about to INHERIT a folder already on disk: {} — stamp \
         failed to separate what it should, and what follows is a false red \
         in a test measuring something else",
        p.display()
    );
    // `expect` and not `let _`: if even creating the folder failed, the test must
    // report it right here, and not fail ten lines below for a reason that
    // points to the wrong place.
    std::fs::create_dir_all(&p).expect("create fixture folder");
    p
}

/// The stamp that separates this setup from all others — adjacent ones,
/// earlier ones, and those of other threads.
pub(crate) fn stamp() -> String {
    // The timestamp is read ONCE per process: what it identifies is the
    // execution run, not the call. What separates call from call is the counter.
    static RUN: std::sync::OnceLock<u128> = std::sync::OnceLock::new();
    static N: AtomicU64 = AtomicU64::new(0);
    let run = *RUN.get_or_init(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos())
    });
    format!(
        "{}-{run:x}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    )
}

const ECHO: &str = "FIXTURE-ECHO=";

/// **Two setups of the same fixture do not collide.** This is the fix in full,
/// and is the assertion that was missing: the defect slipped through because each setup
/// was made ONCE per run, and a lone setup has nothing to collide with.
#[test]
fn two_setups_of_same_fixture_do_not_collide() {
    let a = folder("gate-same-fixture");
    let b = folder("gate-same-fixture");

    assert_ne!(
        a, b,
        "two setups of the same name fell on the same path — issue #45 is back"
    );

    // And it is not just the name being different: what one writes, the other does not see.
    // That is what the concurrent suite truly demands.
    std::fs::write(a.join("meu_app.txt"), "i was here").expect("write to first");
    assert!(
        !b.join("meu_app.txt").exists(),
        "second setup sees what first wrote: {} and {} are the same folder in practice",
        a.display(),
        b.display()
    );
    assert_eq!(
        std::fs::read_dir(&b).expect("read second").count(),
        0,
        "setup was born with garbage inside: {}",
        b.display()
    );
}

/// **Two binary executions do not reuse the path** — even when PID repeats.
///
/// This is the test that `std::process::id()` alone cannot pass, and why the stamp
/// contains the execution timestamp. It runs the test binary ITSELF twice, gets the
/// folder name each child created, and strips the PID from both: what remains must
/// still be different. With a name made only of PID and counter, what remains is the
/// SAME string — and that is exactly the pair of folders Windows hands over when
/// recycling the number.
#[test]
fn two_binary_executions_do_not_reuse_path() {
    let (pid_a, name_a) = echo_from_child();
    let (pid_b, name_b) = echo_from_child();

    assert_ne!(
        name_a, name_b,
        "two executions set up fixture at the same location"
    );

    let without_pid_a = name_a.replace(&format!("-{pid_a}-"), "-");
    let without_pid_b = name_b.replace(&format!("-{pid_b}-"), "-");
    assert_ne!(
        without_pid_a, without_pid_b,
        "stripping PID, both executions assemble the SAME name ({without_pid_a}) — \
         and Windows recycles PID: measured on 08/09/2026, PID 21008 appeared \
         in two distinct runs of this binary"
    );
}

/// **Fixture refuses inherited folder** — what Windows hands out when it recycles PID.
///
/// This is the gate that does not depend on luck: instead of waiting for recycling,
/// it manually constructs what it produces — the folder of the previous run, with
/// `meu_app` already inside — and requires the helper to refuse returning it.
/// Without this refusal, the helper returns an occupied folder and cargo complains
/// with the phrase `already exists here, with a project in it`, pointing to the
/// wrong place.
#[test]
#[should_panic(expected = "INHERIT")]
fn fixture_refuses_folder_already_on_disk() {
    let from_prev_run = folder("gate-inherited").join("prev-run");
    std::fs::create_dir_all(from_prev_run.join("meu_app")).expect("prev run folder");
    setup(from_prev_run);
}

/// **The sweep removes only stale STAMPED fixture folders**. The clock
/// moves instead of the disk: the same fresh folder is kept at `now` and
/// removed a week and a day later, while a stamped file, a foreign folder and
/// an unstamped `crawrad-*` folder survive both — that last one is the gate's
/// `crawrad-darwin-shim`, which the first cut of this sweep deleted.
///
/// Proven by mutation: dropping the age check removes the fresh folder at
/// `now`, and going back to `starts_with("crawrad-")` removes the shim. Dropping
/// `is_dir` alone does NOT fail it — `remove_dir_all` already refuses a file —
/// so that check is belt and braces, not the gated rule.
#[test]
fn sweep_removes_only_stale_stamped_fixtures() {
    let root = folder("gate-sweep");
    let fixture = root.join("crawrad-gate-echo-21008-18a2f3c4-7");
    let shim = root.join("crawrad-darwin-shim");
    let file = root.join("crawrad-file-1-ab-2");
    std::fs::create_dir_all(fixture.join("meu_app")).expect("fixture folder");
    std::fs::create_dir_all(&shim).expect("shim folder");
    std::fs::create_dir_all(root.join("other-tool")).expect("foreign folder");
    std::fs::write(&file, "x").expect("stamped file");

    let now = std::time::SystemTime::now();
    assert_eq!(sweep_stale(&root, now, STALE_AFTER), 0);
    assert!(fixture.exists(), "a fresh fixture was swept");

    let later = now + STALE_AFTER + std::time::Duration::from_secs(24 * 60 * 60);
    assert_eq!(sweep_stale(&root, later, STALE_AFTER), 1);
    assert!(!fixture.exists(), "a stale fixture survived");
    assert!(shim.exists(), "an unstamped crawrad-* folder was swept");
    assert!(
        root.join("other-tool").exists(),
        "a foreign folder was swept"
    );
    assert!(file.exists(), "a file was swept");
}

/// The echo: exists for [`two_binary_executions_do_not_reuse_path`] to read from a child process,
/// because the collision it measures is BETWEEN processes and cannot be measured from within a single one.
#[test]
fn fixture_path_echo() {
    let p = folder("gate-echo");
    let name = p
        .file_name()
        .expect("fixture is a folder with name")
        .to_string_lossy()
        .to_string();
    println!("{ECHO}{}|{name}", std::process::id());
}

/// Runs this test binary asking for fixture echo, returns (pid, folder_name).
fn echo_from_child() -> (String, String) {
    let exe = std::env::current_exe().expect("test binary itself");
    let output = std::process::Command::new(&exe)
        .args(["fixture::fixture_path_echo", "--exact", "--nocapture"])
        .output()
        .expect("run test binary itself");
    let txt = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.status.success(), "child failed:\n{txt}");
    let line = txt
        .lines()
        .find_map(|l| l.trim().strip_prefix(ECHO))
        .unwrap_or_else(|| panic!("child did not echo fixture path:\n{txt}"));
    let (pid, name) = line
        .split_once('|')
        .unwrap_or_else(|| panic!("echo was not `pid|name`: {line}"));
    (pid.to_string(), name.to_string())
}

/// A mock folder and a mock document: no `cargo`.
///
/// What this pair of tests measures is the ORDER between unlinking and planning,
/// and for that the generator and the disk are sufficient. Calling `cargo new`
/// would cost five seconds per test to prove the exact same thing.
///
/// `pub(crate)` because module `the_editor_writes_before_prepass` measures the
/// SAME setup on another axis — the empty stub on disk, the bound event —
/// and a second copy would be the fixture drifting in silence: as soon as the
/// generator changes the stub shape, one of the two modules would test a file
/// nobody writes anymore.
pub(crate) fn mock_project(name: &str) -> (PathBuf, Document) {
    let root = folder(&format!("order-{name}"));
    std::fs::create_dir_all(root.join("src")).expect("test folder");

    let mut d = Document::new("form_main", "Form");
    {
        let root_node = d.node_mut("form_main").expect("root");
        root_node.set_rect(Rect::new(0.0, 0.0, 480.0, 320.0));
        root_node.set_name("form_main");
    }
    let mut btn = crawrad_ir::Node::new("btn1", "Button");
    btn.set_name("btn_ok");
    btn.set_text("&OK");
    btn.set_rect(Rect::new(24.0, 96.0, 96.0, 32.0));
    btn.events.insert("on_click".into(), "btn_okClick".into());
    d.add_child("form_main", btn).expect("button");
    d.validate().expect("document");

    // The FIRST generation, which is the double click: creates
    // `src/form_main.rs` with the empty stub inside. Performed through the
    // standard entrance, not with a file written by hand here — a stub
    // assembled via literal would remain valid even if the generator stopped
    // writing that format.
    let plan = generator::plan_for(&d, "meu_app").expect("first generation");
    crawrad_codegen::escrever(&root, &plan).expect("write first generation");
    (root, d)
}
