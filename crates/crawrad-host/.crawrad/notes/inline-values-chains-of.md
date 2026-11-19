---
id: inline-values-chains-of
kind: doc
target_file: src/inline_values.rs
line_hint: 117
author: 
created: 
title: "chains_of"
tags: []
signature: "fn chains_of (code : & str) -> Vec < String >"
---

Extracts identifier chains (`a.b.c`) from a line of code.

The paths of a line of code: `a.b.c` chains of identifiers, not inside a
string or a `//` comment, not starting after a `.`, and stopping before a
method call (`a.b.len()` gives `a.b`).
