---
id: varrer-orfaos
kind: doc
target_file: src/lib.rs
line_hint: 433
author: 
created: 
title: "varrer_orfaos"
tags: []
signature: "fn varrer_orfaos (raiz : & Path , arquivos : & [Arquivo]) -> Result < Vec < PathBuf > , Erro >"
---

Deletes the designer file left over from an old name.

Renaming the form makes the generator write `src/novo.designer.rs` — and
leave `src/velho.designer.rs` on disk. It is no longer compiled (the
`main.rs` points to the new one), but it stays there confusing whoever opens the folder,
and rust-analyzer complains about a file without a module.

The sweep is deliberately narrow: only `src/*.designer.rs`, which is a
name that **only** the generator writes. Nothing of the user's falls into this pattern, and
no other file is inspected — deleting by mistake in someone's project is
worse than leaving clutter behind.
