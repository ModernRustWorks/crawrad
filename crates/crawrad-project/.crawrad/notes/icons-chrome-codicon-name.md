---
id: icons-chrome-codicon-name
kind: doc
target_file: src/icons.rs
line_hint: 315
author: 
created: 
title: "Chrome::codicon_name"
tags: []
signature: "pub fn codicon_name (self) -> & 'static str"
---

The name of the drawing in Codicons.

The name of the drawing in Codicons — what the `post` table of the file
calls it, and what one searches for in the VS Code catalogue.

It is here so the divergence test can check the NAME as well as the code
point: two tables that agree on `\u{eb4b}` while disagreeing on what it
means would pass a comparison of code points alone.

# Examples

```rust
use crawrad_project::icons::Chrome;

assert_eq!(Chrome::Save.codicon_name(), "save");
```
