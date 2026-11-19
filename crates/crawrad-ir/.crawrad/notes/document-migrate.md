---
id: document-migrate
kind: doc
target_file: src/lib.rs
line_hint: 304
author: 
created: 
title: "Document::migrate"
tags: []
signature: "fn migrate (& mut self)"
---

Brings an older document to the current format.

**v1 → v2:** in v1 the id WAS the name. The migration is straightforward because of
that: existing ids become stable ids, and the name of each node
is born identical to the id it already had. Nothing moves, nothing is renumbered, and
the file remains readable — it just states the two things instead
of one.
