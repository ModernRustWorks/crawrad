---
id: explorer-model-badge-for
kind: doc
target_file: src/explorer_model.rs
line_hint: 200
author: 
created: 
title: "badge_for"
tags: []
signature: "pub fn badge_for (owner : Dono) -> & 'static str"
---

The owner badge — the EN text that goes to the narrow column.

The three words answer the **same** question, which is the only one that matters
in front of a file: *what does the next Generate do with this?* That is why
`Rewritten` and not `Generated` — "generated" describes the past and warns of
nothing; `Rewritten` is the same word the status bar already uses in the generation
report, and says typing there is wasted effort.

Exhaustive `match`, without wildcard arm, and deliberately so: ADR 0016 concludes
by stating that a new `Owner` requires a name readable on screen. With a wildcard
this would be a promise; like this it is a **compilation error** — a new owner
cannot enter without someone choosing its word.

# Examples

```rust
use crawrad_codegen::Dono;
use crawrad_project::explorer_model::badge_for;

assert_eq!(badge_for(Dono::Gerador), "Rewritten");
```
