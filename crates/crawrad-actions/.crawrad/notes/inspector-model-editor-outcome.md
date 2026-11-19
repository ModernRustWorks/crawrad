---
id: inspector-model-editor-outcome
kind: doc
target_file: src/inspector_model/editor.rs
line_hint: 113
author: 
created: 
title: "Outcome"
tags: []
signature: "pub enum Outcome"
---

What the shell does about a gesture.

The variants are ordered by how much the house likes them, and that is not
a joke: [`Self::Set`] and [`Self::Run`] both end at the catalogue — one at
`component.set_property` and the other at whatever action the editor names
— so the answer to *does an agent get to do this by tool, without a mouse?*
is yes for both. The other two change nothing.

# Examples

```rust
use crawrad_actions::inspector_model::editor::Outcome;

let outcome = Outcome::Ignore;
assert!(matches!(outcome, Outcome::Ignore));
```
