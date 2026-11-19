---
id: targets-egui-components-drawer-close
kind: doc
target_file: src/targets/egui/components/drawer.rs
line_hint: 32
author: 
created: 
title: "CLOSE"
tags: []
signature: "const CLOSE : & str = \"\\\n\\x20               });\\n\\\n\\x20           });\\n\\\n\\x20       }\\n\""
---

Closes the block opened by [`emit`] after child controls.

What closes the block [`emit`] opened, after the children: the scope, the
enabled ui, and the `if visible && opened`. A plain `&str`, so single braces.
