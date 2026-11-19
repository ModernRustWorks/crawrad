---
id: dialogs-all
kind: doc
target_file: src/dialogs.rs
line_hint: 171
author: 
created: 
title: "all"
tags: []
signature: "pub fn all () -> Vec < DialogSpec >"
---

Returns all supported dialog specifications.

Every dialog `view.get_dialog` reads: the three written by hand here and the
ones generated from the DSL (`dialogs_table`, `tools/gen_dialog_table.py`).

The Color dialog (#562) comes first: it opens OVER Options, and the dialog
reported is the first one shown — the one on top, where the clicks land.

# Examples

```rust
use crawrad_mcp::dialogs::all;

let specs = all();
assert!(!specs.is_empty());
```
