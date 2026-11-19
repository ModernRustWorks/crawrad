//! The **clean code** gate: the generated project must not be born with warnings.
//!
//! [`gate_fase3`](gate_fase3) proves that the generated project *compiles and runs*.
//! This gate enforces the next bar, which is the product's calling card: `cargo`
//! must have nothing to say about what the IDE just wrote.
//! Code born with warnings contradicts CrawRAD's core premise — *a RAD app
//! checked by the compiler* — on the very first screen the user sees, and they cannot
//! even fix it: `.designer.rs` is completely rewritten on subsequent generation.
//!
//! ## The bug that originated this file
//!
//! The `Blank form` template — the first project anyone creates — produced
//! TWO `unused variable` warnings (`origin` and `grown`) and a `clippy::derivable_impls`.
//! The `show()` of a form without any components sized the `Ui` for
//! nobody, and `impl Default` for a fieldless struct is exactly what
//! `derive` produces. No unit test saw this: they verify output text,
//! and the text was *correct* — just not clean.
//!
//! ## Why `cargo clippy` and not `cargo build`
//!
//! `clippy-driver` runs **rustc's lint suite plus its own**, so
//! `-D warnings` in a single pass rejects both families. And `--` sets
//! the flag only on the user's crate: dependencies can still emit
//! whatever warnings they like, which is not our concern.
//!
//! Marked `#[ignore]` for the same reason as the Phase 3 gate — calls `cargo`
//! from inside `cargo` and may need the crates.io index:
//!
//! ```text
//! cargo test -p crawrad-codegen --test gate_sem_avisos -- --ignored --nocapture
//! ```

// The same rule as `gate_fase3`, and now the SAME function. It used to be two copies,
// and this copy was left with `contains("[workspace]")` which the other dropped in
// `4959652` for being insufficient. The reason why it is single is documented in
// `comum/mod.rs`.
mod comum;

use comum::manifesto_intocado;
use crawrad_codegen::Gerador as _;
use crawrad_ir::{Document, Node, Rect};
use crawrad_registry::Registry;
use std::path::{Path, PathBuf};
use std::process::Command;

/// An empty form, sized like the `Blank form` template.
fn form(nome: &str) -> Document {
    let mut d = Document::new(nome, "Form");
    let raiz = d.node_mut(nome).expect("the newly created root");
    raiz.set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
    raiz.set_name(nome);
    raiz.set_text("Form1");
    d
}

/// The handler name the IDE would write for an event: `btn1` + `on_click`.
fn handler(nome: &str, evento: &str) -> String {
    format!("{nome}_{evento}")
}

/// **The `Blank form` template.** This was the failing case, and it is the common case:
/// `File → New Project` and nothing else.
fn caso_em_branco() -> Document {
    let d = form("form_main");
    d.validate().expect("an empty form is valid");
    d
}

/// Just a non-visual component. Nothing draws, yet `Ui` is used — `Timer`
/// reads its clock. This case separates "needs an anchor" from "needs `Ui`".
///
/// **It was previously claimed here that emitting `let _ = ui;` in this case
/// "would cause an error instead of a warning". That is false, and was measured.**
/// `let _ = x;` does not move anything — it is a wildcard pattern, the value is
/// never bound —, so an extra `let _ = ui;` in this form compiles cleanly even
/// with `-D warnings`: `ui` remains available for `Timer` to use later. The
/// self-verifying property that comment promised — overestimating `precisa_ui`
/// breaks compilation — **DOES NOT exist**. Overestimating passes silently; the
/// opposite direction is what is enforced, and only that: omitting `let _ = ui;`
/// on a form that does not consume `Ui` triggers `unused variable`, which this
/// gate catches.
fn caso_so_um_timer() -> Document {
    let mut d = form("form_main");
    let mut t = Node::new("tmr1", "Timer");
    t.set_name("tmr1");
    t.set_rect(Rect::new(400.0, 8.0, 28.0, 28.0));
    t.props
        .insert("interval".into(), crawrad_ir::Value::Int(250));
    d.add_child("form_main", t).expect("the timer enters");
    d.validate().expect("valid");
    d
}

/// A non-visual component with data collection: `ImageList`. The form contains
/// only the non-visual component, generating the matching struct and clean state.
fn caso_so_image_list() -> Document {
    let mut d = form("form_main");
    let r = Registry::builtin();
    let n = r
        .instantiate("ImageList", "iml1", Rect::new(8.0, 8.0, 28.0, 28.0))
        .expect("the catalog knows ImageList");
    d.add_child("form_main", n).expect("enters");
    d.validate().expect("valid");
    d
}

/// An `ActionList` with child actions, generating code and dispatching an event.
fn caso_action_list_e_acoes() -> Document {
    let mut d = form("form_main");
    let r = Registry::builtin();
    let acl = r
        .instantiate("ActionList", "acl1", Rect::new(8.0, 8.0, 28.0, 28.0))
        .expect("the catalog knows ActionList");
    d.add_child("form_main", acl).expect("enters ActionList");
    let mut act = r
        .instantiate("Action", "act_salvar", Rect::new(8.0, 8.0, 28.0, 28.0))
        .expect("the catalog knows Action");
    act.events
        .insert("on_execute".into(), "act_salvar_execute".into());
    d.add_child("acl1", act).expect("enters Action");
    let mut btn = r
        .instantiate("Button", "btn_salvar", Rect::new(50.0, 50.0, 80.0, 28.0))
        .expect("the catalog knows Button");
    btn.props
        .insert("action".into(), crawrad_ir::Value::Ref("act_salvar".into()));
    d.add_child("form_main", btn).expect("enters Button");
    d.validate().expect("valid");
    d
}

