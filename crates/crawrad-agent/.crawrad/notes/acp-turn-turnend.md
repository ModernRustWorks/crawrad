---
id: acp-turn-turnend
kind: doc
target_file: src/acp_turn.rs
line_hint: 13
author: 
created: 
title: "TurnEnd"
tags: []
signature: "pub enum TurnEnd"
---

How a turn ended.

# Examples

```rust
use crawrad_agent::acp_turn::TurnEnd;

let end = TurnEnd::Cancelled;
assert_eq!(end, TurnEnd::Cancelled);
```
