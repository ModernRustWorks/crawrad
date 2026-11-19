---
id: workspace-hex-to-rgba
kind: doc
target_file: src/workspace.rs
line_hint: 901
author: 
created: 
title: "hex_to_rgba"
tags: []
signature: "pub fn hex_to_rgba (hex : & str) -> Option < [f32 ; 4] >"
---

Converts a hex string (#RGB, #RRGGBB or #RRGGBBAA) to normalized RGBA [f32; 4].

# Examples

```rust
use crawrad_project::workspace::hex_to_rgba;

assert_eq!(hex_to_rgba("#ffffff"), Some([1.0, 1.0, 1.0, 1.0]));
assert_eq!(hex_to_rgba("invalid"), None);
```
