---
id: session-chatsession-apply-turn-result
kind: doc
target_file: src/session.rs
line_hint: 262
author: 
created: 
title: "ChatSession::apply_turn_result"
tags: []
signature: "pub fn apply_turn_result (& mut self , result : & Result < String , String >) -> TurnOutcome"
---

Applies an AI turn result defaulting to Pinned session semantics.

Applies the execution result of an AI assistant query defaulting to Pinned session semantics.

# Examples

```rust
use crawrad_agent::{ChatSession, TurnOutcome};

let mut session = ChatSession::default();
let res = Ok("hello".to_string());
let outcome = session.apply_turn_result(&res);
assert!(matches!(outcome, TurnOutcome::Success { .. }));
```
