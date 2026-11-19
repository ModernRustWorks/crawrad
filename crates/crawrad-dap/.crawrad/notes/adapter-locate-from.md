---
id: adapter-locate-from
kind: doc
target_file: src/adapter.rs
line_hint: 70
author: 
created: 
title: "locate_from"
tags: []
signature: "pub fn locate_from (overridden : Option < PathBuf > , exe_dir : Option < & Path > ,) -> Result < PathBuf , AdapterMissing >"
---

Locates the adapter from an optional override and IDE directory.

[`locate`] with its inputs given: the override, and the IDE's folder.
The `PATH` is never searched.

# Examples

```rust
use crawrad_dap::adapter::locate_from;

let err = locate_from(None, None).unwrap_err();
assert!(err.looked_at.is_empty());
```

# Errors

Returns [`AdapterMissing`] if no adapter file is found in the candidate locations.
