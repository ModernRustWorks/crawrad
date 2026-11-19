---
id: explorer-explorercommand
kind: doc
target_file: src/explorer.rs
line_hint: 32
author: 
created: 
title: "ExplorerCommand"
tags: []
signature: "pub enum ExplorerCommand"
---

What the IDE should do to the Explorer tree.

# Examples

```rust
use crawrad_actions::explorer::ExplorerCommand;

let cmd = ExplorerCommand::Refresh;
assert_eq!(cmd, ExplorerCommand::Refresh);
```
