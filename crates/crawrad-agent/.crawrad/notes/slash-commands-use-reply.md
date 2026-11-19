---
id: slash-commands-use-reply
kind: doc
target_file: src/slash_commands.rs
line_hint: 355
author: 
created: 
title: "use_reply"
tags: []
signature: "pub fn use_reply (rest : & str , provider : & str , agent_id : & str) -> (Option < & 'static str > , String)"
---

Formats reply for `/use` command to list or switch agents.

`/use [agent]`: with no name, the list of agents with the active one
marked; with a name, the provider to switch to and the sentence saying so.

# Examples

```rust
use crawrad_agent::slash_commands::use_reply;

let (target, text) = use_reply("", "claude", "claude");
assert!(target.is_none());
assert!(text.contains("Available ACP agents"));
```
