---
id: manifest-tag
kind: doc
target_file: src/manifest.rs
line_hint: 975
author: 
created: 
title: "TAG"
tags: []
signature: "const TAG : & str = \"t\""
---

The key that marks the LONG form — the `tag` of `crawrad_ir::Value`.

A constant because it is the entire disambiguation rule, and it appears
on both sides of the conversion. Written twice, it would be the divergence that
would cause one value to be read as the other.
