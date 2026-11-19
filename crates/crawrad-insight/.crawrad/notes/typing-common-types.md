---
id: typing-common-types
kind: doc
target_file: src/typing.rs
line_hint: 124
author: 
created: 
title: "COMMON_TYPES"
tags: []
signature: "pub const COMMON_TYPES : & [& str] = & [\"String\" , \"Vec\" , \"Option\" , \"Result\" , \"Some\" , \"None\" , \"Ok\" , \"Err\" , \"Box\" , \"Rc\" , \"Arc\" , \"Cell\" , \"RefCell\" , \"HashMap\" , \"HashSet\" , \"BTreeMap\" , \"BTreeSet\" , \"Path\" , \"PathBuf\" , \"bool\" , \"usize\" , \"isize\" , \"u8\" , \"u16\" , \"u32\" , \"u64\" , \"u128\" , \"i8\" , \"i16\" , \"i32\" , \"i64\" , \"f32\" , \"f64\" , \"char\" , \"str\" ,]"
---

Common std types and constructors, for the same local list.

# Examples

```rust
use crawrad_insight::typing::COMMON_TYPES;

assert!(COMMON_TYPES.contains(&"String"));
```
