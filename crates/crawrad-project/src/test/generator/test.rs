//! Generating is testable without a screen, and this file is the proof.
//!
//! Everything measured here used to be reachable only by opening the IDE: the
//! whole round trip lived in a method of the `egui` shell's `Ide`, and
//! building one asks for a window and a video context. What the suite could
//! reach were the pieces — the plan, the sentences — and never the **door**:
//! flush, pre-pass, plan, unlink, write, reload, in that order.
//!
//! No `cargo` is started anywhere in here. A folder, a document and the
//! generator are enough to measure every law this module carries, and calling
//! `cargo new` would cost five seconds per test to prove exactly the same
//! thing.
//!
//! The fixture is [`crate::fixture::mock_project`], and it is
//! borrowed on purpose rather than copied: it is the same setup the shell's
//! own generation tests use — the first generation already on disk, with the
//! empty stub the generator writes — and a second copy of it would be the
//! fixture drifting in silence the day the stub changes shape.

// NOT `use super::*`, because this module is no longer a child of
// `generator.rs`: it is declared by `main.rs`, which is where the fixture it
// borrows lives. The reason is written out at the bottom of `generator.rs`. So
// the module under test is named, and so is everything the parent's own `use`
// lines used to hand over for free.
use std::path::{Path, PathBuf};

use crawrad_doc::Command;

use crate::generator::*;

/// The handler the fixture links, and the shape its stub has on disk.
const HANDLER: &str = "btn_okClick";
const STUB: &str = "pub fn btn_okClick(&mut self) {";

/// The user's file of the fixture — the one that promises *"This file is
/// yours"*.
fn user_file(root: &Path) -> PathBuf {
    root.join("src/form_main.rs")
}

/// The generated half — rewritten whole on every generation.
fn designer_file(root: &Path) -> PathBuf {
    root.join("src/form_main.designer.rs")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("reading {}: {e}", path.display()))
}

/// What an editor did, said out loud so a test can charge it.
///
/// A spy and not the real code tab, for the same reason [`PendingEdits`]
/// exists: the trait is what both shells will implement, and a test that used
/// one of them would measure the wiring of one shell instead of the door.
enum Editing {
    /// Nothing was pending — the common case.
    Nothing,
    /// There was something, and this text went into the user's file. It writes
    /// on the **disk**, because that is the only way to tell whether the
    /// pre-pass ran before or after it.
    Typed(PathBuf, String),
    /// The write was refused: the file changed underneath, and generating on
    /// top would judge a disk the user does not control.
    Refused(String),
}

struct Spy {
    editing: Editing,
    /// What [`PendingEdits::reload`] will answer — the tabs left showing the
    /// old text.
    stale: Vec<PathBuf>,
    flushed: usize,
    reloaded: usize,
}

impl Spy {
    fn quiet() -> Self {
        Self {
            editing: Editing::Nothing,
            stale: Vec::new(),
            flushed: 0,
            reloaded: 0,
        }
    }

    fn typing(file: &Path, text: &str) -> Self {
        Self {
            editing: Editing::Typed(file.to_path_buf(), text.to_owned()),
            ..Self::quiet()
        }
    }

    fn refusing(reason: &str) -> Self {
        Self {
            editing: Editing::Refused(reason.to_owned()),
            ..Self::quiet()
        }
    }
}

impl PendingEdits for Spy {
    fn flush(&mut self) -> Result<Option<PathBuf>, String> {
        self.flushed += 1;
        match &self.editing {
            Editing::Nothing => Ok(None),
            Editing::Typed(path, text) => {
                std::fs::write(path, text).expect("the spy writing the editor's text");
                Ok(Some(path.clone()))
            }
            Editing::Refused(reason) => Err(reason.clone()),
        }
    }

    fn reload(&mut self) -> Vec<PathBuf> {
        self.reloaded += 1;
        std::mem::take(&mut self.stale)
    }
}

// ------------------------------------------------------------- the ordering