/// Does the generator **call** this event's handler?
///
/// The question is posed to the generated text itself, not a list written here.
/// There are events that the catalog provides but the generator does not yet wire —
/// a `ListBox`'s `OnClick` is one of them —, and their stub is born dead in the
/// user file. That is a DEBT from another area, enforced by
/// `every_form_event_becomes_a_call_in_the_application` on the form side.
/// This gate is about warnings, and wiring an uncalled event here would measure
/// that debt instead of this one.
fn vira_chamada(classe: &str, evento: &str) -> bool {
    let r = Registry::builtin();
    let mut d = form("sonda");
    if classe == "Form" {
        // The root is nobody's child: form events are wired on IT.
        d.node_mut("sonda")
            .expect("the root")
            .events
            .insert(evento.into(), "sonda_handler".into());
    } else {
        let (w, h) = r.get(classe).expect("catalog class").default_size;
        let mut n = r
            .instantiate(classe, "sonda1", Rect::new(8.0, 8.0, w, h))
            .expect("the catalog instantiates");
        n.set_name("sonda1");
        n.events.insert(evento.into(), "sonda_handler".into());
        d.add_child("sonda", n).expect("enters");
    }
    d.validate().expect("valid");
    crawrad_codegen::GeradorRxp
        .gerar(&d, "sonda", &crawrad_registry::Registry::builtin())
        .expect("generate the probe")
        .iter()
        // `.sonda_handler(` with an OPEN parenthesis, for two reasons confirmed
        // by measurement:
        //
        // - `.sonda_handler()` and not `self.sonda_handler()` because
        //   `OnCreate` is called in `new()`, where `self` does not exist yet —
        //   the receiver is the local variable being assembled;
        // - and without the closing parenthesis because the EIGHT events with
        //   parameters are emitted as `.sonda_handler(e)`. With hardcoded `()`,
        //   they were excluded from this gate by construction, which is how
        //   `raw_scroll_delta` — a field egui 0.36 lacks — almost reached the
        //   user project without anything turning red.
        .any(|a| a.conteudo.contains(".sonda_handler("))
}

/// **One instance of every catalog class**, with or without events.
///
/// This sweep ensures this gate covers classes yet to be written: whoever
/// adds a component to the catalog passes through here unconditionally.
fn caso_o_catalogo_inteiro(com_eventos: bool) -> Document {
    let r = Registry::builtin();
    let mut d = form("form_main");
    let mut y = 8.0;
    for desc in r.all() {
        if desc.class == "Form" {
            // `Form` is not instantiated: it is ALREADY in the document as root.
            // Its events are placed on itself — without this, `OnCreate` and
            // `OnClose` code would never pass through `-D warnings`.
            if com_eventos {
                for e in &desc.events {
                    if vira_chamada(desc.class, e.name) {
                        let raiz = d.node_mut("form_main").expect("the root");
                        raiz.events
                            .insert(e.name.into(), handler("form_main", e.name));
                    }
                }
            }
            continue;
        }
        let id = r.suggest_id(desc.class, |i| d.node(i).is_some());
        let (w, h) = desc.default_size;
        let mut n = r
            .instantiate(desc.class, id.clone(), Rect::new(8.0, y, w, h))
            .expect("the catalog instantiates what it lists");
        if com_eventos {
            for e in &desc.events {
                if vira_chamada(desc.class, e.name) {
                    n.events.insert(e.name.into(), handler(&id, e.name));
                }
            }
        }
        d.add_child("form_main", n).expect("enters the form");
        y += h + 4.0;
    }
    d.validate().expect("valid");
    d
}

