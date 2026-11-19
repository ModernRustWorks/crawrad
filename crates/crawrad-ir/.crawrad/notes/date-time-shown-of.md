---
id: date-time-shown-of
kind: doc
target_file: src/date_time.rs
line_hint: 395
author: 
created: 
title: "shown_of"
tags: []
signature: "pub fn shown_of (node : & crate :: Node) -> String"
---

Returns the display text for a `DateTimePicker` node.

What a `DateTimePicker` node shows: its `date_time` (or its kind's
default) in the shown form of its `kind`.

# Examples

```rust
use crawrad_ir::date_time::shown_of;
use crawrad_ir::Node;

let node = Node::new("dtp", "DateTimePicker");
assert_eq!(shown_of(&node), "01/01/2000");
```
