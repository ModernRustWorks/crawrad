---
id: list-crawradlistboxref-first
kind: doc
target_file: src/list.rs
line_hint: 480
author: 
created: 
title: "CrawradListBoxRef::first"
tags: []
signature: "fn first (& self , actions : & Actions , pick : impl Fn (CrawradListBoxAction) -> Option < usize > ,) -> Option < usize >"
---

Finds the first matching row action from the action stream.

A click and a double click land in the same batch, the click first:
each query looks for its own kind, not for the first action.
