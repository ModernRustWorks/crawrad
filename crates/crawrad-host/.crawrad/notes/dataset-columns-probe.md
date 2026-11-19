---
id: dataset-columns-probe
kind: doc
target_file: src/dataset_columns.rs
line_hint: 192
author: 
created: 
title: "probe"
tags: []
signature: "pub fn probe (exe : & Path , url : & str , table : & str) -> Answer"
---

Runs the probe, killing it past [`PROBE_TIME_LIMIT`].

# Examples

```rust,no_run
// Spawns an external dbprobe process.
use std::path::Path;
use crawrad_host::dataset_columns::probe;

let res = probe(Path::new("crawrad-dbprobe"), "sqlite://db.sqlite", "users");
assert!(res.is_err());
```

# Errors

Returns an error if spawning `exe` fails, if the probe times out, or if output parsing fails.
