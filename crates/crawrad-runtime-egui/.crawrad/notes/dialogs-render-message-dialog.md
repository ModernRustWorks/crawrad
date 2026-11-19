---
id: dialogs-render-message-dialog
kind: doc
target_file: src/dialogs.rs
line_hint: 7
author: 
created: 
title: "render_message_dialog"
tags: []
signature: "pub fn render_message_dialog (ctx : & egui :: Context) -> bool"
---

Renders the active message dialog as a centered modal window.

Renders the active message dialog as a centered modal window with dimmed backdrop.

Handles keyboard confirmation (Enter) and cancellation (Escape).
Returns `true` if a dialog was rendered, or `false` if idle.

# Examples

```rust
use crawrad_runtime_egui::render_message_dialog;

let ctx = egui::Context::default();
assert!(!render_message_dialog(&ctx));
```
