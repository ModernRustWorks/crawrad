---
id: chat-history-mcp-json
kind: doc
target_file: src/chat_history.rs
line_hint: 235
author: 
created: 
title: "mcp_json"
tags: []
signature: "pub fn mcp_json (store : & HistoryStore , open : Option < & str > , query : & str) -> Value"
---

The response payload for `chat.history`.

`chat.history`'s answer: the open conversation and the list (`query` filters it).

# Examples

```rust
use crawrad_agent::chat_history::{HistoryStore, mcp_json};
use std::path::Path;

let store = HistoryStore::at(Path::new("/tmp/history_empty"));
let res = mcp_json(&store, None, "");
assert!(res.get("history").is_some());
```
