---
id: session-turnoutcome
kind: doc
target_file: src/session.rs
line_hint: 136
author: 
created: 
title: "TurnOutcome"
tags: []
signature: "pub enum TurnOutcome"
---

Outcome of applying an AI turn execution result to the chat session.

# Examples

```rust
use crawrad_agent::TurnOutcome;

let outcome = TurnOutcome::Success { reply: "Done".to_string() };
assert_eq!(outcome.reply_text(), "Done");
```
