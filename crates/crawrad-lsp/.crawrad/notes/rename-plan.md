---
id: rename-plan
kind: doc
target_file: src/rename.rs
line_hint: 164
author: 
created: 
title: "plan"
tags: []
signature: "pub fn plan (edits : & WorkspaceEdit , enc : crate :: encoding :: PositionEncoding , read : & mut dyn FnMut (& Path) -> Option < String > ,) -> Result < Vec < Planned > , PathBuf >"
---

Plans edits across all affected files in path order.

Each file of `edits` with its edits applied to its text — the text the
editor holds, from `read` — in path order. `Err` names the first file
that cannot be read: a rename applies to every file or to none.

# Errors

Returns `Err` with the path of the first file that cannot be read or
converted from a URI.

# Examples

```rust
use crawrad_lsp::edit::WorkspaceEdit;
use crawrad_lsp::encoding::PositionEncoding;
use crawrad_lsp::rename::plan;
use std::collections::BTreeMap;
use std::path::Path;

let edits: WorkspaceEdit = BTreeMap::new();
let mut read = |_: &Path| None;
let res = plan(&edits, PositionEncoding::Utf8, &mut read);
assert_eq!(res, Ok(Vec::new()));
```
