---
id: sufixo-gerado
kind: doc
target_file: src/lib.rs
line_hint: 79
author: 
created: 
title: "SUFIXO_GERADO"
tags: []
signature: "pub const SUFIXO_GERADO : & str = \".designer.rs\""
---

The suffix of the file that `Generate` rewrites completely.

# Examples

```rust
use crawrad_codegen::SUFIXO_GERADO;

assert_eq!(SUFIXO_GERADO, ".designer.rs");
```

It is a classic RAD IDE's split between `.pas` and `.dfm`, and it is not a mere convention: a
file that the program overwrites cannot accept typing, because
accepting it would be promising work that the next generation deletes.

**It lives here because whoever writes the file dictates its name.** The
suffix was in five places — three in this crate, two in the shell — and the
five copies had different roles: one decides the output name, one the
`#[path]` of the generated `main.rs`, one decides **what to delete**, one marks the
buffer as read-only, and one paints the `Rewritten` badge in the Explorer.
Changing the suffix by touching four of them would yield an IDE that generates `a`, compiles
`b`, deletes `c`, and lets the user type over it — with no error along
the way, because each copy would remain consistent with itself.

It was verified: changed to `.gerado.rs`, the Phase 3 gate continues compiling
and running the generated project.
