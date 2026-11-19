---
id: i18n-language-toggle
kind: doc
target_file: src/i18n.rs
line_hint: 17
author: 
created: 
title: "Language::toggle"
tags: []
signature: "pub fn toggle (& mut self)"
---

Toggles between English and Portuguese languages.

# Examples

```rust
use crawrad_actions::i18n::Language;

let mut lang = Language::English;
lang.toggle();
assert_eq!(lang, Language::Portuguese);
```
