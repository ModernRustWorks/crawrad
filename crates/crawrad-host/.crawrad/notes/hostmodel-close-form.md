---
id: hostmodel-close-form
kind: doc
target_file: src/lib.rs
line_hint: 300
author: 
created: 
title: "HostModel::close_form"
tags: []
signature: "pub fn close_form (& mut self, i : usize) -> bool"
---

Closes the tab at `i`, answering whether there was one there.

Closing a tab BEHIND the one in front must not change which form the person is
looking at, so the front index follows its own form instead of staying put. This is
the bug every tab strip is born with. Closing the front tab brings its right-hand
neighbour forward, as every editor does, falling back to the left at the end.

# Examples

```rust
use crawrad_host::HostModel;
use crawrad_doc::Doc;

let mut host = HostModel::new();
host.open_form(Doc::default(), None);
host.open_form(Doc::default(), None);
host.open_form(Doc::default(), None);
host.bring_to_front(2);
host.close_form(0);
assert_eq!(host.front(), Some(1), "the person is still looking at the same form");
```
