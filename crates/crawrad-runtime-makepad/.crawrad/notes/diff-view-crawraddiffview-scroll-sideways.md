---
id: diff-view-crawraddiffview-scroll-sideways
kind: doc
target_file: src/diff_view.rs
line_hint: 381
author: 
created: 
title: "CrawradDiffView::scroll_sideways"
tags: []
signature: "fn scroll_sideways (& mut self , cx : & mut Cx , event : & Event) -> bool"
---

Scrolls the side under the pointer on horizontal scroll events.

`Shift` + wheel, or a touchpad's sideways swipe, over the view
scrolls the side under the pointer. `true` when it took the event,
which then does not reach the list (it would also scroll down).
