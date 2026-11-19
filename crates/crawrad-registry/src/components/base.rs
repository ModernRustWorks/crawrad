use crate::{ComponentDesc, EventDesc, Events, PropDesc, Props};
use crawrad_ir::Rect;
use crawrad_note::note;

#[note("components-base-icon-size")]
pub const ICON_SIZE: (f32, f32) = (28.0, 28.0);

#[note("components-base-align")]
pub const ALIGN: &[&str] = &["None", "Top", "Bottom", "Left", "Right", "Client"];

#[note("components-base-component-props")]
pub fn component_props() -> Vec<PropDesc> {
    vec![Props::name()]
}

#[note("components-base-non-visual-props")]
pub fn non_visual_props() -> Vec<PropDesc> {
    let mut props = component_props();
    props.push(Props::rect(Rect::new(0.0, 0.0, ICON_SIZE.0, ICON_SIZE.1)));
    props
}

#[note("components-base-control-props")]
pub fn control_props() -> Vec<PropDesc> {
    let mut props = component_props();
    props.extend([
        Props::rect(Rect::new(0.0, 0.0, 80.0, 24.0)),
        Props::enabled(true),
        Props::visible(true),
        Props::hint(""),
        Props::show_hint(true),
        Props::cursor(crawrad_ir::Cursor::Default),
        Props::popup_menu(),
        Props::anchors(),
        Props::align(),
        Props::margins(crawrad_ir::Edges::uniform(3.0)),
        Props::align_with_margins(false),
    ]);
    props
}

#[note("components-base-page-props")]
pub fn page_props() -> Vec<PropDesc> {
    let mut props = control_props();
    props.retain(|p| {
        !matches!(
            p.key,
            "align" | "anchors" | "margins" | "align_with_margins" | "popup_menu" | "cursor"
        )
    });
    props
}

#[note("components-base-window-props")]
pub fn window_props() -> Vec<PropDesc> {
    let mut props = control_props();
    props.push(Props::tab_order(0));
    props.push(Props::tab_stop(true));
    props
}

#[note("components-base-without-tab-stop")]
pub fn without_tab_stop(props: &mut Vec<PropDesc>) {
    props.retain(|p| p.key != "tab_stop");
}

#[note("components-base-theme-font")]
pub fn theme_font() -> crawrad_ir::Font {
    crawrad_ir::Font {
        name: String::new(),
        size: 0.0,
        ..Default::default()
    }
}

#[note("components-base-font-props")]
pub fn font_props() -> Vec<PropDesc> {
    vec![Props::font(theme_font()), Props::parent_font(true)]
}

#[note("components-base-color-props")]
pub fn color_props() -> Vec<PropDesc> {
    vec![
        Props::color(crawrad_ir::Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }),
        Props::parent_color(true),
    ]
}

#[note("components-base-appearance-props")]
pub fn appearance_props() -> Vec<PropDesc> {
    let mut v = font_props();
    v.extend(color_props());
    v
}

#[note("components-base-windowed")]
pub const WINDOWED: &[&str] = &[
    "BitBtn",
    "Breadcrumb",
    "Button",
    "CheckBox",
    "CheckListBox",
    "Chips",
    "ColorBox",
    "ComboBox",
    "DateTimePicker",
    "Edit",
    "Grid",
    "GroupBox",
    "ListBox",
    "ListView",
    "MaskEdit",
    "Memo",
    "MultiSelect",
    "PageControl",
    "Pagination",
    "Panel",
    "ProgressBar",
    "RadioButton",
    "RadioGroup",
    "ScrollBar",
    "ScrollBox",
    "SpinEdit",
    "StatusBar",
    "ToolBar",
    "Toggle",
    "TrackBar",
    "TreeView",
];

#[note("components-base-graphic")]
pub const GRAPHIC: &[&str] = &[
    "Bevel",
    "Image",
    "Label",
    "LinkLabel",
    "PaintBox",
    "Shape",
    "SpeedButton",
    "Splitter",
];

#[note("components-base-graphic-events")]
pub fn graphic_events() -> Vec<EventDesc> {
    vec![
        Events::MOUSE_CLICK,
        Events::MOUSE_DOUBLE_CLICK,
        Events::MOUSE_DOWN,
        Events::MOUSE_ENTER,
        Events::MOUSE_LEAVE,
        Events::MOUSE_MOVE,
        Events::MOUSE_UP,
    ]
}

#[note("components-base-windowed-events")]
pub fn windowed_events() -> Vec<EventDesc> {
    let mut v = graphic_events();
    v.extend([
        Events::MOUSE_ACTIVATE,
        Events::MOUSE_WHEEL,
        Events::ENTER,
        Events::EXIT,
        Events::KEY_DOWN,
        Events::KEY_PRESS,
        Events::KEY_UP,
    ]);
    v
}

#[note("components-base-with-classic-events")]
pub fn with_classic_events(mut d: ComponentDesc) -> ComponentDesc {
    let classic = if WINDOWED.contains(&d.class) {
        windowed_events()
    } else if GRAPHIC.contains(&d.class) {
        graphic_events()
    } else {
        return d;
    };
    for e in classic {
        if !d.events.iter().any(|x| x.name == e.name) {
            d.events.push(e);
        }
    }
    d.events.sort_by_key(|e| e.display);
    d
}
