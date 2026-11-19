---
id: canvas-step
kind: doc
target_file: src/canvas.rs
line_hint: 585
author: 
created: 
title: "Step"
tags: []
signature: "pub enum Step < 'a >"
---

One emission of a frame.

A control is not always a single step: a container that paints over its own
children — a `ScrollBox` and its bars — is emitted **twice**, and what goes
between the two is its whole subtree.

# Examples

```rust
use crawrad_ir::NodeId;
use crawrad_runtime_makepad::canvas::Step;

let id = NodeId::from("btn");
let step = Step::Paint(&id);
assert_eq!(step, Step::Paint(&id));
```
