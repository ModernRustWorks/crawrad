---
id: workspace-is-inside
kind: doc
target_file: src/workspace.rs
line_hint: 1934
author: 
created: 
title: "is_inside"
tags: []
signature: "fn is_inside (path : & Path , outer : Option < & Path >) -> bool"
---

True when `path` is the install folder or sits under it.

`None` for the outer folder means it is not known, and then nothing is inside it —
an unknown install folder must not make every path look unwritable.
