---
id: render-utf8-at
kind: doc
target_file: src/render.rs
line_hint: 107
author: 
created: 
title: "utf8_at"
tags: []
signature: "fn utf8_at (cx : & Cx < '_ > , addr : u64 , len : u64) -> Option < String >"
---

`len` bytes at `addr` as a `{:?}` string, cut at [`MAX_STRING_BYTES`].
