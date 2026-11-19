---
id: pages-of
kind: doc
target_file: src/lib.rs
line_hint: 1423
author: 
created: 
title: "pages_of"
tags: []
signature: "fn pages_of (ir : & crawrad_ir :: Document , control : & str) -> usize"
---

How many PAGES the control has — and not how many children.

`active_page` is an index into the `TabSheet` list, and a `PageControl` can
carry children that are not pages. Counting `children.len()` would give the wrong
index the day it has one.
