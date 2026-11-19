---
id: editorcommand
kind: doc
target_file: src/lib.rs
line_hint: 657
author: 
created: 
title: "EditorCommand"
tags: []
signature: "pub enum EditorCommand"
---

Commands that the code editor is requested to perform.

What the code editor is asked to do — its own family, so the IDE answers it
in the editor's module and the window's effect switch stays one line.

# Examples

```rust
use crawrad_actions::EditorCommand;

let cmd = EditorCommand::SelectAll;
assert_eq!(cmd, EditorCommand::SelectAll);
```
