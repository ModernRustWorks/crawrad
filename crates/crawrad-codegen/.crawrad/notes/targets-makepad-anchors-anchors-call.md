---
id: targets-makepad-anchors-anchors-call
kind: doc
target_file: src/targets/makepad/anchors.rs
line_hint: 19
author: 
created: 
title: "anchors_call"
tags: []
signature: "pub fn anchors_call (reg : & crawrad_registry :: Registry , doc : & Document , emitted : & [& Node] ,) -> Result < String , Erro >"
---

The anchors call for the emitted controls.

The anchors call for the `emitted` controls; empty when none follows the
window.

# Examples

```rust
use crawrad_codegen::targets::makepad::anchors::anchors_call;
use crawrad_ir::Document;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let doc = Document::new("MainForm", "Form");
let call = anchors_call(&reg, &doc, &[]).unwrap();
assert!(call.is_empty());
```

# Errors

Returns an error if an identifier name is invalid.
