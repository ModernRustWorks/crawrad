---
id: i18n-language-tag
kind: doc
target_file: src/i18n.rs
line_hint: 25
author: 
created: 
title: "Language::tag"
tags: []
signature: "pub fn tag (& self) -> & 'static str"
---

Returns a short label/tag representing the language.

# Examples

```rust
use crawrad_actions::i18n::Language;

assert_eq!(Language::English.tag(), "EN");
assert_eq!(Language::Portuguese.tag(), "PT");
```
