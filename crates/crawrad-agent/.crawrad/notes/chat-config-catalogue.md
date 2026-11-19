---
id: chat-config-catalogue
kind: doc
target_file: src/chat_config.rs
line_hint: 238
author: 
created: 
title: "CATALOGUE"
tags: []
signature: "pub const CATALOGUE : & str = \"\""
---

The configuration identifier for the IDE's own model catalogue.

The `config_id` of the IDE's own model catalogue, shown when the agent sent no
model menu (not live yet, CLI path, or an agent without `configOptions`).

# Examples

```rust
use crawrad_agent::chat_config::CATALOGUE;

assert_eq!(CATALOGUE, "");
```
