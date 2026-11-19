---
id: settler-settler-new
kind: doc
target_file: src/settler.rs
line_hint: 28
author: 
created: 
title: "Settler::new"
tags: []
signature: "pub (crate) fn new (name : & str , settle : Settle , work : impl FnMut (J , & Receiver < J >) -> Result < A , Option < J > > + Send + 'static ,) -> Self"
---

Spawns a worker thread that executes jobs after settling.

A thread named `name`; `work` answers one job, or gives up because a newer one arrived:
`Err(Some(newer))` runs `newer` next, `Err(None)` waits for the next request.
