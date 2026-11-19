---
id: icons-chrome
kind: doc
target_file: src/icons.rs
line_hint: 76
author: 
created: 
title: "Chrome"
tags: []
signature: "pub enum Chrome"
---

A **role** of the IDE chrome — what the button means, not how it is drawn.

Role and not glyph: it is what lets the whole set be swapped in one place.

# Examples

```rust
use crawrad_project::icons::Chrome;

assert_eq!(Chrome::Save.codicon_name(), "save");
```
