---
id: icons-classicon-of-class
kind: doc
target_file: src/icons.rs
line_hint: 546
author: 
created: 
title: "ClassIcon::of_class"
tags: []
signature: "pub fn of_class (class : & str) -> Option < Self >"
---

The icon of a class, when it has one.

`Option`, and not a generic icon handed back in silence: a catalogue
class with no drawing is a defect, and the test that charges for it is
`every_palette_class_has_an_icon`. On screen the row draws nothing where
the icon would be, and the row still reads.

# Examples

```rust
use crawrad_project::icons::ClassIcon;

assert_eq!(ClassIcon::of_class("Button"), Some(ClassIcon::Button));
```
