use crate::Color;
use crate::canvas::{Axis, Canvas, CanvasCommand, FillStyle, StrokeStyle, TextAlign, ThemeRole};

#[test]
fn canvas_initialization_and_dimensions() {
    let canvas = Canvas::new(140.0, 48.0);
    assert_eq!(canvas.width(), 140.0);
    assert_eq!(canvas.height(), 48.0);
    assert_eq!(*canvas.fill_style(), FillStyle::Theme(ThemeRole::Surface));
    assert_eq!(
        *canvas.stroke_style(),
        StrokeStyle::Theme(ThemeRole::Border)
    );
    assert_eq!(canvas.line_width(), 1.0);
}

#[test]
fn canvas_fill_and_stroke_styles_with_theme_and_colors() {
    let mut canvas = Canvas::new(100.0, 100.0);

    // Theme role
    canvas.set_fill_style(ThemeRole::Accent);
    assert_eq!(*canvas.fill_style(), FillStyle::Theme(ThemeRole::Accent));

    // Explicit color
    let red = Color::rgb(255, 0, 0);
    canvas.set_fill_style(red);
    assert_eq!(*canvas.fill_style(), FillStyle::Color(red));

    // Stroke
    canvas.set_stroke_style(ThemeRole::TextDim);
    assert_eq!(
        *canvas.stroke_style(),
        StrokeStyle::Theme(ThemeRole::TextDim)
    );

    canvas.set_line_width(2.5);
    assert_eq!(canvas.line_width(), 2.5);
}

#[test]
fn canvas_save_and_restore_state() {
    let mut canvas = Canvas::new(100.0, 100.0);
    canvas.set_fill_style(ThemeRole::Surface);
    canvas.set_line_width(1.0);
    canvas.set_text_align(TextAlign::Left);

    canvas.save();

    canvas.set_fill_style(ThemeRole::Accent);
    canvas.set_line_width(5.0);
    canvas.set_text_align(TextAlign::Center);

    assert_eq!(*canvas.fill_style(), FillStyle::Theme(ThemeRole::Accent));
    assert_eq!(canvas.line_width(), 5.0);
    assert_eq!(canvas.text_align(), TextAlign::Center);

    canvas.restore();

    assert_eq!(*canvas.fill_style(), FillStyle::Theme(ThemeRole::Surface));
    assert_eq!(canvas.line_width(), 1.0);
    assert_eq!(canvas.text_align(), TextAlign::Left);
}

#[test]
fn canvas_rectangles_and_paths() {
    let mut canvas = Canvas::new(200.0, 100.0);
    canvas.fill_rect(10.0, 10.0, 180.0, 80.0);
    canvas.stroke_rect(10.0, 10.0, 180.0, 80.0);
    canvas.clear_rect(20.0, 20.0, 50.0, 50.0);
    canvas.fill_round_rect(0.0, 0.0, 200.0, 100.0, 4.0);

    canvas.begin_path();
    canvas.move_to(0.0, 0.0);
    canvas.line_to(100.0, 50.0);
    canvas.arc(50.0, 50.0, 25.0, 0.0, std::f32::consts::PI);
    canvas.close_path();
    canvas.stroke();
    canvas.fill();

    let cmds = canvas.commands();
    assert!(cmds.contains(&CanvasCommand::FillRect {
        x: 10.0,
        y: 10.0,
        width: 180.0,
        height: 80.0,
    }));
    assert!(cmds.contains(&CanvasCommand::StrokeRect {
        x: 10.0,
        y: 10.0,
        width: 180.0,
        height: 80.0,
    }));
    assert!(cmds.contains(&CanvasCommand::BeginPath));
    assert!(cmds.contains(&CanvasCommand::ClosePath));
    assert!(cmds.contains(&CanvasCommand::Stroke));
    assert!(cmds.contains(&CanvasCommand::Fill));
}

#[test]
fn canvas_text_and_meter_proportions() {
    let mut canvas = Canvas::new(140.0, 48.0);
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

    let json = canvas.to_manifest_json();
    assert!(json.contains(r#""verb": "box""#));
    assert!(json.contains(r#""fill": "surface""#));
    assert!(json.contains(r#""radius": 4.0"#));
    assert!(json.contains(r#""fraction_of": "value""#));
    assert!(json.contains(r#""thickness": 2.0"#));
    assert!(json.contains(r#""verb": "text""#));
    assert!(json.contains(r#""prop": "text""#));
    assert!(json.contains(r#""centered": true"#));
}
