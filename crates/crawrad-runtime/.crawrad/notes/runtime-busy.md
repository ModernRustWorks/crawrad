---
id: runtime-busy
kind: doc
target_file: src/lib.rs
line_hint: 215
author: 
created: 
title: "Runtime::busy"
tags: []
signature: "fn busy (& self) -> bool"
---

Checks whether there is an ongoing user gesture.

Is there an ongoing gesture? The IDE uses this to decide if it needs to keep
rendering frames or if it can sleep.
