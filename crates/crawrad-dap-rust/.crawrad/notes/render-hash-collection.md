---
id: render-hash-collection
kind: doc
target_file: src/render.rs
line_hint: 347
author: 
created: 
title: "hash_collection"
tags: []
signature: "fn hash_collection (var : & Variable , cx : & Cx < '_ > , ty : & str) -> Option < String >"
---

Formats HashMap or HashSet entries using DWARF layout or entry count.

`HashMap`/`HashSet`: its entries, read from hashbrown's buckets with the
entry layout from the program's DWARF (ADR 0048 §3.3). Without DWARF, the
count: the adapter's fields are hashbrown's internals — hash keys, control
bytes — and printing them was measured: worse than the raw type name.
