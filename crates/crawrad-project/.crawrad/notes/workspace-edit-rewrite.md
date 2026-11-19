---
id: workspace-edit-rewrite
kind: doc
target_file: src/workspace_edit.rs
line_hint: 132
author: 
created: 
title: "rewrite"
tags: []
signature: "fn rewrite (root : & Path , edit : impl FnOnce (& str) -> Result < String , String >) -> Result < () , String >"
---

Rewrites the root manifest with `edit`.
