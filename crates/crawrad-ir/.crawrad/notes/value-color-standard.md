---
id: value-color-standard
kind: doc
target_file: src/value.rs
line_hint: 538
author: 
created: 
title: "Color::STANDARD"
tags: []
signature: "pub const STANDARD : [(& 'static str , Color) ; 16] = [(\"Black\" , Color :: rgb (0x00 , 0x00 , 0x00)) , (\"Maroon\" , Color :: rgb (0x80 , 0x00 , 0x00)) , (\"Green\" , Color :: rgb (0x00 , 0x80 , 0x00)) , (\"Olive\" , Color :: rgb (0x80 , 0x80 , 0x00)) , (\"Navy\" , Color :: rgb (0x00 , 0x00 , 0x80)) , (\"Purple\" , Color :: rgb (0x80 , 0x00 , 0x80)) , (\"Teal\" , Color :: rgb (0x00 , 0x80 , 0x80)) , (\"Gray\" , Color :: rgb (0x80 , 0x80 , 0x80)) , (\"Silver\" , Color :: rgb (0xC0 , 0xC0 , 0xC0)) , (\"Red\" , Color :: rgb (0xFF , 0x00 , 0x00)) , (\"Lime\" , Color :: rgb (0x00 , 0xFF , 0x00)) , (\"Yellow\" , Color :: rgb (0xFF , 0xFF , 0x00)) , (\"Blue\" , Color :: rgb (0x00 , 0x00 , 0xFF)) , (\"Fuchsia\" , Color :: rgb (0xFF , 0x00 , 0xFF)) , (\"Aqua\" , Color :: rgb (0x00 , 0xFF , 0xFF)) , (\"White\" , Color :: rgb (0xFF , 0xFF , 0xFF)) ,]"
---

The sixteen standard colours a `ColorBox` offers.

The sixteen standard colours a `ColorBox` offers (#500), in the
classic RAD IDE's order and with its names. One list for the targets,
the designer and the preview.

# Examples

```rust
use crawrad_ir::Color;

assert_eq!(Color::STANDARD.len(), 16);
assert_eq!(Color::STANDARD[0].0, "Black");
```
