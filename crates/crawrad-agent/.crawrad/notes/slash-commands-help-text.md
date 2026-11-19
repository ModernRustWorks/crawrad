---
id: slash-commands-help-text
kind: doc
target_file: src/slash_commands.rs
line_hint: 346
author: 
created: 
title: "help_text"
tags: []
signature: "pub fn help_text () -> String"
---

`/help`: every slash command and what it does.

# Examples

```rust
use crawrad_agent::slash_commands::help_text;

let help = help_text();
assert!(help.contains("/memory"));
```
