---
id: word-motion-class
kind: doc
target_file: src/word_motion.rs
line_hint: 5
author: 
created: 
title: "Class"
tags: []
signature: "enum Class"
---

What a character counts as when a word boundary is being looked for.

Three classes and not two, and the third is what makes `Ctrl+Left` useful in
code: on `self.edt_nome_.text` the walk stops at `text`, then at `.`, then
at `edt_nome_` — instead of jumping the whole expression in one press.
