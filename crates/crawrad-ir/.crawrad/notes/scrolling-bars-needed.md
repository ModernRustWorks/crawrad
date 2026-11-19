---
id: scrolling-bars-needed
kind: doc
target_file: src/scrolling.rs
line_hint: 71
author: 
created: 
title: "bars_needed"
tags: []
signature: "pub fn bars_needed (client : (f32 , f32) , content : (f32 , f32)) -> Bars"
---

**Which bars this content asks for**, given the box's inner size.

`client` is the size of the box and `content` is how far the content
reaches from its origin corner — the furthest right edge and the furthest
bottom edge.

`>` and not `>=`: content ending exactly on the edge **fits**, and a bar
that appears to scroll zero pixels is the bar a classic RAD IDE does not show.

# Examples

```rust
use crawrad_ir::scrolling::bars_needed;

let bars = bars_needed((100.0, 100.0), (150.0, 50.0));
assert!(bars.horizontal);
assert!(!bars.vertical);
```
