---
id: hostmodel-front
kind: doc
target_file: src/lib.rs
line_hint: 260
author: 
created: 
title: "HostModel::front"
tags: []
signature: "pub fn front (& self) -> Option < usize >"
---

Which form's tab is in front, as an index into [`HostModel::forms`].

`doc()` answers for this one, which is how a hundred callers that ask the host for
"the document" keep asking exactly as they did when there could only be one.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert_eq!(host.front(), None, "nothing is in front when nothing is open");
```
