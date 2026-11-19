---
id: targets-makepad-components-edit-actions-of
kind: doc
target_file: src/targets/makepad/components/edit.rs
line_hint: 130
author: 
created: 
title: "actions_of"
tags: []
signature: "pub fn actions_of (document : & Document , node : & Node , field : & str , hold : & str ,) -> Result < String , crate :: Erro >"
---

Emits the handle_actions block for a text box of the Edit family.

The `handle_actions` block of a text box of the `Edit` family: `hold` is
one more step that shapes `_val` after its length and its case (a
`MaskEdit`'s mask, #71), before the state takes it and `OnChange` runs.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::edit::actions_of;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let node = Node::new("ed", "Edit");
let acts = actions_of(&doc, &node, "ed", "").unwrap();
assert!(acts.contains("self.ui.text_input(id!(ed)).changed"));
```

# Errors

Returns an error if an identifier name is invalid.
