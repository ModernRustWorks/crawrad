---
id: note-rustdoc-lines
kind: doc
target_file: src/note.rs
line_hint: 1553
author: 
created: 
title: "rustdoc_lines"
tags: []
signature: "fn rustdoc_lines (note : Option < & NoteFile > , line : Option < & str > , id : & str ,) -> Result < Vec < String > , String >"
---

Formats doc comment lines from a note file.

The `///` lines that replace the anchor `line` of note `id`, or why it
stays: rustdoc strips one space after `///`, so the text is the same
`#[doc = body]` the `#[note]` macro injects.
