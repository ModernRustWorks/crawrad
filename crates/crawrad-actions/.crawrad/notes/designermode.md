---
id: designermode
kind: doc
target_file: src/lib.rs
line_hint: 772
author: 
created: 
title: "DesignerMode"
tags: []
signature: "pub enum DesignerMode"
---

A mode of the DESIGNER — how it responds to gestures, not what it shows.

A family separate from [`ViewEffect`] by the same criterion that separated `ViewEffect` from
[`FileEffect`]: `ViewEffect` says *what is in the center of the window*, and this does not
change anything from place — it changes how mouse is handled. An agent looking at
`Effect` to know what will happen reads the family and already knows: this does not
write to disk, does not launch a process and does not move anyone's eyes.

Nothing here enters the document. It is editor state, not of the form:
`.crawform` and `.designer.rs` come out identical with the mode on or
off, and therefore ADR 0002
has no extra line because of this.

# Examples

```rust
use crawrad_actions::DesignerMode;

let mode = DesignerMode::ToggleLock;
assert_eq!(mode, DesignerMode::ToggleLock);
```
