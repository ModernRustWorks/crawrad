---
id: code-lens-lenses-of
kind: doc
target_file: src/code_lens.rs
line_hint: 269
author: 
created: 
title: "lenses_of"
tags: []
signature: "fn lenses_of (job : & Job , jobs : & Receiver < Job > , newer : & mut Option < Job >) -> Option < Vec < CodeLens > >"
---

Resolves lenses for the given job in range order.

The lenses of the job's file, each resolved and in range order; `None`
when a newer job arrived meanwhile (it is left in `newer`). A lens whose
resolve fails keeps no command.
