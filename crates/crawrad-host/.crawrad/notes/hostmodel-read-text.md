---
id: hostmodel-read-text
kind: doc
target_file: src/lib.rs
line_hint: 362
author: 
created: 
title: "HostModel::read_text"
tags: []
signature: "pub fn read_text (& self , path : & Path , key : & str) -> Result < String , (& 'static str , String) >"
---

Reads the in-memory or on-disk text for the given path.

The text an MCP read sees for `path`: its unsaved buffer (by path,
else by the canonical `key`), else the file on disk. `Err` is the
tool's (code, sentence).

# Examples

```rust
use crawrad_host::HostModel;
use std::path::Path;

let mut host = HostModel::default();
host.set_buffer("foo.rs".into(), "let x = 1;".into());
assert_eq!(host.read_text(Path::new("foo.rs"), "foo.rs").unwrap(), "let x = 1;");
```

# Errors

Returns an error if the file is neither buffered nor found on disk, or
if reading the file from disk fails.
