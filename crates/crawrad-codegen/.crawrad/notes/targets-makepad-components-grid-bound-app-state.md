---
id: targets-makepad-components-grid-bound-app-state
kind: doc
target_file: src/targets/makepad/components/grid.rs
line_hint: 107
author: 
created: 
title: "bound_app_state"
tags: []
signature: "fn bound_app_state (node : & Node , field : & str) -> Option < AppState >"
---

What a bound grid keeps in the `App`: whether its table was opened.

Decided by the property alone (this hook has no document); a reference to
a deleted engine leaves an unused `#[rust]` flag, which is harmless.
