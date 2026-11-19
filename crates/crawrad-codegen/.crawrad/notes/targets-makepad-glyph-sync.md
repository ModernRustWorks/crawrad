---
id: targets-makepad-glyph-sync
kind: doc
target_file: src/targets/makepad/glyph.rs
line_hint: 40
author: 
created: 
title: "sync"
tags: []
signature: "pub fn sync (field : & str) -> String"
---

The sync line that shows the button's glyph.

# Examples

```rust
use crawrad_codegen::targets::makepad::glyph::sync;

let code = sync("btn1");
assert!(code.contains("btn1_glyph"));
```
