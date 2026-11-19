---
id: options-savingcheck
kind: doc
target_file: src/options.rs
line_hint: 350
author: 
created: 
title: "SavingCheck"
tags: []
signature: "pub enum SavingCheck"
---

A check box of the Saving and Recovering page.

A check box of the Saving and Recovering page (#555): what it says, the
preference it reads and writes, and every widget that shows it — the same
setting can sit on its page, on Commonly Used and in the search results.

# Examples

```rust
use crawrad_project::options::SavingCheck;

assert_eq!(SavingCheck::ALL.len(), 4);
```
