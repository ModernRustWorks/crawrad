---
id: context-ir
kind: doc
target_file: src/lib.rs
line_hint: 547
author: 
created: 
title: "Context::ir"
tags: []
signature: "fn ir (& self) -> Option < & 'a crawrad_ir :: Document >"
---

The model of the open document.

Form actions pass through here and give up when there is none —
returning `Effect::none()`, which is the same thing they already returned when
the operation had nothing to do. It is not dead code: their `enabled`
already says no, but the catalogue is **also** the MCP surface
(ADR 0005), and an applier on the other side may call `execute` without
consulting the predicate.
