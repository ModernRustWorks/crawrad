---
id: protocol-documentation
kind: doc
target_file: src/protocol.rs
line_hint: 242
author: 
created: 
title: "Documentation"
tags: []
signature: "pub enum Documentation"
---

A human-readable documentation string or formatted markdown.

# Examples

```rust
use crawrad_lsp::protocol::Documentation;

let doc = Documentation::String("doc text".to_string());
assert_eq!(doc.as_str(), "doc text");
```
