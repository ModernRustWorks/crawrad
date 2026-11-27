---
id: server-says-read
kind: doc
target_file: src/server_says.rs
line_hint: 104
author:
created:
title: "read"
tags: []
signature: "pub fn read (method : & str , params : & serde_json :: Value) -> Option < Health >"
---

Turns one notification into a verdict, or `None`.

Returns `None` when it says nothing about health. Kept free of the lock so it
can be read and tested on its own.

# Examples

```rust
use crawrad_lsp::server_says::{read, Health};
use serde_json::json;

let said = read(
    "window/showMessage",
    &json!({ "type": 1, "message": "cargo metadata failed" }),
);
assert!(matches!(said, Some(Health::ProjectFailed(_))));

// A plain information message is not a health verdict.
assert!(read("window/showMessage", &json!({ "type": 3, "message": "hi" })).is_none());
```
