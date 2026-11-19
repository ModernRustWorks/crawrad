use crate::{BREADCRUMB_STYLES, PropKind, Registry};
use crawrad_ir::Value;

#[test]
fn test_breadcrumb_registry_props() {
    let r = Registry::builtin();
    let desc = r.get("Breadcrumb").expect("Breadcrumb component");

    let style_prop = desc
        .props
        .iter()
        .find(|p| p.key == "style")
        .expect("style property");
    assert_eq!(style_prop.display, "Style");
    assert_eq!(style_prop.default, Value::Enum("bsText".into()));
    assert_eq!(style_prop.kind, PropKind::Enum(BREADCRUMB_STYLES));
    assert_eq!(style_prop.options, &["bsText", "bsChevron"]);

    let segment_color_prop = desc
        .props
        .iter()
        .find(|p| p.key == "segment_color")
        .expect("segment_color property");
    assert_eq!(segment_color_prop.display, "SegmentColor");
    assert_eq!(
        segment_color_prop.default,
        Value::Color(crawrad_ir::Color::rgb(0x45, 0x46, 0x4B))
    );
    assert_eq!(segment_color_prop.kind, PropKind::Color);

    let current_color_prop = desc
        .props
        .iter()
        .find(|p| p.key == "current_color")
        .expect("current_color property");
    assert_eq!(current_color_prop.display, "CurrentColor");
    assert_eq!(
        current_color_prop.default,
        Value::Color(crawrad_ir::Color::rgb(0xE0, 0x61, 0x2B))
    );
    assert_eq!(current_color_prop.kind, PropKind::Color);
}
