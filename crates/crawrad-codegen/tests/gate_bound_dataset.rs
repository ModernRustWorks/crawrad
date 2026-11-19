//! The gate of the **bound controls**: an `Edit` and a `CheckBox` bound
//! to a `Dataset` generate, on both targets, a project that compiles clean.
//!
//! The unit tests in `src/test/data_binding_to_code` check the text; this one
//! asks the compiler. The generated code names `crawrad_binding::PortaDbBridge`,
//! `DatasetAdapter` and the widgets' setters -- none of it is checked until a
//! real project with the crates `component_dependencies` lists builds it.
//!
//! `#[ignore]` like the other gates: it calls `cargo` inside `cargo`.
//!
//! ```text
//! cargo test -p crawrad-codegen --test gate_bound_dataset -- --ignored --nocapture
//! ```

mod comum;

use comum::manifesto_intocado;
use crawrad_ir::binding::{BindingDef, BindingMode};
use crawrad_ir::{Document, Rect, Value};
use crawrad_package::manifest::Manifest;
use crawrad_registry::Registry;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The catalog with the `Connection` read from the real `manifest.json`.
fn catalog() -> Registry {
    let text = include_str!("../../../components/portadb-crawrad/manifest.json");
    let manifest: Manifest = serde_json::from_str(text).expect("valid manifest");
    let mut r = Registry::builtin();
    for c in manifest.to_components().expect("the manifest converts") {
        r.register_external(c).expect("the catalog takes it");
    }
    r
}

/// `conn1` on a SQLite file, `dst1` on its `clients` table, an Edit bound
/// TwoWay to `name`, a CheckBox OneWay to `active` and one OneTime to `vip`.
fn form(r: &Registry) -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").expect("the root");
        root.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        root.set_name("form_main");
        root.set_text("Clients");
    }
    let mut engine = r
        .instantiate("Connection", "conn1", Rect::new(8.0, 8.0, 28.0, 28.0))
        .expect("Connection instantiates");
    engine.set_name("conn1");
    engine
        .props
        .insert("url".to_string(), Value::Str("sqlite://shop.db".into()));
    d.add_child("form_main", engine)
        .expect("the engine goes in");

    let mut dataset = r
        .instantiate("Dataset", "dst1", Rect::new(40.0, 8.0, 28.0, 28.0))
        .expect("Dataset instantiates");
    dataset.set_name("dst1");
    dataset
        .props
        .insert("connection".to_string(), Value::Ref("conn1".into()));
    dataset
        .props
        .insert("table".to_string(), Value::Str("clients".into()));
    d.add_child("form_main", dataset)
        .expect("the dataset goes in");

    let controls: [(&str, &str, f32, &str, &str, BindingMode); 3] = [
        (
            "Edit",
            "edit_name",
            48.0,
            "text",
            "name",
            BindingMode::TwoWay,
        ),
        (
            "CheckBox",
            "chk_active",
            80.0,
            "checked",
            "active",
            BindingMode::OneWay,
        ),
        (
            "CheckBox",
            "chk_vip",
            112.0,
            "checked",
            "vip",
            BindingMode::OneTime,
        ),
    ];
    for (class, name, y, prop, column, mode) in controls {
        let mut node = r
            .instantiate(class, name, Rect::new(16.0, y, 200.0, 24.0))
            .expect("the control instantiates");
        node.set_name(name);
        node.set_binding(prop, BindingDef::new("dst1", mode).with_field(column));
        d.add_child("form_main", node).expect("the control goes in");
    }

    let mut acl = r
        .instantiate("ActionList", "acl1", Rect::new(80.0, 8.0, 28.0, 28.0))
        .expect("ActionList instantiates");
    acl.set_name("acl1");
    d.add_child("form_main", acl)
        .expect("the ActionList goes in");

    // every dataset operation, each on its own button: the generated app compiles
    // the dispatch and the enabled rule of all ten, not just a sample
    let operations = [
        "First", "Prior", "Next", "Last", "Insert", "Edit", "Delete", "Post", "Cancel", "Refresh",
    ];
    for (i, op) in operations.iter().enumerate() {
        let act_name = format!("act_{}", op.to_ascii_lowercase());
        let btn_name = format!("btn_{}", op.to_ascii_lowercase());
        let mut act = r
            .instantiate("Action", &act_name, Rect::new(0.0, 0.0, 28.0, 28.0))
            .expect("Action instantiates");
        act.set_name(&act_name);
        act.props
            .insert("dataset".to_string(), Value::Ref("dst1".into()));
        act.props
            .insert("dataset_operation".to_string(), Value::Enum((*op).into()));
        act.props
            .insert("text".to_string(), Value::Str((*op).into()));
        d.add_child("acl1", act).expect("the action goes in");

        let x = 16.0 + 100.0 * (i % 5) as f32;
        let y = 144.0 + 32.0 * (i / 5) as f32;
        let mut btn = r
            .instantiate("Button", &btn_name, Rect::new(x, y, 96.0, 28.0))
            .expect("Button instantiates");
        btn.set_name(&btn_name);
        btn.props.insert("action".to_string(), Value::Ref(act_name));
        d.add_child("form_main", btn).expect("the button goes in");
    }

    d.validate().expect("valid");
    d
}

