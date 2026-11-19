---
id: cargo-tool-ran
kind: doc
target_file: src/cargo_tool.rs
line_hint: 65
author: 
created: 
title: "Ran"
tags: []
signature: "pub type Ran = Result < (Value , String , bool) , String >"
---

Result of a cargo tool execution.

One cargo run as a tool answers it: the JSON, the sentence and whether it
succeeded; `Err` (the sentence) when cargo could not be started.

# Examples

```rust
use crawrad_mcp::cargo_tool::Ran;

let res: Ran = Ok((serde_json::json!({}), "done".into(), true));
assert!(res.is_ok());
```
