---
id: installed-declared-drawings
kind: doc
target_file: src/installed.rs
line_hint: 130
author: 
created: 
title: "declared_drawings"
tags: []
signature: "pub fn declared_drawings () -> crawrad_ir :: drawing :: DeclaredLooks"
---

DECLARATED drawings, by class — what the shell hands to the adapter.

Empty is normal: the twenty native ones draw via code, and only a third-party
component brings verbs. It also carries each inheriting class's native base
(#934), so the canvas paints it as the base.

# Examples

```rust
use crawrad_project::installed::declared_drawings;

let _drawings = declared_drawings();
```
