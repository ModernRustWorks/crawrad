---
id: dialogs-options
kind: doc
target_file: src/dialogs.rs
line_hint: 182
author: 
created: 
title: "OPTIONS"
tags: []
signature: "pub const OPTIONS : DialogSpec = DialogSpec"
---

Dialog specification for Tools > Options.

Tools > Options (`dialogs/dsl/options_dialog.rs`): every field, check box,
drop-down and button of every page; the controls of the pages not shown
report `visible: false`. The page tree on the left is a list drawn row by
row (a `PortalList`), so its rows come in `rows`, as drawn.

# Examples

```rust
use crawrad_mcp::dialogs::OPTIONS;

assert_eq!(OPTIONS.name, "options");
```
