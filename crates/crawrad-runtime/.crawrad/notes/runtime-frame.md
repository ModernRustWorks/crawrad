---
id: runtime-frame
kind: doc
target_file: src/lib.rs
line_hint: 220
author: 
created: 
title: "Runtime::frame"
tags: []
signature: "fn frame (& mut self)"
---

Notifies the runtime that the IDE just rendered a frame.

The IDE just rendered a frame. A runtime living inside the
IDE window needs this: the GPU renderer paints the entire client
area every frame, including on top of it.
