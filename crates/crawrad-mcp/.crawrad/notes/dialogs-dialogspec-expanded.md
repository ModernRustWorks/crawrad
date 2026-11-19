---
id: dialogs-dialogspec-expanded
kind: doc
target_file: src/dialogs.rs
line_hint: 86
author: 
created: 
title: "DialogSpec::expanded"
tags: []
signature: "pub fn expanded (& self) -> Vec < Control >"
---

Every control, `{i}` expanded.

# Examples

```rust
use crawrad_mcp::dialogs::NEW_PROJECT;

let controls = NEW_PROJECT.expanded();
assert!(!controls.is_empty());
```
