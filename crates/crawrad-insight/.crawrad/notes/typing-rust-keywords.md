---
id: typing-rust-keywords
kind: doc
target_file: src/typing.rs
line_hint: 116
author: 
created: 
title: "RUST_KEYWORDS"
tags: []
signature: "pub const RUST_KEYWORDS : & [& str] = & [\"as\" , \"async\" , \"await\" , \"break\" , \"const\" , \"continue\" , \"crate\" , \"dyn\" , \"else\" , \"enum\" , \"extern\" , \"false\" , \"fn\" , \"for\" , \"if\" , \"impl\" , \"in\" , \"let\" , \"loop\" , \"match\" , \"mod\" , \"move\" , \"mut\" , \"pub\" , \"ref\" , \"return\" , \"self\" , \"Self\" , \"static\" , \"struct\" , \"super\" , \"trait\" , \"true\" , \"type\" , \"unsafe\" , \"use\" , \"where\" , \"while\" ,]"
---

Rust's keywords, for the local list while rust-analyzer has not answered.

# Examples

```rust
use crawrad_insight::typing::RUST_KEYWORDS;

assert!(RUST_KEYWORDS.contains(&"fn"));
```
