---
id: render-scalar
kind: doc
target_file: src/render.rs
line_hint: 387
author: 
created: 
title: "scalar"
tags: []
signature: "fn scalar (ty : & str , value : & str) -> Option < String >"
---

Formats scalar types whose adapter output differs from Rust formatting.

A value without children, where LLDB's text is not what `{:?}` prints
(measured on 30/09/2026, both toolchains): a `char` comes as
`U+007A U+007A U'z'`, and an `i8` — `char` from DWARF, `signed char` from
a PDB — as `'\xff'`.
