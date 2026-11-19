---
id: designer-start-drag
kind: doc
target_file: src/lib.rs
line_hint: 344
author: 
created: 
title: "Designer::start_drag"
tags: []
signature: "fn start_drag (& mut self , doc : & Document , d : Drag) -> bool"
---

Registers the drag if the lock allows, and indicates whether it started.

It is the **only** entry point to `self.drag`: as long as it is, there is no way
for a phantom gesture to arise.
