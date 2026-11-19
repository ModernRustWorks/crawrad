---
id: generator-catalog
kind: doc
target_file: src/generator.rs
line_hint: 177
author: 
created: 
title: "catalog"
tags: []
signature: "pub (crate) fn catalog () -> & 'static crawrad_registry :: Registry"
---

The component catalogue read by every generation pass.

**The catalogue every generation reads: the IDE's**, builtin plus the
installed packages.

The designer, the palette and the Inspector already read it; `Generate`
read `Registry::builtin()` and refused the very third-party component the
designer had just dropped (*"the node 'gau1' is of class 'Gauge', which is
not installed"*), found by the end-to-end run of the Gauge (#538).
