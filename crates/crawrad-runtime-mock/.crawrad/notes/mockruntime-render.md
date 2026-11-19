---
id: mockruntime-render
kind: doc
target_file: src/lib.rs
line_hint: 104
author: 
created: 
title: "MockRuntime::render"
tags: []
signature: "pub fn render (& self) -> String"
---

The form drawn in text — the mock's "screen".

Each character corresponds to 8 design units. That is what enables a golden
test: if the document renders identically, the boundary did not alter anything
along the way.

# Examples

```rust
use crawrad_runtime_mock::MockRuntime;

let rt = MockRuntime::new();
let output = rt.render();
assert!(!output.is_empty());
```
