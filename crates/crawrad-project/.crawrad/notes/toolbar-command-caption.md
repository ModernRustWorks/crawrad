---
id: toolbar-command-caption
kind: doc
target_file: src/toolbar.rs
line_hint: 1
author: 
created: 
title: "Command::caption"
tags: []
signature: "pub fn caption (self) -> Option < & 'static str >"
---

The word a toolbar button shows after its icon, when it has one.

Build and Run carry their names (the approved mockup); the rest are icon-only.

# Examples

```rust
use crawrad_project::toolbar::Command;

assert_eq!(Command::Build.caption(), Some("Build"));
assert_eq!(Command::Run.caption(), Some("Run"));
assert_eq!(Command::Undo.caption(), None);
```
