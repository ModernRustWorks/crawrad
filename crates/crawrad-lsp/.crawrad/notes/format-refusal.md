---
id: format-refusal
kind: doc
target_file: src/format.rs
line_hint: 122
author: 
created: 
title: "refusal"
tags: []
signature: "pub fn refusal (front : Option < & Path > , generated_suffix : & str) -> & 'static str"
---

Explains why formatting was refused for the active file.

Why Format Document is refused on the file in front (`front`), which
takes no typing: the generated form source is the generator's.

# Examples

```rust
use crawrad_lsp::format::refusal;
use std::path::Path;

let msg = refusal(Some(Path::new("form.gen.rs")), ".gen.rs");
assert!(msg.contains("generated"));
```
