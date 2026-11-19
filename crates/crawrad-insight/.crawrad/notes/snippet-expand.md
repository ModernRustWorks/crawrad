---
id: snippet-expand
kind: doc
target_file: src/snippet.rs
line_hint: 23
author: 
created: 
title: "expand"
tags: []
signature: "pub fn expand (snippet : & str , indent : & str) -> Expanded"
---

`snippet` written on a line indented by `indent`.

# Examples

```rust
use crawrad_insight::snippet::expand;

let res = expand("soma($1, $2)$0", "    ");
assert_eq!(res.text, "soma(, )");
```
