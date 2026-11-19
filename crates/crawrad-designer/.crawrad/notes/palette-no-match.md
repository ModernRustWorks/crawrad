---
id: palette-no-match
kind: doc
target_file: src/palette.rs
line_hint: 49
author: 
created: 
title: "NO_MATCH"
tags: []
signature: "pub const NO_MATCH : & str = \"no component matches\""
---

What the Toolbox says when the filter matches no component.

It is the SAME phrase from egui's Toolbox (`crates/crawrad-ide/src/paleta.rs:277`):
both shells show the same IDE, and a list that is simply
blank does not say whether the filter did not match or the list broke.

# Examples

```rust
use crawrad_designer::palette::NO_MATCH;

assert_eq!(NO_MATCH, "no component matches");
```
