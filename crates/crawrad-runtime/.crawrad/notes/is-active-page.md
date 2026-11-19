---
id: is-active-page
kind: doc
target_file: src/lib.rs
line_hint: 266
author: 
created: 
title: "is_active_page"
tags: []
signature: "fn is_active_page (doc : & Document , page : & str) -> bool"
---

Returns whether this `TabSheet` is the open page of its `PageControl`.

Is this `TabSheet` the open page of its `PageControl`?

A page without a parent `PageControl` is an odd document, and the honest answer there
is *yes*: better for it to appear and make the problem visible than disappear and have
the user look for an invisible node.
