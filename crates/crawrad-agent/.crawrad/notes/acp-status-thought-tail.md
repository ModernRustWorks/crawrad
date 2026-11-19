---
id: acp-status-thought-tail
kind: doc
target_file: src/acp_status.rs
line_hint: 193
author: 
created: 
title: "thought_tail"
tags: []
signature: "fn thought_tail (thought : & str) -> String"
---

Cuts thought text to its last tail characters.

The thought on one line (whitespace runs collapsed), cut to its last
[`THOUGHT_TAIL_CHARS`] characters.
