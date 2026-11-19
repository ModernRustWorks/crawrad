---
id: installation-do-not-copy
kind: doc
target_file: src/installation.rs
line_hint: 201
author: 
created: 
title: "DO_NOT_COPY"
tags: []
signature: "const DO_NOT_COPY : & [& str] = & [\"target\" , \".git\"]"
---

`target/` does NOT go along.

It is a compilation artifact from the publisher's machine, can be gigabytes,
and the user's `cargo` rebuilds it. Copying it would turn a half-second installation
into a minute and fill the installer's disk.
