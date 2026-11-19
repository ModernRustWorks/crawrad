---
id: dialogs-build
kind: doc
target_file: src/dialogs.rs
line_hint: 307
author: 
created: 
title: "BUILD"
tags: []
signature: "pub const BUILD : DialogSpec = DialogSpec"
---

Dialog specification for the Build dialog.

The Build dialog (`dialogs/dsl/tools_dialogs.rs`): one check box and one
chip per target platform, in `TargetPlatform::ALL`'s order.

# Examples

```rust
use crawrad_mcp::dialogs::BUILD;

assert_eq!(BUILD.name, "build");
```
