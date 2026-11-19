---
id: explorer-rows-rows-file-file-is-the-users
kind: doc
target_file: src/explorer_rows/rows/file.rs
line_hint: 86
author: 
created: 
title: "File::is_the_users"
tags: []
signature: "fn is_the_users (& self) -> bool"
---

Returns whether this file belongs to the user to keep.

Is this file the user's to keep?

`PrimeiraVez` counts as the user's, and that is the whole point of the
third owner: it is ours once and theirs from then on, so from the
second `Generate` onwards it behaves exactly like `Usuario`. Colouring
it as ours would be the panel promising less than the generator does.
