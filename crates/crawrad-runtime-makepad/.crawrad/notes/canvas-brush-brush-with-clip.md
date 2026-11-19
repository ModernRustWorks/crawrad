---
id: canvas-brush-brush-with-clip
kind: doc
target_file: src/canvas/brush.rs
line_hint: 523
author: 
created: 
title: "Brush::with_clip"
tags: []
signature: "pub fn with_clip (& mut self , clip : Rect , paint : impl FnOnce (& mut Self))"
---

Paints `paint` with primitives clipped to `clip`.

Paints `paint` with every primitive it emits clipped to `clip`, in
screen points.

# The law, and what it is NOT

The clip **never shrinks the rectangle being painted**. The control is
still asked for its whole rectangle — the one the document gave it —
and what the clip removes is only the *ink* that falls outside the
parent. That is the classic RAD contract the owner measured on the classic
RAD IDE: shrink the form over the components and a `TListBox` cut in half is
still a whole `TListBox` underneath, nothing snaps and nothing shrinks.
The RXP states the same law in `crawrad-runtime-egui/src/lib.rs:2330`
(`recortar_pelo_pai`) and it is what the
`encolher_o_formulario_recorta_o_componente_e_nao_o_encolhe` test
guards there.

# Why a nested turtle, and not rectangle arithmetic

Makepad clips by **vertex clamping in the shader**, not by a GPU
scissor (`makepad-draw-1.0.0/src/shader/draw_quad.rs:16-26`, and
`draw_text.rs:47` for glyphs). The clamp bounds live in the `draw_clip`
instance slot, and they are written *after the fact*, by walking the
align list at the end of the pass
(`makepad-draw-1.0.0/src/turtle.rs:762-776`): every instance emitted
between a `BeginTurtle` and its `EndTurtle` gets that turtle's clip,
already intersected with the enclosing ones
(`turtle.rs:749-758`). So writing `draw_clip` by hand would simply be
overwritten — a turtle is the only door.

Two things this buys that arithmetic could not: the SDF shapes keep
their true geometry (`pos` is recomputed after the clamp, so a rounded
corner stays a rounded corner instead of being re-cut), and **text is
clipped too**, which no rectangle intersection can do.

The turtle walks with `abs_pos`, and the canvas flows `Overlay`, so it
does not move the parent turtle (`turtle.rs:544-547`).

An empty clip paints nothing at all: `clamp` with `min > max` is
undefined in GLSL, so a container with no area is answered here, where
the answer is cheap and exact, rather than on the GPU.

# Examples

```rust
use makepad_widgets::{dvec2, Rect};
use crawrad_runtime_makepad::canvas::{Brush, Probe};
use crawrad_runtime_makepad::Transformacao;

let mut probe = Probe::default();
let mut brush = Brush::probing(&mut probe, Transformacao::new(0.0, 0.0, 1.0));
let clip = Rect { pos: dvec2(0.0, 0.0), size: dvec2(100.0, 100.0) };
brush.with_clip(clip, |_| {});
```