/// Container with child inside, where the generator's trickiest code lives:
/// `PageControl`'s page guard and mutual exclusion between sibling radio buttons
/// of the same parent.
///
/// **What this case nests, and what it DOES NOT prove.** It nests intentionally,
/// in all three existing forms: page inside page, standard container inside
/// page, and page inside standard container.
///
/// It was previously stated here that this was "why the gate passed green over
/// the bug" of the non-composing page guard — implying that nesting would close
/// that loophole. **Measured, and it does not.** This gate measures *compiler
/// warnings*, and an INCORRECT yet valid guard compiles cleanly:
/// `if self.pc_interno.active_page == 0` without the outer tab condition is
/// valid code, and the click reaches the button of a closed tab without `cargo`
/// saying a word. What catches that bug is `gate_fase3`, via click, alone.
///
/// What this case does is different, and worthwhile: it forces text emitted for
/// nested containers through `-D warnings` — unused variables, guards that read
/// nothing, unused anchors.
fn caso_conteineres() -> Document {
    let r = Registry::builtin();
    let mut d = form("form_main");
    let novo = |d: &mut Document, pai: &str, classe: &str, rect: Rect| -> String {
        let id = r.suggest_id(classe, |i| d.node(i).is_some());
        let n = r
            .instantiate(classe, id.clone(), rect)
            .expect("the catalog instantiates");
        d.add_child(pai, n).expect("enters");
        id
    };

    let pc = novo(
        &mut d,
        "form_main",
        "PageControl",
        Rect::new(8.0, 8.0, 400.0, 200.0),
    );
    let ts1 = novo(&mut d, &pc, "TabSheet", Rect::new(0.0, 24.0, 400.0, 176.0));
    let ts2 = novo(&mut d, &pc, "TabSheet", Rect::new(0.0, 24.0, 400.0, 176.0));
    novo(&mut d, &ts1, "Button", Rect::new(8.0, 8.0, 96.0, 32.0));
    novo(&mut d, &ts2, "Edit", Rect::new(8.0, 8.0, 145.0, 24.0));
    // A password field and a read-only Memo, so their DSL flags and run
    // time sync go through `-D warnings` too
    let senha = novo(&mut d, &ts2, "Edit", Rect::new(8.0, 40.0, 145.0, 24.0));
    let memo = novo(&mut d, &ts2, "Memo", Rect::new(8.0, 72.0, 145.0, 60.0));
    let props = |d: &mut Document, id: &str, k: &str, v: crawrad_ir::Value| {
        d.node_mut(id).expect("exists").props.insert(k.into(), v);
    };
    props(
        &mut d,
        &senha,
        "password_char",
        crawrad_ir::Value::Str("*".into()),
    );
    props(&mut d, &memo, "readonly", crawrad_ir::Value::Bool(true));

    // PAGE INSIDE PAGE: the bottom button guard must include both
    // conditions. That was what was missing here.
    let pc_dentro = novo(
        &mut d,
        &ts1,
        "PageControl",
        Rect::new(8.0, 48.0, 300.0, 120.0),
    );
    let ts_dentro = novo(
        &mut d,
        &pc_dentro,
        "TabSheet",
        Rect::new(0.0, 24.0, 300.0, 96.0),
    );
    novo(
        &mut d,
        &pc_dentro,
        "TabSheet",
        Rect::new(0.0, 24.0, 300.0, 96.0),
    );
    novo(
        &mut d,
        &ts_dentro,
        "Button",
        Rect::new(8.0, 8.0, 96.0, 32.0),
    );

    // STANDARD CONTAINER INSIDE PAGE: `Panel` adds no condition,
    // and traversal must not halt on it.
    let pn_na_pagina = novo(&mut d, &ts2, "Panel", Rect::new(8.0, 40.0, 200.0, 100.0));
    novo(
        &mut d,
        &pn_na_pagina,
        "Label",
        Rect::new(8.0, 8.0, 80.0, 16.0),
    );

    let gb = novo(
        &mut d,
        "form_main",
        "GroupBox",
        Rect::new(8.0, 220.0, 200.0, 120.0),
    );
    novo(
        &mut d,
        &gb,
        "RadioButton",
        Rect::new(8.0, 24.0, 120.0, 20.0),
    );
    novo(
        &mut d,
        &gb,
        "RadioButton",
        Rect::new(8.0, 48.0, 120.0, 20.0),
    );

    let pn = novo(
        &mut d,
        "form_main",
        "Panel",
        Rect::new(220.0, 220.0, 200.0, 120.0),
    );
    novo(&mut d, &pn, "CheckBox", Rect::new(8.0, 8.0, 120.0, 20.0));

    // PAGE INSIDE STANDARD CONTAINER: the reverse path of the above.
    let pc_no_painel = novo(
        &mut d,
        &pn,
        "PageControl",
        Rect::new(8.0, 32.0, 180.0, 80.0),
    );
    let ts_no_painel = novo(
        &mut d,
        &pc_no_painel,
        "TabSheet",
        Rect::new(0.0, 24.0, 180.0, 56.0),
    );
    novo(
        &mut d,
        &ts_no_painel,
        "CheckBox",
        Rect::new(8.0, 8.0, 120.0, 20.0),
    );

    d.validate().expect("valid");
    d
}

/// **The controls' routed events, few at a time** (`control_events` in the
/// egui target). The whole catalogue with every event wired uses every arm at
/// once; these are the thin ends — one arm (`if let`, no position, no
/// modifiers), focus alone, and a control inside a container, whose rect is
/// left from inside the container's closure. Each `(parent, class, events)`
/// goes in that order; a parent is an index into the list, or the form.
fn case_routed_events(controls: &[(Option<usize>, &str, &[&str])]) -> Document {
    let r = Registry::builtin();
    let mut d = form("form_main");
    let mut ids: Vec<String> = Vec::new();
    for (i, (parent, class, events)) in controls.iter().enumerate() {
        let id = r.suggest_id(class, |x| d.node(x).is_some());
        let (w, h) = r.get(class).expect("catalog class").default_size;
        let mut n = r
            .instantiate(
                class,
                id.clone(),
                Rect::new(8.0, 8.0 + 40.0 * i as f32, w, h),
            )
            .expect("the catalog instantiates");
        for e in *events {
            n.events.insert((*e).into(), handler(&id, e));
        }
        let parent = parent.map_or("form_main".to_string(), |p| ids[p].clone());
        d.add_child(&parent, n).expect("enters");
        ids.push(id);
    }
    d.validate().expect("valid");
    d
}

