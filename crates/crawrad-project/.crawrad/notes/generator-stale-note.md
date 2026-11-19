---
id: generator-stale-note
kind: doc
target_file: src/generator.rs
line_hint: 398
author: 
created: 
title: "stale_note"
tags: []
signature: "pub fn stale_note (stale : & [PathBuf]) -> Option < String >"
---

The tail of the sentence naming what stayed on screen showing the old text.

`None` when nothing stayed, and the silence is the point: a bar that talks
about stale tabs in every single generation teaches people not to read the
bar.

# Examples

```rust
use crawrad_project::generator::stale_note;

assert_eq!(stale_note(&[]), None);
```
