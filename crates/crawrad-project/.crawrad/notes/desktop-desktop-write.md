---
id: desktop-desktop-write
kind: doc
target_file: src/desktop.rs
line_hint: 147
author: 
created: 
title: "Desktop::write"
tags: []
signature: "pub fn write (& self , root : & Path) -> std :: io :: Result < () >"
---

Writes it into the project, making `.crawrad` when it is missing.

# Examples

```rust,no_run
// Does not write files to disk during doctests.
use std::path::Path;
use crawrad_project::desktop::Desktop;

let d = Desktop::default();
assert!(d.write(Path::new(".")).is_ok());
```

# Errors

Returns `std::io::Error` if creating `.crawrad` or writing `desktop.txt` fails.
