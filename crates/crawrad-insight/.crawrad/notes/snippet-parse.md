---
id: snippet-parse
kind: doc
target_file: src/snippet.rs
line_hint: 66
author: 
created: 
title: "parse"
tags: []
signature: "fn parse (chars : & [char] , i : & mut usize , out : & mut Out < '_ > , nested : bool)"
---

Writes `chars` from `i` until the end, or an unescaped `}` when `nested`.
