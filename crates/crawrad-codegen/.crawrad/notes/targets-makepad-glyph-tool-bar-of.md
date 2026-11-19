---
id: targets-makepad-glyph-tool-bar-of
kind: doc
target_file: src/targets/makepad/glyph.rs
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
use crawrad_codegen::targets::makepad::glyph::tool_bar_of;
use crawrad_ir::{Document, Node};

let doc = Document::new("form", "Form");
let node = Node::new("btn", "Button");
let tb = tool_bar_of(&doc, &node).unwrap();
assert!(tb.is_none());
```

# Errors

Returns an error if the parent toolbar's name is not a valid identifier.
