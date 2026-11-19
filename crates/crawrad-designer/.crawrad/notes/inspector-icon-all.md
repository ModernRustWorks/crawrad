---
id: inspector-icon-all
kind: doc
target_file: src/inspector_icon.rs
line_hint: 37
author: 
created: 
title: "ALL"
tags: []
signature: "pub const ALL : & [RowIcon] = & [FOLD_CLOSED , FOLD_OPEN , OPEN_EDITOR , CLEAR]"
---

Every drawing this panel uses, for the oracle that reads the font.

`#[cfg(test)]` for the same reason as `explorer::sample_rows`: outside testing
nobody reads it.

Derived from nothing and written by hand, and that is the one hand-written
list here on purpose: it is what the test iterates, and a list built from
the constants would be the constants proving themselves.

# Examples

```rust
use crawrad_designer::inspector_icon::ALL;

assert_eq!(ALL.len(), 4);
```
