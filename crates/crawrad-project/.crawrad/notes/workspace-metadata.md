---
id: workspace-metadata
kind: doc
target_file: src/workspace.rs
line_hint: 1453
author: 
created: 
title: "Metadata"
tags: []
signature: "pub (crate) struct Metadata"
---

A line from the `cargo metadata` response.

A line from the `cargo metadata` response. **Only what this module reads.**

Unknown fields are ignored by serde, as in [`crate::diagnostics_model`]:
the response brings `id`, `manifest_path`, `dependencies`, `features` and a
dozen other things per package, and listing all here would mean maintaining
the entire format just to use two fields.
