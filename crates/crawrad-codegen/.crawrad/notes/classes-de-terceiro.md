---
id: classes-de-terceiro
kind: doc
target_file: src/lib.rs
line_hint: 913
author: 
created: 
title: "classes_de_terceiro"
tags: []
signature: "pub fn classes_de_terceiro < 'a > (doc : & crawrad_ir :: Document , reg : & 'a crawrad_registry :: Registry ,) -> Vec < & 'a str >"
---

Returns third-party component classes used in the form.

**THE THIRD-PARTY CLASSES THAT THIS FORM USES.**

In order, without duplicates. Empty for a form with only built-in components, which is the
common case.

# Examples

```rust
use crawrad_codegen::classes_de_terceiro;
use crawrad_ir::Document;
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let reg = Registry::builtin();
let classes = classes_de_terceiro(&doc, &reg);
assert!(classes.is_empty());
```

# Why it exists

The code comes out right and the project **does not compile**: the type
`gauge_crawrad::Gauge` only resolves if the project's `Cargo.toml` depends on the
crate. What adds the dependency is the shell, using `cargo add`
(ADR 0018
— the IDE talks to cargo instead of reimplementing it), and this is the question
it asks beforehand.

# Why it lives HERE

Because it is the same computation that decides what goes into the code: a class needs
a dependency exactly when it has `crate_module`, which is the same field that
causes `emitir_de_terceiro` to write out the full type. Two separate computations
would diverge, and the divergence would result in a project that generates and does not compile.
