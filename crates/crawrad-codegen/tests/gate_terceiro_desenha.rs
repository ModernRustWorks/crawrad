//! **The THIRD-PARTY VISUAL gate**: components sourced from a `manifest.json`
//! appear in the generated application, and the project compiles cleanly with them.
//!
//! # The bug that originated this file, and it had two facets
//!
//! On 18/09/2026 `Gauge` already drew itself on the IDE canvas and turned into
//! `gauge_crawrad::Gauge` in egui target code. Yet the application opened with
//! the gauge **invisible**: the generator calculated its rectangle and discarded it.
//! The user's `cargo build` pointed this out loud and clear — *"unused variable: `r`"*
//! and *"struct `Widget` is never constructed"* —, two warnings they could not fix
//! because `.designer.rs` gets rewritten.
//!
//! On the **Makepad** target, which is the default, it was worse: the component did
//! not exist. `GeradorMakepad` filtered nodes against the list of classes it registered
//! itself, dropping third-party classes without a word — producing `pub struct FormMain {}`,
//! completely empty.
//!
//! # What this gate enforces, and why it compiles for real
//!
//! That declared drawing crosses over: in both targets, with the component crate
//! installed by path, as the IDE does (`cargo add --path`).
//!
//! Text tests would not suffice. What decides is `cargo` in the user's project:
//! `self.gau1.value as f32` against a nonexistent field, a `RoundedView` Makepad
//! does not know, a `fraction` nobody emitted — none of this shows in text, and all
//! of it shows up here.
//!
//! Marked `#[ignore]` for the same reason as the other two: calls `cargo` from
//! inside `cargo`.
//!
//! ```text
//! cargo test -p crawrad-codegen --test gate_terceiro_desenha -- --ignored --nocapture
//! ```
//!
//! # What it DOES NOT prove, noted so it is not forgotten
//!
//! The shader for the two strokes that Makepad has no widget to render —
//! `Tracejado` and `Marca`. `live_design!` retains the text and compiles the shader
//! **at run time**, so errors in it pass silently through this gate. Verifying them
//! requires looking at the screen.

mod comum;

use comum::manifesto_intocado;
use crawrad_codegen::Gerador as _;
use crawrad_ir::{Document, Rect, Value};
use crawrad_package::manifest::Manifest;
use crawrad_registry::Registry;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The catalog with `Gauge` parsed from the **real** `manifest.json`.
///
/// From the file, not from an in-memory descriptor built here: what this gate
/// measures is the path an installed component travels, and a mock catalog
/// would skip the very front door.
fn catalogo() -> Registry {
    let mut r = Registry::builtin();
    // **BOTH components in the suite**, not just the visual one. `Connection`
    // is non-visual and generates code, and until 18/09/2026 it generated code
    // that DID NOT COMPILE: the package was manifest-only, without any crate.
    // Present here so this regression does not return silently.
    for texto in [
        include_str!("../../../components/gauge-crawrad/manifest.json"),
        include_str!("../../../components/portadb-crawrad/manifest.json"),
    ] {
        let manifesto: Manifest =
            serde_json::from_str(texto).expect("the `manifest.json` is valid JSON");
        for c in manifesto
            .to_components()
            .expect("the package manifest passes conversion")
        {
            r.register_external(c)
                .expect("and passes the seven refusals");
        }
    }
    r
}

