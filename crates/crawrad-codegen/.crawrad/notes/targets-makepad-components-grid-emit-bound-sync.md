---
id: targets-makepad-components-grid-emit-bound-sync
kind: doc
target_file: src/targets/makepad/components/grid.rs
line_hint: 121
author: 
created: 
title: "emit_bound_sync"
tags: []
signature: "fn emit_bound_sync (doc : & Document , node : & Node , field : & str , engine : & str) -> String"
---

Synchronizes a bound grid by opening its data table once on first sync.

The sync of a bound grid: open the table once, the first time the form is
synced, from the engine's `url` and the grid's `table_name` as the form
holds them then, showing the grid's `GridColumn`s (#936). The rows are the
widget's from there on.
