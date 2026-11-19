---
id: magnification
kind: doc
target_file: src/lib.rs
line_hint: 757
author: 
created: 
title: "Zoom"
tags: []
signature: "pub enum Zoom"
---

THE CANVAS ZOOM — how much screen is worth one unit of the form.

A family separate from [`ViewEffect`] by the criterion that already separates the others:
`ViewEffect` says *what* is in the center of the window, and this does not move anything
from place — it changes the size at which the same thing appears. An agent reading
`Effect` to know what will happen reads the family and already knows: it does not
write to disk, does not launch a process and does not touch the document.

**It is not the `Ctrl +` / `Ctrl -` of the IDE.** That is egui's `zoom_factor`,
which enlarges the entire interface — menus, panels, inspector — and the
user likes it. This here enlarges only the designed form. They are two
different levers on different things, and that is why chords
also have to be different: see the `shortcut` of each action below.

Nothing here enters the document. It is editor state, not of the form:
`.crawform` and `.designer.rs` come out identical at any zoom
(ADR 0019).

# Examples

```rust
use crawrad_actions::Zoom;

let z = Zoom::In;
assert_eq!(z, Zoom::In);
```
