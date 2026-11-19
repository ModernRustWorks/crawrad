---
id: ide-app-constants-no-icon
kind: doc
target_file: src/ide_app/constants.rs
line_hint: 121
author: 
created: 
title: "NO_ICON"
tags: []
signature: "pub (super) const NO_ICON : f64 = - 1.0"
---

The `kind` that draws NOTHING in the class-icon shader.

Any value no branch matches would do; a negative one says out loud that it
is not an icon, and it can never collide with a position in
`ClassIcon::ALL`.
