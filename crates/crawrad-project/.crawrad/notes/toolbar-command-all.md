---
id: toolbar-command-all
kind: doc
target_file: src/toolbar.rs
line_hint: 41
author: 
created: 
title: "Command::ALL"
tags: []
signature: "pub const ALL : & 'static [Self] = & [Self :: Undo , Self :: Redo , Self :: Save , Self :: SaveAll , Self :: Build , Self :: Run , Self :: Preview , Self :: DebugPause , Self :: DebugStop , Self :: StepOver , Self :: StepInto , Self :: StepOut ,]"
---

The strip, left to right.

# Examples

```rust
use crawrad_project::toolbar::Command;

assert_eq!(Command::ALL.len(), 12);
```
