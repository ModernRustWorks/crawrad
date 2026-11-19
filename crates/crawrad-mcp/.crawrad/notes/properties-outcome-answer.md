---
id: properties-outcome-answer
kind: doc
target_file: src/properties.rs
line_hint: 43
author: 
created: 
title: "Outcome::answer"
tags: []
signature: "pub fn answer (& self , node_id : & str , sent : usize , revision : u64) -> (Value , String)"
---

The JSON data and the sentence of the answer.

# Examples

```rust
use crawrad_mcp::properties::Outcome;

let outcome = Outcome::default();
let (data, msg) = outcome.answer("btn1", 0, 1);
assert_eq!(data["id"], "btn1");
assert!(msg.contains("Updated 0 of 0 properties"));
```
