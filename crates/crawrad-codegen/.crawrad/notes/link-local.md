---
id: link-local
kind: doc
target_file: src/link.rs
line_hint: 43
author: 
created: 
title: "Local"
tags: []
signature: "pub struct Local"
---

Where a definition is, formatted as an editor needs.

# Examples

```rust
use crawrad_codegen::link::Local;

let loc = Local { linha: 1, coluna: 1, byte: 0 };
assert_eq!(loc.linha, 1);
```

`linha` and `coluna` are **1-based**, as every status bar shows;
`byte` is the offset within the passed `fonte`, which is what serves
to position the cursor without recounting the file.

The column counts **characters**, not bytes: an accented letter before the name shifts
one extra byte, and the status bar would be lying.
