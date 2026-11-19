---
id: gravar-bytes
kind: doc
target_file: src/lib.rs
line_hint: 479
author: 
created: 
title: "gravar_bytes"
tags: []
signature: "fn gravar_bytes (destino : & Path , bytes : & [u8]) -> Result < () , Erro >"
---

Replaces the ENTIRE file at once: a temporary file alongside it, then `rename`.

`std::fs::write` opens with `truncate`. Between the truncate and the last byte,
a window exists where the file on disk is half-written — and if the
process dies there (full disk, antivirus holding the handle, power outage),
that is what remains.

For the designer and `main.rs`, this window costs nothing: a `Generate`
recreates them. For the **user file**, it costs their work, which is not
regenerable — and two paths here write to it: `crescer()`, which has always
written (reads back, appends the new handler, writes back), and
`desbomar()`. Closing the window for one and leaving the other open would make no
sense, so it closes here, for both.

`rename` is what `write` is not: for an observer of the path, it replaces the
file all at once — one sees either the old file entirely or the new one entirely, never half.
The temporary file is created **in the same folder** on purpose: `rename` across volumes
fails on Windows, and the system temp folder is usually on
another. If `rename` fails, the temporary file is removed and the original remains
exactly as it was.

`Document::save` in `crawrad-ir` writes directly and has the same window. That is not
changed from here — it belongs to another crate and another owner —, but this is the house standard.
There the cost is also lower: the user can reopen and redesign the `.crawform`;
the `form_main.rs` with hand-written handlers, cannot.
