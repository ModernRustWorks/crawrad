---
id: debug-panel-function-chars
kind: doc
target_file: src/debug_panel.rs
line_hint: 205
author: 
created: 
title: "FUNCTION_CHARS"
tags: []
signature: "pub const FUNCTION_CHARS : usize = 34"
---

How many characters a frame's function name displays.

How many characters a frame's function shows: its END, which names the
type and the method (`…FormMain::btn_ok_click`), not the crate path.

# Examples

```rust
use crawrad_host::debug_panel::FUNCTION_CHARS;

assert_eq!(FUNCTION_CHARS, 34);
```