/// The form's own `Hint`: the flag the controls set and the tooltip
/// over the body, with controls on it and with none.
fn case_form_hint(controls: &[(Option<usize>, &str, &[&str])]) -> Document {
    let mut d = case_routed_events(controls);
    d.node_mut("form_main")
        .expect("the root")
        .props
        .insert("hint".into(), crawrad_ir::Value::Str("the form".into()));
    d
}

/// Gate cases, in the order most helpful to read when one fails.
fn casos() -> Vec<(&'static str, Document)> {
    vec![
        ("blank form", caso_em_branco()),
        ("a timer and nothing else", caso_so_um_timer()),
        ("an image list and nothing else", caso_so_image_list()),
        ("an action list with actions", caso_action_list_e_acoes()),
        ("one of every class", caso_o_catalogo_inteiro(false)),
        (
            "one of every class, every event wired",
            caso_o_catalogo_inteiro(true),
        ),
        ("containers with children", caso_conteineres()),
        (
            "a label's click and nothing else",
            case_routed_events(&[(None, "Label", &["on_click"])]),
        ),
        (
            "a button's focus and nothing else",
            case_routed_events(&[(None, "Button", &["on_enter"]), (None, "Edit", &[])]),
        ),
        ("a form's hint and nothing else", case_form_hint(&[])),
        (
            "a form's hint over a button and a panel",
            case_form_hint(&[
                (None, "Button", &[]),
                (None, "Panel", &[]),
                (Some(1), "Label", &[]),
            ]),
        ),
        (
            "a routed control inside a panel",
            case_routed_events(&[
                (None, "Panel", &["on_click"]),
                (Some(0), "Button", &["on_mouse_enter", "on_key_down"]),
            ]),
        ),
    ]
}

/// A `cargo` invocation inside the generated project. Same as Phase 3 gate,
/// and for the same reason: outer cargo variables would confuse the inner one.
fn cargo(destino: &Path, args: &[&str]) -> (bool, String) {
    let saida = Command::new(env!("CARGO"))
        .args(args)
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("RUSTFLAGS")
        .current_dir(destino)
        .output()
        .expect("call cargo inside the generated project");
    let texto = format!(
        "{}{}",
        String::from_utf8_lossy(&saida.stdout),
        String::from_utf8_lossy(&saida.stderr)
    );
    (saida.status.success(), texto)
}

/// The GROUP root, written before `cargo new` — as the IDE does (ADR 0018).
const RAIZ_DO_GRUPO: &str = r#"[workspace]
members = ["app"]
resolver = "2"
default-members = ["app"]
"#;

/// Is the draft **intact**, or is only a fragment left?
///
/// Sibling to the Phase 3 gate function, and for the same defect: the draft lives
/// in `target/`, `Swatinem/rust-cache` restores a pruned `target/` at the start of
/// each CI run, and pruning leaves `app/` standing without `Cargo.toml` inside.
/// A guard checking only the manifest concluded "there is no project", while
/// `cargo new` saw the directory and concluded "there already is", killing the
/// gate before asserting anything. **One file does not prove a project:** here
/// the entire set is checked, and if any piece is missing, the draft is wiped
/// and rebuilt. Why it remains inside `target/` is explained in detail in
/// `gate_fase3.rs` under `rascunho_intacto`.
///
/// **The set here is smaller than there, intentionally:** this gate lacks the mock
/// repository — it writes two artifacts (the group root and what `cargo` puts
/// inside the member), and checks for those two. The rule is the same:
/// *the guard checks for everything setup writes.*
///
/// The group root is checked for EQUALITY via [`manifesto_intocado`] shared by
/// both gates. A `contains("[workspace]")` used to live here, and was insufficient
/// for reasons measured on its sibling: a manifest that OPENS and is not ours
/// reports "intact", nothing rewrites the file, and clippy inside the member dies
/// against a root that does not list it — on every subsequent run, never healing,
/// because a guard that says "intact" never rewrites the file. `.exists()` is even
/// worse: a zero-byte `Cargo.toml` exists but cannot open.
fn rascunho_intacto(grupo: &Path, destino: &Path) -> bool {
    if !manifesto_intocado(grupo, RAIZ_DO_GRUPO) || !destino.join("src").is_dir() {
        return false;
    }
    // The generated egui code calls `crawrad_runtime::file_dialog`.
    std::fs::read_to_string(destino.join("Cargo.toml"))
        .is_ok_and(|m| m.contains("eframe") && m.contains("crawrad-runtime"))
}

