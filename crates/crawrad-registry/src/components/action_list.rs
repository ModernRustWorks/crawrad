use crate::{
    Children, ComponentDesc, Events, ICON_SIZE, Props, UNIVERSAL_TARGETS, non_visual_props,
};
use crawrad_note::note;

#[note("components-action-list-action-list")]
pub fn action_list() -> ComponentDesc {
    crate::component!(
        "ActionList",
        display: "ActionList",
        base_name: "action_list",
        palette: "Standard",
        prefix: "acl",
        size: (ICON_SIZE.0, ICON_SIZE.1),
        visual: false,
        // Pure data: `ActionList` exists for the designer and for whoever reads the
        // `.crawform`. None of it becomes running code — `Timer` is the first
        // non-visual that does.
        emits_code: true,
        // `Collection`, and not `Area`: `Action`s are its children, and it has
        // no area at all — it is a 28-point badge. While this was just
        // `container: true`, `drop_parent` accepted the badge as parent and
        // the component dropped on top of it disappeared. See [`crate::Children`].
        children: Children::Collection { item: "Action" },
        base: non_visual_props(),
        props: [
            Props::enabled(true),
            Props::images(),
        ],
    )
}

#[note("components-action-list-action")]
pub fn action() -> ComponentDesc {
    let mut props = non_visual_props();
    props.extend([
        Props::caption(""),
        Props::hint(""),
        Props::shortcut(""),
        // The `Category` of the action: the ActionList editor groups by it,
        // and it is what standard templates already carried without a home.
        Props::action_category(""),
        Props::image_index(-1),
        Props::enabled(true),
        Props::visible(true),
        Props::checked(false),
        Props::dataset(),
        Props::dataset_operation("None"),
    ]);
    ComponentDesc {
        inherits: None,
        class: "Action",
        // Off the palette: action is born inside a list, not loose on the
        // form — this is what the classic RAD IDE does in the ActionList editor.
        palette: "",
        display: "Action",
        base_name: "action",
        prefix: "act",
        default_size: ICON_SIZE,
        props,
        events: vec![Events::EXECUTE, Events::UPDATE],
        targets: UNIVERSAL_TARGETS,
        children: Children::NoChildren,
        derived_geometry: false,
        visual: false,
        emits_code: true,
        // Native has no hidden field: its type is emitted by us.
        private_state: false,
        // Native offers no capability beyond its own class.
        provides: &[],
        text_line: None,
        requires: Vec::new(),
        crate_module: "",
        desenho: Vec::new(),
        icon: Vec::new(),
    }
}
