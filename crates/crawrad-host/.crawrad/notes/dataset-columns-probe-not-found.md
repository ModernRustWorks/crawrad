---
id: dataset-columns-probe-not-found
kind: doc
target_file: src/dataset_columns.rs
line_hint: 19
author: 
created: 
title: "PROBE_NOT_FOUND"
tags: []
signature: "pub const PROBE_NOT_FOUND : & str = \"crawrad-dbprobe not found next to the IDE\""
---

What the IDE says when the probe is not installed next to it.

# Examples

```rust
use crawrad_host::dataset_columns::PROBE_NOT_FOUND;

assert_eq!(PROBE_NOT_FOUND, "crawrad-dbprobe not found next to the IDE");
```
