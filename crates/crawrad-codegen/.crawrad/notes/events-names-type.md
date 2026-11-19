---
id: events-names-type
kind: doc
target_file: src/events.rs
line_hint: 426
author: 
created: 
title: "names_type"
tags: []
signature: "fn names_type (params : & str , tipo : & str) -> bool"
---

Checks whether a parameter list contains a specific type name.

Does this parameter list name `tipo` — the whole word, and not the tail of
a longer one?

`MouseEvent` is a suffix of `MouseMoveEvent`, so a plain `contains` would
drag the wrong type in and then fail to notice the right one is missing.
