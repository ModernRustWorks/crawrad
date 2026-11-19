//! Diagnostic: generates the project from a .crawform and reports where it was written.
use crawrad_codegen::Gerador;
fn main() {
    let doc = std::env::args()
        .nth(1)
        // The example lives inside the MEMBER now: the open folder is the group, and
        // the `.crawform` sits next to the `.rs` it generates (ADR 0017, Amendment 1).
        .unwrap_or_else(|| "examples/calculator/calculator/src/form_main.crawform".into());
    let saida = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "target/gerado".into());
    let mut d = crawrad_ir::Document::load(&doc).expect("open form");
    let destino = std::path::Path::new(&saida);

    // The PRE-PASS, and it comes before the plan — in the IDE it is the same order, inside
    // `gerar_projeto`. Empty event handlers are removed on save, whether bound or
    // unbound; and the binding must be removed BEFORE the `.designer.rs` is generated,
    // otherwise it emits a call to a method that pruning will remove (`E0599`).
    //
    // Here unbinding is done manually in the in-memory document without touching
    // the input `.crawform`: in the IDE, this is handled by a `Transaction` from
    // `crawrad-doc`, which is the only write port. The example has no open
    // document or undo stack — it is a CLI diagnostic, which
    // is why this order copy exists (ADR 0025, Amendment 1).
    let vencidos = crawrad_codegen::vinculos_vencidos(destino, &d);
    for x in &vencidos {
        for (no, evento) in &x.vinculos {
            if let Some(n) = d.node_mut(no) {
                n.events.remove(evento);
            }
        }
    }
    // In Portuguese like the other lines here: this is not the status bar,
    // it is the output of an example. The sentence the user reads in the IDE is
    // `frase_da_poda`, which the shell appends to `frase_da_geracao`.
    if !vencidos.is_empty() {
        let nomes: Vec<&str> = vencidos.iter().map(|x| x.handler.as_str()).collect();
        println!("unbound:     {nomes:?}");
    }

    let arqs = crawrad_codegen::GeradorRxp
        .gerar(&d, "meu_app", &crawrad_registry::Registry::builtin())
        .expect("generate");
    let r = crawrad_codegen::escrever(destino, &arqs).expect("write");
    println!("created:     {:?}", r.criados);
    println!("rewritten:   {:?}", r.reescritos);
    println!("preserved:   {:?}", r.preservados);
    println!("grew:        {:?}", r.cresceram);
    // What SHRANK. This line is diagnostic, and is the only clue outside the IDE
    // that pruning ran — without it, the handler disappeared and the example did not report it.
    println!("pruned:      {:?}", r.podados);

    // The generator writes only `src/*.rs` since ADR 0018 — without `cargo init` and
    // `cargo add` this is not a project, and anyone running the example expecting
    // `cargo build` deserves to know immediately, rather than from a Cargo error.
    if !destino.join("Cargo.toml").exists() {
        println!(
            "\nproject missing: Cargo.toml belongs to cargo, not us.\n\
             \n  cd {saida}\n  cargo new meu_app --bin\n  \
             cargo add -p meu_app eframe@0.36\n\
             \n`cargo new` in a SUBFOLDER, not `cargo init` here: the open\n\
             folder is the group, and projects enter as members (ADR 0017,\n\
             Amendment 1). The member registers itself in `members`.\n\
             \nonly `eframe`: the generated designer uses `eframe::egui`, and requesting\n\
             a loose `egui` invites the version to diverge from what `eframe` brings."
        );
    }
}
