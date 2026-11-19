---
id: format-version
kind: doc
target_file: src/lib.rs
line_hint: 54
author: 
created: 
title: "FORMAT_VERSION"
tags: []
signature: "pub const FORMAT_VERSION : u32 = 2"
---

Version of the `.crawform` file format.

# Examples

```rust
use crawrad_ir::FORMAT_VERSION;

assert_eq!(FORMAT_VERSION, 2);
```

**v2** separated identity from label (ADR 0008):
the node `id` is stable and never changes, and the `Name` that the user edits is
a property like any other. A v1 file is migrated on open —
its ids become the stable ids and seed the names.
