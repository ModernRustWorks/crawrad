---
id: targets-makepad-actions-single-if
kind: doc
target_file: src/targets/makepad/actions.rs
line_hint: 47
author: 
created: 
title: "single_if"
tags: []
signature: "fn single_if (block : & str) -> Option < & str >"
---

Extracts the condition of a block when it is a single if statement.

The condition of `block` when it is a single `if` with no `else` (its
first line opens it, its last line closes it, nothing else sits at its
level), else `None`.
