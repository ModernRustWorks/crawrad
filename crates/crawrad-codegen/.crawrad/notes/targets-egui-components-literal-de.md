---
id: targets-egui-components-literal-de
kind: doc
target_file: src/targets/egui/components/mod.rs
line_hint: 278
author: 
created: 
title: "literal_de"
tags: []
signature: "fn literal_de (v : & crawrad_ir :: Value) -> Option < String >"
---

The Rust literal of a value, or `None` for types not yet supported.

The Rust literal of a value, or `None` for what the target does not yet know
how to write — and what it does not know how to write is **skipped**, not guessed.
