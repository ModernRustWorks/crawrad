---
id: targets-shared-hex
kind: doc
target_file: src/targets/shared.rs
line_hint: 63
author: 
created: 
title: "hex"
tags: []
signature: "pub fn hex (cor : [f32 ; 4]) -> String"
---

The `#RRGGBB` of a role, for the `live_design!` of the Makepad target.

The alpha is left out: the five roles are opaque, and a `#RRGGBBAA` with `FF`
at the end is noise in a file that someone opens to read.

# Examples

```rust
use crawrad_codegen::targets::shared::hex;

assert_eq!(hex([1.0, 0.0, 0.0, 1.0]), "#FF0000");
```
