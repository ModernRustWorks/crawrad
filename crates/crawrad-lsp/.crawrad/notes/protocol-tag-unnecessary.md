---
id: protocol-tag-unnecessary
kind: doc
target_file: src/protocol.rs
line_hint: 416
author: 
created: 
title: "TAG_UNNECESSARY"
tags: []
signature: "pub const TAG_UNNECESSARY : u32 = 1"
---

LSP `DiagnosticTag.Unnecessary`: unused or unreachable code, drawn faded.

# Examples

```rust
use crawrad_lsp::protocol::TAG_UNNECESSARY;

assert_eq!(TAG_UNNECESSARY, 1);
```
