---
id: desbomar
kind: doc
target_file: src/lib.rs
line_hint: 510
author: 
created: 
title: "desbomar"
tags: []
signature: "fn desbomar (destino : & Path) -> Result < bool , Erro >"
---

Strips the BOM from a user file that came from an older project.

# The exception, and its boundary

The law is that **the user file is born once and then only grows** — the
generator never rewrites what the user typed. This function is the only exception, and
it is narrow on purpose:

**Allowed:** remove the three bytes `EF BB BF` from the start of any `.rs`
in the project that has them. Almost always they are OURS — `gravar()`
put them in every `.rs` until 2026-09-01 —, but the function **does not check that, and
could not**: if the user himself saved the file with a BOM
using his editor, we strip it just the same, without asking.

What is guaranteed is not the origin of the three bytes, but what stripping them does: the BOM
does not appear on the screen in any editor, disappears without changing a single character of text,
and blinds rust-analyzer — with it in front, the file's `#![allow(...)]`
ceases to be an inner attribute and its `mod` begins flagging
`non_snake_case` over in `main.rs`. Regenerating an older project could not
leave half of it dirty.

**Forbidden:** anything else. Do not reformat, do not normalize line
endings, do not reorder `use`, do not fix indentation, do not add or remove
a blank line. That is why it operates on BYTES and writes raw `b[3..]`:
there is no path here through which the user's text can change. If one day
someone wants to take advantage of this rewrite to "clean up" something else,
the answer is no — the second exception is always easier than the first, and
the test `o_bom_sai_e_mais_nada_muda_no_arquivo_do_usuario` is here to
enforce this.

Returns whether it made changes.
