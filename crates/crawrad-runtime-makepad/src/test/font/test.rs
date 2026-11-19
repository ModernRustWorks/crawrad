//! Tests for control font and effective font resolution.

use std::collections::BTreeMap;

use crawrad_ir::{Color, Document, Font, Rect, Value};
use crawrad_registry::Registry;
use crawrad_runtime::Runtime;

use crate::MakepadRuntime;
use crate::Transformacao;
use crate::canvas::{Brush, Layer, Mark, Probe, effective_font, paint_controls};

const BOX: (f32, f32, f32, f32) = (10.0, 10.0, 200.0, 120.0);

fn probe_form_control(
    form_props: &[(&str, Value)],
    class: &str,
    ctl_props: &[(&str, Value)],
) -> Result<Vec<Mark>, String> {
    let mut doc = Document::new("form", "Form");
    if let Some(form) = doc.node_mut("form") {
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
        for (k, v) in form_props {
            form.props.insert((*k).to_string(), (*v).clone());
        }
    }
    let reg = Registry::builtin();
    let mut node = reg
        .instantiate(class, "probe", Rect::new(BOX.0, BOX.1, BOX.2, BOX.3))
        .ok_or_else(|| "the catalogue does not instantiate it".to_string())?;
    for (k, v) in ctl_props {
        node.props.insert((*k).to_string(), (*v).clone());
    }
    doc.add_child("form", node).map_err(|e| format!("{e:?}"))?;
    let mut rt = MakepadRuntime::new();
    rt.open(&doc).map_err(|e| format!("{e:?}"))?;
    let mut probe = Probe::default();
    let mut p = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
    paint_controls(
        &mut p,
        &rt.ctls,
        &rt.ordem,
        rt.modo,
        rt.form_rect,
        rt.form_font.as_ref(),
    );
    Ok(probe.marks().to_vec())
}

#[test]
fn test_effective_font_without_font_prop() {
    let props = BTreeMap::new();
    let form_font = Font::new("Segoe UI", 12.0);
    assert_eq!(effective_font(&props, None), (None, None));
    assert_eq!(effective_font(&props, Some(&form_font)), (None, None));

    let mut other_props = BTreeMap::new();
    other_props.insert("text".into(), Value::Str("Hello".into()));
    assert_eq!(effective_font(&other_props, Some(&form_font)), (None, None));
}

#[test]
fn test_effective_font_parent_font_true() {
    let mut props = BTreeMap::new();
    props.insert("parent_font".into(), Value::Bool(true));
    props.insert(
        "font".into(),
        Value::Font(Font::new("Arial", 18.0).color(Color::rgb(0, 255, 0))),
    );

    let form_font = Font::new("Segoe UI", 14.0).color(Color::rgb(255, 0, 0));
    let (size, color) = effective_font(&props, Some(&form_font));
    assert_eq!(size, Some(14.0));
    assert_eq!(color, Some([1.0, 0.0, 0.0, 1.0]));

    // Without a form font, resolves to (None, None)
    assert_eq!(effective_font(&props, None), (None, None));
}

#[test]
fn test_effective_font_parent_font_false() {
    let mut props = BTreeMap::new();
    props.insert("parent_font".into(), Value::Bool(false));
    props.insert(
        "font".into(),
        Value::Font(Font::new("Arial", 18.0).color(Color::rgb(0, 0, 255))),
    );

    let form_font = Font::new("Segoe UI", 14.0);
    let (size, color) = effective_font(&props, Some(&form_font));
    assert_eq!(size, Some(18.0));
    assert_eq!(color, Some([0.0, 0.0, 1.0, 1.0]));
}

#[test]
fn test_effective_font_size_zero_and_transparent_color() {
    let mut props = BTreeMap::new();
    props.insert("parent_font".into(), Value::Bool(false));
    props.insert(
        "font".into(),
        Value::Font(Font::new("Arial", 0.0).color(Color::transparent())),
    );

    let (size, color) = effective_font(&props, None);
    assert_eq!(size, None);
    assert_eq!(color, None);
}

#[test]
fn test_effective_font_parent_font_absent() {
    let mut props = BTreeMap::new();
    props.insert(
        "font".into(),
        Value::Font(Font::new("Arial", 16.0).color(Color::rgb(128, 0, 0))),
    );

    let form_font = Font::new("Segoe UI", 10.0);
    let (size, color) = effective_font(&props, Some(&form_font));
    assert_eq!(size, Some(16.0));
    assert_eq!(color, Some([128.0 / 255.0, 0.0, 0.0, 1.0]));
}

