---
id: ide-app-link-ideapp-hint-inherited-badge
kind: doc
target_file: src/ide_app/link.rs
line_hint: 1
author:
created:
title: "IdeApp::hint_inherited_badge"
tags: []
signature: "pub (super) fn hint_inherited_badge (& mut self , cx : & mut Cx , at : DVec2)"
---

The tooltip of an inherited component's corner badge (#851).

"Inherited from <base>" while the pointer is over the badge and no menu is
open; gone when it leaves, and when the component menu opens over it.
