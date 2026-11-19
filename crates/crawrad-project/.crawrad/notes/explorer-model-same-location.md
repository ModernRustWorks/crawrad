---
id: explorer-model-same-location
kind: doc
target_file: src/explorer_model.rs
line_hint: 192
author: 
created: 
title: "same_location"
tags: []
signature: "pub fn same_location (a : & Path , b : & Path) -> bool"
---

Returns whether two paths point to the same location on disk.

Do two paths point to the same location on disk?

`Path::eq` is not enough, and the reason is measured, not assumed: it compares
**components**, so it already normalizes `./app` on its own — but does not normalize
`..`, and knows nothing about casing. The `members` in the manifest is human-written
text: `crates/../app` is legal, and on Windows `members = ["App"]` opens the
folder `app` without complaining about anything.

What is lost without this is not elegance: the member of the open document fails
to match the declared member, and the same project appears **twice** in the
tree — one of them without any mark, which is precisely the one the user
would click.

`canonicalize` decides on disk. When it fails — the folder does not exist,
permission was denied — it falls back to component comparison, which is the best
remaining guess and never worse than using it alone.

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer_model::same_location;

assert!(same_location(Path::new("."), Path::new(".")));
```
