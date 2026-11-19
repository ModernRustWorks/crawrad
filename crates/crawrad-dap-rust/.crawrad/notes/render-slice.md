---
id: render-slice
kind: doc
target_file: src/render.rs
line_hint: 151
author: 
created: 
title: "slice"
tags: []
signature: "fn slice (var : & Variable , cx : & Cx < '_ > , elem : & str) -> Option < String >"
---

Formats elements of a slice from data_ptr and length children.

A slice reference: the same elements as a `Vec`, found through the
reference's `data_ptr` and `length` children.
