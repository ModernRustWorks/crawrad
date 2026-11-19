---
id: acp-live-turnresult
kind: doc
target_file: src/acp_live.rs
line_hint: 325
author: 
created: 
title: "TurnResult"
tags: []
signature: "pub type TurnResult = Result < (String , Vec < ToolCallRecord > , Via , StatusLine) , String >"
---

A finished turn: the reply, its tools, how it came, and its final status line.

# Examples

```rust
use crawrad_agent::acp_live::TurnResult;

let res: TurnResult = Err("cancelled".to_string());
assert!(res.is_err());
```
