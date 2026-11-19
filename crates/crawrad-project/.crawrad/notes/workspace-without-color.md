---
id: workspace-without-color
kind: doc
target_file: src/workspace.rs
line_hint: 1730
author: 
created: 
title: "without_color"
tags: []
signature: "pub fn without_color (raw : & str) -> String"
---

The same cargo text, **without colors**.

Exists because of issue #63, and the bug it tells is worth a paragraph.
CI sets `CARGO_TERM_COLOR: always` so GitHub logs come out colored;
`cargo test` inherits the variable, the `cargo new` tests launch inherits it
again, and `always` means *colorize even when writing to a pipe*. The line
that arrives then is not normal text: it is

```text
\x1b[1m\x1b[92m      Adding\x1b[0m `meu_app` as member of workspace at `…`
```

`trim()` does not strip ANSI escapes, `starts_with("Adding ")` becomes false,
and adoption becomes invisible. That is what kept `main` red for four merges —
and what made it pass locally: **locally no one exports that variable**,
and the test for this function fed it hand-written output that never had colors.

Reading tool text means reading what the tool writes **in the runner's environment**,
and color is part of that.

Strips CSI sequences (`ESC [ … letter`), which is what cargo emits, and the
two two-character sequences that linger. Not intended to be a terminal emulator.

# Examples

```rust
use crawrad_project::workspace::without_color;

assert_eq!(without_color("\x1b[1mhello\x1b[0m"), "hello");
```
