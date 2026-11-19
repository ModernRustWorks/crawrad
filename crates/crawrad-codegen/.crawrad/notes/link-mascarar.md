---
id: link-mascarar
kind: doc
target_file: src/link.rs
line_hint: 232
author: 
created: 
title: "mascarar"
tags: []
signature: "fn mascarar (fonte : & str) -> Vec < u8 >"
---

A source with everything that **is not code** replaced by spaces.

Replacing instead of trimming is the point: offsets remain those of the
original file, so the byte returned by this module is the byte the editor
uses. Line breaks are preserved only for cleanliness — whoever counts lines
counts them in the real source.

# Coupling with handler pruning (`fecha_o_que_veio_antes`)

Pruning of empty handlers (`stubs::podaveis`) depends directly on the
masking performed by this function: if the user wraps an entire handler
in a block comment (`/* ... */`) or multiline string, pruning refusal is
guaranteed here — `handlers_do_arquivo` does not see the commented `fn` in the
masked source, preventing it from entering the inventory to be pruned, while
`fecha_o_que_veio_antes` only validates the syntactic closing of the immediately preceding line.
