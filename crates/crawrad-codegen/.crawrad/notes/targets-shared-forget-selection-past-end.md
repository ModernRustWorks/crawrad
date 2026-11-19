---
id: targets-shared-forget-selection-past-end
kind: doc
target_file: src/targets/shared.rs
line_hint: 485
author: 
created: 
title: "forget_selection_past_end"
tags: []
signature: "pub fn forget_selection_past_end (owner : & str , selection : Selection) -> String"
---

Emits code resetting out-of-bounds selection to none.

#88: the selection a shorter list no longer reaches goes back to none.

A handler that removes items changes the collection and leaves the index
alone. While the index still falls inside the list it stays where it is;
once the list ends before it, it becomes `None` (`-1`), so the program
never reads a row that is gone. It is not a change the user made, so no
`OnChange` fires for it. `owner` is the control's state as the target
reaches it (`self.form.list_box1`, `self.ListBox1`).

# Examples

```rust
use crawrad_codegen::targets::shared::{Selection, forget_selection_past_end};

let code = forget_selection_past_end("self.form.list1", Selection::Optional);
assert!(code.contains("selected = None"));
```
