use crate::{Category, ComponentDesc, Events, Props, component};
use crawrad_note::note;

const SCROLL_BARS: &[&str] = &["ssNone", "ssHorizontal", "ssVertical", "ssBoth"];

#[note("components-memo-memo")]
pub fn memo() -> ComponentDesc {
    let mut d = component!(
        "Memo",
        display: "Memo",
        base_name: "memo",
        palette: "Standard",
        prefix: "mmo",
        size: (200.0, 90.0),
        props: [
            Props::multiline_text("text", "Lines", "Line1\nLine2\nLine3"),
            Props::readonly(false),
            Props::word_wrap(true),
            Props::want_returns(true),
            Props::want_tabs(false),
            Props::max_length(0),
            Props::enum_options(
                "scroll_bars",
                "ScrollBars",
                "ssNone",
                SCROLL_BARS,
                Category::Appearance,
            ),
        ],
        events: [Events::CHANGE],
    );
    // Font, Color, ParentFont, ParentColor
    d.props.extend(crate::components::base::appearance_props());
    d
}
