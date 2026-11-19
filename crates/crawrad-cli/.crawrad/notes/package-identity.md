---
id: package-identity
kind: doc
target_file: src/lib.rs
line_hint: 74
author: 
created: 
title: "package_identity"
tags: []
signature: "fn package_identity (root : & Path) -> Result < (String , String , String) , String >"
---

Who the package is, read from its `Cargo.toml`.

**The `Cargo.toml` is still TOML**, and only it: the manifest became JSON
in ADR 0039, this file did not.
