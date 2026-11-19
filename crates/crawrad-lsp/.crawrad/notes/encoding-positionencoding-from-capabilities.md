---
id: encoding-positionencoding-from-capabilities
kind: doc
target_file: src/encoding.rs
line_hint: 30
author: 
created: 
title: "PositionEncoding::from_capabilities"
tags: []
signature: "pub fn from_capabilities (capabilities : Option < & Value >) -> Self"
---

Determines the position encoding selected from server capabilities.

The server's choice from its `capabilities`; absent or unknown means
UTF-16 (the spec's default).

# Examples

```rust
use crawrad_lsp::encoding::PositionEncoding;
use serde_json::json;

let caps = json!({ "positionEncoding": "utf-8" });
assert_eq!(PositionEncoding::from_capabilities(Some(&caps)), PositionEncoding::Utf8);
```
