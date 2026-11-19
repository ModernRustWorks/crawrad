---
id: icons-classicon-class
kind: doc
target_file: src/icons.rs
line_hint: 486
author: 
created: 
title: "ClassIcon::class"
tags: []
signature: "pub fn class (self) -> & 'static str"
---

The catalogue class this icon draws.

**One source** for the class-to-icon pair: [`Self::of_class`] searches
through here instead of repeating the `match` backwards, otherwise the
two tables could disagree.

# Examples

```rust
use crawrad_project::icons::ClassIcon;

assert_eq!(ClassIcon::Edit.class(), "Edit");
```
