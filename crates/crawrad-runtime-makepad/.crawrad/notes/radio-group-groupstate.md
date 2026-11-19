---
id: radio-group-groupstate
kind: doc
target_file: src/radio_group.rs
line_hint: 44
author: 
created: 
title: "GroupState"
tags: []
signature: "pub struct GroupState"
---

What the form gives the group to show.

# Examples

```rust
use crawrad_runtime_makepad::radio_group::GroupState;

let state = GroupState::default();
assert_eq!(state.index, -1);
```
