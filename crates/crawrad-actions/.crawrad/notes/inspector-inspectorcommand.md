---
id: inspector-inspectorcommand
kind: doc
target_file: src/inspector.rs
line_hint: 41
author: 
created: 
title: "InspectorCommand"
tags: []
signature: "pub enum InspectorCommand"
---

What the IDE should open — or close — inside the Object Inspector.

# Examples

```rust
use crawrad_actions::inspector::InspectorCommand;

let cmd = InspectorCommand::CloseEditor;
assert_eq!(cmd, InspectorCommand::CloseEditor);
```
