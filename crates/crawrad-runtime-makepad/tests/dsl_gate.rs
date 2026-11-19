//! Headless gate over the Makepad DSL — ADR 0027.
//!
//! # The hole this file closes
//!
//! In Makepad 1.0.0 an error inside `live_design!` **does not break the build**:
//! the DSL is a string `rustc` never reads. The error only shows up at run time,
//! and even then only as one line of log — the widget goes on instancing and
//! returning `Some`. The mechanism is in
//! `makepad-platform-1.0.0/src/live_cx.rs:166` (`Cx::apply_error` ->
//! `error!(...)`), and `error!` ends in a `println!` at
//! `makepad-platform-1.0.0/src/log.rs:103`.
//!
//! The upshot: it compiles green, it instances green, the screen comes out wrong
//! and no test goes red. That is how four migration attempts died.
//!
//! # How the gate works
//!
//! There are three stages, and each one uses the best channel available:
//!
//! 1. **Expansion** — instead of `cx.live_expand()` (which swallows the errors
//!    into a `println!`, `live_cx.rs:366`), the gate calls
//!    `LiveRegistry::expand_all_documents(&mut errs)` directly
//!    (`makepad-live-compiler-1.0.0/src/live_registry.rs:945`), which returns a
//!    **structured** `Vec<LiveError>`. It catches syntax, import and
//!    non-existent-class errors.
//! 2. **Instantiation** — here there is no structured channel: the only way out
//!    is the log. So the probe runs in a **child process** and the parent reads
//!    its `stdout`/`stderr` looking for Makepad's error marks. It is ugly, and it
//!    is what there is: the other path would be the Makepad Studio websocket
//!    (`web_socket.rs:231`, `Cx::send_studio_message`), which needs a server.
//! 3. **Template instantiation** — measured: whatever sits inside a
//!    `<Button>{...}` kept in an `Option<LivePtr>` is **not** checked until
//!    somebody calls `WidgetRef::new_from_ptr`. A template not instanced is a
//!    template not checked.
//!
//! The child is this very test binary, re-executed with
//! `--exact <probe> --ignored --nocapture` and the `CRAWRAD_DSL_GATE_PROBE`
//! environment variable. Without a child process there is no way to capture
//! `println!`: `libtest` redirects each test's output into an unreachable
//! buffer, and `std::io::set_output_capture` is unstable API.
//!
//! # What the gate does NOT catch
//!
//! Proved by mutation (every line below was measured, not deduced):
//!
//! | class of DSL error | the gate |
//! |---|---|
//! | non-existent property name (`color` -> `colorr`) | **red** |
//! | the same, inside a `<Button>{...}` template | **red** (stage 3) |
//! | syntax (`width 80` with no colon) | **red** |
//! | non-existent widget (`<Butao>`) | **red** (stage 1) |
//! | non-existent field in a `fn pixel(self)` (GLSL) | **red** |
//! | font not loaded | **red** (the probe panics) |
//! | a wrong but valid value (`#f00` where `#0f0` was meant) | green — unknowable |
//! | a wrong but valid layout (`Fill` where `Fit` was meant) | green — unknowable |
//! | a shader that compiles but paints wrong | green — needs a GPU and an eye |
//! | the DSL of a widget the probe does not instance | green — only what is instanced is checked |
//!
//! That last line is the rule that governs everything: **the gate covers exactly
//! what the probe instances**. A new widget in the crate asks for a new line in
//! the probe.

use std::process::Command;

use crawrad_runtime_makepad::{DesignCanvas, makepad_widgets};

use makepad_widgets::*;

// ---------------------------------------------------------------------------
// The DSL under test
// ---------------------------------------------------------------------------
//
// This block is the gate's own target. It deliberately exercises the three
// forms the designer canvas is going to use (ADR 0027): one shader declared once
// and instanced N times (`draw_background`), an `Option<LivePtr>` template for a
// dynamic widget (`button_template`), and a `DrawText` with `text_style` — which
// is the form that loads a font and which, without the `*_dependencies` calls
// below, panics headless at
// `makepad-draw-1.0.0/src/shader/draw_text.rs:599`.

