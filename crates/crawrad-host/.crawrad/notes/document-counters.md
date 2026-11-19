---
id: document-counters
kind: doc
target_file: src/document.rs
line_hint: 154
author: 
created: 
title: "Counters"
tags: []
signature: "pub struct Counters"
---

**What each status bar cell says** about the open document.

# Why a type, and not the single string that used to be here

Until 17/09/2026 this was a `resumo()` that returned
`"no selection    19 nodes    rev 0"` — three facts pasted together with spaces,
for a single `Label`. The owner asked for a modern editor's rich bar, where each fact is
a CELL with its own outline, and separating them back would require searching
for a space inside a text where a component name can contain anything.

It is the same argument that the tree's [`Linha`] already records for not joining
name and class: whatever paints in different places receives them separated.

# The `—` is not decoration

Without a document, the two numbers become `—`. A zero there would be a
plausible number for something that does not exist, and it is the kind of lie
that this shell's bar already had to take off the screen once.

# Examples

```rust
use crawrad_host::document::Counters;

let counters = Counters::default();
assert_eq!(counters.nodes, "");
```
