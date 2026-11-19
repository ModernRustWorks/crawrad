---
id: note-preview-preview-markdown
kind: doc
target_file: src/note_preview.rs
line_hint: 9
author: 
created: 
title: "preview_markdown"
tags: []
signature: "pub fn preview_markdown (signature : & str , body : & str) -> String"
---

The markdown of the Preview tab: the signature as code, a rule, the body.

An intra-doc link (`` [`Name`] `` with no target) has no URL outside
rustdoc, so it shows as code instead of literal brackets.

# Examples

```rust
use crawrad_project::note_preview::preview_markdown;

let md = preview_markdown("fn foo()", "Sample body.");
assert!(md.contains("fn foo()"));
```