/// Three gauges with different values, and a `Button` alongside.
///
/// The three values are not decoration: each bar's length is calculated from
/// `value`, and a single value would not distinguish *"the bar follows the value"*
/// from *"the bar always draws the same thing"*.
///
/// `Button` is there because real forms mix native and third-party components,
/// and bugs have previously occurred right at their seam.
fn caso(reg: &Registry) -> Document {
    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("the root");
        raiz.set_rect(Rect::new(0.0, 0.0, 420.0, 260.0));
        raiz.set_name("form_main");
        raiz.set_text("Third party");
    }
    for (i, (nome, valor)) in [("gau_a", 20), ("gau_b", 65), ("gau_c", 95)]
        .into_iter()
        .enumerate()
    {
        let y = 8.0 + i as f32 * 56.0;
        let mut n = reg
            .instantiate("Gauge", nome.to_string(), Rect::new(8.0, y, 180.0, 36.0))
            .expect("the catalogue instantiates what it registered");
        n.set_name(nome);
        n.props
            .insert("text".to_string(), Value::Str(format!("Disk {nome}")));
        n.props.insert("value".to_string(), Value::Int(valor));
        // **THE FIRST HAS `OnClick` WIRED.** A third-party component that
        // notifies is what separates a picture from a component, and the handler
        // must exist in the user file for the project to compile — the flip side
        // of the old bug, where the handler existed and nobody called it.
        if nome == "gau_a" {
            n.events
                .insert("on_click".to_string(), "gau_a_click".to_string());
        }
        d.add_child("form_main", n).expect("into the form");
    }
    let mut b = reg
        .instantiate(
            "Button",
            "btn_ok".to_string(),
            Rect::new(220.0, 8.0, 96.0, 32.0),
        )
        .expect("instantiates");
    b.set_name("btn_ok");
    d.add_child("form_main", b).expect("enters");
    // **THE NON-VISUAL, in the same form.** It draws nothing and becomes a
    // typed field of its crate — the other half of what a third-party package
    // can be.
    let mut e = reg
        .instantiate(
            "Connection",
            "eng1".to_string(),
            Rect::new(220.0, 60.0, 28.0, 28.0),
        )
        .expect("instantiates");
    e.set_name("eng1");
    e.props
        .insert("url".to_string(), Value::Str("data.db".into()));
    d.add_child("form_main", e).expect("enters");
    d.validate().expect("valid");
    d
}

/// A target for this gate: name, generator, and what the project requires.
struct Alvo {
    nome: &'static str,
    gerador: Box<dyn crawrad_codegen::Gerador>,
    toolkit: &'static str,
}

fn alvos() -> Vec<Alvo> {
    vec![
        Alvo {
            nome: "egui",
            gerador: Box::new(crawrad_codegen::GeradorRxp),
            toolkit: "eframe@0.36",
        },
        Alvo {
            nome: "makepad",
            gerador: Box::new(crawrad_codegen::GeradorMakepad),
            toolkit: "makepad-widgets@1.0",
        },
    ]
}

/// A `cargo` invocation inside the generated project, without outer cargo variables.
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

const RAIZ_DO_GRUPO: &str = r#"[workspace]
members = ["app"]
resolver = "2"
default-members = ["app"]
"#;

/// ABSOLUTE paths to the suite crates, for `cargo add --path`.
fn caminhos_dos_componentes() -> Vec<PathBuf> {
    ["gauge-crawrad", "portadb-crawrad"]
        .iter()
        .map(|n| {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../components")
                .join(n)
                .canonicalize()
                .unwrap_or_else(|e| panic!("`components/{n}` is in the repository: {e}"))
        })
        .collect()
}

fn rascunho_intacto(grupo: &Path, destino: &Path, toolkit: &str) -> bool {
    if !manifesto_intocado(grupo, RAIZ_DO_GRUPO) || !destino.join("src").is_dir() {
        return false;
    }
    let pacote = toolkit.split('@').next().unwrap_or(toolkit);
    std::fs::read_to_string(destino.join("Cargo.toml")).is_ok_and(|m| {
        m.contains(pacote)
            && m.contains("gauge-crawrad")
            && m.contains("portadb-crawrad")
            && (!toolkit.contains("makepad")
                || (m.contains("crawrad-runtime") && m.contains("crawrad-runtime-makepad")))
    })
}

