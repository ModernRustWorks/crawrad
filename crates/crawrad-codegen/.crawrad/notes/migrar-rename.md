---
id: migrar-rename
kind: doc
target_file: src/lib.rs
line_hint: 337
author: 
created: 
title: "migrar_rename"
tags: []
signature: "fn migrar_rename (raiz : & Path , arquivos : & [Arquivo]) -> Result < Vec < (PathBuf , PathBuf) > , Erro >"
---

Brings the user file along when the form changes its name.

Without this, renaming the form in the inspector is a silent loss of
work: the generator writes an empty `src/novo.rs`, `main.rs` starts
pointing to it, and `src/velho.rs` — with the hand-written handlers —
remains on disk without anyone compiling it. It is what a classic RAD IDE does when renaming a
unit, and for the same reason.

It only acts when **there is no doubt**: exactly one orphaned designer on disk,
its user file existing, and the one with the new name not existing yet. With two
candidates, there is no way to know whose code it was, so it is left untouched.
