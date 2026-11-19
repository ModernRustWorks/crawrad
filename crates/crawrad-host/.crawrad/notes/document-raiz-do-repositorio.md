---
id: document-raiz-do-repositorio
kind: doc
target_file: src/document.rs
line_hint: 38
author: 
created: 
title: "raiz_do_repositorio"
tags: []
signature: "fn raiz_do_repositorio () -> Option < PathBuf >"
---

The repository root inferred at compile time.

The repository root, inferred from the location of this file at compile
time: `crates/crawrad-host` → two folders up (moved out of the
shell, which only shows it — #398 paid the shell's ratchet with it).

Serves as a SECOND base, after the working directory, so that
`cargo run` works from any folder on the developer's machine. It
never changes the displayed path: what the bar says is the path that opened.
