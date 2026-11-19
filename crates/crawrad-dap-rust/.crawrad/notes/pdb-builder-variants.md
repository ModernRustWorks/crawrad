---
id: pdb-builder-variants
kind: doc
target_file: src/pdb.rs
line_hint: 272
author: 
created: 
title: "Builder::variants"
tags: []
signature: "fn variants (& self , members : & [Member]) -> Option < Variants >"
---

Extracts enum variant arms and discriminant tag from union members.

The variants of an `enum2$<T>` union: its `tag` and, for each
`variantN`, the payload `value` and the tag values it owns.
