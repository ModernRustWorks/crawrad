---
id: installation-crate-name
kind: doc
target_file: src/installation.rs
line_hint: 165
author: 
created: 
title: "crate_name"
tags: []
signature: "fn crate_name (text : & str) -> Option < String >"
---

The `[package] name` of a `Cargo.toml`, parsed as TOML.

As TOML and not as text: a `name` inside `[dependencies]` or inside a comment
would answer the wrong question.
