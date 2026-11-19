//! **Reading a whole `.rs` file**: what `crawrad manifest` does.
//!
//! The macro receives ONE `struct`, already chosen by the compiler. The CLI
//! receives the text of a file and has to find which `struct`s are components.
//! This difference is what the tests here cover.

use crate::from_source;

const HEADER: &str = r#"#[crawrad(palette = "P", prefix = "p", size = (10.0, 10.0))]"#;

fn component(name: &str, derive: &str) -> String {
    format!(
        "#[derive({derive})]\n{HEADER}\npub struct {name} {{\n    \
         #[prop(category = \"behavior\", default = 1)]\n    pub v: i64,\n}}\n"
    )
}

/// **The `struct` with `#[derive(Component)]` is found; the one without is not.**
#[test]
fn only_what_asks_for_the_derive_becomes_a_component() {
    let source = format!(
        "{}\n{}\npub struct Loose {{ pub v: i64 }}\n",
        component("Meter", "Component, Debug"),
        component("Other", "Debug, Clone")
    );
    let found = from_source(&source).expect("the file must read");
    let classes: Vec<&str> = found.iter().map(|c| c.class.as_str()).collect();
    assert_eq!(classes, vec!["Meter"], "found: {classes:?}");
}

/// **The order is the file's**, and it decides the order in the palette.
#[test]
fn the_order_is_the_files() {
    let source = format!(
        "{}\n{}\n{}",
        component("One", "Component"),
        component("Two", "Component"),
        component("Three", "Component")
    );
    let found = from_source(&source).expect("reads");
    let classes: Vec<&str> = found.iter().map(|c| c.class.as_str()).collect();
    assert_eq!(classes, vec!["One", "Two", "Three"]);
}

/// **A component inside a `mod` is found too.**
///
/// Stopping at the root would make the CLI and the macro disagree: the macro
/// does not stop, because the compiler calls it wherever the `struct` is. And
/// the disagreement would show up as a component that exists in `cargo build`
/// and vanishes from `manifest.json`.
#[test]
fn a_component_inside_a_mod_is_found() {
    let source = format!(
        "pub mod meters {{\n    pub mod backgrounds {{\n{}\n    }}\n}}\n",
        component("Hidden", "Component")
    );
    let found = from_source(&source).expect("reads");
    assert_eq!(found.len(), 1, "found {} components", found.len());
    assert_eq!(found[0].class, "Hidden");
}

/// **The full path counts**, because that is how `rustc` resolves it.
///
/// A CLI that only recognised a bare `Component` would answer differently from
/// the compiler, and a second truth about what a component is, is the start of
/// a file that does not match the build.
#[test]
fn a_qualified_derive_counts_too() {
    let source = component("Meter", "crawrad_component_derive::Component");
    let found = from_source(&source).expect("reads");
    assert_eq!(found.len(), 1);
}

/// **A file with no component at all is not an error.**
///
/// A package spreads its components over several files, and most of them have
/// none. Complaining here would force the CLI to know which files "should"
/// have one, which is knowledge it does not have and must not invent.
#[test]
fn a_file_without_a_component_is_not_an_error() {
    let found = from_source("pub fn sum(a: i64, b: i64) -> i64 { a + b }\n").expect("reads");
    assert!(found.is_empty());
}

/// **Rust that does not compile is refused with the parser's complaint.**
#[test]
fn broken_source_returns_the_complaint() {
    let Err(e) = from_source("pub struct {{{") else {
        panic!("broken Rust must be refused");
    };
    assert!(!e.to_string().is_empty());
}

/// **And what comes out of here is the same object the macro writes.**
///
/// Both ends of ADR 0039 meet in this test: the CLI reads text, the macro
/// receives the compiler's tree, and both go through the same
/// `ComponentSpec::to_json`. If there are ever two writing paths, this is where
/// it shows.
#[test]
fn the_object_is_the_same_the_macro_writes() {
    let found = from_source(&component("Meter", "Component")).expect("reads");
    let written = found[0].to_json(0);
    let v: serde_json::Value = serde_json::from_str(&written).expect("is JSON");
    assert_eq!(v["class"], "Meter");
    assert_eq!(v["palette"], "P");
    // `name` and `rect` are still born on their own, as in the macro.
    let props = v["props"].as_array().expect("props");
    assert_eq!(props[0]["key"], "name");
    assert_eq!(props[1]["key"], "rect");
}

