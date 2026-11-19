---
id: slash-commands-slashcategory
kind: doc
target_file: src/slash_commands.rs
line_hint: 4
author: 
created: 
title: "SlashCategory"
tags: []
signature: "pub enum SlashCategory"
---

Categories for organizing slash commands in the autocomplete popup.

# Examples

```rust
use crawrad_agent::SlashCategory;

assert_eq!(SlashCategory::Tools.label(), "Tools & Utilities");
```
