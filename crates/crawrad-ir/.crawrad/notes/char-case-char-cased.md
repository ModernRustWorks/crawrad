---
id: char-case-char-cased
kind: doc
target_file: src/char_case.rs
line_hint: 15
author: 
created: 
title: "char_cased"
tags: []
signature: "pub fn char_cased (text : & str , char_case : & str) -> String"
---

Converts `text` to the case named by `char_case`.

`text` in the case `char_case` names: `ecUpperCase` and `ecLowerCase`
turn it, `ecNormal` (and a name this does not know) leaves it.

# Examples

```rust
use crawrad_ir::char_case::char_cased;

assert_eq!(char_cased("Hello", "ecUpperCase"), "HELLO");
assert_eq!(char_cased("Hello", "ecLowerCase"), "hello");
assert_eq!(char_cased("Hello", "ecNormal"), "Hello");
```
