---
id: hover-plain
kind: doc
target_file: src/hover.rs
line_hint: 192
author: 
created: 
title: "plain"
tags: []
signature: "pub fn plain (markdown : & str) -> String"
---

Strips markdown syntax into clean plain text for display cards.

Markdown made plain for a text card: fences dropped (their code kept
as is), `[text](link)` keeps the text, `**`/`` ` `` marks and heading
`#`s go, a `---` rule becomes a line, and blank runs collapse to one.

# Examples

```rust
use crawrad_lsp::hover::plain;

assert_eq!(plain("**bold**"), "bold");
```
