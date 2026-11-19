---
id: targets-shared-bound-dataset-connection
kind: doc
target_file: src/targets/shared.rs
line_hint: 148
author: 
created: 
title: "bound_dataset_connection"
tags: []
signature: "pub fn bound_dataset_connection (doc : & crawrad_ir :: Document , dataset_id : & str ,) -> Result < Option < BoundDatasetConnection > , crate :: Erro >"
---

Resolves a `Dataset` node id to its connection identifiers.

Resolves a `Dataset` node id to its own identifier and its referenced `Connection` node identifier.

Follows: `Dataset` node -> `connection` (`Value::Ref`) -> `Connection` node.
Returns `Ok(Some(BoundDatasetConnection))` if `dataset_id` refers to a `Dataset` node
whose `connection` property refers to an existing `Connection` node.
Returns `Ok(None)` if the source is not a Dataset, or the connection is missing/dangling.

# Examples

```rust
use crawrad_codegen::targets::shared::bound_dataset_connection;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
let conn = bound_dataset_connection(&doc, "missing").unwrap();
assert!(conn.is_none());
```

# Errors

Returns an error if dataset or engine node names cannot be converted to identifiers.
