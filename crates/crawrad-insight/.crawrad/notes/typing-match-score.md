---
id: typing-match-score
kind: doc
target_file: src/typing.rs
line_hint: 69
author: 
created: 
title: "match_score"
tags: []
signature: "pub fn match_score (label : & str , query : & str) -> Option < u32 >"
---

Scores how well a completion candidate label matches a typed query.

How well `label` answers `query`, a modern editor's way; `None` = it does not.
Starting with the query (ignoring case) beats containing it, which beats
having its letters in order (fewer gaps first). Inside a class the order
given stands. An empty query keeps everything, in its order.

# Examples

```rust
use crawrad_insight::typing::match_score;

assert_eq!(match_score("split", "sp"), Some(3000));
assert_eq!(match_score("other", "sp"), None);
```