/// The `cargo new` + `cargo add` the IDE runs before generation touches the
/// folder. Only sets up what does not exist yet: the gate generates multiple times
/// into the same destination intentionally to keep `target/` warm between cases.
fn preparar_projeto(destino: &Path) {
    let grupo = destino
        .parent()
        .expect("the destination is a member of a group");
    if rascunho_intacto(grupo, destino) {
        return;
    }

    // A partial draft cannot be reused. `cargo new` rejects an existing destination,
    // which is exactly why this `remove_dir_all` is here.
    if destino.exists() {
        std::fs::remove_dir_all(destino).expect("delete the half draft");
    }
    std::fs::create_dir_all(grupo).expect("create the group root");
    std::fs::write(grupo.join("Cargo.toml"), RAIZ_DO_GRUPO).expect("the group root");

    let nome = destino
        .file_name()
        .and_then(|n| n.to_str())
        .expect("the name");
    let (ok, texto) = cargo(
        grupo,
        &[
            "new",
            "--bin",
            "--vcs",
            "none",
            "--name",
            "gate_limpo",
            nome,
        ],
    );
    assert!(ok, "`cargo new` failed:\n{texto}");

    let (ok, offline) = cargo(destino, &["add", "eframe@0.36", "--offline"]);
    if !ok {
        let (ok, online) = cargo(destino, &["add", "eframe@0.36"]);
        assert!(
            ok,
            "`cargo add` failed offline AND online — without the crates.io index \
             this gate cannot set up the project.\n--- offline ---\n{offline}\
             \n--- online ---\n{online}"
        );
    }

    // Same dependency the IDE writes for an egui form (`dependencias_do_projeto`).
    let runtime = Path::new(env!("CARGO_MANIFEST_DIR")).join("../crawrad-runtime");
    let runtime = runtime.to_str().expect("the runtime path");
    let (ok, texto) = cargo(destino, &["add", "--path", runtime, "--offline"]);
    assert!(ok, "the `cargo add` of crawrad-runtime failed:\n{texto}");
}

/// Deletes all of `src/` before each case.
///
/// The user file is `Dono::Usuario` and only grows; leaving it across cases
/// would cause the gate to measure a mix that no project has. Each case is a
/// freshly created project, which is what must be proven.
fn limpar_src(destino: &Path) {
    let src = destino.join("src");
    for e in std::fs::read_dir(&src).expect("read src") {
        let p = e.expect("an entry").path();
        if p.extension().is_some_and(|x| x == "rs") {
            std::fs::remove_file(&p).expect("delete the old file");
        }
    }
}

/// **The gate.** Every case in the list must succeed without a single warning line.
#[test]
#[ignore = "builds a real project; run with --ignored"]
fn o_projeto_gerado_nao_nasce_com_aviso() {
    let destino: PathBuf =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/gate-sem-avisos/app");
    preparar_projeto(&destino);

    let mut sujos = Vec::new();
    for (rotulo, doc) in casos() {
        limpar_src(&destino);
        let arqs = crawrad_codegen::GeradorRxp
            .gerar(&doc, "gate_limpo", &crawrad_registry::Registry::builtin())
            .unwrap_or_else(|e| panic!("generate `{rotulo}`: {e}"));
        crawrad_codegen::escrever(&destino, &arqs).expect("write the project");

        // `-D warnings` AFTER `--`: applies to the user's crate and not
        // to its dependencies.
        let (ok, texto) = cargo(
            &destino,
            &["clippy", "--all-targets", "--", "-D", "warnings"],
        );
        if !ok {
            sujos.push(format!("=== {rotulo} ===\n{texto}"));
        }
        println!("{rotulo}: {}", if ok { "clean" } else { "DIRTY" });
    }

    assert!(
        sujos.is_empty(),
        "the generated project was born with a warning — and the user cannot fix it, \
         because `.designer.rs` is rewritten on the next generation:\n\n{}",
        sujos.join("\n")
    );
}

/// A project that ships languages compiles clean too — the form's
/// `apply_language`, the app's `set_language` and the generated `src/locales.rs`
/// (with `sys-locale`) pass `clippy -D warnings` in the user's crate.
#[test]
#[ignore = "builds a real project; run with --ignored"]
fn a_translated_project_is_born_clean() {
    let dest: PathBuf =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/gate-translated/app");
    preparar_projeto(&dest);
    let (ok, out) = cargo(&dest, &["add", "sys-locale@0.3", "--offline"]);
    if !ok {
        let (ok, online) = cargo(&dest, &["add", "sys-locale@0.3"]);
        assert!(ok, "the `cargo add` of sys-locale failed:\n{out}\n{online}");
    }
    limpar_src(&dest);

    let mut doc = form("form_main");
    let mut button = Node::new("btn_ok", "Button");
    button.set_name("btn_ok");
    button.set_rect(Rect::new(10.0, 10.0, 80.0, 24.0));
    button.set_text("OK \"now\"");
    doc.add_child("form_main", button).expect("the button");

    let files = {
        let _scope = crawrad_codegen::LocalesScope::new(true);
        crawrad_codegen::GeradorRxp
            .gerar(&doc, "gate_limpo", &Registry::builtin())
            .expect("generate the translated form")
    };
    crawrad_codegen::escrever(&dest, &files).expect("write the project");
    let locales = crawrad_codegen::locales::locales_module(&[(
        "de".to_string(),
        vec![(
            "form_main.btn_ok.text".to_string(),
            "Jetzt \"ok\" — ä".to_string(),
            "OK \"now\"".to_string(),
        )],
    )]);
    std::fs::write(dest.join("src/locales.rs"), locales).expect("write src/locales.rs");

    let (ok, out) = cargo(&dest, &["clippy", "--all-targets", "--", "-D", "warnings"]);
    assert!(ok, "the translated project was born with a warning:\n{out}");
}

