---
id: actions-terminalcatalog
kind: doc
target_file: src/actions.rs
line_hint: 20
author: 
created: 
title: "TerminalCatalog"
tags: []
signature: "pub struct TerminalCatalog"
---

Default collection of actions and snippets.

# Examples

```rust
use crawrad_terminal::TerminalCatalog;

let actions = TerminalCatalog::default_actions();
assert!(!actions.is_empty());
```
