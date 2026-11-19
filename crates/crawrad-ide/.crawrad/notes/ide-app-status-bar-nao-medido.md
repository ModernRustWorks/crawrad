---
id: ide-app-status-bar-nao-medido
kind: doc
target_file: src/ide_app/status_bar.rs
line_hint: 83
author: 
created: 
title: "NAO_MEDIDO"
tags: []
signature: "pub (super) const NAO_MEDIDO : & str = \"—\""
---

What the problems cell says before anything has been built.

A `—` and not a `0`, and it is the same rule the node count already
follows: zero errors is a **verdict**, and claiming one before the compiler
has spoken is the kind of plausible lie this bar had to be cleaned of once
already.
