---
id: bookmark-effect
kind: doc
target_file: src/lib.rs
line_hint: 642
author: 
created: 
title: "bookmark_effect"
tags: []
signature: "fn bookmark_effect (c : & Context , make : fn (u8) -> EditorCommand) -> Effect"
---

THE DEBUGGER GESTURES (issue #276).

The effect of a bookmark action: its `number` (1–9) into `make`; anything
else asks nothing.
