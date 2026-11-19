---
id: ide-app-constants-recuo-sub
kind: doc
target_file: src/ide_app/constants.rs
line_hint: 26
author: 
created: 
title: "RECUO_SUB"
tags: []
signature: "pub (crate) const RECUO_SUB : f64 = 14.0"
---

How much an Inspector sub-row name indents per level.

In Rust and not in the DSL because what multiplies by level is drawing: the
`padding` of a template is a fixed number, and depth belongs to the data.
