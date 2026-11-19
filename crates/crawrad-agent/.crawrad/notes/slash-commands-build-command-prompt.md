---
id: slash-commands-build-command-prompt
kind: doc
target_file: src/slash_commands.rs
line_hint: 221
author: 
created: 
title: "build_command_prompt"
tags: []
signature: "pub fn build_command_prompt (cmd : & str , argument : & str) -> Option < String >"
---

Expands slash command arguments into prompt text for coding assistance commands.

# Examples

```rust
use crawrad_agent::slash_commands::build_command_prompt;

let prompt = build_command_prompt("/explain", "main function");
assert!(prompt.is_some());
```
