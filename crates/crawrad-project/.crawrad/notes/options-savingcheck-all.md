---
id: options-savingcheck-all
kind: doc
target_file: src/options.rs
line_hint: 364
author: 
created: 
title: "SavingCheck::ALL"
tags: []
signature: "pub const ALL : [Self ; 4] = [Self :: ReopenLastProject , Self :: SaveBeforeBuild , Self :: SaveDesktop , Self :: RecoverCopies ,]"
---

The four, in the order the page shows them.

# Examples

```rust
use crawrad_project::options::SavingCheck;

assert_eq!(SavingCheck::ALL[0], SavingCheck::ReopenLastProject);
```
