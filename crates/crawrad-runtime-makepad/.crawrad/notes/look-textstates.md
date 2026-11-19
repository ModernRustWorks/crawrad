---
id: look-textstates
kind: doc
target_file: src/look.rs
line_hint: 48
author: 
created: 
title: "TextStates"
tags: []
signature: "pub enum TextStates"
---

Identifies which state colours a widget's `draw_text` shader mixes.

Which state colours a widget's `draw_text` mixes (#433): only these
fields exist, and applying another is logged as an error.

# Examples

```rust
use crawrad_runtime_makepad::look::TextStates;

let states = TextStates::Plain;
assert_eq!(states, TextStates::Plain);
```
