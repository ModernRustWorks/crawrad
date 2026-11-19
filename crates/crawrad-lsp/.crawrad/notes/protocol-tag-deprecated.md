---
id: protocol-tag-deprecated
kind: doc
target_file: src/protocol.rs
line_hint: 419
author: 
created: 
title: "TAG_DEPRECATED"
tags: []
signature: "pub const TAG_DEPRECATED : u32 = 2"
---

LSP `DiagnosticTag.Deprecated`: drawn struck through.

# Examples

```rust
use crawrad_lsp::protocol::TAG_DEPRECATED;

assert_eq!(TAG_DEPRECATED, 2);
```
