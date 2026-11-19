---
id: targets-makepad-menus-app-state
kind: doc
target_file: src/targets/makepad/menus.rs
line_hint: 222
author: 
created: 
title: "app_state"
tags: []
signature: "pub fn app_state (doc : & Document , emitted : & [& Node]) -> Result < Option < AppState > , Erro >"
---

Emits `App` event handling code closing menu lists on outside clicks.

What the menus add to the `App`: a press outside the bar and the open
lists closes them. It runs before the controls' events, so a secondary
click that opens a `PopupMenu` is not closed by its own press.

# Examples

```rust
use crawrad_codegen::targets::makepad::menus::app_state;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
let state = app_state(&doc, &[]).unwrap();
assert!(state.is_none());
```

# Errors

Returns an error if menu component names cannot be converted to identifiers.
