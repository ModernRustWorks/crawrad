---
id: events-rewrite-statement-calls
kind: doc
target_file: src/events.rs
line_hint: 759
author: 
created: 
title: "rewrite_statement_calls"
tags: []
signature: "pub (crate) fn rewrite_statement_calls (code : & str , tipo : & str , wired : & [WiredEvent] , prefix : & str , reg : & crawrad_registry :: Registry , used_indices : & mut std :: collections :: BTreeSet < usize > , rewritten : & mut Vec < WiredEvent > ,) -> String"
---

Rewrites whole-statement handler calls line-by-line to reporting.