/// The Makepad member is whole: the group root is ours and the member carries
/// the crates the IDE adds for a Makepad form.
fn makepad_draft_intact(group: &Path, dest: &Path) -> bool {
    if !manifesto_intocado(group, RAIZ_DO_GRUPO) || !dest.join("src").is_dir() {
        return false;
    }
    std::fs::read_to_string(dest.join("Cargo.toml"))
        .is_ok_and(|m| m.contains("makepad-widgets") && m.contains("crawrad-runtime-makepad"))
}

/// The `cargo new` + the dependencies the IDE writes for a Makepad form
/// (`dependencias_do_projeto`), built only when the draft is not whole.
fn prepare_makepad_project(dest: &Path) {
    let group = dest.parent().expect("the member sits in a group");
    if makepad_draft_intact(group, dest) {
        return;
    }
    if dest.exists() {
        std::fs::remove_dir_all(dest).expect("drop the half draft");
    }
    std::fs::create_dir_all(group).expect("the group root");
    std::fs::write(group.join("Cargo.toml"), RAIZ_DO_GRUPO).expect("the group root");
    let name = dest.file_name().and_then(|n| n.to_str()).expect("the name");
    let (ok, out) = cargo(
        group,
        &[
            "new",
            "--bin",
            "--vcs",
            "none",
            "--name",
            "gate_limpo",
            name,
        ],
    );
    assert!(ok, "`cargo new` failed:\n{out}");
    for dep in
        crawrad_codegen::dependencias_do_projeto(crawrad_codegen::AlvoGerador::Makepad, true, false)
    {
        let args = dep.args_cargo_add();
        let mut cmd = vec!["add"];
        cmd.extend(args.iter().map(|s| s.as_str()));
        let (ok, out) = cargo(dest, &cmd);
        assert!(ok, "`cargo add` of `{}` failed:\n{out}", dep.nome());
    }
}

/// **The gate, on Makepad**. Every case leaves the Makepad target with
/// not one warning either: the egui gate alone let ten lints sit in the
/// Makepad actions of the Edit, the Memo and the CheckBox, and nobody saw.
#[test]
#[ignore = "builds a real project; run with --ignored"]
fn the_makepad_project_is_born_without_a_warning() {
    let dest: PathBuf =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/gate-sem-avisos-makepad/app");
    prepare_makepad_project(&dest);

    let mut dirty = Vec::new();
    for (label, doc) in casos() {
        limpar_src(&dest);
        let files = crawrad_codegen::GeradorMakepad
            .gerar(&doc, "gate_limpo", &crawrad_registry::Registry::builtin())
            .unwrap_or_else(|e| panic!("generating `{label}`: {e}"));
        crawrad_codegen::escrever(&dest, &files).expect("writing the project");
        let (ok, out) = cargo(&dest, &["clippy", "--all-targets", "--", "-D", "warnings"]);
        if !ok {
            dirty.push(format!("=== {label} ===\n{out}"));
        }
        println!("{label}: {}", if ok { "clean" } else { "DIRTY" });
    }

    assert!(
        dirty.is_empty(),
        "the generated Makepad project was born with a warning — and the user \
         cannot fix it, since the `.designer.rs` is rewritten on the next \
         generation:\n\n{}",
        dirty.join("\n")
    );
}

/// The base and the derived form, both generated for Makepad.
fn inheritance_pair() -> (Document, Document) {
    let mut base = form("form_parent");
    let mut save = Node::new("btn_save", "Button");
    save.set_name("btn_save");
    save.set_text("Save");
    save.set_rect(Rect::new(20.0, 20.0, 120.0, 32.0));
    save.events
        .insert("on_click".into(), "btn_save_click".into());
    base.add_child("form_parent", save).unwrap();
    let mut status = Node::new("lbl_status", "Label");
    status.set_name("lbl_status");
    status.set_text("Base label");
    status.set_rect(Rect::new(20.0, 70.0, 200.0, 24.0));
    base.add_child("form_parent", status).unwrap();

    let mut derived = form("form_child");
    derived.set_inherits(Some("form_parent"));
    let mut extra = Node::new("btn_extra", "Button");
    extra.set_name("btn_extra");
    extra.set_text("Extra");
    extra.set_rect(Rect::new(160.0, 20.0, 120.0, 32.0));
    derived.add_child("form_child", extra).unwrap();
    (base, derived)
}

