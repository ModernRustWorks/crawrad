---
id: ide-app-chrome-ideapp-language
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 14
author: 
created: 
title: "IdeApp::language"
tags: []
signature: "pub fn language (& self) -> Language"
---

Returns the currently active UI language.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let app = IdeApp::default();
assert_eq!(app.language(), Language::default());
```
