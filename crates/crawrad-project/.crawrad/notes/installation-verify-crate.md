---
id: installation-verify-crate
kind: doc
target_file: src/installation.rs
line_hint: 137
author: 
created: 
title: "verify_crate"
tags: []
signature: "fn verify_crate (origin : & Path , manifest : & Manifest) -> Result < () , String >"
---

Verifies that the crate name in `Cargo.toml` matches the package manifest.

**Does the crate name in the folder match the manifest?**

Only required from whatever generates code: a package whose components do not
turn into any type in the application does not need a crate.
