---
id: mode
kind: doc
target_file: src/lib.rs
line_hint: 81
author: 
created: 
title: "Mode"
tags: []
signature: "pub enum Mode"
---

Render the form, or let it be used.

It is the `Run` of the classic RAD IDE before compilation existed: in `Live`, the component
receives clicks and keyboard input, the designer adornments disappear, and the form
becomes the application. The document is not touched — whatever the user types in
live mode dies when they return to design mode, as in the classic RAD IDE.

Each adapter achieves this in its own way, and that is where the difference between them
becomes visible: RCL only **stops neutralizing the mouse** on controls, which are already
real controls; RXP swaps painting for real egui widgets — and gets text input,
focus, selection, and IME for free, because they belong to the toolkit and not to us.

# Examples

```rust
use crawrad_runtime::Mode;

assert!(!Mode::Design.is_live());
assert!(Mode::Live.is_live());
```
