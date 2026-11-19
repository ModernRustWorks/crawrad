---
id: chat-layout-hero-shortcuts
kind: doc
target_file: src/chat_layout.rs
line_hint: 207
author: 
created: 
title: "HERO_SHORTCUTS"
tags: []
signature: "pub const HERO_SHORTCUTS : [(& str , & str) ; 7] = [(\"explain\" , \"/explain \") , (\"refactor\" , \"/refactor \") , (\"test\" , \"/test \") , (\"document\" , \"/docs \") , (\"find\" , \"/find \") , (\"optimize\" , \"/optimize \") , (\"teach\" , \"/teach \") ,]"
---

Shortcut cards displayed in an empty chat session.

The empty chat's shortcut cards, as their widgets end (`btn_card_explain`…),
and the slash command each one types into the prompt. The last one, `teach`,
sits alone and centred under the pairs: the Rust Teacher card.

# Examples

```rust
use crawrad_agent::chat_layout::HERO_SHORTCUTS;

assert_eq!(HERO_SHORTCUTS[0].0, "explain");
```
