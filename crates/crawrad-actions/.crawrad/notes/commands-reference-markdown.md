---
id: commands-reference-markdown
kind: doc
target_file: src/reference.rs
line_hint: 10
author: 
created: 
title: "commands_reference_markdown"
tags: []
signature: "pub fn commands_reference_markdown () -> String"
---

The manual page listing every IDE command, its shortcuts and its id.

# Examples

```rust
let md = crawrad_actions::reference::commands_reference_markdown();
assert!(md.starts_with("# Menus and commands"));
```
