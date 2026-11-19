---
id: adapter-locate
kind: doc
target_file: src/adapter.rs
line_hint: 59
author: 
created: 
title: "locate"
tags: []
signature: "pub fn locate () -> Result < PathBuf , AdapterMissing >"
---

Locates the debug adapter executable used by the IDE.

The adapter this IDE uses: [`ADAPTER_ENV`] when set, else the one in
[`ADAPTER_DIR`] beside the executable.

# Examples

```rust
use crawrad_dap::adapter::locate;

match locate() {
    Ok(path) => assert!(path.is_file()),
    Err(err) => assert!(!err.to_string().is_empty()),
}
```

# Errors

Returns [`AdapterMissing`] if neither the override nor the bundled adapter exists.
