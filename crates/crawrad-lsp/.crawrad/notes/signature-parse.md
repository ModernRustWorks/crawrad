---
id: signature-parse
kind: doc
target_file: src/signature.rs
line_hint: 176
author: 
created: 
title: "parse"
tags: []
signature: "pub fn parse (help : & Value) -> Option < Signature >"
---

Parses the active signature from a `SignatureHelp` response.

The active signature of a `SignatureHelp` answer; `None` for `null` or no
signatures. The signature's own `activeParameter` wins over the answer's
(LSP 3.16); an index past the parameters marks none.

# Examples

```rust
use crawrad_lsp::signature::parse;
use serde_json::json;

let val = json!({
    "signatures": [{
        "label": "fn test(a: i32)",
        "parameters": [{ "label": "a: i32" }]
    }],
    "activeSignature": 0,
    "activeParameter": 0
});
let sig = parse(&val).unwrap();
assert_eq!(sig.label, "fn test(a: i32)");
assert_eq!(sig.active, Some(0));
```
