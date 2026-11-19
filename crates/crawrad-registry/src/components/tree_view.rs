use crate::{ComponentDesc, Events, Props, component};

pub fn tree_view() -> ComponentDesc {
    let mut d = component!(
        "TreeView",
        display: "TreeView",
        base_name: "tree_view",
        palette: "Additional",
        prefix: "tv",
        size: (140.0, 160.0),
        props: [
            Props::tree_items_sample(),
            Props::images(),
            // Removed with RCL (ADR 0013): `ShowLines`, `ShowRoot`,
            // `ShowButtons`, `FullRowSelect`, `HotTrack`, and tree `ReadOnly`
            // are `comctl32` vocabulary and mean nothing in
            // egui. A file containing them remains valid — the namespace exists
            // for this, and the `.crawform` preserves them.
        ],
        events: [
            Events::CHANGE,
            Events::CHANGING,
            Events::COLLAPSED,
            Events::COLLAPSING,
            Events::EXPANDED,
            Events::EXPANDING,
            Events::MOUSE_CLICK,
        ],
    );
    d.props.extend(crate::components::base::appearance_props());
    d
}
