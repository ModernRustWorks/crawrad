---
id: designer-nudge
kind: doc
target_file: src/lib.rs
line_hint: 971
author: 
created: 
title: "Designer::nudge"
tags: []
signature: "pub fn nudge (& self , doc : & Document , sel : & [NodeId] , dx : f32 , dy : f32) -> Option < Transaction >"
---

Moves selection via keyboard — designer arrows, as in the classic IDE.

**Does not check lock**, on purpose. The arrow produces `SetRect` like
any other producer, and what refuses is [`Designer::refuses`] in the
applier. A check here would be the second copy of the rule — and would have
cost the feedback: returning `None`, the locked arrow was total
silence, whereas the locked mouse explained what happened. Passing through the
gate, the same sentence serves both.

# Examples

```rust
use crawrad_designer::Designer;
use crawrad_ir::Document;

let d = Designer::default();
let doc = Document::new("form", "Form");
assert!(d.nudge(&doc, &[], 8.0, 0.0).is_none());
```
