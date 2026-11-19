---
id: hostmodel-probe-dataset-columns
kind: doc
target_file: src/lib.rs
line_hint: 168
author: 
created: 
title: "HostModel::probe_dataset_columns"
tags: []
signature: "pub fn probe_dataset_columns (& mut self , project_dir : Option < & Path >)"
---

Starts the column probes and the grids' row reads (#925) of the open form.

Starts the column probes of the open form's bound datasets (#788);
`project_dir` is where its relative SQLite files live.

# Examples

```rust
use crawrad_host::HostModel;

let mut host = HostModel::new();
host.probe_dataset_columns(None);
```
