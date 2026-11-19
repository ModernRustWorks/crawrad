---
id: manifest-limitentry
kind: doc
target_file: src/manifest.rs
line_hint: 264
author: 
created: 
title: "LimitEntry"
tags: []
signature: "pub enum LimitEntry"
---

**AN ENDPOINT OF THE RANGE, in the file format** — number or property
name.

# The grammar is the same as the rest of the manifest: bare name, or value

The `text` of a verb already chooses between `prop = "text"` and `text = "Hello"` by
different fields; here the choice fits in the SAME field because the two types
are not confused — `0.0` is a number, `"min"` is text, and TOML already separates them.
Two fields (`min` and `min_prop`) would be two keys for the same question, and
the format would gain a wrong way of writing.

# Examples

```rust
use crawrad_package::manifest::LimitEntry;

let limit = LimitEntry::Number(0.0);
assert!(matches!(limit, LimitEntry::Number(_)));
```
