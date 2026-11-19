---
id: ide-app-arrangement-ideapp-arranjo-padrao
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 343
author: 
created: 
title: "IdeApp::arranjo_padrao"
tags: []
signature: "pub (super) fn arranjo_padrao () -> std :: collections :: HashMap < LiveId , DockItem >"
---

The factory layout, in Rust.

It is the counterpart of the `root = Splitter { … }` block in the DSL above, and the
`o_arranjo_padrao_bate_com_a_dsl` gate compares the two as text —
because diverging here would yield a `Reset Layout` returning a screen that
was never the initial one, with nothing complaining.