/// The `cargo new` + two `cargo add` calls the IDE runs before generating.
///
/// The second call settles ADR 0033: component dependencies are added
/// **by path**, rather than by published version. The IDE designed the
/// form using this `manifest.json` descriptor; depending on another version
/// would compile the application against different code than what the designer
/// displayed.
fn preparar(destino: &Path, toolkit: &str) {
    let grupo = destino.parent().expect("destination is member of a group");
    if rascunho_intacto(grupo, destino, toolkit) {
        return;
    }
    if destino.exists() {
        std::fs::remove_dir_all(destino).expect("delete half-finished draft");
    }
    std::fs::create_dir_all(grupo).expect("create group root");
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
            "gate_terceiro",
            nome,
        ],
    );
    assert!(ok, "`cargo new` failed:\n{texto}");

    let alvo = if toolkit.contains("makepad") {
        crawrad_codegen::AlvoGerador::Makepad
    } else {
        crawrad_codegen::AlvoGerador::Egui
    };

    for dep in crawrad_codegen::dependencias_do_projeto(alvo, true, false) {
        let args = dep.args_cargo_add();
        let mut cmd = vec!["add"];
        let refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        cmd.extend(refs);
        let (ok, texto) = cargo(destino, &cmd);
        assert!(
            ok,
            "`cargo add` of unified dependency `{}` failed:\n{texto}",
            dep.nome()
        );
    }

    for caminho in caminhos_dos_componentes() {
        let (ok, texto) = cargo(destino, &["add", "--path", caminho.to_str().expect("path")]);
        assert!(ok, "component `cargo add --path` failed:\n{texto}");
    }
}

fn limpar_src(destino: &Path) {
    let src = destino.join("src");
    for e in std::fs::read_dir(&src).expect("read src") {
        let p = e.expect("an entry").path();
        if p.extension().is_some_and(|x| x == "rs") {
            std::fs::remove_file(&p).expect("delete old file");
        }
    }
}

/// **The gate.** In both targets, the third-party visual component crosses over and compiles clean.
#[test]
#[ignore = "compiles a real project; run with --ignored"]
fn o_terceiro_visual_atravessa_e_o_projeto_compila_limpo() {
    let reg = catalogo();
    let doc = caso(&reg);

    for alvo in alvos() {
        let destino: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../target/gate-terceiro-{}/app", alvo.nome));
        preparar(&destino, alvo.toolkit);
        limpar_src(&destino);

        let arquivos = alvo
            .gerador
            .gerar(&doc, "gate_terceiro", &reg)
            .expect("generating");
        let mut designer = String::new();
        for a in &arquivos {
            let caminho = destino.join(&a.caminho);
            if let Some(pai) = caminho.parent() {
                std::fs::create_dir_all(pai).expect("the file folder");
            }
            std::fs::write(&caminho, &a.conteudo).expect("write generated file");
            if a.caminho.to_string_lossy().contains("designer") {
                designer = a.conteudo.clone();
            }
        }

        // **THE THIRD-PARTY TYPE, spelled out.** Without this the project even
        // compiles — it simply lacks the component —, which was Makepad's status
        // until now.
        assert!(
            designer.contains("gauge_crawrad::Gauge"),
            "[{}] third-party type did not appear in the designer",
            alvo.nome
        );
        assert!(
            designer.contains("portadb_crawrad::Connection"),
            "[{}] third-party non-visual did not appear in the designer",
            alvo.nome
        );
        // **DRAWING.** The fraction is the part only declared drawing possesses,
        // and that was missing on both sides.
        assert!(
            designer.contains("fraction(")
                && designer.contains("gau_a")
                && designer.contains("gau_c"),
            "[{}] declared drawing did not cross over: no `fraction` in the designer",
            alvo.nome
        );

        // **THE EVENT IS WIRED.** Without this assertion the gate would pass
        // green on a generator that emits drawing and discards `OnClick` — the
        // state of every third-party component until 18/09/2026.
        assert!(
            designer.contains("gau_a_click"),
            "[{}] third-party `OnClick` did not turn into any call",
            alvo.nome
        );

        let (ok, texto) = cargo(&destino, &["clippy", "--", "-D", "warnings"]);
        assert!(
            ok,
            "[{}] project with a third-party component inside does not compile \
             cleanly:\n{texto}",
            alvo.nome
        );
    }
}
