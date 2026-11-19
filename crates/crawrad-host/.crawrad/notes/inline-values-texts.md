---
id: inline-values-texts
kind: doc
target_file: src/inline_values.rs
line_hint: 93
author: 
created: 
title: "texts"
tags: []
signature: "pub fn texts (lines : & [(usize , Vec < String >)] , asked : & [String] , values : & [ShownVariable] ,) -> Vec < (usize , String) >"
---

Generates inline annotations formatted from variable values.

The text after each line, from the session's answer to [`expressions`]
(`asked[i]` evaluated to `values[i]`). A path the session could not read
is left out.

# Examples

```rust
use crawrad_host::inline_values::texts;

let annotations = texts(&[], &[], &[]);
assert!(annotations.is_empty());
```
