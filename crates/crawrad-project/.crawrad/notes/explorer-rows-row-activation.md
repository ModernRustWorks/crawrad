---
id: explorer-rows-row-activation
kind: doc
target_file: src/explorer_rows/row.rs
line_hint: 36
author: 
created: 
title: "Activation"
tags: []
signature: "pub enum Activation"
---

What a click on a row does.

The variants are ordered by how much the house likes them, and that is not
a joke: [`Self::Run`] is the answer the rule of the house wants — *does an
agent get to do this by tool, without a mouse?* — and the other three are
each a declared exception with a reason.

# Examples

```rust
use crawrad_project::explorer_rows::row::Activation;

let act = Activation::Inert;
assert_eq!(act, Activation::Inert);
```
