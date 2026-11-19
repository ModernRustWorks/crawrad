---
id: crescer-e-podar
kind: doc
target_file: src/lib.rs
line_hint: 528
author: 
created: 
title: "crescer_e_podar"
tags: []
signature: "fn crescer_e_podar (raiz : & Path , destino : & Path , arquivo : & Arquivo , generated_before : & str ,) -> Result < (bool , Vec < String >) , Erro >"
---

Updates the user file by adding new handlers and pruning empty ones.

The two halves of the user file contract, in a single read and a single
write: **grows** with the event that was connected, **shrinks** with what was
disconnected and was never filled in.

A single function, and a single write, because both modify the same text. In
two separate steps the file would go to disk twice — and a crash in the
middle would leave the growth written without the pruning, or worse, the opposite.

**Order matters:** grows first. That way the handlers the document
currently points to are already in the text when pruning runs, and pruning knows
them from the list of `ligados` — there is no moment in which a newly created handler
looks orphaned.

Returns whether it grew, and the names that were pruned.
