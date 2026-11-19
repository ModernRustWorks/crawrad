---
id: doc-note-parse-item-start
kind: doc
target_file: src/doc_note.rs
line_hint: 274
author: 
created: 
title: "parse_item_start"
tags: []
signature: "fn parse_item_start (line : & str) -> Option < (ItemKind , String , bool) >"
---

Parses the beginning of an item declaration on a line.

Parses the beginning of an item declaration on a line.
Returns `(kind, name, is_unsafe)`.