live_design! {
    use link::widgets::*;
    use link::theme::*;

    pub ProbeCanvas = {{ProbeCanvas}} {
        width: Fill,
        height: Fill,

        draw_background: {
            color: #1e1e2e
        }

        draw_label: {
            color: #cdd6f4
            text_style: <THEME_FONT_REGULAR> {
                font_size: 9.0
            }
        }

        button_template: <Button> {
            width: 80,
            height: 24,
            text: "ok"
        }

        edit_template: <TextInput> {
            width: 80,
            height: 24,
            text: "ok",
            empty_text: "hint",
            is_password: true,
            is_read_only: false,
            is_numeric_only: false,
            label_align: { x: 0.0, y: 0.0 },
        }
    }
}

/// The probe widget: the smallest shape that still applies a shader, a font and
/// a template.
#[derive(Live, LiveHook, Widget)]
pub struct ProbeCanvas {
    #[walk]
    walk: Walk,
    #[redraw]
    #[live]
    draw_background: DrawColor,
    #[live]
    draw_label: DrawText,
    #[live]
    button_template: Option<LivePtr>,
    #[live]
    edit_template: Option<LivePtr>,
}

impl Widget for ProbeCanvas {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_background.draw_walk(cx, walk);
        DrawStep::done()
    }
}

// ---------------------------------------------------------------------------
// Parent <-> child protocol
// ---------------------------------------------------------------------------

/// The variable that authorises the probe to run (only the parent sets it).
const PROBE_VAR: &str = "CRAWRAD_DSL_GATE_PROBE";

/// The mark the probe prints when it reaches the end without panicking.
const MARK_OK: &str = "<<<CRAWRAD-DSL-GATE-OK>>>";

/// The mark the probe reports an already structured expansion error with.
const MARK_EXPANSION: &str = "<<<CRAWRAD-DSL-GATE-EXPANSION>>>";

/// Every textual mark Makepad uses to report a DSL error.
///
/// The format of every log line is `{file}:{line}:{column} - {message}`
/// (`makepad-platform-1.0.0/src/log.rs:103`), and the level (`Error`/`Log`) does
/// **not** appear in the text. That is why the marks are the first words of the
/// message.
///
/// Addresses in the local registry:
/// - `makepad-platform/src/live_cx.rs:190` — `error!("Apply error: {} {:?}", ...)`
/// - `makepad-platform/src/live_cx.rs:193` — `error!("Apply without file, at index ...")`
/// - `makepad-platform/src/live_cx.rs:366` — `println!("Error expanding live file {}", ...)`
/// - `makepad-platform/src/live_cx.rs:419` — `error!("Error parsing live file {}", ...)`
/// - `makepad-platform/src/draw_vars.rs:312` — `log!("Error {}", err)`, the **shader**
/// - `makepad-live-compiler/src/live_registry.rs:955` — `println!("Cannot find main file id ...")`
/// - `makepad-platform/src/os/cx_native.rs:37` — `println!("Could not load resource {}", ...)`
const ERROR_MARKS: &[&str] = &[
    MARK_EXPANSION,
    "Apply error:",
    "Apply without file",
    // Covers `Error expanding live file`, `Error parsing live file` and — the
    // case that nearly escaped — the `log!("Error {}", err)` of the shader
    // compiler at `draw_vars.rs:312`, which is the only warning of broken GLSL
    // before the GPU.
    "- Error ",
    "Cannot find main file id",
    "Could not load resource",
];

/// Collects from the text every line that carries a Makepad error mark.
fn collect_errors(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| ERROR_MARKS.iter().any(|mark| line.contains(mark)))
        .map(|line| line.trim().to_string())
        .collect()
}

// ---------------------------------------------------------------------------
// The probes (they run in the child process)
// ---------------------------------------------------------------------------

