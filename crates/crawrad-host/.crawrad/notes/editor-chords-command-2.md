---
id: editor-chords-command-2
kind: doc
target_file: src/editor_chords.rs
line_hint: 130
author: 
created: 
title: "command"
tags: []
signature: "pub fn command (is : & dyn Fn (& str) -> bool , classic : bool) -> Option < Command >"
---

Resolves a pressed key chord into an editor command.

The command of the key pressed in the scheme (`classic` or VS Code), or
`None` when the key belongs to the framework or to the IDE.

# Examples

```rust
use crawrad_host::editor_chords::{command, Command};

let cmd = command(&|c| c == "Ctrl+/", false);
assert_eq!(cmd, Some(Command::ToggleComment));
```
