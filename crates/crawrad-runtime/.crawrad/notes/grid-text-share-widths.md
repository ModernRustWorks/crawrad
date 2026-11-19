---
id: grid-text-share-widths
kind: doc
target_file: src/grid.rs
line_hint: 146
author: 
created: 
title: "text_share_widths"
tags: []
signature: "pub fn text_share_widths < 'a > (titles : & [& str], cells : impl Fn (usize) -> Vec < & 'a str >, width : f64) -> Vec < f64 >"
---
Column widths shared by text length, as in the Designer.

# Examples

```
let w = crawrad_runtime::grid::text_share_widths(&["Name", "City"], |_| vec![], 100.0);
assert_eq!(w.len(), 2);
```
