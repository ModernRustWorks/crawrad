---
id: link-fim-do-bloco
kind: doc
target_file: src/link.rs
line_hint: 289
author: 
created: 
title: "fim_do_bloco"
tags: []
signature: "fn fim_do_bloco (b : & [u8] , ini : usize) -> usize"
---

The byte right after the `*/` that closes the block opened at `ini`.

Rust block comments **nest** — unlike C. A `/* /* */ */`
that closed on the first `*/` would leave the rest of the file read as code
that the compiler never sees.
