---
id: color-dialog-channel
kind: doc
target_file: src/color_dialog.rs
line_hint: 12
author: 
created: 
title: "Channel"
tags: []
signature: "pub enum Channel"
---

Color channel identifier for manual component editing.

# Examples

```rust
use crawrad_actions::color_dialog::Channel;

let ch = Channel::Red;
assert_eq!(ch, Channel::Red);
```
