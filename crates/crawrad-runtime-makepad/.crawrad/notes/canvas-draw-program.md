---
id: canvas-draw-program
kind: doc
target_file: src/canvas.rs
line_hint: 596
author: 
created: 
title: "draw_program"
tags: []
signature: "pub fn draw_program < 'a > (ordem : & [& 'a NodeId] , ctls : & std :: collections :: BTreeMap < NodeId , MakepadCtl > ,) -> Vec < Step < 'a > >"
---

The steps of one frame, in the order the canvas emits them.

The steps of one frame, in the order the canvas emits them — **the issue
#131**.

# What the owner saw

> *"The `ScrollBox` clipped the components inside it well, but there is
> a detail: the components are overlaying the scroll bar. In a classic RAD IDE the
> scroll bar still stays on top."*

A container painted itself whole — surface **and** bars — in a single step,
and every child drawn afterwards passed over the bars. The cure is not to
carve a client area out of the model (Makepad overlays the bar rather than
reserving a trough, `makepad-widgets-1.0.0/src/scroll_bars.rs:245-250`): it
is to emit the container's chrome **after its subtree**.

# Why after the SUBTREE, and not at the end of everything

Because the bar belongs to that container, not to the form. Flushing every
overlay at the end of the frame would put the `ScrollBox` bar over a
**sibling** that legitimately covers it — in a classic RAD IDE a sibling window above
it in Z order does cover its scroll bar. Closing right after the subtree is
the only reading that is right in both directions.

The walk is depth-first and a subtree is contiguous
(`crawrad-ir/src/lib.rs:368-383`, `em_ordem_de_desenho`), so a stack of open
containers is enough: an id that is no longer a descendant of the top closes
it.

Pure of purpose: the order of a frame is the thing that broke, and it is
measurable with no window.

# Examples

```rust
use std::collections::BTreeMap;
use crawrad_ir::NodeId;
use crawrad_runtime_makepad::canvas::draw_program;
use crawrad_runtime_makepad::MakepadCtl;

let ordem: Vec<&NodeId> = Vec::new();
let ctls: BTreeMap<NodeId, MakepadCtl> = BTreeMap::new();
let steps = draw_program(&ordem, &ctls);
assert!(steps.is_empty());
```
