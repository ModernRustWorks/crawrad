---
id: canvas-designcanvas-handle-event
kind: doc
target_file: src/canvas.rs
line_hint: 1357
author: 
created: 
title: "DesignCanvas::handle_event"
tags: []
signature: "fn handle_event (& mut self , cx : & mut Cx , event : & Event , _scope : & mut Scope)"
---

Translates interface events into runtime events.

**Translation, and nothing more.** Each `Hit` becomes a [`RuntimeEvent`] in
design units and enters the runtime queue; what decides what it
means is `crawrad-designer`, on the other side of `drain_events()`.
