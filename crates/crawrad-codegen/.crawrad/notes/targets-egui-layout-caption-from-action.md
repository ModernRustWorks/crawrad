---
id: targets-egui-layout-caption-from-action
kind: doc
target_file: src/targets/egui/layout.rs
line_hint: 185
author: 
created: 
title: "caption_from_action"
tags: []
signature: "pub fn caption_from_action (doc : & crawrad_ir :: Document , n : & crawrad_ir :: Node , nome : & str ,) -> Result < String , crate :: Erro >"
---

Synchronizes a control caption with its linked action.

**The caption an `Action` gives to the control linked to it** (#444, the
egui half of what `targets/makepad/action_link.rs` does): the action owns
the caption, so when it has one the control takes it before it is drawn.
Empty when the control has no action, or the action no caption.

# Examples

```rust
use crawrad_codegen::targets::egui::layout::caption_from_action;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let node = Node::new("btn", "Button");
let code = caption_from_action(&doc, &node, "btn").unwrap();
assert!(code.is_empty());
```

# Errors

Returns an error if an action identifier name is invalid.
