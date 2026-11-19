---
id: code-lens-codelensworker-take
kind: doc
target_file: src/code_lens.rs
line_hint: 263
author: 
created: 
title: "CodeLensWorker::take"
tags: []
signature: "pub fn take (& self) -> Option < CodeLensAnswer >"
---

The newest answer that arrived since the last call, if any.

# Examples

```rust
use crawrad_lsp::code_lens::CodeLensWorker;

let worker = CodeLensWorker::new();
assert!(worker.take().is_none());
```
