---
id: targets-makepad-glyph-dsl
kind: doc
target_file: src/targets/makepad/glyph.rs
line_hint: 25
author: 
created: 
title: "dsl"
tags: []
signature: "pub fn dsl (field : & str , x : f32 , y : f32 , h : f32) -> String"
---

Emits the Makepad DSL view definition for a button glyph.

The glyph's view, after the button `field` at `(x, y)`, `h` tall, in the
same parent: over its left edge, centred on its height, hidden until the
sync shows a picture.

# Examples

```rust
use crawrad_codegen::targets::makepad::glyph::dsl;

let code = dsl("btn1", 10.0, 20.0, 30.0);
assert!(code.contains("btn1_glyph = <View>"));
```
