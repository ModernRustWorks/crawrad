---
id: session-turnoutcome-reply-text
kind: doc
target_file: src/session.rs
line_hint: 148
author: 
created: 
title: "TurnOutcome::reply_text"
tags: []
signature: "pub fn reply_text (self) -> String"
---

What the reply card says: the answer, or the error (and what was reset).

# Examples

```rust
use crawrad_agent::TurnOutcome;

let outcome = TurnOutcome::Error { error: "failed".to_string() };
assert!(outcome.reply_text().contains("failed"));
```
