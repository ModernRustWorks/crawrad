---
id: events
kind: doc
target_file: src/lib.rs
line_hint: 317
author: 
created: 
title: "Events"
tags: []
signature: "pub struct Events"
---

Centralized catalog of standard events from the classic RAD IDE / CrawRAD.

Allows reusing event definitions in a concise and typed manner:

The block is `text`, not `ignore`, on purpose: this is not Rust, it is a
piece of the `component!` body. With `ignore`, `rustdoc` still tries
to compile it when the suite runs with `--include-ignored` — which is the
house gate's mandatory flag —, and the crate's `--doc` stayed red for
weeks because of that. `text` does not compile under any flag.
```text
events: [
    Events::CHANGE,
    Events::EXPANDING,
    Events::COLLAPSING,
    Events::MOUSE_CLICK,
]
```

# Examples

```rust
use crawrad_registry::Events;

assert_eq!(Events::MOUSE_CLICK.name, "on_click");
```
