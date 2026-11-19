---
id: signature-triggers-from
kind: doc
target_file: src/signature.rs
line_hint: 239
author: 
created: 
title: "triggers_from"
tags: []
signature: "pub fn triggers_from (capabilities : Option < & Value >) -> Vec < String >"
---

Extracts signature trigger characters from server capabilities.

`signatureHelpProvider.triggerCharacters` and `retriggerCharacters`, or
none: rust-analyzer lists `(`, `,` and `<`.

# Examples

```rust
use crawrad_lsp::signature::triggers_from;
use serde_json::json;

let caps = json!({
    "signatureHelpProvider": {
        "triggerCharacters": ["(", ","]
    }
});
let triggers = triggers_from(Some(&caps));
assert_eq!(triggers, vec!["(".to_string(), ",".to_string()]);
```