/// Builds a headless `Cx`, registers the DSL, expands it and loads the fonts.
///
/// `W` is the widget that becomes the **main module**: the expansion starts from
/// it (`live_registry.rs:952`) and only its file has its dependencies scanned
/// (`live_cx.rs:373`). That is why every DSL under test needs a probe of its
/// own, with a `Cx` of its own — a `Cx` has exactly one main module.
fn prepare<W: LiveNew>(register: fn(&mut Cx)) -> Cx {
    let mut cx = Cx::new(Box::new(|_, _| {}));

    // Without this, `expand_all_documents` panics on the `unwrap()` of
    // `main_module` (makepad-live-compiler-1.0.0/src/live_registry.rs:952).
    W::register_main_module(&mut cx);

    makepad_widgets::live_design(&mut cx);
    // the crate's shared templates, registered first as `crate::live_design` does
    crawrad_runtime_makepad::borderless::live_design(&mut cx);
    register(&mut cx);

    // ---- stage 1: expansion, with the errors in hand instead of in the log ---
    // `cx.live_expand()` would throw everything into a loose `println!`
    // (`live_cx.rs:366`); here the errors come structured and go out carrying
    // the gate's mark.
    let mut errors = Vec::new();
    {
        let registry = cx.live_registry.clone();
        let mut registry = registry.borrow_mut();
        registry.expand_all_documents(&mut errors);
        for error in errors.drain(..) {
            println!(
                "{MARK_EXPANSION} {}",
                registry.live_error_to_live_file_error(error)
            );
        }
    }

    // ---- fonts: what avoids the DrawText panic ----
    // Measured: without these two lines, any `text_style` panics headless at
    // `makepad-draw-1.0.0/src/shader/draw_text.rs:599` ("Dependency not loaded
    // .../IBMPlexSans-Text.ttf"). With them, it loads — both methods are public
    // and this is the same order the native backend uses
    // (`makepad-platform-1.0.0/src/os/windows/windows.rs:430-431`).
    cx.live_scan_dependencies();
    cx.native_load_dependencies();

    cx
}

/// The probe that counts: it instances the real `DesignCanvas`, with the four
/// SDF shaders and the `DrawText` it declares in `src/canvas.rs`.
#[test]
#[ignore = "gate probe; the parent runs it in a child process"]
fn probe_canvas() {
    // The probe only makes sense inside the child process the gate launches.
    // Under `--include-ignored` it runs directly, and `#[ignore]` does NOT
    // protect against the very flag that exists to switch it off. Returning
    // quietly is what keeps the house gate green — failing here would leave
    // `cargo test --workspace -- --include-ignored` red forever.
    if std::env::var(PROBE_VAR).is_err() {
        return;
    }

    let mut cx = prepare::<DesignCanvas>(crawrad_runtime_makepad::canvas::live_design);

    // ---- stage 2: instantiation (only the log reports a property name) ----
    let canvas = DesignCanvas::new_main(&mut cx);
    assert!(
        canvas.is_some(),
        "the DesignCanvas was not found in the expanded DSL"
    );

    println!("{MARK_OK}");
}

/// The ListBox probe: the generated app's `CrawradListBox`, with BOTH row
/// templates instanced — they are `PortalList` templates, applied only when a
/// row is made (stage 3), so a wrong property inside them is only caught here.
#[test]
#[ignore = "gate probe; the parent runs it in a child process"]
fn probe_list_box() {
    if std::env::var(PROBE_VAR).is_err() {
        return;
    }
    let mut cx = prepare::<crawrad_runtime_makepad::list::CrawradListBox>(
        crawrad_runtime_makepad::list::live_design,
    );
    let Some(list_box) = crawrad_runtime_makepad::list::CrawradListBox::new_main(&mut cx) else {
        panic!("the CrawradListBox was not found in the expanded DSL");
    };
    let list = list_box.portal_list(id!(list));
    for template in [live_id!(row), live_id!(row_selected)] {
        let row = list.item(&mut cx, 0, template);
        assert!(
            !row.is_empty(),
            "the `{template}` row template produced no widget"
        );
    }
    println!("{MARK_OK}");
}

/// The ComboBox probe: the generated app's `CrawradComboBox`.
#[test]
#[ignore = "gate probe; the parent runs it in a child process"]
fn probe_combo_box() {
    if std::env::var(PROBE_VAR).is_err() {
        return;
    }
    // its boxes and caret are the MultiSelect's: registered first, as `lib.rs` does
    let mut cx = prepare::<crawrad_runtime_makepad::combo_box::CrawradComboBox>(|cx| {
        crawrad_runtime_makepad::multi_select::live_design(cx);
        crawrad_runtime_makepad::combo_box::live_design(cx);
    });
    let Some(_combo_box) = crawrad_runtime_makepad::combo_box::CrawradComboBox::new_main(&mut cx)
    else {
        panic!("the CrawradComboBox was not found in the expanded DSL");
    };
    println!("{MARK_OK}");
}

