---
id: explorer-rows-rows-file-hint
kind: doc
target_file: src/explorer_rows/rows/file.rs
line_hint: 169
author: 
created: 
title: "hint"
tags: []
signature: "pub fn hint (state : model :: FileState , owner : Option < Dono >) -> String"
---

What the panel says about a file — the whole sentence, not the word.

It is the hover text of the egui panel, word for word: the same four
branches, the same sentences, and `model::explanation` doing the talking.
This shell has no tooltip — the reason is measured and written out in the
`selo` of `CrawradBadgeRow` — so a **click** is how it reaches anybody, and it
is the most expensive information the panel carries, since it is what
separates typing that survives from typing that is lost.

# Examples

```rust
use crawrad_project::explorer_rows::rows::file::hint;
use crawrad_project::explorer_model::FileState;

let h = hint(FileState::Stale, None);
assert!(h.contains("older form"));
```
