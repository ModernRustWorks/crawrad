---
id: manifest-evententry
kind: doc
target_file: src/manifest.rs
line_hint: 747
author: 
created: 
title: "EventEntry"
tags: []
signature: "pub struct EventEntry"
---

An event, in the file format.

# Examples

```rust
use crawrad_package::manifest::EventEntry;

let ev = EventEntry {
    name: "click".into(),
    display: "OnClick".into(),
    signature: "fn()".into(),
};
assert_eq!(ev.name, "click");
```
