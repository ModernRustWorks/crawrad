---
id: targets-makepad-components-page-control-app-state
kind: doc
target_file: src/targets/makepad/components/page_control.rs
line_hint: 555
author: 
created: 
title: "app_state"
tags: []
signature: "pub fn app_state (doc : & Document , node : & Node , field : & str) -> Result < AppState , crate :: Erro >"
---

Generates AppState for PageControl arrow key tab navigation.

#139: with the open tab holding the key focus, Left / Right open the
previous / next page shown (no wrapping), as the scroll buttons do,, through the same
`OnChanging` / `OnChange` a click goes through, and the focus follows.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::page_control::app_state;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let node = Node::new("pc", "PageControl");
let state = app_state(&doc, &node, "pc").unwrap();
assert!(state.handle.contains("PageControl"));
```

# Errors

Returns an error if an identifier name is invalid.
