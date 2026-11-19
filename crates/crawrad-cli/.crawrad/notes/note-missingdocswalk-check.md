---
id: note-missingdocswalk-check
kind: doc
target_file: src/note.rs
line_hint: 1784
author: 
created: 
title: "MissingDocsWalk::check"
tags: []
signature: "fn check (& mut self , attrs : & [syn :: Attribute] , node : & impl ToTokens , kind : & 'static str , name : & syn :: Ident ,)"
---

Records `node` unless it is documented or compiled for tests only.
