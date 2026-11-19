---
id: client-completion-triggers-from
kind: doc
target_file: src/client.rs
line_hint: 106
author: 
created: 
title: "completion_triggers_from"
tags: []
signature: "pub fn completion_triggers_from (capabilities : Option < & serde_json :: Value >) -> Vec < String >"
---

`capabilities.completionProvider.triggerCharacters`, or none.

# Examples

```rust
use crawrad_lsp::client::completion_triggers_from;
use serde_json::json;

let caps = json!({
    "completionProvider": {
        "triggerCharacters": [".", ":"]
    }
});
assert_eq!(completion_triggers_from(Some(&caps)), vec![".", ":"]);
```