/// The CheckListBox probe: its `row` template instanced — a PortalList
/// template is applied only when a row is made, so a wrong property inside it
/// is only caught here.
#[test]
#[ignore = "gate probe; the parent runs it in a child process"]
fn probe_check_list_box() {
    if std::env::var(PROBE_VAR).is_err() {
        return;
    }
    let mut cx = prepare::<crawrad_runtime_makepad::check_list::CrawradCheckListBox>(
        crawrad_runtime_makepad::check_list::live_design,
    );
    let Some(list_box) =
        crawrad_runtime_makepad::check_list::CrawradCheckListBox::new_main(&mut cx)
    else {
        panic!("the CrawradCheckListBox was not found in the expanded DSL");
    };
    let list = list_box.portal_list(id!(list));
    let row = list.item(&mut cx, 0, live_id!(row));
    assert!(!row.is_empty(), "the `row` template produced no widget");
    println!("{MARK_OK}");
}

/// The ListView probe: the generated app's `CrawradListView`, with BOTH row
/// templates instanced.
#[test]
#[ignore = "gate probe; the parent runs it in a child process"]
fn probe_list_view() {
    if std::env::var(PROBE_VAR).is_err() {
        return;
    }
    let mut cx = prepare::<crawrad_runtime_makepad::list_view::CrawradListView>(
        crawrad_runtime_makepad::list_view::live_design,
    );
    let Some(list_view) = crawrad_runtime_makepad::list_view::CrawradListView::new_main(&mut cx)
    else {
        panic!("the CrawradListView was not found in the expanded DSL");
    };
    let list = list_view.portal_list(id!(list));
    for template in [live_id!(row), live_id!(row_selected)] {
        let row = list.item(&mut cx, 0, template);
        assert!(
            !row.is_empty(),
            "the `{template}` row template produced no widget"
        );
    }
    println!("{MARK_OK}");
}

/// The DataGrid probe: the visual `CrawradDataGrid`, with its row template instanced.
#[test]
#[ignore = "gate probe; the parent runs it in a child process"]
fn probe_data_grid() {
    if std::env::var(PROBE_VAR).is_err() {
        return;
    }
    let mut cx = prepare::<crawrad_runtime_makepad::data_grid::CrawradDataGrid>(
        crawrad_runtime_makepad::data_grid::live_design,
    );
    let Some(data_grid) = crawrad_runtime_makepad::data_grid::CrawradDataGrid::new_main(&mut cx)
    else {
        panic!("the CrawradDataGrid was not found in the expanded DSL");
    };
    let list = data_grid.portal_list(id!(list));
    let item = list.item(&mut cx, 0, live_id!(item));
    assert!(
        !item.is_empty(),
        "the `item` row template produced no widget"
    );
    println!("{MARK_OK}");
}

/// The StringGrid probe: the visual `CrawradStringGrid`.
#[test]
#[ignore = "gate probe; the parent runs it in a child process"]
fn probe_string_grid() {
    if std::env::var(PROBE_VAR).is_err() {
        return;
    }
    let mut cx = prepare::<crawrad_runtime_makepad::string_grid::CrawradStringGrid>(
        crawrad_runtime_makepad::string_grid::live_design,
    );
    let Some(_string_grid) =
        crawrad_runtime_makepad::string_grid::CrawradStringGrid::new_main(&mut cx)
    else {
        panic!("the CrawradStringGrid was not found in the expanded DSL");
    };
    println!("{MARK_OK}");
}

