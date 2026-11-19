---
id: prompt-assembler-resolve-mentions
kind: doc
target_file: src/prompt_assembler.rs
line_hint: 245
author: 
created: 
title: "resolve_mentions"
tags: []
signature: "pub fn resolve_mentions (mention_paths : & [String] , workspace_root : Option < & Path > , live_buffers : Option < & BTreeMap < PathBuf , String > > , max_bytes_per_file : usize ,) -> Vec < MentionedFile >"
---

Resolves mentioned files from live editor buffers or filesystem.

# Examples

```rust
use crawrad_agent::prompt_assembler::resolve_mentions;

let files = resolve_mentions(&[], None, None, 1024);
assert!(files.is_empty());
```
