---
id: guideline
kind: doc
target_file: src/lib.rs
line_hint: 107
author: 
created: 
title: "GuideLine"
tags: []
signature: "pub struct GuideLine"
---

A guide line to render, in design units.

# Examples

```rust
use crawrad_runtime::{GuideKind, GuideLine};

let guide = GuideLine {
    vertical: true,
    pos: 100.0,
    from: 0.0,
    to: 200.0,
    kind: GuideKind::Component,
    thickness: 1.0,
};
assert!(guide.vertical);
```
