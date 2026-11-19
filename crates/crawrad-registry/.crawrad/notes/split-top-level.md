---
id: split-top-level
kind: doc
target_file: src/lib.rs
line_hint: 285
author: 
created: 
title: "split_top_level"
tags: []
signature: "fn split_top_level (inner : & str) -> Vec < & str >"
---

Splits parameter text at top-level commas respecting nesting.

Splits the parameter list at **top-level** commas, respecting the
nesting of `<>`, `()` and `[]`.

No event in the catalog has a comma inside the type today, and an
`inner.split(',')` passed green because of this. The first event with generic
or tuple — `fn(&mut self, e: Result<A, B>)` — would come out **split in half**, and
the error would appear in the user's project, far from here. It is a latent defect, and
the place to fix it is before it is born.

The arrow of a function type (`fn(u8) -> u8`) is the only `>` that does not close anything;
therefore it is recognized by the `-` preceding it.
