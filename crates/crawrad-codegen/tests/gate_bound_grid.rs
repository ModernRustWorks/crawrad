//! The gate of the **bound grid**: a `Grid` pointing at a `Connection`
//! generates, on both targets, a project that compiles clean.
//!
//! The unit tests in `src/test/bound_grid` check the text; this one asks the
//! compiler. The generated code names `datagrid_sql`, `crawrad_runtime::grid` and
//! (on egui) `crawrad_runtime_egui::data_grid` — none of it is checked until a
//! real project with the crates `component_dependencies` lists builds it.
//!
//! `#[ignore]` like the other gates: it calls `cargo` inside `cargo`.
//!
//! ```text
//! cargo test -p crawrad-codegen --test gate_bound_grid -- --ignored --nocapture
//! ```

mod comum;

use comum::manifesto_intocado;
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

/// `conn1` on a SQLite file and `grid1` showing its `orders` table.
fn form(r: &Registry) -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").expect("the root");
        root.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        root.set_name("form_main");
        root.set_text("Orders");
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

    let mut grid = r
        .instantiate("Grid", "grid1", Rect::new(16.0, 48.0, 600.0, 400.0))
        .expect("Grid instantiates");
    grid.set_name("grid1");
    grid.props
        .insert("data_source".to_string(), Value::Ref("conn1".into()));
    grid.props
        .insert("table_name".to_string(), Value::Str("orders".into()));
    d.add_child("form_main", grid).expect("the grid goes in");
    d.validate().expect("valid");
    d
}

/// The same form through a table-mode `dst1` (Connection -> Dataset -> Grid),
/// with `edit1` bound to `dst1.customer`: the grid opens the dataset's table,
/// moves its cursor and reloads after a dataset operation.
fn dataset_form(r: &Registry) -> Document {
    let mut d = form(r);
    let mut dst = r
        .instantiate("Dataset", "dst1", Rect::new(40.0, 8.0, 28.0, 28.0))
        .expect("Dataset instantiates");
    dst.set_name("dst1");
    dst.props
        .insert("connection".to_string(), Value::Ref("conn1".into()));
    dst.props
        .insert("table".to_string(), Value::Str("orders".into()));
    d.add_child("form_main", dst).expect("the dataset goes in");
    d.node_mut("grid1")
        .expect("the grid")
        .props
        .insert("data_source".to_string(), Value::Ref("dst1".into()));
    let mut edit = r
        .instantiate("Edit", "edit1", Rect::new(16.0, 452.0, 200.0, 24.0))
        .expect("Edit instantiates");
    edit.set_name("edit1");
    edit.bindings.insert(
        "text".to_string(),
        crawrad_ir::binding::BindingDef::new("dst1", crawrad_ir::binding::BindingMode::TwoWay)
            .with_field("customer"),
    );
    d.add_child("form_main", edit).expect("the edit goes in");
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
        m.contains(toolkit) && m.contains("datagrid-sql") && m.contains("portadb-crawrad")
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
            "gate_bound_grid",
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

/// **The gate.** On both targets the bound grid generates and compiles clean.
#[test]
#[ignore = "builds a real project; run with --ignored"]
fn a_bound_grid_project_compiles_clean_on_both_targets() {
    let reg = catalog();
    let mut dirty = Vec::new();
    let forms = [("", form(&reg)), ("-dataset", dataset_form(&reg))];
    for (target, (variant, doc)) in forms
        .iter()
        .flat_map(|f| targets().into_iter().map(move |t| (t, f)))
    {
        let dest: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
            "../../target/gate-bound-grid-{}{variant}/app",
            target.name
        ));
        prepare(&dest, &target, doc);
        clean_src(&dest);

        let files = target
            .generator
            .gerar(doc, "gate_bound_grid", &reg)
            .expect("generate");
        crawrad_codegen::escrever(&dest, &files).expect("write the project");
        let designer = files
            .iter()
            .find(|a| a.caminho.to_string_lossy().contains("designer"))
            .map(|a| a.conteudo.clone())
            .unwrap_or_default();
        assert!(
            designer.contains("open_sqlite_table"),
            "[{}] the grid was not generated bound",
            target.name
        );

        let (ok, text) = cargo(&dest, &["clippy", "--", "-D", "warnings"]);
        println!(
            "{}{variant}: {}",
            target.name,
            if ok { "clean" } else { "DIRTY" }
        );
        if !ok {
            dirty.push(format!("=== {}{variant} ===\n{text}", target.name));
        }
    }
    assert!(
        dirty.is_empty(),
        "the bound grid project does not compile clean:\n\n{}",
        dirty.join("\n")
    );
}
