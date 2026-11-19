---
id: ide-app-live-ideapp-after-apply
kind: doc
target_file: src/ide_app/live.rs
line_hint: 7
author: 
created: 
title: "IdeApp::after_apply"
tags: []
signature: "fn after_apply (& mut self , cx : & mut Cx , _apply : & mut Apply , _index : usize , _nodes : & [LiveNode])"
---

Hides the default Makepad caption bar and window buttons.

THE MAKEPAD CAPTION BAR GOES, and it has to go HERE. The owner saw two
title bars stacked (the framework's 27 px "Makepad" bar above ours),
which pushed the footer off a maximized window. `visible: false` in our
DSL does not hold: `makepad-widgets-1.0.0/src/window.rs:169-173` turns
`caption_bar` and `windows_buttons` back on in its own `after_apply` on
Windows, and the only later point is the container's `after_apply`. Both
views are hidden, in the order they were shown, through the public
`set_visible` — no fork.
