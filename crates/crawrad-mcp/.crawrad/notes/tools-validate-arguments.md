---
id: tools-validate-arguments
kind: doc
target_file: src/tools.rs
line_hint: 1096
author: 
created: 
title: "validate_arguments"
tags: []
signature: "pub fn validate_arguments (action : & crawrad_actions :: Action , args : & serde_json :: Value ,) -> Result < () , String >"
---

Validates tool call arguments against the catalogue action parameters.

Validates a tool call's arguments against the catalogue action's
parameters: required ones present, each of its declared kind, and a value
from the domain when there is one.

# Examples

```rust
use crawrad_mcp::tools::validate_arguments;
use serde_json::json;

let action = crawrad_actions::catalogue()
    .into_iter()
    .find(|a| a.id == "edit.undo")
    .expect("edit.undo is in the catalogue");
let res = validate_arguments(&action, &json!({}));
assert!(res.is_ok());
```

# Errors

Returns `Err` with a descriptive message when arguments are missing, wrong type, or invalid.
