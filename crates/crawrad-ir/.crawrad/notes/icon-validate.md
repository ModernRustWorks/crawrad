---
id: icon-validate
kind: doc
target_file: src/icon.rs
line_hint: 73
author: 
created: 
title: "validate"
tags: []
signature: "pub fn validate (steps : & [IconStep]) -> Result < () , IconError >"
---

Validates an icon drawing according to grid bounds and drawing semantics.

# Examples

```rust
use crawrad_ir::icon::{validate, IconStep};

let steps = [
    IconStep::Rect { x: 0.0, y: 0.0, w: 16.0, h: 16.0 },
    IconStep::Fill,
];
assert!(validate(&steps).is_ok());
```

# Errors

Returns [`IconError`] if `steps` is empty, does not end with `Fill` or `Stroke`,
draws a line without a preceding `MoveTo`, or contains non-finite or out-of-bounds coordinates.
