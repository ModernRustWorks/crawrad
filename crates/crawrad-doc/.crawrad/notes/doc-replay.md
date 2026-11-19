---
id: doc-replay
kind: doc
target_file: src/lib.rs
line_hint: 360
author: 
created: 
title: "Doc::replay"
tags: []
signature: "fn replay (& mut self , tx : Transaction , label : String , push_to : Stack ,) -> Result < Applied , DocError >"
---

Replays a transaction from history and pushes its inverse.

Applies a transaction coming from the history and pushes the inverse to the other
side: undo feeds redo, and vice versa.
