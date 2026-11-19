---
id: doc-note-clean-type
kind: doc
target_file: src/doc_note.rs
line_hint: 1360
author: 
created: 
title: "clean_type"
tags: []
signature: "fn clean_type (ty : & str , generic_params : & [String]) -> String"
---

Cleans a type signature for use in generated doc examples.

The type written in the example: `_` when it names a generic parameter
or an `impl Trait`, else the type without its named lifetimes.
