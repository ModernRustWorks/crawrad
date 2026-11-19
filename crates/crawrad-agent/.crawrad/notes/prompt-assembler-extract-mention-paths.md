---
id: prompt-assembler-extract-mention-paths
kind: doc
target_file: src/prompt_assembler.rs
line_hint: 225
author: 
created: 
title: "extract_mention_paths"
tags: []
signature: "pub fn extract_mention_paths (text : & str) -> Vec < String >"
---

Extracts mention targets (tokens starting with `@`) from query text.

# Examples

```rust
use crawrad_agent::extract_mention_paths;

let paths = extract_mention_paths("See @src/main.rs for details");
assert_eq!(paths, vec!["src/main.rs"]);
```
