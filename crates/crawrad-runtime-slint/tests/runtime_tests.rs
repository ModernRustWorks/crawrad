use crawrad_runtime_slint::*;
use std::time::Duration;

#[test]
fn test_dialog_choice_variants() {
    let choice = DialogChoice::Ok;
    assert_eq!(choice, DialogChoice::Ok);
    assert_ne!(choice, DialogChoice::Cancel);
}

#[test]
fn test_key_event_args() {
    let args = KeyEventArgs::new("Enter", false, true, false);
    assert_eq!(args.key, "Enter");
    assert!(args.control);
    assert!(!args.shift);
}

#[test]
fn test_hex_to_slint_color() {
    assert_eq!(hex_to_slint_color("#FFFFFF"), "#FFFFFFff");
    assert_eq!(hex_to_slint_color("#80FF0000"), "#FF000080");
    assert_eq!(hex_to_slint_color("#F00"), "#FF0000ff");
}

#[test]
fn test_text_alignment_slint() {
    assert_eq!(text_alignment_slint("taLeftJustify"), "left");
    assert_eq!(text_alignment_slint("taCenter"), "center");
    assert_eq!(text_alignment_slint("taRightJustify"), "right");
}

#[test]
fn test_simple_list_model() {
    let mut model = SimpleListModel::new(vec!["First".into(), "Second".into()]);
    assert_eq!(model.selected_text(), None);
    model.select(1);
    assert_eq!(model.selected_text(), Some("Second"));
    model.select(99);
    assert_eq!(model.selected_text(), None);
}

#[test]
fn test_timer_state_duration() {
    let timer = TimerState::new(500, true);
    assert_eq!(timer.duration(), Duration::from_millis(500));
    assert!(timer.enabled);
}

#[test]
fn test_form_window_state() {
    assert_eq!(
        FormWindowState::from_property("wsMaximized"),
        FormWindowState::Maximized
    );
    assert_eq!(
        FormWindowState::from_property("wsMinimized"),
        FormWindowState::Minimized
    );
    assert_eq!(
        FormWindowState::from_property("other"),
        FormWindowState::Normal
    );
}

#[test]
fn test_calculate_screen_center() {
    let (x, y) = calculate_screen_center(1920.0, 1080.0, 800.0, 600.0);
    assert_eq!(x, 560.0);
    assert_eq!(y, 240.0);
}
