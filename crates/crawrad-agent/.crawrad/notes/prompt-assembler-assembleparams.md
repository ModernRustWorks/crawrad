---
id: prompt-assembler-assembleparams
kind: doc
target_file: src/prompt_assembler.rs
line_hint: 156
author: 
created: 
title: "AssembleParams"
tags: []
signature: "pub struct AssembleParams < 'a >"
---

Parameters passed to the prompt assembler.

# Examples

```rust
use crawrad_agent::AssembleParams;

let params = AssembleParams::default();
assert_eq!(params.user_query, "");
```
