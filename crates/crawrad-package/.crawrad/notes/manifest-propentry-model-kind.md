---
id: manifest-propentry-model-kind
kind: doc
target_file: src/manifest.rs
line_hint: 1103
author: 
created: 
title: "PropEntry::model_kind"
tags: []
signature: "fn model_kind (& self) -> ExternalKind"
---

Builds the model kind using payload from neighboring fields.

**The model type, assembled with the payload from neighboring fields.**

See the doc of [`Kind`] for why the payload does not live inside the name in
the file. Absent limits become those of the type, which is what
`PropDesc::new` of the catalogue uses when nobody says anything.
