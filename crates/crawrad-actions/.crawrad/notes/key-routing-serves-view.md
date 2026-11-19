---
id: key-routing-serves-view
kind: doc
target_file: src/key_routing.rs
line_hint: 180
author: 
created: 
title: "serves_view"
tags: []
signature: "pub fn serves_view (id : & str , code_in_front : bool) -> bool"
---

Returns whether an action serves the currently active view.

Does the action serve the view in front? A key with two owners, one per
view (Ctrl+C: the components' copy, the text's copy, #257), runs the one
of the view in front. An action of any view always serves.

# Examples

```rust
use crawrad_actions::key_routing::serves_view;

assert!(serves_view("file.save", true));
```
