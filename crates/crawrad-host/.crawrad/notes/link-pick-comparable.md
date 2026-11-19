---
id: link-pick-comparable
kind: doc
target_file: src/link_pick.rs
line_hint: 134
author: 
created: 
title: "comparable"
tags: []
signature: "fn comparable (p : & Path) -> String"
---

Normalizes a path for comparison with lower case and trailing slash.

A path as compared: forward slashes, lower case (Windows paths come
back from the server as `c:\...`), and one trailing slash.
