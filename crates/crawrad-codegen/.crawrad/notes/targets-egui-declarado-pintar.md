---
id: targets-egui-declarado-pintar
kind: doc
target_file: src/targets/egui/declarado.rs
line_hint: 57
author: 
created: 
title: "pintar"
tags: []
signature: "pub fn pintar (nome : & str , desc : & ComponentDesc , ligacoes : & [(String , String)]) -> Option < Pintura >"
---

Emits egui code for the declared drawing of a node.

**The declared drawing of a node, turned into egui code.**

`nome` is the field identifier in the form (`self.gau1`), and `desc` is the
class descriptor — from which the verbs and the types of the properties
they read originate.

Returns `None` when the class declares no drawing. The caller then
falls back to what it previously did, which remains the correct answer to *"this
class did not specify how it paints"*.

`ligacoes` are the events that this NODE has bound and that the target knows how to deliver
— see [`crate::targets::makepad::binding::BINDINGS_DECLARADOS`], which is the
same table for both targets.

# Examples

```rust
use crawrad_codegen::targets::egui::declarado::pintar;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let desc = reg.get("Button").unwrap();
let pintura = pintar("btn", desc, &[]);
assert!(pintura.is_none());
```
