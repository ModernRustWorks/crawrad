---
id: render-variant-value
kind: doc
target_file: src/render.rs
line_hint: 335
author: 
created: 
title: "variant_value"
tags: []
signature: "fn variant_value (parts : & [Variable] , cx : & Cx < '_ > , depth : usize) -> Option < String >"
---

Renders the formatted representation of an active enum variant.

`Name`, `Name(a)` or `Name { f: a }` for the active variant, whose parts
hold its payload as `value` (both encodings).
