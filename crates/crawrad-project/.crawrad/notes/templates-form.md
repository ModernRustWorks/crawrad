---
id: templates-form
kind: doc
target_file: src/templates.rs
line_hint: 57
author: 
created: 
title: "FORM"
tags: []
signature: "const FORM : (f32 , f32) = (640.0 , 420.0)"
---

The form size used by all templates.

Same as the initial form before them, and same as `document()` in tests:
a single number, so previews of different templates share the same aspect ratio
and the list does not look misaligned.