/// The control probe: it proves the gate still bites.
///
/// It exists because the canvas DSL changes with every slice, and the gate is
/// only worth anything as long as the machine that moves it stays alive. This
/// DSL is stable and it is the target of the mutations that prove the gate —
/// stage 3 included, which the canvas does not yet exercise for having no
/// `Option<LivePtr>` template.
#[test]
#[ignore = "gate probe; the parent runs it in a child process"]
fn control_probe() {
    // The probe only makes sense inside the child process the gate launches.
    // Under `--include-ignored` it runs directly, and `#[ignore]` does NOT
    // protect against the very flag that exists to switch it off. Returning
    // quietly is what keeps the house gate green — failing here would leave
    // `cargo test --workspace -- --include-ignored` red forever.
    if std::env::var(PROBE_VAR).is_err() {
        return;
    }

    let mut cx = prepare::<ProbeCanvas>(live_design);

    let Some(probe) = ProbeCanvas::new_main(&mut cx) else {
        panic!("the control probe was not found in the expanded DSL");
    };

    // ---- stage 3: instance every template ----
    // Measured: without this, a wrong property name INSIDE a `<Button>{...}`
    // kept as an `Option<LivePtr>` slips through — the block is only applied
    // when somebody calls `WidgetRef::new_from_ptr`
    // (`makepad-widgets-1.0.0/src/portal_list.rs:563`). A template not instanced
    // is a template not checked.
    let button = WidgetRef::new_from_ptr(&mut cx, probe.button_template);
    assert!(
        !button.is_empty(),
        "the `button_template` template produced no widget"
    );
    let edit = WidgetRef::new_from_ptr(&mut cx, probe.edit_template);
    assert!(
        !edit.is_empty(),
        "the `edit_template` template produced no widget"
    );

    println!("{MARK_OK}");
}

// ---------------------------------------------------------------------------
// The gate (it runs in the parent process)
// ---------------------------------------------------------------------------

/// Runs one probe in a child process and returns everything it printed.
fn run_probe(name: &str) -> (String, bool) {
    let exe = std::env::current_exe().expect("path of the test binary");
    let output = Command::new(exe)
        .args([
            "--exact",
            name,
            "--ignored",
            "--nocapture",
            "--test-threads",
            "1",
        ])
        .env(PROBE_VAR, "1")
        .output()
        .expect("run the probe in a child process");

    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    (text, output.status.success())
}

/// Runs a probe and fails with everything it said, if anything went wrong.
fn demand(name: &str) {
    let (text, success) = run_probe(name);
    let found = collect_errors(&text);

    assert!(
        found.is_empty(),
        "probe `{name}` found {} DSL error(s) that Makepad only reports by log:\n{}\n\
         --- full probe output ---\n{text}",
        found.len(),
        found.join("\n")
    );

    assert!(
        success,
        "probe `{name}` did not finish well (a headless panic?):\n{text}"
    );

    assert!(
        text.contains(MARK_OK),
        "probe `{name}` never reached the end; without the success mark the gate is worth nothing:\n{text}"
    );
}

/// The gate over the canvas's real DSL (`src/canvas.rs`).
#[test]
fn the_design_canvas_dsl_is_sound() {
    demand("probe_canvas");
}

/// The gate over the generated app's ListBox (`src/list.rs`).
#[test]
fn the_list_box_dsl_is_sound() {
    demand("probe_list_box");
}

/// The gate over the generated app's ComboBox (`src/combo_box.rs`).
#[test]
fn the_combo_box_dsl_is_sound() {
    demand("probe_combo_box");
}

/// The gate over the palette's CheckListBox (`src/check_list.rs`).
#[test]
fn the_check_list_box_dsl_is_sound() {
    demand("probe_check_list_box");
}

/// The gate over the generated app's ListView (`src/list_view.rs`).
#[test]
fn the_list_view_dsl_is_sound() {
    demand("probe_list_view");
}

/// The gate over the visual DataGrid (`src/data_grid.rs`).
#[test]
fn the_data_grid_dsl_is_sound() {
    demand("probe_data_grid");
}

/// The gate over the visual StringGrid (`src/string_grid.rs`).
#[test]
fn the_string_grid_dsl_is_sound() {
    demand("probe_string_grid");
}

/// The gate over the control DSL. If this one goes green with a proving mutation
/// applied, the whole gate is broken.
#[test]
fn the_control_dsl_is_sound() {
    demand("control_probe");
}

/// Proves the error collector is not decorative: it has to recognise every mark
/// Makepad uses. Without this test, a `collect_errors` that always returned
/// empty would go unnoticed.
#[test]
fn the_collector_recognises_every_makepad_mark() {
    for mark in ERROR_MARKS {
        let line = format!("src/some_file.rs:10:4 {mark} blah blah");
        assert_eq!(
            collect_errors(&line).len(),
            1,
            "the collector let the mark {mark} through"
        );
    }
    assert!(collect_errors("just an innocent line").is_empty());
}
