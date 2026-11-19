---
id: hover-contents-markdown
kind: doc
target_file: src/hover.rs
line_hint: 166
author: 
created: 
title: "contents_markdown"
tags: []
signature: "pub fn contents_markdown (hover : & Value) -> Option < String >"
---

Extracts hover markdown contents from server response JSON.

The hover's `contents` as markdown, from any of the spec's three shapes;
`None` when there is nothing to say.

# Examples

```rust
use crawrad_lsp::hover::contents_markdown;
use serde_json::json;

let res = contents_markdown(&json!({ "contents": "hello" }));
assert_eq!(res.as_deref(), Some("hello"));
```
