use crate::rail::{RAIL, visible};
use crawrad_actions::Panel;

#[test]
fn visible_with_form_equals_rail() {
    assert_eq!(visible(true), RAIL.to_vec());
}

#[test]
fn visible_without_form_omits_inspector_and_keeps_order() {
    let panels = visible(false);
    assert!(!panels.contains(&Panel::Inspector));
    let expected: Vec<Panel> = RAIL
        .iter()
        .copied()
        .filter(|p| *p != Panel::Inspector)
        .collect();
    assert_eq!(panels, expected);
}
