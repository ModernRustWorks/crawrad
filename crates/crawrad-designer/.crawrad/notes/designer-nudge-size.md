---
id: designer-nudge-size
kind: doc
target_file: src/lib.rs
line_hint: 994
author: 
created: 
title: "Designer::nudge_size"
tags: []
signature: "pub fn nudge_size (& self , doc : & Document , sel : & [NodeId] , dw : f32 , dh : f32 ,) -> Option < Transaction >"
---

Resizes selection via keyboard.

**Resizes selection via keyboard** — `Shift+arrow`, as in the classic IDE.

Sister of [`Designer::nudge`], and it was missing: `Shift+arrow` was the only
key of the designer that did nothing, and the Makepad shell keymap
carried the reason written — *"this function DOES NOT EXIST in the
brain"*. It exists now.

The corner that moves is the **bottom-right**, and it is that of the classic IDE: the
origin stays put and the component grows to the right and downward.
Whoever wants to grow in the other direction moves and resizes, which is what the
mouse also requires.

# The floor is ZERO, and is the same floor as the rest

Shrinking without limit would invert the rectangle, and an inverted rectangle is the
component that vanishes from the canvas without saying anything. The cutoff is the same
`non_inverted` that handle dragging uses — a single floor in the house, not
two that disagree.

**Does not check lock**, for the same reason as `nudge`: what refuses is
[`Designer::refuses`] in the applier, and refusing here would be the second copy
of the rule — and the one answering with silence instead of a sentence.

# Examples

```rust
use crawrad_designer::Designer;
use crawrad_ir::Document;

let d = Designer::default();
let doc = Document::new("form", "Form");
assert!(d.nudge_size(&doc, &[], 8.0, 0.0).is_none());
```
