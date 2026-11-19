use super::*;
use crate::Gerador;
use crawrad_ir::{Document, Node, Rect, Value};
use crawrad_registry::Registry;

fn form_with_track_bar(look: Option<&str>) -> Document {
    let mut doc = Document::new("form_main", "Form");
    {
        let root = doc.node_mut("form_main").expect("form root");
        root.set_rect(Rect::new(0.0, 0.0, 640.0, 480.0));
        root.set_name("form_main");
        root.set_text("Form1");
    }
    let mut tb = Node::new("tb_1", "TrackBar");
    tb.set_name("track_bar1");
    tb.set_rect(Rect::new(10.0, 10.0, 150.0, 32.0));
    if let Some(l) = look {
        tb.props
            .insert("makepad.look".to_string(), Value::Enum(l.to_string()));
    }
    doc.add_child("form_main", tb)
        .expect("add track bar to form");
    doc
}

#[test]
fn each_look_emits_its_own_slider() {
    let doc = Document::new("form_main", "Form");
    let reg = Registry::builtin();

    // 1. Key absent: emits <SliderMinimal>
    let mut node = Node::new("tb_1", "TrackBar");
    node.set_name("tb_1");
    let dsl_absent = TrackBarCodegen
        .emit_dsl(&doc, &reg, &node, 0.0, 0.0, 150.0, 32.0)
        .expect("emit_dsl absent");
    assert!(
        dsl_absent.contains("<SliderMinimal>"),
        "expected <SliderMinimal>, got: {dsl_absent}"
    );

    // 2. Minimal: emits <SliderMinimal>
    node.props.insert(
        "makepad.look".to_string(),
        Value::Enum("Minimal".to_string()),
    );
    let dsl_minimal = TrackBarCodegen
        .emit_dsl(&doc, &reg, &node, 0.0, 0.0, 150.0, 32.0)
        .expect("emit_dsl Minimal");
    assert!(
        dsl_minimal.contains("<SliderMinimal>"),
        "expected <SliderMinimal>, got: {dsl_minimal}"
    );

    // 3. Box: emits <Slider>, and NOT <SliderMinimal> or <SliderRound>
    node.props
        .insert("makepad.look".to_string(), Value::Enum("Box".to_string()));
    let dsl_box = TrackBarCodegen
        .emit_dsl(&doc, &reg, &node, 0.0, 0.0, 150.0, 32.0)
        .expect("emit_dsl Box");
    assert!(
        dsl_box.contains("<Slider>"),
        "expected <Slider>, got: {dsl_box}"
    );
    assert!(
        !dsl_box.contains("<SliderMinimal>"),
        "dsl for Box must not contain <SliderMinimal>: {dsl_box}"
    );
    assert!(
        !dsl_box.contains("<SliderRound>"),
        "dsl for Box must not contain <SliderRound>: {dsl_box}"
    );

    // 4. Round: emits <SliderRound>
    node.props
        .insert("makepad.look".to_string(), Value::Enum("Round".to_string()));
    let dsl_round = TrackBarCodegen
        .emit_dsl(&doc, &reg, &node, 0.0, 0.0, 150.0, 32.0)
        .expect("emit_dsl Round");
    assert!(
        dsl_round.contains("<SliderRound>"),
        "expected <SliderRound>, got: {dsl_round}"
    );
    assert!(
        dsl_round.contains("label_size: 0.0"),
        "the pill must not keep the 75 px label gap: {dsl_round}"
    );
    assert!(
        !dsl_box.contains("label_size"),
        "only the pill needs the label gap closed: {dsl_box}"
    );
}

#[test]
fn the_look_never_becomes_a_state_field() {
    let reg = Registry::builtin();
    let desc = reg.get("TrackBar").expect("TrackBar descriptor");
    let fields = state::fields_of(desc);
    assert!(
        !fields.iter().any(|(name, _)| name.contains("look")),
        "state fields must not contain `look`, got: {fields:?}"
    );
}

#[test]
fn egui_ignores_the_makepad_look() {
    let reg = Registry::builtin();
    let doc_plain = form_with_track_bar(None);
    let doc_round = form_with_track_bar(Some("Round"));

    let out_plain = crate::GeradorRxp
        .gerar(&doc_plain, "app", &reg)
        .expect("generating plain egui form");
    let out_round = crate::GeradorRxp
        .gerar(&doc_round, "app", &reg)
        .expect("generating round egui form");

    assert_eq!(
        out_plain, out_round,
        "egui codegen output must be identical with or without `makepad.look`"
    );
}