/// **The `Paint` implementation is read from the AST and produces the `draw` section.**
#[test]
fn the_canvas_paint_becomes_a_draw_spec_in_the_manifest() {
    let source = r#"
        #[derive(Component)]
        #[crawrad(palette = "Standard", prefix = "gauge", size = (140.0, 48.0))]
        pub struct Gauge {
            #[prop(category = "data", default = 0)]
            pub value: i64,
        }

        impl Paint for Gauge {
            fn paint(&self, canvas: &mut Canvas) {
                canvas.set_fill_style(ThemeRole::Surface);
                canvas.set_stroke_style(ThemeRole::Border);
                canvas.fill_round_rect(0.0, 0.0, 140.0, 48.0, 4.0);

                canvas.set_fill_style(ThemeRole::Accent);
                canvas.fill_fraction_rect(Axis::X, "value", "min", "max");

                canvas.set_stroke_style(ThemeRole::Accent);
                canvas.set_line_width(2.0);
                canvas.stroke_round_rect(0.0, 0.0, 140.0, 48.0, 4.0);

                canvas.set_fill_style(ThemeRole::Text);
                canvas.fill_text_prop("text", 0.0, 0.0, 140.0, 48.0, TextAlign::Center);
            }
        }
    "#;

    let found = from_source(source).expect("reads the source with Paint");
    assert_eq!(found.len(), 1);
    let spec = &found[0];
    assert_eq!(spec.class, "Gauge");
    assert_eq!(spec.draws.len(), 4);

    let json_text = spec.to_json(0);
    let v: serde_json::Value = serde_json::from_str(&json_text).expect("valid JSON");
    let draw_array = v["draw"].as_array().expect("draw array");
    assert_eq!(draw_array.len(), 4);
    assert_eq!(draw_array[0]["verb"], "box");
    assert_eq!(draw_array[0]["fill"], "surface");
    assert_eq!(draw_array[0]["border"], "border");
    assert_eq!(draw_array[0]["radius"], 4.0);

    assert_eq!(draw_array[1]["verb"], "box");
    assert_eq!(draw_array[1]["fill"], "accent");
    assert_eq!(draw_array[1]["area"]["fraction_of"], "value");
    assert_eq!(draw_array[1]["area"]["axis"], "x");

    assert_eq!(draw_array[2]["verb"], "frame");
    assert_eq!(draw_array[2]["color"], "accent");
    assert_eq!(draw_array[2]["thickness"], 2.0);

    assert_eq!(draw_array[3]["verb"], "text");
    assert_eq!(draw_array[3]["prop"], "text");
    assert_eq!(draw_array[3]["color"], "text");
    assert_eq!(draw_array[3]["centered"], true);
}

/// Every canonical source shipped in `source/` must be syntactically valid and
/// emit valid manifests through `from_source`.
#[test]
fn every_reference_source_in_source_is_valid() {
    let files = [
        "source/standard/button.rs",
        "source/standard/label.rs",
        "source/standard/panel.rs",
        "source/standard/edit.rs",
        "source/standard/check_box.rs",
        "source/standard/radio_button.rs",
        "source/standard/group_box.rs",
        "source/standard/memo.rs",
        "source/standard/combo_box.rs",
        "source/standard/list_box.rs",
        "source/standard/page_control.rs",
        "source/system/timer.rs",
    ];
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    for rel in files {
        let path = root.join(rel);
        assert!(path.exists(), "file {rel} must exist");
        let content = std::fs::read_to_string(&path).expect("read");
        let specs = from_source(&content).unwrap_or_else(|e| panic!("error reading {rel}: {e}"));
        assert!(
            !specs.is_empty(),
            "file {rel} must contain at least 1 component"
        );
        for spec in &specs {
            assert!(!spec.class.is_empty(), "class must not be empty in {rel}");
            assert!(
                !spec.palette.is_empty(),
                "palette must not be empty in {rel}"
            );
            let json = spec.to_json(2);
            let _: serde_json::Value = serde_json::from_str(&json)
                .unwrap_or_else(|e| panic!("invalid JSON for {} in {rel}: {e}", spec.class));
        }
    }
}
