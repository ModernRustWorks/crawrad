---
id: explorer-model-sources-on-disk
kind: doc
target_file: src/explorer_model.rs
line_hint: 122
author: 
created: 
title: "sources_on_disk"
tags: []
signature: "fn sources_on_disk (member : & Path) -> Vec < String >"
---

Returns the Rust source files in `src/` non-recursively.

The `*.rs` files in `src/`, non-recursive. A missing folder returns empty —
before the first generation this is normal, not an error to display.
