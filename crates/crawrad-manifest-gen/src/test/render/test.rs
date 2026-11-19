//! **What the macro writes** — a struct in, a component object out.
//!
//! These read a struct from source text and render it, which is exactly what
//! the macro does at expansion time. Nothing is mocked: the same
//! [`ComponentSpec::read`] and the same [`ComponentSpec::to_json`] run here as
//! in a real build.

use crate::spec::ComponentSpec;
use syn::DeriveInput;

/// Render a struct written as source, the way the macro would.
fn render(source: &str) -> String {
    let input: DeriveInput = syn::parse_str(source).expect("the test struct has to parse");
    ComponentSpec::read(&input)
        .expect("the test struct has to be a valid component")
        .to_json(0)
}

/// The `Gauge` as it really is, so the test moves when the component moves.
fn gauge() -> String {
    render(
        r##"
        #[crawrad(palette = "Additional", prefix = "gau", size = (140.0, 48.0))]
        #[event(name = "on_click")]
        #[draw(
            r#box(fill = "surface", border = "border", radius = 4.0),
            r#box(fill = "accent", area(fraction_of = "value", axis = "x", min = "min", max = "max")),
            text(prop = "text", color = "text", centered),
        )]
        pub struct Gauge {
            /// The caption.
            #[prop(display = "Caption", category = "appearance", default = "Gauge")]
            pub text: String,
            #[prop(display = "Value", category = "behavior", default = 65, min = 0, max = 100)]
            pub value: i64,
            #[prop(display = "Min", category = "behavior", default = 0, min = -1000000, max = 1000000)]
            pub min: i64,
        }
        "##,
    )
}

fn parsed_gauge() -> serde_json::Value {
    let rendered = gauge();
    serde_json::from_str(&rendered).unwrap_or_else(|e| panic!("{e}\n{rendered}"))
}

/// **What the macro writes is JSON a parser accepts**, and not JSON by eye.
///
/// Every other test here reads the parsed document. This one is the only guard
/// that there IS a document — without it a stray comma would be caught far
/// away, on the machine of whoever installed the package.
#[test]
fn what_the_macro_writes_parses_as_json() {
    let rendered = gauge();
    let parsed: serde_json::Value =
        serde_json::from_str(&rendered).unwrap_or_else(|e| panic!("{e}\n{rendered}"));
    assert!(parsed.get("class").is_some(), "{rendered}");
}

/// The class is the struct's name, and nobody writes it twice.
#[test]
fn the_class_is_the_struct_name() {
    let v = parsed_gauge();
    assert_eq!(v["class"], "Gauge");
    assert_eq!(v["display"], "Gauge");
}

/// `name` and `rect` arrive without being asked for, and `rect` carries `size`.
///
/// Every hand-written manifest in the tree declares these twelve lines
/// identically. They are what the IDE needs to place an instance, not what the
/// component is about, and forgetting them is a mistake nothing would catch.
#[test]
fn name_and_rect_come_for_free() {
    let v = parsed_gauge();
    let props = v["props"].as_array().expect("props is an array");
    // They come FIRST, because that is the order the Object Inspector shows.
    assert_eq!(props[0]["key"], "name");
    assert_eq!(props[1]["key"], "rect");
    assert_eq!(props[1]["default"]["w"], 140.0);
    assert_eq!(props[1]["default"]["h"], 48.0);
}

/// The kind comes from the Rust type when the author does not spell it out.
#[test]
fn the_kind_follows_the_rust_type() {
    let rendered = render(
        r#"
        #[crawrad(palette = "P", prefix = "p", size = (10.0, 10.0))]
        struct Sample {
            #[prop(category = "appearance", default = "")]
            pub a: String,
            #[prop(category = "behavior", default = true)]
            pub b: bool,
            #[prop(category = "behavior", default = 1)]
            pub c: i64,
            #[prop(category = "behavior", default = 1.5)]
            pub d: f32,
        }
        "#,
    );
    let v: serde_json::Value = serde_json::from_str(&rendered).expect("JSON");
    let props = v["props"].as_array().expect("props");
    for (key, kind) in [("a", "text"), ("b", "bool"), ("c", "int"), ("d", "float")] {
        let found = props
            .iter()
            .find(|p| p["key"] == key)
            .unwrap_or_else(|| panic!("no property `{key}`\n{rendered}"));
        assert_eq!(found["kind"], kind, "`{key}`\n{rendered}");
    }
}

/// A field with no `#[prop]`, and one marked `skip`, stay out of the manifest.
///
/// A component is allowed private state. Publishing every field would put the
/// component's internals in the Object Inspector, and the author would have to
/// remember to hide them one by one.
#[test]
fn a_field_without_prop_is_not_published() {
    let rendered = render(
        r#"
        #[crawrad(palette = "P", prefix = "p", size = (10.0, 10.0))]
        struct Sample {
            #[prop(category = "behavior", default = 1)]
            pub shown: i64,
            pub quiet: i64,
            #[prop(skip)]
            pub hidden: i64,
        }
        "#,
    );
    assert!(rendered.contains("\"shown\""), "{rendered}");
    assert!(!rendered.contains("\"quiet\""), "{rendered}");
    assert!(!rendered.contains("\"hidden\""), "{rendered}");
    // And because a field stayed out, this type has state the manifest does not
    // describe -- which the generated application has to start from `Default`.
    assert!(
        !rendered.contains("private_state"),
        "a component with a hidden field must not claim to cover everything\n{rendered}"
    );
}

