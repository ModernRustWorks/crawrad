---
id: manifest-is-false
kind: doc
target_file: src/manifest.rs
line_hint: 692
author: 
created: 
title: "is_false"
tags: []
signature: "fn is_false (b : & bool) -> bool"
---

Checks whether a boolean reference is false.

THE FIELD WITH ITS DEFAULT VALUE IS NOT WRITTEN, and the reason is not saving bytes.

A manifest with thirty lines of `readonly = false` hides the only one that says
`readonly = true`. The file is what a person opens to understand what they
installed, and what `New Package` writes for them to edit: what is there must
be what was decided, not what the serializer emitted.
