---
id: targets-egui-form-tema-da-barra
kind: doc
target_file: src/targets/egui/form.rs
line_hint: 636
author: 
created: 
title: "TEMA_DA_BARRA"
tags: []
signature: "const TEMA_DA_BARRA : & str = \"\\\n\\x20       // The scroll bar stays VISIBLE while there is something to scroll.\\n\\\n\\x20       //\\n\\\n\\x20       // egui fades the handle out when nobody is touching it\\n\\\n\\x20       // (`ScrollStyle::solid()` starts at `dormant_handle_opacity: 0.0`).\\n\\\n\\x20       // That hides the one hint that there is more content below: you have\\n\\\n\\x20       // to already know, and move the mouse there, to find out.\\n\\\n\\x20       //\\n\\\n\\x20       // A classic RAD IDE keeps it on screen, and so does this.\\n\\\n\\x20       ui.style_mut().spacing.scroll = egui::style::ScrollStyle {\\n\\\n\\x20           dormant_background_opacity: 1.0,\\n\\\n\\x20           dormant_handle_opacity: 1.0,\\n\\\n\\x20           active_background_opacity: 1.0,\\n\\\n\\x20           active_handle_opacity: 1.0,\\n\\\n\\x20           ..egui::style::ScrollStyle::solid()\\n\\\n\\x20       };\\n\""
---

The scrollbar theme of the generated application, in one place.

Applies to ALL scrollbars in the form — `ScrollBox`, `ListBox`, `Memo`,
`TreeView` —, and this is intentional: a scrollbar that is visible in one component and disappears
in another is worse than two that disappear.
