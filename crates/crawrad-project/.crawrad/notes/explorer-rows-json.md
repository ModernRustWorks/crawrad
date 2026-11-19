---
id: explorer-rows-json
kind: doc
target_file: src/explorer.rs
line_hint: 464
author: 
created: 
title: "rows_json"
tags: []
signature: "pub fn rows_json (rows : & [Row] , rects : & [(usize , [f64 ; 4])]) -> serde_json :: Value"
---

Serializes explorer rows and their bounding rects to JSON.

What `view.get_explorer` reads (#412): each row's template — which says
its colour —, icon, text, badge and depth, with its rect in window pixels
(x, y, w, h) when it was drawn.

# Examples

```rust
use crawrad_project::explorer::rows_json;

let json = rows_json(&[], &[]);
assert!(json.is_array());
```
