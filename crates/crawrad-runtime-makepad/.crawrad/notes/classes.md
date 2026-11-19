---
id: classes
kind: doc
target_file: src/lib.rs
line_hint: 151
author: 
created: 
title: "CLASSES"
tags: []
signature: "pub static CLASSES : LazyLock < Vec < & 'static str > > = LazyLock :: new (components :: classes)"
---

Classes supported and managed by this Makepad runtime adapter.

**Derived from the drawing catalog** (`componentes::desenho_de`), rather than
written by hand: a class exists for this runtime exactly when there is
a `src/componentes/<classe>.rs` that knows how to draw it. While the list was
literal, adding a component required two places — and forgetting the second
was silent, which is the defect that the house rule forbids.

# Examples

```rust
use crawrad_runtime_makepad::CLASSES;

assert!(CLASSES.contains(&"Button"));
```
