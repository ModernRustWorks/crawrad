---
id: editor-footer-somente-leitura
kind: doc
target_file: src/editor_footer.rs
line_hint: 10
author: 
created: 
title: "SOMENTE_LEITURA"
tags: []
signature: "pub const SOMENTE_LEITURA : & str = \"Read only\""
---

What the mode cell says when the view refuses every key.

# Examples

```rust
use crawrad_host::editor_footer::SOMENTE_LEITURA;

assert_eq!(SOMENTE_LEITURA, "Read only");
```
