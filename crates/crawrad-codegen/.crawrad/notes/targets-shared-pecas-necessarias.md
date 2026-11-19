---
id: targets-shared-pecas-necessarias
kind: doc
target_file: src/targets/shared.rs
line_hint: 219
author: 
created: 
title: "pecas_necessarias"
tags: []
signature: "pub fn pecas_necessarias (doc : & crawrad_ir :: Document , reg : & crawrad_registry :: Registry ,) -> std :: collections :: BTreeSet < & 'static str >"
---

The pieces that this form needs, read from what the components declare.

**The pieces that this form needs**, read from what the components
DECLARE.

# Why it is not a list of class names

The old way — `usados.contains("ListBox")` — answers the wrong question:
it asks *who is on the screen* to discover *what type the code needs*.
No third-party component enters a list written here, and it is
precisely what may declare a `kind` that no native component uses.

The right question is the descriptor's: does anyone declare a property **of this
type**? A new class, native or external, comes in on its own.

Geometry ones are left out by `why_not_set`, as everywhere: they
are consumed in drawing and do not become fields.

# Examples

```rust
use crawrad_codegen::targets::shared::pecas_necessarias;
use crawrad_ir::Document;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let doc = Document::new("form", "Form");
let pecas = pecas_necessarias(&doc, &reg);
let _ = pecas;
```
