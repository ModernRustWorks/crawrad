---
id: render-pdb-enumeration
kind: doc
target_file: src/render.rs
line_hint: 289
author: 
created: 
title: "pdb_enumeration"
tags: []
signature: "fn pdb_enumeration (var : & Variable , children : & [Variable] , cx : & Cx < '_ > , depth : usize ,) -> Option < String >"
---

Formats a Rust enum described by PDB union and tag encodings.

A Rust `enum` as a PDB describes it (`-msvc`), measured on 30/09/2026:
`enum2$<T>` holds one `variantN` per variant, each with a `value`, plus a
`tag`. Each variant owns the tag values its constants name — `DISCR_EXACT`,
or `DISCR_BEGIN..=DISCR_END` for the one a niche encodes (`Some` of an
`Option<String>`: `0..=i64::MAX`, the capacities a `String` can have).
