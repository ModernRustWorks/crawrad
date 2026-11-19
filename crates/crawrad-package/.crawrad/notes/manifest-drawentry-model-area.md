---
id: manifest-drawentry-model-area
kind: doc
target_file: src/manifest.rs
line_hint: 341
author: 
created: 
title: "DrawEntry::model_area"
tags: []
signature: "fn model_area (& self , class : & str) -> Result < Area , ManifestError >"
---

Resolves the painting area for this drawing verb.

**WHERE this verb paints.** Absent means the entire rectangle, which is
the common case and therefore does not need to be written.