/// When every field is published, the manifest says so.
///
/// It is what lets the generated application build the component with a
/// complete struct literal — the only shape that does not trip
/// `clippy::needless_update` in the user's own project.
#[test]
fn a_component_that_publishes_everything_says_so() {
    let v = parsed_gauge();
    assert_eq!(v["private_state"], false);
}

/// The drawing is a list, in the order the author wrote it.
///
/// The order is the meaning: the bar is painted over the box, and the same verb
/// appearing twice is how that happens — not a copy-paste mistake.
#[test]
fn the_drawing_keeps_the_order_and_the_verbs() {
    let v = parsed_gauge();
    let draw = v["draw"].as_array().expect("draw is an array");
    let verbs: Vec<&str> = draw
        .iter()
        .map(|d| d["verb"].as_str().expect("every step names its verb"))
        .collect();
    assert_eq!(verbs, vec!["box", "box", "text"]);
}

/// The `area` of a verb is an object, and it follows the properties.
///
/// A number there instead of a property name is the defect this project already
/// paid for once: the painted bar stopped following `min` at run time while
/// `fraction()` kept following it, and the two agreed only while the values
/// were the defaults.
#[test]
fn the_area_is_an_object_that_follows_the_properties() {
    let v = parsed_gauge();
    let draw = v["draw"].as_array().expect("draw");
    let bar = draw
        .iter()
        .find(|d| d.get("area").is_some())
        .expect("one verb has an area");
    assert_eq!(bar["area"]["fraction_of"], "value");
    assert_eq!(bar["area"]["min"], "min");
    assert_eq!(bar["area"]["max"], "max");
}

/// The doc comment of a field survives, as DATA.
///
/// JSON has no comments, and this is the better answer anyway: a `#` line is
/// dead weight only a reader of the file ever sees, while a `doc` field is
/// something the Object Inspector can show as a hint.
#[test]
fn the_doc_comment_becomes_a_field() {
    let v = parsed_gauge();
    let props = v["props"].as_array().expect("props");
    let text = props.iter().find(|p| p["key"] == "text").expect("text");
    assert_eq!(text["doc"], "The caption.");
}

/// What matches the default is not written down.
#[test]
fn only_what_differs_from_the_default_is_written() {
    let rendered = gauge();
    assert!(!rendered.contains("\"visual\""), "{rendered}");
    assert!(!rendered.contains("\"children\""), "{rendered}");
    assert!(!rendered.contains("\"derived_geometry\""), "{rendered}");
}

/// A non-visual component says so, and a universal one lists its targets.
#[test]
fn a_non_visual_component_says_so() {
    let rendered = render(
        r#"
        #[crawrad(palette = "Data", prefix = "de", size = (32.0, 32.0), visual = false, targets = ["*"])]
        struct Engine {
            #[prop(category = "behavior", default = "")]
            pub url: String,
        }
        "#,
    );
    let v: serde_json::Value = serde_json::from_str(&rendered).expect("JSON");
    assert_eq!(v["visual"], false);
    assert_eq!(v["targets"][0], "*");
}

/// `on_click` shows up as `OnClick`, the way the Object Inspector writes it.
#[test]
fn an_event_gets_the_classic_display_name() {
    let v = parsed_gauge();
    let event = &v["events"][0];
    assert_eq!(event["name"], "on_click");
    assert_eq!(event["display"], "OnClick");
    assert_eq!(event["signature"], "fn(&mut self)");
}

#[test]
fn the_item_class_is_rendered_and_inferred_for_collection() {
    let source = r#"
        #[crawrad(palette = "Data", prefix = "grd", size = (200.0, 100.0))]
        pub struct Grid {
            #[prop(display = "Columns", category = "appearance")]
            pub columns: Collection<GridColumn>,
        }
    "#;
    let json_text = render(source);
    let v: serde_json::Value = serde_json::from_str(&json_text).unwrap();
    let props = v["props"].as_array().unwrap();
    let col_prop = props.iter().find(|p| p["key"] == "columns").unwrap();
    assert_eq!(col_prop["kind"], "collection");
    assert_eq!(col_prop["item_class"], "GridColumn");
    assert_eq!(col_prop["default"], serde_json::json!([]));
}

