---
id: source-files
kind: doc
target_file: src/lib.rs
line_hint: 93
author: 
created: 
title: "source_files"
tags: []
signature: "fn source_files (src : & Path) -> Result < Vec < PathBuf > , String >"
---

Every `.rs` under `src/`, in a **deterministic** order.

`src/lib.rs` first, and the rest by path. The order decides the order in
which components appear in the manifest and, in turn, in the palette, so it
cannot be whatever order the file system chose to return today.
