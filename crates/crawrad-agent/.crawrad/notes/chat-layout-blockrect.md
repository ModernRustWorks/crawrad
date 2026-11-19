---
id: chat-layout-blockrect
kind: doc
target_file: src/chat_layout.rs
line_hint: 116
author: 
created: 
title: "BlockRect"
tags: []
signature: "pub type BlockRect < 'a > = (& 'a str , [f64 ; 4] , Option < [f64 ; 4] >)"
---

One block descriptor of a reply card for JSON serialization.

One block of a reply card, for [`card_json`]: its kind, rect and, for a
code block, the rect of its Copy.

# Examples

```rust
use crawrad_agent::chat_layout::BlockRect;

let b: BlockRect<'_> = ("text", [0.0, 0.0, 100.0, 50.0], None);
assert_eq!(b.0, "text");
```
