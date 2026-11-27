//! Generates a CrawRAD project's Rust files from its `.crawform`, outside the IDE.
//!
//! It does what the Run button does before it calls cargo: read the form, run the
//! project's target generator, and write `src/`. It exists to prove the generator
//! without clicking in the IDE — the proof the owner asked for: that the running
//! window is the size of the form on the canvas.
//!
//! Usage: `cargo run -p crawrad-project --example generate_project -- <member-folder>`

use std::path::PathBuf;

fn main() {
    let root = match std::env::args().nth(1) {
        Some(p) => PathBuf::from(p),
        None => {
            eprintln!("usage: generate_project <member-folder>");
            std::process::exit(2);
        }
    };

    let forms: Vec<PathBuf> = std::fs::read_dir(root.join("src"))
        .expect("read src/")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "crawform"))
        .collect();
    assert!(!forms.is_empty(), "no .crawform in {}/src", root.display());

    let text = std::fs::read_to_string(&forms[0]).expect("read the .crawform");
    let doc = crawrad_ir::Document::from_json(&text).expect("read the form");

    let package = crawrad_project::generator::package_name_in(&root);
    if let Some(t) = crawrad_project::generator::project_target(&root) {
        crawrad_project::generator::set_active_target(t);
    }
    let generator = crawrad_project::generator::project_generator();
    // The IDE's catalog, not only the built-ins: Connection, Dataset and Grid come
    // from the installed components, and without them the generator does not know
    // the form's classes.
    let registry = crawrad_project::installed::ide_catalog().catalog.clone();

    let files = generator
        .gerar(&doc, &package, &registry)
        .expect("generate the files");

    let report = crawrad_codegen::escrever(&root, &files).expect("write the files");

    println!("project: {}", root.display());
    println!("target:  {}", generator.alvo());
    println!("package: {package}");
    println!("{report:?}");
}
