use super::*;

#[test]
fn an_inheriting_component_wears_its_base_icon() {
    let mut reg = crawrad_registry::Registry::builtin();
    reg.register_external(crawrad_registry::sample_price_edit())
        .unwrap();
    let extras = extra_icons(&reg);
    let price = extras
        .iter()
        .find(|(c, _)| c == "PriceEdit")
        .expect("PriceEdit cell");
    assert_eq!(
        price.1.as_slice(),
        ClassIcon::of_class("SpinEdit").unwrap().steps()
    );
    // A native class never gets an extra cell.
    assert!(extras.iter().all(|(c, _)| c != "SpinEdit" && c != "Button"));
    let cells = IconCells::new(extras);
    assert!(cells.kind_of("PriceEdit").is_some());
}

#[test]
fn declared_looks_map_each_inheriting_class_to_its_base() {
    let mut reg = crawrad_registry::Registry::builtin();
    reg.register_external(crawrad_registry::sample_price_edit())
        .unwrap();
    let looks = crate::installed::declared_looks(&reg);
    assert_eq!(
        looks.bases.get("PriceEdit").map(String::as_str),
        Some("SpinEdit")
    );
    assert_eq!(looks.painted_class("PriceEdit"), "SpinEdit");
    assert_eq!(looks.painted_class("Button"), "Button");
    assert_eq!(looks.bases.len(), 1);
}

#[test]
fn declared_looks_carry_the_builtin_non_visual_flag_and_icon() {
    let looks = crate::installed::declared_looks(&crawrad_registry::Registry::builtin());
    assert!(looks.non_visual.contains("Timer"));
    assert!(!looks.non_visual.contains("Button"));
    assert!(!looks.icon_of("Timer").is_empty());
}
