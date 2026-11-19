---
id: project-default
kind: doc
target_file: src/lib.rs
line_hint: 187
author: 
created: 
title: "Project::default"
tags: []
signature: "fn default () -> Self"
---

**Open**, and the choice is deliberate.

When querying `cargo` failed — it is not in PATH, the manifest is
broken, there is no open folder yet — the IDE **does not invent a
verdict**: it leaves the button as it always was, and whoever explains
what happened is `cargo` itself in the status bar. Disabling `Run`
because the query failed would trade a readable error for a dead button
that cannot be read.
