---
id: rename-declared
kind: doc
target_file: src/rename.rs
line_hint: 147
author: 
created: 
title: "declared"
tags: []
signature: "fn declared (client : & LspClient , file : & Path , at : Position) -> Vec < PathBuf >"
---

Finds the files where the name at `at` is declared.

Where the name at `at` is declared: the files of its definitions
(`ContentModified` asked again; no answer is no file).
