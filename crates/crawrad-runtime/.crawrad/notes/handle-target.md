---
id: handle-target
kind: doc
target_file: src/lib.rs
line_hint: 303
author: 
created: 
title: "HANDLE_TARGET"
tags: []
signature: "pub const HANDLE_TARGET : f32 = HANDLE_SIDE * 2.0"
---

Side of the handle click target in points.

The side of the **click** target, in points — deliberately larger than the
rendering.

A target of the exact rendering size requires pixel precision to hit a
7pt square, and hands miss. Double is what makes the handle feel like it "pulls"
the pointer. The relationship between the two was implicit in two stray numbers
in different crates — `3.5` in the adapter and `7.0` in the designer — and nobody
noted that one was half of the other on purpose.

# Examples

```rust
use crawrad_runtime::{HANDLE_SIDE, HANDLE_TARGET};

assert_eq!(HANDLE_TARGET, HANDLE_SIDE * 2.0);
```
