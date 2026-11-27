---
id: hostmodel-bring-to-front
kind: doc
target_file: src/lib.rs
line_hint: 290
author: 
created: 
title: "HostModel::bring_to_front"
tags: []
signature: "pub fn bring_to_front (& mut self, i : usize) -> bool"
---

Brings the form at `i` to the front, answering whether there was one there.

An index past the end answers `false` and changes nothing, so a stale click from a
strip that has since shrunk cannot leave the host pointing at a form that is gone.

# Examples

```rust
use crawrad_host::HostModel;
use crawrad_doc::Doc;

let mut host = HostModel::new();
host.open_form(Doc::default(), None);
assert!(!host.bring_to_front(7), "there is no seventh tab");
assert_eq!(host.front(), Some(0), "and the front did not move");
```
