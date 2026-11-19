---
id: menu-sectioned-rows
kind: doc
target_file: src/menu.rs
line_hint: 218
author: 
created: 
title: "sectioned_rows"
tags: []
signature: "fn sectioned_rows (mut actions : Vec < crate :: Action > , section : impl Fn (& crate :: Action) -> u8 , collapse : bool , ctx : & Context ,) -> Vec < Row >"
---

Builds menu rows partitioned into sections with rules.

The rows of `actions` in sections (#558): stably sorted by `section`, a rule
between two sections and never at either end. In a top menu (`collapse`) the
actions of a submenu become ONE row that opens it, where its first one sits.
