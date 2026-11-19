---
id: font-test-answer
kind: doc
target_file: src/font.rs
line_hint: 28
author: 
created: 
title: "test_answer"
tags: []
signature: "pub fn test_answer () -> Option < String >"
---

The test's answer, if a test set one — read where the dialog would open.

# Examples

```rust
use crawrad_os::font::test_answer;

let ans = test_answer();
assert!(ans.is_none() || ans.is_some());
```
