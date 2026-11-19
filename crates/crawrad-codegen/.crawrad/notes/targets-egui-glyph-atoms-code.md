---
id: targets-egui-glyph-atoms-code
kind: doc
target_file: src/targets/egui/glyph.rs
line_hint: 25
author: 
created: 
title: "atoms_code"
tags: []
signature: "pub fn atoms_code (doc : & Document , node : & Node , nome : & str , pad : & str) -> Result < String , Erro >"
---

Prepares button atoms code including caption and optional glyph.

The lines that leave `atoms` ready for the button `nome`: its caption
(hidden on a `ToolBar` whose `ShowCaptions` is off) and its glyph at the
left. `pad` is the indentation.

# Examples

```rust
use crawrad_codegen::targets::egui::glyph::atoms_code;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let node = Node::new("btn", "Button");
let code = atoms_code(&doc, &node, "btn", "    ").unwrap();
assert!(code.contains("egui::Atoms"));
```

# Errors

Returns an error if an identifier name is invalid.
