---
id: targets-makepad-binding-emitir-com-corpo
kind: doc
target_file: src/targets/makepad/binding.rs
line_hint: 303
author: 
created: 
title: "emitir_com_corpo"
tags: []
signature: "pub fn emitir_com_corpo (b : & Binding , field : & str , body : & str) -> String"
---

Emits the event dispatch block with arbitrary inner body code.

# Examples

```rust
use crawrad_codegen::targets::makepad::binding::{binding_of, emitir_com_corpo};

let b = binding_of("Button", "on_click").unwrap();
let code = emitir_com_corpo(&b, "btn", "            do_work();\n");
assert!(code.contains("do_work()"));
```
