---
id: note-broken-links
kind: doc
target_file: src/note.rs
line_hint: 1426
author: 
created: 
title: "broken_links"
tags: []
signature: "pub fn broken_links (root : & Path) -> Result < Vec < BrokenLink > , String >"
---

Runs cargo doc and parses broken intra-doc link diagnostics.

Runs `cargo doc --no-deps --lib --message-format=json` and parses broken intra-doc link diagnostics.

# Examples

```rust
use std::path::Path;
use crawrad_cli::note::broken_links;

let res = broken_links(Path::new("nonexistent_path"));
assert!(res.is_err());
```

# Errors

Returns an error if `cargo doc` cannot be spawned or fails unexpectedly.
