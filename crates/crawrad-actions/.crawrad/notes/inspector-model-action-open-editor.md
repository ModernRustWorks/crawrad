---
id: inspector-model-action-open-editor
kind: doc
target_file: src/inspector_model.rs
line_hint: 113
author: 
created: 
title: "ACTION_OPEN_EDITOR"
tags: []
signature: "pub const ACTION_OPEN_EDITOR : & str = \"inspector.open_editor\""
---

The catalogue action that opens the window behind a property.

It lives here and not in the shell for the reason
`ide_app/explorer.rs::ACTION_TOGGLE_LEGEND` gives: an action id written in
`ide_app.rs` is a second list of commands, and the gate
`the_shell_writes_no_command_id` fails it. The ids of this panel travel with
the panel.

# Examples

```rust
use crawrad_actions::inspector_model::ACTION_OPEN_EDITOR;

assert_eq!(ACTION_OPEN_EDITOR, "inspector.open_editor");
```
