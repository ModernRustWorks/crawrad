---
id: events-select-item
kind: doc
target_file: src/lib.rs
line_hint: 362
author: 
created: 
title: "Events::SELECT_ITEM"
tags: []
signature: "pub const SELECT_ITEM : EventDesc = EventDesc :: new (\"on_select_item\" , \"OnSelectItem\" , \"fn(&mut self, index: usize, selected: bool)\" ,)"
---

Fired when an item enters or leaves selection.

A row entered (`selected` true) or left (false) the selection: the row
that leaves is announced before the one that arrives.

# Examples

```rust
use crawrad_registry::Events;

assert_eq!(Events::SELECT_ITEM.name, "on_select_item");
```