#[test]
fn test_probe_label_font_size_18_and_parent_font_false() {
    let marks = probe_form_control(
        &[],
        "Label",
        &[
            ("font", Value::Font(Font::new("Segoe UI", 18.0))),
            ("parent_font", Value::Bool(false)),
        ],
    )
    .expect("probe label");

    let text_mark = marks
        .iter()
        .find(|m| m.layer == Layer::Text)
        .expect("text mark");
    assert_eq!(text_mark.font_size, Some(18.0));

    let theme_marks = probe_form_control(&[], "Label", &[]).expect("probe theme label");
    let theme_text_mark = theme_marks
        .iter()
        .find(|m| m.layer == Layer::Text)
        .expect("theme text mark");
    assert!(
        text_mark.rect.size.x > theme_text_mark.rect.size.x,
        "size 18 mark width ({}) must be wider than theme width ({})",
        text_mark.rect.size.x,
        theme_text_mark.rect.size.x
    );
}

#[test]
fn test_probe_label_parent_font_true_with_form_font_14() {
    let marks = probe_form_control(
        &[("font", Value::Font(Font::new("Segoe UI", 14.0)))],
        "Label",
        &[("parent_font", Value::Bool(true))],
    )
    .expect("probe label");

    let text_mark = marks
        .iter()
        .find(|m| m.layer == Layer::Text)
        .expect("text mark");
    assert_eq!(text_mark.font_size, Some(14.0));
}

#[test]
fn test_probe_label_size_zero_yields_none() {
    let marks = probe_form_control(
        &[],
        "Label",
        &[
            ("font", Value::Font(Font::new("Segoe UI", 0.0))),
            ("parent_font", Value::Bool(false)),
        ],
    )
    .expect("probe label");

    let text_mark = marks
        .iter()
        .find(|m| m.layer == Layer::Text)
        .expect("text mark");
    assert_eq!(text_mark.font_size, None);
}

#[test]
fn test_probe_label_font_colour_alpha_gt_zero() {
    let red = Color::rgb(255, 0, 0);
    let marks = probe_form_control(
        &[],
        "Label",
        &[
            ("font", Value::Font(Font::new("Segoe UI", 12.0).color(red))),
            ("parent_font", Value::Bool(false)),
        ],
    )
    .expect("probe label");

    let text_mark = marks
        .iter()
        .find(|m| m.layer == Layer::Text)
        .expect("text mark");
    assert_eq!(text_mark.color, [1.0, 0.0, 0.0, 1.0]);
}

#[test]
fn test_probe_disabled_control_dim_colour_kept() {
    let red = Color::rgb(255, 0, 0);
    let marks = probe_form_control(
        &[],
        "Label",
        &[
            ("enabled", Value::Bool(false)),
            ("font", Value::Font(Font::new("Segoe UI", 12.0).color(red))),
            ("parent_font", Value::Bool(false)),
        ],
    )
    .expect("probe label");

    let text_mark = marks
        .iter()
        .find(|m| m.layer == Layer::Text)
        .expect("text mark");
    assert_ne!(text_mark.color, [1.0, 0.0, 0.0, 1.0]);
    assert_eq!(text_mark.color, crate::components::comum::MAKEPAD_TEXT_DIM);
}

