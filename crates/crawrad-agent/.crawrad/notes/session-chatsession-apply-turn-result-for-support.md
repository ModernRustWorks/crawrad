---
id: session-chatsession-apply-turn-result-for-support
kind: doc
target_file: src/session.rs
line_hint: 267
author: 
created: 
title: "ChatSession::apply_turn_result_for_support"
tags: []
signature: "pub fn apply_turn_result_for_support (& mut self , result : & Result < String , String > , support : SessionSupport ,) -> TurnOutcome"
---

Applies an AI turn result considering session support strategy.

Applies the execution result of an AI assistant query considering the agent's session support strategy.

Pure function managing session state transitions:
- For `Captured` (e.g. `agy`):
  - Checks for silent exit 0 with `warning: conversation "..." not found` and resets session.
  - Parses structured JSON response (`{"conversation_id": "...", "status": "SUCCESS", "response": "..."}`).
  - Captures `conversation_id` exclusively from JSON structure.
- For `Pinned` (e.g. `claude`):
  - Never parses response text for conversation IDs, protecting pinned UUID from accidental JSON outputs.
- On session lost/expired: resets CLI session and returns `TurnOutcome::SessionLost`.
- On startup error during the first turn: resets CLI session so next attempt starts clean.

# Examples

```rust
use crawrad_agent::agent_profile::SessionSupport;
use crawrad_agent::{ChatSession, TurnOutcome};

let mut session = ChatSession::default();
let res = Ok("hello".to_string());
let outcome = session.apply_turn_result_for_support(&res, SessionSupport::Pinned);
assert!(matches!(outcome, TurnOutcome::Success { .. }));
```
