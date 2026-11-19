---
id: note-missingdocswalk-module
kind: doc
target_file: src/note.rs
line_hint: 1891
author: 
created: 
title: "MissingDocsWalk::module"
tags: []
signature: "fn module (& mut self , m : & syn :: ItemMod , dir : & Path) -> Result < () , String >"
---

Inspects a public module for missing documentation.

A public module: inline, or `mod x;` read from `dir/x.rs` or
`dir/x/mod.rs`, whose `//!` documents it as well as `///` does.