#[test]
fn test_probe_grid_column_title_font_and_cell_font() {
    let mut doc = Document::new("form", "Form");
    if let Some(form) = doc.node_mut("form") {
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    }
    let reg = Registry::builtin();
    let mut grid_node = reg
        .instantiate("Grid", "grid1", Rect::new(10.0, 10.0, 400.0, 300.0))
        .expect("Grid instantiates");
    grid_node.set_name("grid1");

    let mut col_node = reg
        .instantiate("GridColumn", "col1", Rect::new(0.0, 0.0, 28.0, 28.0))
        .expect("GridColumn instantiates");
    col_node.set_name("col1");
    col_node
        .props
        .insert("field".into(), Value::Str("name".into()));
    col_node.props.insert(
        "title_font".into(),
        Value::Font(Font::new("Segoe UI", 16.0)),
    );
    col_node
        .props
        .insert("font".into(), Value::Font(Font::new("Segoe UI", 12.0)));
    grid_node.children.push(col_node.id.clone());
    doc.nodes.insert(col_node.id.clone(), col_node);
    doc.add_child("form", grid_node).expect("grid goes in form");

    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("open document");

    let mut rows = BTreeMap::new();
    rows.insert(
        "grid1".to_string(),
        crawrad_ir::design_rows::GridRows::Ready {
            sample: crawrad_ir::design_rows::Sample {
                columns: vec!["name".into()],
                kinds: vec!["TEXT".into()],
                rows: vec![vec![Some("Alice".into())]],
                total: 1,
            },
            table: "users".into(),
            file: None,
        },
    );
    rt.set_design_rows(rows);

    let mut probe = Probe::default();
    let mut p = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
    paint_controls(
        &mut p,
        &rt.ctls,
        &rt.ordem,
        rt.modo,
        rt.form_rect,
        rt.form_font.as_ref(),
    );

    let header_mark = probe
        .marks()
        .iter()
        .find(|m| m.layer == Layer::Text && m.text.as_deref() == Some("Name"))
        .expect("header text mark");
    assert_eq!(header_mark.font_size, Some(16.0));

    let cell_mark = probe
        .marks()
        .iter()
        .find(|m| m.layer == Layer::Text && m.text.as_deref() == Some("Alice"))
        .expect("cell text mark");
    assert_eq!(cell_mark.font_size, Some(12.0));
}

/// A canvas probe test showing that a wrapped cell produces two text marks on two lines.
#[test]
fn canvas_probe_wrapped_cell_produces_two_text_marks_on_two_lines() {
    let mut doc = Document::new("form", "Form");
    if let Some(form) = doc.node_mut("form") {
        form.set_rect(Rect::new(0.0, 0.0, 800.0, 600.0));
    }
    let reg = Registry::builtin();
    let mut grid_node = reg
        .instantiate("Grid", "grid1", Rect::new(10.0, 10.0, 400.0, 300.0))
        .expect("Grid instantiates");
    grid_node.set_name("grid1");
    grid_node
        .props
        .insert("default_row_height".into(), Value::Int(48));

    let mut col_node = reg
        .instantiate("GridColumn", "col1", Rect::new(0.0, 0.0, 28.0, 28.0))
        .expect("GridColumn instantiates");
    col_node.set_name("col1");
    col_node
        .props
        .insert("field".into(), Value::Str("title".into()));
    col_node.props.insert("width".into(), Value::Int(60));
    col_node.props.insert("word_wrap".into(), Value::Bool(true));
    grid_node.children.push(col_node.id.clone());
    doc.nodes.insert(col_node.id.clone(), col_node);
    doc.add_child("form", grid_node).expect("grid goes in form");

    let mut rt = MakepadRuntime::new();
    rt.open(&doc).expect("open document");

    let mut rows = BTreeMap::new();
    rows.insert(
        "grid1".to_string(),
        crawrad_ir::design_rows::GridRows::Ready {
            sample: crawrad_ir::design_rows::Sample {
                columns: vec!["title".into()],
                kinds: vec!["TEXT".into()],
                rows: vec![vec![Some("Hello World".into())]],
                total: 1,
            },
            table: "books".into(),
            file: None,
        },
    );
    rt.set_design_rows(rows);

    let mut probe = Probe::default();
    let mut p = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
    paint_controls(
        &mut p,
        &rt.ctls,
        &rt.ordem,
        rt.modo,
        rt.form_rect,
        rt.form_font.as_ref(),
    );

    let cell_marks: Vec<_> = probe
        .marks()
        .iter()
        .filter(|m| {
            m.layer == Layer::Text
                && (m.text.as_deref() == Some("Hello") || m.text.as_deref() == Some("World"))
        })
        .collect();

    assert_eq!(
        cell_marks.len(),
        2,
        "wrapped cell must produce two text marks: {cell_marks:?}"
    );
    assert_eq!(cell_marks[0].text.as_deref(), Some("Hello"));
    assert_eq!(cell_marks[1].text.as_deref(), Some("World"));
    assert!(
        cell_marks[1].rect.pos.y > cell_marks[0].rect.pos.y,
        "second line must be below first line: y0={}, y1={}",
        cell_marks[0].rect.pos.y,
        cell_marks[1].rect.pos.y
    );
}
