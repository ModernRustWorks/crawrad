---
id: targets-makepad-declarado-desenho
kind: doc
target_file: src/targets/makepad/declarado.rs
line_hint: 59
author: 
created: 
title: "desenho"
tags: []
signature: "pub fn desenho (field : & str , desc : & ComponentDesc , w : f32 , h : f32) -> Option < Declarado >"
---

The declared drawing of a class, turned into a widget tree.

**The declared drawing of a class, turned into a widget tree.**

`field` is the name of the field on the form (`gau1`), and through it
`sync_ui` reaches each piece: `id!(gau1.d1)`.

`w`/`h` are the component size, because in Makepad the fraction becomes a
size in design pixels, and not a proportion.

`None` when the class declares no drawing — and then the component is non-visual,
or is a visual one that did not declare how it paints itself.

# Examples

```rust
use crawrad_codegen::targets::makepad::declarado::desenho;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let desc = reg.get("Button").unwrap();
let res = desenho("btn", desc, 100.0, 30.0);
let _ = res;
```
