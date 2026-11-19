---
id: capability-paletteitem
kind: doc
target_file: src/capability.rs
line_hint: 924
author: 
created: 
title: "PaletteItem"
tags: []
signature: "pub struct PaletteItem"
---

One row of the command palette.

# Examples

```rust
use crawrad_actions::capability::PaletteItem;

let item = PaletteItem {
    id: "file.save".to_string(),
    label: "Save".to_string(),
    description: "Save file".to_string(),
    group: "File".to_string(),
    shortcut: Some("Ctrl+S".to_string()),
};
assert_eq!(item.id, "file.save");
```
