---
id: targets-shared-eventos-do-declarado
kind: doc
target_file: src/targets/shared.rs
line_hint: 69
author: 
created: 
title: "EVENTOS_DO_DECLARADO"
tags: []
signature: "pub const EVENTOS_DO_DECLARADO : & [& str] = & [\"on_click\"]"
---

The events that a declared component can respond to.

**THE EVENTS THAT A DECLARED COMPONENT CAN RESPOND TO.**

Target-neutral, and that is why it lives here: what decides the set is neither egui nor
Makepad, but the nature of the declared component.

# Why it is so short, and why that is honest

A third-party component **is not a toolkit widget**: it is a drawing of
verbs over its own rectangle. There is no text box to ask
*"did it change?"*, nor a list to ask *"which item?"* — the only surface
it has is the rectangle, and the only thing the framework knows how to tell about a
rectangle is what the pointer did on it.

An additional event comes in here when BOTH targets know how to deliver it. Entering
before that would cause the defect that the `verbo_bind` gate pursues: an event that
appears in the Object Inspector, a handler that the user writes, and a call that
never happens.

**What is left out today, and is named:** events with a payload
(`on_mouse_down`, `on_mouse_up`, `on_mouse_move`) — their signature carries
a `MouseEvent`, and not even native ones construct it yet; and `on_mouse_enter`/
`on_mouse_leave`, which Makepad delivers directly (`finger_hover_in`/`out`) and
egui only delivers with edge state stored between frames.

# Examples

```rust
use crawrad_codegen::targets::shared::EVENTOS_DO_DECLARADO;

assert!(EVENTOS_DO_DECLARADO.contains(&"on_click"));
```
