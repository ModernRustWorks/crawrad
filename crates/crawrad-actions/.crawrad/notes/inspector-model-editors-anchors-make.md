---
id: inspector-model-editors-anchors-make
kind: doc
target_file: src/inspector_model/editors/anchors.rs
line_hint: 158
author: 
created: 
title: "make"
tags: []
signature: "fn make (s : & Subject < '_ >) -> Option < Editor >"
---

Decides whether to build the Anchors editor.

**The label decides, on its own.**

This used to be `(PropKind::Edges, Value::Map(m))` — the SAME label as the
number editor, broken as a tie by the shape of the value. It worked because it
was us on both sides; a third party writing `kind = "edges"` in a `manifest.json`
would have no way of knowing which of the two editors they would get.

With `PropKind::Set`, the component's label speaks for itself (ADR 0037), and
`Value::Map` remains in the pattern match for a different reason: a `Set` that
arrived with another value shape is a faulty manifest, and the Inspector rejects
it instead of drawing just anything.
