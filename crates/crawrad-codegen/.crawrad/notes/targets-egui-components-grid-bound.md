---
id: targets-egui-components-grid-bound
kind: doc
target_file: src/targets/egui/components/grid.rs
line_hint: 165
author: 
created: 
title: "bound"
tags: []
signature: "fn bound (doc : & crawrad_ir :: Document , node : & crawrad_ir :: Node , nome : & str , engine : & str , padrao : String ,) -> ResultadoComponente"
---

A grid bound to a Connection.

A grid bound to a `Connection`: the data grid, which opens the table on the
first frame from the engine's `url` and the grid's `table_name` as the
form holds them then, showing the grid's `GridColumn`s (#936). The rows live
beside the grid's own state, in `<name>_rows`, because the data grid is not
`Clone` and the form state is.
