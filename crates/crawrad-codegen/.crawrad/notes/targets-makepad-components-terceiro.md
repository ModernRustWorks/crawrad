---
id: targets-makepad-components-terceiro
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 313
author: 
created: 
title: "terceiro"
tags: []
signature: "pub fn terceiro < 'a > (reg : & 'a crawrad_registry :: Registry , classe : & str ,) -> Option < & 'a crawrad_registry :: ComponentDesc >"
---

Returns the class descriptor if it is a third-party component.

**DOES THE CLASS COME FROM OUTSIDE?** — its descriptor, when it does.

The question is a single one, and it is `crate_module`: it is the same field that makes the type be
written in full, that makes the shell run `cargo add`, and that decides whether the node
enters the form. Two separate calculations would diverge, and the divergence would be
a project that generates and does not compile.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::terceiro;
use crawrad_registry::Registry;

let reg = Registry::builtin();
assert!(terceiro(&reg, "Button").is_none());
```
