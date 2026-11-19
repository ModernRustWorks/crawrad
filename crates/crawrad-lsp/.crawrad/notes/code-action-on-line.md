---
id: code-action-on-line
kind: doc
target_file: src/code_action.rs
line_hint: 207
author: 
created: 
title: "on_line"
tags: []
signature: "pub fn on_line (published : & [Value] , line : u32) -> Vec < Value >"
---

Filters published diagnostics whose range intersects `line`.

The diagnostics of `published` (a `publishDiagnostics`' list) whose range
touches `line`.

# Examples

```rust
use crawrad_lsp::code_action::on_line;
use serde_json::json;

let diags = vec![json!({
    "range": {
        "start": { "line": 2, "character": 0 },
        "end": { "line": 4, "character": 0 }
    }
})];
assert_eq!(on_line(&diags, 3).len(), 1);
assert_eq!(on_line(&diags, 5).len(), 0);
```
