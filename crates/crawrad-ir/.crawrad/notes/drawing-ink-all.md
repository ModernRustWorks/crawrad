---
id: drawing-ink-all
kind: doc
target_file: src/drawing.rs
line_hint: 75
author: 
created: 
title: "Ink::ALL"
tags: []
signature: "pub const ALL : & 'static [Self] = & [Self :: Surface , Self :: Border , Self :: Text , Self :: TextDim , Self :: Accent ,]"
---

All of them, for a gate to scan without a second list.

# Examples

```rust
use crawrad_ir::drawing::Ink;

assert_eq!(Ink::ALL.len(), 5);
assert!(Ink::ALL.contains(&Ink::Surface));
```
