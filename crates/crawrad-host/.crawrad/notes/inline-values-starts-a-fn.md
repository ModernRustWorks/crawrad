---
id: inline-values-starts-a-fn
kind: doc
target_file: src/inline_values.rs
line_hint: 59
author: 
created: 
title: "starts_a_fn"
tags: []
signature: "fn starts_a_fn (code : & str) -> bool"
---

Checks whether a line opens a function declaration.

Whether a line opens a function: `fn`, after any of `pub`, `pub(crate)`,
`async`, `const`, `unsafe`, `extern "C"`.
