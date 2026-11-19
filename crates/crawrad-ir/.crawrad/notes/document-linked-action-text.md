---
id: document-linked-action-text
kind: doc
target_file: src/lib.rs
line_hint: 575
author: 
created: 
title: "Document::linked_action_text"
tags: []
signature: "pub fn linked_action_text (& self , id : & str) -> Option < & str >"
---

The caption a component takes from its linked Action, if any.

The canvas, the running app and the Object Inspector all resolve the
caption of an action-linked control through this one function, so the
three never disagree.

# Examples

```rust
use crawrad_ir::{Document, Node, Value};

let mut doc = Document::new("MainForm", "Form");
let act = Node::new("act1", "Action").with_prop("text", Value::Str("Save".into()));
let btn = Node::new("btn1", "Button")
    .with_prop("text", Value::Str("button1".into()))
    .with_prop("action", Value::Ref("act1".into()));
doc.add_child("MainForm", act).unwrap();
doc.add_child("MainForm", btn).unwrap();
assert_eq!(doc.linked_action_text("btn1"), Some("Save"));
```
