---
id: targets-makepad-tab-order-tab-key-call
kind: doc
target_file: src/targets/makepad/tab_order.rs
line_hint: 18
author: 
created: 
title: "tab_key_call"
tags: []
signature: "pub fn tab_key_call (reg : & crawrad_registry :: Registry , doc : & Document , emitted : & [& Node] ,) -> Result < String , Erro >"
---

The Tab call for the `emitted` controls; empty when none takes part.

# Examples

```rust
use crawrad_codegen::targets::makepad::tab_order::tab_key_call;
use crawrad_ir::Document;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let doc = Document::new("form", "Form");
let code = tab_key_call(&reg, &doc, &[]).unwrap();
assert!(code.is_empty());
```

# Errors

Returns an error if a node name cannot be converted to a Rust identifier.
