---
id: slash-commands-acp-commands-help
kind: doc
target_file: src/slash_commands.rs
line_hint: 341
author: 
created: 
title: "acp_commands_help"
tags: []
signature: "pub fn acp_commands_help () -> & 'static str"
---

Help string listing available `/acp` subcommands.

# Examples

```rust
use crawrad_agent::slash_commands::acp_commands_help;

assert!(acp_commands_help().contains("/acp test"));
```
