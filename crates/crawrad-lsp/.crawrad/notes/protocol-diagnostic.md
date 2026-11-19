---
id: protocol-diagnostic
kind: doc
target_file: src/protocol.rs
line_hint: 114
author: 
created: 
title: "Diagnostic"
tags: []
signature: "pub struct Diagnostic"
---

A diagnostic, such as a compiler error, warning, or lint notice.

# Examples

```rust
use crawrad_lsp::protocol::{Diagnostic, DiagnosticSeverity, Position, Range};

let diag = Diagnostic {
    range: Range::new(Position::new(0, 0), Position::new(0, 1)),
    severity: Some(DiagnosticSeverity::Error),
    code: None,
    source: Some("rustc".to_string()),
    message: "syntax error".to_string(),
    tags: Vec::new(),
};
assert_eq!(diag.severity, Some(DiagnosticSeverity::Error));
```
