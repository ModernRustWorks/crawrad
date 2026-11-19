---
id: templates-templatepiece
kind: doc
target_file: src/templates.rs
line_hint: 34
author: 
created: 
title: "TemplatePiece"
tags: []
signature: "pub struct TemplatePiece"
---

A piece of the template: a catalogue class, with name, location and text.

# Why text is explicit, and not inherited

`instantiate` makes a text component born **with its own name written in it** —
it is the classic RAD IDE's `Button1`, `Edit1`, `Label1`, and is right for someone who just
dragged from the palette: it helps find the component before it has real text.

In a template that looks wrong, and the owner saw it on screen: the preview showed
`lblName`, `chkActive`, `btnSave` — variable names in the user's face.
A template is a **ready** starting point; if it arrives asking to rename
everything, it saved no work at all.

So `name` remains the id — the `btnOk` that appears in the Inspector and
generated code —, and the text is declared separately. `None` lets the
catalogue default apply, which is right for components with no text.

# Examples

```rust
use crawrad_ir::Rect;
use crawrad_project::templates::TemplatePiece;

let piece = TemplatePiece {
    class: "Button",
    name: "btnOk",
    text: Some("OK"),
    rect: Rect::new(0.0, 0.0, 80.0, 30.0),
};
assert_eq!(piece.name, "btnOk");
```
