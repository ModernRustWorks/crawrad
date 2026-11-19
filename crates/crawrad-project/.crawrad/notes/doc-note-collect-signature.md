---
id: doc-note-collect-signature
kind: doc
target_file: src/doc_note.rs
line_hint: 381
author: 
created: 
title: "collect_signature"
tags: []
signature: "fn collect_signature (lines : & [& str] , start_line : usize , indent_len : usize ,) -> (String , usize , Option < char >)"
---

Collects the signature starting from `item_line` up to `{` or `;`.
