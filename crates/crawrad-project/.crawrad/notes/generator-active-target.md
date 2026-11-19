---
id: generator-active-target
kind: doc
target_file: src/generator.rs
line_hint: 96
author: 
created: 
title: "ACTIVE_TARGET"
tags: []
signature: "static ACTIVE_TARGET : AtomicU8 = AtomicU8 :: new (0)"
---

Which generator the project is built with — process-wide.

An atomic and not a field because the two readers are far apart: the
`Generate` of the bar has an IDE in hand, and project creation generates
from a worker thread that has no shell at all.
