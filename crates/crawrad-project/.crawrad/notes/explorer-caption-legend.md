---
id: explorer-caption-legend
kind: doc
target_file: src/explorer.rs
line_hint: 102
author: 
created: 
title: "CAPTION_LEGEND"
tags: []
signature: "pub const CAPTION_LEGEND : & str = \"What the badges mean\""
---

The title of the legend SECTION.

It is a section and no longer a loose caption, and the decision is the
owner's: the legend was eating a third of a narrow panel, always. It cannot
be deleted — `Rewritten` / `Yours` / `active` is a concept of this product
and of no other IDE, so nobody arrives knowing it, unlike the `M`/`U` of
git that the reference does not explain either. Folded away, closed by
default, it leaves the screen without leaving the product.

# Examples

```rust
use crawrad_project::explorer::CAPTION_LEGEND;

assert_eq!(CAPTION_LEGEND, "What the badges mean");
```
