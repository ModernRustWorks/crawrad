---
id: render-field-text
kind: doc
target_file: src/render.rs
line_hint: 62
author: 
created: 
title: "field_text"
tags: []
signature: "fn field_text (var : & Variable , cx : & Cx < '_ > , depth : usize) -> String"
---

Formats a field value, falling back to adapter text with float fixes.

A field's text: presented when its shape is known, else the adapter's,
with the one fix `{:?}` makes for floats (`2` → `2.0`).
