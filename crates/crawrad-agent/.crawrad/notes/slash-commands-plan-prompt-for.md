---
id: slash-commands-plan-prompt-for
kind: doc
target_file: src/slash_commands.rs
line_hint: 316
author: 
created: 
title: "plan_prompt_for"
tags: []
signature: "pub fn plan_prompt_for (objective : & str) -> String"
---

Expanded prompt sent to the agent for `/plan <objective>`.

# Examples

```rust
use crawrad_agent::slash_commands::plan_prompt_for;

let prompt = plan_prompt_for("refactor module");
assert!(prompt.contains("refactor module"));
```
