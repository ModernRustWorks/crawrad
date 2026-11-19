---
id: render-pointee
kind: doc
target_file: src/render.rs
line_hint: 190
author: 
created: 
title: "pointee"
tags: []
signature: "fn pointee (var : & Variable , ty : & str , children : & [Variable] , cx : & Cx < '_ > , depth : usize ,) -> Option < String >"
---

Formats the pointee value behind Box or reference pointer types.

`Box<T>` and `&T` come as a pointer; `{:?}` prints the pointee, and so does
this. LLDB gives the pointee two ways, both measured on 30/09/2026: a
pointer to a primitive has one child named `*boxed`; a pointer to a struct
(`&mut self` in a handler) has the struct's FIELDS as its children, with no
`*self` node in between.
