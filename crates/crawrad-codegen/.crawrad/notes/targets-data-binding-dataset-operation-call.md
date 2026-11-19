---
id: targets-data-binding-dataset-operation-call
kind: doc
target_file: src/targets/data_binding.rs
line_hint: 125
author: 
created: 
title: "dataset_operation_call"
tags: []
signature: "pub fn dataset_operation_call (operation : & str , adapter_expr : & str , indent : & str) -> String"
---

Returns the code invoking a dataset operation on the adapter (ADR 0041, #788).

# Examples

```rust
use crawrad_codegen::targets::data_binding::dataset_operation_call;

let code = dataset_operation_call("First", "adapter", "    ");
assert!(code.contains("first()"));
```
