---
id: locate-locate-from
kind: doc
target_file: src/locate.rs
line_hint: 107
author: 
created: 
title: "locate_from"
tags: []
signature: "pub fn locate_from (overridden : Option < PathBuf > , exe_dir : Option < & Path > , path_var : Option < OsString > , home : Option < PathBuf > ,) -> Result < LocatedServer , ServerMissing >"
---

Locates the server executable from explicit search paths and inputs.

[`locate`] with its inputs given: the override, the IDE's folder, the
`PATH` and the home folder.

# Examples

```rust
use crawrad_lsp::locate::locate_from;

let res = locate_from(None, None, None, None);
assert!(res.is_err());
```

# Errors

Returns [`ServerMissing`] if no candidate path exists on disk.
