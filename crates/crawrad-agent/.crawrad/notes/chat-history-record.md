---
id: chat-history-record
kind: doc
target_file: src/chat_history.rs
line_hint: 201
author: 
created: 
title: "record"
tags: []
signature: "pub fn record (store : & HistoryStore , open : & mut Option < String > , agent : & str , live : & crate :: acp_live :: LiveAgent , messages : & [ChatMessage] ,) -> Result < () , String >"
---

Keeps the open conversation after a turn.

Keeps the open conversation after a turn: gives it an id the first time,
and notes who answered, its ACP session and whether that can be resumed.

# Errors

Returns an error if saving the conversation to the history store fails.

# Examples

```rust,no_run
// Writes conversation to disk history store.
use crawrad_agent::chat_history::{HistoryStore, record};
use crawrad_agent::LiveAgent;
use std::path::Path;

let store = HistoryStore::at(Path::new("/tmp/history"));
let mut open = None;
let live = LiveAgent::new();
assert!(record(&store, &mut open, "Claude", &live, &[]).is_ok());
```
