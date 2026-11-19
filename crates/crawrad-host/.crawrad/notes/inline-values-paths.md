---
id: inline-values-paths
kind: doc
target_file: src/inline_values.rs
line_hint: 22
author: 
created: 
title: "paths"
tags: []
signature: "pub fn paths (text : & str , stop_line : usize , locals : & [& str]) -> Vec < (usize , Vec < String >) >"
---

Collects evaluated variable paths used on lines around `stop_line`.

The paths each line around `stop_line` (zero-based) uses, in order of
appearance: `(zero-based line, paths)`.

# Examples

```rust
use crawrad_host::inline_values::paths;

let res = paths("fn main() {\n    let x = 1;\n}\n", 1, &["x"]);
assert_eq!(res.len(), 1);
```
