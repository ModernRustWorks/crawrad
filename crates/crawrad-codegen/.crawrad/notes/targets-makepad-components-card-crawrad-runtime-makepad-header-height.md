---
id: targets-makepad-components-card-crawrad-runtime-makepad-header-height
kind: doc
target_file: src/targets/makepad/components/card.rs
line_hint: 128
author: 
created: 
title: "crawrad_runtime_makepad_header_height"
tags: []
signature: "fn crawrad_runtime_makepad_header_height (subtitle : & str) -> f32"
---

Returns the card header height in generated DSL.

The header's height in the generated DSL: 42 with a subtitle, 28 without —
`crawrad_runtime_makepad::card::header_height`, which the app's sync uses
(the generator does not link the runtime).
