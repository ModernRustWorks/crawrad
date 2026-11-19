---
id: targets-makepad-control-events-tooltip-dsl
kind: doc
target_file: src/targets/makepad/control_events.rs
line_hint: 430
author: 
created: 
title: "TOOLTIP_DSL"
tags: []
signature: "pub const TOOLTIP_DSL : & str = \"                crawrad_tooltip = <Tooltip> {\\n\\\n     \\x20                   content: <View> {\\n\\\n     \\x20                       flow: Overlay, width: Fit, height: Fit,\\n\\\n     \\x20                       <RoundedView> {\\n\\\n     \\x20                           width: Fit, height: Fit,\\n\\\n     \\x20                           padding: { left: 8.0, right: 8.0, top: 4.0, bottom: 4.0 },\\n\\\n     \\x20                           draw_bg: { color: #2b2d33, border_size: 1.0, border_color: #4a4d55, border_radius: 3.0 }\\n\\\n     \\x20                           tooltip_label = <Label> {\\n\\\n     \\x20                               width: Fit,\\n\\\n     \\x20                               draw_text: { text_style: <THEME_FONT_REGULAR> { font_size: 9.0 }, color: #e4e5e8 }\\n\\\n     \\x20                           }\\n\\\n     \\x20                       }\\n\\\n     \\x20                   }\\n\\\n     \\x20               }\\n\""
---

Makepad DSL definition for the shared form tooltip.

The form's one tooltip (`crawrad_tooltip`): a small dark box the size of its
text, not makepad 1.0.0's wide white default (`tooltip.rs:29-55`), so it
looks like egui's. The controls' hints and the form's share it.

# Examples

```rust
use crawrad_codegen::targets::makepad::control_events::TOOLTIP_DSL;

assert!(TOOLTIP_DSL.contains("crawrad_tooltip = <Tooltip>"));
```
