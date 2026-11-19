---
id: options-optionrow
kind: doc
target_file: src/options.rs
line_hint: 414
author: 
created: 
title: "OptionRow"
tags: []
signature: "pub enum OptionRow"
---

A flattened tree row, for flat list renderers.

# Examples

```rust
use crawrad_project::options::OptionRow;

let row = OptionRow::Group { name: "IDE", open: true };
match row {
    OptionRow::Group { name, open } => {
        assert_eq!(name, "IDE");
        assert!(open);
    }
    _ => unreachable!(),
}
```
