---
id: chat-history-reopen
kind: doc
target_file: src/chat_history.rs
line_hint: 222
author: 
created: 
title: "reopen"
tags: []
signature: "pub fn reopen (store : & HistoryStore , id : & str , live : & mut crate :: acp_live :: LiveAgent ,) -> Option < Vec < ChatMessage > >"
---

Opens a conversation again and prepares to resume its ACP session.

Opens conversation `id` again: its messages, and — when the same agent
answered and it can load sessions — `live` resumes its ACP session on the
next warm-up. `None` when it is not stored.

# Examples

```rust
use crawrad_agent::chat_history::{HistoryStore, reopen};
use crawrad_agent::LiveAgent;
use std::path::Path;

let store = HistoryStore::at(Path::new("/tmp/history_empty"));
let mut live = LiveAgent::new();
assert!(reopen(&store, "c_non_existent", &mut live).is_none());
```
