---
id: hostmodel-edited-text
kind: doc
target_file: src/lib.rs
line_hint: 397
author: 
created: 
title: "HostModel::edited_text"
tags: []
signature: "pub fn edited_text (& self , path : & Path) -> Option < String >"
---

Returns the edited text for `path` matched by canonical key.

The text on screen for `path` when it has unsaved edits, matched by
canonical key like [`Self::buffer_by_key`] (#59).

# Examples

```rust
use crawrad_host::HostModel;
use std::path::Path;

let mut host = HostModel::default();
host.set_buffer("src/lib.rs".into(), "edited".into());
assert_eq!(host.edited_text(Path::new("src/lib.rs")), Some("edited".into()));
```
