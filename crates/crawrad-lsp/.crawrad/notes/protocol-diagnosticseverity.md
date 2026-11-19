---
id: protocol-diagnosticseverity
kind: doc
target_file: src/protocol.rs
line_hint: 82
author: 
created: 
title: "DiagnosticSeverity"
tags: []
signature: "pub enum DiagnosticSeverity"
---

The severity level of a diagnostic.

# Examples

```rust
use crawrad_lsp::protocol::DiagnosticSeverity;

assert_eq!(DiagnosticSeverity::from(1), DiagnosticSeverity::Error);
```
