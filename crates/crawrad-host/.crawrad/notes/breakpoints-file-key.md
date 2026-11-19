---
id: breakpoints-file-key
kind: doc
target_file: src/breakpoints.rs
line_hint: 27
author: 
created: 
title: "file_key"
tags: []
signature: "pub fn file_key (path : & Path) -> PathBuf"
---

Canonicalizes a path into a uniform file key for breakpoint lookups.

One name per file: the editor, the MCP and the debugger may spell a path
differently (`/` or `\`, `..`, letter case on Windows), and a breakpoint
set through one must be found through the others. The file on disk decides;
a path that does not exist is kept as written.

# Examples

```rust
use std::path::Path;
use crawrad_host::breakpoints::file_key;

let key = file_key(Path::new("src/lib.rs"));
assert!(!key.as_os_str().is_empty());
```
