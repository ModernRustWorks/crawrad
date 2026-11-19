---
id: label-align-of
kind: doc
target_file: src/label.rs
line_hint: 13
author: 
created: 
title: "align_of"
tags: []
signature: "pub fn align_of (alignment : & str , layout : & str) -> (f64 , f64)"
---

Translates alignment and layout names into Makepad align fractions.

`Alignment` (`taLeftJustify` / `taCenter` / `taRightJustify`) and `Layout`
(`tlTop` / `tlCenter` / `tlBottom`) as makepad's `align` fractions.

# Examples

```rust
use crawrad_runtime_makepad::label::align_of;

assert_eq!(align_of("taCenter", "tlCenter"), (0.5, 0.5));
assert_eq!(align_of("taLeftJustify", "tlTop"), (0.0, 0.0));
```
