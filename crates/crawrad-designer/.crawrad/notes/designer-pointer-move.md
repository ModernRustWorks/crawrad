---
id: designer-pointer-move
kind: doc
target_file: src/lib.rs
line_hint: 710
author: 
created: 
title: "Designer::pointer_move"
tags: []
signature: "pub fn pointer_move (& mut self , doc : & Document , x : f32 , y : f32) -> Outcome"
---

Returns transaction reflecting the drag in progress.

**The IDE applies with each movement**, for drawing to follow the hand, and
therefore every transaction from here leaves stamped with the gesture number:
history merges the frames into a single step, and `Ctrl+Z` undos the
whole drag. See [`crawrad_doc::Transaction::no_gesto`].

This comment once claimed that "history groups everything on `pointer_up`"
**before that was true**. It did not group: each frame was an undo step,
and since `max_undo` is 100, two drags evicted the user's entire
history. Left on record because documentation describing a
non-existent behavior is worse than no documentation — the
latter makes the reader inspect the code.

# Examples

```rust
use crawrad_designer::Designer;
use crawrad_ir::Document;

let mut d = Designer::default();
let doc = Document::new("form", "Form");
let outcome = d.pointer_move(&doc, 10.0, 10.0);
assert!(!outcome.dragging);
```
