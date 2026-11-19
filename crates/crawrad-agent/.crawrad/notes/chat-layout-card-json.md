---
id: chat-layout-card-json
kind: doc
target_file: src/chat_layout.rs
line_hint: 119
author: 
created: 
title: "card_json"
tags: []
signature: "pub fn card_json (text_rect : [f64 ; 4] , copy_rect : serde_json :: Value , selection : & str , blocks : & [BlockRect < '_ >] ,) -> serde_json :: Value"
---

Builds the JSON representation of a reply card for view inspection.

A reply card in `view.get_chat` (#427, #625): where its first text and its
Copy are, what is selected, and every block in order (`kind`, `rect`, and a
code block's `copy_rect`). Rects are `[x, y, w, h]`.

# Examples

```rust
use crawrad_agent::chat_layout::card_json;
use serde_json::Value;

let json = card_json([0.0, 0.0, 10.0, 10.0], Value::Null, "", &[]);
assert!(json.get("block_rects").is_some());
```
