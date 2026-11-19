---
id: pdb-builder-convert
kind: doc
target_file: src/pdb.rs
line_hint: 116
author: 
created: 
title: "Builder::convert"
tags: []
signature: "fn convert (& mut self , index : PdbIndex , nesting : usize) -> Option < TypeId >"
---

Converts a PDB type and its dependencies into a TypeId.

Our id for the PDB type at `index`, converting it (and what it needs)
the first time.
