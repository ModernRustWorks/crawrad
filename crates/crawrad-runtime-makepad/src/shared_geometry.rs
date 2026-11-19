// SINGLE SOURCE OF THE SCROLL BAR GEOMETRY — one file, two readers.
//
// This file is compiled HERE, as a module of `crawrad-runtime-makepad`, which is
// what paints the scroll bar on the designer canvas. And it is compiled AGAIN
// inside `crawrad-codegen`, by an `include!` on the path, which is what computes
// the numbers baked into the generated application.
//
// ## Why it exists, and what it cost before
//
// The canvas had `BAR_BUTTON_SIDE: f32 = 16.0` as a named constant. The
// generator had `16.0` and `32.0` as BARE LITERALS, in another crate, written
// out eight times between `emit_dsl`, `emit_sync` and `emit_actions`. Nothing
// tied the two, and changing either one made the designer and the application
// disagree in silence — the broken-WYSIWYG family of defect this house keeps
// paying for.
//
// It also made the `ScrollBar` look irreducible. Its `emit_sync` does
// arithmetic over numbers that only its `emit_dsl` knew — the `32.0` is the
// width of the `<` `>` buttons that `emit_dsl` itself drew. That is not a
// property of the class: it is a coupling between two functions that had no
// name in common. Naming the geometry dissolves it.
//
// ## Why this is NOT the egui `desenho_comum.rs`
//
// In the egui target, the preview and the generated application run the SAME
// drawing code, so the shared file carries `fn page_control(ui, ...)` whole.
//
// In Makepad they cannot: the preview paints with `Brush` (ADR 0027, a custom
// immediate-draw widget) and the application builds a retained widget tree from
// `live_design!`. There is no drawing text that compiles in both places.
//
// What CAN be shared is the arithmetic — and the arithmetic is what makes the
// two agree. So the Makepad valve carries numbers and pure functions, never
// painting.
//
// ## The authoring rules
//
// 1. **No `use`.** The file is included in two crates whose imports differ.
// 2. **No type of ours.** `f32` and nothing else — `crawrad_ir`, `Brush` and
//    `makepad_widgets` all exist on one side only.
// 3. **Pure.** Every function here is arithmetic over its arguments, so the
//    canvas and the generator cannot drift by calling it differently.

/// Side of the `<` and `>` buttons at the ends of the bar, in design units.
///
/// It is a fixed screen measure on the canvas so the button stays 16 px at any
/// zoom, and it is the `width: 16.0` of the two `<Button>` in the emitted
/// `live_design!`. One number, and now one place.
pub const BAR_BUTTON_SIDE: f32 = 16.0;

/// Length of the thumb along the bar's main axis, in design units.
pub const BAR_THUMB_LEN: f32 = 32.0;

/// The track: what is left of the bar once both end buttons are taken out.
///
/// `len` is the bar's length along its main axis — width for a horizontal bar,
/// height for a vertical one.
pub fn bar_track_len(len: f32) -> f32 {
    (len - 2.0 * BAR_BUTTON_SIDE).max(0.0)
}

/// How far the thumb can actually travel: the track minus the thumb itself.
///
/// Zero when the bar is too short to hold a thumb, which is why the clamp is
/// here and not at every call site.
pub fn bar_available_travel(track_len: f32) -> f32 {
    (track_len - BAR_THUMB_LEN).max(0.0)
}

/// Where the thumb sits, in design units from the start of the track.
///
/// `ratio` is the position already normalised to `0.0..=1.0`.
pub fn bar_thumb_offset(ratio: f32, available_travel: f32) -> f32 {
    ratio.clamp(0.0, 1.0) * available_travel
}

/// The ratio a pointer at `rel` design units into the track stands for.
///
/// The `max(1.0)` is deliberate and not a guard against nonsense: a zero-length
/// track would divide by zero, and the answer that costs least is *the bar is
/// at its start*.
pub fn bar_ratio_at(rel: f32, track_len: f32) -> f32 {
    (rel / track_len.max(1.0)).clamp(0.0, 1.0)
}

/// Height of one row of a `ListBox`, in design units.
///
/// # The designer said 18 and the application said 20
///
/// This number was declared in `componentes/list_box.rs` as `ALTURA_DA_LINHA`
/// and, in the generator, as a bare `height: 20.0` inside the emitted
/// `live_design!`. Two numbers, two crates, nothing comparing them — so a
/// `ListBox` showing five rows on the canvas showed four and a half in the
/// running application.
///
/// The canvas wins, and it is not a coin toss: the designer IS the contract.
/// What the owner positions is what has to appear.
pub const LIST_ROW_HEIGHT: f32 = 18.0;

/// Height of one row of a `TreeView`, in design units.
///
/// Same defect as [`LIST_ROW_HEIGHT`], same fix: the canvas drew 17 and the
/// generated application emitted 20.
pub const TREE_ROW_HEIGHT: f32 = 17.0;

/// Height of one line of a `Memo`, in design units.
///
/// It has no divergence to fix — the generated `Memo` uses the framework's own
/// `TextInput` and never emits a per-line height. It is here because it is the
/// third of the three row measures and keeping one of them somewhere else is
/// how the first two drifted apart.
pub const MEMO_LINE_HEIGHT: f32 = 15.0;

/// Width of one tab of a `PageControl` strip, in design units. Mirrors
/// `crawrad_registry::TAB_WIDTH`; the canvas test holds the copy to the catalog.
pub const TAB_WIDTH: f32 = 92.0;

/// Width of each of the two scroll buttons (`<` `>`) a strip shows when its
/// tabs do not fit. Mirrors `crawrad_registry::TAB_SCROLL_BUTTON_WIDTH`.
pub const TAB_SCROLL_BUTTON_WIDTH: f32 = 16.0;

/// Which tabs of a strip are on screen, as `(first, count, arrows)`.
///
/// `width` is the strip's width, `shown` how many tabs it has, `active` the
/// position of the open one among them, and `reserved` what the right edge
/// keeps for itself (the design-time "+", or nothing in the application).
/// When the tabs overflow, the `<` `>` buttons take the right end and the
/// window is the run of tabs that fits and holds the open one.
///
/// This is `crawrad_registry::tab_window` — the runtime does not depend on the
/// catalog, so it carries the rule here, and the canvas test checks the two
/// answer the same over a grid of inputs.
pub fn tab_window(width: f32, shown: usize, active: usize, reserved: f32) -> (usize, usize, bool) {
    if shown == 0 {
        return (0, 0, false);
    }
    let avail = width - reserved;
    if shown as f32 * TAB_WIDTH <= avail {
        return (0, shown, false);
    }
    let per = ((avail - 2.0 * TAB_SCROLL_BUTTON_WIDTH) / TAB_WIDTH)
        .floor()
        .max(1.0) as usize;
    let count = per.min(shown);
    let first = active
        .min(shown - 1)
        .saturating_sub(count - 1)
        .min(shown - count);
    (first, count, true)
}

// ## `fraction` MOVED OUT, on 18/09/2026
//
// It used to live here, cut out by `[CRAWRAD:INICIO fraction]`. It moved to
// `crates/crawrad-component/src/fraction.rs` — THE BOX — when the owner asked the
// question this whole file is an answer to: *"where is our box, the one people
// build components on top of?"*
//
// The reason it moved and the bar geometry did not: the bar geometry is ours,
// read by the canvas and by the generator. `fraction` is needed by whoever
// WRITES a component, and that person reaches only what the box offers.
//
// Everything below is still read as VALUES: compiled here, and compiled again
// inside `crawrad-codegen`, which bakes the numbers into the generated application.
