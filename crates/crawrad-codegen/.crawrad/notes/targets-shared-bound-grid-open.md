---
id: targets-shared-bound-grid-open
kind: doc
target_file: src/targets/shared.rs
line_hint: 161
author: 
created: 
title: "bound_grid_open"
tags: []
signature: "pub fn bound_grid_open (opener : & str , url : & str , table : & str , indent : & str , specs : & str , lookups_code : & str ,) -> String"
---

Expression opening a bound grid's table source and schema.

The expression that opens a bound grid's table: a
`(Arc<dyn datagrid::RowSource>, Vec<GridColumn>)`.

`opener` is the `SqlSource` constructor ([`bound_grid_target`]):
`open_sqlite_table` for a table, `open_sqlite_query` for a Dataset's `sql`.
`url` and `table` are Rust expressions of type `&str`, read from the form
when the app starts, so code that changes them before then is obeyed. `specs`
is the grid's columns ([`column_specs_literal`]). A table that does not open
becomes an `UnavailableSource`: the grid shows the reason in its footer
instead of looking like an empty table. `lookups_code` runs right after the
columns are built — it gives edLookup columns their lookup
([`grid_lookups_code`]); empty when the grid has none.

# Examples

```rust
use crawrad_codegen::targets::shared::bound_grid_open;

let expr = bound_grid_open("open_sqlite_table", "\"db.sqlite\"", "\"users\"", "    ", "&[]", "");
assert!(expr.contains("open_sqlite_table"));
assert!(expr.contains("from_schema_and_design"));
```
