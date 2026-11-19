---
id: session-terminalsession
kind: doc
target_file: src/session.rs
line_hint: 68
author: 
created: 
title: "TerminalSession"
tags: []
signature: "pub struct TerminalSession"
---

An individual terminal session tab.

# Examples

```rust
use crawrad_terminal::TerminalSession;

let session = TerminalSession::new_welcome();
assert!(session.is_welcome);
```
