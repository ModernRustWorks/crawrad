---
id: label-labellook
kind: doc
target_file: src/label.rs
line_hint: 28
author: 
created: 
title: "LabelLook"
tags: []
signature: "pub struct LabelLook < 'a >"
---

What the form says about a label at this moment.

# Examples

```rust
use crawrad_runtime_makepad::label::LabelLook;

let look = LabelLook {
    alignment: "taLeftJustify",
    layout: "tlTop",
    word_wrap: false,
    auto_size: true,
    size: (100.0, 20.0),
};
assert!(look.auto_size);
```
