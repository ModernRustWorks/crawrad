---
id: debug-debug-select-frame
kind: doc
target_file: src/debug.rs
line_hint: 214
author: 
created: 
title: "Debug::select_frame"
tags: []
signature: "pub fn select_frame (& mut self , index : usize) -> Option < (PathBuf , u32) >"
---

Selects stack frame `index` and returns where it stands.

Picks frame `index` of the stack, as a click on it does, and returns
where it stands so the editor can show it. The top frame's locals came
with the stop; any other frame's are asked of the session.

# Examples

```rust
use crawrad_host::Debug;

let mut debug = Debug::default();
assert_eq!(debug.select_frame(0), None);
```
