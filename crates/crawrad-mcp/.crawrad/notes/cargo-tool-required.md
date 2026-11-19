---
id: cargo-tool-required
kind: doc
target_file: src/cargo_tool.rs
line_hint: 48
author: 
created: 
title: "required"
tags: []
signature: "pub fn required < 'a > (args : & 'a Value , tool : & str , keys : & [& str]) -> Result < Vec < & 'a str > , String >"
---

Validates and extracts required string parameters.

The required string parameters of `tool`, in order; the error names the
first one missing, as `code.read` and `code.edit` always said it.

# Examples

```rust
use crawrad_mcp::cargo_tool::required;

let args = serde_json::json!({"path": "src/main.rs"});
let keys = required(&args, "code.read", &["path"]).unwrap();
assert_eq!(keys, vec!["src/main.rs"]);
```

# Errors

Returns an error message identifying the first missing parameter.
