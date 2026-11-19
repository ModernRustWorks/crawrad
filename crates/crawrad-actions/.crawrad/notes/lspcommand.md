---
id: lspcommand
kind: doc
target_file: src/lib.rs
line_hint: 721
author: 
created: 
title: "LspCommand"
tags: []
signature: "pub enum LspCommand"
---

Requests sent to the language server from the status bar menu.

The language server (rust-analyzer), from the status bar's menu, as in
a modern editor. The action only asks; the shell owns the server.

# Examples

```rust
use crawrad_actions::LspCommand;

let cmd = LspCommand::Restart;
assert_eq!(cmd, LspCommand::Restart);
```
