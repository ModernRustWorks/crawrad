---
id: doc-note-scan-leading-block
kind: doc
target_file: src/doc_note.rs
line_hint: 464
author: 
created: 
title: "scan_leading_block"
tags: []
signature: "fn scan_leading_block (lines : & [& str] , item_line : usize ,) -> (usize , std :: ops :: Range < usize > , String , Option < String >)"
---

Scans the leading block directly above an item.

Scans the leading block directly above an item.
Returns `(insert_line, doc_lines, existing_docs, existing_note)`.
