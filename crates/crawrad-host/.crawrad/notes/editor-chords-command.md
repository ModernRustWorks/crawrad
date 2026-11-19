---
id: editor-chords-command
kind: doc
target_file: src/editor_chords.rs
line_hint: 41
author: 
created: 
title: "Command"
tags: []
signature: "pub enum Command"
---

What a chord asks the editor to do.

# Examples

```rust
use crawrad_host::editor_chords::Command;

let cmd = Command::DeleteLine;
assert_eq!(cmd, Command::DeleteLine);
```
