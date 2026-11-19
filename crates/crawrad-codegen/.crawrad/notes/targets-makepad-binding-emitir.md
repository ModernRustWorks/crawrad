---
id: targets-makepad-binding-emitir
kind: doc
target_file: src/targets/makepad/binding.rs
line_hint: 298
author: 
created: 
title: "emitir"
tags: []
signature: "pub fn emitir (b : & Binding , field : & str , handler : & str) -> String"
---

Emits the Rust code for an event binding.

**The TEXT of a binding.** All four forms, in one place.

# It was created for the third-party component, and is the first to use the table

[`BINDINGS`] existed as DATA and nothing was emitted from it: the sixteen
built-in `emit_actions` remain hand-written, and the fidelity gate
compares both sides. The declared component has no hand-written `emit_actions`
to write — it only has the table —, and that is why it pioneers the
path designed by ADR 0036.

The value variable is **`_val` in all four forms**. It used to be `_idx` in
`ComboBox` and `_txt` in `Edit`/`Memo`, and while nothing emitted from the
table this cost nothing; the day it did, it would be uncompilable text,
generated without anyone noticing.

# Examples

```rust
use crawrad_codegen::targets::makepad::binding::{binding_of, emitir};

let b = binding_of("Button", "on_click").unwrap();
let code = emitir(&b, "btn", "btn_click");
assert!(code.contains("self.ui.button(id!(btn)).clicked"));
```
