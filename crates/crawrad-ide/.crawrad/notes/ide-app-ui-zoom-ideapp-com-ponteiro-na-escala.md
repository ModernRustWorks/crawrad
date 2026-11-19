---
id: ide-app-ui-zoom-ideapp-com-ponteiro-na-escala
kind: doc
target_file: src/ide_app/ui_zoom.rs
line_hint: 83
author: 
created: 
title: "IdeApp::com_ponteiro_na_escala"
tags: []
signature: "pub (super) fn com_ponteiro_na_escala (& self , cx : & Cx , event : & Event) -> Option < Event >"
---

Transforms pointer event coordinates to current interface scaling.

**The pointer, brought into the scale the drawing is in.**

The framework does this on macOS and not on Windows — the measurement
is in the module header. Without it the IDE draws at 150% and every
click lands where the 100% layout used to be.

It returns an owned `Event` instead of mutating, because `handle_event`
receives a `&Event`: the shell forwards the corrected copy to the
widget tree.

#### At `Event::Drag(e) => {`

DRAG AND DROP, and they were missing.

The owner spotted the gap dropping a component from the Toolbox onto the
form: *"look at the arrow, where I dropped it and where the component
went"*. `Event::Drop` is not a pointer event according to the
framework — it lives in `event/drag_drop.rs`, not in
`event/finger.rs` —, which is why it escaped the first list.

Yet it carries `abs` just the same (`drag_drop.rs:22` and `:31`), and it is what
the canvas's `drag_hits` reads to turn into design coordinates.
Without scaling, the component lands `fator` times farther from where the
hand released it.

#### At `let escalar = |p: DVec2| p * escala;`

EACH VARIANT WITH POSITION, one by one — and the final arm returns
`None` instead of a wrong event.

The `_` here is not the `_ => {}` that this codebase has already paid for three times: it
covers events WITHOUT position, which are the majority, and returning `None`
for them is the right answer — the caller uses the original.

#### At `let escala = 1.0 / self.zoom_da_ide;`

THE SCALE IS OURS, and the first draft used the framework's — which
DOES NOT WORK on this platform.

`remap_dpi_override` (`window.rs:264-271`) only responds when
`os_dpi_factor` is set, and this field is only assigned on
macOS (`os/apple/macos/macos.rs:310`). On Windows it is `None`
forever, the function returns the position intact, and the correction never
happened — the owner saw the result at 110%: *"I click in one place
it selects another"*.

The right calculation is exact and does not depend on that field: the
`dpi_override` we write is `base * fator`, and the position arrives
divided by `base` (`win32_window.rs:778-783`), so what remains
is to divide by the FACTOR.

#### At `Event::WindowDragQuery(e) => {`

THE WINDOW QUERY — *"does this point drag the window?"*. It
carries `abs` (`event/window.rs:66`) and is what makes the title
bar move the window; without scale, the strip that answers `yes`
is offset and dragging grabs in the wrong place.
