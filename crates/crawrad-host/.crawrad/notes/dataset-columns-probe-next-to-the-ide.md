---
id: dataset-columns-probe-next-to-the-ide
kind: doc
target_file: src/dataset_columns.rs
line_hint: 183
author: 
created: 
title: "probe_next_to_the_ide"
tags: []
signature: "pub fn probe_next_to_the_ide () -> Option < PathBuf >"
---

`crawrad-dbprobe` next to the running executable.

# Examples

```rust
use crawrad_host::dataset_columns::probe_next_to_the_ide;

let probe = probe_next_to_the_ide();
assert_eq!(probe.is_some(), probe.as_ref().is_some_and(|p| p.exists()));
```
