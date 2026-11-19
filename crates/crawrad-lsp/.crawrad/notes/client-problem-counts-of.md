---
id: client-problem-counts-of
kind: doc
target_file: src/client.rs
line_hint: 1219
author: 
created: 
title: "problem_counts_of"
tags: []
signature: "fn problem_counts_of (kept : & HashMap < String , Vec < serde_json :: Value > > ,) -> Vec < (String , usize , usize) >"
---

Counts errors and warnings per file from kept diagnostics.

The counts of [`LspClient::problem_counts`] from what was kept: an LSP
severity of 1 is an error, 2 a warning.
