---
id: inspector-model-linked-to-action
kind: doc
target_file: src/inspector_model.rs
line_hint: 159
author: 
created: 
title: "LINKED_TO_ACTION"
tags: []
signature: "pub const LINKED_TO_ACTION : & str = \"action\""
---

Why a caption row linked to an Action shows the action's text read-only.

ONE word, like `bound`: the note is drawn at its own width and the value
box takes what is left. The caption is changed on the Action itself.

# Examples

```rust
use crawrad_actions::inspector_model::LINKED_TO_ACTION;

assert_eq!(LINKED_TO_ACTION, "action");
```
