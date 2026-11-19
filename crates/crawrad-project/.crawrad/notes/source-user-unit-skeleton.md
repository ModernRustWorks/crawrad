---
id: source-user-unit-skeleton
kind: doc
target_file: src/source.rs
line_hint: 152
author: 
created: 
title: "user_unit_skeleton"
tags: []
signature: "fn user_unit_skeleton (doc : & crawrad_ir :: Document , package : & str) -> Option < String >"
---

The user file as the generator would write it, without touching the disk.

`None` when the generator does not produce a user file for this
document — a form whose plan fails, for example. Saying *"I don't know"*
is better than inventing a skeleton that generation would not confirm.
