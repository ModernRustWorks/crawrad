---
id: targets-egui-glyph-tool-bar-of
kind: doc
target_file: src/targets/egui/glyph.rs
line_hint: 16
author: 
created: 
title: "tool_bar_of"
tags: []
signature: "pub fn tool_bar_of (doc : & Document , node : & Node) -> Result < Option < String > , Erro >"
---

The `ToolBar` field `node` sits on, when it sits on one.

# Examples

```rust
use crawrad_codegen::targets::egui::glyph::tool_bar_of;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let node = Node::new("btn", "Button");
let bar = tool_bar_of(&doc, &node).unwrap();
assert!(bar.is_none());
```

# Errors

Returns an error if the toolbar identifier name is invalid.
