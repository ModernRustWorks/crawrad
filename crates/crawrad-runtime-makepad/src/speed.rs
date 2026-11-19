//! **A SpeedButton's down and flat looks**.
//!
//! The generated SpeedButton states its three colours in the DSL (up,
//! hover, pressed), so this knows what to put back: down shows the pressed
//! colour at rest, flat leaves the rest colour transparent (the frame shows
//! only under the pointer).

use crawrad_note::note;
use makepad_widgets::*;

#[note("speed-up")]
pub const UP: Vec4 = Vec4 {
    x: 0.118,
    y: 0.118,
    z: 0.118,
    w: 1.0,
};
pub const HOVER: Vec4 = Vec4 {
    x: 0.173,
    y: 0.173,
    z: 0.204,
    w: 1.0,
};
pub const DOWN: Vec4 = Vec4 {
    x: 0.180,
    y: 0.278,
    z: 0.478,
    w: 1.0,
};
pub const DOWN_HOVER: Vec4 = Vec4 {
    x: 0.220,
    y: 0.345,
    z: 0.588,
    w: 1.0,
};
const NONE: Vec4 = Vec4 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 0.0,
};

#[note("speed-look")]
pub fn look(cx: &mut Cx, widget: &WidgetRef, down: bool, flat: bool) {
    let (rest, hover) = if down {
        (DOWN, DOWN_HOVER)
    } else if flat {
        (NONE, HOVER)
    } else {
        (UP, HOVER)
    };
    // a clicked button keeps the focus, and the shader mixes the focus colour
    // in: it is the rest colour here, or a pressed button looks up
    widget.apply_over(
        cx,
        live! { draw_bg: { color: (rest), color_hover: (hover), color_focus: (rest) } },
    );
}
