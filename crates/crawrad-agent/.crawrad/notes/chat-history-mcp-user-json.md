---
id: chat-history-mcp-user-json
kind: doc
target_file: src/chat_history.rs
line_hint: 242
author: 
created: 
title: "mcp_user_json"
tags: []
signature: "pub fn mcp_user_json (open : Option < & str > , args : & Value) -> Value"
---

The `chat.history` answer read from the user's own history store.

`args` is the MCP call's: its optional `query` filters the summaries.

# Examples

```no_run
let found = crawrad_agent::chat_history::mcp_user_json(None, &serde_json::json!({}));
assert!(found["conversation"].is_null());
```
