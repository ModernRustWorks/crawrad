---
id: slash-commands-slashcommand-match-exact
kind: doc
target_file: src/slash_commands.rs
line_hint: 188
author: 
created: 
title: "SlashCommand::match_exact"
tags: []
signature: "pub fn match_exact (text : & str) -> Option < Self >"
---

Match an exact invocation text by its first word.

Match an exact invocation text by its first word (e.g. `"/memory"` or `"/compare Rc Arc"`).

# Examples

```rust
use crawrad_agent::SlashCommand;

let cmd = SlashCommand::match_exact("/memory");
assert!(cmd.is_some());
assert_eq!(cmd.unwrap().id, "memory");
```
