---
id: item-local-items
kind: doc
target_file: src/item.rs
line_hint: 382
author: 
created: 
title: "local_items"
tags: []
signature: "pub fn local_items (typed : & str , text : & str , base : Option < & str >) -> Vec < ItemSugestao >"
---

Produces local completion suggestions without language server support.

**The local list** — what the IDE offers before (or without) the
language server: after `base.`, the members the file uses on `base`
and its `fn`s that take `self`; else the file's words, Rust's keywords,
common types and the catalog's component classes. Names starting with
`typed` first; at most 24 members or 16 items.

# Examples

```rust
use crawrad_insight::item::local_items;

let items = local_items("le", "let x = 1;", None);
assert!(items.iter().any(|i| i.rotulo == "let"));
```