#[test]
fn an_item_spec_renders_its_properties_and_class() {
    let source = r#"
        pub struct GridColumn {
            /// The title of the column.
            #[prop(display = "Title", category = "appearance", default = "Column")]
            pub title: String,

            #[prop(display = "Width", category = "layout", default = 100.0)]
            pub width: f64,
        }
    "#;
    let input: syn::DeriveInput = syn::parse_str(source).unwrap();
    let spec = crate::spec::ItemSpec::read(&input).unwrap();
    let json_text = spec.to_json(0);
    let v: serde_json::Value = serde_json::from_str(&json_text).unwrap();
    assert_eq!(v["class"], "GridColumn");
    let props = v["props"].as_array().unwrap();
    assert_eq!(props.len(), 2);
    assert_eq!(props[0]["key"], "title");
    assert_eq!(props[0]["default"], "Column");
    assert_eq!(props[1]["key"], "width");
    assert_eq!(props[1]["default"], 100.0);
}

#[test]
fn a_struct_with_icon_renders_expected_json() {
    let source = r#"
        #[crawrad(palette = "Icons", prefix = "ico", size = (32.0, 32.0))]
        #[icon(
            move_to(2.0, 3.0),
            line_to(14.0, 15.0),
            close_path,
            circle(8.0, 8.0, 4.0),
            r#box(1.0, 2.0, 14.0, 12.0, 1.5),
            rect(2.0, 3.0, 10.0, 8.0),
            fill,
            stroke,
        )]
        pub struct IconWidget {
            #[prop(category = "appearance", default = "")]
            pub title: String,
        }
    "#;
    let json_text = render(source);
    let v: serde_json::Value = serde_json::from_str(&json_text).unwrap();
    let icons = v["icon"].as_array().expect("icon array is present");
    assert_eq!(icons.len(), 8);
    assert_eq!(icons[0]["step"], "move_to");
    assert_eq!(icons[0]["x"], 2.0);
    assert_eq!(icons[0]["y"], 3.0);
    assert_eq!(icons[1]["step"], "line_to");
    assert_eq!(icons[1]["x"], 14.0);
    assert_eq!(icons[1]["y"], 15.0);
    assert_eq!(icons[2]["step"], "close_path");
    assert_eq!(icons[3]["step"], "circle");
    assert_eq!(icons[3]["x"], 8.0);
    assert_eq!(icons[3]["y"], 8.0);
    assert_eq!(icons[3]["r"], 4.0);
    assert_eq!(icons[4]["step"], "box");
    assert_eq!(icons[4]["x"], 1.0);
    assert_eq!(icons[4]["y"], 2.0);
    assert_eq!(icons[4]["w"], 14.0);
    assert_eq!(icons[4]["h"], 12.0);
    assert_eq!(icons[4]["r"], 1.5);
    assert_eq!(icons[5]["step"], "rect");
    assert_eq!(icons[5]["x"], 2.0);
    assert_eq!(icons[5]["y"], 3.0);
    assert_eq!(icons[5]["w"], 10.0);
    assert_eq!(icons[5]["h"], 8.0);
    assert_eq!(icons[6]["step"], "fill");
    assert_eq!(icons[7]["step"], "stroke");
}

/// A step with no numbers is also written with empty parentheses — the form that
/// lets an icon repeat its paints under Clippy's `duplicated_attributes`.
#[test]
fn steps_with_empty_parentheses_render_like_bare_ones() {
    let source = r#"
        #[crawrad(palette = "Icons", prefix = "ico", size = (32.0, 32.0))]
        #[icon(
            circle(8.0, 9.0, 6.0),
            stroke(),
            move_to(8.0, 9.0),
            line_to(11.5, 5.5),
            close_path(),
            stroke(),
            circle(8.0, 9.0, 1.2),
            fill()
        )]
        pub struct Dial {
            #[prop(category = "appearance", default = "")]
            pub title: String,
        }
    "#;
    let v: serde_json::Value = serde_json::from_str(&render(source)).unwrap();
    let steps: Vec<&str> = v["icon"]
        .as_array()
        .expect("icon array is present")
        .iter()
        .map(|s| s["step"].as_str().unwrap_or_default())
        .collect();
    assert_eq!(
        steps,
        [
            "circle",
            "stroke",
            "move_to",
            "line_to",
            "close_path",
            "stroke",
            "circle",
            "fill"
        ]
    );
    assert!(
        v["icon"][1].get("x").is_none(),
        "a paint carries no numbers"
    );
}

/// A struct with `inherits` renders `"inherits": "SpinEdit"`.
#[test]
fn struct_with_inherits_renders_inherits_key() {
    let rendered = render(
        r#"
        #[crawrad(palette = "Additional", prefix = "pe", size = (121.0, 24.0), inherits = "SpinEdit")]
        pub struct PriceEdit {
            #[prop(category = "appearance", default = "USD")]
            pub currency: String,
        }
        "#,
    );
    let v: serde_json::Value = serde_json::from_str(&rendered).unwrap();
    assert_eq!(v["inherits"], "SpinEdit");
    assert!(rendered.contains(r#""inherits": "SpinEdit""#));
}

/// A struct without `inherits` renders no `inherits` key.
#[test]
fn struct_without_inherits_renders_no_inherits_key() {
    let rendered = gauge();
    let v: serde_json::Value = serde_json::from_str(&rendered).unwrap();
    assert!(v.get("inherits").is_none());
    assert!(!rendered.contains("\"inherits\""));
}
