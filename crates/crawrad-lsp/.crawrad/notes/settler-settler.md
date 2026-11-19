---
id: settler-settler
kind: doc
target_file: src/settler.rs
line_hint: 20
author: 
created: 
title: "Settler"
tags: []
signature: "pub (crate) struct Settler < J , A >"
---

Executes debounced jobs sequentially on a dedicated worker thread.

One thread; `work` gets the job by value and the job receiver, must check
for a newer job only through `jobs.try_iter().last()` (via the client's superseded
callback) and return `Err(Some(newer))` with it so it runs next; `Err(None)` answers nothing.
