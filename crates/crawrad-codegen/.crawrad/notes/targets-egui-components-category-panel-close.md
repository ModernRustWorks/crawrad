---
id: targets-egui-components-category-panel-close
kind: doc
target_file: src/targets/egui/components/category_panel.rs
line_hint: 32
author: 
created: 
title: "CLOSE"
tags: []
signature: "const CLOSE : & str = \"\\\n\\x20               });\\n\\\n\\x20           });\\n\\\n\\x20           }\\n\\\n\\x20       }\\n\""
---

Closes the category panel block opened by `emit`.

What closes the block [`emit`] opened, after the children: the scope, the
enabled ui, the `if !collapsed` and the `if visible`. A plain `&str`, so
single braces.
