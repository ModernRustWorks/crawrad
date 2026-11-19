---
id: semantic-next-data
kind: doc
target_file: src/semantic.rs
line_hint: 177
author: 
created: 
title: "next_data"
tags: []
signature: "pub fn next_data (answer : & Value , prev : Option < & [u32] >) -> Option < (Option < String > , Vec < u32 >) >"
---

Extracts token data from a full or full/delta response.

A `full` or `full/delta` answer as (resultId, data), given the data the
delta is against.

# Examples

```rust
use crawrad_lsp::semantic::next_data;
use serde_json::json;

let answer = json!({ "resultId": "1", "data": [1, 2, 3] });
let res = next_data(&answer, None);
assert_eq!(res, Some((Some("1".to_string()), vec![1, 2, 3])));
```
