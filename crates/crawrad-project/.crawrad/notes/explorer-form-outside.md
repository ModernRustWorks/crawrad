---
id: explorer-form-outside
kind: doc
target_file: src/explorer.rs
line_hint: 93
author: 
created: 
title: "FORM_OUTSIDE"
tags: []
signature: "pub const FORM_OUTSIDE : & str = \"the open form is not inside a <member>/src/ folder\""
---

What the panel says when the open form is not inside a member.

# Examples

```rust
use crawrad_project::explorer::FORM_OUTSIDE;

assert!(!FORM_OUTSIDE.is_empty());
```