struct Target {
    name: &'static str,
    generator: Box<dyn crawrad_codegen::Gerador>,
    kind: crawrad_codegen::AlvoGerador,
    toolkit: &'static str,
}

fn targets() -> Vec<Target> {
    vec![
        Target {
            name: "egui",
            generator: Box::new(crawrad_codegen::GeradorRxp),
            kind: crawrad_codegen::AlvoGerador::Egui,
            toolkit: "eframe",
        },
        Target {
            name: "makepad",
            generator: Box::new(crawrad_codegen::GeradorMakepad),
            kind: crawrad_codegen::AlvoGerador::Makepad,
            toolkit: "makepad-widgets",
        },
    ]
}

/// A `cargo` inside the generated project, without the outer cargo's variables.
fn cargo(dest: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO"))
        .args(args)
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("RUSTFLAGS")
        .current_dir(dest)
        .output()
        .expect("run cargo inside the generated project");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (out.status.success(), text)
}

const GROUP_ROOT: &str = r#"[workspace]
members = ["app"]
resolver = "2"
default-members = ["app"]
"#;

/// The draft is whole when the group root is ours and the member already has
/// the toolkit and the grid crates; anything less is rebuilt from scratch.
fn draft_intact(group: &Path, dest: &Path, toolkit: &str) -> bool {
    if !manifesto_intocado(group, GROUP_ROOT) || !dest.join("src").is_dir() {
        return false;
    }
    std::fs::read_to_string(dest.join("Cargo.toml")).is_ok_and(|m| {
        m.contains(toolkit) && m.contains("crawrad-binding") && m.contains("portadb-crawrad")
    })
}

/// The `cargo new` and the `cargo add`s the IDE runs: the target's crates,
/// then the ones THIS form's components need.
fn prepare(dest: &Path, target: &Target, doc: &Document) {
    let group = dest.parent().expect("the member has a group");
    if draft_intact(group, dest, target.toolkit) {
        return;
    }
    if dest.exists() {
        std::fs::remove_dir_all(dest).expect("remove the half draft");
    }
    std::fs::create_dir_all(group).expect("create the group root");
    std::fs::write(group.join("Cargo.toml"), GROUP_ROOT).expect("the group root");

    let name = dest.file_name().and_then(|n| n.to_str()).expect("the name");
    let (ok, text) = cargo(
        group,
        &[
            "new",
            "--bin",
            "--vcs",
            "none",
            "--name",
            "gate_bound_dataset",
            name,
        ],
    );
    assert!(ok, "`cargo new` failed:\n{text}");

    let deps = crawrad_codegen::dependencias_do_projeto(target.kind, true, false)
        .into_iter()
        .chain(crawrad_codegen::component_dependencies(doc, target.kind));
    for dep in deps {
        let args = dep.args_cargo_add();
        let mut cmd = vec!["add"];
        cmd.extend(args.iter().map(String::as_str));
        let (ok, text) = cargo(dest, &cmd);
        assert!(ok, "`cargo add` of `{}` failed:\n{text}", dep.nome());
    }
    // The `Connection` is a third-party component: the IDE adds it from the
    // installed catalog, by path, like `gate_terceiro_desenha` does.
    let engine = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../components/portadb-crawrad")
        .canonicalize()
        .expect("`components/portadb-crawrad` is in the repository");
    let (ok, text) = cargo(dest, &["add", "--path", engine.to_str().expect("the path")]);
    assert!(ok, "`cargo add` of portadb-crawrad failed:\n{text}");
}

fn clean_src(dest: &Path) {
    for e in std::fs::read_dir(dest.join("src")).expect("read src") {
        let p = e.expect("an entry").path();
        if p.extension().is_some_and(|x| x == "rs") {
            std::fs::remove_file(&p).expect("remove the old file");
        }
    }
}

/// **The gate.** On both targets the bound controls generate and compile clean.
#[test]
#[ignore = "builds a real project; run with --ignored"]
fn a_bound_dataset_project_compiles_clean_on_both_targets() {
    let reg = catalog();
    let doc = form(&reg);
    let mut dirty = Vec::new();
    for target in targets() {
        let dest: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
            "../../target/gate-bound-dataset-{}/app",
            target.name
        ));
        prepare(&dest, &target, &doc);
        clean_src(&dest);

        let files = target
            .generator
            .gerar(&doc, "gate_bound_dataset", &reg)
            .expect("generate");
        crawrad_codegen::escrever(&dest, &files).expect("write the project");
        let designer = files
            .iter()
            .find(|a| a.caminho.to_string_lossy().contains("designer"))
            .map(|a| a.conteudo.clone())
            .unwrap_or_default();
        assert!(
            designer.contains("PortaDbBridge::open_table"),
            "[{}] the controls were not generated bound",
            target.name
        );

        let (ok, text) = cargo(&dest, &["clippy", "--", "-D", "warnings"]);
        println!("{}: {}", target.name, if ok { "clean" } else { "DIRTY" });
        if !ok {
            dirty.push(format!("=== {} ===\n{text}", target.name));
        }
    }
    assert!(
        dirty.is_empty(),
        "the bound dataset project does not compile clean:\n\n{}",
        dirty.join("\n")
    );
}
