---
id: link-fim-do-char
kind: doc
target_file: src/link.rs
line_hint: 373
author: 
created: 
title: "fim_do_char"
tags: []
signature: "fn fim_do_char (fonte : & str , ini : usize) -> Option < usize >"
---

Returns the end position of a Rust character literal.

The end of a character literal starting at `ini`, or `None` when
that single quote is a **lifetime** or a loop label.

The difference is only one: `'x'` closes, `'a` does not. Being wrong in either
direction is costly — reading `&'a str` as a literal consumes up to the next single quote,
and reading `'"'` as a lifetime leaves a stray double quote that turns half
the file into a string.
