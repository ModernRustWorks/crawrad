---
id: dataset-columns-probe-time-limit
kind: doc
target_file: src/dataset_columns.rs
line_hint: 16
author: 
created: 
title: "PROBE_TIME_LIMIT"
tags: []
signature: "pub const PROBE_TIME_LIMIT : Duration = Duration :: from_secs (10)"
---

How long a probe may run before it is killed.

# Examples

```rust
use std::time::Duration;
use crawrad_host::dataset_columns::PROBE_TIME_LIMIT;

assert_eq!(PROBE_TIME_LIMIT, Duration::from_secs(10));
```