/// On Makepad: a derived form draws the base's components, and the
/// project the two forms make builds without a warning.
#[test]
#[ignore = "builds a real project; run with --ignored"]
fn the_derived_makepad_form_draws_the_base_and_builds_clean() {
    let dest: PathBuf =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/gate-makepad-inheritance/app");
    prepare_makepad_project(&dest);
    limpar_src(&dest);

    let reg = Registry::builtin();
    let (base, derived) = inheritance_pair();
    let base_files = crawrad_codegen::GeradorMakepad
        .generate_with_base(&base, "gate_limpo", &reg, None, true)
        .expect("generating the base");
    let derived_files = crawrad_codegen::GeradorMakepad
        .generate_with_base(&derived, "gate_limpo", &reg, Some(&base), false)
        .expect("generating the derived form");
    // `escrever` replaces the whole set, so the pair is written by hand
    for a in derived_files.iter().chain(&base_files) {
        if a.caminho != Path::new("src/main.rs") || a.conteudo.contains("mod form_child;") {
            std::fs::write(dest.join(&a.caminho), &a.conteudo).expect("writing the pair");
        }
    }
    let main = dest.join("src/main.rs");
    let text = std::fs::read_to_string(&main).expect("main.rs");
    let text = text.replacen(
        "mod form_child;",
        "mod form_parent;\n#[path = \"form_parent.designer.rs\"]\npub mod form_parent_designer;\nmod form_child;",
        1,
    );
    std::fs::write(&main, text).expect("main.rs");

    let designer = std::fs::read_to_string(dest.join("src/form_child.designer.rs")).unwrap();
    for widget in [
        "btn_save = <Button>",
        "lbl_status = <View>",
        "btn_extra = <Button>",
    ] {
        assert_eq!(designer.matches(widget).count(), 1, "{widget} drawn once");
    }
    let (ok, out) = cargo(&dest, &["clippy", "--all-targets", "--", "-D", "warnings"]);
    assert!(ok, "the derived Makepad form must build clean:\n{out}");

    // the derived form overrides the base's Save and calls it (`inherited`)
    let mut overriding = derived;
    let mut save = Node::new("btn_save", "Button");
    save.set_name("btn_save");
    save.events
        .insert("on_click".into(), "btn_child_save_click".into());
    overriding.add_child("form_child", save).unwrap();
    let files = crawrad_codegen::GeradorMakepad
        .generate_with_base(&overriding, "gate_limpo", &reg, Some(&base), false)
        .expect("generating the overriding form");
    for a in &files {
        if a.caminho != Path::new("src/main.rs") {
            std::fs::write(dest.join(&a.caminho), &a.conteudo).expect("writing the override");
        }
    }
    let designer = std::fs::read_to_string(dest.join("src/form_child.designer.rs")).unwrap();
    assert_eq!(designer.matches("btn_save = <Button>").count(), 1);
    let (ok, out) = cargo(&dest, &["clippy", "--all-targets", "--", "-D", "warnings"]);
    assert!(ok, "the overriding Makepad form must build clean:\n{out}");
}

/// One form of the three-level chain: `name` inherits `inherits` and, with a
/// `handler`, overrides the click of the first base's `btn_save`.
fn chain_level(name: &str, inherits: Option<&str>, handler: Option<&str>) -> Document {
    let mut doc = form(name);
    doc.set_inherits(inherits);
    if inherits.is_none() {
        let mut status = Node::new("lbl_status", "Label");
        status.set_name("lbl_status");
        status.set_text("Status");
        status.set_rect(Rect::new(20.0, 70.0, 300.0, 24.0));
        doc.add_child(name, status).unwrap();
    }
    if inherits.is_none() || handler.is_some() {
        let mut save = Node::new("btn_save", "Button");
        save.set_name("btn_save");
        save.set_text("Save");
        save.set_rect(Rect::new(20.0, 20.0, 120.0, 32.0));
        if let Some(h) = handler {
            save.events.insert("on_click".into(), h.into());
        }
        doc.add_child(name, save).unwrap();
    }
    doc
}

/// Three levels, on Makepad: C over B over A, each overriding the
/// click; the project builds without a warning, and C draws A's components.
#[test]
#[ignore = "builds a real project; run with --ignored"]
fn a_three_level_makepad_chain_builds_clean() {
    let dest: PathBuf =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/gate-makepad-three-levels/app");
    prepare_makepad_project(&dest);
    limpar_src(&dest);

    let reg = Registry::builtin();
    let a = chain_level("form_a", None, Some("btn_save_click"));
    let b = chain_level("form_b", Some("form_a"), Some("btn_b_save_click"));
    let c = chain_level("form_c", Some("form_b"), Some("btn_c_save_click"));
    let level = |doc: &Document, bases: &[Document], is_base: bool| {
        crawrad_codegen::GeradorMakepad
            .generate_with_bases(doc, "gate_limpo", &reg, bases, is_base)
            .expect("generating a level")
    };
    let files = [
        level(&c, &[b.clone(), a.clone()], false),
        level(&b, std::slice::from_ref(&a), true),
        level(&a, &[], true),
    ];
    for (i, set) in files.iter().enumerate() {
        for f in set {
            if i == 0 || f.caminho != Path::new("src/main.rs") {
                std::fs::write(dest.join(&f.caminho), &f.conteudo).expect("writing a level");
            }
        }
    }
    let main = dest.join("src/main.rs");
    let text = std::fs::read_to_string(&main).expect("main.rs");
    let mods = "mod form_a;\n#[path = \"form_a.designer.rs\"]\npub mod form_a_designer;\n\
                mod form_b;\n#[path = \"form_b.designer.rs\"]\npub mod form_b_designer;\nmod form_c;";
    std::fs::write(&main, text.replacen("mod form_c;", mods, 1)).expect("main.rs");

    let designer = std::fs::read_to_string(dest.join("src/form_c.designer.rs")).unwrap();
    for widget in ["btn_save = <Button>", "lbl_status = <View>"] {
        assert_eq!(
            designer.matches(widget).count(),
            1,
            "{widget} drawn once in C"
        );
    }
    let (ok, out) = cargo(&dest, &["clippy", "--all-targets", "--", "-D", "warnings"]);
    assert!(ok, "the three-level Makepad chain must build clean:\n{out}");
}

