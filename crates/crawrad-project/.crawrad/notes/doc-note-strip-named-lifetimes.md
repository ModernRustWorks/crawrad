---
id: doc-note-strip-named-lifetimes
kind: doc
target_file: src/doc_note.rs
line_hint: 1373
author: 
created: 
title: "strip_named_lifetimes"
tags: []
signature: "fn strip_named_lifetimes (ty : & str) -> String"
---

`&'a T` → `&T`, `Foo<'a, T>` → `Foo<T>`, `Foo<'a>` → `Foo`.
