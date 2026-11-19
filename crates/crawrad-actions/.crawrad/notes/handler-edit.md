---
id: handler-edit
kind: doc
target_file: src/lib.rs
line_hint: 1277
author: 
created: 
title: "handler_edit"
tags: []
signature: "pub fn handler_edit (ir : Option < & crawrad_ir :: Document > , node : & str , event : & str , text : & str ,) -> (& 'static str , Args)"
---

Determines the handler action when editing an event binding.

The action the Object Inspector's Events field runs when `text` is
confirmed for `node.event`: a new, unused name for a bound handler renames
it; anything else binds, shares an existing handler or (empty) unbinds.

# Examples

```rust
use crawrad_actions::handler_edit;

let (action_id, _args) = handler_edit(None, "Button1", "OnClick", "Button1Click");
assert_eq!(action_id, "event.bind");
```
