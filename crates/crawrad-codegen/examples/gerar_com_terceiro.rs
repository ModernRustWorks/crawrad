//! Generates a project with a THIRD-PARTY component inside, to prove that it
//! compiles.
//!
//! This is not a unit test: what is proven here is the `cargo build` of the
//! generated project, and that is an external process.

use crawrad_codegen::Gerador;
use crawrad_ir::{Document, Rect, Value};
use crawrad_package::manifest::Manifest;
use crawrad_registry::Registry;

fn main() {
    let destino = std::env::args()
        .nth(1)
        .expect("usage: gerar_com_terceiro <folder>");

    let texto = include_str!("../../../components/gauge-crawrad/manifest.json");
    let manifesto: Manifest = serde_json::from_str(texto).expect("valid JSON");
    let mut r = Registry::builtin();
    for c in manifesto.to_components().expect("conversion") {
        r.register_external(c).expect("port accepts");
    }

    let mut d = Document::new("form_main", "Form");
    {
        let raiz = d.node_mut("form_main").expect("root");
        raiz.set_rect(Rect::new(0.0, 0.0, 480.0, 240.0));
        raiz.set_name("form_main");
        raiz.set_text("Prova");
    }
    let mut n = r
        .instantiate(
            "Gauge",
            "gau1".to_string(),
            Rect::new(20.0, 20.0, 200.0, 40.0),
        )
        .expect("instantiate");
    n.set_name("gau1");
    n.props
        .insert("text".to_string(), Value::Str("Disco".into()));
    n.props.insert("value".to_string(), Value::Int(95));
    d.add_child("form_main", n).expect("add child");

    let arquivos = crawrad_codegen::GeradorRxp
        .gerar(&d, "app", &r)
        .expect("generating");
    crawrad_codegen::escrever(std::path::Path::new(&destino), &arquivos).expect("writing");

    for c in crawrad_codegen::classes_de_terceiro(&d, &r) {
        println!("PRECISA: {c}");
    }
}
