---
id: actions-terminalsnippet
kind: doc
target_file: src/actions.rs
line_hint: 11
author: 
created: 
title: "TerminalSnippet"
tags: []
signature: "pub struct TerminalSnippet"
---

A reusable snippet for insertion or clipboard copy.

# Examples

```rust
use crawrad_terminal::TerminalSnippet;

let snippet = TerminalSnippet {
    id: "status".into(),
    title: "Status".into(),
    snippet: "git status".into(),
    category: "Git".into(),
};
assert_eq!(snippet.id, "status");
```
