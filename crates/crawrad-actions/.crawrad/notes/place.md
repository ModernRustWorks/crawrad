---
id: place
kind: doc
target_file: src/lib.rs
line_hint: 1165
author: 
created: 
title: "Place"
tags: []
signature: "pub struct Place"
---

Menu placement specifying section and optional submenu for an action.

Where an action sits inside its menu (#558): the section it is drawn with
(a rule between two sections) and, for an action in a submenu, the submenu's
name and the section inside it.

# Examples

```rust
use crawrad_actions::Place;

let place = Place::section(1);
assert_eq!(place.section, 1);
```
