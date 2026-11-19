---
id: runtime-set-mode
kind: doc
target_file: src/lib.rs
line_hint: 207
author: 
created: 
title: "Runtime::set_mode"
tags: []
signature: "fn set_mode (& mut self , mode : Mode)"
---

Sets runtime execution mode between design and live.

Design or execution. Default does nothing: a runtime that does not yet
know how to live keeps rendering, and that is honest.
