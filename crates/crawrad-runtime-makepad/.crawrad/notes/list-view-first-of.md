---
id: list-view-first-of
kind: doc
target_file: src/list_view.rs
line_hint: 568
author: 
created: 
title: "first_of"
tags: []
signature: "fn first_of (actions : impl Iterator < Item = CrawradListViewAction > , pick : impl Fn (CrawradListViewAction) -> Option < usize > ,) -> Option < usize >"
---

Finds the first matching action from the list view action iterator.

A click and a double click land in the same batch, and the click comes
first: each query looks for its own kind, not the first action.
