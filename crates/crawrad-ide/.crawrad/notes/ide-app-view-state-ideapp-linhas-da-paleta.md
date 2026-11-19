---
id: ide-app-view-state-ideapp-linhas-da-paleta
kind: doc
target_file: src/ide_app/view_state.rs
line_hint: 292
author: 
created: 
title: "IdeApp::linhas_da_paleta"
tags: []
signature: "pub (super) fn linhas_da_paleta (& self) -> Vec < palette :: Row >"
---

The Toolbox rows, from the catalog — the single source (see `paleta`).

The registry is the SAME one that `designer` already loads
(`crawrad-designer/src/lib.rs:150`): a second `Registry::builtin()` here
would be a second catalog to diverge from the first, and the catalog is
precisely what this iteration came to unify.
`filtro` is the raw text from the `Filter` field; what trims and lowercases
is `paleta::linhas`, which also hides sections that ended up with no
items.
