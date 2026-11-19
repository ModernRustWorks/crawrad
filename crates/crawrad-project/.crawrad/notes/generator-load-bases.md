---
id: generator-load-bases
kind: doc
target_file: src/generator.rs
line_hint: 230
author:
created:
title: "load_bases"
tags: []
signature: "pub (crate) fn load_bases (destination : Option < & Path > , ir : & Document) -> Vec < Document >"
---

The form's chain of bases from the project folder, nearest first.

It follows `inherits` up until a form inherits nothing, a base file is
missing, or a name comes back (a cycle, refused where the form is saved).
