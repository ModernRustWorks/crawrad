---
id: canvas-draw-order
kind: doc
target_file: src/canvas.rs
line_hint: 561
author: 
created: 
title: "draw_order"
tags: []
signature: "pub fn draw_order < 'a > (ordem : & 'a [NodeId] , ctls : & 'a std :: collections :: BTreeMap < NodeId , MakepadCtl > , modo : crawrad_runtime :: Mode ,) -> Vec < & 'a NodeId >"
---

The frame painting order: visuals first, then non-visuals.

In which order the frame paints: **first every visual, then every non-visual
badge**.

It is the owner's rule — *"the drawings of non-visual components in designer
mode must always stay in front of all visual components"* —, and
it is not a matter of taste: in a classic RAD IDE the non-visual is a **designer overlay**,
does not participate in the form's Z-order, and vanishes in the application. An `ActionList`
hidden behind a `Panel` is a component the user **cannot click to select**,
and that is a gesture defect.

Two things make the rule hold, and both must exist:

1. **this order**, which emits badges after everything; and
2. the badge's **own layer** ([`DrawSelo`]/[`DrawTextoSelo`]), because in
   Makepad painting order is per shader and there is no Z between instances of the
   same draw call.

One without the other does not solve it: the order alone does not separate instances of the same
shader, and the layer alone would not be deterministic — if the document's first node
were a non-visual, the badge call would originate BEFORE that of the components'
text.

Who is visual and who is not comes from the catalog ([`crate::components::e_visual`]),
never from a hand-written class list. And invisible components disappear here, in a
single place.

# In `Preview` the non-visual DOES NOT EXIST

It is the other half of the rule above, and the owner demanded it looking at the screen:

> *"in preview the non-visual components are showing up, that is WRONG, in
> egui it didn't show up"*

The badge is a **designer overlay**: it exists so the user can click an
`ActionList` and select it. In the application there is no one to select, and an
`ImageList` draws nothing — in a classic RAD IDE it disappears when running. The RXP already did
this via `no_formulario_vivo` (`crawrad-runtime-egui/src/lib.rs:1732`), and the
divergence between the two adapters was the defect.

The filter lives HERE and not in the drawing branch for the same reason as
`visible`: a single place, measurable without a window.

A free-standing function on purpose: it is measurable without a window, and that is what the test
`o_nao_visual_e_desenhado_depois_de_todo_componente_visual` requires.

# Examples

```rust
use std::collections::BTreeMap;
use crawrad_ir::NodeId;
use crawrad_runtime::Mode;
use crawrad_runtime_makepad::canvas::draw_order;
use crawrad_runtime_makepad::MakepadCtl;

let ordem: Vec<NodeId> = Vec::new();
let ctls: BTreeMap<NodeId, MakepadCtl> = BTreeMap::new();
let result = draw_order(&ordem, &ctls, Mode::Design);
assert!(result.is_empty());
```
