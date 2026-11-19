---
id: layout-normalize-pages
kind: doc
target_file: src/layout.rs
line_hint: 120
author: 
created: 
title: "normalize_pages"
tags: []
signature: "fn normalize_pages (pass : & mut Pass , id : & str)"
---

The page fills the control, below the tabs — and does not move.

This is the **derived geometry** of the amendment to ADR 0012: in the classic component library `TTabSheet` is
`alClient` and the user does not drag it. Instead of teaching `crawrad-ir` to
calculate client indent per class — which would break its blindness, which
is precisely what makes the model valid for containers that do not yet
exist —, the rectangle is real and a command maintains it.

Runs alongside `Align` because it has the same nature: geometry that the container
imposes on the child, and not a choice of who drew it. And runs **interleaved** with
it, and not before everything: a `PageControl` that is `alClient` needs its
own rectangle already decided to know where the page begins.
