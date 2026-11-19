---
id: components-speed-button-speed-button
kind: doc
target_file: src/components/speed_button.rs
line_hint: 4
author: 
created: 
title: "speed_button"
tags: []
signature: "pub fn speed_button () -> ComponentDesc"
---

Compact toolbar button with pressed state and group support.

`SpeedButton`: compact button for toolbars with pressed state support and mutual groups.

# Examples

```rust
use crawrad_registry::speed_button;

let desc = speed_button();
assert_eq!(desc.class, "SpeedButton");
```
