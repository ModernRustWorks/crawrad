---
id: doc-note-terminator-pos
kind: doc
target_file: src/doc_note.rs
line_hint: 119
author: 
created: 
title: "terminator_pos"
tags: []
signature: "fn terminator_pos (text : & str , depth : & mut i32) -> Option < usize >"
---

Finds the byte position where the signature ends on this line.

Where the signature ends on this line: a `{`, or a `;` that is not inside
`(..)`/`[..]` (`-> [u8; 4]`). `depth` carries the nesting across lines.
