---
id: view-tool-drive
kind: doc
target_file: src/view_tool.rs
line_hint: 13
author: 
created: 
title: "Drive"
tags: []
signature: "pub enum Drive"
---

Operation requested by a drive tool to be executed by the shell.

What a drive tool asks the shell to do once its arguments check out: the
shell runs it through the same seam the UI uses, then answers with [`done`].

# Examples

```rust
use crawrad_mcp::view_tool::Drive;

let d = Drive::RunAction("edit.undo".to_string());
assert!(matches!(d, Drive::RunAction(_)));
```
