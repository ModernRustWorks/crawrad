---
id: inspector-model-categories-closed-on-open
kind: doc
target_file: src/inspector_model.rs
line_hint: 600
author: 
created: 
title: "categories_closed_on_open"
tags: []
signature: "pub fn categories_closed_on_open () -> BTreeSet < String >"
---

The categories born CLOSED when the IDE opens.

It is the egui shell's `default_open(cat != Category::Avancado)`
(`crates/crawrad-ide/src/inspector.rs`), expressed on this side — and derived
from `ORDER`, never from a hand-written list: a new category in the
catalogue enters here through the filter, not because someone remembered.

`Advanced` closes because it is what gets touched least, which is the same
reason why it is the last in `ORDER`. The other four open: an Object
Inspector opening closed forces four clicks before the first edit.

# Examples

```rust
use crawrad_actions::inspector_model::categories_closed_on_open;

let closed = categories_closed_on_open();
assert!(closed.contains("Advanced"));
```
