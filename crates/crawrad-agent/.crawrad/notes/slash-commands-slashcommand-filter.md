---
id: slash-commands-slashcommand-filter
kind: doc
target_file: src/slash_commands.rs
line_hint: 173
author: 
created: 
title: "SlashCommand::filter"
tags: []
signature: "pub fn filter (query : & str) -> Vec < Self >"
---

Filter commands matching a user query string.

Filter commands matching a user query string (e.g. `"/m"` -> `[/memory, /mcp]`).

# Examples

```rust
use crawrad_agent::SlashCommand;

let matches = SlashCommand::filter("mem");
assert_eq!(matches.len(), 1);
assert_eq!(matches[0].id, "memory");
```