/// **The gate's gate, on this side.**
///
/// IT WAS PREVIOUSLY STATED HERE that "since it is the same rule it is TWO copies —
/// so each copy needs its own gate, otherwise one will diverge on its own again".
/// The prediction was right and the remedy wrong: one diverged anyway in the next
/// commit, **because each copy's gate only watches its own**. Both guards were
/// correct, each proven in its own file, and neither could notice the other had
/// changed. Two watchmen do not make a watchman of the pair.
///
/// The proper remedy was eliminating duplication, which is what exists today:
/// the criterion for "this manifest is mine" is a single function in `comum/mod.rs`,
/// and can no longer diverge. What legitimately remains different is the SET of
/// artifacts — three there, two here —, and each table tests that at home, where
/// the difference belongs.
///
/// The set here has three pieces (group root, `src/`, and `eframe`), and each
/// state below is a way disk state returns from cache pruning — or Ctrl-C —
/// missing one of them. The two exceptions are `empty group root` and
/// `group root with OTHER content`, which come from editing [`RAIZ_DO_GRUPO`]
/// while leaving the previous version's draft on disk.
///
/// Mutation proof: reverting the guard to `.exists()` on group root fails
/// `empty group root`; replacing `==` in [`manifesto_intocado`] with
/// `contains("[workspace]")` — literally what this file had — fails
/// `group root with OTHER content`, and only that; removing the check for
/// `src/` fails `without `src/``; removing the check for `eframe` fails
/// `member without `cargo add``.
#[test]
fn o_guarda_reprova_todo_rascunho_pela_metade() {
    type Poda = fn(&Path, &Path);
    let casos: &[(&str, Poda, bool)] = &[
        ("intact", |_, _| {}, true),
        (
            "without the group root",
            |grupo, _| std::fs::remove_file(grupo.join("Cargo.toml")).unwrap(),
            false,
        ),
        (
            "empty group root",
            |grupo, _| std::fs::write(grupo.join("Cargo.toml"), "").unwrap(),
            false,
        ),
        // The missing line that the sibling already had. A `[workspace]` that
        // OPENS and is not ours: without `members = ["app"]`, clippy inside
        // the member dies against a root that does not list it, and the guard
        // returning "intact" never rewrote the file — on every subsequent run,
        // without self-healing.
        (
            "group root with OTHER content",
            |grupo, _| {
                std::fs::write(grupo.join("Cargo.toml"), "[workspace]\nmembers = []\n").unwrap()
            },
            false,
        ),
        (
            "without `src/`",
            |_, dest| std::fs::remove_dir_all(dest.join("src")).unwrap(),
            false,
        ),
        (
            "without the member manifest",
            |_, dest| std::fs::remove_file(dest.join("Cargo.toml")).unwrap(),
            false,
        ),
        (
            "member without `cargo add`",
            |_, dest| {
                std::fs::write(
                    dest.join("Cargo.toml"),
                    "[package]\nname = \"gate_limpo\"\n",
                )
                .unwrap()
            },
            false,
        ),
        (
            "member without crawrad-runtime",
            |_, dest| {
                std::fs::write(
                    dest.join("Cargo.toml"),
                    "[package]\nname = \"gate_limpo\"\n\n[dependencies]\neframe = \"0.36\"\n",
                )
                .unwrap()
            },
            false,
        ),
    ];

    let base = std::env::temp_dir().join("crawrad_gate_sem_avisos_guarda");
    let _ = std::fs::remove_dir_all(&base);
    let mut errados: Vec<&str> = Vec::new();
    for (i, (estado, podar, serve)) in casos.iter().enumerate() {
        let grupo = base.join(format!("caso{i}")).join("grupo");
        let destino = grupo.join("app");
        std::fs::create_dir_all(destino.join("src")).unwrap();
        std::fs::write(grupo.join("Cargo.toml"), RAIZ_DO_GRUPO).unwrap();
        std::fs::write(
            destino.join("Cargo.toml"),
            "[package]\nname = \"gate_limpo\"\n\n[dependencies]\neframe = \"0.36\"\n\
             crawrad-runtime = { path = \"../crawrad-runtime\" }\n",
        )
        .unwrap();
        podar(&grupo, &destino);
        if rascunho_intacto(&grupo, &destino) != *serve {
            errados.push(estado);
        }
    }
    assert!(
        errados.is_empty(),
        "the guard answered the opposite of what these disks deserve: {errados:?}"
    );
    let _ = std::fs::remove_dir_all(&base);
}
