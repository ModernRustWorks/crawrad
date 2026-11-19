---
id: render-enumeration
kind: doc
target_file: src/render.rs
line_hint: 217
author: 
created: 
title: "enumeration"
tags: []
signature: "fn enumeration (ty : & str , variants : & Variable , cx : & Cx < '_ > , depth : usize) -> Option < String >"
---

Formats a Rust enum described by DWARF variant encodings.

A Rust `enum` as DWARF describes it (`-gnu`): `$variants$` holds one
`$variant$N` per variant, each with a `$discr$` and a `value`. The active
one is the variant whose `N` equals its `$discr$`; when none does, it is the
variant written without a number — the one that carries the data while a
niche encodes the others (`Option<String>`: `None` is a null pointer).
