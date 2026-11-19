---
id: removal
kind: doc
target_file: src/lib.rs
line_hint: 1228
author: 
created: 
title: "removal"
tags: []
signature: "fn removal (c : & Context , label : & str) -> Option < Transaction >"
---

Deletes an entire selection and disconnects dangling references.

Commands deleting an entire selection: descendants along with it, leaves
first (the removal command only accepts leaves), and **dangling reference
disconnected** — deleting an action cannot leave a button pointing into the void.
