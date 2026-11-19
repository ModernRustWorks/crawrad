---
id: inspector-model-name-width
kind: doc
target_file: src/inspector_model.rs
line_hint: 618
author: 
created: 
title: "name_width"
tags: []
signature: "pub fn name_width (requested : f64 , room : f64) -> f64"
---

Computes the clamped name column width for the available room.

Where the splitter lands for a `requested` name width in a row that has
`room` for name and value together.

The request is kept as asked and clamped here, at drawing time: an agent
that sends 9999 gets the widest the panel holds, and the column comes back
to what it asked when the dock widens. A dock too narrow for both minimums
favours the names, because a value can still be read in its editor.

# Examples

```rust
use crawrad_actions::inspector_model::name_width;

assert_eq!(name_width(100.0, 300.0), 100.0);
```
