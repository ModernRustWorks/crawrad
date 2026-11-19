---
id: session-place-same-line-elsewhere
kind: doc
target_file: src/session.rs
line_hint: 137
author: 
created: 
title: "Place::same_line_elsewhere"
tags: []
signature: "fn same_line_elsewhere (& self , origin : & Place) -> bool"
---

Checks whether another location belongs to the same source line.

Another piece of the SAME line's code: the breakpoint of a line that
compiles to several places hit again, not a stop worth showing. The
same instruction again is a real stop (a loop's next turn, recursion).
