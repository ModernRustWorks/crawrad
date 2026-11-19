---
id: resolve-bases
kind: doc
target_file: src/lib.rs
line_hint: 205
author:
created:
title: "resolve_bases"
tags: []
signature: "pub (crate) fn resolve_bases (bases : & [crawrad_ir :: Document]) -> Option < crawrad_ir :: Document >"
---

The nearest base with every base above it folded in, or none.

Resolved from the top down, so each level's overrides sit over the one
above: the components of the whole chain, as the derived form inherits them.
