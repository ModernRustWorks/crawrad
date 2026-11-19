---
id: controller-render-log-message
kind: doc
target_file: src/controller.rs
line_hint: 424
author: 
created: 
title: "render_log_message"
tags: []
signature: "pub fn render_log_message (template : & str , mut value : impl FnMut (& str) -> String) -> String"
---

Renders a logpoint template by replacing expressions with their values.

`total = {total}` with each `{expr}` replaced by `value(expr)`. `\{` is a
literal brace, as in a modern editor; an unclosed `{` is kept as written.

# Examples

```rust
use crawrad_dap::render_log_message;

let msg = render_log_message("x = {x}", |var| format!("{var}: 42"));
assert_eq!(msg, "x = x: 42");
```
