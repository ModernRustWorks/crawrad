---
id: emission-why-not-set
kind: doc
target_file: src/emission.rs
line_hint: 111
author: 
created: 
title: "why_not_set"
tags: []
signature: "pub fn why_not_set (key : & str , category : Category) -> Option < NotSet >"
---

Indicates whether a property does not become a `Set` for a declared reason.

Does the property become a `Set`, or does it not for a declared reason?

**It does not query any class.** This is the rule that holds for today's twenty
and for the twenty-first arriving from a `manifest.json`.

# Examples

```rust
use crawrad_codegen::emission::{NotSet, why_not_set};
use crawrad_registry::Category;

assert_eq!(why_not_set("name", Category::Identity), Some(NotSet::Identity));
assert_eq!(why_not_set("caption", Category::Appearance), None);
```
