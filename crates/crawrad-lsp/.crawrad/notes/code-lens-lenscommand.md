---
id: code-lens-lenscommand
kind: doc
target_file: src/code_lens.rs
line_hint: 55
author: 
created: 
title: "LensCommand"
tags: []
signature: "pub struct LensCommand"
---

The command attached to a resolved lens.

# Examples

```rust
use crawrad_lsp::code_lens::{LensCommand, LensKind};

let cmd = LensCommand {
    title: "3 references".to_string(),
    kind: LensKind::Other,
};
assert_eq!(cmd.title, "3 references");
```
