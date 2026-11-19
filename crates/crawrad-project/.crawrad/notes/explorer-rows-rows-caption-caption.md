---
id: explorer-rows-rows-caption-caption
kind: doc
target_file: src/explorer_rows/rows/caption.rs
line_hint: 10
author: 
created: 
title: "Caption"
tags: []
signature: "pub struct Caption"
---

A group label.

# Examples

```rust
use crawrad_project::explorer_rows::rows::caption::Caption;
use crawrad_project::explorer_rows::row::ExplorerRow;

let c = Caption::new("Forms", 0);
assert_eq!(c.text(), "Forms");
```