/// **THE ORDER GATE.** The pre-pass judges the disk the flush has just
/// written.
///
/// This is measured through the new door. The spy types a line into
/// the handler on disk; if [`prepare`] asked `vinculos_vencidos` first and
/// flushed afterwards, the pre-pass would still see the empty stub and condemn
/// the handler the user has just written.
#[test]
fn the_pre_pass_judges_the_disk_the_flush_has_just_written() {
    let (root, doc) = crate::fixture::mock_project("door-order");
    let typed = read(&user_file(&root)).replace(STUB, &format!("{STUB}\n        self.n += 1;"));
    let mut spy = Spy::typing(&user_file(&root), &typed);

    let plan = prepare(&root, &mut spy, &doc, "meu_app").expect("the generation");

    assert_eq!(spy.flushed, 1, "the editor was not asked to write itself");
    assert!(
        plan.expired.is_empty(),
        "the pre-pass condemned the handler the user has just written — the \
         flush ran after it: {:?}",
        plan.expired.iter().map(|x| &x.handler).collect::<Vec<_>>()
    );
    assert_eq!(
        plan.saved.as_deref(),
        Some(user_file(&root).as_path()),
        "the bar has no way to say what was written without being asked"
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// **THE NEGATIVE CONTROL**, and without it the test above proves nothing.
///
/// Same fixture, nobody typing: the stub must still be condemned. A fix that
/// simply switched the pruning off would leave the test above green and undo
/// the whole of ADR 0025.
#[test]
fn the_stub_nobody_touched_is_still_pruned() {
    let (root, doc) = crate::fixture::mock_project("door-control");
    let mut spy = Spy::quiet();

    let plan = prepare(&root, &mut spy, &doc, "meu_app").expect("the generation");

    assert_eq!(
        plan.expired
            .iter()
            .map(|x| x.handler.as_str())
            .collect::<Vec<_>>(),
        vec![HANDLER],
        "the pruning of the empty handler stopped happening — this is ADR 0025 \
         undone"
    );
    assert!(
        plan.saved.is_none(),
        "a clean buffer was written anyway: rewriting an identical `.rs` moves \
         the `mtime` and cargo rebuilds the world on the next Run"
    );
    let _ = std::fs::remove_dir_all(&root);
}

// ------------------------------------------------------------ the unlinking

/// The pre-pass proposes **one** transaction, and the shell applies it.
///
/// Two links and one transaction: undo is a single gesture, like the save that
/// triggered it. One per link would make the user press `Ctrl+Z` twice to undo
/// one `Ctrl+S`, and between the two steps a shared handler would sit with a
/// live link calling a method that had already left the file.
#[test]
fn every_expired_link_travels_in_one_transaction() {
    let (root, mut doc) = crate::fixture::mock_project("door-one-tx");
    // A second button, linked to the same empty handler file: two links, and
    // the fixture's first generation already wrote the stubs for both.
    let mut second = crawrad_ir::Node::new("btn2", "Button");
    second.set_name("btn_cancel");
    second.set_text("&Cancel");
    second.set_rect(crawrad_ir::Rect::new(140.0, 96.0, 96.0, 32.0));
    second
        .events
        .insert("on_click".into(), "btn_cancelClick".into());
    doc.add_child("form_main", second).expect("the button");
    // The stubs of both live on disk only after a generation that knows about
    // the two of them.
    let files = plan_for(&doc, "meu_app").expect("the plan with two buttons");
    crawrad_codegen::escrever(&root, &files).expect("writing the two stubs");

    let mut spy = Spy::quiet();
    let plan = prepare(&root, &mut spy, &doc, "meu_app").expect("the generation");
    assert_eq!(
        plan.expired.len(),
        2,
        "both stubs are empty and both expired"
    );

    let tx = plan.unlink().expect("the unlinking of two empty handlers");
    assert_eq!(
        tx.commands.len(),
        2,
        "two links came back as {} transaction(s) worth of commands — undo \
         stopped being one gesture",
        tx.commands.len()
    );
    assert!(
        tx.commands
            .iter()
            .all(|c| matches!(c, Command::SetEvent { handler: None, .. })),
        "the unlinking wrote something other than clearing the event: {:?}",
        tx.commands
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// Nothing expired, **no transaction at all**.
///
/// An empty transaction would still be a step on the undo stack: the user
/// would press `Ctrl+Z` after a `Generate` and watch nothing happen.
#[test]
fn nothing_expired_means_no_transaction() {
    let (root, doc) = crate::fixture::mock_project("door-no-tx");
    let typed = read(&user_file(&root)).replace(STUB, &format!("{STUB}\n        self.n += 1;"));
    let mut spy = Spy::typing(&user_file(&root), &typed);
    let plan = prepare(&root, &mut spy, &doc, "meu_app").expect("the generation");
    assert!(plan.unlink().is_none());
    let _ = std::fs::remove_dir_all(&root);
}

// ----------------------------------------------------------- the whole door

/// The round trip: prune, write, and **say so**.
///
/// The three halves that used to be measurable only with the IDE open: the
/// `fn` leaves the user's file, the designer stops calling it, and the bar
/// counts the pruning. Silence here is what looks like the IDE eating code.
#[test]
fn the_whole_door_prunes_writes_and_reports() {
    let (root, doc) = crate::fixture::mock_project("door-round-trip");
    let mut spy = Spy::quiet();
    let plan = prepare(&root, &mut spy, &doc, "meu_app").expect("the generation");

    let sentence = write(&root, plan, &mut spy);

    assert!(
        !refused(&sentence),
        "the generation refused, and Run will not launch: {sentence}"
    );
    assert!(
        sentence.contains("1 empty handler removed"),
        "the bar did not count the pruning that just happened: {sentence}"
    );
    let unit = read(&user_file(&root));
    let designer = read(&designer_file(&root));
    assert!(
        !unit.contains(HANDLER) && !designer.contains(HANDLER),
        "the pruned handler's name survived on disk:\nunit:\n{unit}\ndesigner:\n{designer}"
    );
    assert_eq!(
        spy.reloaded, 1,
        "the editor was not reloaded after the generation rewrote the files \
         underneath it"
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// **ADR 0010: the user's code is never overwritten, it only grows.**
///
/// The most expensive thing this door could get wrong, and the one that no
/// test reached before: the round trip, twice, with a hand-written line in
/// between.
#[test]
fn the_user_file_is_never_overwritten_only_grown() {
    let (root, doc) = crate::fixture::mock_project("door-ownership");
    // The user types inside the handler — that is what keeps the link alive,
    // and it is what must survive the next generation.
    let mine = read(&user_file(&root)).replace(
        STUB,
        &format!("{STUB}\n        // mine, and it stays\n        self.n += 1;"),
    );
    std::fs::write(user_file(&root), &mine).expect("the user typing in his own file");

    let mut spy = Spy::quiet();
    let plan = prepare(&root, &mut spy, &doc, "meu_app").expect("the generation");
    let sentence = write(&root, plan, &mut spy);

    let after = read(&user_file(&root));
    assert!(
        after.contains("// mine, and it stays") && after.contains("self.n += 1;"),
        "the generation overwrote the user's file — this is ADR 0010 \
         undone:\n{after}"
    );
    assert!(
        sentence.contains("1 kept"),
        "the bar stopped showing the proof that the user's file was not \
         touched: {sentence}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// A shell **with no editor** generates — and this is the Makepad path.
///
/// The unit implementation is not a test convenience: a shell without a code
/// tab has nothing to flush and nothing to reload, and this is what the next
/// round wires up.
#[test]
fn a_shell_with_no_editor_can_generate() {
    let (root, doc) = crate::fixture::mock_project("door-no-editor");
    let mut no_editor = ();
    let plan = prepare(&root, &mut no_editor, &doc, "meu_app").expect("the generation");
    assert!(plan.saved.is_none(), "there was no editor to write");
    let sentence = write(&root, plan, &mut no_editor);
    assert!(!refused(&sentence), "{sentence}");
    assert!(
        designer_file(&root).exists(),
        "the generation wrote nothing for a shell with no code tab"
    );
    let _ = std::fs::remove_dir_all(&root);
}

// --------------------------------------------------------------- the refusals

/// A refused write **aborts the generation**, and `Run` can see it.
///
/// The file changed underneath — someone's editor open on the same file, which
/// ADR 0015 treats as a normal flow. Generating anyway would make the pre-pass
/// judge a disk the user does not control, and the pruning would take the
/// handler he has just written.
#[test]
fn a_refused_flush_aborts_before_anything_is_judged() {
    let (root, doc) = crate::fixture::mock_project("door-refused-flush");
    let mut spy = Spy::refusing("form_main.rs changed on disk");

    let Err(sentence) = prepare(&root, &mut spy, &doc, "meu_app") else {
        panic!("generated on top of a file that changed underneath");
    };

    assert!(
        refused(&sentence),
        "Run will not abort with this sentence, and it launches cargo on top of \
         a generation that never happened: {sentence}"
    );
    assert!(
        sentence.contains("changed on disk"),
        "the refusal lost the reason on the way: {sentence}"
    );
    assert_eq!(spy.reloaded, 0, "nothing was written, nothing to reload");
    let _ = std::fs::remove_dir_all(&root);
}

/// A plan that failed **writes nothing**, and does not touch the editor.
///
/// Reloading here would be the worst kind of wrong: the files on screen are
/// still the right ones, and reloading would throw away what is being typed
/// for a generation that did not happen.
#[test]
fn a_failed_plan_writes_nothing_and_reloads_nothing() {
    let (root, _doc) = crate::fixture::mock_project("door-failed-plan");
    let before = read(&designer_file(&root));
    let mut spy = Spy::quiet();
    let plan = Plan {
        saved: Some(user_file(&root)),
        expired: Vec::new(),
        files: Err(crawrad_codegen::Erro::SemRaiz),
    };

    let sentence = write(&root, plan, &mut spy);

    assert!(refused(&sentence), "{sentence}");
    assert_eq!(spy.reloaded, 0, "the editor was reloaded for nothing");
    assert_eq!(
        read(&designer_file(&root)),
        before,
        "a failed plan reached the disk anyway"
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// The bar names the tab that stayed showing the old text.
///
/// What could come back came back; what was dirty stayed, and is **named**.
/// Reloading over what is being typed would trade a silent loss for another
/// one, and staying silent about it would leave the user editing a file the
/// generation has already rewritten.
#[test]
fn the_bar_names_the_tab_left_showing_old_text() {
    let (root, doc) = crate::fixture::mock_project("door-stale-tab");
    let mut spy = Spy::quiet();
    spy.stale = vec![designer_file(&root)];
    let plan = prepare(&root, &mut spy, &doc, "meu_app").expect("the generation");

    let sentence = write(&root, plan, &mut spy);

    assert!(
        sentence.contains("the Code tab still shows the old form_main.designer.rs"),
        "the tab kept showing text the generation had already replaced, and \
         nothing said so: {sentence}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// Nothing stale, nothing said. A bar that says it in every generation teaches
/// people not to read the bar.
#[test]
fn a_clean_reload_says_nothing() {
    assert!(stale_note(&[]).is_none());
}

// --------------------------------------------------------- the first project

/// The project born a second ago comes with its code — and with the user's
/// file.
///
/// a classic RAD IDE hands you `Unit1.pas` already written. Measured in the IDE: create a
/// project, double click a `Button`, and get the invitation screen instead of
/// the handler.
#[test]
fn a_new_project_is_born_with_its_code() {
    let root = crate::fixture::folder("door-new-project");
    let mut doc = crawrad_ir::Document::new("form_main", "Form");
    doc.node_mut("form_main")
        .expect("the root")
        .set_rect(crawrad_ir::Rect::new(0.0, 0.0, 480.0, 320.0));
    doc.validate().expect("the document");

    generate_for_new_project(&root, &doc).expect("the first generation");

    assert!(
        user_file(&root).exists() && designer_file(&root).exists(),
        "the project was created without code in src/"
    );

    // And the second time it does NOT overwrite the user's half: the ownership
    // rule belongs to `escrever`, and it applies with or without a pre-pass.
    std::fs::write(user_file(&root), "// written by hand\n").expect("the user's file");
    generate_for_new_project(&root, &doc).expect("the second generation");
    assert_eq!(
        read(&user_file(&root)),
        "// written by hand\n",
        "creating over an existing folder overwrote the user's file"
    );
    let _ = std::fs::remove_dir_all(&root);
}

// ------------------------------------------------------------- the manifest

/// The package name comes from the manifest, and `app` when there is none.
///
/// Refusing to generate because of an unreadable `Cargo.toml` would trade a
/// crooked name for no code at all — and the name is what the generated `use`
/// is written with.
#[test]
fn the_package_name_comes_from_the_manifest_or_falls_back() {
    let root = crate::fixture::folder("door-manifest");
    assert_eq!(
        package_name_in(&root),
        "app",
        "a member with no manifest must still generate"
    );
    std::fs::write(
        root.join(crate::workspace::MANIFEST),
        "[package]\nname = \"meu_app\"\nedition = \"2021\"\n",
    )
    .expect("the manifest");
    assert_eq!(package_name_in(&root), "meu_app");
    let _ = std::fs::remove_dir_all(&root);
}

/// The refusal is spelled in **one** place, and `Run` reads exactly that.
///
/// The prefix written twice is the family of the shortcut that disagreed with
/// its own tooltip (ADR 0011); here the price is `Run` launching cargo over a
/// generation that did not happen.
#[test]
fn every_refusal_carries_the_prefix_run_looks_for() {
    assert!(refused(&refusal("no form open")));
    assert!(refused(&refusal(NO_MEMBER)));
    assert!(refused(&refusal(NO_DOCUMENT)));
    assert!(
        !refused("saved form_main.rs — generated for egui in C:/work/app"),
        "a generation that worked was read as a refusal, and Run stopped \
         launching"
    );
}

/// **A form three levels down sees its whole chain**: the bases
/// come nearest first, and a cycle stops the walk instead of looping.
#[test]
fn the_bases_load_as_a_chain_and_a_cycle_stops() {
    let dir = std::env::temp_dir().join(format!("crawrad-bases-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    let form = |name: &str, inherits: Option<&str>| {
        let mut doc = crawrad_ir::Document::new(name, "Form");
        doc.set_inherits(inherits);
        doc.save(dir.join("src").join(format!("{name}.crawform")))
            .unwrap();
        doc
    };
    form("form_a", None);
    form("form_b", Some("form_a"));
    let c = form("form_c", Some("form_b"));
    let roots =
        |bases: Vec<crawrad_ir::Document>| bases.into_iter().map(|b| b.root).collect::<Vec<_>>();
    assert_eq!(
        roots(super::load_bases(Some(&dir), &c)),
        ["form_b", "form_a"]
    );
    assert!(
        super::load_bases(None, &c).is_empty(),
        "no folder, no bases"
    );

    form("form_a", Some("form_c"));
    assert_eq!(
        roots(super::load_bases(Some(&dir), &c)),
        ["form_b", "form_a"],
        "the walk stops where the chain comes back to the form"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// **A form's project has a target only once it was generated**: until
/// then rust-analyzer cannot analyse it (the owner's Ctrl+Space that
/// opened and closed, 01/10/2026).
#[test]
fn a_member_has_a_target_only_with_main_or_lib() {
    let dir = std::env::temp_dir().join(format!("crawrad-target-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(dir.join("src").join("form_main.rs"), "").unwrap();
    assert!(!super::has_a_target(&dir), "a form alone is not a target");
    std::fs::write(dir.join("src").join("main.rs"), "fn main() {}").unwrap();
    assert!(super::has_a_target(&dir));
    std::fs::remove_file(dir.join("src").join("main.rs")).unwrap();
    std::fs::write(dir.join("src").join("lib.rs"), "").unwrap();
    assert!(super::has_a_target(&dir));
    let _ = std::fs::remove_dir_all(&dir);
}

/// **The framework is the project's**: `eframe` (or the egui runtime)
/// is egui, `makepad-widgets` (or the Makepad runtime) is Makepad, in any of
/// Cargo's spellings; a manifest naming neither has no framework.
#[test]
fn the_framework_comes_from_the_dependencies() {
    use crawrad_codegen::AlvoGerador::{Egui, Makepad};
    let deps = |body: &str| format!("[package]\nname = \"app\"\n\n[dependencies]\n{body}\n");
    assert_eq!(target_of_manifest(&deps("eframe = \"0.36\"")), Some(Egui));
    assert_eq!(
        target_of_manifest(&deps("makepad-widgets = \"1.0\"")),
        Some(Makepad)
    );
    assert_eq!(
        target_of_manifest(&deps("crawrad-runtime-makepad = { path = \"../r\" }")),
        Some(Makepad)
    );
    assert_eq!(
        target_of_manifest(&deps("eframe.workspace = true")),
        Some(Egui)
    );
    assert_eq!(target_of_manifest(&deps("serde = \"1\"")), None);
    // both frameworks: the runtime decides (the example app carries eframe too)
    let both = "makepad-widgets = \"1.0.0\"
eframe = \"0.36\"";
    assert_eq!(target_of_manifest(&deps(both)), None);
    let runtime = format!(
        "{both}
crawrad-runtime-makepad = {{ path = \"../r\" }}"
    );
    assert_eq!(target_of_manifest(&deps(&runtime)), Some(Makepad));
    // a name that only starts like a framework is not one
    assert_eq!(target_of_manifest(&deps("eframe-extras = \"1\"")), None);
}

/// [`project_target`] reads the member's manifest; no manifest, no framework.
#[test]
fn the_project_target_reads_the_member_manifest() {
    let dir = std::env::temp_dir().join(format!("crawrad-framework-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    assert_eq!(project_target(&dir), None);
    std::fs::write(
        dir.join("Cargo.toml"),
        "[dependencies]\neframe = \"0.36\"\n",
    )
    .unwrap();
    assert_eq!(
        project_target(&dir),
        Some(crawrad_codegen::AlvoGerador::Egui)
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn project_kind_now_distinguishes_console_and_library() {
    assert_eq!(
        project_kind_now(true, false, true, false),
        (Some("console"), true)
    );
    assert_eq!(
        project_kind_now(true, false, false, false),
        (Some("library"), true)
    );
    assert_eq!(project_kind_now(true, true, true, false), (None, false));
    assert_eq!(project_kind_now(true, false, true, true), (None, true));
}

#[test]
fn plan_with_locales_directory_adds_locales_module_to_plan() {
    let (root, doc) = crate::fixture::mock_project("locales-plan");
    let mut spy = Spy::quiet();

    // 1. Without locales/: plan does not contain src/locales.rs
    let plan_without = prepare(&root, &mut spy, &doc, "my_app").expect("plan without locales");
    let files_without = plan_without.files.expect("files without locales");
    assert!(
        !files_without
            .iter()
            .any(|f| f.caminho == Path::new("src/locales.rs"))
    );

    // 2. With locales/de.toml: plan holds src/locales.rs with de entry
    let locales_dir = root.join("locales");
    std::fs::create_dir_all(&locales_dir).unwrap();
    std::fs::write(
        locales_dir.join("de.toml"),
        "\"form_main.btn_ok.text\" = { text = \"OK\", from = \"OK\" }\n",
    )
    .unwrap();

    let plan_with = prepare(&root, &mut spy, &doc, "my_app").expect("plan with locales");
    let files_with = plan_with.files.expect("files with locales");
    let locales_file = files_with
        .iter()
        .find(|f| f.caminho == Path::new("src/locales.rs"))
        .expect("plan must hold src/locales.rs");
    assert_eq!(locales_file.dono, crawrad_codegen::Dono::Gerador);
    assert!(locales_file.conteudo.contains("\"de\""));
    assert!(locales_file.conteudo.contains("\"form_main.btn_ok.text\""));
    assert!(locales_file.conteudo.contains("\"OK\""));

    let _ = std::fs::remove_dir_all(&root);
}
