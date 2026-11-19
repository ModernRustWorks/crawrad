---
id: targets-makepad-cursor-after-widgets
kind: doc
target_file: src/targets/makepad/cursor.rs
line_hint: 33
author: 
created: 
title: "after_widgets"
tags: []
signature: "pub fn after_widgets (reg : & crawrad_registry :: Registry , doc : & Document , emitted : & [& Node] ,) -> Result < String , Erro >"
---

Generates the code block updating mouse cursor after widgets handle events.

The block that shows the pointer, or nothing when no control and not the
form has a `Cursor` to show.

# Examples

```rust
use crawrad_codegen::targets::makepad::cursor::after_widgets;
use crawrad_ir::Document;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let doc = Document::new("form", "Form");
let code = after_widgets(&reg, &doc, &[]).unwrap();
assert!(code.is_empty());
```

# Errors

Returns an error if any node name is not a valid Rust identifier.
